use crate::model::{Action, LauncherView, View};
use mirui::prelude::*;

const HOLD_MS: u64 = 700;
const FEEDBACK_MS: u64 = 130;
const MAX_FRAME_GAP_MS: u64 = 250;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Intent {
    Action(Action),
    Game { key: String, multiplayer: bool },
}

#[derive(Default)]
enum Gesture {
    #[default]
    Idle,
    Pressed {
        key: String,
        at: u64,
        previous: u64,
    },
    Consumed,
}

#[model]
#[derive(Default)]
pub struct Launcher {
    view: LauncherView,
    #[observe]
    revision: u64,
    #[observe]
    pub progress: u16,
    #[observe]
    pub selection_age: u16,
    now: u64,
    selected_at: u64,
    gesture: Gesture,
    pending: Option<Intent>,
}

#[model]
impl Launcher {
    fn snapshot(&self) -> LauncherView {
        self.view.clone()
    }
    pub fn present(&mut self, view: &View) {
        if self.view.overlay != view.launcher.overlay
            || self.view.selected != view.launcher.selected
        {
            self.selected_at = self.now;
        }
        if self.view.key != view.launcher.key
            || self.view.navigation != view.launcher.navigation
            || self.view.overlay != view.launcher.overlay
            || self.view.enabled != view.launcher.enabled
        {
            self.cancel();
        }
        if self.view != view.launcher {
            self.view = view.launcher.clone();
            self.revision += 1;
        }
    }

    pub fn blocked(&self) -> bool {
        self.view.enabled && (self.view.navigation || !self.view.overlay.is_empty())
    }

    pub fn cancel(&mut self) {
        if !matches!(self.gesture, Gesture::Idle) {
            self.gesture = Gesture::Consumed;
        }
        self.progress = 0;
    }

    pub fn press(&mut self, at: u64) {
        if !matches!(self.gesture, Gesture::Idle) {
            return;
        }
        if !self.view.enabled {
            self.pending = Some(Intent::Action(Action::Refresh));
            self.gesture = Gesture::Consumed;
        } else if !self.blocked() && !self.view.key.is_empty() {
            self.gesture = Gesture::Pressed {
                key: self.view.key.clone(),
                at,
                previous: at,
            };
        } else {
            self.gesture = Gesture::Consumed;
        }
    }

    pub fn release(&mut self, at: u64) {
        self.tick(at);
        if let Gesture::Pressed { key, .. } = std::mem::take(&mut self.gesture) {
            self.pending = Some(Intent::Game {
                key,
                multiplayer: false,
            });
        }
        self.progress = 0;
    }

    pub fn tick(&mut self, now: u64) {
        self.now = now;
        self.selection_age = now.saturating_sub(self.selected_at).min(800) as u16;
        let Gesture::Pressed { key, at, previous } = &mut self.gesture else {
            return;
        };
        if now.saturating_sub(*previous) > MAX_FRAME_GAP_MS {
            self.cancel();
            return;
        }
        *previous = now;
        let elapsed = now.saturating_sub(*at);
        if elapsed >= HOLD_MS {
            self.pending = Some(Intent::Game {
                key: key.clone(),
                multiplayer: true,
            });
            self.gesture = Gesture::Consumed;
            self.progress = 0;
        } else if elapsed >= FEEDBACK_MS {
            self.progress = (elapsed * 1000 / HOLD_MS) as u16;
        }
    }

    #[effects]
    fn take_intent(&mut self) -> Option<Intent> {
        self.pending.take()
    }
}

impl LauncherHandle {
    pub fn view(&self) -> LauncherView {
        self.revision();
        self.snapshot()
    }
}

#[cfg(target_os = "linux")]
pub mod input;

#[cfg(any(target_os = "linux", test))]
mod controls;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn release_hold_cancel_and_stale_selection_are_exclusive() {
        let mut model = Launcher {
            view: LauncherView {
                enabled: true,
                key: "game".into(),
                netplay: true,
                ..Default::default()
            },
            ..Default::default()
        };
        model.press(0);
        model.release(80);
        assert_eq!(
            model.pending.take(),
            Some(Intent::Game {
                key: "game".into(),
                multiplayer: false
            })
        );
        model.press(100);
        for t in (200..=800).step_by(100) {
            model.tick(t);
        }
        assert_eq!(
            model.pending.take(),
            Some(Intent::Game {
                key: "game".into(),
                multiplayer: true
            })
        );
        model.press(810);
        model.release(820);
        assert!(model.pending.is_none());
        model.press(900);
        model.cancel();
        model.release(950);
        assert!(model.pending.is_none());
        model.press(1000);
        model.tick(1600);
        model.release(1601);
        assert!(model.pending.is_none());
        model.press(1700);
        model.present(&View::default());
        model.release(1800);
        assert!(model.pending.is_none());
    }
}
