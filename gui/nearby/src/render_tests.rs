use super::*;
use super::{copy::*, selection::FocusTarget};
use crate::feedback::Feedback;
use crate::model::{GameRow, Page, Phase, RoomRow};
use mirui::core::model::ModelHandle;
use mirui::render::texture::ColorFormat;
use mirui::surface::FramebufferAccess;
use mirui::surface::framebuf::FramebufSurface;
use mirui::types::PhysicalRect;
use mirui::ui::ComputedRect;
use mirui::ui::OffscreenGeneration;
use mirui::ui::render_system::LastDirtyRegions;
use mirui::ui::widgets::FollowBounds;
use mirui::ui::widgets::Text;
use std::cell::Cell;
use std::rc::Rc;

type TestApp = App<FramebufSurface<fn(&[u8], PhysicalRect)>>;

thread_local! { static TIME: Cell<u64> = const { Cell::new(0) }; }

fn now_ns() -> u64 {
    TIME.with(Cell::get)
}

fn body(app: &TestApp) -> Entity {
    app.world
        .query::<selection::SelectionFrame>()
        .iter()
        .find(|(_, frame)| frame.page == frame.state.page_key())
        .unwrap()
        .1
        .body
}

fn focus_rect(app: &TestApp) -> Option<[i32; 4]> {
    let (entity, _) = app
        .world
        .query::<selection::SelectionFrame>()
        .iter()
        .find(|(_, frame)| frame.page == frame.state.page_key())?;
    let target = app.world.get::<FollowBounds>(entity)?.target;
    let rect = app.world.get::<ComputedRect>(target)?.0;
    Some([
        rect.x.to_int(),
        rect.y.to_int(),
        rect.w.to_int(),
        rect.h.to_int(),
    ])
}

fn paint_at<B: FramebufferAccess>(
    app: &mut App<B>,
    feedback: &mut Feedback,
    scene: &Scene,
    time: u64,
) {
    let mut current = now_ns() / 1_000_000;
    while current < time {
        current = (current + 16).min(time);
        TIME.with(|clock| clock.set(current * 1_000_000));
        feedback.paint(app, scene, current).unwrap();
    }
}

fn offset_focus(app: &mut TestApp, axis: usize) {
    let node = app
        .world
        .query::<selection::SelectionFrame>()
        .iter()
        .next()
        .unwrap()
        .0;
    let bounds = app.world.get_mut::<FollowBounds>(node).unwrap();
    bounds.transition = None;
    if axis == 0 {
        bounds.offset.x += Fixed::ONE;
    } else {
        bounds.offset.y += Fixed::ONE;
    }
}

fn app(size: [u16; 2]) -> TestApp {
    app_with_format(size, ColorFormat::RGBA8888)
}

fn app_with_format(size: [u16; 2], format: ColorFormat) -> TestApp {
    let surface = FramebufSurface::with_format(
        size[0],
        size[1],
        format,
        (|_, _| {}) as fn(&[u8], PhysicalRect),
    );
    let mut app = App::new(surface);
    app.with_default_widgets().with_default_systems();
    install(&mut app).unwrap();
    TIME.with(|clock| clock.set(0));
    app.add_resource(mirui::ecs::MonoClock::new(now_ns));
    app
}

fn view(page: Page) -> View {
    View {
        page,
        title: "游戏标题 / Multiplayer".into(),
        core: "NES / FC".into(),
        room_name: "房间 A216".into(),
        rooms: (0..4)
            .map(|i| RoomRow {
                name: format!("房间 {i}"),
                detail: "同一游戏，可以加入".into(),
                signal: 70,
                available: true,
            })
            .collect(),
        entries: (0..4)
            .map(|i| GameRow {
                name: format!("游戏 {i} / Multiplayer"),
                detail: "NES / FC".into(),
                directory: false,
            })
            .collect(),
        status: "请选择".into(),
        ..View::default()
    }
}

fn section(app: &TestApp, kind: Section) -> Entity {
    let root = body(app);
    app.world
        .query::<Section>()
        .iter()
        .find(|(entity, section)| {
            **section == kind
                && selection::contains(root, *entity, |node| {
                    app.world
                        .get::<mirui::ui::Parent>(node)
                        .map(|parent| parent.0)
                })
        })
        .unwrap()
        .0
}

