mod bank;

use crate::model::{Action, Page, Phase, View};
use mirui::app::{App, RendererFactory};
use mirui::audio::{AudioOutputState, CueId};
use mirui::surface::Surface;
use serde::Deserialize;
use std::collections::BTreeSet;
use std::path::Path;

const SETTINGS: &str = "/opt/arkos-nearby/ui-sound.json";
const NAVIGATION_INTERVAL_MS: u64 = 65;
const ACTION_INTERVAL_MS: u64 = 120;
const INPUT_WINDOW_MS: u64 = 750;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Cue {
    Focus,
    Confirm,
    Back,
    Refresh,
    RoomFound,
    JoinRequest,
    Ready,
    Start,
    Error,
    Restored,
}

impl Cue {
    const fn id(self) -> CueId {
        CueId::new(self as u16)
    }
}

#[derive(Deserialize)]
#[serde(default, deny_unknown_fields)]
struct Preferences {
    enabled: bool,
    gain: u8,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            enabled: true,
            gain: 96,
        }
    }
}

impl Preferences {
    fn load(path: &Path) -> Self {
        match std::fs::read(path) {
            Ok(data) => serde_json::from_slice(&data).unwrap_or_else(|error| {
                eprintln!("Sound preferences: {error}");
                Self::default()
            }),
            Err(error) => {
                if error.kind() != std::io::ErrorKind::NotFound {
                    eprintln!("Sound preferences: {error}");
                }
                Self::default()
            }
        }
    }
}

pub fn install<B: Surface, F: RendererFactory<B>>(app: &mut App<B, F>) {
    use mirui::app::plugins::AudioPlugin;
    #[cfg(feature = "sound")]
    let sink = mirui::audio::SdlAudioSink::new();
    #[cfg(not(feature = "sound"))]
    let sink = mirui::audio::SilentAudioSink::default();
    app.add_plugin(AudioPlugin::new(sink, &bank::BANK));
    let preferences = Preferences::load(Path::new(SETTINGS));
    if let Some(audio) = app.audio() {
        audio.set_master_gain(preferences.gain);
        audio.set_muted(!preferences.enabled);
        eprintln!(
            "{}",
            serde_json::json!({"ui_audio":format!("{:?}", audio.state()),"sdl_output":cfg!(feature="sound")})
        );
    }
}

pub fn play<B: Surface, F: RendererFactory<B>>(app: &App<B, F>, cue: Option<Cue>) {
    if let (Some(audio), Some(cue)) = (app.audio(), cue) {
        if !audio
            .state()
            .is_some_and(|state| state.output == AudioOutputState::Ready && !state.muted)
        {
            return;
        }
        audio.stop(Cue::Focus.id());
        audio.stop(cue.id());
        audio.play(cue.id());
    }
}

#[cfg(all(target_os = "linux", feature = "sound"))]
pub fn probe() -> Result<(), String> {
    let mut app = App::headless(1, 1);
    app.with_default_widgets().with_default_systems();
    install(&mut app);
    let audio = app.audio().ok_or("Audio plugin is unavailable")?;
    let state = audio.state().ok_or("Audio state is unavailable")?;
    if state.output != AudioOutputState::Ready {
        return Err(format!("Audio output: {:?}", state.output));
    }
    play(&app, Some(Cue::Confirm));
    app.render()
        .map_err(|error| format!("Audio dispatch: {error:?}"))?;
    std::thread::sleep(std::time::Duration::from_millis(200));
    println!(
        "{}",
        serde_json::json!({"audio_output":"ready","muted":state.muted,"gain":state.master_gain,"cue":"confirm"})
    );
    Ok(())
}

#[derive(Default)]
pub struct Feedback {
    navigation_at: Option<u64>,
    last_focus: Option<u64>,
    last_action: Option<u64>,
    seen_rooms: BTreeSet<String>,
}

