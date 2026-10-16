use crate::model::{GameRow, RoomRow};
use mirui::core::model::ModelList;
use mirui::prelude::*;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Key {
    Named(String),
    Anonymous { revision: u64, index: usize },
}

#[derive(Clone, Copy)]
pub(super) struct TextChanged;

impl TextChanged {
    fn contains(self, _: Self) -> bool {
        true
    }
}

#[model(change = TextChanged, watch(text = TextChanged))]
pub(super) struct Row {
    key: Key,
    name: String,
    detail: String,
    room: bool,
    #[observe]
    pub index: usize,
    #[observe]
    pub strength: u8,
    #[observe]
    pub available: bool,
    #[observe]
    pub directory: bool,
    #[observe]
    pub icon: u8,
}

#[model]
impl Row {
    fn text_value(&self, title: bool) -> String {
        if title {
            self.name.clone()
        } else if self.available || !self.room {
            self.detail.clone()
        } else {
            "房间已离线，等待重新出现".into()
        }
    }
}

impl RowHandle {
    pub fn label(&self, title: bool) -> String {
        self.text_revision();
        self.available();
        self.text_value(title)
    }
}

impl Row {
    pub fn room(index: usize, data: &RoomRow) -> Self {
        Self {
            key: Key::Named(data.key.clone()),
            index,
            name: data.name.clone(),
            detail: data.detail.clone(),
            room: true,
            strength: data.signal,
            available: data.available,
            directory: false,
            icon: 0,
        }
    }
    pub fn game(index: usize, data: &GameRow) -> Self {
        Self {
            key: Key::Named(data.key.clone()),
            index,
            name: data.name.clone(),
            detail: data.detail.clone(),
            room: false,
            strength: 0,
            available: data.available,
            directory: data.directory,
            icon: match data.icon.as_str() {
                "gb" | "gbc" | "gba" => 1,
                "arcade" | "neogeo" | "cps1" | "cps2" | "cps3" => 2,
                _ => 0,
            },
        }
    }
}

// Normalize ambiguous transport keys at ingestion; ModelList owns member identities.
pub(super) fn reconcile(list: &mut ModelList<Row>, mut rows: Vec<Row>, revision: u64) {
    let ambiguous: Vec<_> = rows
        .iter()
        .map(|row| {
            matches!(&row.key, Key::Named(key) if key.is_empty())
                || rows.iter().filter(|other| other.key == row.key).count() > 1
        })
        .collect();
    for (row, ambiguous) in rows.iter_mut().zip(ambiguous) {
        if ambiguous {
            row.key = Key::Anonymous {
                revision,
                index: row.index,
            };
        }
    }
    list.reconcile(
        rows,
        |row| row.key.clone(),
        |current, incoming| *current = incoming,
    )
    .unwrap_or_else(|failure| panic!("Invalid visible row collection: {:?}", failure.error()));
}
