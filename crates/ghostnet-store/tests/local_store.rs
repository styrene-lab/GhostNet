use ghostnet_store::{LocalStorePort, MemoryLocalStore, ViewCommit};

#[tokio::test]
async fn cursor_and_view_replace_atomically() {
    let store = MemoryLocalStore::default();
    store
        .commit_view(ViewCommit {
            view: "reports".into(),
            cursor: "event:1".into(),
            bytes: b"first".to_vec(),
        })
        .await
        .expect("first commit");
    store
        .commit_view(ViewCommit {
            view: "reports".into(),
            cursor: "event:2".into(),
            bytes: b"second".to_vec(),
        })
        .await
        .expect("second commit");

    let stored = store
        .load_view("reports")
        .await
        .expect("load")
        .expect("stored view");
    assert_eq!(stored.cursor, "event:2");
    assert_eq!(stored.bytes, b"second");
}
