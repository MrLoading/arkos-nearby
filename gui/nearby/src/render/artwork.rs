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
type ArtworkKey = (String, Option<(u64, Option<std::time::SystemTime>)>);

#[model]
#[derive(Default)]
struct Artwork {
    #[observe]
    revision: u64,
    #[observe]
    ready: bool,
}

#[model]
impl Artwork {
    fn show(&mut self, revision: u64, ready: bool) {
        self.revision = revision;
        self.ready = ready;
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
    send: SyncSender<(u64, String)>,
    receive: Receiver<(u64, Option<Pixels>)>,
    path: String,
    revision: u64,
    changed: Instant,
    pending: bool,
    texture: Option<String>,
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
    let (send, jobs) = mpsc::sync_channel::<(u64, String)>(1);
    let (results, receive) = mpsc::sync_channel(1);
    std::thread::spawn(move || {
        let mut cache: VecDeque<(ArtworkKey, Option<Pixels>)> = VecDeque::new();
        while let Ok((revision, path)) = jobs.recv() {
            let stamp = std::fs::metadata(&path)
                .ok()
                .map(|meta| (meta.len(), meta.modified().ok()));
            let key = (path.clone(), stamp);
            let pixels = if let Some(index) = cache.iter().position(|(old, _)| old == &key) {
                let item = cache.remove(index).expect("Located cached artwork");
                let pixels = item.1.clone();
                cache.push_back(item);
                pixels
            } else {
                let pixels = decode(&path);
                if cache.len() == 8 {
                    cache.pop_front();
                }
                cache.push_back((key, pixels.clone()));
                pixels
            };
            if results.send((revision, pixels)).is_err() {
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
        revision: 0,
        changed: Instant::now(),
        pending: false,
        texture: None,
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
    if path != loader.path {
        loader.path = path;
        loader.revision += 1;
        loader.changed = Instant::now();
        loader.pending = !loader.path.is_empty();
        loader.model.show(loader.revision, false);
    }
    if loader.pending
        && loader.changed.elapsed() >= DEBOUNCE
        && loader
            .send
            .try_send((loader.revision, loader.path.clone()))
            .is_ok()
    {
        loader.pending = false;
    }
    while let Ok((revision, pixels)) = loader.receive.try_recv() {
        if revision != loader.revision {
            continue;
        }
        if let Some(old) = loader.texture.take() {
            textures.remove_token(&old);
        }
        if let Some(pixels) = pixels {
            if let Some(texture) = Texture::from_vec(
                pixels.bytes,
                pixels.width,
                pixels.height,
                ColorFormat::RGBA8888,
            ) {
                let key = format!("nearby-cover-{revision}");
                textures.add_factory(key.clone(), move || Some(texture));
                loader.texture = Some(key);
                loader.model.show(revision, true);
            }
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
                walk ${ art.ready().then(|| art.revision()).into_iter() } with revision by revision {
                    Image (
                        src: format!("nearby-cover-{revision}"),
                        shrink: 0.0,
                        width: @(width, height) { width.min(height * Fixed::from_int(4) / Fixed::from_int(3)) },
                        height: @(width, height) { height.min(width * Fixed::from_int(3) / Fixed::from_int(4)) }
                    )
                }
                if ${ !placeholder.ready() } {
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
            let old_revision = app.world.resource::<Loader>().unwrap().revision;
            view.artwork = "second".into();
            super::super::update(&mut scene, &view).unwrap();
            super::super::paint_at(&mut app, 20).unwrap();
            send.send((
                old_revision,
                Some(Pixels {
                    bytes: vec![255; 16],
                    width: 2,
                    height: 2,
                }),
            ))
            .unwrap();
            super::super::paint_at(&mut app, 30).unwrap();
            let loader = app.world.resource::<Loader>().unwrap();
            assert!(!loader.model.ready());
            assert!(loader.texture.is_none());
            let revision = loader.revision;
            send.send((
                revision,
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
            assert!(app.world.resource::<Loader>().unwrap().model.ready());
            let key = format!("nearby-cover-{revision}");
            let entity = app.world.query::<Image>().iter().find_map(|(entity, image)| {
                matches!(&image.src, mirui::ui::widgets::image::ImageSource::Texture(token) if token == &key).then_some(entity)
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
        }
    }
}
