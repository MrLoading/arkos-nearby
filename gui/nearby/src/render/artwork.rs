use std::collections::VecDeque;
use std::path::Path;
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::time::{Duration, Instant};

use mirui::app::RendererFactory;
use mirui::core::cache::MaxSize;
use mirui::core::resource::ResourceManager;
use mirui::prelude::*;
use mirui::render::texture::{ColorFormat, Texture};
use mirui::surface::Surface;
use mirui::ui::widgets::Image;

use super::consoles::console;
use super::{state::DisplayState, style::*};

const PREVIEW_WIDTH: u32 = 256;
const PREVIEW_HEIGHT: u32 = 192;
const DEBOUNCE: Duration = Duration::from_millis(120);
const CACHE_ENTRIES: usize = 8;
type ArtworkKey = (String, Option<(u64, Option<std::time::SystemTime>)>);

fn texture_token(id: u64) -> String {
    format!("nearby-cover-{id}")
}

fn source_key(path: String) -> ArtworkKey {
    let stamp = std::fs::metadata(&path)
        .ok()
        .map(|meta| (meta.len(), meta.modified().ok()));
    (path, stamp)
}

#[model]
#[derive(Default)]
struct Artwork {
    #[observe]
    token: Option<u64>,
}

#[model]
impl Artwork {
    fn show(&mut self, token: Option<u64>) {
        self.token = token;
    }
}

#[derive(Clone)]
struct Pixels {
    bytes: Vec<u8>,
    width: u16,
    height: u16,
}

struct Loader {
    model: ArtworkHandle,
    send: SyncSender<ArtworkKey>,
    receive: Receiver<(ArtworkKey, Option<Pixels>)>,
    path: String,
    key: ArtworkKey,
    changed: Instant,
    pending: bool,
    serial: u64,
    cached: VecDeque<(ArtworkKey, Option<u64>)>,
    in_flight: Vec<ArtworkKey>,
    neighbors: Vec<String>,
    prefetch: VecDeque<ArtworkKey>,
}

impl Loader {
    // A resident resource is reusable without decoding, copying pixels or waiting.
    fn cached(
        &mut self,
        key: &ArtworkKey,
        textures: &ResourceManager<Texture<'static>>,
    ) -> Option<Option<u64>> {
        let index = self.cached.iter().position(|(old, _)| old == key)?;
        let entry = self.cached.remove(index)?;
        if entry
            .1
            .as_ref()
            .is_some_and(|token| textures.resolve(&texture_token(*token)).width == 1)
        {
            if let Some(token) = &entry.1 {
                textures.remove_token(&texture_token(*token));
            }
            return None;
        }
        let token = entry.1;
        self.cached.push_back(entry);
        Some(token)
    }

    fn enqueue(&mut self, key: ArtworkKey) -> bool {
        if self.in_flight.contains(&key) {
            return true;
        }
        if self.in_flight.len() >= 2 || self.send.try_send(key.clone()).is_err() {
            return false;
        }
        self.in_flight.push(key);
        true
    }

    fn accept(
        &mut self,
        key: ArtworkKey,
        pixels: Option<Pixels>,
        textures: &ResourceManager<Texture<'static>>,
    ) {
        self.in_flight.retain(|old| old != &key);
        // A result for another selection can populate the cache, but cannot change the view.
        let current = source_key(key.0.clone());
        if current != key {
            if self.key == key {
                self.key = current;
                self.pending = true;
            }
            return;
        }
        if let Some(index) = self.cached.iter().position(|(old, _)| old == &key) {
            if let Some(token) = self.cached.remove(index).and_then(|entry| entry.1) {
                textures.remove_token(&texture_token(token));
            }
        }
        let token = pixels.and_then(|pixels| {
            let texture = Texture::from_vec(
                pixels.bytes,
                pixels.width,
                pixels.height,
                ColorFormat::RGBA8888,
            )?;
            self.serial += 1;
            let token = self.serial;
            textures.add_factory(texture_token(token), move || Some(texture));
            textures.prefetch(&texture_token(token));
            Some(token)
        });
        while self.cached.len() >= CACHE_ENTRIES {
            if let Some((_, Some(token))) = self.cached.pop_front() {
                textures.remove_token(&texture_token(token));
            }
        }
        self.cached.push_back((key.clone(), token));
        if key == self.key {
            self.model.show(token);
        }
    }
}

