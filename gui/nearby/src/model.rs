use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Page {
    Lobby,
    Candidates,
    Mode,
    #[default]
    Choice,
    Host,
    Rooms,
    Joined,
    Leaving,
    Unavailable,
    Games,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Phase {
    ResolvingGame,
    SelectingGame,
    #[default]
    Idle,
    Creating,
    WaitingPeer,
    PeerPreparing,
    ApprovalPending,
    HostReady,
    Connecting,
    Checking,
    Notifying,
    WaitingStart,
    Starting,
    Leaving,
    Closed,
    Failed,
    RestoreFailed,
    RoomActive,
    NetworkConfirming,
    NetworkConnected,
    NetworkReconnecting,
    ApprovalPendingClient,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RoomRow {
    #[serde(default)]
    pub key: String,
    pub name: String,
    pub detail: String,
    pub signal: u8,
    pub available: bool,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct View {
    #[serde(default)]
    pub library: LibraryView,
    #[serde(default)]
    pub platforms: bool,
    #[serde(default)]
    pub lobby: bool,
    #[serde(default)]
    pub current_game: bool,
    #[serde(default)]
    pub artwork: String,
    #[serde(default)]
    pub network_only: bool,
    #[serde(default)]
    pub connected: bool,
    #[serde(default)]
    pub local_address: String,
    #[serde(default)]
    pub peers: Vec<NetworkPeer>,
    pub page: Page,
    pub title: String,
    pub core: String,
    pub selected: usize,
    #[serde(default)]
    pub room_selected: Option<usize>,
    pub rooms: Vec<RoomRow>,
    pub room_name: String,
    pub ready: bool,
    pub busy: bool,
    #[serde(default)]
    pub refreshing: bool,
    pub status: String,
    #[serde(default)]
    pub phase: Phase,
    #[serde(default)]
    pub back_label: String,
    #[serde(default)]
    pub entries: Vec<GameRow>,
}

impl View {
    pub fn scene_page(&self) -> Page {
        if self.lobby && self.page == Page::Mode {
            Page::Lobby
        } else {
            self.page
        }
    }

    pub fn can_switch_theme(&self) -> bool {
        matches!(self.page, Page::Mode | Page::Choice) && !self.busy
    }

    pub fn host_action_enabled(&self) -> bool {
        (self.ready || self.connected || self.phase == Phase::ApprovalPending) && !self.busy
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkPeer {
    pub name: String,
    pub address: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GameRow {
    #[serde(default)]
    pub favorite: bool,
    #[serde(default)]
    pub icon: String,
    #[serde(default = "available")]
    pub available: bool,
    #[serde(default)]
    pub key: String,
    pub name: String,
    pub detail: String,
    pub directory: bool,
}

impl Default for GameRow {
    fn default() -> Self {
        Self {
            favorite: false,
            icon: String::new(),
            available: true,
            key: String::new(),
            name: String::new(),
            detail: String::new(),
            directory: false,
        }
    }
}

fn available() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn game_rows_are_display_only_and_old_room_views_remain_valid() {
        assert!(serde_json::from_value::<GameRow>(serde_json::json!({"name":"game","detail":"select","directory":false,"path":"/roms/game.zip"})).is_err());
        let view: View = serde_json::from_value(serde_json::json!({"page":"choice","title":"game","core":"core","selected":0,"rooms":[],"room_name":"","ready":false,"busy":false,"status":""})).unwrap();
        assert!(view.entries.is_empty());
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[cfg(any(target_os = "linux", test))]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Confirm,
    Back,
    Refresh,
    PageUp,
    PageDown,
    Auxiliary,
    Start,
    #[serde(skip)]
    Theme,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LibraryKind {
    Platforms,
    #[default]
    Directory,
    Favorites,
    Search,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default, deny_unknown_fields)]
pub struct LibraryView {
    pub kind: LibraryKind,
    pub query: String,
    pub keyboard: bool,
    pub key_selected: usize,
    pub row_selected: usize,
    pub sort: String,
}
