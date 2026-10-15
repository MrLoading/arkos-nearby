use super::style::FONT;
use mirui::app::RendererFactory;
use mirui::prelude::*;
use mirui::render::font::{Font, FontBackend, FontManager, mirx::MirxFontProvider};
use mirui::surface::Surface;
use std::rc::Rc;
const FONT_BYTES: &[u8] = include_bytes!("../../assets/nearby-menu-20.mirx");

pub(super) fn install<B: Surface, F: RendererFactory<B>>(
    app: &mut App<B, F>,
) -> Result<(), String> {
    let limits = mirx::reader::PayloadLimits::HOST;
    let options = mirx::reader::ReadOptions::new().with_payload_limits(limits);
    let reader = mirx::Reader::open_with(FONT_BYTES, &options)
        .map_err(|error| format!("Menu font container: {error:?}"))?;
    let mut faces = reader
        .chunks()
        .filter(|chunk| chunk.chunk_type() == mirx::ChunkType::FONT);
    let face = faces.next().ok_or("The menu asset contains no font")?;
    if faces.next().is_some() {
        return Err("The menu asset contains multiple fonts".into());
    }
    let provider = MirxFontProvider::from_payload(face.payload(), &limits)
        .map_err(|error| format!("Menu font face: {error:?}"))?;
    let font = Font {
        family: "Nearby Menu",
        size: 20,
        backend: FontBackend::Custom(Rc::new(provider)),
    };
    let fonts = FontManager::new(mirui::core::cache::MaxSize::Unbound, Font::bitmap_8x8());
    fonts.add_static(FONT.cache_key(), font);
    app.with_fonts(fonts);
    Ok(())
}
