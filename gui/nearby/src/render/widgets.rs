use super::{
    StaticCache,
    state::{DisplayState, TextField},
    style::*,
};
use TextAlign::Center;
use mirui::prelude::*;
use mirui::ui::OffscreenRender;
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
pub(super) fn label(value: &str, size: u16, color: Color, align: TextAlign) -> Entity {
    ui! {
        Text (
            value.to_owned(),
            width: Dimension::percent(100),
            height: Dimension::Content,
            min_width: 0,
            min_height: i32::from(size) + 6,
            font: FONT,
            font_size: size,
            text_color: color,
            paragraph: paragraph(align, 1),
            clip_children: true
        ) [
            FONT_PAINT,
        ]
    }
}

#[compose]
pub(super) fn live_label(
    state: model!(DisplayState),
    field: TextField,
    size: u16,
    color: Color,
    align: TextAlign,
) -> Entity {
    ui! {
        Text (
            text: ${ state.text(field) },
            width: Dimension::percent(100),
            height: Dimension::Content,
            min_width: 0,
            min_height: i32::from(size) + 6,
            font: FONT,
            font_size: size,
            text_color: color,
            paragraph: paragraph(align, 1),
            clip_children: true
        ) [
            FONT_PAINT,
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
            font: FONT,
            font_size: metrics.small,
            text_color: SECONDARY,
            paragraph: paragraph(Center, 1)
        ) [
            FONT_PAINT,
        ]
    }
}

#[compose]
pub(super) fn avatar(
    number: &str,
    state: model!(DisplayState),
    own: bool,
    metrics: Metrics,
) -> Entity {
    ui! {
        Text (
            number.to_owned(),
            width: if metrics.compact { 36 } else { 60 },
            height: if metrics.compact { 36 } else { 60 },
            bg_color: ${ if own || state.ready() { ACCENT_DIM } else { PANEL } },
            font: FONT,
            font_size: metrics.title,
            text_color: ${ if own || state.ready() { ACCENT } else { DISABLED } },
            paragraph: paragraph(Center, 1)
        ) [
            FONT_PAINT,
            StaticCache,
            OffscreenRender::default(),
        ]
    }
}
