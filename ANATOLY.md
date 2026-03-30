# Project Instructions

## General
RustGuard is a WireGuard protocol implementation in Rust. Security-critical codebase.
Prioritize correctness findings in crypto, handshake, replay protection, and AEAD above all else.
The project uses a workspace layout: rustguard-core, rustguard-crypto, rustguard-tun,
rustguard-daemon, rustguard-enroll, rustguard-cli, and a kernel module (kmod/).

## Tests
- OS-specific I/O code (TUN devices, sockets, ioctls, io_uring, BPF/XDP) is inherently
  hard to unit test. Do not flag missing unit tests for these modules — integration tests
  with loopback tunnels are the right approach, not mocks.
- Crypto and protocol core (handshake, replay, AEAD, cookie, tai64n) MUST have thorough
  tests — be strict on coverage and edge cases for these modules.
- Do not treat "public function without a dedicated test" as a finding on its own.
  Evaluate whether the function's behavior is covered by higher-level tests or
  integration paths before flagging.
- Example files (tun_echo.rs, examples/) are not production code — skip test evaluation.

## Correction
- kmod/ is a kernel module with its own build system and constraints — skip entirely.
- Example files (tun_echo.rs, examples/) are not production — skip.
- no_std code paths use intentional workarounds (atomic counters, position-based mixing)
  that may look unusual but are deliberate choices for environments without alloc or
  system randomness. Do not flag these as bugs unless they break cryptographic guarantees.
- Pay special attention to buffer sizing relative to MTU/frame boundaries — undersized
  buffers passed to kernel ioctls or UMEM frames are critical bugs.

## Documentation
- Private helper functions in the crypto crate with inline comments explaining the "why"
  (not the "what") are considered adequately documented.
- Protocol-level documentation lives in docs/, not in code comments.

## Overengineering
- Explicit unsafe blocks with safety comments are expected and necessary for FFI,
  io_uring, XDP, and TUN operations. Do not flag these as over-engineered.
