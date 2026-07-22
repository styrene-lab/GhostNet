use ghostnet_styrene::{
    ArtifactKey, FakeStyrenePort, Fault, IdentityRef, PollRequest, PublishRequest, StyreneError,
    StyrenePort, TopicPath,
};

fn artifact(index: u8) -> ArtifactKey {
    ArtifactKey {
        artifact_id: format!("artifact-{index}"),
        body_hash: format!("sha256:{index:064x}"),
    }
}

fn publication(index: u8) -> PublishRequest {
    PublishRequest {
        topic: TopicPath("ghostnet/net/example/reports".into()),
        artifact: artifact(index),
        body: vec![index],
        correlation_id: format!("correlation-{index}"),
    }
}

fn poll() -> PollRequest {
    PollRequest {
        topic: TopicPath("ghostnet/net/example/reports".into()),
        after: None,
        limit: 100,
    }
}

#[tokio::test]
async fn duplicate_publication_is_idempotent() {
    let fake = FakeStyrenePort::new(IdentityRef("local".into()));
    let first = fake.publish(publication(1)).await.expect("first publish");
    let second = fake.publish(publication(1)).await.expect("retry publish");

    assert!(!first.duplicate);
    assert!(second.duplicate);
    assert_eq!(first.event_id, second.event_id);
    assert_eq!(fake.poll(poll()).await.expect("poll").events.len(), 1);
}

#[tokio::test]
async fn timeout_before_acceptance_does_not_retain_event() {
    let fake = FakeStyrenePort::new(IdentityRef("local".into()));
    fake.inject(Fault::TimeoutBeforeAcceptance);

    assert_eq!(
        fake.publish(publication(1)).await,
        Err(StyreneError::Timeout)
    );
    assert!(fake.poll(poll()).await.expect("poll").events.is_empty());
}

#[tokio::test]
async fn timeout_after_acceptance_is_safe_to_retry() {
    let fake = FakeStyrenePort::new(IdentityRef("local".into()));
    fake.inject(Fault::TimeoutAfterAcceptance);

    assert_eq!(
        fake.publish(publication(1)).await,
        Err(StyreneError::Timeout)
    );
    let retry = fake
        .publish(publication(1))
        .await
        .expect("idempotent retry");
    assert!(retry.duplicate);
    assert_eq!(fake.poll(poll()).await.expect("poll").events.len(), 1);
}

#[tokio::test]
async fn denial_is_explicit_and_has_no_side_effect() {
    let fake = FakeStyrenePort::new(IdentityRef("local".into()));
    fake.inject(Fault::Deny("publish forbidden".into()));

    assert_eq!(
        fake.publish(publication(1)).await,
        Err(StyreneError::Denied("publish forbidden".into()))
    );
    assert!(fake.poll(poll()).await.expect("poll").events.is_empty());
}

#[tokio::test]
async fn poll_can_inject_duplicate_and_reordering() {
    let fake = FakeStyrenePort::new(IdentityRef("local".into()));
    fake.publish(publication(1)).await.expect("publish 1");
    fake.publish(publication(2)).await.expect("publish 2");

    fake.inject(Fault::DuplicateNextPoll);
    let duplicated = fake.poll(poll()).await.expect("duplicate poll");
    assert_eq!(duplicated.events.len(), 3);
    assert_eq!(duplicated.events[0], duplicated.events[1]);

    fake.inject(Fault::ReverseNextPoll);
    let reversed = fake.poll(poll()).await.expect("reverse poll");
    assert_eq!(reversed.events[0].artifact, artifact(2));
    assert_eq!(reversed.events[1].artifact, artifact(1));
}

#[tokio::test]
async fn retention_gap_is_visible_and_omits_retained_prefix() {
    let fake = FakeStyrenePort::new(IdentityRef("local".into()));
    fake.publish(publication(1)).await.expect("publish 1");
    fake.publish(publication(2)).await.expect("publish 2");
    fake.inject(Fault::RetentionGapNextPoll);

    let page = fake.poll(poll()).await.expect("gap poll");
    assert!(page.retention_gap);
    assert_eq!(page.events.len(), 1);
    assert_eq!(page.events[0].artifact, artifact(2));
}
