use super::pixel::Stage::Search;
use super::selection::FocusTarget;
use super::{
    Section,
    pixel::{self, Scenery, art},
    rows::Row,
    state::DisplayState,
    style::*,
    widgets::{caption, label, paragraph, shadow},
};
use mirui::prelude::*;
use mirui::ui::widgets::Text;
use mirui::ui::widgets::TextAlign::{Center, Start};

#[compose]
fn row_signal(state: model!(DisplayState), row: model!(Row), metrics: Metrics) -> Entity {
    ui! {
        View (width: 30, height: if metrics.compact { 26 } else { 30 }, shrink: 0.0) {
            View (grow: 1.0, width: Dimension::percent(100)) [
                pixel::SignalSprite {
                    state: state.clone(),
                    row: row.clone(),
                },
            ]
        }
    }
}

#[compose]
pub(super) fn rooms(state: model!(DisplayState), metrics: Metrics) -> Entity {
    ui! {
        Column (grow: 1.0, min_height: 0) [
            Scenery::Ridge,
        ] {
            room_rows (state, metrics)
        }
    }
}

#[compose]
pub(super) fn room_rows(state: model!(DisplayState), metrics: Metrics) -> Entity {
    ui! {
        Column (
            grow: 1.0,
            min_height: 0,
            row_gap: metrics.gap
        ) [
            Section::FocusList,
        ] {
            if ${ state.rooms().is_empty() } {
                Column (grow: 1.0, min_height: 0) {
                    Column (
                        grow: 1.0,
                        bg_color: PANEL,
                        padding: Padding::all(metrics.inset),
                        justify: JustifyContent::Center,
                        row_gap: metrics.gap
                    ) {
                        art (state, Search, if metrics.compact { 48 } else { 88 })
                        label (
                            "正在搜索附近的房间",
                            metrics.title,
                            FOREGROUND,
                            Center
                        )
                        Text (
                            "请好友先创建房间，并把两台掌机放近一些",
                            width: Dimension::percent(100),
                            height: Dimension::Content,
                            font: font(metrics.small),
                            font_size: metrics.small,
                            text_color: SECONDARY,
                            paragraph: paragraph(Center, 2)
                        ) [
                            FONT_PAINT,
                        ]
                    }
                    shadow (metrics)
                }
            } else {
                Column (grow: 1.0, min_height: 0, row_gap: metrics.gap) {
                    walk ${ state.rooms().entries() } with item by item.0 {
                        list_row (state, item.1, metrics, true)
                    }
                }
            }
        }
    }
}

#[compose]
pub(super) fn games(state: model!(DisplayState), metrics: Metrics) -> Entity {
    ui! {
        Column (grow: 1.0, min_height: 0) [
            Scenery::Ridge,
        ] {
            game_rows (state, metrics)
        }
    }
}

#[compose]
pub(super) fn game_rows(state: model!(DisplayState), metrics: Metrics) -> Entity {
    ui! {
        Column (
            grow: 1.0,
            min_height: 0,
            row_gap: metrics.gap
        ) [
            Section::FocusList,
        ] {
            if ${ state.entries().is_empty() } {
                Column (grow: 1.0, min_height: 0) {
                    Column (
                        grow: 1.0,
                        bg_color: PANEL,
                        justify: JustifyContent::Center,
                        padding: Padding::all(metrics.inset)
                    ) {
                        label (
                            "没有可选游戏",
                            metrics.title,
                            FOREGROUND,
                            Center
                        )
                    }
                    shadow (metrics)
                }
            } else {
                Column (grow: 1.0, min_height: 0, row_gap: metrics.gap) {
                    walk ${ state.entries().entries() } with item by item.0 {
                        list_row (state, item.1, metrics, false)
                    }
                }
            }
        }
    }
}

#[compose]
fn row_label(row: model!(Row), size: u16, title: bool) -> Entity {
    let content = row.clone();
    let colors = row;
    ui! {
        caption (
            move || content.label(title),
            move || {
                if !colors.available() {
                    DISABLED
                } else if title {
                    FOREGROUND
                } else {
                    SECONDARY
                }
            },
            size,
            Start,
            ROW_LEADING
        )
    }
}

#[compose]
fn list_row(
    state: model!(DisplayState),
    item: model!(Row),
    metrics: Metrics,
    has_signal: bool,
) -> Entity {
    let color_index = item.clone();
    let transform_index = item.clone();
    let accent_index = item.clone();
    let available = item.clone();
    let marker = item.clone();
    ui! {
        Column (
            grow: 1.0,
            min_height: metrics.row_min_height(),
            max_height: metrics.row_height(),
            min_width: 0,
            transform: ${ state.motion().card_transform(transform_index.index()) }
        ) {
            Row (
                grow: 1.0,
                min_height: 0,
                padding: metrics.row_padding(),
                column_gap: metrics.gap,
                bg_color: ${ state.motion().card_color(color_index.index()) },
                align: AlignItems::Center
            ) [
                FocusTarget::row(item.clone()),
            ] {
                View (
                    width: metrics.rail_width(),
                    height: Dimension::percent(100),
                    shrink: 0.0,
                    bg_color: ${
                        if available.available() && state.selected() == Some(accent_index.index()) {
                            ACCENT
                        } else {
                            DIVIDER
                        }
                    }
                )
                if !has_signal {
                    Text (
                        text: ${ if marker.directory() { ">" } else { "·" } },
                        width: if metrics.compact { 16 } else { 24 },
                        shrink: 0.0,
                        height: Dimension::Content,
                        font: font(metrics.title),
                        font_size: metrics.title,
                        text_color: ACCENT,
                        paragraph: paragraph(Center, 1)
                    ) [
                        FONT_PAINT,
                    ]
                }
                Column (grow: 1.0, min_width: 0, justify: JustifyContent::Center) {
                    row_label (item.clone(), metrics.row_title, true)
                    row_label (item.clone(), metrics.row_detail, false)
                }
                if has_signal {
                    row_signal (state, item, metrics)
                }
            }
            shadow (metrics)
        }
    }
}
