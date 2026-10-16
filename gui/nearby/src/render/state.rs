use super::{copy::*, selection, style::*};
use crate::model::{Page, Phase, View};
use TextField::{Headline, HostButton, Next, Peer};
use mirui::app::RendererFactory;
use mirui::prelude::*;
use mirui::surface::Surface;
use mirui::ui::widgets::navigation::{PageDirection, PageTransition};

#[derive(Clone, Copy)]
pub(super) enum TextField {
    Headline,
    Next,
    HostButton,
    Peer,
    RoomName,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct StepColors {
    pub(super) background: ColorToken,
    pub(super) foreground: ColorToken,
}

pub(super) fn publish<T: Clone + PartialEq>(signal: &Signal<T>, value: T) {
    if signal.get_untracked() != value {
        signal.set(value);
    }
}

#[model]
pub(super) struct DisplayState {
    view: View,
    direction: PageDirection,
    rooms: super::rows::Rows<crate::model::RoomRow>,
    entries: super::rows::Rows<crate::model::GameRow>,
    devices: Signal<Vec<crate::model::NetworkPeer>>,
    room_name: Signal<String>,
    heading: Signal<String>,
    subtitle: Signal<String>,
    status: Signal<String>,
    hint: Signal<String>,
    back: Signal<String>,
    headline: Signal<String>,
    next: Signal<String>,
    host_button: Signal<String>,
    peer: Signal<String>,
    confirm: Signal<String>,
    #[observe]
    pub(super) confirm_visible: bool,
    #[observe]
    pub(super) steps: [StepColors; 3],
    #[observe]
    pub(super) stamp: Color,
    #[observe]
    pub(super) activity: [Color; 3],
    timeline: super::motion::Timeline,
    #[observe]
    pub(super) motion: super::motion::Frame,
}

#[model]
impl DisplayState {
    pub(super) fn present(&mut self, view: &View) {
        publish(&self.status, status_text(view).into());
        publish(&self.hint, hint_text(view).into());
        publish(
            &self.back,
            if view.back_label.is_empty() {
                "返回"
            } else {
                &view.back_label
            }
            .into(),
        );
        publish(&self.headline, headline(view).into());
        publish(&self.next, next_step(view).into());
        publish(&self.host_button, button_text(view).into());
        publish(&self.peer, peer_text(view).into());
        publish(&self.confirm, confirm_text(view).into());
        self.confirm_visible = !confirm_text(view).is_empty();
        let active = match phase(view) {
            Phase::Connecting => Some(0),
            Phase::Checking | Phase::Notifying => Some(1),
            Phase::WaitingStart | Phase::Starting => Some(2),
            _ => None,
        };
        self.steps = std::array::from_fn(|index| {
            if active == Some(index) {
                StepColors {
                    background: ACCENT,
                    foreground: ON_ACCENT,
                }
            } else if active.is_some_and(|step| index < step) {
                StepColors {
                    background: ACCENT_DIM,
                    foreground: ACCENT,
                }
            } else {
                StepColors {
                    background: PANEL,
                    foreground: DISABLED,
                }
            }
        });
        let first = view
            .selected
            .saturating_sub(3)
            .min(view.rooms.len().saturating_sub(4));
        let rows: Vec<_> = view
            .rooms
            .iter()
            .enumerate()
            .skip(first)
            .take(4)
            .map(|(i, row)| (i, row.clone()))
            .collect();
        self.rooms.present(rows);
        let entries = view.entries.iter().take(4).cloned().enumerate().collect();
        self.entries.present(entries);
        let mut devices = vec![crate::model::NetworkPeer {
            name: "本机".into(),
            address: if view.local_address.is_empty() {
                "等待分配地址".into()
            } else {
                view.local_address.clone()
            },
        }];
        devices.extend(view.peers.iter().cloned());
        publish(&self.devices, devices);
        publish(&self.room_name, view.room_name.clone());
        publish(&self.heading, page_heading(view).into());
        publish(
            &self.subtitle,
            match view.page {
                Page::Mode => "连接附近的掌机".into(),
                _ if view.network_only => "仅组网 · 局域网连接".into(),
                Page::Games => view.core.clone(),
                _ => format!("当前游戏  {}", view.title),
            },
        );
        self.view.clone_from(view);
    }

    pub(super) fn rooms(&self) -> Vec<super::rows::Row<crate::model::RoomRow>> {
        self.rooms.get()
    }
    pub(super) fn entries(&self) -> Vec<super::rows::Row<crate::model::GameRow>> {
        self.entries.get()
    }
    pub(super) fn devices(&self) -> Vec<crate::model::NetworkPeer> {
        self.devices.get()
    }
    pub(super) fn room_name(&self) -> String {
        self.room_name.get()
    }
    pub(super) fn heading(&self) -> String {
        self.heading.get()
    }
    pub(super) fn subtitle(&self) -> String {
        self.subtitle.get()
    }
    pub(super) fn status(&self) -> String {
        self.status.get()
    }
    pub(super) fn hint(&self) -> String {
        self.hint.get()
    }
    pub(super) fn back(&self) -> String {
        self.back.get()
    }
    pub(super) fn headline(&self) -> String {
        self.headline.get()
    }
    pub(super) fn next(&self) -> String {
        self.next.get()
    }
    pub(super) fn host_button(&self) -> String {
        self.host_button.get()
    }
    pub(super) fn peer(&self) -> String {
        self.peer.get()
    }
    pub(super) fn confirm(&self) -> String {
        self.confirm.get()
    }

