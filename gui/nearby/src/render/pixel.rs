use super::{
    motion::{self, Frame},
    state::DisplayState,
    style::*,
};
use crate::model::Phase;
use mirui::prelude::*;
use mirui::render::command::DrawCommand;
use mirui::ui::theme::{Theme, ThemedColor};

#[derive(Clone, Copy, Debug)]
pub(super) enum Kind {
    Choice {
        index: usize,
        mode: bool,
    },
    Walker,
    Session,
    Signal {
        index: usize,
        strength: u8,
        available: bool,
    },
    Search,
}

#[component]
pub(super) struct Sprite {
    pub state: model!(DisplayState),
    pub kind: Kind,
}

#[derive(Clone, Copy, Debug)]
#[component]
pub(super) enum Scenery {
    Logo,
    Header,
    Map,
    Margins,
    Landscape,
    Ridge,
}

struct Painter<'a, F> {
    theme: &'a Theme,
    emit: F,
    width: i32,
    height: i32,
    unit: i32,
}

impl<F: FnMut(Rect, Color)> Painter<'_, F> {
    fn tree(&mut self, x: i32, y: i32, scale: i32, color: impl Into<ThemedColor> + Copy) {
        self.fill(x - scale, y - 3 * scale, 2 * scale, 3 * scale, DEEP);
        for row in 0..5 {
            self.fill(
                x - (row + 1) * scale,
                y - (12 - row * 2) * scale,
                (row * 2 + 2) * scale,
                2 * scale,
                color,
            );
        }
    }
    fn tent(&mut self, x: i32, y: i32, scale: i32) {
        for row in 0..8 {
            self.fill(
                x - (row + 1) * scale,
                y - (8 - row) * scale,
                (row * 2 + 2) * scale,
                scale,
                CLAY,
            );
        }
        for row in 0..6 {
            self.fill(
                x - (row + 1) * scale,
                y - (6 - row) * scale,
                (row + 1) * scale,
                scale,
                WARNING,
            );
        }
        self.fill(x, y - 4 * scale, 3 * scale, 4 * scale, DEEP);
        self.fill(x - 9 * scale, y, 18 * scale, scale, DEEP);
    }
    fn mountain(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color: impl Into<ThemedColor> + Copy,
    ) {
        for row in (0..height).step_by(4) {
            let span = (width * (row + 4).min(height) / height).min(self.width);
            self.fill(
                (x - span / 2).clamp(0, self.width - span),
                y - height + row,
                span,
                4.min(height - row),
                color,
            );
        }
    }
    fn scenery(&mut self, kind: Scenery) {
        let (w, h) = (self.width, self.height);
        match kind {
            Scenery::Logo => {
                let scale = (w.min(h) / 24).max(1);
                self.fill(0, 0, w, h, BACKGROUND);
                self.fill(4, 4, w - 8, h - 8, SKY);
                self.tree(w - 5 * scale, h - 4 * scale, scale, ACCENT);
                self.tent(w / 2 - 2 * scale, h - 5 * scale, scale);
                self.fill(4, h - 5, w - 8, 2, ACCENT);
            }
            Scenery::Header => {
                self.mountain(w - w / 4, h, w / 3, (h - 4).min(48), self.alpha(HILL, 45));
                self.mountain(w - w / 8, h, w / 4, (h - 4).min(64), self.alpha(SKY, 30));
                let scale = (h / 24).clamp(1, 3).min((w / 24).max(1));
                self.tree(w - 8 * scale, h, scale, self.alpha(HILL, 55));
                self.tree(w * 2 / 3, h, 1, self.alpha(HILL, 55));
            }
            Scenery::Map | Scenery::Margins => {
                if matches!(kind, Scenery::Margins) {
                    let (edge, length, stride) = if w < 500 { (4, 4, 16) } else { (10, 8, 22) };
                    for y in (h / 4..h - 36).step_by(stride) {
                        self.fill(edge, y, length, 2, self.alpha(DIVIDER, 125));
                        self.fill(
                            w - edge - length,
                            y + 8,
                            length,
                            2,
                            self.alpha(DIVIDER, 125),
                        );
                    }
                }
                for y in (12..h - 4).step_by(38) {
                    for x in (8..w - 8).step_by(94) {
                        self.fill(x, y, 2, 2, self.alpha(DIVIDER, 70));
                        self.fill(x + 4, y, 2, 2, self.alpha(DIVIDER, 70));
                    }
                }
            }
            Scenery::Landscape | Scenery::Ridge => {
                let floor = h - 10;
                if matches!(kind, Scenery::Landscape) {
                    self.fill(0, 0, w, floor, SKY);
                }
                self.mountain(w / 3, floor, w * 2 / 3, (floor - 4).min(60), PANEL);
                self.mountain(w * 3 / 4, floor, w / 2, (floor - 4).min(44), HILL);
                self.tree(20, floor, 2, ACCENT);
                self.tree(w - 20, floor, 2, ACCENT);
                self.tree(w - 38, floor, 1, HILL);
                self.fill(0, floor, w, 4, HILL);
                for x in (10..w - 6).step_by(32) {
                    self.fill(x, floor + 6, 6, 2, DIVIDER);
                }
                self.fill(20, 14, 32, 3, BACKGROUND);
                self.fill(28, 11, 16, 3, BACKGROUND);
            }
        }
    }
    fn fill(&mut self, x: i32, y: i32, w: i32, h: i32, color: impl Into<ThemedColor>) {
        if w <= 0 || h <= 0 {
            return;
        }
        #[cfg(test)]
        assert!(
            x >= 0 && y >= 0 && x + w <= self.width && y + h <= self.height,
            "Sprite primitive ({x}, {y}, {w}, {h}) exceeds {}×{}",
            self.width,
            self.height
        );
        (self.emit)(
            Rect::new(
                x / self.unit,
                y / self.unit,
                (w / self.unit).max(1),
                (h / self.unit).max(1),
            ),
            color.into().resolve(self.theme),
        );
    }
    fn alpha(&self, token: ColorToken, alpha: u8) -> Color {
        let color = self.theme.resolve(token);
        Color::rgba(color.r, color.g, color.b, alpha)
    }
    fn person(&mut self, x: i32, y: i32, guest: bool, step: u16, facing_left: bool) {
        let color = if guest {
            Color::rgb(100, 119, 207)
        } else {
            Color::rgb(219, 96, 61)
        };
        let light = if guest {
            Color::rgb(181, 199, 255)
        } else {
            Color::rgb(255, 173, 104)
        };
        let boots = Color::rgb(48, 43, 56);
        let f = step / 90 % 4;
        self.fill(x - 13, y + 34, 28, 3, DIVIDER);
        for (dx, dy, w, h) in [
            (-8, 0, 16, 6),
            (-12, 6, 24, 4),
            (-8, 19, 16, 10),
            (-12, 20 + i32::from(f % 2) * 2, 4, 8),
            (8, 20 - i32::from(f % 2) * 2, 4, 8),
        ] {
            self.fill(x + dx, y + dy, w, h, color);
        }
        self.fill(x - 7, y, 14, 2, light);
        self.fill(x - 8, y + 19, 3, 10, light);
        self.fill(x - 8, y + 10, 16, 9, Color::rgb(255, 224, 170));
        self.fill(x + if facing_left { -6 } else { 3 }, y + 12, 3, 3, boots);
        self.fill(x - 8 + if f == 1 { 3 } else { 0 }, y + 29, 6, 5, boots);
        self.fill(x + 2 - if f == 3 { 3 } else { 0 }, y + 29, 6, 5, boots);
        if guest {
            self.fill(
                x + if facing_left { 8 } else { -12 },
                y + 4,
                4,
                8,
                Color::rgb(244, 200, 95),
            );
            self.fill(
                x + if facing_left { 12 } else { -16 },
                y + 6,
                4,
                4,
                Color::rgb(244, 200, 95),
            );
        }
    }
    fn flag(&mut self, x: i32, y: i32, age: u16) {
        let h = (56.0 * (f32::from(age) / 240.0).min(1.0)).round() as i32;
        self.fill(x, y - h, 3, h, FOREGROUND);
        if h > 22 {
            self.fill(x + 3, y - h, 29, 17, ACCENT);
            self.fill(x + 27, y - h + 13, 8, 4, PANEL);
        }
        self.fill(x - 8, y, 19, 4, DIVIDER);
    }
    fn beacon(&mut self, x: i32, y: i32, wave: Option<u16>) {
        self.fill(x - 10, y - 12, 20, 12, FOREGROUND);
        self.fill(x - 4, y - 26, 8, 14, ACCENT);
        self.fill(x - 2, y - 30, 4, 4, WARNING);
        if let Some(wave) = wave {
            let reach = (y - 28).min(x - 2).min(self.width - x - 2).clamp(0, 48);
            for index in 0..3 {
                let p = f32::from((wave + index * 400) % 1200) / 1200.0;
                let r = 12 + (p * (reach - 12).max(0) as f32).round() as i32;
                if reach < 12 {
                    continue;
                }
                let color = self.alpha(ACCENT, ((1.0 - p) * 150.0) as u8);
                for (dx, dy, w, h) in [
                    (-r, -22 - r, 4, 12),
                    (r - 4, -22 - r, 4, 12),
                    (-r + 4, -26 - r, 12, 4),
                    (r - 16, -26 - r, 12, 4),
                ] {
                    self.fill(x + dx, y + dy, w, h, color);
                }
            }
        }
    }
    fn dust(&mut self, x: i32, y: i32, age: u16) {
        if !(300..560).contains(&age) {
            return;
        }
        let p = f32::from(age - 300) / 260.0;
        for i in 0..6 {
            let side = if i % 2 == 0 { -1.0 } else { 1.0 };
            let dx = (side * (8.0 + (i + 1) as f32 * 6.0 * p)).round() as i32;
            let dy = -(p * std::f32::consts::PI)
                .sin()
                .mul_add((3 + i * 2) as f32, 0.0)
                .round() as i32;
            self.fill(
                x + dx,
                y + dy,
                3,
                3,
                self.alpha(SECONDARY, ((1.0 - p) * 180.0) as u8),
            );
        }
    }
    fn stars(&mut self, x: i32, y: i32, age: u16) {
        if age >= 420 {
            return;
        }
        let p = f32::from(age) / 420.0;
        for i in 0..8 {
            let a = i as f32 * std::f32::consts::FRAC_PI_4;
            let reach = (x - 4)
                .min(self.width - x - 6)
                .min(y - 2)
                .min(self.height - y - 6)
                .clamp(0, 26) as f32;
            let r = reach * (0.35 + 0.65 * (1.0 - (1.0 - p).powi(3)));
            self.fill(
                x + (a.cos() * r).round() as i32,
                y + (a.sin() * r).round() as i32,
                4,
                4,
                self.alpha(ACCENT, ((1.0 - p) * 255.0) as u8),
            );
        }
    }
    fn session(&mut self, w: i32, h: i32, frame: Frame) {
        let floor = h - 12;
        let position = |value: u16| 48 + (w - 96) * i32::from(value) / 1000;
        let host = position(frame.host);
        let guest = position(frame.guest);
        let ready = if frame.network_only {
            frame.connected
        } else {
            frame.ready
        };
        let celebrate = ready
            && frame.phase_age < 800
            && matches!(
                frame.phase,
                Phase::HostReady | Phase::WaitingStart | Phase::NetworkConnected
            );
        self.fill(16, floor, w - 32, 2, DIVIDER);
        for x in (24..w - 20).step_by(22) {
            self.fill(x, floor + 4, 8, 2, DIVIDER);
        }
        if w > 250 && h >= 84 {
            self.flag(
                w / 7,
                floor,
                if frame.phase == Phase::Creating {
                    frame.phase_age
                } else {
                    800
                },
            );
        }
        let shake = if matches!(frame.phase, Phase::Failed | Phase::RestoreFailed) {
            motion::recoil(frame.phase_age)
        } else {
            0
        };
        if h >= 116 {
            self.beacon(w / 2 + shake, floor - 50, frame.wave);
        }
        self.person(
            host,
            floor - 34
                + if celebrate {
                    motion::hop(frame.phase_age.saturating_sub(70), 7.0)
                } else {
                    0
                },
            false,
            if frame.host_walking {
                frame.phase_age
            } else {
                0
            },
            matches!(frame.phase, Phase::Leaving | Phase::Closed),
        );
        if frame.visitor {
            self.person(
                guest,
                floor - 34
                    + if celebrate {
                        motion::hop(frame.phase_age, 11.0)
                    } else {
                        0
                    },
                true,
                if frame.guest_walking {
                    frame.phase_age
                } else {
                    0
                },
                frame.phase != Phase::Starting,
            );
        }
        self.fill(position(200) - 20, floor + 7, 40, 4, ACCENT_DIM);
        self.fill(
            position(800) - 20,
            floor + 7,
            40,
            4,
            if ready { ACCENT } else { DIVIDER },
        );
        if w >= 240
            && matches!(
                frame.phase,
                Phase::ApprovalPending | Phase::ApprovalPendingClient
            )
        {
            let gate = position(900) - 16;
            self.fill(gate, floor - 42, 4, 40, ACCENT);
            self.fill(gate - 10, floor - 42, 24, 4, ACCENT);
        }
        if w >= 240 && matches!(frame.phase, Phase::Checking | Phase::Notifying) {
            self.fill(w / 2 - 12, floor - 40, 24, 2, ACCENT);
            self.fill(w / 2 - 12, floor - 18, 24, 2, ACCENT);
            self.fill(w / 2 - 12, floor - 40, 2, 24, ACCENT);
            self.fill(w / 2 + 10, floor - 40, 2, 24, ACCENT);
            let bars = if frame.phase == Phase::Notifying {
                3
            } else {
                1
            };
            for i in 0..bars {
                self.fill(w / 2 - 6 + i * 5, floor - 32, 3, 8, ACCENT);
            }
        }
        if celebrate {
            self.stars(guest, floor - 30, frame.phase_age);
            self.dust(guest, floor, frame.phase_age);
        }
    }
}

