//! Reusable behavioral conformance checks for [`crate::StyrenePort`].

use crate::{ArtifactKey, IdentityRef, PollRequest, PublishRequest, StyrenePort, TopicPath};

/// Runs the transport-independent baseline required of every Styrene port.
///
/// # Errors
///
/// Returns the first contract error produced by the implementation.
///
/// # Panics
///
/// Panics when the implementation violates a baseline contract invariant.
pub async fn run_baseline_conformance(port: &dyn StyrenePort) -> Result<(), crate::StyreneError> {
    let capabilities = port.capabilities().await?;
    assert!(capabilities.contract_version > 0);
    let _identity: IdentityRef = port.local_identity().await?;

    let topic = TopicPath("ghostnet/net/conformance/reports".into());
    let artifact = ArtifactKey {
        artifact_id: "conformance-artifact".into(),
        body_hash: format!("sha256:{}", "0".repeat(64)),
    };
    let request = PublishRequest {
        topic: topic.clone(),
        artifact: artifact.clone(),
        body: b"{}".to_vec(),
        correlation_id: "conformance-publish".into(),
    };
    let first = port.publish(request.clone()).await?;
    let retry = port.publish(request).await?;
    assert_eq!(first.event_id, retry.event_id);
    assert!(retry.duplicate);

    let page = port
        .poll(PollRequest {
            topic,
            after: None,
            limit: 100,
        })
        .await?;
    assert_eq!(page.events.len(), 1);
    assert_eq!(page.events[0].artifact, artifact);
    Ok(())
}
