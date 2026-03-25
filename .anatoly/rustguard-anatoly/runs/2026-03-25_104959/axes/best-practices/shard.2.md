# Best Practices — Shard 2

## Findings

| File | Verdict | BP Score | Details |
|------|---------|----------|---------|
| `rustguard-kmod/src/cookie.rs` | NEEDS_REFACTOR | 7/10 | [details](../reviews/rustguard-kmod-src-cookie.rev.md) |
| `rustguard-enroll/src/client.rs` | NEEDS_REFACTOR | 5.75/10 | [details](../reviews/rustguard-enroll-src-client.rev.md) |
| `rustguard-kmod/src/timers.rs` | NEEDS_REFACTOR | 6/10 | [details](../reviews/rustguard-kmod-src-timers.rev.md) |
| `rustguard-tun/src/uring.rs` | NEEDS_REFACTOR | 5/10 | [details](../reviews/rustguard-tun-src-uring.rev.md) |
| `rustguard-kmod/src/allowedips.rs` | NEEDS_REFACTOR | 5/10 | [details](../reviews/rustguard-kmod-src-allowedips.rev.md) |

## Details

### `rustguard-kmod/src/cookie.rs` — 7/10

**Failed rules:**

- Rule 3: Proper error handling with Result/Option (HIGH)

- Check the return value of wg_xchacha20poly1305_encrypt in create_reply and propagate failure instead of silently returning a zeroed reply.
  - Before: `unsafe {
    wg_xchacha20poly1305_encrypt(
        key.as_ptr(), nonce.as_ptr(),
        cookie.as_ptr(), COOKIE_LEN as u32,
        mac1.as_ptr(), 16,
        encrypted_cookie.as_mut_ptr(),
    );
}`
  - After: `// SAFETY: all pointers derive from valid Rust references with lifetimes
// that encompass this call; buffer sizes match the API contract.
let ret = unsafe {
    wg_xchacha20poly1305_encrypt(
        key.as_ptr(), nonce.as_ptr(),
        cookie.as_ptr(), COOKIE_LEN as u32,
        mac1.as_ptr(), 16,
        encrypted_cookie.as_mut_ptr(),
    )
};
if ret != 0 { return None; }  // change return type to Option<[u8; 64]>`
- Add SAFETY comments to each unsafe block documenting pointer validity invariants.
  - Before: `unsafe { wg_blake2s_hash(ptrs.as_ptr(), lens.as_ptr(), chunks.len() as u32, out.as_mut_ptr()) };`
  - After: `// SAFETY: ptrs[i] point into `chunks` slice data which is alive for this call;
// lens[i] match the corresponding slice lengths; out is a valid 32-byte stack buffer.
unsafe { wg_blake2s_hash(ptrs.as_ptr(), lens.as_ptr(), chunks.len() as u32, out.as_mut_ptr()) };`
- Derive Debug on public-crate types to aid in diagnostics and testing.
  - Before: `pub(crate) struct CookieChecker {
    our_public: [u8; 32],
    secret: [u8; 32],
    secret_generated: u64,
    pub(crate) under_load: bool,
}`
  - After: `#[derive(Debug)]
pub(crate) struct CookieChecker {
    our_public: [u8; 32],
    secret: [u8; 32],
    secret_generated: u64,
    pub(crate) under_load: bool,
}`
- Document the new() constructors to explain the expected inputs and initialization semantics.
  - Before: `pub(crate) fn new(our_public: [u8; 32]) -> Self {`
  - After: `/// Creates a new `CookieChecker` bound to the given 32-byte Curve25519 public key.
/// Immediately generates a fresh random cookie secret valid for `COOKIE_SECRET_LIFETIME_NS`.
pub(crate) fn new(our_public: [u8; 32]) -> Self {`

### `rustguard-enroll/src/client.rs` — 5.75/10

**Failed rules:**

- Rule 1: No unwrap in production code (CRITICAL)
- Rule 4: Derive common traits on public types (MEDIUM)

- Replace .unwrap() on Mutex::lock() with explicit poison handling or expect with context
  - Before: `let mut sess = session_out.lock().unwrap();`
  - After: `let mut sess = session_out.lock().unwrap_or_else(|e| e.into_inner());`
