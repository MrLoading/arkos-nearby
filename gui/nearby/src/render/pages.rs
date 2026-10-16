use super::state::PageKey;
use super::{
    Section,
    cache::GpuLayerCache,
    choices::choice,
    chrome::{footer, header},
    lists::{games, rooms},
    pixel::Scenery,
    sessions::{host, joined, leaving, network_panel, unavailable},
    state::DisplayState,
    style::*,
    widgets::paragraph,
};
use crate::model::Page;
use mirui::prelude::*;
use mirui::ui::widgets::{Text, TextAlign::Start};

#[compose]
pub(super) fn page(view: PageKey, state: model!(DisplayState), metrics: Metrics) -> Entity {
    ui! {
        Column (
            grow: 1.0,
            min_width: 0,
            min_height: 0,
            bg_color: BACKGROUND
        ) [
            Section::Shell,
        ] {
            header (
                view,
                state,
                metrics
            )
            View (
                height: metrics.rule_height(),
                shrink: 0.0,
                bg_color: WARNING
            )
            Column (
                grow: 1.0,
                min_height: 0,
                padding: metrics.page_padding(),
                row_gap: metrics.gap
            ) [
                Scenery::Margins,
            ] {
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
                            font: font(metrics.title),
                            font_size: metrics.title,
                            text_color: FOREGROUND,
                            paragraph: paragraph(Start, 1)
                        ) [
                            FONT_PAINT,
                            GpuLayerCache,
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
                        min_height: i32::from(metrics.small) + LABEL_LEADING,
                        shrink: 0.0,
                        width: Dimension::percent(100),
                        min_width: 0,
                        font: font(metrics.small),
                        font_size: metrics.small,
                        text_color: SECONDARY,
                        paragraph: paragraph(Start, 2),
                        clip_children: true
                    ) [
                        FONT_PAINT,
                        Section::Status,
                        GpuLayerCache,
                    ]
                    if view.network_only && matches!(view.page, Page::Host | Page::Joined) {
                        network_panel (state, metrics)
                    } else {
                        match view.page {
                            Page :: Choice | Page :: Mode => {
                                choice (view, state, metrics)
                            }
                            Page :: Host => {
                                host (state, metrics)
                            }
                            Page :: Rooms => {
                                rooms (state, metrics)
                            }
                            Page :: Joined => {
                                joined (state, metrics)
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
}