#[view(component = Sprite, priority = 60)]
pub(super) fn sprite(
    component: component!(),
    state: model!(component.state, watch(motion)),
    paint: paint!(),
    theme: res!(Theme),
) {
    let frame = state.motion;
    let rect = paint.rect();
    let (w, h) = (rect.w.to_int(), rect.h.to_int());
    draw(component.kind, w, h, frame, &theme, |area, color| {
        paint.draw_clipped(
            &DrawCommand::Fill {
                area: Rect {
                    x: rect.x + area.x,
                    y: rect.y + area.y,
                    ..area
                },
                transform: paint.transform(),
                quad: None,
                color,
                radius: Fixed::ZERO,
                opa: 255,
            },
            paint.transform().apply_rect_bbox(rect),
        );
    });
}

#[view(component = Scenery, priority = 50)]
pub(super) fn scenery(component: component!(), paint: paint!(), theme: res!(Theme)) {
    let rect = paint.rect();
    draw_scenery(
        *component,
        rect.w.to_int(),
        rect.h.to_int(),
        &theme,
        |area, color| {
            paint.draw_clipped(
                &DrawCommand::Fill {
                    area: Rect {
                        x: rect.x + area.x,
                        y: rect.y + area.y,
                        ..area
                    },
                    transform: paint.transform(),
                    quad: None,
                    color,
                    radius: Fixed::ZERO,
                    opa: 255,
                },
                paint.transform().apply_rect_bbox(rect),
            );
        },
    );
}

