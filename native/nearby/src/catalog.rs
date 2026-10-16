use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::context::{self, Context};
use crate::{Result, require};

pub(crate) mod matching;
mod metadata;

#[derive(Clone)]
pub struct Entry {
    path: PathBuf,
    name: String,
    directory: bool,
    frontend: String,
    core: String,
    platform: String,
    count: usize,
    image: Option<PathBuf>,
}
impl Entry {
    pub fn prepare(&self) -> Result<Context> {
        let library = context::config(&self.frontend)?
            .join("cores")
            .join(format!("{}_libretro.so", self.core));
        context::parse(
            &self.frontend,
            &[
                "-L".into(),
                library.to_string_lossy().into(),
                self.path.to_string_lossy().into(),
            ],
        )
    }
}

struct Level {
    path: Option<Entry>,
    entries: Vec<Entry>,
    selected: usize,
}

pub struct Browser {
    roots: Vec<PathBuf>,
    levels: Vec<Level>,
    pub busy: bool,
    pub input_epoch: f64,
    canceled: bool,
    pub exit: bool,
    pub selected_context: Option<Context>,
    status: String,
    metadata: BTreeMap<PathBuf, metadata::Metadata>,
    counts: BTreeMap<PathBuf, usize>,
    pub indexing: bool,
}

pub struct Index {
    metadata: BTreeMap<PathBuf, metadata::Metadata>,
    counts: BTreeMap<PathBuf, usize>,
}

pub struct IndexJob {
    roots: Vec<PathBuf>,
    directories: Vec<PathBuf>,
}

impl IndexJob {
    pub fn run(self) -> Index {
        let mut index = Index {
            metadata: BTreeMap::new(),
            counts: BTreeMap::new(),
        };
        for path in self.directories {
            index.metadata.extend(metadata::read(&path, &self.roots));
            let files = metadata::files(&path, &self.roots);
            index.counts.insert(path.clone(), files.len());
            for file in &files {
                for parent in file
                    .ancestors()
                    .skip(1)
                    .take_while(|parent| *parent != path && parent.starts_with(&path))
                {
                    *index.counts.entry(parent.to_path_buf()).or_default() += 1;
                }
            }
            let nested: Vec<_> = index
                .counts
                .keys()
                .filter(|directory| directory.starts_with(&path) && **directory != path)
                .cloned()
                .collect();
            for directory in nested {
                index
                    .metadata
                    .extend(metadata::read(&directory, &self.roots));
            }
        }
        index
    }
}

const PLATFORMS: &[(&str, &str)] = &[
    ("nes", "NES / FC"),
    ("fds", "FDS"),
    ("snes", "SFC"),
    ("megadrive", "MD"),
    ("mastersystem", "SMS"),
    ("segacd", "Sega CD"),
    ("sega32x", "32X"),
    ("pcengine", "PC Engine"),
    ("arcade", "街机"),
    ("neogeo", "Neo Geo"),
    ("cps1", "CPS1"),
    ("cps2", "CPS2"),
    ("cps3", "CPS3"),
    ("psx", "PlayStation"),
    ("gb", "GB"),
    ("gbc", "GBC"),
    ("gba", "GBA"),
];

fn game_file(path: &Path) -> bool {
    path.extension().is_some_and(|extension| {
        matches!(
            extension.to_string_lossy().to_ascii_lowercase().as_str(),
            "zip"
                | "7z"
                | "nes"
                | "fds"
                | "unf"
                | "unif"
                | "sfc"
                | "smc"
                | "gb"
                | "gbc"
                | "gba"
                | "bin"
                | "md"
                | "gen"
                | "sms"
                | "pce"
                | "32x"
                | "chd"
                | "pbp"
                | "iso"
        )
    })
}