    pub(super) fn snapshot(&self) -> View {
        self.view.clone()
    }

    #[observe]
    pub(super) fn page_key(&self) -> PageKey {
        PageKey {
            page: self.view.page,
            network_only: self.view.network_only
                && matches!(
                    self.view.page,
                    Page::Choice | Page::Rooms | Page::Host | Page::Joined | Page::Leaving
                ),
        }
    }

    #[observe]
    pub(super) fn transition(&self) -> PageTransition {
        PageTransition::slide(mirui::anim::SNAPPY).with_direction(self.direction)
    }

    #[cfg(any(target_os = "linux", test))]
    pub(super) fn navigate_back(&mut self) {
        self.direction = PageDirection::Backward;
    }

    #[cfg(any(target_os = "linux", test))]
    pub(super) fn navigate_forward(&mut self) {
        self.direction = PageDirection::Forward;
    }

    #[observe]
    pub(super) fn selected(&self) -> Option<usize> {
        match self.view.page {
            Page::Mode | Page::Choice => Some(self.view.selected.min(1)),
            Page::Rooms if !self.view.rooms.is_empty() => {
                Some(self.view.selected.min(self.view.rooms.len() - 1))
            }
            Page::Games if !self.view.entries.is_empty() => Some(self.view.selected.min(3)),
            Page::Host if host_action_enabled(&self.view) => Some(0),
            Page::Joined if self.view.network_only && host_action_enabled(&self.view) => Some(0),
            _ => None,
        }
    }

    #[observe]
    pub(super) fn ready(&self) -> bool {
        self.view.ready
    }

    #[observe]
    pub(super) fn enabled(&self) -> bool {
        host_action_enabled(&self.view)
    }

    pub(super) fn feedback(&mut self, stamp: Color, activity: [Color; 3]) {
        self.stamp = stamp;
        self.activity = activity;
    }

    #[observe]
    pub(super) fn choice_motion(&self) -> super::motion::Frame {
        self.motion.choice()
    }

    #[observe]
    pub(super) fn walker_motion(&self) -> super::motion::Frame {
        self.motion.walker()
    }

    #[observe]
    pub(super) fn session_motion(&self) -> super::motion::Frame {
        self.motion.session()
    }

    #[observe]
    pub(super) fn signal_motion(&self) -> super::motion::Frame {
        self.motion.signal()
    }

    #[observe]
    pub(super) fn search_motion(&self) -> super::motion::Frame {
        self.motion.search()
    }

    pub(super) fn animate(&mut self, now: u64, press_sequence: u64) {
        let selected = self.selected();
        self.motion = self
            .timeline
            .sample(&self.view, selected, now, press_sequence);
    }
}

pub(super) type DisplayHandle = <DisplayState as mirui::core::model::Model>::Handle;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct PageKey {
    pub(super) page: Page,
    pub(super) network_only: bool,
}

impl DisplayStateHandle {
    pub(super) fn text(&self, field: TextField) -> String {
        match field {
            Headline => self.headline(),
            Next => self.next(),
            HostButton => self.host_button(),
            Peer => self.peer(),
            TextField::RoomName => self.room_name(),
        }
    }
}

pub(super) struct DisplayRuntime {
    state: DisplayHandle,
}

pub(super) fn install<B: Surface, F: RendererFactory<B>>(app: &mut App<B, F>) {
    let state = app
        .add_model(DisplayState {
            view: View::default(),
            direction: PageDirection::Forward,
            rooms: super::rows::Rows::default(),
            entries: super::rows::Rows::default(),
            devices: Signal::new(Vec::new()),
            room_name: Signal::new(String::new()),
            heading: Signal::new(String::new()),
            subtitle: Signal::new(String::new()),
            status: Signal::new(String::new()),
            hint: Signal::new(String::new()),
            back: Signal::new(String::new()),
            headline: Signal::new(String::new()),
            next: Signal::new(String::new()),
            host_button: Signal::new(String::new()),
            peer: Signal::new(String::new()),
            confirm: Signal::new(String::new()),
            confirm_visible: false,
            steps: [StepColors {
                background: PANEL,
                foreground: DISABLED,
            }; 3],
            stamp: app.theme().resolve(ACCENT_DIM),
            activity: [app.theme().resolve(BACKGROUND); 3],
            timeline: super::motion::Timeline::default(),
            motion: super::motion::Frame::default(),
        })
        .finish();
    app.add_system(selection::update::system(state.clone()));
    app.add_resource(DisplayRuntime { state });
}

#[compose]
pub(super) fn display_state() -> DisplayHandle {
    res!(DisplayRuntime).with(|runtime| runtime.state.clone())
}
