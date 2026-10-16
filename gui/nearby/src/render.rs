use crate::model::View;
use copy::loading;
use mirui::app::RendererFactory;
use mirui::prelude::*;
use mirui::surface::Surface;
use mirui::ui::widgets::page_view;
use state::{DisplayHandle, PageKey};
use style::{ACCENT_DIM, BACKGROUND, Metrics};

mod cache;
mod choices;
mod chrome;
mod composition;
mod copy;
mod font;
mod lists;
mod motion;
mod pages;
mod pixel;
mod selection;
mod sessions;
mod state;
mod style;
mod widgets;

#[component]
struct StaticCache;

#[component]
#[derive(Clone, Copy, PartialEq, Eq)]
enum Section {
    Shell,
    Header,
    Content,
    Footer,
    Status,
    FocusList,
    FocusHost,
}

pub fn install<B: Surface, F: RendererFactory<B>>(app: &mut App<B, F>) -> Result<(), String> {
    app.with_theme(crate::theme::Palette::default().theme());
    app.with_offscreen_pool_budget(2 * 1024 * 1024);
    app.with_widget(page_view::view::<PageKey>());
    app.with_widget(selection::target::view());
    app.with_widget(selection::frame::view());
    app.with_widget(pixel::sprite::view());
    app.with_widget(pixel::scenery::view());
    app.with_widget(cache::text_cache::view());
    font::install(app)?;
    state::install(app);
    Ok(())
}

pub struct Scene {
    pub loading: bool,
    state: DisplayHandle,
    pub colors: mirui::ui::theme::Theme,
}

impl Scene {
    #[cfg(any(target_os = "linux", test))]
    pub fn navigation_intent(&self, backward: bool) {
        if backward {
            self.state.navigate_back();
        } else {
            self.state.navigate_forward();
        }
    }

    pub fn feedback(&self, stamp: Color, activity: [Color; 3]) {
        self.state.feedback(stamp, activity);
    }

    pub fn animate(&self, now: u64, press_sequence: u64) {
        self.state.animate(now, press_sequence);
    }
}

pub fn update(scene: &mut Scene, view: &View) -> Result<(), String> {
    scene.state.present(view);
    scene.loading = loading(view);

    Ok(())
}

#[cfg(any(target_os = "linux", test))]
pub fn cycle_theme<B: Surface, F: RendererFactory<B>>(
    app: &mut App<B, F>,
    scene: &mut Scene,
) -> Result<Option<crate::theme::Palette>, String> {
    use crate::theme::Palette;
    if !Palette::can_switch(&scene.state.snapshot()) {
        return Ok(None);
    }
    let palette = Palette::parse(scene.colors.id().as_str())
        .unwrap_or_default()
        .next();
    app.set_theme(palette.theme())
        .map_err(|error| format!("Theme: {error:?}"))?;
    scene.colors = app.theme().clone();
    Ok(Some(palette))
}

pub fn build<B: Surface, F: RendererFactory<B>>(
    app: &mut App<B, F>,
    view: &View,
) -> Result<Scene, String> {
    let root = match app.root {
        Some(root) => root,
        None => app.spawn_root().bg_color(BACKGROUND).id(),
    };
    let state = app.compose(root, state::display_state());
    state.present(view);
    state.animate(0, 0);
    state.feedback(
        app.theme().resolve(ACCENT_DIM),
        [if loading(view) {
            app.theme().resolve(ACCENT_DIM)
        } else {
            app.theme().resolve(BACKGROUND)
        }; 3],
    );
    if !app.compose(root, has_pages()) {
        let metrics = Metrics::for_height(app.backend.viewport().logical_size().1);
        app.compose(root, composition::shell(state.clone(), metrics));
    }
    Ok(Scene {
        loading: loading(view),
        state,
        colors: app.theme().clone(),
    })
}

#[compose]
fn has_pages() -> bool {
    res!(mirui::ui::IdMap).with(|ids| ids.get("nearby_pages").is_some())
}

pub fn draw<B: Surface, F: RendererFactory<B>>(
    app: &mut App<B, F>,
    view: &View,
) -> Result<Scene, String> {
    let scene = build(app, view)?;
    app.render()
        .map_err(|error| format!("Menu frame: {error:?}"))?;
    Ok(scene)
}

#[cfg(test)]
#[path = "render_tests.rs"]
mod tests;
