use super::{
    Section,
    cache::GpuTextCache,
    choices::choice,
    chrome::{footer, header},
    lists::{games, rooms},
    sessions::{host, joined, leaving, network_panel, unavailable},
    state::DisplayState,
    style::*,
    widgets::paragraph,
};
use crate::model::{Page, View};
use mirui::prelude::*;
use mirui::ui::widgets::{Text, TextAlign::Start};

#[compose]
pub(super) fn page(view: &View, state: model!(DisplayState), metrics: Metrics) -> Entity {
    ui! {
        Column (
            grow: 1.0,
            min_width: 0,
            min_height: 0,
            bg_color: BACKGROUND,
            padding: Padding::all(if metrics.compact { 8 } else { 24 }),
            row_gap: metrics.gap
        ) [
            Section::Shell,
        ] {
            header (
                view,
                state,
                metrics
            )
            View (
                height: 1,
                shrink: 0.0,
                bg_color: DIVIDER
            )
            Column (
                grow: 1.0,
                min_height: 0,
                row_gap: metrics.gap
            ) [
                Section::Content,
            ] {
                Row (
                    height: i32::from(metrics.title) + 8,
                    shrink: 0.0,
                    align: AlignItems::Center,
                    column_gap: metrics.gap
                ) {
                    Text (
                        text: ${ state.heading() },
                        grow: 1.0,
                        min_width: 0,
                        height: Dimension::Content,
                        min_height: i32::from(metrics.title) + 8,
                        font: FONT,
                        font_size: metrics.title,
                        text_color: FOREGROUND,
                        paragraph: paragraph(Start, 1)
                    ) [
                        FONT_PAINT,
                        GpuTextCache,
                    ]
                    Row (
                        width: 36,
                        shrink: 0.0,
                        align: AlignItems::Center,
                        justify: JustifyContent::SpaceBetween
                    ) {
                        walk 0_usize..3 with index {
                            View (
                                width: 6,
                                height: 6,
                                bg_color: ${ state.activity()[index] }
                            )
                        }
                    }
                }
                Text (
                    text: ${ state.status() },
                    height: Dimension::Content,
                    min_height: i32::from(metrics.small) + 6,
                    shrink: 0.0,
                    width: Dimension::percent(100),
                    min_width: 0,
                    font: FONT,
                    font_size: metrics.small,
                    text_color: SECONDARY,
                    paragraph: paragraph(Start, 2),
                    clip_children: true
                ) [
                    FONT_PAINT,
                    Section::Status,
                    GpuTextCache,
                ]
                if view.network_only && matches!(view.page, Page::Host | Page::Joined) {
                    network_panel (view, state, metrics)
                } else {
                    match view.page {
                        Page :: Choice | Page :: Mode => {
                            choice (view, metrics)
                        }
                        Page :: Host => {
                            host (view, state, metrics)
                        }
                        Page :: Rooms => {
                            rooms (state, metrics)
                        }
                        Page :: Joined => {
                            joined (view, state, metrics)
                        }
                        Page :: Leaving => {
                            leaving (state, metrics)
                        }
                        Page :: Unavailable => {
                            unavailable (metrics)
                        }
                        Page :: Games => {
                            games (state, metrics)
                        }
                    }
                }
            }
            View (
                height: 1,
                shrink: 0.0,
                bg_color: DIVIDER
            )
            footer (
                view,
                state,
                metrics
            )
        }
    }
}
