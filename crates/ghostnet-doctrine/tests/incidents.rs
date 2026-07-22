use ghostnet_doctrine::{
    ApplyTransition, IncidentChain, IncidentError, IncidentState, IncidentTransition,
};

fn transition(
    artifact: &str,
    hash: &str,
    previous: Option<&str>,
    state: IncidentState,
) -> IncidentTransition {
    IncidentTransition {
        artifact_id: artifact.into(),
        body_hash: hash.into(),
        incident_id: "incident-1".into(),
        previous_transition_hash: previous.map(str::to_owned),
        state,
        occurred_at_ms: 100,
        expires_at_ms: None,
    }
}

#[test]
fn applies_valid_lifecycle_and_rejects_terminal_escape() {
    let mut chain = IncidentChain::new("incident-1");
    assert_eq!(
        chain.apply(transition("a", "h1", None, IncidentState::Active), 100),
        Ok(ApplyTransition::Applied)
    );
    assert_eq!(
        chain.apply(
            transition("b", "h2", Some("h1"), IncidentState::Monitoring),
            100
        ),
        Ok(ApplyTransition::Applied)
    );
    assert_eq!(
        chain.apply(
            transition("c", "h3", Some("h2"), IncidentState::Closed),
            100
        ),
        Ok(ApplyTransition::Applied)
    );
    assert_eq!(
        chain.apply(
            transition("d", "h4", Some("h3"), IncidentState::Active),
            100
        ),
        Err(IncidentError::Terminal(IncidentState::Closed))
    );
}

#[test]
fn duplicate_is_idempotent_but_artifact_reuse_conflicts() {
    let mut chain = IncidentChain::new("incident-1");
    let original = transition("a", "h1", None, IncidentState::Active);
    chain.apply(original.clone(), 100).expect("initial");
    assert_eq!(chain.apply(original, 100), Ok(ApplyTransition::Duplicate));
    assert_eq!(
        chain.apply(
            transition("a", "other", Some("h1"), IncidentState::Closed),
            100
        ),
        Err(IncidentError::ArtifactConflict)
    );
}

#[test]
fn divergent_predecessor_is_reported_without_changing_head() {
    let mut chain = IncidentChain::new("incident-1");
    chain
        .apply(transition("a", "h1", None, IncidentState::Active), 100)
        .expect("initial");
    assert_eq!(
        chain.apply(
            transition("b", "h2", Some("other"), IncidentState::Monitoring),
            100
        ),
        Ok(ApplyTransition::Fork {
            expected: Some("h1".into()),
            received: Some("other".into())
        })
    );
    assert_eq!(chain.head().expect("head").body_hash, "h1");
}

#[test]
fn expired_transition_is_rejected() {
    let mut chain = IncidentChain::new("incident-1");
    let mut expired = transition("a", "h1", None, IncidentState::Active);
    expired.expires_at_ms = Some(100);
    assert_eq!(chain.apply(expired, 100), Err(IncidentError::Expired));
}
