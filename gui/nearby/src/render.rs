use std::rc::Rc;

use mirui::ecs::Entity;
use mirui::prelude::*;
use mirui::render::font::{Font, FontBackend, FontManager, FontToken, mirx::MirxFontProvider};
use mirui::surface::FramebufferAccess;
use mirui::ui::OffscreenRender;
use mirui::ui::widgets::navigation::{PageDirection, PageTransition};
use mirui::ui::widgets::page_view::{self, CreatePage, PageView};
use mirui::ui::widgets::{ParagraphStyle, Text, TextAlign, TextOverflow, TextWrap};

mod selection;

use crate::model::{Page, Phase, View};
use TextAlign::{Center, End, Start};
use TextField::{Headline, HostButton, Next, Peer};

const FONT_PAINT: mirui::ui::dirty::PaintInflate =
    mirui::ui::dirty::PaintInflate::uniform(Fixed::ONE);

const FONT: FontToken = FontToken::Custom("nearby_menu");
const FONT_BYTES: &[u8] = include_bytes!("../assets/nearby-menu-20.mirx");
const BACKGROUND: Color = Color::rgb(242, 235, 221);
const PANEL: Color = Color::rgb(231, 221, 200);
const ACCENT: Color = Color::rgb(166, 61, 50);
const ACCENT_DIM: Color = Color::rgb(234, 207, 192);
const FOREGROUND: Color = Color::rgb(41, 37, 32);
const SECONDARY: Color = Color::rgb(112, 98, 83);
const DISABLED: Color = Color::rgb(131, 117, 99);
const DIVIDER: Color = Color::rgb(204, 192, 169);
const WARNING: Color = Color::rgb(152, 106, 32);
const WARNING_PANEL: Color = Color::rgb(233, 218, 186);

#[derive(Clone, Copy)]
enum TextField {
    Headline,
    Next,
    HostButton,
    Peer,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct StepColors {
    background: Color,
    foreground: Color,
}

fn publish<T: Clone + PartialEq>(signal: &Signal<T>, value: T) {
    if signal.get_untracked() != value {
        signal.set(value);
    }
}

#[model]
struct DisplayState {
    view: View,
    direction: PageDirection,
    rooms: Signal<Vec<(usize, crate::model::RoomRow)>>,
    entries: Signal<Vec<crate::model::GameRow>>,
    address: Signal<String>,
    peers: Signal<String>,
    room_name: Signal<String>,
    heading: Signal<String>,
    subtitle: Signal<String>,
    status: Signal<String>,
    hint: Signal<String>,
    back: Signal<String>,
    headline: Signal<String>,
    next: Signal<String>,
    host_button: Signal<String>,
    peer: Signal<String>,
    confirm: Signal<String>,
    #[observe]
    confirm_visible: bool,
    #[observe]
    steps: [StepColors; 3],
    #[observe]
    stamp: Color,
    #[observe]
    activity: [Color; 3],
}

#[model]
impl DisplayState {
    fn present(&mut self, view: &View) {
        publish(&self.status, status_text(view).into());
        publish(&self.hint, hint_text(view).into());
        publish(
            &self.back,
            if view.back_label.is_empty() {
                "返回"
            } else {
                &view.back_label
            }
            .into(),
        );
        publish(&self.headline, headline(view).into());
        publish(&self.next, next_step(view).into());
        publish(&self.host_button, button_text(view).into());
        publish(&self.peer, peer_text(view).into());
        publish(&self.confirm, confirm_text(view).into());
        self.confirm_visible = !confirm_text(view).is_empty();
        let active = match phase(view) {
            Phase::Connecting => Some(0),
            Phase::Checking | Phase::Notifying => Some(1),
            Phase::WaitingStart | Phase::Starting => Some(2),
            _ => None,
        };
        self.steps = std::array::from_fn(|index| {
            if active == Some(index) {
                StepColors {
                    background: ACCENT,
                    foreground: BACKGROUND,
                }
            } else if active.is_some_and(|step| index < step) {
                StepColors {
                    background: ACCENT_DIM,
                    foreground: ACCENT,
                }
            } else {
                StepColors {
                    background: PANEL,
                    foreground: DISABLED,
                }
            }
        });
        let first = view
            .selected
            .saturating_sub(3)
            .min(view.rooms.len().saturating_sub(4));
        let rows: Vec<_> = view
            .rooms
            .iter()
            .enumerate()
            .skip(first)
            .take(4)
            .map(|(i, row)| (i, row.clone()))
            .collect();
        publish(&self.rooms, rows);
        let entries: Vec<_> = view.entries.iter().take(4).cloned().collect();
        publish(&self.entries, entries);
        publish(
            &self.address,
            if view.local_address.is_empty() {
                "等待分配地址".into()
            } else {
                format!("本机  {}", view.local_address)
            },
        );
        publish(
            &self.peers,
            view.peers
                .iter()
                .map(|peer| format!("{}  {}", peer.name, peer.address))
                .collect::<Vec<_>>()
                .join("\n"),
        );
        publish(&self.room_name, view.room_name.clone());
        publish(&self.heading, page_heading(view).into());
        publish(
            &self.subtitle,
            match view.page {
                Page::Mode => "连接附近的掌机".into(),
                _ if view.network_only => "仅组网 · 局域网连接".into(),
                Page::Games => view.core.clone(),
                _ => format!("当前游戏  {}", view.title),
            },
        );
        self.view.clone_from(view);
    }