fn draw_scenery(
    kind: Scenery,
    width: i32,
    height: i32,
    theme: &Theme,
    emit: impl FnMut(Rect, Color),
) {
    if width <= 0 || height <= 0 {
        return;
    }
    let (mw, mh) = match kind {
        Scenery::Logo => (48, 48),
        Scenery::Landscape | Scenery::Ridge => (112, 40),
        _ => (32, 16),
    };
    let unit = ((mw + width - 1) / width)
        .max((mh + height - 1) / height)
        .max(1);
    Painter {
        theme,
        emit,
        width: width * unit,
        height: height * unit,
        unit,
    }
    .scenery(kind);
}

fn draw(
    kind: Kind,
    width: i32,
    height: i32,
    frame: Frame,
    theme: &Theme,
    emit: impl FnMut(Rect, Color),
) {
    if width <= 0 || height <= 0 {
        return;
    }
    // Each stage includes the full jump, shadow and particle envelope.
    let (min_width, min_height) = match kind {
        Kind::Choice { .. } => (112, 80),
        Kind::Walker => (112, 38),
        Kind::Session => (160, 60),
        Kind::Search => (112, 42),
        Kind::Signal { .. } => (30, 26),
    };
    let unit = ((min_width + width - 1) / width)
        .max((min_height + height - 1) / height)
        .max(1);
    let (w, h) = (width * unit, height * unit);
    let mut p = Painter {
        theme,
        emit,
        width: w,
        height: h,
        unit,
    };
    match kind {
        Kind::Choice { index, mode } => {
            let age = if frame.selected == Some(index) {
                frame.selection
            } else {
                800
            };
            let jump = motion::hop(age, 10.0);
            let y = h - 10;
            p.fill(8, y, w - 16, 2, DIVIDER);
            if mode && index == 1 {
                p.fill(w / 2 - 26, y - 45 + jump, 52, 30, FOREGROUND);
                p.fill(w / 2 - 20, y - 39 + jump, 40, 18, BACKGROUND);
                p.fill(w / 2 - 16, y - 14 + jump, 4, 10, ACCENT);
                p.fill(w / 2 - 19, y - 11 + jump, 10, 4, ACCENT);
                p.fill(w / 2 + 12, y - 11 + jump, 5, 5, ACCENT);
            } else if index == 0 && !mode {
                p.tent(w / 2 - 12, y + jump, 2);
                p.flag(w / 2 + 20, y + jump, 800)
            } else {
                p.beacon(w / 2, y - 12 + jump, (age < 600).then_some(age));
            }
            p.dust(w / 2, y, age);
        }
        Kind::Walker => {
            let x = 48 + (w - 96) * i32::from(frame.walker) / 1000;
            p.person(
                x,
                h - 38
                    + if frame.selection >= 240 {
                        motion::hop(frame.selection - 240, (h - 38).min(4) as f32)
                    } else {
                        0
                    },
                false,
                if frame.walking { frame.selection } else { 0 },
                frame.selected == Some(0),
            );
            p.dust(x, h - 4, frame.selection.saturating_sub(240));
        }
        Kind::Session => {
            p.session(w, h, frame);
        }
        Kind::Search => p.beacon(w / 2, h - 8, frame.wave),
        Kind::Signal {
            index,
            strength,
            available,
        } => {
            for i in 0..4 {
                let age = if let Some(wave) = frame.wave {
                    wave.saturating_sub(i * 55)
                } else if frame.selected == Some(index) {
                    frame.selection.saturating_sub(i * 55)
                } else {
                    800
                };
                let hbar = 6 + i32::from(i) * 4;
                p.fill(
                    i32::from(i) * 7 + 2,
                    h - hbar - 2 + motion::hop(age, 3.0),
                    4,
                    hbar,
                    if available && u16::from(strength) > i * 25 {
                        ACCENT
                    } else {
                        DIVIDER
                    },
                );
            }
        }
    }
}