fn rect(app: &TestApp, id: &'static str) -> Rect {
    let kind = match id {
        "nearby_header" => Section::Header,
        "nearby_content" => Section::Content,
        "nearby_footer" => Section::Footer,
        "nearby_status" => Section::Status,
        _ => panic!("unknown section"),
    };
    app.world.get::<ComputedRect>(section(app, kind)).unwrap().0
}

fn cache_generations(app: &TestApp) -> Vec<(Entity, u32)> {
    app.world
        .query::<StaticCache>()
        .iter()
        .map(|(entity, _)| {
            (
                entity,
                app.world
                    .get::<OffscreenGeneration>(entity)
                    .map_or(0, |v| v.0),
            )
        })
        .collect()
}

fn assert_full_frame_matches(app: &mut TestApp, context: &str) {
    let incremental = app.backend.framebuffer().buf.as_slice().to_vec();
    app.render().unwrap();
    let full_frame = app.backend.framebuffer();
    let full = full_frame.buf.as_slice();
    assert_eq!(
        incremental.iter().zip(full).filter(|(a, b)| a != b).count(),
        0,
        "{context}"
    );
}

#[test]
fn dynamic_updates_preserve_static_caches_and_one_model_registration() {
    for page in [
        Page::Mode,
        Page::Choice,
        Page::Rooms,
        Page::Games,
        Page::Host,
        Page::Joined,
        Page::Leaving,
        Page::Unavailable,
    ] {
        let mut app = app([640, 480]);
        let initial = view(page);
        let mut scene = draw(&mut app, &initial).unwrap();
        let registered = scene.state.clone();
        let identity = registered.read(|model| std::ptr::from_ref(model) as usize);
        let generations = cache_generations(&app);
        assert!(!generations.is_empty());
        let mut next = initial.clone();
        next.status = "正在准备".into();
        update(&mut scene, &next).unwrap();
        let mut motion = Feedback::default();

        paint_at(&mut app, &mut motion, &scene, 70);
        assert_eq!(cache_generations(&app), generations, "{page:?}");
        let previous_root = app.root.unwrap();
        let previous_page = body(&app);
        let next_scene = draw(
            &mut app,
            &view(if page == Page::Choice {
                Page::Mode
            } else {
                Page::Choice
            }),
        )
        .unwrap();
        paint_at(&mut app, &mut motion, &next_scene, 1500);
        assert_eq!(app.root, Some(previous_root));
        assert!(app.world.is_alive(previous_root));
        assert!(!app.world.is_alive(previous_page));
        assert_eq!(
            identity,
            next_scene
                .state
                .read(|model| std::ptr::from_ref(model) as usize)
        );
        drop(scene);
        drop(next_scene);
        drop(app);
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| registered.status())).is_err(),
            "Model lifetime must end with its App"
        );
    }
}

#[test]
fn text_fields_invalidate_their_own_bindings_and_old_page_bindings_are_released() {
    let mut app = app([640, 480]);
    let initial = view(Page::Choice);
    let mut scene = draw(&mut app, &initial).unwrap();
    let texts: Vec<_> = app.world.query::<Text>().iter().map(|(e, _)| e).collect();
    let status_entity = section(&app, Section::Status);
    let mut next = initial.clone();
    next.status = "准备完成".into();
    update(&mut scene, &next).unwrap();
    app.render_dirty().unwrap();
    assert_eq!(
        app.world
            .get::<Text>(status_entity)
            .unwrap()
            .resolve(app.world.resource())
            .as_ref(),
        "准备完成"
    );
    scene.navigation_intent(false);
    scene.state.present(&view(Page::Unavailable));
    let mut feedback = Feedback::default();
    paint_at(&mut app, &mut feedback, &scene, 1500);
    assert!(texts.iter().all(|e| !app.world.is_alive(*e)));
    let next_texts: Vec<_> = app
        .world
        .query::<Text>()
        .iter()
        .map(|(entity, _)| entity)
        .collect();
    assert!(!next_texts.is_empty());
    scene.state.present(&view(Page::Unavailable));
    app.render_dirty().unwrap();
    assert!(next_texts.iter().all(|e| app.world.is_alive(*e)));
}