fn selection(view: &View) -> Option<String> {
    match view.page {
        Page::Mode | Page::Choice => Some(view.selected.to_string()),
        Page::Rooms => view.rooms.get(view.selected).map(|row| row.key.clone()),
        Page::Games => view.entries.get(view.selected).map(|row| row.key.clone()),
        _ => None,
    }
    .filter(|key| !key.is_empty())
}

fn elapsed(last: Option<u64>, now: u64, interval: u64) -> bool {
    last.is_none_or(|last| now.saturating_sub(last) >= interval)
}

impl Feedback {
    pub fn input(&mut self, action: Action, view: &View, now: u64) -> Option<Cue> {
        if matches!(
            action,
            Action::Up
                | Action::Down
                | Action::Left
                | Action::Right
                | Action::PageUp
                | Action::PageDown
        ) {
            if !view.busy {
                self.navigation_at = Some(now);
            }
            return None;
        }
        let cue = match action {
            Action::Back if view.page != Page::Leaving => Some(Cue::Back),
            Action::Theme if crate::theme::Palette::can_switch(view) => Some(Cue::Confirm),
            Action::Refresh if view.page == Page::Rooms && !view.busy && !view.refreshing => {
                Some(Cue::Refresh)
            }
            Action::Confirm if !view.busy => match view.page {
                Page::Mode | Page::Choice => Some(Cue::Confirm),
                Page::Games if !view.entries.is_empty() => Some(Cue::Confirm),
                Page::Rooms
                    if view
                        .rooms
                        .get(view.selected)
                        .is_some_and(|room| room.available) =>
                {
                    Some(Cue::Confirm)
                }
                Page::Host if view.host_action_enabled() => Some(Cue::Confirm),
                Page::Joined if view.network_only && view.host_action_enabled() => {
                    Some(Cue::Confirm)
                }
                Page::Leaving if matches!(view.phase, Phase::RestoreFailed | Phase::RoomActive) => {
                    Some(Cue::Confirm)
                }
                _ => None,
            },
            _ => None,
        };
        if cue.is_some() && elapsed(self.last_action, now, ACTION_INTERVAL_MS) {
            self.last_action = Some(now);
            cue
        } else {
            None
        }
    }

