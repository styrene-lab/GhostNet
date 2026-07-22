use ghostnet_doctrine::{ApplyRevision, NetError, NetRevision, NetRevisionChain};

fn revision(number: u64, hash: &str, previous: Option<&str>) -> NetRevision {
    NetRevision {
        artifact_id: format!("artifact-{number}"),
        body_hash: hash.into(),
        net_id: "net-1".into(),
        revision: number,
        previous_revision_hash: previous.map(str::to_owned),
        effective_at_ms: 100,
        expires_at_ms: None,
    }
}

#[test]
fn applies_linear_revisions_and_duplicates_idempotently() {
    let mut chain = NetRevisionChain::new("net-1");
    let first = revision(1, "h1", None);
    assert_eq!(chain.apply(first.clone(), 100), Ok(ApplyRevision::Applied));
    assert_eq!(chain.apply(first, 100), Ok(ApplyRevision::Duplicate));
    assert_eq!(
        chain.apply(revision(2, "h2", Some("h1")), 100),
        Ok(ApplyRevision::Applied)
    );
    assert_eq!(chain.head().expect("head").revision, 2);
}

#[test]
fn fork_is_explicit_and_does_not_replace_head() {
    let mut chain = NetRevisionChain::new("net-1");
    chain.apply(revision(1, "h1", None), 100).expect("first");
    assert_eq!(
        chain.apply(revision(2, "fork", Some("other")), 100),
        Ok(ApplyRevision::Fork {
            expected_revision: 2,
            expected_predecessor: Some("h1".into()),
            received_revision: 2,
            received_predecessor: Some("other".into()),
        })
    );
    assert_eq!(chain.head().expect("head").body_hash, "h1");
}

#[test]
fn validity_and_initial_predecessor_are_enforced() {
    let mut chain = NetRevisionChain::new("net-1");
    let mut future = revision(1, "h1", None);
    future.effective_at_ms = 101;
    assert_eq!(chain.apply(future, 100), Err(NetError::NotEffective));

    let mut expired = revision(1, "h1", None);
    expired.expires_at_ms = Some(100);
    assert_eq!(chain.apply(expired, 100), Err(NetError::Expired));

    assert_eq!(
        chain.apply(revision(1, "h1", Some("impossible")), 100),
        Err(NetError::InitialPredecessor)
    );
}