#[test]
fn responsive_layout_keeps_focus_and_text_sections_inside_the_viewport() {
    for size in [[480, 320], [640, 480], [800, 480]] {
        for page in [
            Page::Mode,
            Page::Choice,
            Page::Rooms,
            Page::Games,
            Page::Host,
            Page::Joined,
            Page::Leaving,
            Page::Unavailable,
        ] {
            let mut app = app(size);
            let mut view = view(page);
            view.ready = true;
            view.selected = 3;
            view.status = "正在核对两台设备的游戏和模拟核心，请稍候。Preparing the selected game and checking the connection before starting multiplayer.".into();
            view.title = "较长的游戏标题 / A long multiplayer game title that should fit its available space".into();
            let _scene = draw(&mut app, &view).unwrap();
            let header = rect(&app, "nearby_header");
            let body = rect(&app, "nearby_content");
            let footer = rect(&app, "nearby_footer");
            let status = rect(&app, "nearby_status");
            assert!(header.y + header.h <= body.y, "{size:?} {page:?}");
            assert!(body.y + body.h <= footer.y, "{size:?} {page:?}");
            assert!(status.y + status.h <= body.y + body.h, "{size:?} {page:?}");
            assert!(footer.y + footer.h <= Fixed::from_int(i32::from(size[1])));
            for (entity, _) in app.world.query::<Text>().iter() {
                let text = app.world.get::<ComputedRect>(entity).unwrap().0;
                let mut ancestor = entity;
                while let Some(parent) = app.world.get::<mirui::ui::Parent>(ancestor) {
                    ancestor = parent.0;
                    if app.world.get::<FocusTarget>(ancestor).is_some() {
                        let row = app.world.get::<ComputedRect>(ancestor).unwrap().0;
                        assert!(
                            text.y >= row.y && text.y + text.h <= row.y + row.h,
                            "{size:?} {page:?}: text {text:?} escapes row {row:?}"
                        );
                        break;
                    }
                }
            }
            if let Some([x, y, w, h]) = focus_rect(&app) {
                assert!(w > 40 && h >= 26, "{size:?} {page:?}: {w}×{h}");
                assert!(
                    x >= 0 && y >= 0 && x + w <= i32::from(size[0]) && y + h <= footer.y.to_int(),
                    "{size:?} {page:?}: focus {:?} escapes footer {footer:?}",
                    [x, y, w, h]
                );
            }
        }
    }
}

#[test]
fn network_status_keeps_connection_actions_inside_the_viewport() {
    for size in [[480, 320], [640, 480], [800, 480]] {
        for page in [Page::Host, Page::Joined] {
            let mut app = app(size);
            let mut view = view(page);
            view.network_only = true;
            view.connected = true;
            view.phase = Phase::NetworkConnected;
            view.local_address = "192.168.49.123".into();
            view.back_label = "断开并恢复 Wi-Fi".into();
            view.peers = (0..3)
                .map(|index| crate::model::NetworkPeer {
                    name: format!("好友 {index} / Handheld"),
                    address: format!("192.168.49.{}", index + 2),
                })
                .collect();
            let _scene = draw(&mut app, &view).unwrap();
            let footer = rect(&app, "nearby_footer");
            let [x, y, w, h] = focus_rect(&app).unwrap();
            assert!(x >= 0 && y >= 0 && w > 40 && h >= 26);
            assert!(x + w <= i32::from(size[0]) && y + h <= footer.y.to_int());
            assert!(footer.y + footer.h <= Fixed::from_int(i32::from(size[1])));
            assert_eq!(confirm_text(&view), "返回系统");
        }
    }
}

