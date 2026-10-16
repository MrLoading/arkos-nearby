use super::{copy, style::*};
use crate::model::{Page, Phase, View};
use mirui::prelude::*;
use mirui::types::Transform;

const SETTLED: u16 = 800;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Frame {
    pub page: Page,
    pub phase: Phase,
    pub selected: Option<usize>,
    pub ready: bool,
    pub connected: bool,
    pub network_only: bool,
    pub selection: u16,
    pub pulse: u16,
    pub phase_age: u16,
    pub press: u16,
    pub wave: Option<u16>,
    pub walker: u16,
    pub host: u16,
    pub guest: u16,
    pub walking: bool,
    pub host_walking: bool,
    pub guest_walking: bool,
    pub visitor: bool,
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            page: Page::Choice,
            phase: Phase::Idle,
            selected: None,
            ready: false,
            connected: false,
            network_only: false,
            selection: SETTLED,
            pulse: SETTLED,
            phase_age: SETTLED,
            press: SETTLED,
            wave: None,
            walker: 250,
            host: 200,
            guest: 800,
            walking: false,
            host_walking: false,
            guest_walking: false,
            visitor: false,
        }
    }
}

#[derive(Default)]
pub(super) struct Timeline {
    key: Option<(Page, Phase, bool, bool, bool, String)>,
    selected: Option<usize>,
    catalog: Option<(String, Vec<crate::model::GameRow>)>,
    selection_at: Option<u64>,
    pulse_at: Option<u64>,
    phase_at: Option<u64>,
    press_at: Option<u64>,
    press_sequence: u64,
    walker: Travel,
    host: Travel,
    guest: Travel,
    visitor: bool,
}

#[derive(Default)]
struct Travel {
    from: f32,
    to: f32,
    at: u64,
}

impl Travel {
    fn moving(&self, now: u64) -> bool {
        now.saturating_sub(self.at) < 240 && self.from != self.to
    }
    fn value(&self, now: u64) -> u16 {
        let p = (now.saturating_sub(self.at) as f32 / 240.0).min(1.0);
        (self.from + (self.to - self.from) * (1.0 - (1.0 - p).powi(3))).round() as u16
    }
    fn target(&mut self, target: u16, now: u64, snap: bool) {
        if snap {
            self.from = f32::from(target);
            self.to = self.from;
            self.at = now;
        } else if self.to != f32::from(target) {
            self.from = f32::from(self.value(now));
            self.to = f32::from(target);
            self.at = now;
        }
    }
}

fn age(now: u64, start: Option<u64>) -> u16 {
    start.map_or(SETTLED, |start| {
        now.saturating_sub(start).min(u64::from(SETTLED)) as u16
    })
}