- Replace .unwrap() on try_into() for fixed-size slice conversions with expect carrying context
  - Before: `let resp = Response::from_bytes(resp_buf[..RESPONSE_SIZE].try_into().unwrap());`
  - After: `let resp = Response::from_bytes(
    resp_buf[..RESPONSE_SIZE]
        .try_into()
        .expect("RESPONSE_SIZE is a compile-time constant; slice is guaranteed correct size"),
);`
- Replace .unwrap() on thread join with error propagation
  - Before: `outbound.join().unwrap();
inbound.join().unwrap();`
  - After: `outbound.join().map_err(|_| io::Error::new(io::ErrorKind::Other, "outbound thread panicked"))?;
inbound.join().map_err(|_| io::Error::new(io::ErrorKind::Other, "inbound thread panicked"))?;`
- Return Result from rand_index instead of panicking on RNG failure
  - Before: `fn rand_index() -> u32 {
    let mut buf = [0u8; 4];
    getrandom::getrandom(&mut buf).expect("rng");
    u32::from_le_bytes(buf)
}`
  - After: `fn rand_index() -> io::Result<u32> {
    let mut buf = [0u8; 4];
    getrandom::getrandom(&mut buf)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(u32::from_le_bytes(buf))
}`
- Add derive macros and doc comments to the public JoinConfig struct
  - Before: `pub struct JoinConfig {
    pub server_endpoint: SocketAddr,
    pub token: String,
}`
  - After: `/// Configuration required to enroll with a RustGuard server.
#[derive(Debug, Clone)]
pub struct JoinConfig {
    /// UDP endpoint of the enrollment server.
    pub server_endpoint: SocketAddr,
    /// Shared enrollment token used to authenticate this peer.
    pub token: String,
}`
- Log or count silently discarded send/write errors instead of using let _ = ...
  - Before: `let _ = udp_out.send_to(&transport.to_bytes(), endpoint);`
  - After: `if let Err(e) = udp_out.send_to(&transport.to_bytes(), endpoint) {
    eprintln!("[warn] send_to failed: {e}");
}`

### `rustguard-kmod/src/timers.rs` — 6/10

**Failed rules:**

- Rule 2: No unsafe blocks without clear justification comment (CRITICAL)

- Add a `// SAFETY:` comment to the `unsafe` block in `now_ns()` explaining why the FFI call is sound.
  - Before: `fn now_ns() -> u64 {
    unsafe { wg_ktime_get_ns() }
}`
  - After: `fn now_ns() -> u64 {
    // SAFETY: `wg_ktime_get_ns` wraps the kernel's `ktime_get_ns()`, which is
    // always callable from any execution context, is reentrant, has no
    // preconditions, and never returns an error.
    unsafe { wg_ktime_get_ns() }
}`
- Derive common traits on `SessionTimers` to improve testability and diagnostics.
  - Before: `pub(crate) struct SessionTimers {`
  - After: `#[derive(Debug, Clone, PartialEq)]
pub(crate) struct SessionTimers {`
- Implement `Default` for `SessionTimers` (or call `Default::default()` from `new()`) to satisfy the `clippy::new_without_default` lint.
  - Before: `impl SessionTimers {
    pub(crate) fn new() -> Self {
        Self {
            session_established: 0,
            last_handshake_sent: 0,
            last_received: 0,
            last_sent: 0,
            rekey_requested: false,
            keepalive_interval_ns: 0,
        }
    }`
  - After: `impl Default for SessionTimers {
    fn default() -> Self {
        Self {
            session_established: 0,
            last_handshake_sent: 0,
            last_received: 0,
            last_sent: 0,
            rekey_requested: false,
            keepalive_interval_ns: 0,
        }
    }
}

impl SessionTimers {
    pub(crate) fn new() -> Self {
        Self::default()
    }`

### `rustguard-tun/src/uring.rs` — 5/10

**Failed rules:**

- Rule 2: No unsafe blocks without clear justification comment (CRITICAL)
- Rule 4: Derive common traits on public types (MEDIUM)

