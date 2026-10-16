use super::{
    Section, StaticCache,
    cache::GpuTextCache,
    pixel::Scenery,
    state::DisplayState,
    style::*,
    widgets::{keycap, label, paragraph},
};
use crate::model::{Page, View};
use mirui::prelude::*;
use mirui::ui::OffscreenRender;
use mirui::ui::widgets::Text;
use mirui::ui::widgets::TextAlign::{Center, End, Start};

#[compose]
pub(super) fn header(view: &View, state: model!(DisplayState), metrics: Metrics) -> Entity {
    let step = if view.page == Page::Mode {
        "选择模式".into()
    } else if view.page == Page::Games {
        "选择游戏".into()
    } else {
        format!("步骤 {} / 2", if view.page == Page::Choice { 1 } else { 2 })
    };
    ui! {
        Row (
            height: if metrics.tiny { 44 } else if metrics.compact { 52 } else { 88 },
            shrink: 0.0,
            column_gap: metrics.gap + 4,
            padding: Padding {
                top: 4.into(),
                bottom: 4.into(),
                left: (if metrics.compact { 16 } else { 24 }).into(),
                right: (if metrics.compact { 16 } else { 24 }).into(),
            },
            bg_color: DEEP,
            align: AlignItems::Center
        ) [
            Section::Header,
            Scenery::Header,
        ] {
            View (
                width: if metrics.compact { 30 } else { 48 },
                height: if metrics.compact { 30 } else { 48 },
                shrink: 0.0
            ) [
                Scenery::Logo,
                StaticCache,
                OffscreenRender::default(),
            ]
            Column (
                grow: 1.0,
                min_width: 0,
                justify: JustifyContent::Center,
                clip_children: true
            ) [
                StaticCache,
                OffscreenRender::default(),
            ] {
                label (
                    "附近联机",
                    metrics.title,
                    HEADER_TEXT,
                    Start
                )
                Text (
                    text: ${ state.subtitle() },
                    font: font(metrics.small),
                    font_size: metrics.small,
                    text_color: HEADER_DETAIL,
                    height: Dimension::Content,
                    paragraph: paragraph(Start, 1)
                ) [
                    FONT_PAINT,
                ]
            }
            Text (
                step,
                width: if metrics.compact { 76 } else { 96 },
                height: if metrics.compact { 26 } else { 34 },
                shrink: 0.0,
                bg_color: DEEP,
                font: font(metrics.small),
                font_size: metrics.small,
                text_color: HEADER_DETAIL,
                paragraph: paragraph(Center, 1)
            ) [
                FONT_PAINT,
                GpuTextCache,
            ]
        }
    }
}

#[compose]
pub(super) fn footer(view: &View, state: model!(DisplayState), metrics: Metrics) -> Entity {
    ui! {
        Row (
            height: if metrics.compact { 20 } else { 32 },
            shrink: 0.0,
            column_gap: metrics.gap,
            align: AlignItems::Center
        ) [
            Section::Footer,
            GpuTextCache,
        ] {
            Text (
                text: ${ if state.confirm_visible() { "A" } else { "" } },
                width: if metrics.compact { 20 } else { 25 },
                height: if metrics.compact { 20 } else { 25 },
                shrink: 0.0,
                bg_color: ${ if state.confirm_visible() { ACCENT } else { BACKGROUND } },
                font: font(metrics.small),
                font_size: metrics.small,
                text_color: ON_ACCENT,
                paragraph: paragraph(Center, 1)
            ) [
                FONT_PAINT,
            ]
            Text (
                text: ${ state.confirm() },
                width: Dimension::Content,
                shrink: 0.0,
                height: Dimension::Content,
                font: font(metrics.small),
                font_size: metrics.small,
                text_color: SECONDARY,
                paragraph: paragraph(Start, 1)
            ) [
                FONT_PAINT,
            ]
            keycap ("B", metrics)
            Text (
                text: ${ state.back() },
                max_width: if metrics.compact { 130 } else { 210 },
                min_width: 0,
                height: Dimension::Content,
                font: font(metrics.small),
                font_size: metrics.small,
                text_color: SECONDARY,
                paragraph: paragraph(Start, 1)
            ) [
                FONT_PAINT,
            ]
            if view.page == Page::Rooms {
                keycap ("X", metrics)
                Text (
                    "刷新",
                    height: Dimension::Content,
                    font: font(metrics.small),
                    font_size: metrics.small,
                    text_color: SECONDARY,
                    paragraph: paragraph(Start, 1),
                    clip_children: true
                ) [
                    FONT_PAINT,
                ]
            }
            if matches!(view.page, Page::Mode | Page::Choice) {
                keycap ("Y", metrics)
                Text (
                    "主题",
                    width: Dimension::Content,
                    height: Dimension::Content,
                    shrink: 0.0,
                    font: font(metrics.small),
                    font_size: metrics.small,
                    text_color: SECONDARY,
                    paragraph: paragraph(Start, 1)
                ) [
                    FONT_PAINT,
                ]
            }
            Text (
                text: ${ state.hint() },
                grow: 1.0,
                min_width: 0,
                height: Dimension::Content,
                font: font(metrics.small),
                font_size: metrics.small,
                text_color: SECONDARY,
                paragraph: paragraph(End, 1),
                clip_children: true
            ) [
                FONT_PAINT,
            ]
        }
    }
}
