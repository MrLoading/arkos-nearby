use super::*;

fn fixture() -> (PathBuf, IndexJob) {
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
    fs::create_dir(root.join("cache")).unwrap();
    let platform = Entry {
        extensions: String::new(),
        path: root.join("nes"),
        name: "NES".into(),
        directory: true,
        frontend: "retroarch".into(),
        core: "fceumm".into(),
        platform: "nes".into(),
        count: 0,
        image: None,
    };
    let job = IndexJob {
        roots: vec![root.clone()],
        platforms: vec![platform],
        cache: root.join("cache/index.json"),
        seed: None,
    };
    (root, job)
}

#[test]
fn snapshots_share_identity_and_refresh_nested_inventory_and_names() {
    let (root, job) = fixture();
    let first = job.clone().run();
    for query in ["炸弹", "bomber", "zhadanren", "zdr"] {
        assert_eq!(first.search(query, None).len(), 1, "{query}");
    }
    let mut seeded = job.clone();
    seeded.seed = Some(first.clone());
    let second = seeded.clone().run();
    assert!(Arc::ptr_eq(&first, &second));
    assert!(Arc::ptr_eq(&second, &seeded.cached().unwrap()));
    fs::write(root.join("nes/nested/new.nes"), b"new").unwrap();
    let second = job.clone().run();
    assert_eq!(second.search("new", None).len(), 1);
    assert_eq!(second.counts.get(&root.join("nes")), Some(&2));
    assert_eq!(second.children(&root.join("nes")).unwrap().len(), 2);
    fs::write(
        root.join("nes/nested/gamelist.xml"),
        "<gameList><game><path>new.nes</path><name>Changed</name></game></gameList>",
    )
    .unwrap();
    assert_eq!(job.clone().run().search("changed", None).len(), 1);
    fs::remove_file(root.join("nes/bomber.nes")).unwrap();
    let third = job.clone().run();
    assert!(third.search("bomber", None).is_empty());
    assert_eq!(third.children(&root.join("nes")).unwrap().len(), 1);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn browser_visibility_tracks_nonempty_index_and_preserves_selected_identity() {
    let (root, mut job) = fixture();
    let mut empty = job.platforms[0].clone();
    empty.path = root.join("empty");
    empty.platform = "empty".into();
    fs::create_dir(&empty.path).unwrap();
    job.platforms.insert(0, empty.clone());
    let mut browser = crate::catalog::Browser::with_inventory(job.clone(), true).unwrap();
    assert!(browser.view()["entries"].as_array().unwrap().is_empty());
    browser.indexed(job.clone().run());
    let selected = browser.current().unwrap().key();
    assert_eq!(browser.view()["entries"].as_array().unwrap().len(), 1);
    assert_eq!(browser.view()["library"]["total"], 1);
    assert_eq!(browser.view()["status"], "");
    browser.dispatch("confirm").unwrap();
    assert_eq!(browser.view()["entries"].as_array().unwrap().len(), 1);
    assert!(!browser.view()["entries"][0]["directory"].as_bool().unwrap());
    browser.dispatch("back").unwrap();
    fs::write(empty.path.join("new.nes"), b"new").unwrap();
    browser.indexed(job.clone().run());
    assert_eq!(browser.current().unwrap().key(), selected);
    assert_eq!(browser.view()["library"]["position"], 2);
    assert_eq!(browser.view()["library"]["total"], 2);
    fs::remove_file(root.join("nes/bomber.nes")).unwrap();
    browser.indexed(job.run());
    assert_eq!(browser.current().unwrap().path(), empty.path);
    assert_eq!(browser.view()["library"]["position"], 1);
    assert!(
        browser.view()["entries"]
            .as_array()
            .unwrap()
            .iter()
            .all(|row| row["icon"] != "favorites")
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn refinement_keeps_all_candidates_beyond_the_display_limit() {
    let (root, job) = fixture();
    let mut index = (*job.run()).clone();
    let record = index.records[0].clone();
    index.records = (0..5000)
        .map(|i| {
            let mut row = record.clone();
            row.search = format!("game {i}");
            row.entry.name = row.search.clone();
            row
        })
        .collect();
    let all = Arc::new(index.search("game", None));
    assert_eq!(index.materialize(&all).len(), VISIBLE_RESULTS);
    let mut queries = Queries::default();
    queries.put("game".into(), all.clone());
    let base = queries.candidates("game 4999").unwrap();
    assert_eq!(index.search("game 4999", Some(&base)), vec![4999]);
    assert!(Arc::ptr_eq(&queries.get("game").unwrap(), &all));
    assert_eq!(
        index.search("game 49", Some(&base)),
        index.search("game 49", None)
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn old_disk_snapshot_migrates_without_changing_results() {
    let (root, job) = fixture();
    let current = job.clone().run();
    let mut legacy = serde_json::to_value(&*current).unwrap();
    legacy["schema"] = 1.into();
    legacy["metadata"] = serde_json::to_value(&current.metadata).unwrap();
    legacy.as_object_mut().unwrap().remove("directories");
    crate::system::write(&job.cache, &legacy).unwrap();
    assert_eq!(job.cached().unwrap().search("zdr", None).len(), 1);
    let updated = job.run();
    assert_eq!(updated.schema, 3);
    assert_eq!(updated.search("zdr", None).len(), 1);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn inventory_preserves_matching_scope_and_confines_cached_children() {
    use std::os::unix::fs::symlink;
    let (root, mut job) = fixture();
    fs::write(root.join("nes/.hidden.nes"), b"hidden").unwrap();
    symlink("/etc", root.join("nes/outside")).unwrap();
    let index = job.clone().run();
    assert_eq!(index.entries().count(), 2);
    assert_eq!(index.children(&root.join("nes")).unwrap().len(), 1);
    assert!(index.search("hidden", None).is_empty());
    job.platforms[0].core.clear();
    let index = job.run();
    assert_eq!(
        index.entries().count(),
        2,
        "Room matching retains games without a default core"
    );
    assert!(index.search("bomber", None).is_empty());
    fs::remove_dir_all(root).unwrap();
}