#[test]
fn reflow_and_reverse_retarget_match_complete_frames_at_every_viewport() {
    for size in [[480, 320], [640, 480], [800, 480]] {
        for page in [Page::Choice, Page::Rooms, Page::Games] {
            let mut app = app(size);
            let mut view = view(page);
            let mut scene = draw(&mut app, &view).unwrap();
            let original = focus_rect(&app).unwrap();
            let mut motion = Feedback::default();

            for (time, selected, status) in [
                (
                    10,
                    1,
                    "正在核对两台设备的游戏和模拟核心，请稍候。Preparing the selected game and checking the connection before starting multiplayer.",
                ),
                (70, 0, "Ready"),
                (140, 1, "连接准备完成"),
                (400, 1, "连接准备完成"),
            ] {
                view.selected = selected;
                view.status = status.into();
                update(&mut scene, &view).unwrap();
                paint_at(&mut app, &mut motion, &scene, time + 16);
                if time == 10 {
                    assert_ne!(
                        focus_rect(&app).unwrap()[1],
                        original[1],
                        "Text must reflow before retarget"
                    );
                }

                assert_full_frame_matches(&mut app, &format!("{size:?} {page:?} at {time}"));
            }
            paint_at(&mut app, &mut motion, &scene, 1500);
            let frame = app
                .world
                .query::<selection::SelectionFrame>()
                .iter()
                .next()
                .unwrap()
                .0;
            let target = focus_rect(&app).unwrap();
            let current = app.world.get::<ComputedRect>(frame).unwrap().0;
            assert_eq!(
                [
                    current.x.to_int(),
                    current.y.to_int(),
                    current.w.to_int(),
                    current.h.to_int()
                ],
                target
            );
        }
    }
}

#[test]
fn semantic_phase_transitions_match_complete_frames() {
    for page in [Page::Host, Page::Joined, Page::Leaving] {
        let mut app = app([640, 480]);
        let mut previous = view(page);
        let mut scene = draw(&mut app, &previous).unwrap();
        let mut motion = Feedback::default();
        for (i, phase) in [
            Phase::ApprovalPending,
            Phase::PeerPreparing,
            Phase::HostReady,
            Phase::Connecting,
            Phase::Checking,
            Phase::Notifying,
            Phase::WaitingStart,
            Phase::Starting,
            Phase::Leaving,
            Phase::RestoreFailed,
        ]
        .into_iter()
        .enumerate()
        {
            let mut next = previous.clone();
            next.phase = phase;
            next.ready = phase == Phase::HostReady;
            next.back_label = if phase == Phase::Leaving {
                "正在退出"
            } else {
                "返回"
            }
            .into();
            update(&mut scene, &next).unwrap();

            paint_at(&mut app, &mut motion, &scene, i as u64 * 300 + 250);
            assert_full_frame_matches(&mut app, &format!("{page:?} {phase:?}"));
            previous = next;
        }
    }
}

#[test]
fn focus_damage_stays_disjoint_and_avoids_unchanged_interiors() {
    for page in [Page::Choice, Page::Rooms] {
        let mut app = app([640, 480]);
        let _scene = draw(&mut app, &view(page)).unwrap();
        let mut target = focus_rect(&app).unwrap();
        target[usize::from(page == Page::Rooms)] += 1;
        offset_focus(&mut app, usize::from(page == Page::Rooms));
        app.render_dirty().unwrap();
        let plan = &app.world.resource::<LastDirtyRegions>().unwrap().0;
        assert!(plan.rects.len() <= 4);
        let area: i32 = plan.rects.iter().map(|r| r.w.to_int() * r.h.to_int()).sum();
        assert!(area < (target[2] + target[3]) * 8);
        for (i, a) in plan.rects.iter().enumerate() {
            for b in &plan.rects[i + 1..] {
                assert!(
                    a.x >= b.x + b.w || b.x >= a.x + a.w || a.y >= b.y + b.h || b.y >= a.y + a.h
                );
            }
        }
        assert_full_frame_matches(&mut app, "narrow focus damage");
    }
}

