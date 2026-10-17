use mirui::prelude::*;
use mirui::ui::widgets::{Image, Text, TextAlign::Center};

use super::{
    artwork::preview,
    consoles::console,
    lists::{game_rows, games},
    pixel::{Scenery, Stage::Walker, art},
    rows::Row,
    selection::FocusTarget,
    state::DisplayState,
    style::*,
    widgets::{caption, label, paragraph, shadow},
};
use crate::model::LibraryKind;

const KEYS: [&str; 40] = [
    "Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P", "A", "S", "D", "F", "G", "H", "J", "K", "L",
    "退格", "Z", "X", "C", "V", "B", "N", "M", "空格", "清空", "结果", "1", "2", "3", "4", "5",
    "6", "7", "8", "9", "0",
];

#[compose]
pub(super) fn library(kind: LibraryKind, state: model!(DisplayState), metrics: Metrics) -> Entity {
    ui! {
        Column (grow: 1.0, min_height: 0, min_width: 0) {
            match kind {
                LibraryKind :: Favorites => {
                    shelf (state, metrics)
                }
                LibraryKind :: Search => {
                    search (state, metrics)
                }
                _ => {
                    games (state, metrics)
                }
            }
        }
    }
}

#[compose]
fn shelf(state: model!(DisplayState), metrics: Metrics) -> Entity {
    ui! {
        Column (grow: 1.0, min_height: 0, row_gap: metrics.gap) [
            Scenery::Ridge,
        ] {
            if ${ state.entries().is_empty() } {
                Column (grow: 1.0, min_height: 0, bg_color: PANEL, align: AlignItems::Center) {
                    console (super::consoles::Platform::Favorites)
                    label ("收藏夹还是空的", metrics.title, FOREGROUND, Center)
                }
            } else {
                Row (grow: 1.0, min_height: 0, column_gap: metrics.gap, padding: Padding::all(4)) {
                    walk ${ state.entries().entries() } with item by item.0 {
                        cartridge (state, item.1, metrics)
                    }
                }
            }
            art (state, Walker, metrics.walker_height())
        }
    }
}

#[compose]
fn cartridge(state: model!(DisplayState), row: model!(Row), metrics: Metrics) -> Entity {
    let movement = row.clone();
    let focus = row.clone();
    let name = row.clone();
    let platform = row.clone();
    let selected = row.clone();
    let enabled = row.clone();
    ui! {
        Column (
            grow: 0.0,
            width: Dimension::percent(33),
            min_width: 0,
            min_height: 0,
            transform: ${ state.motion().card_transform(movement.index()) }
        ) {
            Column (
                grow: 1.0,
                min_width: 0,
                min_height: 0,
                padding: Padding::all(metrics.gap),
                container: true,
                row_gap: metrics.gap,
                bg_color: ${ state.motion().card_color(focus.index()) }
            ) [
                FocusTarget::row(row.clone()),
            ] {
                Column (
                    grow: 1.0,
                    min_height: 36,
                    min_width: 0,
                    bg_color: DEEP,
                    padding: Padding::all(4),
                    border_width: 3,
                    border_color: DIVIDER
                ) {
                    Column (grow: 1.0, min_height: 0, min_width: 0, bg_color: SKY) {
                        if ${ state.selected() == Some(selected.index()) } {
                            preview (state, metrics)
                        } else {
                            Column (grow: 1.0, min_height: 0) {
                                walk ${ std::iter::once(platform.platform()) } with kind by kind {
                                    console (kind)
                                }
                            }
                        }
                    }
                    Row (height: 8, shrink: 0.0, justify: JustifyContent::Center, column_gap: 3) {
                        walk 0..8 with _index {
                            View (width: 5, height: 8, bg_color: WARNING)
                        }
                    }
                }
                Text (
                    text: ${ name.label(true) },
                    width: Dimension::percent(100),
                    min_width: 0,
                    height: 52,
                    shrink: 0.0,
                    font: font(metrics.title),
                    font_size: metrics.title,
                    text_color: ${ if enabled.available() { FOREGROUND } else { DISABLED } },
                    paragraph: paragraph(Center, 2)
                ) [
                    FONT_PAINT,
                ]
                View (
                    position: Position::Absolute,
                    top: 2,
                    left: @(width) { width - Fixed::from_int(14) },
                    width: 12,
                    height: 12,
                    shrink: 0.0
                ) {
                    Image (
                        src: path!(
                            M 8 0 H 12 V 4 H 20 V 8 H 16 V 12 H 18 V 20 H 12 V 16 H 8 V 20 H 2 V 12 H 4 V 8 H
                            0 V 4 H 8 Z
                        ),
                        viewbox: 20,
                        width: 12,
                        height: 12,
                        color: WARNING
                    )
                }
            }
            shadow (metrics)
        }
    }
}

#[compose]
fn search(state: model!(DisplayState), metrics: Metrics) -> Entity {
    let query_state = state.clone();
    ui! {
        Column (grow: 1.0, min_height: 0, row_gap: metrics.gap) {
            Column (
                height: 32,
                shrink: 0.0,
                padding: Padding::all(4),
                bg_color: PANEL,
                border_width: 1,
                border_color: ACCENT
            ) {
                caption (
                    move || {
                        let query = query_state.query();
                        if query.is_empty() { "输入游戏名或文件名".into() } else { query }
                    },
                    || FOREGROUND,
                    metrics.body,
                    Center,
                    LABEL_LEADING
                )
            }
            if ${ state.keyboard() } {
                Column (grow: 1.0, min_height: 0, row_gap: metrics.gap) {
                    walk 0..4 with line {
                        Row (grow: 1.0, min_height: 0, column_gap: metrics.gap) {
                            walk 0..10 with column {
                                keycap (state, line * 10 + column, metrics)
                            }
                        }
                    }
                }
            } else {
                game_rows (state, metrics)
            }
        }
    }
}

#[compose]
fn keycap(state: model!(DisplayState), index: usize, metrics: Metrics) -> Entity {
    ui! {
        Column (
            grow: 1.0,
            min_width: 0,
            min_height: 0,
            transform: ${ state.motion().card_transform(100 + index) }
        ) {
            Column (
                grow: 1.0,
                min_width: 0,
                min_height: 0,
                justify: JustifyContent::Center,
                bg_color: ${ if state.selected() == Some(100 + index) { ACCENT } else { PANEL } }
            ) [
                FocusTarget::fixed(100 + index),
            ] {
                caption (
                    move || KEYS[index].into(),
                    move || if state.selected() == Some(100 + index) { ON_ACCENT } else { FOREGROUND },
                    metrics.body,
                    Center,
                    LABEL_LEADING
                )
            }
            shadow (metrics)
        }
    }
}