    fn rooms(&self) -> Vec<(usize, crate::model::RoomRow)> {
        self.rooms.get()
    }
    fn entries(&self) -> Vec<crate::model::GameRow> {
        self.entries.get()
    }
    fn address(&self) -> String {
        self.address.get()
    }
    fn peers(&self) -> String {
        self.peers.get()
    }
    fn room_name(&self) -> String {
        self.room_name.get()
    }
    fn heading(&self) -> String {
        self.heading.get()
    }
    fn subtitle(&self) -> String {
        self.subtitle.get()
    }
    fn status(&self) -> String {
        self.status.get()
    }
    fn hint(&self) -> String {
        self.hint.get()
    }
    fn back(&self) -> String {
        self.back.get()
    }
    fn headline(&self) -> String {
        self.headline.get()
    }
    fn next(&self) -> String {
        self.next.get()
    }
    fn host_button(&self) -> String {
        self.host_button.get()
    }
    fn peer(&self) -> String {
        self.peer.get()
    }
    fn confirm(&self) -> String {
        self.confirm.get()
    }

    fn snapshot(&self) -> View {
        self.view.clone()
    }

    #[observe]
    fn page_key(&self) -> PageKey {
        PageKey {
            page: self.view.page,
            network_only: self.view.network_only
                && matches!(
                    self.view.page,
                    Page::Choice | Page::Rooms | Page::Host | Page::Joined | Page::Leaving
                ),
        }
    }

    #[observe]
    fn transition(&self) -> PageTransition {
        PageTransition::slide(mirui::anim::SNAPPY).with_direction(self.direction)
    }

    #[cfg(any(target_os = "linux", test))]
    fn navigate_back(&mut self) {
        self.direction = PageDirection::Backward;
    }

    #[cfg(any(target_os = "linux", test))]
    fn navigate_forward(&mut self) {
        self.direction = PageDirection::Forward;
    }

    #[observe]
    fn selected(&self) -> Option<usize> {
        match self.view.page {
            Page::Mode | Page::Choice => Some(self.view.selected.min(1)),
            Page::Rooms if !self.view.rooms.is_empty() => {
                Some(self.view.selected.min(self.view.rooms.len() - 1))
            }
            Page::Games if !self.view.entries.is_empty() => Some(self.view.selected.min(3)),
            Page::Host if host_action_enabled(&self.view) => Some(0),
            Page::Joined if self.view.network_only && host_action_enabled(&self.view) => Some(0),
            _ => None,
        }
    }

    #[observe]
    fn ready(&self) -> bool {
        self.view.ready
    }

    #[observe]
    fn enabled(&self) -> bool {
        host_action_enabled(&self.view)
    }