#[test]
fn unchanged_idle_frames_do_not_submit_pixels() {
    let pixels = Rc::new(Cell::new(0_u32));
    let output = pixels.clone();
    let surface = FramebufSurface::with_format(640, 480, ColorFormat::RGBA8888, move |_, area| {
        output.set(output.get() + u32::from(area.width()) * u32::from(area.height()))
    });
    let mut app = App::new(surface);
    app.with_default_widgets().with_default_systems();
    install(&mut app).unwrap();
    let view = view(Page::Choice);
    let mut scene = draw(&mut app, &view).unwrap();
    TIME.with(|clock| clock.set(0));
    app.add_resource(mirui::ecs::MonoClock::new(now_ns));
    let mut motion = Feedback::default();

    paint_at(&mut app, &mut motion, &scene, 1500);
    pixels.set(0);
    update(&mut scene, &view).unwrap();
    paint_at(&mut app, &mut motion, &scene, 1800);

    assert_eq!(pixels.get(), 0);
}

#[test]
fn readiness_is_the_only_source_of_waiting_for_host_copy() {
    let mut view = view(Page::Joined);
    view.status.clear();
    for phase in [
        Phase::Connecting,
        Phase::Checking,
        Phase::Notifying,
        Phase::WaitingStart,
        Phase::Starting,
        Phase::RestoreFailed,
    ] {
        view.phase = phase;
        assert_eq!(status_text(&view), next_step(&view));
        if matches!(
            phase,
            Phase::Connecting | Phase::Checking | Phase::Notifying
        ) {
            assert_ne!(headline(&view), "已就绪，等待房主开局");
            assert_ne!(status_text(&view), "请房主按 A 开始游戏");
        }
    }
    view.status = "设备提供的当前状态".into();
    assert_eq!(status_text(&view), view.status);
    view.page = Page::Host;
    view.phase = Phase::ApprovalPending;
    assert!(host_action_enabled(&view));
    let approval = view.clone();
    view.phase = Phase::HostReady;
    view.ready = true;
    assert!(host_action_enabled(&view));
    let mut app = app([640, 480]);
    let mut scene = draw(&mut app, &approval).unwrap();
    let page = body(&app);
    update(&mut scene, &view).unwrap();
    app.tick();
    assert_eq!(body(&app), page);
    view.busy = true;
    assert!(!host_action_enabled(&view));
}

fn page_frames(app: &TestApp) -> Vec<(Entity, Page)> {
    app.world
        .query::<selection::SelectionFrame>()
        .iter()
        .map(|(_, frame)| (frame.body, frame.page.page))
        .collect()
}

fn translation(app: &TestApp, mut node: Entity) -> Fixed {
    loop {
        if let Some(transform) = app.world.get::<mirui::ui::widgets::WidgetTransform>(node) {
            return transform.0.apply_point(mirui::types::Point::ZERO).x;
        }
        node = app.world.get::<mirui::ui::Parent>(node).unwrap().0;
    }
}

fn advance_checked(app: &mut TestApp, feedback: &mut Feedback, scene: &Scene, until: u64) {
    while now_ns() / 1_000_000 < until {
        let next = (now_ns() / 1_000_000 + 16).min(until);
        paint_at(app, feedback, scene, next);
        assert_full_frame_matches(app, "interrupted page transition");
    }
}

#[test]
fn rapid_return_reuses_the_page_and_interrupted_navigation_keeps_the_latest_target() {
    let mut app = app([480, 320]);
    let mut scene = draw(&mut app, &view(Page::Mode)).unwrap();
    let root = app.root;
    let original = body(&app);
    let mut feedback = Feedback::default();
    scene.navigation_intent(false);
    update(&mut scene, &view(Page::Choice)).unwrap();
    advance_checked(&mut app, &mut feedback, &scene, 16);
    let incoming = body(&app);
    let entered_from = translation(&app, incoming);
    assert!(entered_from > Fixed::ZERO);
    advance_checked(&mut app, &mut feedback, &scene, 64);
    assert!(translation(&app, incoming) < entered_from);

    scene.navigation_intent(true);
    update(&mut scene, &view(Page::Mode)).unwrap();
    advance_checked(&mut app, &mut feedback, &scene, 80);
    assert_eq!(body(&app), original);
    advance_checked(&mut app, &mut feedback, &scene, 1200);
    assert_eq!(page_frames(&app), vec![(original, Page::Mode)]);
    assert!(!app.world.is_alive(incoming));
    assert_eq!(app.root, root);

    scene.navigation_intent(false);
    update(&mut scene, &view(Page::Choice)).unwrap();
    advance_checked(&mut app, &mut feedback, &scene, 1216);
    let retired = page_frames(&app);
    update(&mut scene, &view(Page::Host)).unwrap();
    advance_checked(&mut app, &mut feedback, &scene, 1232);
    update(&mut scene, &view(Page::Rooms)).unwrap();
    advance_checked(&mut app, &mut feedback, &scene, 3000);
    assert_eq!(page_frames(&app).len(), 1);
    assert_eq!(page_frames(&app)[0].1, Page::Rooms);
    assert!(retired.iter().all(|(node, _)| !app.world.is_alive(*node)));
    assert_eq!(app.root, root);
    assert!(focus_rect(&app).is_some());
}

