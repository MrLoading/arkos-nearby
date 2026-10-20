use crate::model::{LibraryKind, View};
use mirui::prelude::*;
use serde::Serialize;

pub(super) const MAX_VISIBLE: usize = 8;
pub(super) const INSET: i32 = 12;
pub(super) const GAP: i32 = 12;
pub(super) const CARD_INSET: i32 = 2;
pub(super) const LABEL_HEIGHT: i32 = 18;
pub(super) const LABEL_ACTOR_SPACE: i32 = 24;
pub(super) const SHADOW: i32 = 2;
const MIN_CARD_WIDTH: i32 = 164;
const MAX_CARD_WIDTH: i32 = 280;
const IMAGE_CHROME_WIDTH: i32 = 28;
const IMAGE_CHROME_HEIGHT: i32 = 40;
const PAGE_MARK_HEIGHT: i32 = 4;
const PAGE_MARK_GAP: i32 = 4;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Grid {
    pub columns: usize,
    pub rows: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct LayoutRequest {
    event: &'static str,
    launcher: bool,
    grid: Grid,
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub(super) struct Plan {
    pub grid: Grid,
    pub card_width: i32,
    pub card_height: i32,
}

impl Plan {
    fn fit(width: i32, height: i32, total: usize) -> Self {
        let available_width = (width - INSET * 2).max(1);
        let available_height = (height - INSET * 2 - PAGE_MARK_HEIGHT - PAGE_MARK_GAP).max(1);
        let mut best = Self {
            grid: Grid {
                columns: 1,
                rows: 1,
            },
            card_width: available_width.min(MAX_CARD_WIDTH),
            card_height: available_height,
        };
        let mut score = 0;
        for rows in 1..=2 {
            for columns in 1..=4 {
                if rows > 1 && total <= columns {
                    continue;
                }
                let row_height = (available_height - GAP * (rows as i32 - 1)) / rows as i32;
                let height_limit = (row_height - IMAGE_CHROME_HEIGHT) * 4 / 3 + IMAGE_CHROME_WIDTH;
                let card_width = ((available_width - GAP * (columns as i32 - 1)) / columns as i32)
                    .min(MAX_CARD_WIDTH)
                    .min(height_limit);
                if card_width < MIN_CARD_WIDTH {
                    continue;
                }
                let card_height = (card_width - IMAGE_CHROME_WIDTH) * 3 / 4 + IMAGE_CHROME_HEIGHT;
                let candidate =
                    (rows * columns).min(total.max(1)) as i32 * 1000 + card_width - rows as i32;
                if candidate > score {
                    score = candidate;
                    best = Self {
                        grid: Grid { columns, rows },
                        card_width,
                        card_height,
                    };
                }
            }
        }
        best
    }

    pub fn width(self) -> i32 {
        self.card_width * self.grid.columns as i32
            + GAP * self.grid.columns.saturating_sub(1) as i32
    }
}

#[model]
pub(super) struct Shelf {
    launcher: bool,
    active: bool,
    size: (i32, i32),
    layout_pending: bool,
    total: usize,
    position: usize,
    #[observe]
    pub plan: Plan,
    sent: Option<Grid>,
    pending: Option<LayoutRequest>,
    #[observe]
    pub pages: (usize, usize, usize),
}

impl Shelf {
    pub fn new(launcher: bool) -> Self {
        Self {
            launcher,
            active: false,
            size: (0, 0),
            layout_pending: false,
            total: 0,
            position: 0,
            plan: Plan::default(),
            sent: None,
            pending: None,
            pages: (0, 0, 0),
        }
    }

    fn update(&mut self) {
        if !self.active || self.size.0 <= 0 || self.size.1 <= 0 {
            return;
        }
        self.plan = Plan::fit(self.size.0, self.size.1, self.total);
        if self.sent != Some(self.plan.grid) {
            self.sent = Some(self.plan.grid);
            self.pending = Some(LayoutRequest {
                event: "shelf_layout",
                launcher: self.launcher,
                grid: self.plan.grid,
            });
        }
        let capacity = self.plan.grid.columns * self.plan.grid.rows;
        let page = self.position.saturating_sub(1) / capacity;
        let count = self.total.div_ceil(capacity);
        let first = page.saturating_sub(3).min(count.saturating_sub(7));
        self.pages = if count > 1 {
            (first, count.min(first + 7), page)
        } else {
            (0, 0, 0)
        };
    }
}

#[model]
impl Shelf {
    pub fn present(&mut self, view: &View) {
        let active =
            view.library.kind == LibraryKind::Favorites && view.launcher.enabled == self.launcher;
        if self.active != active {
            self.sent = None;
            self.pending = None;
        }
        self.active = active;
        self.total = view.library.total.max(view.entries.len());
        self.position = view
            .library
            .position
            .max(view.selected.saturating_add(1))
            .min(self.total);
        self.update();
    }

    pub fn set_size(&mut self, width: Fixed, height: Fixed) {
        let size = (width.to_int(), height.to_int());
        if self.size != size {
            self.size = size;
            self.layout_pending = true;
        }
    }

    pub fn layout(&mut self) {
        if self.layout_pending {
            self.layout_pending = false;
            self.update();
        }
    }

    #[effects]
    fn take_layout(&mut self) -> Option<LayoutRequest> {
        self.pending.take()
    }
}

#[system(order = Animation)]
pub(super) fn layout_shelves(runtime: res!(super::state::DisplayRuntime)) {
    // Mount cards before layout events so their size observers can register normally.
    runtime.state.shelf().layout();
    runtime.shelf.shelf().layout();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn grid_fits_available_bounds_and_retains_landscape_artwork() {
        for width in [424, 560, 720] {
            for height in (190..=440).step_by(8) {
                for total in 1..=17 {
                    let plan = Plan::fit(width, height, total);
                    assert!((1..=2).contains(&plan.grid.rows));
                    assert!(plan.grid.columns * plan.grid.rows <= MAX_VISIBLE);
                    assert!(plan.width() <= width - INSET * 2);
                    assert!(
                        plan.card_height * plan.grid.rows as i32
                            + GAP * (plan.grid.rows as i32 - 1)
                            <= height - INSET * 2 - PAGE_MARK_HEIGHT - PAGE_MARK_GAP
                    );
                    if plan.card_width >= MIN_CARD_WIDTH {
                        assert!(
                            ((plan.card_width - IMAGE_CHROME_WIDTH) * 3
                                - (plan.card_height - IMAGE_CHROME_HEIGHT) * 4)
                                .abs()
                                < 4
                        );
                    }
                }
            }
        }
        assert_eq!(
            Plan::fit(560, 350, 6).grid,
            Grid {
                columns: 3,
                rows: 2
            }
        );
        assert_eq!(Plan::fit(424, 205, 6).grid.rows, 1);
    }
}