impl Timeline {
    pub(super) fn sample(
        &mut self,
        view: &View,
        selected: Option<usize>,
        now: u64,
        press: u64,
    ) -> Frame {
        let key = (
            view.page,
            copy::phase(view),
            view.ready,
            view.connected,
            view.network_only,
            view.room_name.clone(),
        );
        let page_changed = self
            .key
            .as_ref()
            .is_none_or(|old| old.0 != key.0 || old.5 != key.5);
        let catalog_changed = if view.page == Page::Games {
            self.catalog
                .as_ref()
                .is_none_or(|(path, entries)| path != &view.core || entries != &view.entries)
        } else {
            self.catalog.is_some()
        };
        let entered =
            page_changed || catalog_changed || self.key.as_ref().is_some_and(|old| old.4 != key.4);
        if catalog_changed {
            self.catalog =
                (view.page == Page::Games).then(|| (view.core.clone(), view.entries.clone()));
        }
        let session_page = |page| matches!(page, Page::Host | Page::Joined | Page::Leaving);
        let continues_session = self.key.as_ref().is_some_and(|old| {
            session_page(old.0) && session_page(key.0) && (old.5 == key.5 || key.0 == Page::Leaving)
        });
        if !matches!(
            key.1,
            Phase::Leaving | Phase::Failed | Phase::RestoreFailed | Phase::NetworkReconnecting
        ) || !continues_session
        {
            self.visitor = view.ready
                || view.connected
                || view.page == Page::Joined
                || matches!(key.1, Phase::ApprovalPending | Phase::PeerPreparing);
        }
        let entering = self.key.is_some()
            && !continues_session
            && matches!(key.1, Phase::Creating | Phase::Connecting);
        if self.key.as_ref() != Some(&key) {
            // Opening an existing state does not replay its readiness celebration.
            self.phase_at = self
                .key
                .as_ref()
                .filter(|old| {
                    (old.0 == key.0 && old.5 == key.5)
                        || matches!(key.1, Phase::Creating | Phase::Connecting | Phase::Leaving)
                })
                .map(|_| now);
            self.key = Some(key);
        }
        if entered || self.selected != selected {
            self.selection_at = Some(now);
            if entered || age(now, self.pulse_at) >= 240 {
                self.pulse_at = Some(now);
            }
            self.selected = selected;
        }
        if press != self.press_sequence {
            self.press_sequence = press;
            self.press_at = Some(now);
        }
        self.walker.target(
            if selected == Some(1) { 750 } else { 250 },
            now,
            page_changed,
        );
        let guest = match copy::phase(view) {
            Phase::ApprovalPending | Phase::ApprovalPendingClient => 980,
            Phase::Connecting
            | Phase::PeerPreparing
            | Phase::NetworkConfirming
            | Phase::NetworkReconnecting => 940,
            Phase::Checking | Phase::Notifying => 880,
            Phase::Starting => 940,
            Phase::Leaving | Phase::Closed => 300,
            Phase::Failed | Phase::RestoreFailed if continues_session => self.guest.value(now),
            _ => 800,
        };
        let host = match copy::phase(view) {
            Phase::Starting => 730,
            Phase::Leaving | Phase::Closed => 100,
            Phase::Failed | Phase::RestoreFailed if continues_session => self.host.value(now),
            _ => 200,
        };
        if entering {
            self.host.target(100, now, true);
            self.guest.target(980, now, true);
        }
        self.host.target(host, now, !continues_session && !entering);
        self.guest
            .target(guest, now, !continues_session && !entering);
        Frame {
            page: view.page,
            phase: copy::phase(view),
            selected,
            ready: view.ready,
            connected: view.connected,
            network_only: view.network_only,
            selection: age(now, self.selection_at),
            pulse: age(now, self.pulse_at),
            phase_age: age(now, self.phase_at),
            press: age(now, self.press_at),
            wave: copy::loading(view).then_some((now % 1200 / 32 * 32) as u16),
            walker: self.walker.value(now),
            host: self.host.value(now),
            guest: self.guest.value(now),
            walking: self.walker.moving(now),
            host_walking: self.host.moving(now),
            guest_walking: self.guest.moving(now),
            visitor: self.visitor,
        }
    }
}

pub(super) fn hop(age: u16, height: f32) -> i32 {
    let p = (f32::from(age) / 300.0).min(1.0);
    -(p * std::f32::consts::PI)
        .sin()
        .mul_add(height, 0.0)
        .round() as i32
}

pub(super) fn recoil(age: u16) -> i32 {
    let p = (f32::from(age) / 240.0).min(1.0);
    ((p * std::f32::consts::PI * 5.0).sin() * 3.0 * (1.0 - p).powi(2)).round() as i32
}

impl Frame {
    pub(super) fn expansion(self) -> (i32, i32) {
        let p = (f32::from(self.pulse) / 240.0).min(1.0);
        let amount = 16.0 * p.powi(2) * (1.0 - p).powi(2);
        ((amount * 9.0).round() as i32, (amount * 7.0).round() as i32)
    }

    pub(super) fn card_transform(self, index: usize) -> Transform {
        if self.selected != Some(index) {
            return Transform::IDENTITY;
        }
        let pressed = if self.press < 120 { 2 } else { 0 };
        let age = if matches!(self.page, Page::Host | Page::Joined) {
            self.phase_age.min(self.selection)
        } else {
            self.selection
        };
        Transform::translate(recoil(age).into(), (hop(age, 3.0) + pressed).into())
    }

