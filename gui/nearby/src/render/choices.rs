use super::selection::FocusTarget;
use super::{
    Section, StaticCache,
    style::*,
    widgets::{label, paragraph},
};
use crate::model::{Page, View};
use mirui::prelude::*;
use mirui::ui::OffscreenRender;
use mirui::ui::widgets::Text;
use mirui::ui::widgets::TextAlign::{Center, Start};

#[compose]
pub(super) fn choice(view: &View, metrics: Metrics) -> Entity {
    let cards = if view.page == Page::Mode {
        [
            (0, "仅组网", "连接附近的掌机", "返回系统后保持连接"),
            (1, "选择游戏", "选择要一起玩的游戏", "创建或加入游戏房间"),
        ]
    } else if view.network_only {
        [
            (0, "创建房间", "邀请好友连接你的掌机", "你是房主"),
            (1, "加入房间", "寻找附近好友的房间", "建立局域网连接"),
        ]
    } else {
        [
            (0, "创建房间", "邀请好友加入你的房间", "你是房主"),
            (1, "加入房间", "寻找附近好友的房间", "一起开始游戏"),
        ]
    };
    ui! {
        Row (
            grow: 1.0,
            min_height: 0,
            column_gap: metrics.gap + 6
        ) [
            Section::FocusList,
            StaticCache,
            OffscreenRender::default(),
        ] {
            walk cards.into_iter() with card {
                Column (
                    grow: 1.0,
                    min_width: 0,
                    padding: Padding::all(metrics.inset),
                    row_gap: metrics.gap,
                    bg_color: PANEL,
                    border_radius: 0,
                    clip_children: true
                ) [
                    FocusTarget { index: card.0 },
                ] {
                    Row (
                        height: if metrics.compact { 28 } else { 44 },
                        align: AlignItems::Center
                    ) {
                        Text (
                            if card.0 == 0 { "+" } else { "2" },
                            width: if metrics.compact { 28 } else { 44 },
                            height: Dimension::percent(100),
                            bg_color: ACCENT_DIM,
                            font: font(metrics.title),
                            font_size: metrics.title,
                            text_color: ACCENT,
                            paragraph: paragraph(Center, 1)
                        ) [
                            FONT_PAINT,
                        ]
                    }
                    View (
                        grow: 1.0,
                        min_height: 0
                    )
                    label (
                        card.1,
                        metrics.title,
                        FOREGROUND,
                        Start
                    )
                    Text (
                        card.2,
                        width: Dimension::percent(100),
                        min_width: 0,
                        height: Dimension::Content,
                        font: font(metrics.small),
                        font_size: metrics.small,
                        text_color: SECONDARY,
                        paragraph: paragraph(Start, 2)
                    ) [
                        FONT_PAINT,
                    ]
                    if !metrics.compact {
                        label (
                            card.3,
                            metrics.small,
                            SECONDARY,
                            Start
                        )
                    }
                }
            }
        }
    }
}