    fn feedback(&mut self, stamp: Color, activity: [Color; 3]) {
        self.stamp = stamp;
        self.activity = activity;
    }
}

type DisplayHandle = <DisplayState as mirui::core::model::Model>::Handle;

#[derive(Clone, Copy, PartialEq, Eq)]
struct PageKey {
    page: Page,
    network_only: bool,
}

impl DisplayStateHandle {
    fn text(&self, field: TextField) -> String {
        match field {
            Headline => self.headline(),
            Next => self.next(),
            HostButton => self.host_button(),
            Peer => self.peer(),
        }
    }
}

struct DisplayRuntime {
    state: DisplayHandle,
}

pub fn install_fonts<B: FramebufferAccess>(app: &mut App<B>) -> Result<(), String> {
    app.with_offscreen_pool_budget(2 * 1024 * 1024);
    app.with_widget(page_view::view::<PageKey>());
    app.with_widget(selection::target::view());
    app.with_widget(selection::frame::view());
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
    let state = app
        .add_model(DisplayState {
            view: View::default(),
            direction: PageDirection::Forward,
            rooms: Signal::new(Vec::new()),
            entries: Signal::new(Vec::new()),
            address: Signal::new(String::new()),
            peers: Signal::new(String::new()),
            room_name: Signal::new(String::new()),
            heading: Signal::new(String::new()),
            subtitle: Signal::new(String::new()),
            status: Signal::new(String::new()),
            hint: Signal::new(String::new()),
            back: Signal::new(String::new()),
            headline: Signal::new(String::new()),
            next: Signal::new(String::new()),
            host_button: Signal::new(String::new()),
            peer: Signal::new(String::new()),
            confirm: Signal::new(String::new()),
            confirm_visible: false,
            steps: [StepColors {
                background: PANEL,
                foreground: DISABLED,
            }; 3],
            stamp: ACCENT_DIM,
            activity: [BACKGROUND; 3],
        })
        .finish();
    app.add_system(selection::update::system(state.clone()));
    app.add_resource(DisplayRuntime { state });
    Ok(())
}

#[derive(Clone, Copy)]
struct Metrics {
    compact: bool,
    title: u16,
    body: u16,
    small: u16,
    row_title: u16,
    row_detail: u16,
    gap: i32,
    inset: i32,
}

impl Metrics {
    fn for_height(height: u16) -> Self {
        let compact = height < 400;
        Self {
            compact,
            title: if compact { 18 } else { 25 },
            body: if compact { 14 } else { 17 },
            small: if compact { 10 } else { 15 },
            row_title: if compact { 14 } else { 18 },
            row_detail: if compact { 9 } else { 12 },
            gap: if compact { 3 } else { 8 },
            inset: if compact { 8 } else { 18 },
        }
    }
}

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

use selection::FocusTarget;

fn paragraph(align: TextAlign, lines: u16) -> ParagraphStyle {
    ParagraphStyle {
        wrap: TextWrap::Word,
        align,
        vertical_align: mirui::ui::widgets::text::TextVerticalAlign::Center,
        overflow: TextOverflow::Ellipsis,
        max_lines: Some(lines),
        ..Default::default()
    }
}

#[compose]
fn label(value: &str, size: u16, color: Color, align: TextAlign) -> Entity {
    ui! {
        Text (
            value.to_owned(),
            width: Dimension::percent(100),
            height: Dimension::Content,
            min_width: 0,
            min_height: i32::from(size) + 6,
            font: FONT,
            font_size: size,
            text_color: color,
            paragraph: paragraph(align, 1),
            clip_children: true
        ) [
            FONT_PAINT,
        ]
    }
}

#[compose]
fn live_label(
    state: model!(DisplayState),
    field: TextField,
    size: u16,
    color: Color,
    align: TextAlign,
) -> Entity {
    ui! {
        Text (
            text: ${ state.text(field) },
            width: Dimension::percent(100),
            height: Dimension::Content,
            min_width: 0,
            min_height: i32::from(size) + 6,
            font: FONT,
            font_size: size,
            text_color: color,
            paragraph: paragraph(align, 1),
            clip_children: true
        ) [
            FONT_PAINT,
        ]
    }
}

#[compose]
fn header(view: &View, state: model!(DisplayState), metrics: Metrics) -> Entity {
    let step = if view.page == Page::Mode {
        "选择模式".into()
    } else if view.page == Page::Games {
        "选择游戏".into()
    } else {
        format!("步骤 {} / 2", if view.page == Page::Choice { 1 } else { 2 })
    };
    ui! {
        Row (
            height: if metrics.compact { 44 } else { 68 },
            shrink: 0.0,
            column_gap: metrics.gap + 4,
            align: AlignItems::Center
        ) [
            Section::Header,
        ] {
            Row (
                width: if metrics.compact { 30 } else { 42 },
                height: if metrics.compact { 30 } else { 42 },
                shrink: 0.0,
                bg_color: ${ state.stamp() },
                justify: JustifyContent::Center,
                align: AlignItems::Center,
                border_radius: 0
            ) {
                View (
                    width: 7,
                    height: 18,
                    bg_color: ACCENT
                )
                View (
                    width: 7,
                    height: 6,
                    bg_color: ACCENT
                )
                View (
                    width: 7,
                    height: 18,
                    bg_color: ACCENT
                )
            }
            Column (
                grow: 1.0,
                min_width: 0,
                justify: JustifyContent::Center,
                clip_children: true
            ) [
                StaticCache,
                OffscreenRender::default(),
            ] {
                label (
                    "附近联机",
                    if metrics.compact { 20 } else { 27 },
                    FOREGROUND,
                    Start
                )
                Text (
                    text: ${ state.subtitle() },
                    font: FONT,
                    font_size: metrics.small,
                    text_color: SECONDARY,
                    height: Dimension::Content,
                    paragraph: paragraph(Start, 1)
                ) [
                    FONT_PAINT,
                ]
            }
            Text (
                step,
                width: if metrics.compact { 76 } else { 96 },
                height: if metrics.compact { 26 } else { 34 },
                shrink: 0.0,
                bg_color: PANEL,
                font: FONT,
                font_size: metrics.small,
                text_color: SECONDARY,
                paragraph: paragraph(Center, 1)
            ) [
                FONT_PAINT,
            ]
        }
    }
}

#[compose]
fn keycap(value: &str, metrics: Metrics) -> Entity {
    ui! {
        Text (
            value.to_owned(),
            width: if metrics.compact { 20 } else { 25 },
            height: if metrics.compact { 20 } else { 25 },
            shrink: 0.0,
            bg_color: PANEL,
            font: FONT,
            font_size: metrics.small,
            text_color: SECONDARY,
            paragraph: paragraph(Center, 1)
        ) [
            FONT_PAINT,
        ]
    }
}

#[compose]
fn footer(view: &View, state: model!(DisplayState), metrics: Metrics) -> Entity {
    ui! {
        Row (
            height: if metrics.compact { 20 } else { 32 },
            shrink: 0.0,
            column_gap: metrics.gap,
            align: AlignItems::Center
        ) [
            Section::Footer,
        ] {
            Text (
                text: ${ if state.confirm_visible() { "A" } else { "" } },
                width: if metrics.compact { 20 } else { 25 },
                height: if metrics.compact { 20 } else { 25 },
                shrink: 0.0,
                bg_color: ${ if state.confirm_visible() { PANEL } else { BACKGROUND } },
                font: FONT,
                font_size: metrics.small,
                text_color: SECONDARY,
                paragraph: paragraph(Center, 1)
            ) [
                FONT_PAINT,
            ]
            Text (
                text: ${ state.confirm() },
                width: Dimension::Content,
                shrink: 0.0,
                height: Dimension::Content,
                font: FONT,
                font_size: metrics.small,
                text_color: SECONDARY,
                paragraph: paragraph(Start, 1)
            ) [
                FONT_PAINT,
            ]
            keycap ("B", metrics)
            Text (
                text: ${ state.back() },
                max_width: if metrics.compact { 130 } else { 210 },
                min_width: 0,
                height: Dimension::Content,
                font: FONT,
                font_size: metrics.small,
                text_color: SECONDARY,
                paragraph: paragraph(Start, 1)
            ) [
                FONT_PAINT,
            ]
            if view.page == Page::Rooms {
                keycap ("X", metrics)
                Text (
                    "刷新",
                    height: Dimension::Content,
                    font: FONT,
                    font_size: metrics.small,
                    text_color: SECONDARY,
                    paragraph: paragraph(Start, 1),
                    clip_children: true
                ) [
                    FONT_PAINT,
                ]
            }
            Text (
                text: ${ state.hint() },
                grow: 1.0,
                min_width: 0,
                height: Dimension::Content,
                font: FONT,
                font_size: metrics.small,
                text_color: SECONDARY,
                paragraph: paragraph(End, 1),
                clip_children: true
            ) [
                FONT_PAINT,
            ]
        }
    }
}

#[compose]
fn choice(view: &View, metrics: Metrics) -> Entity {
    let cards = if view.page == Page::Mode {
        [
            (0, "仅组网", "连接附近的掌机", "返回系统后保持连接"),
            (1, "选择游戏", "选择要一起玩的游戏", "创建或加入游戏房间"),
        ]
    } else if view.network_only {
        [
            (0, "创建房间", "邀请好友连接你的掌机", "你是房主"),
            (1, "加入房间", "寻找附近好友的房间", "建立局域网连接"),
        ]
    } else {
        [
            (0, "创建房间", "邀请好友加入你的房间", "你是房主"),
            (1, "加入房间", "寻找附近好友的房间", "一起开始游戏"),
        ]
    };
    ui! {
        Row (
            grow: 1.0,
            min_height: 0,
            column_gap: metrics.gap + 6
        ) [
            Section::FocusList,
            StaticCache,
            OffscreenRender::default(),
        ] {
            walk cards.into_iter() with card {
                Column (
                    grow: 1.0,
                    min_width: 0,
                    padding: Padding::all(metrics.inset),
                    row_gap: metrics.gap,
                    bg_color: PANEL,
                    border_radius: 0,
                    clip_children: true
                ) [
                    FocusTarget { index: card.0 },
                ] {
                    Row (
                        height: if metrics.compact { 28 } else { 44 },
                        align: AlignItems::Center
                    ) {
                        Text (
                            if card.0 == 0 { "+" } else { "2" },
                            width: if metrics.compact { 28 } else { 44 },
                            height: Dimension::percent(100),
                            bg_color: ACCENT_DIM,
                            font: FONT,
                            font_size: metrics.title,
                            text_color: ACCENT,
                            paragraph: paragraph(Center, 1)
                        ) [
                            FONT_PAINT,
                        ]
                    }
                    View (
                        grow: 1.0,
                        min_height: 0
                    )
                    label (
                        card.1,
                        metrics.title,
                        FOREGROUND,
                        Start
                    )
                    Text (
                        card.2,
                        width: Dimension::percent(100),
                        min_width: 0,
                        height: Dimension::Content,
                        font: FONT,
                        font_size: metrics.small,
                        text_color: SECONDARY,
                        paragraph: paragraph(Start, 2)
                    ) [
                        FONT_PAINT,
                    ]
                    if !metrics.compact {
                        label (
                            card.3,
                            metrics.small,
                            SECONDARY,
                            Start
                        )
                    }
                }
            }
        }
    }
}

fn host_action_enabled(view: &View) -> bool {
    (view.ready || view.connected || view.phase == Phase::ApprovalPending) && !view.busy
}

#[compose]
fn avatar(number: &str, state: model!(DisplayState), own: bool, metrics: Metrics) -> Entity {
    ui! {
        Text (
            number.to_owned(),
            width: if metrics.compact { 36 } else { 60 },
            height: if metrics.compact { 36 } else { 60 },
            bg_color: ${ if own || state.ready() { ACCENT_DIM } else { PANEL } },
            font: FONT,
            font_size: metrics.title,
            text_color: ${ if own || state.ready() { ACCENT } else { DISABLED } },
            paragraph: paragraph(Center, 1)
        ) [
            FONT_PAINT,
            StaticCache,
            OffscreenRender::default(),
        ]
    }
}

#[compose]
fn host(_view: &View, state: model!(DisplayState), metrics: Metrics) -> Entity {
    ui! {
        Column (
            grow: 1.0,
            min_height: 0,
            row_gap: metrics.gap
        ) {
            Row (
                grow: 1.0,
                min_height: 0,
                padding: Padding::all(metrics.inset),
                bg_color: PANEL,
                align: AlignItems::Center,
                justify: JustifyContent::Center
            ) {
                Column (
                    grow: 1.0,
                    min_width: 0,
                    align: AlignItems::Center,
                    justify: JustifyContent::Center,
                    row_gap: metrics.gap
                ) {
                    avatar (
                        "1",
                        state,
                        true,
                        metrics
                    )
                    label (
                        "你 · 房主",
                        metrics.body,
                        FOREGROUND,
                        Center
                    )
                }
                Row (
                    width: if metrics.compact { 40 } else { 70 },
                    justify: JustifyContent::SpaceEvenly,
                    align: AlignItems::Center
                ) {
                    walk 0..3 with _index {
                        View (
                            width: 5,
                            height: 5,
                            bg_color: ${ if state.ready() { ACCENT } else { DISABLED } }
                        )
                    }
                }
                Column (
                    grow: 1.0,
                    min_width: 0,
                    align: AlignItems::Center,
                    justify: JustifyContent::Center,
                    row_gap: metrics.gap
                ) {
                    avatar (
                        "2",
                        state,
                        false,
                        metrics
                    )
                    live_label (
                        state,
                        Peer,
                        metrics.body,
                        FOREGROUND,
                        Center
                    )
                }
            }
            Column (
                height: if metrics.compact { 34 } else { 50 },
                shrink: 0.0,
                bg_color: ${ if state.enabled() { ACCENT } else { PANEL } },
                border_color: ACCENT,
                border_width: ${ if state.enabled() { 2 } else { 0 } },
                justify: JustifyContent::Center
            ) [
                Section::FocusHost,
                FocusTarget { index: 0 },
            ] {
                live_label (
                    state,
                    HostButton,
                    metrics.title,
                    BACKGROUND,
                    Center
                )
            }
        }
    }
}

#[compose]
fn network_panel(_view: &View, state: model!(DisplayState), metrics: Metrics) -> Entity {
    ui! {
        Column (grow: 1.0, min_height: 0, row_gap: metrics.gap) {
            Column (
                grow: 1.0,
                min_height: 0,
                padding: Padding::all(metrics.inset),
                bg_color: PANEL,
                justify: JustifyContent::Center,
                row_gap: metrics.gap
            ) {
                live_label (state, Headline, metrics.title, FOREGROUND, Center)
                Text (
                    text: ${ state.address() },
                    font: FONT,
                    font_size: metrics.body,
                    text_color: SECONDARY,
                    height: Dimension::Content,
                    paragraph: paragraph(Center, 1)
                ) [
                    FONT_PAINT,
                ]
                Text (
                    text: ${ state.peers() },
                    width: Dimension::percent(100),
                    min_width: 0,
                    height: Dimension::Content,
                    font: FONT,
                    font_size: metrics.small,
                    text_color: SECONDARY,
                    paragraph: paragraph(Center, 3),
                    clip_children: true
                ) [
                    FONT_PAINT,
                ]
                live_label (state, Next, metrics.small, SECONDARY, Center)
            }
            Column (
                height: if metrics.compact { 34 } else { 50 },
                shrink: 0.0,
                bg_color: ${ if state.enabled() { ACCENT } else { PANEL } },
                justify: JustifyContent::Center
            ) [
                Section::FocusHost,
                FocusTarget { index: 0 },
            ] {
                live_label (
                    state,
                    HostButton,
                    metrics.title,
                    BACKGROUND,
                    Center
                )
            }
        }
    }
}

#[compose]
fn room_signal(strength: u8, available: bool) -> Entity {
    ui! {
        Row (
            width: 30,
            height: 24,
            shrink: 0.0,
            column_gap: 3,
            align: AlignItems::FlexEnd
        ) {
            walk 0..4 with index {
                View (
                    width: 4,
                    height: 6 + index * 4,
                    bg_color: if available && strength as i32 > index * 25 { ACCENT } else { DIVIDER }
                )
            }
        }
    }
}

#[compose]
fn rooms(state: model!(DisplayState), metrics: Metrics) -> Entity {
    ui! {
        Column (grow: 1.0, min_height: 0) {
            match ${ state.rooms() } {
                rows => {
                    room_rows (&rows, metrics)
                }
            }
        }
    }
}

#[compose]
fn room_rows(rows: &[(usize, crate::model::RoomRow)], metrics: Metrics) -> Entity {
    ui! {
        Column (
            grow: 1.0,
            min_height: 0,
            row_gap: metrics.gap
        ) [
            Section::FocusList,
            StaticCache,
            OffscreenRender::default(),
        ] {
            if rows.is_empty() {
                Column (
                    grow: 1.0,
                    bg_color: PANEL,
                    padding: Padding::all(metrics.inset),
                    justify: JustifyContent::Center,
                    row_gap: metrics.gap
                ) {
                    label (
                        "正在搜索附近的房间",
                        metrics.title,
                        FOREGROUND,
                        Center
                    )
                    Text (
                        "请好友先创建房间，并把两台掌机放近一些",
                        width: Dimension::percent(100),
                        height: Dimension::Content,
                        font: FONT,
                        font_size: metrics.small,
                        text_color: SECONDARY,
                        paragraph: paragraph(Center, 2)
                    ) [
                        FONT_PAINT,
                    ]
                }
            } else {
                walk rows.iter() with item {
                    Row (
                        grow: 1.0,
                        min_height: i32::from(metrics.row_title + metrics.row_detail) + 17,
                        max_height: if metrics.compact { 42 } else { 70 },
                        padding: Padding::all(2),
                        column_gap: metrics.gap,
                        bg_color: PANEL,
                        align: AlignItems::Center,
                        clip_children: true
                    ) [
                        FocusTarget { index: item.0 },
                    ] {
                        Column (
                            grow: 1.0,
                            min_width: 0,
                            justify: JustifyContent::Center
                        ) {
                            label (
                                &item.1.name,
                                metrics.row_title,
                                if item.1.available { FOREGROUND } else { DISABLED },
                                Start
                            )
                            label (
                                if item.1.available {
                                    &item.1.detail
                                } else {
                                    "房间已离线，等待重新出现"
                                },
                                metrics.row_detail,
                                if item.1.available { SECONDARY } else { DISABLED },
                                Start
                            )
                        }
                        room_signal (
                            item.1.signal,
                            item.1.available
                        )
                    }
                }
                View (
                    grow: 1.0,
                    min_height: 0
                )
            }
        }
    }
}

#[compose]
fn games(state: model!(DisplayState), metrics: Metrics) -> Entity {
    ui! {
        Column (grow: 1.0, min_height: 0) {
            match ${ state.entries() } {
                rows => {
                    game_rows (&rows, metrics)
                }
            }
        }
    }
}

#[compose]
fn game_rows(rows: &[crate::model::GameRow], metrics: Metrics) -> Entity {
    ui! {
        Column (
            grow: 1.0,
            min_height: 0,
            row_gap: metrics.gap
        ) [
            Section::FocusList,
            StaticCache,
            OffscreenRender::default(),
        ] {
            if rows.is_empty() {
                Column (
                    grow: 1.0,
                    bg_color: PANEL,
                    justify: JustifyContent::Center,
                    padding: Padding::all(metrics.inset)
                ) {
                    label (
                        "没有可选游戏",
                        metrics.title,
                        FOREGROUND,
                        Center
                    )
                }
            } else {
                walk rows.iter().enumerate() with item {
                    Row (
                        grow: 1.0,
                        min_height: i32::from(metrics.row_title + metrics.row_detail) + 17,
                        max_height: if metrics.compact { 42 } else { 70 },
                        padding: Padding::all(2),
                        column_gap: metrics.gap,
                        bg_color: PANEL,
                        align: AlignItems::Center,
                        clip_children: true
                    ) [
                        FocusTarget { index: item.0 },
                    ] {
                        Text (
                            if item.1.directory { ">" } else { "·" },
                            width: if metrics.compact { 20 } else { 32 },
                            shrink: 0.0,
                            height: Dimension::Content,
                            font: FONT,
                            font_size: metrics.title,
                            text_color: ACCENT,
                            paragraph: paragraph(Center, 1)
                        ) [
                            FONT_PAINT,
                        ]
                        Column (
                            grow: 1.0,
                            min_width: 0,
                            justify: JustifyContent::Center
                        ) {
                            label (
                                &item.1.name,
                                metrics.row_title,
                                FOREGROUND,
                                Start
                            )
                            label (
                                &item.1.detail,
                                metrics.row_detail,
                                SECONDARY,
                                Start
                            )
                        }
                    }
                }
                View (
                    grow: 1.0,
                    min_height: 0
                )
            }
        }
    }
}

#[compose]
fn joined(_view: &View, state: model!(DisplayState), metrics: Metrics) -> Entity {
    ui! {
        Column (
            grow: 1.0,
            min_height: 0,
            bg_color: PANEL,
            padding: Padding::all(metrics.inset),
            row_gap: metrics.gap,
            justify: JustifyContent::Center,
            clip_children: true
        ) {
            Row (
                height: if metrics.compact { 44 } else { 60 },
                shrink: 0.0,
                column_gap: metrics.gap
            ) {
                walk [(0, "连接房间"), (1, "核对游戏"), (2, "等待开局")].into_iter() with step {
                    Column (
                        grow: 1.0,
                        min_width: 0,
                        align: AlignItems::Center,
                        row_gap: metrics.gap
                    ) {
                        Text (
                            (step.0 + 1).to_string(),
                            width: if metrics.compact { 22 } else { 28 },
                            height: if metrics.compact { 22 } else { 28 },
                            bg_color: ${ state.steps()[step.0].background },
                            font: FONT,
                            font_size: metrics.small,
                            text_color: ${ state.steps()[step.0].foreground },
                            paragraph: paragraph(Center, 1)
                        ) [
                            FONT_PAINT,
                        ]
                        label (
                            step.1,
                            metrics.small,
                            SECONDARY,
                            Center
                        )
                    }
                }
            }
            View (
                grow: 1.0,
                min_height: 0,
                max_height: 24
            )
            live_label (
                state,
                Headline,
                metrics.title,
                FOREGROUND,
                Center
            )
            live_label (
                state,
                Next,
                metrics.small,
                SECONDARY,
                Center
            )
            if ${ !state.room_name().is_empty() } {
                Text (
                    text: ${ state.room_name() },
                    font: FONT,
                    font_size: metrics.small,
                    text_color: SECONDARY,
                    height: Dimension::Content,
                    paragraph: paragraph(Center, 1)
                ) [
                    FONT_PAINT,
                ]
            }
        }
    }
}

#[compose]
fn leaving(state: model!(DisplayState), metrics: Metrics) -> Entity {
    ui! {
        Column (
            grow: 1.0,
            min_height: 0,
            bg_color: PANEL,
            padding: Padding::all(metrics.inset),
            row_gap: metrics.gap,
            justify: JustifyContent::Center,
            align: AlignItems::Center
        ) {
            Text (
                "B",
                width: 44,
                height: 36,
                bg_color: ACCENT_DIM,
                font: FONT,
                font_size: metrics.title,
                text_color: ACCENT,
                paragraph: paragraph(Center, 1)
            ) [
                FONT_PAINT,
            ]
            live_label (
                state,
                Headline,
                metrics.title,
                FOREGROUND,
                Center
            )
            live_label (
                state,
                Next,
                metrics.small,
                SECONDARY,
                Center
            )
        }
    }
}

#[compose]
fn unavailable(metrics: Metrics) -> Entity {
    ui! {
        Column (
            grow: 1.0,
            min_height: 0,
            bg_color: PANEL,
            padding: Padding::all(metrics.inset),
            row_gap: metrics.gap,
            justify: JustifyContent::Center,
            align: AlignItems::Center
        ) [
            StaticCache,
            OffscreenRender::default(),
        ] {
            Text (
                "!",
                width: 44,
                height: 40,
                bg_color: WARNING_PANEL,
                font: FONT,
                font_size: metrics.title,
                text_color: WARNING,
                paragraph: paragraph(Center, 1)
            ) [
                FONT_PAINT,
            ]
            label (
                "暂时无法进行联机",
                metrics.title,
                FOREGROUND,
                Center
            )
            label (
                "按 B 返回，仍然可以单机游玩",
                metrics.small,
                ACCENT,
                Center
            )
        }
    }
}

fn page_heading(view: &View) -> &str {
    match view.page {
        Page::Mode => "选择连接方式",
        Page::Choice if view.network_only => "和好友建立局域网",
        Page::Choice => "和好友一起，开始游戏",
        Page::Host => {
            if view.room_name.is_empty() {
                "正在创建房间"
            } else {
                &view.room_name
            }
        }
        Page::Rooms => "选择好友的房间",
        Page::Games => &view.title,
        Page::Joined => "加入房间",
        Page::Leaving => "退出联机",
        Page::Unavailable => "附近联机",
    }
}

#[compose]
fn page(view: &View, state: model!(DisplayState), metrics: Metrics) -> Entity {
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
            border_color: ACCENT,
            border_width: 2
        ) [
            mirui::ui::IgnoreHitTest,
            selection::SelectionFrame {
                body,
                page: state.page_key(),
                state: state.clone(),
            },
        ]
    }
}