fn decode(path: &str) -> Option<Pixels> {
    let path = Path::new(path).canonicalize().ok()?;
    if !["/roms", "/roms2"]
        .iter()
        .filter_map(|root| Path::new(root).canonicalize().ok())
        .any(|root| path.starts_with(root))
        || path.metadata().ok()?.len() > 4 * 1024 * 1024
    {
        return None;
    }
    let mut reader = image::ImageReader::open(path)
        .ok()?
        .with_guessed_format()
        .ok()?;
    let mut limits = image::Limits::default();
    limits.max_image_width = Some(4096);
    limits.max_image_height = Some(4096);
    limits.max_alloc = Some(24 * 1024 * 1024);
    reader.limits(limits);
    let image = reader
        .decode()
        .ok()?
        .thumbnail(PREVIEW_WIDTH, PREVIEW_HEIGHT)
        .to_rgba8();
    let mut canvas = image::RgbaImage::new(PREVIEW_WIDTH, PREVIEW_HEIGHT);
    image::imageops::overlay(
        &mut canvas,
        &image,
        i64::from((PREVIEW_WIDTH - image.width()) / 2),
        i64::from((PREVIEW_HEIGHT - image.height()) / 2),
    );
    Some(Pixels {
        width: PREVIEW_WIDTH as u16,
        height: PREVIEW_HEIGHT as u16,
        bytes: canvas.into_raw(),
    })
}

pub(super) fn install<B: Surface, F: RendererFactory<B>>(app: &mut App<B, F>) {
    let (send, jobs) = mpsc::sync_channel::<ArtworkKey>(1);
    let (results, receive) = mpsc::sync_channel(1);
    std::thread::spawn(move || {
        let mut cache: VecDeque<(ArtworkKey, Option<Pixels>)> = VecDeque::new();
        while let Ok(key) = jobs.recv() {
            let pixels = if let Some(index) = cache.iter().position(|(old, _)| old == &key) {
                let item = cache.remove(index).expect("Located cached artwork");
                let pixels = item.1.clone();
                cache.push_back(item);
                pixels
            } else {
                let pixels = decode(&key.0);
                if cache.len() == CACHE_ENTRIES {
                    cache.pop_front();
                }
                cache.push_back((key.clone(), pixels.clone()));
                pixels
            };
            if results.send((key, pixels)).is_err() {
                break;
            }
        }
    });
    let model = app.add_model(Artwork::default()).finish();
    app.add_resource(Loader {
        model,
        send,
        receive,
        path: String::new(),
        key: (String::new(), None),
        changed: Instant::now(),
        pending: false,
        serial: 0,
        cached: VecDeque::new(),
        in_flight: Vec::new(),
        neighbors: Vec::new(),
        prefetch: VecDeque::new(),
    });
    if app
        .world
        .resource::<ResourceManager<Texture<'static>>>()
        .is_none()
    {
        app.add_resource(ResourceManager::new(
            MaxSize::Bytes(1024 * 1024),
            Texture::owned(1, 1, ColorFormat::RGBA8888),
        ));
    }
    let root = app.root.unwrap_or_else(|| app.spawn_root().id());
    let display = app.compose(root, super::state::display_state());
    app.add_system(update::system(display));
}