#[compose]
pub(super) fn art(state: model!(DisplayState), kind: Kind, height: i32) -> Entity {
    let scenery = if matches!(kind, Kind::Session | Kind::Search) {
        Scenery::Landscape
    } else {
        Scenery::Map
    };
    ui! {
        View (
            height: height,
            min_height: 0,
            shrink: 1.0,
            width: Dimension::percent(100),
            clip_children: true
        ) [
            scenery,
            Sprite {
                state: state.clone(),
                kind,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scenery_stays_in_bounds_and_character_colors_are_theme_independent() {
        let mut reference = None;
        for palette in crate::theme::Palette::ALL {
            let theme = palette.theme();
            for kind in [
                Scenery::Logo,
                Scenery::Header,
                Scenery::Map,
                Scenery::Margins,
                Scenery::Landscape,
                Scenery::Ridge,
            ] {
                for (width, height) in [
                    (1, 1),
                    (30, 26),
                    (42, 42),
                    (120, 24),
                    (240, 80),
                    (592, 68),
                    (640, 480),
                ] {
                    draw_scenery(kind, width, height, &theme, |rect, _| {
                        assert!(
                            rect.x >= Fixed::ZERO
                                && rect.y >= Fixed::ZERO
                                && rect.x + rect.w <= Fixed::from_int(width)
                                && rect.y + rect.h <= Fixed::from_int(height)
                        );
                    });
                }
            }
            let mut colors = Vec::new();
            for guest in [false, true] {
                let mut sprite = Vec::new();
                Painter {
                    theme: &theme,
                    emit: |_, color| sprite.push(color),
                    width: 120,
                    height: 60,
                    unit: 1,
                }
                .person(60, 10, guest, 0, false);
                colors.extend(sprite.into_iter().skip(1));
            }
            if let Some(reference) = &reference {
                assert_eq!(&colors, reference);
            } else {
                reference = Some(colors);
            }
        }
    }

    #[test]
    fn motion_envelopes_fit_their_allocated_stages() {
        let kinds = [
            Kind::Choice {
                index: 0,
                mode: false,
            },
            Kind::Choice {
                index: 1,
                mode: false,
            },
            Kind::Choice {
                index: 0,
                mode: true,
            },
            Kind::Choice {
                index: 1,
                mode: true,
            },
            Kind::Walker,
            Kind::Session,
            Kind::Search,
            Kind::Signal {
                index: 0,
                strength: 100,
                available: true,
            },
        ];
        for kind in kinds {
            for (w, h) in [
                (30, 26),
                (120, 24),
                (120, 32),
                (180, 40),
                (240, 64),
                (240, 80),
                (550, 120),
                (720, 200),
            ] {
                for phase in [
                    Phase::Creating,
                    Phase::WaitingPeer,
                    Phase::ApprovalPending,
                    Phase::ApprovalPendingClient,
                    Phase::PeerPreparing,
                    Phase::Connecting,
                    Phase::Checking,
                    Phase::Notifying,
                    Phase::HostReady,
                    Phase::WaitingStart,
                    Phase::NetworkConnected,
                    Phase::NetworkConfirming,
                    Phase::NetworkReconnecting,
                    Phase::Starting,
                    Phase::Leaving,
                    Phase::Closed,
                    Phase::Failed,
                    Phase::RestoreFailed,
                ] {
                    for age in (0..=1200).step_by(16) {
                        for selected in [0, 1] {
                            let frame = Frame {
                                selected: Some(selected),
                                phase,
                                phase_age: age,
                                selection: age,
                                wave: Some(age),
                                ready: true,
                                visitor: true,
                                host: 100 + age.min(800) / 2,
                                guest: 980 - age.min(800),
                                walker: if selected == 0 { 250 } else { 750 },
                                ..Frame::default()
                            };
                            draw(
                                kind,
                                w,
                                h,
                                frame,
                                &crate::theme::Palette::Forest.theme(),
                                |rect, _| {
                                    assert!(
                                        rect.x >= Fixed::ZERO
                                            && rect.y >= Fixed::ZERO
                                            && rect.x + rect.w <= Fixed::from_int(w)
                                            && rect.y + rect.h <= Fixed::from_int(h),
                                        "{kind:?} {phase:?} {age}: {rect:?} outside {w}×{h}"
                                    );
                                },
                            );
                        }
                    }
                }
            }
        }
    }
}