    pub fn observe(&mut self, previous: Option<&View>, next: &View, now: u64) -> Option<Cue> {
        if previous.is_none_or(|old| old.page != next.page) {
            self.seen_rooms.clear();
            self.navigation_at = None;
        }
        let mut found = false;
        if next.page == Page::Rooms {
            for room in next
                .rooms
                .iter()
                .filter(|room| room.available && !room.key.is_empty())
            {
                found |= self.seen_rooms.insert(room.key.clone());
            }
        }
        let old = previous?;
        if old.phase != next.phase {
            let cue = match next.phase {
                Phase::Failed | Phase::RestoreFailed => Some(Cue::Error),
                Phase::ApprovalPending => Some(Cue::JoinRequest),
                Phase::HostReady | Phase::WaitingStart if next.ready => Some(Cue::Ready),
                Phase::NetworkConnected if next.connected => Some(Cue::Ready),
                Phase::Starting => Some(Cue::Start),
                _ => None,
            };
            if cue.is_some() {
                return cue;
            }
        }
        if old.page == Page::Leaving
            && matches!(next.page, Page::Choice | Page::Rooms)
            && !next.busy
        {
            return Some(Cue::Restored);
        }
        if found {
            return Some(Cue::RoomFound);
        }
        if old.page == next.page
            && selection(old) != selection(next)
            && selection(next).is_some()
            && self
                .navigation_at
                .is_some_and(|at| now.saturating_sub(at) <= INPUT_WINDOW_MS)
            && elapsed(self.last_focus, now, NAVIGATION_INTERVAL_MS)
        {
            self.last_focus = Some(now);
            self.navigation_at = None;
            return Some(Cue::Focus);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::RoomRow;
    use mirui::audio::{AudioCommand, AudioMixer};

    #[test]
    fn plugin_controls_expire_with_the_menu() {
        use mirui::surface::framebuf::FramebufSurface;
        let mut app = App::new(FramebufSurface::new(16, 16, |_, _| {}));
        install(&mut app);
        let audio = app.audio().unwrap();
        assert_eq!(audio.state().unwrap().master_gain, 96);
        play(&app, Some(Cue::Confirm));
        drop(app);
        assert!(audio.state().is_none());
    }

    #[test]
    fn all_cues_are_finite_nonzero_and_unclipped() {
        assert_eq!(bank::BANK.cues().len(), Cue::Restored as usize + 1);
        for cue in bank::BANK.cues() {
            let mut mixer = AudioMixer::<4>::new(&bank::BANK, 48_000).unwrap();
            mixer
                .apply(AudioCommand::Play {
                    cue: cue.id,
                    gain: 255,
                })
                .unwrap();
            let mut pcm = vec![0; 24_000];
            mixer.render_mono(&mut pcm);
            assert!(pcm.iter().any(|sample| *sample != 0));
            assert!(pcm.iter().all(|sample| sample.unsigned_abs() < 30_000));
            assert!(pcm[16_800..].iter().all(|sample| *sample == 0));
            assert_eq!(mixer.active_voice_count(), 0);
        }
    }

    #[test]
    fn observation_is_quiet_until_real_edges_and_readiness() {
        let mut feedback = Feedback::default();
        let mut view = View {
            page: Page::Host,
            ..View::default()
        };
        assert_eq!(feedback.observe(None, &view, 0), None);
        let old = view.clone();
        view.phase = Phase::HostReady;
        assert_eq!(feedback.observe(Some(&old), &view, 1), None);
        view.ready = true;
        assert_eq!(feedback.observe(Some(&old), &view, 2), Some(Cue::Ready));
        assert_eq!(feedback.observe(Some(&view), &view, 3), None);
        let old = view.clone();
        view.status = "Updated status".into();
        assert_eq!(feedback.observe(Some(&old), &view, 4), None);
    }

    #[test]
    fn discovery_and_reordering_do_not_replay_focus_or_discovery() {
        let mut feedback = Feedback::default();
        let old = View {
            page: Page::Rooms,
            ..View::default()
        };
        feedback.observe(None, &old, 0);
        let mut view = old.clone();
        view.rooms = vec![
            RoomRow {
                key: "one".into(),
                available: true,
                ..RoomRow::default()
            },
            RoomRow {
                key: "two".into(),
                available: true,
                ..RoomRow::default()
            },
        ];
        assert_eq!(feedback.observe(Some(&old), &view, 1), Some(Cue::RoomFound));
        let old = view.clone();
        view.rooms.reverse();
        view.selected = 1;
        view.rooms[0].signal = 50;
        assert_eq!(feedback.observe(Some(&old), &view, 2), None);
        feedback.input(Action::Up, &view, 100);
        let old = view.clone();
        view.selected = 0;
        assert_eq!(feedback.observe(Some(&old), &view, 101), Some(Cue::Focus));
        assert_eq!(feedback.observe(Some(&view), &view, 102), None);
    }

    #[test]
    fn disabled_actions_and_repeated_cleanup_stay_quiet() {
        let mut feedback = Feedback::default();
        assert_eq!(
            feedback.input(Action::Theme, &View::default(), 0),
            Some(Cue::Confirm)
        );
        feedback.last_action = None;
        let mut view = View {
            page: Page::Host,
            ..View::default()
        };
        assert_eq!(feedback.input(Action::Confirm, &view, 0), None);
        view.ready = true;
        assert_eq!(
            feedback.input(Action::Confirm, &view, 1),
            Some(Cue::Confirm)
        );
        assert_eq!(feedback.input(Action::Confirm, &view, 2), None);
        view.page = Page::Leaving;
        view.busy = true;
        assert_eq!(feedback.input(Action::Back, &view, 500), None);
        view.phase = Phase::RestoreFailed;
        view.busy = false;
        assert_eq!(
            feedback.input(Action::Confirm, &view, 501),
            Some(Cue::Confirm)
        );
    }
}
