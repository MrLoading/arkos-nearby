use super::pixel::Stage::Search;
use super::selection::FocusTarget;
use super::{
    Section,
    pixel::{self, Scenery, art},
    rows::Row as ListRow,
    state::DisplayState,
    style::*,
    widgets::{caption, label, paragraph, shadow},
};
use mirui::prelude::*;
use mirui::ui::widgets::Text;
use mirui::ui::widgets::TextAlign::{Center, Start};

#[compose]
fn row_signal<T: RowLabel>(
    state: model!(DisplayState),
    row: ListRow<T>,
    metrics: Metrics,
) -> Entity {
    ui! {
        View (width: 30, height: if metrics.compact { 26 } else { 30 }, shrink: 0.0) {
            match ${ (row.index.get(), row.data.get().signal(), row.data.get().available()) } {
                values => {
                    View (grow: 1.0, width: Dimension::percent(100)) [
                        pixel::SignalSprite {
                            state: state.clone(),
                            index: values.0,
                            strength: values.1,
                            available: values.2,
                        },
                    ]
                }
            }
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
                    walk ${ state.rooms() } with item by item.token {
                        list_row (state, item, metrics)
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
                    walk ${ state.entries() } with item by item.token {
                        list_row (state, item, metrics)
                    }
                }
            }
        }
    }
}

pub(super) trait RowLabel: super::rows::RowData {
    const HAS_SIGNAL: bool = false;
    fn marker(&self) -> &'static str {
        ""
    }
    fn signal(&self) -> u8 {
        0
    }
    fn label(&self, title: bool) -> String;
    fn available(&self) -> bool {
        true
    }
}

impl RowLabel for crate::model::RoomRow {
    const HAS_SIGNAL: bool = true;
    fn signal(&self) -> u8 {
        self.signal
    }
    fn label(&self, title: bool) -> String {
        if title {
            self.name.clone()
        } else if self.available {
            self.detail.clone()
        } else {
            "房间已离线，等待重新出现".into()
        }
    }
    fn available(&self) -> bool {
        self.available
    }
}

impl RowLabel for crate::model::GameRow {
    fn marker(&self) -> &'static str {
        if self.directory { ">" } else { "·" }
    }
    fn label(&self, title: bool) -> String {
        if title {
            self.name.clone()
        } else {
            self.detail.clone()
        }
    }
}

#[compose]
fn row_label<T: RowLabel>(row: ListRow<T>, size: u16, title: bool) -> Entity {
    let content = row.data.clone();
    let colors = row.data;
    ui! {
        caption (
            move || content.get().label(title),
            move || {
                if !colors.get().available() {
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
fn list_row<T: RowLabel>(
    state: model!(DisplayState),
    item: ListRow<T>,
    metrics: Metrics,
) -> Entity {
    let color_index = item.index.clone();
    let transform_index = item.index.clone();
    let accent_index = item.index.clone();
    let available = item.data.clone();
    let marker = item.data.clone();
    ui! {
        Column (
            grow: 1.0,
            min_height: metrics.row_min_height(),
            max_height: metrics.row_height(),
            min_width: 0,
            transform: ${ state.motion().card_transform(transform_index.get()) }
        ) {
            Row (
                grow: 1.0,
                min_height: 0,
                padding: metrics.row_padding(),
                column_gap: metrics.gap,
                bg_color: ${ state.motion().card_color(color_index.get()) },
                align: AlignItems::Center
            ) [
                FocusTarget::Row(item.index.clone()),
            ] {
                View (
                    width: metrics.rail_width(),
                    height: Dimension::percent(100),
                    shrink: 0.0,
                    bg_color: ${
                        if available.get().available() && state.selected() == Some(accent_index.get()) {
                            ACCENT
                        } else {
                            DIVIDER
                        }
                    }
                )
                if !T::HAS_SIGNAL {
                    Text (
                        text: ${ marker.get().marker() },
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
                if T::HAS_SIGNAL {
                    row_signal (state, item, metrics)
                }
            }
            shadow (metrics)
        }
    }
}
