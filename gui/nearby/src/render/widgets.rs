use super::{
    cache::GpuTextCache,
    state::{DisplayState, TextField},
    style::*,
};
use TextAlign::Center;
use mirui::prelude::*;
use mirui::ui::widgets::{ParagraphStyle, Text, TextAlign, TextOverflow, TextWrap};

pub(super) fn paragraph(align: TextAlign, lines: u16) -> ParagraphStyle {
    ParagraphStyle {
        wrap: TextWrap::Word,
        align,
        vertical_align: mirui::ui::widgets::text::TextVerticalAlign::Center,
        overflow: TextOverflow::Ellipsis,
        max_lines: Some(lines),
        ..Default::default()
    }
}

#[compose]
pub(super) fn label(value: &str, size: u16, color: ColorToken, align: TextAlign) -> Entity {
    ui! {
        Text (
            value.to_owned(),
            width: Dimension::percent(100),
            height: Dimension::Content,
            min_width: 0,
            min_height: i32::from(size) + 6,
            font: font(size),
            font_size: size,
            text_color: color,
            paragraph: paragraph(align, 1),
            clip_children: true
        ) [
            FONT_PAINT,
            GpuTextCache,
        ]
    }
}

#[compose]
pub(super) fn live_label(
    state: model!(DisplayState),
    field: TextField,
    size: u16,
    color: ColorToken,
    align: TextAlign,
) -> Entity {
    ui! {
        Text (
            text: ${ state.text(field) },
            width: Dimension::percent(100),
            height: Dimension::Content,
            min_width: 0,
            min_height: i32::from(size) + 6,
            font: font(size),
            font_size: size,
            text_color: color,
            paragraph: paragraph(align, 1),
            clip_children: true
        ) [
            FONT_PAINT,
            GpuTextCache,
        ]
    }
}

#[compose]
pub(super) fn action_label(state: model!(DisplayState), size: u16) -> Entity {
    ui! {
        Text (
            text: ${ state.text(TextField::HostButton) },
            width: Dimension::percent(100),
            height: Dimension::Content,
            min_width: 0,
            min_height: i32::from(size) + 6,
            font: font(size),
            font_size: size,
            text_color: ${ if state.enabled() { ON_ACCENT } else { FOREGROUND } },
            paragraph: paragraph(Center, 1),
            clip_children: true
        ) [
            FONT_PAINT,
            GpuTextCache,
        ]
    }
}

#[compose]
pub(super) fn keycap(value: &str, metrics: Metrics) -> Entity {
    ui! {
        Text (
            value.to_owned(),
            width: if metrics.compact { 20 } else { 25 },
            height: if metrics.compact { 20 } else { 25 },
            shrink: 0.0,
            bg_color: PANEL,
            font: font(metrics.small),
            font_size: metrics.small,
            text_color: SECONDARY,
            paragraph: paragraph(Center, 1)
        ) [
            FONT_PAINT,
        ]
    }
}

#[compose]
pub(super) fn shadow(metrics: Metrics) -> Entity {
    ui! {
        View (height: if metrics.compact { 3 } else { 4 }, shrink: 0.0, bg_color: DIVIDER)
    }
}
