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
pub(super) const BACKGROUND: ColorToken = ColorToken::Surface;
pub(super) const FOREGROUND: ColorToken = ColorToken::OnSurface;
pub(super) const ACCENT: ColorToken = ColorToken::Primary;
pub(super) const ON_ACCENT: ColorToken = ColorToken::OnPrimary;
pub(super) const PANEL: ColorToken = ColorToken::SurfaceVariant;
pub(super) const SECONDARY: ColorToken = ColorToken::OnSurfaceVariant;
pub(super) const DISABLED: ColorToken = SECONDARY;
pub(super) const DIVIDER: ColorToken = ColorToken::Outline;
pub(super) const ACCENT_DIM: ColorToken = ColorToken::Custom("sky");
pub(super) const WARNING: ColorToken = ColorToken::Custom("gold");
pub(super) const WARNING_PANEL: ColorToken = PANEL;
pub(super) const CLAY: ColorToken = ColorToken::Custom("clay");
pub(super) const SKY: ColorToken = ColorToken::Custom("sky");
pub(super) const HILL: ColorToken = ColorToken::Custom("hill");
pub(super) const DEEP: ColorToken = ColorToken::Custom("deep");
pub(super) const HEADER_TEXT: ColorToken = ColorToken::Custom("header-text");
pub(super) const HEADER_DETAIL: ColorToken = ColorToken::Custom("header-detail");
#[derive(Clone, Copy)]
pub(super) struct Metrics {
    pub(super) compact: bool,
    pub(super) tiny: bool,
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
            tiny: height < 300,
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
