use super::selection::FocusTarget;
use super::{
    Section, StaticCache,
    state::DisplayState,
    style::*,
    widgets::{label, paragraph},
};
use mirui::prelude::*;
use mirui::ui::OffscreenRender;
use mirui::ui::widgets::Text;
use mirui::ui::widgets::TextAlign::{Center, Start};

#[compose]
pub(super) fn room_signal(strength: u8, available: bool) -> Entity {
    ui! {
        Row (
            width: 30,
            height: 24,
            shrink: 0.0,
            column_gap: 3,
            align: AlignItems::FlexEnd
        ) {
            walk 0..4 with index {
                View (
                    width: 4,
                    height: 6 + index * 4,
                    bg_color: if available && strength as i32 > index * 25 { ACCENT } else { DIVIDER }
                )
            }
        }
    }
}

#[compose]
pub(super) fn rooms(state: model!(DisplayState), metrics: Metrics) -> Entity {
    ui! {
        Column (grow: 1.0, min_height: 0) {
            match ${ state.rooms() } {
                rows => {
                    room_rows (&rows, metrics)
                }
            }
        }
    }
}

#[compose]
pub(super) fn room_rows(rows: &[(usize, crate::model::RoomRow)], metrics: Metrics) -> Entity {
    ui! {
        Column (
            grow: 1.0,
            min_height: 0,
            row_gap: metrics.gap
        ) [
            Section::FocusList,
            StaticCache,
            OffscreenRender::default(),
        ] {
            if rows.is_empty() {
                Column (
                    grow: 1.0,
                    bg_color: PANEL,
                    padding: Padding::all(metrics.inset),
                    justify: JustifyContent::Center,
                    row_gap: metrics.gap
                ) {
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
            } else {
                walk rows.iter() with item {
                    Row (
                        grow: 1.0,
                        min_height: i32::from(metrics.row_title + metrics.row_detail) + 17,
                        max_height: if metrics.compact { 42 } else { 70 },
                        padding: Padding::all(2),
                        column_gap: metrics.gap,
                        bg_color: PANEL,
                        align: AlignItems::Center,
                        clip_children: true
                    ) [
                        FocusTarget { index: item.0 },
                    ] {
                        Column (
                            grow: 1.0,
                            min_width: 0,
                            justify: JustifyContent::Center
                        ) {
                            label (
                                &item.1.name,
                                metrics.row_title,
                                if item.1.available { FOREGROUND } else { DISABLED },
                                Start
                            )
                            label (
                                if item.1.available {
                                    &item.1.detail
                                } else {
                                    "房间已离线，等待重新出现"
                                },
                                metrics.row_detail,
                                if item.1.available { SECONDARY } else { DISABLED },
                                Start
                            )
                        }
                        room_signal (
                            item.1.signal,
                            item.1.available
                        )
                    }
                }
                View (
                    grow: 1.0,
                    min_height: 0
                )
            }
        }
    }
}

#[compose]
pub(super) fn games(state: model!(DisplayState), metrics: Metrics) -> Entity {
    ui! {
        Column (grow: 1.0, min_height: 0) {
            match ${ state.entries() } {
                rows => {
                    game_rows (&rows, metrics)
                }
            }
        }
    }
}

#[compose]
pub(super) fn game_rows(rows: &[crate::model::GameRow], metrics: Metrics) -> Entity {
    ui! {
        Column (
            grow: 1.0,
            min_height: 0,
            row_gap: metrics.gap
        ) [
            Section::FocusList,
            StaticCache,
            OffscreenRender::default(),
        ] {
            if rows.is_empty() {
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
            } else {
                walk rows.iter().enumerate() with item {
                    Row (
                        grow: 1.0,
                        min_height: i32::from(metrics.row_title + metrics.row_detail) + 17,
                        max_height: if metrics.compact { 42 } else { 70 },
                        padding: Padding::all(2),
                        column_gap: metrics.gap,
                        bg_color: PANEL,
                        align: AlignItems::Center,
                        clip_children: true
                    ) [
                        FocusTarget { index: item.0 },
                    ] {
                        Text (
                            if item.1.directory { ">" } else { "·" },
                            width: if metrics.compact { 20 } else { 32 },
                            shrink: 0.0,
                            height: Dimension::Content,
                            font: font(metrics.title),
                            font_size: metrics.title,
                            text_color: ACCENT,
                            paragraph: paragraph(Center, 1)
                        ) [
                            FONT_PAINT,
                        ]
                        Column (
                            grow: 1.0,
                            min_width: 0,
                            justify: JustifyContent::Center
                        ) {
                            label (
                                &item.1.name,
                                metrics.row_title,
                                FOREGROUND,
                                Start
                            )
                            label (
                                &item.1.detail,
                                metrics.row_detail,
                                SECONDARY,
                                Start
                            )
                        }
                    }
                }
                View (
                    grow: 1.0,
                    min_height: 0
                )
            }
        }
    }
}