- Add SAFETY comments to every unsafe block
  - Before: `unsafe {
    ring.submitter().register_buffers(&iovecs)?;
}`
  - After: `// SAFETY: `iovecs` point into `bufs.data`, which is heap-allocated and stable
// for the lifetime of `UringTun`. `IoUring` is dropped (deregistering buffers)
// before `bufs` is dropped due to struct field drop order.
unsafe {
    ring.submitter().register_buffers(&iovecs)?;
}`
- Fix slot_ptr to use &mut self and as_mut_ptr to avoid *const→*mut aliasing UB
  - Before: `fn slot_ptr(&self, idx: usize) -> *mut u8 {
    unsafe { self.data.as_ptr().add(idx * BUF_SIZE) as *mut u8 }
}`
  - After: `fn slot_ptr(&mut self, idx: usize) -> *mut u8 {
    // SAFETY: idx is always < NUM_BUFS; callers hold an in-flight slot.
    unsafe { self.data.as_mut_ptr().add(idx * BUF_SIZE) }
}`
- Derive Debug, Clone, PartialEq on the Completion value type
  - Before: `pub struct Completion {
    pub buf_idx: usize,
    pub is_read: bool,
    pub result: i32,
}`
  - After: `#[derive(Debug, Clone, PartialEq)]
pub struct Completion {
    pub buf_idx: usize,
    pub is_read: bool,
    pub result: i32,
}`
- Extract duplicated completion-draining logic into a private helper to eliminate copy-paste
  - Before: `// Identical loop exists in both submit_and_wait and poll:
let cq = self.ring.completion();
for cqe in cq {
    let user_data = cqe.user_data();
    let is_read = user_data & READ_FLAG != 0;
    let buf_idx = (user_data & 0xFFFF_FFFF) as usize;
    let result = cqe.result();
    if is_read { self.pending_reads -= 1; }
    completions.push(Completion { buf_idx, is_read, result });
}`
  - After: `fn drain_cq(&mut self) -> Vec<Completion> {
    self.ring.completion().map(|cqe| {
        let user_data = cqe.user_data();
        let is_read = user_data & READ_FLAG != 0;
        let buf_idx = (user_data & 0xFFFF_FFFF) as usize;
        if is_read { self.pending_reads -= 1; }
        Completion { buf_idx, is_read, result: cqe.result() }
    }).collect()
}`
- Explicitly opt out of Sync to prevent accidental multi-threaded misuse
  - Before: `// (no explicit Send/Sync bounds on UringTun)`
  - After: `// UringTun is intentionally single-threaded: pending_reads is unsynchronised.
impl !Sync for UringTun {}`

### `rustguard-kmod/src/allowedips.rs` — 5/10

**Failed rules:**

- Rule 1: No unwrap() in production code (CRITICAL)

- Replace unwrap() with explicit Option handling after confirmed insertion
  - Before: `if let Ok(n) = KBox::new(TrieNode::new(), GFP_KERNEL) {
    *root = Some(n);
} else {
    return;
}
root.as_mut().unwrap()`
  - After: `let n = KBox::new(TrieNode::new(), GFP_KERNEL).map_err(|_| ())?;
*root = Some(n);
// SAFETY: we just assigned `Some` above
root.as_mut().expect("just inserted")`
- Propagate allocation failures so callers can detect silent insert misses
  - Before: `pub(crate) fn insert_v4(&mut self, ip: [u8; 4], cidr: u8, peer_idx: usize) {
    Self::insert(&mut self.root4, &ip, cidr, peer_idx);
}`
  - After: `pub(crate) fn insert_v4(&mut self, ip: [u8; 4], cidr: u8, peer_idx: usize) -> Result<()> {
    Self::insert(&mut self.root4, &ip, cidr, peer_idx)
}`
- Remove dead assignments overwritten on the very next lines in insert()
  - Before: `node.cidr = 0;
node.peer_idx = peer_idx;
// Store with a sentinel: cidr=255 means "this is a default route"
node.cidr = 255; // special: default
node.peer_idx = peer_idx;`
  - After: `// Store with a sentinel: cidr=255 means "this is a default route"
node.cidr = 255;
node.peer_idx = peer_idx;`
- Simplify always-true compound condition in lookup_recursive
  - Before: `if child.cidr == 255 || (child.cidr > 0 && child.cidr != 255) {`
  - After: `if child.cidr > 0 {`
- Rewrite insert_recursive iteratively to avoid deep kernel stack recursion for IPv6 (up to 128 frames)
  - Before: `fn insert_recursive(node: &mut TrieNode, ip: &[u8], cidr: u8, peer_idx: usize, depth: u32) {
    if depth >= cidr as u32 { ... return; }
    ...
    Self::insert_recursive(child.as_mut().unwrap(), ip, cidr, peer_idx, depth + 1);
}`
  - After: `fn insert_iterative(mut node: &mut TrieNode, ip: &[u8], cidr: u8, peer_idx: usize) {
    for depth in 0..cidr as u32 {
        let bit = Self::get_bit(ip, depth) as usize;
        if node.children[bit].is_none() { /* allocate */ }
        node = node.children[bit].as_deref_mut().expect("just allocated");
    }
    node.cidr = cidr;
    node.peer_idx = peer_idx;
}`
