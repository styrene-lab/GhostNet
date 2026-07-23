// Layer-1 OM policy may restrict but never relax Styrene's compiled invariants.
permit(principal, action == GhostNet::Action::"Transmit", resource)
when { context.posture == "routine" || context.confirmed };