    pub(super) fn card_color(self, index: usize) -> ColorToken {
        if self.selected == Some(index) && self.selection < 300 {
            ACCENT_DIM
        } else {
            PANEL
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_and_catalog_navigation_animate_without_poll_replays() {
        let mut timeline = Timeline::default();
        let mut view = View::default();
        for page in [Page::Mode, Page::Choice, Page::Mode, Page::Games] {
            view.page = page;
            assert_eq!(timeline.sample(&view, Some(0), 1000, 0).selection, 0);
            view.status = "Status update".into();
            assert_eq!(timeline.sample(&view, Some(0), 1120, 0).selection, 120);
            assert_eq!(timeline.sample(&view, Some(0), 2000, 0).selection, SETTLED);
        }
        view.core = "NES / FC".into();
        view.entries.push(crate::model::GameRow {
            name: "Game".into(),
            ..Default::default()
        });
        assert_eq!(timeline.sample(&view, Some(0), 2100, 0).selection, 0);
        let moving = timeline.sample(&view, Some(0), 2160, 0);
        assert_ne!(moving.card_transform(0), Transform::IDENTITY);
        assert_eq!(moving.card_transform(1), Transform::IDENTITY);
        assert_eq!(timeline.sample(&view, Some(0), 3000, 0).selection, SETTLED);
    }

    #[test]
    fn snapshots_do_not_replay_motion_and_bursts_cannot_accumulate_scale() {
        let mut timeline = Timeline::default();
        let view = View::default();
        timeline.sample(&view, Some(0), 0, 0);
        timeline.sample(&view, Some(1), 10, 0);
        assert_eq!(timeline.sample(&view, Some(1), 130, 0).expansion(), (9, 7));
        for i in 0..100 {
            let frame = timeline.sample(&view, Some(i % 2), 140 + i as u64 * 17, 0);
            let (x, y) = frame.expansion();
            assert!((0..=9).contains(&x) && (0..=7).contains(&y));
        }
        let mut next = view.clone();
        next.status = "Updated status".into();
        let a = timeline.sample(&next, Some(1), 5000, 0);
        let b = timeline.sample(&next, Some(1), 6000, 0);
        assert_eq!(a, b);
        assert_eq!(b.expansion(), (0, 0));
    }

    #[test]
    fn readiness_and_reopening_follow_backend_state() {
        let mut timeline = Timeline::default();
        let mut view = View {
            page: Page::Host,
            phase: Phase::WaitingPeer,
            ..View::default()
        };
        timeline.sample(&view, None, 0, 0);
        view.phase = Phase::PeerPreparing;
        assert!(!timeline.sample(&view, None, 10, 0).ready);
        view.phase = Phase::HostReady;
        view.ready = true;
        assert_eq!(timeline.sample(&view, Some(0), 20, 0).phase_age, 0);
        assert_eq!(
            Timeline::default().sample(&view, Some(0), 20, 0).phase_age,
            SETTLED
        );
    }

    #[test]
    fn session_routes_continue_from_current_positions_and_preserve_membership() {
        let mut timeline = Timeline::default();
        let mut view = View {
            page: Page::Host,
            phase: Phase::HostReady,
            ready: true,
            room_name: "Room".into(),
            ..View::default()
        };
        let ready = timeline.sample(&view, Some(0), 0, 0);
        view.phase = Phase::Starting;
        let start = timeline.sample(&view, Some(0), 10, 0);
        assert_eq!((ready.host, ready.guest), (start.host, start.guest));
        let moving = timeline.sample(&view, Some(0), 130, 0);
        assert!(moving.host > start.host && moving.guest > start.guest);
        view.page = Page::Leaving;
        view.phase = Phase::Leaving;
        view.room_name.clear();
        view.ready = false;
        let leaving = timeline.sample(&view, None, 130, 0);
        assert_eq!((moving.host, moving.guest), (leaving.host, leaving.guest));
        assert!(leaving.visitor);
        let settled = timeline.sample(&view, None, 1000, 0);
        assert!(settled.host < leaving.host && settled.guest < leaving.guest);
        assert!(!settled.host_walking && !settled.guest_walking);

        let mut alone = Timeline::default();
        view.page = Page::Host;
        view.phase = Phase::WaitingPeer;
        assert!(!alone.sample(&view, None, 0, 0).visitor);
        view.page = Page::Leaving;
        view.phase = Phase::Leaving;
        assert!(!alone.sample(&view, None, 10, 0).visitor);
    }
}
