use crate::model::View;
use copy::loading;
use mirui::prelude::*;
use mirui::surface::FramebufferAccess;
use mirui::ui::widgets::page_view;
use state::{DisplayHandle, PageKey};
use style::{ACCENT_DIM, BACKGROUND, Metrics};

mod choices;
mod chrome;
mod composition;
mod copy;
mod font;
mod lists;
mod pages;
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

pub fn install<B: FramebufferAccess>(app: &mut App<B>) -> Result<(), String> {
    app.with_offscreen_pool_budget(2 * 1024 * 1024);
    app.with_widget(page_view::view::<PageKey>());
    app.with_widget(selection::target::view());
    app.with_widget(selection::frame::view());
    font::install(app)?;
    state::install(app);
    Ok(())
}

pub struct Scene {
    pub loading: bool,
    state: DisplayHandle,
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
}

pub fn update(scene: &mut Scene, view: &View) -> Result<(), String> {
    scene.state.present(view);
    scene.loading = loading(view);

    Ok(())
}

pub fn build<B: FramebufferAccess>(app: &mut App<B>, view: &View) -> Result<Scene, String> {
    let root = match app.root {
        Some(root) => root,
        None => app.spawn_root().bg_color(BACKGROUND).id(),
    };
    let state = app.compose(root, state::display_state());
    state.present(view);
    state.feedback(
        ACCENT_DIM,
        [if loading(view) {
            ACCENT_DIM
        } else {
            BACKGROUND
        }; 3],
    );
    if !app.compose(root, has_pages()) {
        let metrics = Metrics::for_height(app.backend.viewport().logical_size().1);
        app.compose(root, composition::shell(state.clone(), metrics));
    }
    Ok(Scene {
        loading: loading(view),
        state,
    })
}

#[compose]
fn has_pages() -> bool {
    res!(mirui::ui::IdMap).with(|ids| ids.get("nearby_pages").is_some())
}

pub fn draw<B: FramebufferAccess>(app: &mut App<B>, view: &View) -> Result<Scene, String> {
    let scene = build(app, view)?;
    app.render()
        .map_err(|error| format!("Menu frame: {error:?}"))?;
    Ok(scene)
}

#[cfg(test)]
#[path = "render_tests.rs"]
mod tests;
