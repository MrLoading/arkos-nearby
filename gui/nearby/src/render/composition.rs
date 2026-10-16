use super::{
    pages::page,
    selection,
    state::{DisplayState, PageKey},
    style::*,
};
use crate::model::View;
use mirui::prelude::*;
use mirui::ui::widgets::page_view::{CreatePage, PageView};

#[event]
fn create_page(
    event: event!(CreatePage<PageKey>),
    state: model!(DisplayState),
    metrics: Metrics,
) -> Entity {
    let mut snapshot = state.snapshot();
    snapshot.page = event.route().page;
    snapshot.network_only = event.route().network_only;
    ui! {
        mounted_page (&snapshot, state, metrics)
    }
}

#[compose]
fn mounted_page(view: &View, state: model!(DisplayState), metrics: Metrics) -> Entity {
    ui! {
        Column (grow: 1.0, min_height: 0, bg_color: BACKGROUND, clip_children: true) {
            page_contents (view, state, metrics)
        }
    }
}

#[compose]
fn page_contents(view: &View, state: model!(DisplayState), metrics: Metrics) -> Entity {
    let body = ui! {
        page (view, state, metrics)
    };
    ui! {
        View (
            position: Position::Absolute,
            width: 0,
            height: 0,
            border_width: 0
        ) [
            mirui::ui::IgnoreHitTest,
            selection::SelectionFrame {
                body,
                page: state.page_key(),
                state: state.clone(),
            },
        ] {
            View (
                position: Position::Absolute,
                left: ${
                    Dimension::Percent(
                        Fixed::from_f32(-state.motion().expansion().0 as f32 * 100.0 / 276.0),
                    )
                },
                width: ${
                    Dimension::Percent(
                        Fixed::from_f32(100.0 + state.motion().expansion().0 as f32 * 200.0 / 276.0),
                    )
                },
                top: ${
                    Dimension::Percent(
                        Fixed::from_f32(-state.motion().expansion().1 as f32 * 100.0 / 196.0),
                    )
                },
                height: ${
                    Dimension::Percent(
                        Fixed::from_f32(100.0 + state.motion().expansion().1 as f32 * 200.0 / 196.0),
                    )
                },
                border_color: ACCENT,
                border_width: 2
            ) [
                mirui::ui::IgnoreHitTest,
            ]
        }
    }
}

#[compose]
pub(super) fn shell(state: model!(DisplayState), metrics: Metrics) -> Entity {
    ui! {
        PageView (
            id: "nearby_pages",
            grow: 1.0,
            min_height: 0,
            bg_color: BACKGROUND,
            page: ${ state.page_key() },
            transition: ${ state.transition() }
        ) on CreatePage(create_page::callback(state, metrics))
    }
}
