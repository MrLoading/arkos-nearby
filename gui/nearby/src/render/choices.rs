use super::pixel::Kind::Walker;
use super::{
    Section,
    cache::GpuTextCache,
    pixel::{self, Kind, Scenery, art},
    selection::FocusTarget,
    state::DisplayState,
    style::*,
    widgets::{label, paragraph, shadow},
};
use crate::model::{Page, View};
use mirui::prelude::*;
use mirui::ui::widgets::Text;
use mirui::ui::widgets::TextAlign::Center;

#[compose]
pub(super) fn choice(view: &View, state: model!(DisplayState), metrics: Metrics) -> Entity {
    let cards = if view.page == Page::Mode {
        [
            (0, "仅组网", "连接附近的掌机"),
            (1, "选择游戏", "选择要一起玩的游戏"),
        ]
    } else if view.network_only {
        [
            (0, "创建房间", "邀请好友连接你的掌机"),
            (1, "加入房间", "寻找附近好友的房间"),
        ]
    } else {
        [
            (0, "创建房间", "邀请好友加入你的房间"),
            (1, "加入房间", "寻找附近好友的房间"),
        ]
    };
    ui! {
        Column (grow: 1.0, min_height: 0, row_gap: metrics.gap) {
            Row (grow: 1.0, min_height: 0, column_gap: metrics.gap + 14) [
                Section::FocusList,
            ] {
                walk cards.into_iter() with card {
                    Column (
                        grow: 1.0,
                        min_width: 0,
                        min_height: 0,
                        transform: ${ state.motion().card_transform(card.0) }
                    ) {
                        Column (
                            grow: 1.0,
                            min_width: 0,
                            min_height: 0,
                            padding: Padding {
                                top: 2.into(),
                                bottom: (if metrics.compact { 4 } else { 8 }).into(),
                                left: 2.into(),
                                right: 2.into(),
                            },
                            row_gap: metrics.gap,
                            bg_color: ${ state.motion().card_color(card.0) }
                        ) [
                            FocusTarget { index: card.0 },
                        ] {
                            View (
                                grow: 1.0,
                                min_height: if metrics.tiny { 28 } else if metrics.compact { 40 } else { 80 },
                                clip_children: true
                            ) [
                                Scenery::Landscape,
                                pixel::Sprite {
                                    state: state.clone(),
                                    kind: Kind::Choice {
                                        index: card.0,
                                        mode: view.page == Page::Mode,
                                    },
                                },
                            ] {
                                Text (
                                    if card.0 == 0 { "+" } else { "2" },
                                    position: Position::Absolute,
                                    left: 6,
                                    top: 6,
                                    width: 24,
                                    height: 20,
                                    font: font(12),
                                    font_size: 12,
                                    bg_color: BACKGROUND,
                                    text_color: SECONDARY,
                                    paragraph: paragraph(Center, 1)
                                ) [
                                    FONT_PAINT,
                                    GpuTextCache,
                                ]
                            }
                            label (card.1, metrics.title, FOREGROUND, Center)
                            if !metrics.tiny {
                                Text (
                                    card.2,
                                    width: Dimension::percent(100),
                                    min_width: 0,
                                    height: Dimension::Content,
                                    font: font(metrics.small),
                                    font_size: metrics.small,
                                    text_color: SECONDARY,
                                    paragraph: paragraph(Center, 2)
                                ) [
                                    FONT_PAINT,
                                    GpuTextCache,
                                ]
                            }
                        }
                        shadow (metrics)
                    }
                }
            }
            art (state, Walker, if metrics.tiny { 22 } else if metrics.compact { 38 } else { 46 })
        }
    }
}
