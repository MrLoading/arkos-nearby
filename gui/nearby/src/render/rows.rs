use super::state::publish;
use crate::model::{GameRow, RoomRow};
use mirui::prelude::*;

pub(super) trait RowData: Clone + PartialEq + 'static {
    fn key(&self) -> &str;
}

impl RowData for RoomRow {
    fn key(&self) -> &str {
        &self.key
    }
}

impl RowData for GameRow {
    fn key(&self) -> &str {
        &self.key
    }
}

#[derive(Clone)]
pub(super) struct Row<T: RowData> {
    pub token: u64,
    pub index: Signal<usize>,
    pub data: Signal<T>,
}

impl<T: RowData> PartialEq for Row<T> {
    fn eq(&self, other: &Self) -> bool {
        self.token == other.token
    }
}

pub(super) struct Rows<T: RowData> {
    visible: Signal<Vec<Row<T>>>,
    next_token: u64,
}

impl<T: RowData> Default for Rows<T> {
    fn default() -> Self {
        Self {
            visible: Signal::new(Vec::new()),
            next_token: 0,
        }
    }
}

impl<T: RowData> Rows<T> {
    pub fn get(&self) -> Vec<Row<T>> {
        self.visible.get()
    }

    pub fn present(&mut self, items: Vec<(usize, T)>) {
        let old = self.visible.get_untracked();
        if old.len() == items.len()
            && old.iter().zip(&items).all(|(row, (index, data))| {
                row.index.get_untracked() == *index && row.data.get_untracked() == *data
            })
        {
            return;
        }
        let mut next = Vec::with_capacity(items.len());
        for (index, data) in &items {
            let unique = !data.key().is_empty()
                && items
                    .iter()
                    .filter(|(_, item)| item.key() == data.key())
                    .count()
                    == 1;
            let matches: Vec<_> = old
                .iter()
                .filter(|row| unique && row.data.get_untracked().key() == data.key())
                .collect();
            let row = if let [row] = matches.as_slice() {
                publish(&row.index, *index);
                publish(&row.data, data.clone());
                (*row).clone()
            } else {
                self.next_token += 1;
                Row {
                    token: self.next_token,
                    index: Signal::new(*index),
                    data: Signal::new(data.clone()),
                }
            };
            next.push(row);
        }
        publish(&self.visible, next);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(key: &str) -> RoomRow {
        RoomRow {
            key: key.into(),
            name: "Same name".into(),
            ..RoomRow::default()
        }
    }

    #[test]
    fn identity_is_independent_of_labels_positions_and_mutable_content() {
        let mut rows = Rows::default();
        rows.present(vec![(0, row("a")), (1, row("b"))]);
        let old = rows.get();
        let mut changed = row("a");
        changed.signal = 10;
        rows.present(vec![(0, row("b")), (1, changed.clone())]);
        let next = rows.get();
        assert_eq!(next[0].token, old[1].token);
        assert_eq!(next[1].token, old[0].token);
        assert_eq!(old[0].index.get(), 1);
        assert_eq!(old[0].data.get(), changed);
        rows.present(vec![]);
        rows.present(vec![(0, row("a"))]);
        assert_ne!(rows.get()[0].token, old[0].token);
    }

    #[test]
    fn missing_or_duplicate_keys_cannot_reuse_another_identity() {
        for keys in [["", ""], ["same", "same"]] {
            let mut rows = Rows::default();
            let initial = vec![(0, row(keys[0])), (1, row(keys[1]))];
            rows.present(initial.clone());
            let old = rows.get();
            rows.present(initial.clone());
            assert!(rows.get().iter().zip(&old).all(|(a, b)| a.token == b.token));
            let mut changed = initial;
            changed[0].1.signal = 10;
            rows.present(changed);
            assert!(
                rows.get()
                    .iter()
                    .all(|row| old.iter().all(|before| row.token != before.token))
            );
        }
    }
}
