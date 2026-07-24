// No permit exists for Transmit: Cedar default-deny blocks every transmit intent.
// Activation additionally requires hardware receive-only enforcement so policy is not
// misrepresented as physical radio silence.
permit(principal, action == GhostNet::Action::"Receive", resource);
