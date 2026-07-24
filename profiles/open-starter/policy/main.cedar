// Starter mode adds no OM-specific transmission restriction.
// Styrene's identity, RBAC, destination, disclosure, and exact-byte safety rules still apply.
permit(principal, action == GhostNet::Action::"Transmit", resource);