#[test]
fn refreshed_room_rows_keep_the_page_and_rebind_selection_to_live_entities() {
    let mut app = app([480, 320]);
    let mut next = view(Page::Rooms);
    let mut scene = draw(&mut app, &next).unwrap();
    let page = body(&app);
    let mut feedback = Feedback::default();
    let old_targets: Vec<_> = app
        .world
        .query::<FocusTarget>()
        .iter()
        .map(|(node, _)| node)
        .collect();
    next.rooms.clear();
    next.selected = usize::MAX;
    update(&mut scene, &next).unwrap();
    advance_checked(&mut app, &mut feedback, &scene, 64);
    assert_eq!(body(&app), page);
    assert!(old_targets.iter().all(|node| !app.world.is_alive(*node)));
    next.rooms = view(Page::Rooms).rooms;
    next.selected = 2;
    update(&mut scene, &next).unwrap();
    advance_checked(&mut app, &mut feedback, &scene, 1200);
    assert_eq!(body(&app), page);
    let frame = app
        .world
        .query::<selection::SelectionFrame>()
        .iter()
        .next()
        .unwrap()
        .0;
    let selected = app.world.get::<FollowBounds>(frame).unwrap().target;
    assert!(app.world.is_alive(selected));
    assert_eq!(app.world.get::<FocusTarget>(selected).unwrap().index, 2);
    assert!(old_targets.iter().all(|node| *node != selected));
}

#[test]
fn menu_handoffs_animate_without_blank_frames_in_panel_formats() {
    for format in [ColorFormat::BGRA8888, ColorFormat::RGB565] {
        let mut app = app_with_format([640, 480], format);
        let mut scene = draw(&mut app, &view(Page::Mode)).unwrap();
        let root = app.root;
        let mut feedback = Feedback::default();
        for (page, network_only, backward) in [
            (Page::Games, false, false),
            (Page::Choice, false, false),
            (Page::Host, false, false),
            (Page::Choice, false, true),
            (Page::Mode, false, true),
            (Page::Choice, true, false),
            (Page::Rooms, true, false),
            (Page::Mode, false, true),
        ] {
            let mut next = view(page);
            next.network_only = network_only;
            scene.navigation_intent(backward);
            update(&mut scene, &next).unwrap();
            let start = now_ns() / 1_000_000;
            paint_at(&mut app, &mut feedback, &scene, start + 16);
            assert_eq!(app.root, root);
            assert_eq!(
                page_frames(&app).len(),
                2,
                "{page:?}: outgoing page must stay mounted"
            );
            let offset = translation(&app, body(&app));
            assert!(if backward {
                offset < Fixed::ZERO
            } else {
                offset > Fixed::ZERO
            });
            while now_ns() / 1_000_000 < start + 480 {
                let time = now_ns() / 1_000_000 + 16;
                paint_at(&mut app, &mut feedback, &scene, time);
                for y in (0..480).step_by(16) {
                    for x in (0..640).step_by(16) {
                        let color = app.backend.framebuffer().get_pixel(x, y);
                        assert!(
                            color.r != 0 || color.g != 0 || color.b != 0,
                            "{format:?} {page:?}: black pixel at {x},{y}"
                        );
                    }
                }
                assert_full_frame_matches(&mut app, "continuous menu handoff");
            }
            assert_eq!(page_frames(&app).len(), 1);
        }
    }
}