impl Browser {
    pub fn new() -> Result<Self> {
        let roots: Vec<_> = ["/roms", "/roms2"]
            .iter()
            .filter_map(|root| Path::new(root).canonicalize().ok())
            .collect();
        let mut entries = Vec::new();
        for root in &roots {
            for (platform, name) in PLATFORMS {
                let directory = root.join(platform);
                let Ok(path) = directory.canonicalize() else {
                    continue;
                };
                if !path.is_dir() || !roots.iter().any(|root| path.starts_with(root)) {
                    continue;
                }
                let choice = if let Some(core) =
                    crate::handheld_link::preferred(&directory.join("game.zip"))
                {
                    Some(("retroarch", core))
                } else {
                    crate::core_choice::candidates(&directory.join("game.zip"))
                        .iter()
                        .find(|choice| {
                            context::config(choice.frontend).is_ok_and(|root| {
                                root.join("cores")
                                    .join(format!("{}_libretro.so", choice.id))
                                    .is_file()
                            })
                        })
                        .map(|choice| (choice.frontend, choice.id))
                };
                if let Some((frontend, core)) = choice {
                    if !context::config(frontend)?
                        .join("cores")
                        .join(format!("{core}_libretro.so"))
                        .is_file()
                    {
                        continue;
                    }
                    entries.push(Entry {
                        path,
                        name: format!(
                            "{}{}",
                            name,
                            if root.ends_with("roms2") {
                                " · 卡 2"
                            } else {
                                ""
                            }
                        ),
                        directory: true,
                        frontend: frontend.into(),
                        core: core.into(),
                        platform: (*platform).into(),
                        count: 0,
                        image: None,
                    });
                }
            }
        }
        Ok(Self {
            roots,
            levels: vec![Level {
                path: None,
                entries,
                selected: 0,
            }],
            busy: false,
            input_epoch: crate::system::monotonic(),
            canceled: false,
            exit: false,
            selected_context: None,
            status: String::new(),
            metadata: BTreeMap::new(),
            counts: BTreeMap::new(),
            indexing: true,
        })
    }

    pub fn index_job(&self) -> IndexJob {
        IndexJob {
            roots: self.roots.clone(),
            directories: self.levels[0]
                .entries
                .iter()
                .map(|entry| entry.path.clone())
                .collect(),
        }
    }

    pub fn indexed(&mut self, index: Index) {
        self.metadata = index.metadata;
        self.counts = index.counts;
        self.indexing = false;
        for level in &mut self.levels {
            for entry in &mut level.entries {
                if let Some(count) = self.counts.get(&entry.path) {
                    entry.count = *count;
                }
                if let Some(metadata) = self.metadata.get(&entry.path) {
                    if !metadata.name.is_empty() {
                        entry.name = metadata.name.clone();
                    }
                    entry.image = metadata.image.clone();
                }
            }
        }
    }

