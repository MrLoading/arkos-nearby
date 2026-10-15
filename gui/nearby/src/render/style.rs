use mirui::prelude::*;
pub(super) const FONT_PAINT: mirui::ui::dirty::PaintInflate =
    mirui::ui::dirty::PaintInflate::uniform(Fixed::ONE);

pub(super) const FONT: FontToken = FontToken::Custom("nearby_menu");
pub(super) const TITLE_FONT: FontToken = FontToken::Custom("nearby_title");
pub(super) const TITLE_SIZE: u16 = 24;

pub(super) fn font(size: u16) -> FontToken {
    match size {
        12 => FONT,
        TITLE_SIZE => TITLE_FONT,
        _ => unreachable!("Pixel text requires a native font size"),
    }
}
pub(super) const BACKGROUND: Color = Color::rgb(242, 235, 221);
pub(super) const PANEL: Color = Color::rgb(231, 221, 200);
pub(super) const ACCENT: Color = Color::rgb(166, 61, 50);
pub(super) const ACCENT_DIM: Color = Color::rgb(234, 207, 192);
pub(super) const FOREGROUND: Color = Color::rgb(41, 37, 32);
pub(super) const SECONDARY: Color = Color::rgb(112, 98, 83);
pub(super) const DISABLED: Color = Color::rgb(131, 117, 99);
pub(super) const DIVIDER: Color = Color::rgb(204, 192, 169);
pub(super) const WARNING: Color = Color::rgb(152, 106, 32);
pub(super) const WARNING_PANEL: Color = Color::rgb(233, 218, 186);
#[derive(Clone, Copy)]
pub(super) struct Metrics {
    pub(super) compact: bool,
    pub(super) title: u16,
    pub(super) body: u16,
    pub(super) small: u16,
    pub(super) row_title: u16,
    pub(super) row_detail: u16,
    pub(super) gap: i32,
    pub(super) inset: i32,
}

impl Metrics {
    pub(super) fn for_height(height: u16) -> Self {
        let compact = height < 400;
        Self {
            compact,
            title: if compact { 12 } else { TITLE_SIZE },
            body: 12,
            small: 12,
            row_title: if compact { 12 } else { 24 },
            row_detail: 12,
            gap: if compact { 3 } else { 8 },
            inset: if compact { 8 } else { 18 },
        }
    }
}