#[system(order = Animation)]
fn update(
    state: model!(DisplayState),
    loader: res!(mut Loader),
    textures: res!(ResourceManager<Texture<'static>>),
) {
    let path = state.artwork();
    let neighbors = state.artwork_neighbors();
    if path != loader.path || neighbors != loader.neighbors {
        loader.prefetch = neighbors
            .iter()
            .filter(|path| !path.is_empty())
            .take(2)
            .cloned()
            .map(source_key)
            .collect();
        loader.neighbors = neighbors;
    }
    if path != loader.path {
        loader.path = path;
        loader.key = source_key(loader.path.clone());
        loader.changed = Instant::now();
        loader.pending = !loader.path.is_empty();
        let key = loader.key.clone();
        if let Some(token) = loader.cached(&key, &textures) {
            loader.model.show(token);
            loader.pending = false;
        } else {
            loader.model.show(None);
        }
    }
    if loader.pending && loader.changed.elapsed() >= DEBOUNCE {
        let key = loader.key.clone();
        if loader.enqueue(key) {
            loader.pending = false;
        }
    }
    while let Ok((key, pixels)) = loader.receive.try_recv() {
        loader.accept(key, pixels, &textures);
    }
    if !loader.pending && loader.in_flight.is_empty() && loader.changed.elapsed() >= DEBOUNCE {
        while let Some(key) = loader.prefetch.front().cloned() {
            if loader.cached(&key, &textures).is_none() {
                if loader.enqueue(key) {
                    loader.prefetch.pop_front();
                }
                break;
            }
            loader.prefetch.pop_front();
        }
    }
}

#[compose]
pub(super) fn preview(state: model!(DisplayState), metrics: Metrics) -> Entity {
    let art = res!(Loader).with(|loader| loader.model.clone());
    let placeholder = art.clone();
    ui! {
        Column (
            grow: 1.0,
            min_width: 0,
            min_height: 0,
            bg_color: PANEL,
            padding: Padding::all(2),
            justify: JustifyContent::Center,
            row_gap: metrics.gap
        ) {
            Column (grow: 1.0, min_height: 0, min_width: 0, container: true, align: AlignItems::Center, justify: JustifyContent::Center) {
                walk ${ art.token().into_iter() } with token by token {
                    Image (
                        src: texture_token(token),
                        shrink: 0.0,
                        width: @(width, height) { width.min(height * Fixed::from_int(4) / Fixed::from_int(3)) },
                        height: @(width, height) { height.min(width * Fixed::from_int(3) / Fixed::from_int(4)) }
                    )
                }
                if ${ placeholder.token().is_none() } {
                    Column (grow: 1.0, min_height: 0, width: Dimension::percent(100)) {
                        walk ${ std::iter::once(state.selected_platform()) } with platform by platform {
                            console (platform)
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Page, View};
    use mirui::surface::framebuf::FramebufSurface;
    use mirui::ui::ComputedRect;

    #[test]
    fn selected_artwork_rejects_late_results_and_fits_its_container() {
        for (width, height) in [(480, 320), (640, 480)] {
            let surface =
                FramebufSurface::with_format(width, height, ColorFormat::RGBA8888, |_, _| {});
            let mut app = App::new(surface);
            app.with_default_widgets().with_default_systems();
            super::super::install(&mut app).unwrap();
            let mut view = View {
                page: Page::Games,
                artwork: "first".into(),
                ..View::default()
            };
            let mut scene = super::super::draw(&mut app, &view).unwrap();
            super::super::paint_at(&mut app, 10).unwrap();
            let (send, receive) = mpsc::sync_channel(2);
            app.world.resource_mut::<Loader>().unwrap().receive = receive;
            let old_key = app.world.resource::<Loader>().unwrap().key.clone();
            view.artwork = "second".into();
            super::super::update(&mut scene, &view).unwrap();
            super::super::paint_at(&mut app, 20).unwrap();
            send.send((
                old_key,
                Some(Pixels {
                    bytes: vec![255; 16],
                    width: 2,
                    height: 2,
                }),
            ))
            .unwrap();
            super::super::paint_at(&mut app, 30).unwrap();
            let loader = app.world.resource::<Loader>().unwrap();
            assert!(loader.model.token().is_none());
            let current_key = loader.key.clone();
            send.send((
                current_key,
                Some(Pixels {
                    bytes: vec![255; 16],
                    width: 2,
                    height: 2,
                }),
            ))
            .unwrap();
            for time in [40, 50, 60] {
                super::super::paint_at(&mut app, time).unwrap();
            }
            let key = app
                .world
                .resource::<Loader>()
                .unwrap()
                .model
                .token()
                .unwrap();
            let entity = app.world.query::<Image>().iter().find_map(|(entity, image)| {
                matches!(&image.src, mirui::ui::widgets::image::ImageSource::Texture(token) if token == &texture_token(key)).then_some(entity)
            }).unwrap_or_else(|| panic!("Missing {key}; images {:?}", app.world.query::<Image>().iter().map(|(_, image)| format!("{:?}", image.src)).collect::<Vec<_>>()));
            let rect = app.world.get::<ComputedRect>(entity).unwrap().0;
            assert!(rect.w > Fixed::ZERO && rect.h > Fixed::ZERO);
            assert!(
                (rect.w * Fixed::from_int(3) - rect.h * Fixed::from_int(4)).abs()
                    < Fixed::from_int(4)
            );
            assert!(rect.x >= Fixed::ZERO && rect.y >= Fixed::ZERO);
            assert!(rect.x + rect.w <= Fixed::from_int(i32::from(width)));
            assert!(rect.y + rect.h <= Fixed::from_int(i32::from(height)));

            let texture = app
                .world
                .resource::<ResourceManager<Texture<'static>>>()
                .unwrap()
                .resolve(&texture_token(key));
            view.artwork = "first".into();
            super::super::update(&mut scene, &view).unwrap();
            super::super::paint_at(&mut app, 70).unwrap();
            assert!(
                app.world
                    .resource::<Loader>()
                    .unwrap()
                    .model
                    .token()
                    .is_some()
            );
            assert!(!app.world.resource::<Loader>().unwrap().pending);
            view.artwork = "second".into();
            view.artwork_neighbors = vec!["third".into()];
            super::super::update(&mut scene, &view).unwrap();
            super::super::paint_at(&mut app, 80).unwrap();
            assert_eq!(
                app.world.resource::<Loader>().unwrap().model.token(),
                Some(key)
            );
            let reused = app
                .world
                .resource::<ResourceManager<Texture<'static>>>()
                .unwrap()
                .resolve(&texture_token(key));
            assert!(std::rc::Rc::ptr_eq(&texture, &reused));

            let (jobs, queued) = mpsc::sync_channel(1);
            {
                let loader = app.world.resource_mut::<Loader>().unwrap();
                loader.send = jobs;
                loader.changed = Instant::now() - DEBOUNCE;
            }
            super::super::paint_at(&mut app, 90).unwrap();
            let neighbor = queued.try_recv().unwrap();
            assert_eq!(neighbor.0, "third");
            send.send((
                neighbor,
                Some(Pixels {
                    bytes: vec![255; 16],
                    width: 2,
                    height: 2,
                }),
            ))
            .unwrap();
            super::super::paint_at(&mut app, 100).unwrap();
            assert_eq!(
                app.world.resource::<Loader>().unwrap().model.token(),
                Some(key)
            );
            view.artwork = "third".into();
            view.artwork_neighbors.clear();
            super::super::update(&mut scene, &view).unwrap();
            super::super::paint_at(&mut app, 110).unwrap();
            assert!(
                app.world
                    .resource::<Loader>()
                    .unwrap()
                    .model
                    .token()
                    .is_some()
            );
            assert!(!app.world.resource::<Loader>().unwrap().pending);
            assert!(queued.try_recv().is_err());
        }
    }

    #[test]
    fn resource_cache_is_bounded_and_source_changes_retire_results() {
        let surface = FramebufSurface::with_format(480, 320, ColorFormat::RGBA8888, |_, _| {});
        let mut app = App::new(surface);
        app.with_default_widgets().with_default_systems();
        super::super::install(&mut app).unwrap();
        let textures = ResourceManager::new(
            MaxSize::Bytes(32),
            Texture::owned(1, 1, ColorFormat::RGBA8888),
        );
        let loader = app.world.resource_mut::<Loader>().unwrap();
        let pixels = || {
            Some(Pixels {
                bytes: vec![255; 16],
                width: 2,
                height: 2,
            })
        };
        for i in 0..16 {
            loader.accept(source_key(format!("missing-{i}")), pixels(), &textures);
        }
        assert_eq!(loader.cached.len(), CACHE_ENTRIES);
        assert!(
            loader
                .cached(&source_key("missing-0".into()), &textures)
                .is_none()
        );
        assert!(
            loader
                .cached(&source_key("missing-8".into()), &textures)
                .is_none()
        );
        assert!(
            loader
                .cached(&source_key("missing-15".into()), &textures)
                .unwrap()
                .is_some()
        );

        let path = std::env::temp_dir().join(format!("nearby-artwork-{}", std::process::id()));
        std::fs::write(&path, b"before").unwrap();
        let key = source_key(path.to_string_lossy().into_owned());
        loader.key = key.clone();
        std::fs::write(&path, b"changed bytes").unwrap();
        loader.accept(key.clone(), pixels(), &textures);
        assert!(loader.model.token().is_none());
        assert!(loader.pending);
        assert_ne!(loader.key, key);
        std::fs::remove_file(path).unwrap();
    }
}