#[compose]
fn shell(state: model!(DisplayState), metrics: Metrics) -> Entity {
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

fn loading(view: &View) -> bool {
    if view.network_only && view.connected && view.phase == Phase::NetworkConnected {
        return view.busy;
    }
    if matches!(
        phase(view),
        Phase::ApprovalPending
            | Phase::Closed
            | Phase::Failed
            | Phase::RestoreFailed
            | Phase::RoomActive
    ) {
        return false;
    }
    view.busy
        || view.refreshing
        || (view.page == Page::Rooms && view.rooms.is_empty())
        || (view.page == Page::Host && !view.ready)
        || matches!(view.page, Page::Joined | Page::Leaving)
}

fn phase(view: &View) -> Phase {
    if view.phase != Phase::Idle {
        return view.phase;
    }
    match view.page {
        Page::Joined => {
            if view.ready {
                Phase::WaitingStart
            } else {
                Phase::Connecting
            }
        }
        Page::Host => {
            if view.ready {
                Phase::HostReady
            } else {
                Phase::WaitingPeer
            }
        }
        Page::Leaving => Phase::Leaving,
        _ => Phase::Idle,
    }
}

fn headline(view: &View) -> &str {
    match phase(view) {
        Phase::NetworkConfirming => "正在确认连接",
        Phase::NetworkConnected => "组网已完成",
        Phase::NetworkReconnecting => "正在等待连接恢复",
        Phase::ApprovalPendingClient => "等待房主确认",
        Phase::Creating => "正在创建房间",
        Phase::WaitingPeer => "等待好友加入",
        Phase::PeerPreparing => "好友正在准备",
        Phase::ApprovalPending => "好友加入",
        Phase::HostReady => "可以开始游戏",
        Phase::Connecting => "正在连接房间",
        Phase::Checking => "正在核对游戏",
        Phase::Notifying => "正在确认准备完成",
        Phase::WaitingStart => "已就绪，等待房主开局",
        Phase::Starting => "正在启动游戏",
        Phase::Leaving => "正在退出联机",
        Phase::Closed => "本次联机已结束",
        Phase::Failed => "未能完成本次连接",
        Phase::RestoreFailed => "网络尚未恢复",
        Phase::RoomActive => "这台掌机已有房间",
        Phase::Idle => "请选择创建或加入",
    }
}

fn next_step(view: &View) -> &str {
    if view.network_only {
        return match phase(view) {
            Phase::NetworkConnected => "返回系统后，连接继续保持",
            Phase::NetworkConfirming => "正在确认双方的局域网连接",
            Phase::NetworkReconnecting => "持续失联后会恢复原 Wi-Fi",
            Phase::ApprovalPendingClient => "请房主按 A 确认加入",
            Phase::Connecting => "正在连接房间并获取地址",
            Phase::Leaving => "正在断开连接并恢复 Wi-Fi",
            _ => "选择房间，与附近掌机组网",
        };
    }
    match phase(view) {
        Phase::NetworkConnected
        | Phase::NetworkConfirming
        | Phase::NetworkReconnecting
        | Phase::ApprovalPendingClient => "正在确认连接",
        Phase::Creating => "房间准备好后，好友才能加入",
        Phase::WaitingPeer => "请好友选择这个房间加入",
        Phase::PeerPreparing => "准备完成后，可以开始游戏",
        Phase::ApprovalPending => "按 A 确认，B 关闭房间",
        Phase::HostReady => "按 A 开始，双方一起进入游戏",
        Phase::Connecting => "连接完成后，会核对两台的游戏",
        Phase::Checking => "正在确认两台的游戏一致",
        Phase::Notifying => "房主确认就绪后，才能开始游戏",
        Phase::WaitingStart => "请房主按 A 开始游戏",
        Phase::Starting => "正在准备双方的联机画面",
        Phase::Leaving => "关闭本次连接，恢复原网络",
        Phase::Closed | Phase::Failed => "按 B 返回，重新创建或选择房间",
        Phase::RestoreFailed => "按 A 重试，完成后再返回",
        Phase::RoomActive => "按 A 关闭上一场后继续",
        Phase::Idle => "方向键选择，按 A 确认",
    }
}

fn button_text(view: &View) -> &str {
    if view.network_only && view.connected && view.phase == Phase::NetworkConnected {
        return "保持连接，返回系统";
    }
    match phase(view) {
        Phase::HostReady => {
            if view.busy {
                "正在开局"
            } else {
                "开始游戏"
            }
        }
        Phase::Creating => "房间准备中",
        Phase::PeerPreparing => "等待好友准备",
        Phase::ApprovalPending => "确认加入",
        Phase::Starting => "正在开局",
        Phase::Closed | Phase::Failed => "按 B 返回",
        Phase::Leaving | Phase::RestoreFailed => "正在退出房间",
        _ => "等待好友加入",
    }
}

fn peer_text(view: &View) -> &str {
    match phase(view) {
        Phase::HostReady | Phase::Starting => "好友已就绪",
        Phase::PeerPreparing => "正在核对",
        Phase::ApprovalPending => "等待确认",
        Phase::Closed | Phase::Failed | Phase::Leaving => "联机已结束",
        _ => "等待好友",
    }
}

fn status_text(view: &View) -> &str {
    if !view.status.is_empty() {
        return &view.status;
    }
    match view.page {
        Page::Mode => "请选择仅组网或选择游戏",
        Page::Choice => "两台掌机选择同一款游戏，再创建或加入房间",
        Page::Host => "请好友选择同一款游戏，加入这个房间",
        Page::Rooms => "选择房间后确认加入，列表会自动刷新",
        Page::Joined => next_step(view),
        Page::Leaving => "正在结束本次联机...",
        Page::Unavailable => "这款游戏暂时不支持附近联机",
        Page::Games => "选择游戏后创建或加入房间",
    }
}

fn hint_text(view: &View) -> &str {
    if view.network_only && view.connected && view.phase == Phase::NetworkConnected {
        return "连接保持中";
    }
    if view.page == Page::Games {
        return if view.busy {
            "正在核对游戏"
        } else {
            "L/R 翻页"
        };
    }
    match phase(view) {
        Phase::RestoreFailed => "按 A 重试退出",
        Phase::RoomActive => "A 关闭后继续",
        Phase::Leaving => "等待网络恢复",
        Phase::WaitingStart => "等待房主开局",
        Phase::HostReady => "A 开始游戏",
        Phase::ApprovalPending => "按 A 确认",
        Phase::Creating | Phase::WaitingPeer | Phase::PeerPreparing => "B 关闭房间",
        Phase::Connecting | Phase::Checking | Phase::Notifying => "B 取消加入",
        Phase::Starting => "正在进入游戏",
        Phase::Closed | Phase::Failed => "B 返回后重试",
        _ => {
            if view.page == Page::Rooms {
                "自动搜索"
            } else {
                "方向键选择"
            }
        }
    }
}

fn confirm_text(view: &View) -> &str {
    if view.network_only && view.connected && view.phase == Phase::NetworkConnected && !view.busy {
        return "返回系统";
    }
    match phase(view) {
        Phase::RestoreFailed => "重试",
        Phase::RoomActive => "关闭",
        Phase::Closed | Phase::Failed => "返回",
        Phase::HostReady if !view.busy => "开始",
        Phase::ApprovalPending if !view.busy => "确认",
        Phase::Idle => match view.page {
            Page::Choice | Page::Mode => "确认",
            Page::Rooms => "加入",
            Page::Games if !view.busy && !view.entries.is_empty() => "选择",
            _ => "",
        },
        _ => "",
    }
}

#[compose]
fn display_state() -> DisplayHandle {
    res!(DisplayRuntime).with(|runtime| runtime.state.clone())
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
    let state = app.compose(root, display_state());
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
        app.compose(root, shell(state.clone(), metrics));
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
