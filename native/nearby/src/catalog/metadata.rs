use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const MAX_XML_BYTES: u64 = 32 * 1024 * 1024;
const MAX_RECORDS: usize = 100_000;

#[derive(Clone, Default)]
pub(super) struct Metadata {
    pub name: String,
    pub image: Option<PathBuf>,
}

pub(super) fn local_path(base: &Path, value: &str, roots: &[PathBuf]) -> Option<PathBuf> {
    let path = base.join(value).canonicalize().ok()?;
    (path.is_file() && roots.iter().any(|root| path.starts_with(root))).then_some(path)
}

pub(super) fn read(directory: &Path, roots: &[PathBuf]) -> BTreeMap<PathBuf, Metadata> {
    let path = directory.join("gamelist.xml");
    let result = (|| {
        if fs::metadata(&path).ok()?.len() > MAX_XML_BYTES {
            return None;
        }
        let xml = fs::read_to_string(path).ok()?;
        let document = roxmltree::Document::parse(&xml).ok()?;
        let mut entries = BTreeMap::new();
        for game in document
            .descendants()
            .filter(|node| node.has_tag_name("game"))
            .take(MAX_RECORDS)
        {
            let field = |name| {
                game.children()
                    .find(|node| node.has_tag_name(name))
                    .and_then(|node| node.text())
            };
            let Some(path) = field("path").and_then(|value| local_path(directory, value, roots))
            else {
                continue;
            };
            let name = field("name")
                .map(str::trim)
                .filter(|name| !name.is_empty())
                .map(crate::flow::label)
                .unwrap_or_default();
            let image = ["image", "thumbnail"]
                .into_iter()
                .find_map(|name| field(name).and_then(|value| local_path(directory, value, roots)));
            entries.insert(path, Metadata { name, image });
        }
        Some(entries)
    })();
    result.unwrap_or_default()
}

/// Enumerate unique files under one platform, retaining confinement across symlinks.
pub(super) fn files(directory: &Path, roots: &[PathBuf]) -> Vec<PathBuf> {
    let mut pending = vec![directory.to_path_buf()];
    let mut visited = BTreeSet::new();
    let mut files = BTreeSet::new();
    while let Some(directory) = pending.pop() {
        let Ok(directory) = directory.canonicalize() else {
            continue;
        };
        if !roots.iter().any(|root| directory.starts_with(root))
            || !visited.insert(directory.clone())
        {
            continue;
        }
        let Ok(children) = fs::read_dir(directory) else {
            continue;
        };
        for child in children.flatten() {
            let Ok(path) = child.path().canonicalize() else {
                continue;
            };
            if !roots.iter().any(|root| path.starts_with(root)) {
                continue;
            }
            if path.is_dir() {
                pending.push(path);
            } else if super::game_file(&path) {
                files.insert(path);
            }
            if visited.len() + files.len() >= MAX_RECORDS {
                return files.into_iter().collect();
            }
        }
    }
    files.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata_and_enumeration_share_local_confinement() {
        let root = std::env::temp_dir().join(format!("nearby-catalog-{}", std::process::id()));
        fs::create_dir_all(root.join("nested")).unwrap();
        fs::write(root.join("nested/game.nes"), b"test").unwrap();
        fs::write(root.join("picture.png"), b"test").unwrap();
        fs::write(root.join("gamelist.xml"), "<gameList><game><path>./nested/game.nes</path><name>Game &amp; Friend</name><image>./picture.png</image></game><game><path>/etc/passwd</path><name>Outside</name></game></gameList>").unwrap();
        let root = root.canonicalize().unwrap();
        let roots = [root.clone()];
        let entries = read(&root, &roots);
        assert_eq!(entries.len(), 1);
        let metadata = entries.get(&root.join("nested/game.nes")).unwrap();
        assert_eq!(metadata.name, "Game & Friend");
        assert_eq!(metadata.image, Some(root.join("picture.png")));
        assert_eq!(files(&root, &roots), vec![root.join("nested/game.nes")]);
        fs::remove_dir_all(root).unwrap();
    }
}
