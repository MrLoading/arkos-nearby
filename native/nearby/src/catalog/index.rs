use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use pinyin::ToPinyin;
use serde::{Deserialize, Serialize};

use super::{Entry, metadata};

const MAX_CACHE_BYTES: u64 = 32 * 1024 * 1024;
const MAX_RECORDS: usize = 100_000;
const MAX_QUERIES: usize = 32;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
struct Stamp {
    path: PathBuf,
    device: u64,
    inode: u64,
    modified: (i64, i64),
    bytes: u64,
}

impl Stamp {
    fn read(path: &Path) -> Option<Self> {
        let metadata = fs::metadata(path).ok()?;
        Some(Self {
            path: path.into(),
            device: metadata.dev(),
            inode: metadata.ino(),
            modified: (metadata.mtime(), metadata.mtime_nsec()),
            bytes: metadata.len(),
        })
    }
    fn current(&self) -> bool {
        Self::read(&self.path).as_ref() == Some(self)
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct Record {
    entry: Entry,
    search: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Index {
    schema: u32,
    roots: Vec<PathBuf>,
    platforms: Vec<Entry>,
    stamps: Vec<Stamp>,
    pub(super) metadata: BTreeMap<PathBuf, metadata::Metadata>,
    pub(super) counts: BTreeMap<PathBuf, usize>,
    records: Vec<Record>,
}

impl Index {
    pub(super) fn cached(path: &Path, roots: &[PathBuf], platforms: &[Entry]) -> Option<Self> {
        if fs::metadata(path).ok()?.len() > MAX_CACHE_BYTES {
            return None;
        }
        let bytes = fs::read(path).ok()?;
        let value: Self = serde_json::from_slice(&bytes).ok()?;
        (value.schema == 1
            && value.roots == roots
            && value.platforms == platforms
            && value.records.len() <= MAX_RECORDS
            && value.stamps.len() <= MAX_RECORDS * 2)
            .then_some(value)
    }

    pub(super) fn entries(&self) -> impl Iterator<Item = &Entry> {
        self.records.iter().map(|record| &record.entry)
    }

    fn search(&self, query: &str) -> Vec<Entry> {
        if query.is_empty() {
            return Vec::new();
        }
        let terms: Vec<_> = query.split_whitespace().collect();
        self.records
            .iter()
            .filter(|record| terms.iter().all(|term| record.search.contains(term)))
            .take(4096)
            .map(|record| record.entry.clone())
            .collect()
    }
}

pub struct IndexJob {
    pub(super) roots: Vec<PathBuf>,
    pub(super) platforms: Vec<Entry>,
    pub(super) cache: PathBuf,
}

fn aliases(entry: &Entry) -> String {
    let mut full = String::new();
    let mut initials = String::new();
    for ch in entry.name.chars() {
        if let Some(pinyin) = ch.to_pinyin() {
            full.push_str(pinyin.plain());
            if let Some(first) = pinyin.plain().chars().next() {
                initials.push(first);
            }
        } else {
            full.push(ch);
            initials.push(ch);
        }
    }
    format!(
        "{} {} {full} {initials}",
        entry.name,
        entry.path.file_name().unwrap_or_default().to_string_lossy()
    )
    .to_lowercase()
}

impl IndexJob {
    pub fn run(self) -> Index {
        if let Some(index) = Index::cached(&self.cache, &self.roots, &self.platforms) {
            if index.stamps.iter().all(Stamp::current) {
                return index;
            }
        }
        let mut index = Index {
            schema: 1,
            roots: self.roots.clone(),
            platforms: self.platforms.clone(),
            stamps: Vec::new(),
            metadata: BTreeMap::new(),
            counts: BTreeMap::new(),
            records: Vec::new(),
        };
        let mut watched = BTreeSet::new();
        for root in &self.roots {
            if let Some(stamp) = Stamp::read(root) {
                index.stamps.push(stamp);
            }
        }
        for platform in self.platforms {
            let mut pending = vec![platform.path.clone()];
            let mut files = BTreeSet::new();
            while let Some(directory) = pending.pop() {
                let Ok(directory) = directory.canonicalize() else {
                    continue;
                };
                if !directory.starts_with(&platform.path)
                    || !watched.insert(directory.clone())
                    || watched.len() + index.records.len() >= MAX_RECORDS
                {
                    continue;
                }
                if let Some(stamp) = Stamp::read(&directory) {
                    index.stamps.push(stamp);
                }
                if let Some(stamp) = Stamp::read(&directory.join("gamelist.xml")) {
                    index.stamps.push(stamp);
                }
                index
                    .metadata
                    .extend(metadata::read(&directory, &self.roots));
                let Ok(children) = fs::read_dir(&directory) else {
                    continue;
                };
                for child in children.flatten() {
                    let Ok(path) = child.path().canonicalize() else {
                        continue;
                    };
                    if !path.starts_with(&platform.path)
                        || child.file_name().to_string_lossy().starts_with('.')
                    {
                        continue;
                    }
                    if path.is_dir() {
                        pending.push(path);
                    } else if super::game_file(&path) {
                        files.insert(path);
                    }
                    if files.len() + index.records.len() >= MAX_RECORDS {
                        break;
                    }
                }
            }
            index.counts.insert(platform.path.clone(), files.len());
            for path in files {
                if index.records.len() >= MAX_RECORDS {
                    break;
                }
                for parent in path.ancestors().skip(1).take_while(|parent| {
                    *parent != platform.path && parent.starts_with(&platform.path)
                }) {
                    *index.counts.entry(parent.into()).or_default() += 1;
                }
                let info = index.metadata.get(&path);
                let mut entry = platform.clone();
                entry.name = info
                    .filter(|info| !info.name.is_empty())
                    .map(|info| info.name.clone())
                    .unwrap_or_else(|| {
                        crate::flow::label(&path.file_name().unwrap_or_default().to_string_lossy())
                    });
                entry.image = info.and_then(|info| info.image.clone());
                entry.path = path;
                entry.directory = false;
                entry.count = 0;
                index.records.push(Record {
                    search: aliases(&entry),
                    entry,
                });
            }
        }
        index.records.sort_by(|a, b| {
            (a.entry.name.to_lowercase(), &a.entry.path)
                .cmp(&(b.entry.name.to_lowercase(), &b.entry.path))
        });
        if serde_json::to_vec(&index).is_ok_and(|data| data.len() as u64 <= MAX_CACHE_BYTES) {
            let _ = crate::system::write(&self.cache, &index);
        }
        index
    }
}

#[derive(Default)]
pub(super) struct Queries {
    values: VecDeque<(String, Vec<Entry>)>,
}
impl Queries {
    pub fn get(&mut self, query: &str) -> Option<Vec<Entry>> {
        let index = self.values.iter().position(|(key, _)| key == query)?;
        let value = self.values.remove(index)?;
        let entries = value.1.clone();
        self.values.push_back(value);
        Some(entries)
    }
    pub fn put(&mut self, query: String, entries: Vec<Entry>) {
        self.values.retain(|(key, _)| key != &query);
        while !self.values.is_empty()
            && (self.values.len() == MAX_QUERIES
                || self
                    .values
                    .iter()
                    .map(|(_, entries)| entries.len())
                    .sum::<usize>()
                    + entries.len()
                    > 4096)
        {
            self.values.pop_front();
        }
        self.values.push_back((query, entries));
    }
}

pub struct SearchJob {
    pub(super) index: Arc<Index>,
    pub(super) query: String,
    pub(super) epoch: u64,
}
pub struct SearchResult {
    pub(super) entries: Vec<Entry>,
    pub(super) query: String,
    pub(super) epoch: u64,
}
impl SearchJob {
    pub fn run(self) -> SearchResult {
        SearchResult {
            entries: self.index.search(&self.query),
            query: self.query,
            epoch: self.epoch,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_tracks_nested_changes_and_searches_names_files_and_pinyin() {
        let root = std::env::temp_dir().join(format!(
            "nearby-index-{}",
            crate::system::random_id().unwrap()
        ));
        let dir = root.join("nes");
        fs::create_dir_all(dir.join("nested")).unwrap();
        fs::write(dir.join("bomber.nes"), b"test").unwrap();
        fs::write(
            dir.join("gamelist.xml"),
            "<gameList><game><path>bomber.nes</path><name>炸弹人</name></game></gameList>",
        )
        .unwrap();
        let root = root.canonicalize().unwrap();
        let dir = root.join("nes");
        let platform = Entry {
            path: dir.clone(),
            name: "NES".into(),
            directory: true,
            frontend: "retroarch".into(),
            core: "fceumm".into(),
            platform: "nes".into(),
            count: 0,
            image: None,
        };
        let job = || IndexJob {
            roots: vec![root.clone()],
            platforms: vec![platform.clone()],
            cache: root.join("index.json"),
        };
        let first = job().run();
        for query in ["炸弹", "bomber", "zhadanren", "zdr"] {
            assert_eq!(first.search(query).len(), 1, "{query}");
        }
        fs::write(dir.join("nested/new.nes"), b"new").unwrap();
        let second = job().run();
        assert_eq!(second.search("new").len(), 1);
        assert_eq!(second.counts.get(&dir), Some(&2));
        fs::remove_file(dir.join("bomber.nes")).unwrap();
        assert!(job().run().search("bomber").is_empty());
        fs::remove_dir_all(root).unwrap();
    }
}