    fn children(&self, directory: &Entry) -> Result<Vec<Entry>> {
        let mut entries: Vec<_> = fs::read_dir(&directory.path)?
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let path = entry.path().canonicalize().ok()?;
                if !self.roots.iter().any(|root| path.starts_with(root))
                    || !(path.is_dir() || game_file(&path))
                    || path.is_dir()
                        && !self.indexing
                        && self.counts.get(&path).copied().unwrap_or(0) == 0
                {
                    return None;
                }
                let metadata = self.metadata.get(&path);
                let name = metadata
                    .filter(|value| !value.name.is_empty())
                    .map(|value| value.name.clone())
                    .unwrap_or_else(|| crate::flow::label(&entry.file_name().to_string_lossy()));
                if name.starts_with('.') {
                    return None;
                }
                let count = self.counts.get(&path).copied().unwrap_or(0);
                Some(Entry {
                    directory: path.is_dir(),
                    path,
                    name,
                    frontend: directory.frontend.clone(),
                    core: directory.core.clone(),
                    platform: directory.platform.clone(),
                    count,
                    image: metadata.and_then(|value| value.image.clone()),
                })
            })
            .collect();
        entries.sort_by_key(|entry| (!entry.directory, entry.name.to_lowercase()));
        Ok(entries)
    }

    pub fn dispatch(&mut self, action: &str) -> Result<Option<Entry>> {
        if self.busy {
            if action == "back" {
                self.canceled = true;
                self.status = "正在取消游戏核对...".into();
            }
            return Ok(None);
        }
        self.status.clear();
        if action == "back" {
            self.input_epoch = crate::system::monotonic();
            if self.levels.len() > 1 {
                self.levels.pop();
            } else {
                self.exit = true;
            }
            return Ok(None);
        }
        let level = self.levels.last_mut().ok_or("Missing game browser level")?;
        let delta = match action {
            "up" => -1,
            "down" => 1,
            "left" | "page_up" => -4,
            "right" | "page_down" => 4,
            _ => 0,
        };
        if delta != 0 {
            level.selected = (level.selected as isize + delta)
                .clamp(0, level.entries.len().saturating_sub(1) as isize)
                as usize;
        }
        if action != "confirm" || level.entries.is_empty() {
            return Ok(None);
        }
        let selected = level.entries[level.selected].clone();
        require(
            self.roots
                .iter()
                .any(|root| selected.path.starts_with(root)),
            "Game browser escaped its local roots",
        )?;
        if selected.directory {
            let entries = match self.children(&selected) {
                Ok(entries) => entries,
                Err(_) => {
                    self.status = "无法读取目录，请返回后重试。".into();
                    return Ok(None);
                }
            };
            self.levels.push(Level {
                path: Some(selected),
                entries,
                selected: 0,
            });
            self.input_epoch = crate::system::monotonic();
            Ok(None)
        } else {
            self.busy = true;
            self.canceled = false;
            self.status = "正在核对游戏...".into();
            Ok(Some(selected))
        }
    }

    pub fn prepared(&mut self, result: std::result::Result<Context, String>) {
        self.input_epoch = crate::system::monotonic();
        self.busy = false;
        if self.canceled {
            self.status.clear();
            self.canceled = false;
            return;
        }
        match result {
            Ok(context) => {
                self.status.clear();
                self.selected_context = Some(context);
            }
            Err(_) => self.status = "无法核对这款游戏，请选择其他游戏。".into(),
        }
    }

    pub fn view(&self) -> Value {
        let level = self.levels.last().expect("Browser always has its root");
        let first = level
            .selected
            .saturating_sub(3)
            .min(level.entries.len().saturating_sub(4));
        let entries: Vec<_> = level.entries.iter().skip(first).take(4).map(|entry| {
            let detail = if entry.directory {
                if self.indexing {
                    format!("{} · 正在统计", entry.core)
                } else {
                    format!("{} · {} 款游戏", entry.core, entry.count)
                }
            } else {
                entry.core.clone()
            };
            json!({
                "key":format!("{:x}", Sha256::digest(entry.path.as_os_str().as_encoded_bytes())),
                "name":entry.name,"detail":detail,"icon":entry.platform,"directory":entry.directory
            })
        }).collect();
        let position = format!(
            "{} / {}",
            if level.entries.is_empty() {
                0
            } else {
                level.selected + 1
            },
            level.entries.len()
        );
        let status = if !self.status.is_empty() {
            self.status.clone()
        } else if self.indexing {
            format!("{position} · 正在整理本机游戏")
        } else {
            position
        };
        json!({
            "artwork":level.entries.get(level.selected).and_then(|entry|entry.image.as_ref()).map(|path|path.to_string_lossy().into_owned()).unwrap_or_default(),
            "page":"games","title":if self.levels.len()==1 { "选择游戏平台" } else { "选择游戏" },
            "core":level.path.as_ref().map(|entry|entry.name.clone()).unwrap_or_else(||"本机游戏".into()),
            "selected":level.selected.saturating_sub(first),"entries":entries,"rooms":[],"room_name":"",
            "ready":false,"busy":self.busy,"refreshing":self.indexing,"status":status,"phase":"idle",
            "back_label":if self.busy {"取消"} else if self.levels.len()==1 {"返回"} else {"上级"}
        })
    }
}
