# Best Practices — Shard 1

## Findings

| File | Verdict | BP Score | Details |
|------|---------|----------|---------|
| `rustguard-tun/src/linux_mq.rs` | CRITICAL | 5.25/10 | [details](../reviews/rustguard-tun-src-linux_mq.rev.md) |
| `rustguard-tun/src/xdp.rs` | CRITICAL | 6/10 | [details](../reviews/rustguard-tun-src-xdp.rev.md) |
| `rustguard-enroll/src/server.rs` | CRITICAL | 4.5/10 | [details](../reviews/rustguard-enroll-src-server.rev.md) |
| `rustguard-core/src/replay.rs` | CRITICAL | 8.5/10 | [details](../reviews/rustguard-core-src-replay.rev.md) |
| `rustguard-daemon/src/tunnel.rs` | NEEDS_REFACTOR | 1.5/10 | [details](../reviews/rustguard-daemon-src-tunnel.rev.md) |
| `rustguard-core/src/messages.rs` | NEEDS_REFACTOR | 6/10 | [details](../reviews/rustguard-core-src-messages.rev.md) |
| `rustguard-enroll/src/control.rs` | NEEDS_REFACTOR | 6/10 | [details](../reviews/rustguard-enroll-src-control.rev.md) |
| `rustguard-tun/src/linux.rs` | NEEDS_REFACTOR | 6.75/10 | [details](../reviews/rustguard-tun-src-linux.rev.md) |
| `rustguard-tun/src/bpf_loader.rs` | NEEDS_REFACTOR | 2/10 | [details](../reviews/rustguard-tun-src-bpf_loader.rev.md) |
| `rustguard-enroll/src/fast_udp.rs` | NEEDS_REFACTOR | 5.5/10 | [details](../reviews/rustguard-enroll-src-fast_udp.rev.md) |

## Details

### `rustguard-tun/src/linux_mq.rs` — 5.25/10

**Failed rules:**

- Rule 2: No unsafe blocks without clear justification comment (CRITICAL)
- Rule 11: Memory safety (HIGH)

- Add // SAFETY: comments to every unsafe block explaining the invariants that make each operation safe.
  - Before: `let n = unsafe { libc::read(fd, buf.as_mut_ptr() as *mut _, buf.len()) };`
  - After: `// SAFETY: fd is a valid open TUN file descriptor owned by this MultiQueueTun.
// buf is a valid mutable slice for the duration of this call.
let n = unsafe { libc::read(fd, buf.as_mut_ptr() as *mut _, buf.len()) };`
- Fix file descriptor leak when open_tun_queue fails mid-loop in create(). Wrap the fallible call so previously collected fds are closed on error.
  - Before: `for _ in 1..num_queues {
    let fd = open_tun_queue(None)?;`
  - After: `for _ in 1..num_queues {
    let fd = open_tun_queue(None).map_err(|e| {
        for &prev_fd in &fds {
            libc::close(prev_fd);
        }
        e
    })?;`
- Add Debug to MultiQueueTun so library users can print the struct in error messages and debug output.
  - Before: `pub struct MultiQueueTun {
    fds: Vec<i32>,
    name: String,
    num_queues: usize,
}`
  - After: `#[derive(Debug)]
pub struct MultiQueueTun {
    fds: Vec<i32>,
    name: String,
    num_queues: usize,
}`
- Replace queue_fd() returning a raw i32 with a safer accessor to prevent callers from storing stale file descriptor values that become invalid when MultiQueueTun is dropped.
  - Before: `pub fn queue_fd(&self, queue: usize) -> i32 {
    self.fds[queue % self.num_queues]
}`
  - After: `/// Returns the fd for the given queue. The returned value is only valid
/// for the lifetime of this MultiQueueTun.
pub fn queue_fd(&self, queue: usize) -> BorrowedFd<'_> {
    // SAFETY: fd is open and valid for 'self lifetime
    unsafe { BorrowedFd::borrow_raw(self.fds[queue % self.num_queues]) }
}`

### `rustguard-tun/src/xdp.rs` — 6/10

**Failed rules:**

- Rule 3: Proper error handling with Result/Option (no silent ignores) (HIGH)
- Rule 11: Memory safety (no leaks, proper Drop impls) (HIGH)

- Check return values of the four ring-size setsockopt calls in create_inner to surface configuration failures instead of silently producing a broken socket.
  - Before: `setsockopt_raw(fd, SOL_XDP, XDP_UMEM_FILL_RING, &rs);
setsockopt_raw(fd, SOL_XDP, XDP_UMEM_COMPLETION_RING, &rs);
setsockopt_raw(fd, SOL_XDP, XDP_RX_RING, &rs);
setsockopt_raw(fd, SOL_XDP, XDP_TX_RING, &rs);`
  - After: `for &opt in &[XDP_UMEM_FILL_RING, XDP_UMEM_COMPLETION_RING, XDP_RX_RING, XDP_TX_RING] {
    if setsockopt_raw(fd, SOL_XDP, opt, &rs) < 0 {
        let err = io::Error::last_os_error();
        libc::close(fd);
        libc::munmap(umem as *mut _, umem_size);
        return Err(err);
    }
}`
- Store ring map base pointers and sizes in XdpSocket so Drop can unmap them, eliminating the four mmap leaks.
  - Before: `pub struct XdpSocket {
    fd: i32,
    umem: *mut u8,
    umem_size: usize,
    frame_size: u32,
    rx_ring: Ring,
    tx_ring: Ring,
    fill_ring: Ring,
    comp_ring: Ring,
    // ...
}
// Drop only unmaps umem`
  - After: `pub struct XdpSocket {
    fd: i32,
    umem: *mut u8,
    umem_size: usize,
    frame_size: u32,
    rx_ring: Ring,
    tx_ring: Ring,
    fill_ring: Ring,
    comp_ring: Ring,
    ring_maps: [(*mut u8, usize); 4], // (base, size) for rx/tx/fill/comp
    // ...
}
// In Drop:
for &(base, sz) in &self.ring_maps {
    libc::munmap(base as *mut libc::c_void, sz);
}`
- Add // Safety: comments to inline unsafe blocks in safe public methods, documenting the invariants that justify each block.
  - Before: `let desc = unsafe {
    &*(self.rx_ring.descs.add(idx * std::mem::size_of::<XdpDesc>()) as *const XdpDesc)
};`
  - After: `// Safety: idx is masked to (ring_size - 1), so the offset stays within the
// mmap'd ring region. The Ring is valid for the lifetime of XdpSocket.
let desc = unsafe {
    &*(self.rx_ring.descs.add(idx * std::mem::size_of::<XdpDesc>()) as *const XdpDesc)
};`
- Add Debug and Clone derives to XdpConfig and add Debug and PartialEq to XdpDesc for ergonomic use in tests and logging.
  - Before: `pub struct XdpConfig {
    pub ifname: String,
    pub queue_id: u32,
    pub frame_size: u32,
    pub num_frames: u32,
    pub ring_size: u32,
}`
  - After: `#[derive(Debug, Clone)]
pub struct XdpConfig {
    pub ifname: String,
    pub queue_id: u32,
    pub frame_size: u32,
    pub num_frames: u32,
    pub ring_size: u32,
}`

### `rustguard-enroll/src/server.rs` — 4.5/10

**Failed rules:**

- Rule 1: No unwrap in production code (CRITICAL)
- Rule 3: Proper error handling with Result/Option (HIGH)

- Replace .unwrap() on lock acquisition with .expect() carrying diagnostic context to distinguish poison from logic errors
  - Before: `let peers = state_out.peers.read().unwrap();`
  - After: `let peers = state_out.peers.read().expect("peers RwLock poisoned — a peer-processing thread panicked");`
- In the io_uring path, ps.session.as_mut().unwrap() re-asserts what was checked in a prior match without preserving the binding — use let-else instead
  - Before: `let session = ps.session.as_mut().unwrap();`
  - After: `let Some(session) = ps.session.as_mut() else {
    uring.bufs.free(comp.buf_idx);
    continue;
};`
- Log TUN write errors rather than silently discarding them
  - Before: `let _ = tun_in.write(&decrypt_buf[..pt_len]);`
  - After: `if let Err(e) = tun_in.write(&decrypt_buf[..pt_len]) {
    eprintln!("[warn] tun write error: {e}");
}`
- Log state persistence errors so operators know when peer state is not durable
  - Before: `let _ = state::save(path, &persisted);`
  - After: `if let Err(e) = state::save(path, &persisted) {
    eprintln!("[warn] failed to persist peer state to {}: {e}", path.display());
}`
- Hoist the per-iteration packets Vec outside the inbound loop to eliminate hot-path heap allocations
  - Before: `while running_in.load(Ordering::Relaxed) {
    let mut packets: Vec<(SocketAddr, Vec<u8>)> = Vec::new();`
  - After: `let mut packets: Vec<(SocketAddr, Vec<u8>)> = Vec::new();
while running_in.load(Ordering::Relaxed) {
    packets.clear();`
- Derive Debug and Clone on the public ServeConfig struct
  - Before: `pub struct ServeConfig {`
  - After: `#[derive(Debug, Clone)]
pub struct ServeConfig {`

### `rustguard-core/src/replay.rs` — 8.5/10

**Failed rules:**

- Rule 4: Derive common traits on public types (MEDIUM)

- Derive common traits on ReplayWindow to satisfy Rule 4 and improve ergonomics in tests and diagnostics.
  - Before: `pub struct ReplayWindow {
    top: u64,
    bitmap: [u64; BITMAP_LEN],
}`
  - After: `#[derive(Debug, Clone, PartialEq)]
pub struct ReplayWindow {
    top: u64,
    bitmap: [u64; BITMAP_LEN],
}`
- Implement Default to satisfy the clippy::new_without_default lint (Rule 6).
  - Before: `impl ReplayWindow {
    pub fn new() -> Self {
        Self { top: 0, bitmap: [0; BITMAP_LEN] }
    }`
  - After: `impl Default for ReplayWindow {
    fn default() -> Self {
        Self { top: 0, bitmap: [0; BITMAP_LEN] }
    }
}

impl ReplayWindow {
    pub fn new() -> Self {
        Self::default()
    }`
- Extract the repeated 'is empty' predicate into a private helper to reduce duplication and improve readability (Rule 6).
  - Before: `if self.top == 0 && self.bitmap == [0; BITMAP_LEN] { ... } // appears in both check() and update()`
  - After: `#[inline]
fn is_empty(&self) -> bool {
    self.top == 0 && self.bitmap == [0u64; BITMAP_LEN]
}
// Then: if self.is_empty() { ... }`
- Add /// doc comments to the ReplayWindow struct and its new() constructor (Rule 9).
  - Before: `pub struct ReplayWindow {
    ...
}

impl ReplayWindow {
    pub fn new() -> Self {`
  - After: `/// Sliding-window replay filter (2048-bit bitmap, RFC 6479 / WireGuard).
///
/// Tracks seen nonces and rejects duplicates or counters that have
/// fallen below the window floor.
pub struct ReplayWindow {
    ...
}

impl ReplayWindow {
    /// Creates a new, empty replay window ready to accept the first packet.
    pub fn new() -> Self {`

### `rustguard-daemon/src/tunnel.rs` — 1.5/10

**Failed rules:**

- Rule 1: No unwrap in production code (CRITICAL)
- Rule 2: No unsafe blocks without clear justification comment (CRITICAL)

- Replace Mutex::lock().unwrap() with poison-tolerant recovery
  - Before: `let mut st = state.lock().unwrap();`
  - After: `let mut st = state.lock().unwrap_or_else(|poisoned| {
    eprintln!("state mutex poisoned, recovering");
    poisoned.into_inner()
});`
- Add // SAFETY: comments to unsafe blocks in ctrlc_handler
  - Before: `unsafe {
    libc::signal(libc::SIGINT, signal_noop as *const () as libc::sighandler_t);
    libc::signal(libc::SIGTERM, signal_noop as *const () as libc::sighandler_t);
}`
  - After: `// SAFETY: signal_noop is an extern "C" fn performing no operations, satisfying
// async-signal-safety requirements. We register it before blocking the signal
// to prevent the default termination handler from firing between registration
// and the subsequent sigwait call in the spawned thread.
unsafe {
    libc::signal(libc::SIGINT, signal_noop as *const () as libc::sighandler_t);
    libc::signal(libc::SIGTERM, signal_noop as *const () as libc::sighandler_t);
}`
- Propagate getrandom error instead of panicking
  - Before: `fn fill_random(buf: &mut [u8]) {
    getrandom::getrandom(buf).expect("failed to get random bytes");
}`
  - After: `fn fill_random(buf: &mut [u8]) -> io::Result<()> {
    getrandom::getrandom(buf)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}`
- Use VecDeque instead of Vec for the pending-handshakes ring-buffer to get O(1) pop_front
  - Before: `if st.pending_handshakes.len() >= MAX_PENDING_HANDSHAKES {
    st.pending_handshakes.remove(0); // Drop oldest.
}`
  - After: `// Change field type to VecDeque<(u32, Instant, InitiatorHandshake)>
if st.pending_handshakes.len() >= MAX_PENDING_HANDSHAKES {
    st.pending_handshakes.pop_front(); // O(1) drop of oldest.
}`
- Avoid holding the TunnelState mutex across UDP send in the outbound thread
  - Before: `let mut st = state_out.lock().unwrap();
// ... build transport ...
if let Err(e) = udp_out.send_to(&wire, endpoint) {`
  - After: `let wire = {
    let mut st = state_out.lock().unwrap_or_else(|p| p.into_inner());
    // ... build transport, return wire bytes ...
    transport.to_bytes()
}; // lock released before I/O
if let Err(e) = udp_out.send_to(&wire, endpoint) {`

### `rustguard-core/src/messages.rs` — 6/10

**Failed rules:**

- Rule 1: No unwrap in production code (CRITICAL)

- Replace .unwrap() with .expect() carrying an infallibility justification to preserve intent and meet the no-unwrap rule
  - Before: `pub fn from_bytes(buf: &[u8; INITIATION_SIZE]) -> Self {
    Self {
        sender_index: u32::from_le_bytes(buf[4..8].try_into().unwrap()),
        ephemeral: buf[8..40].try_into().unwrap(),
        encrypted_static: buf[40..88].try_into().unwrap(),
        encrypted_timestamp: buf[88..116].try_into().unwrap(),
        mac1: buf[116..132].try_into().unwrap(),
        mac2: buf[132..148].try_into().unwrap(),
    }
}`
  - After: `pub fn from_bytes(buf: &[u8; INITIATION_SIZE]) -> Self {
    Self {
        sender_index: u32::from_le_bytes(
            buf[4..8].try_into().expect("infallible: fixed-size array guarantees 4-byte slice"),
        ),
        ephemeral: buf[8..40].try_into().expect("infallible: fixed-size array guarantees 32-byte slice"),
        encrypted_static: buf[40..88].try_into().expect("infallible: fixed-size array guarantees 48-byte slice"),
        encrypted_timestamp: buf[88..116].try_into().expect("infallible: fixed-size array guarantees 28-byte slice"),
        mac1: buf[116..132].try_into().expect("infallible: fixed-size array guarantees 16-byte slice"),
        mac2: buf[132..148].try_into().expect("infallible: fixed-size array guarantees 16-byte slice"),
    }
}`
- Add Debug, Clone, and PartialEq derives to all public structs, including Transport which currently has none
  - Before: `pub struct Transport {
    pub receiver_index: u32,
    pub counter: u64,
    pub payload: Vec<u8>,
}

#[derive(Clone)]
pub struct CookieReply {`
  - After: `#[derive(Clone, Debug, PartialEq)]
pub struct Transport {
    pub receiver_index: u32,
    pub counter: u64,
    pub payload: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CookieReply {`
- Add /// documentation to public methods and size constants
  - Before: `pub const INITIATION_SIZE: usize = 148;

pub const RESPONSE_SIZE: usize = 92;`
  - After: `/// Total wire size in bytes of a [`Initiation`] message.
pub const INITIATION_SIZE: usize = 148;

/// Total wire size in bytes of a [`Response`] message.
pub const RESPONSE_SIZE: usize = 92;`
- Document to_bytes and from_bytes methods to clarify ownership and infallibility contracts
  - Before: `pub fn to_bytes(&self) -> [u8; INITIATION_SIZE] {
pub fn from_bytes(buf: &[u8; INITIATION_SIZE]) -> Self {`
  - After: `/// Serializes this initiation message into its canonical 148-byte wire representation.
pub fn to_bytes(&self) -> [u8; INITIATION_SIZE] {

/// Deserializes an initiation message from its 148-byte wire representation.
///
/// This function is infallible: the fixed-size input guarantees all field slices
/// are exactly the right length.
pub fn from_bytes(buf: &[u8; INITIATION_SIZE]) -> Self {`

### `rustguard-enroll/src/control.rs` — 6/10

**Failed rules:**

- Rule 1: No unwrap in production code (CRITICAL)

- Replace `.unwrap()` on `duration_since(UNIX_EPOCH)` with a saturating fallback. UNIX_EPOCH is the zero point; a system clock before it is pathological but possible in embedded/VM environments.
  - Before: `let now = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs() as i64;`
  - After: `let now = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap_or_default()
    .as_secs() as i64;`
- Replace `peer_count.lock().unwrap()` with a graceful fallback to avoid panicking on mutex poisoning.
  - Before: `let count = *peer_count.lock().unwrap();`
  - After: `let count = peer_count.lock().map(|g| *g).unwrap_or(0);`
- Use `Acquire`/`Release` ordering for the enrollment deadline atomic to ensure correct cross-thread visibility on all architectures.
  - Before: `window.store(deadline, Ordering::Relaxed);
// ...
let deadline = window.load(Ordering::Relaxed);`
  - After: `window.store(deadline, Ordering::Release);
// ...
let deadline = window.load(Ordering::Acquire);`
- Add a doc comment to `new_window` for API completeness.
  - Before: `pub fn new_window() -> EnrollmentWindow {`
  - After: `/// Create a new, initially-closed enrollment window.
pub fn new_window() -> EnrollmentWindow {`
- Log or break on write errors in `handle_client` instead of silently discarding them.
  - Before: `let _ = writer.write_all(msg.as_bytes());`
  - After: `if writer.write_all(msg.as_bytes()).is_err() {
    break;
}`

### `rustguard-tun/src/linux.rs` — 6.75/10

**Failed rules:**

- Rule 2: No unsafe blocks without clear justification comment (CRITICAL)

- Add `// SAFETY:` justification comments to every unsafe block, explaining the invariants that make each FFI call sound.
  - Before: `pub fn read(fd: i32, buf: &mut [u8]) -> io::Result<usize> {
    let n = unsafe { libc::read(fd, buf.as_mut_ptr() as *mut _, buf.len()) };
    ...
}`
  - After: `pub fn read(fd: i32, buf: &mut [u8]) -> io::Result<usize> {
    // SAFETY: `fd` is a valid open file descriptor owned by the caller.
    // `buf` is valid for writes of exactly `buf.len()` bytes for the
    // duration of the call. The return value is checked immediately.
    let n = unsafe { libc::read(fd, buf.as_mut_ptr() as *mut _, buf.len()) };
    ...
}`
- Minimise unsafe scope in `create()` and `configure_interface()` by extracting only the FFI calls into targeted unsafe blocks rather than wrapping the entire function body.
  - Before: `pub fn create(config: &TunConfig) -> io::Result<Tun> {
    unsafe {
        let fd = libc::open(...);
        // ... all safe Rust logic also inside unsafe
    }
}`
  - After: `pub fn create(config: &TunConfig) -> io::Result<Tun> {
    // SAFETY: path is a valid NUL-terminated C string literal.
    let fd = unsafe {
        libc::open(b"/dev/net/tun\0".as_ptr() as *const libc::c_char,
                   libc::O_RDWR | libc::O_CLOEXEC)
    };
    if fd < 0 { return Err(last_os_error()); }
    // ... safe Rust logic outside unsafe ...
}`
- Add a `///` doc comment to `pub fn create` describing its purpose, the config fields it consumes, and possible error conditions.
  - Before: `pub fn create(config: &TunConfig) -> io::Result<Tun> {`
  - After: `/// Create and configure a Linux TUN device.
///
/// Opens `/dev/net/tun`, registers an `IFF_TUN | IFF_NO_PI` interface,
/// and configures the address, destination, netmask, MTU, and `IFF_UP`
/// flag via a temporary `AF_INET` socket.
///
/// # Errors
/// Returns `io::Error` if the device cannot be opened, any ioctl fails,
/// or the kernel-assigned interface name is not valid UTF-8.
pub fn create(config: &TunConfig) -> io::Result<Tun> {`

### `rustguard-tun/src/bpf_loader.rs` — 2/10

**Failed rules:**

- Rule 1: No unwrap in production code (CRITICAL)
- Rule 2: No unsafe blocks without clear justification comment (CRITICAL)
- Rule 11: Memory safety (no leaks via mem::forget, proper Drop impls) (HIGH)

- Replace .try_into().unwrap() with bounds-checked error propagation in parse_and_patch_elf
  - Before: `let e_shoff = u64::from_le_bytes(elf[40..48].try_into().unwrap()) as usize;`
  - After: `let e_shoff = u64::from_le_bytes(
    elf.get(40..48)
        .and_then(|s| s.try_into().ok())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "ELF header truncated"))?
) as usize;`
- Add // SAFETY: comments to every unsafe block documenting the invariants upheld
  - Before: `let fd = unsafe {
    libc::syscall(
        libc::SYS_bpf,
        BPF_MAP_CREATE,
        &attr as *const _,
        std::mem::size_of::<BpfAttrMapCreate>(),
    )
} as i32;`
  - After: `// SAFETY: `attr` is a valid, fully-initialized BpfAttrMapCreate whose size matches
// the third argument. SYS_bpf is a stable Linux syscall. No aliasing occurs.
let fd = unsafe {
    libc::syscall(
        libc::SYS_bpf,
        BPF_MAP_CREATE,
        &attr as *const _,
        std::mem::size_of::<BpfAttrMapCreate>(),
    )
} as i32;`
- Fix file-descriptor leak in load_and_attach by cleaning up on error paths
  - Before: `let xsks_map_fd = bpf_create_xskmap(64)?;
let insns = parse_and_patch_elf(XDP_WG_OBJ, xsks_map_fd)?;
let prog_fd = bpf_prog_load(&insns)...?;
attach_xdp(ifindex, prog_fd)...?;
Ok(Self { prog_fd, xsks_map_fd, ifindex })`
  - After: `let xsks_map_fd = bpf_create_xskmap(64)?;
let insns = parse_and_patch_elf(XDP_WG_OBJ, xsks_map_fd).map_err(|e| {
    unsafe { libc::close(xsks_map_fd); }
    e
})?;
let prog_fd = bpf_prog_load(&insns).map_err(|e| {
    unsafe { libc::close(xsks_map_fd); }
    io::Error::new(e.kind(), format!("prog_load: {e}"))
})?;
attach_xdp(ifindex, prog_fd).map_err(|e| {
    unsafe { libc::close(prog_fd); libc::close(xsks_map_fd); }
    io::Error::new(e.kind(), format!("xdp_attach ifindex={ifindex}: {e}"))
})?;
Ok(Self { prog_fd, xsks_map_fd, ifindex })`
- Derive Debug on XdpProgram and document public fields
  - Before: `pub struct XdpProgram {
    pub prog_fd: i32,
    pub xsks_map_fd: i32,
    ifindex: u32,
}`
  - After: `#[derive(Debug)]
pub struct XdpProgram {
    /// Kernel file descriptor for the loaded XDP BPF program. Managed by Drop; do not close externally.
    pub prog_fd: i32,
    /// Kernel file descriptor for the XSKMAP used to register AF_XDP sockets. Managed by Drop.
    pub xsks_map_fd: i32,
    ifindex: u32,
}`

### `rustguard-enroll/src/fast_udp.rs` — 5.5/10

**Failed rules:**

- Rule 2: No unsafe blocks without clear justification comment (CRITICAL)

- Add `// SAFETY:` justification comments before every unsafe block to explain the invariants that make each block sound.
  - Before: `let mut iovecs: [libc::iovec; BATCH_SIZE] = unsafe { std::mem::zeroed() };
let mut msgs: [libc::mmsghdr; BATCH_SIZE] = unsafe { std::mem::zeroed() };
let mut addrs: [libc::sockaddr_storage; BATCH_SIZE] = unsafe { std::mem::zeroed() };`
  - After: `// SAFETY: `iovec`, `mmsghdr`, and `sockaddr_storage` are C structs whose
// all-zeros representation is a valid, well-defined initial state per POSIX.
let mut iovecs: [libc::iovec; BATCH_SIZE] = unsafe { std::mem::zeroed() };
let mut msgs: [libc::mmsghdr; BATCH_SIZE] = unsafe { std::mem::zeroed() };
let mut addrs: [libc::sockaddr_storage; BATCH_SIZE] = unsafe { std::mem::zeroed() };`
- Add `// SAFETY:` comment before the pointer-cast unsafe blocks in `sockaddr_to_socketaddr`.
  - Before: `libc::AF_INET => {
    let sin = unsafe { &*(sa as *const _ as *const libc::sockaddr_in) };`
  - After: `libc::AF_INET => {
    // SAFETY: `ss_family` has been checked to be AF_INET, so the underlying
    // storage is a valid `sockaddr_in`. Alignment is guaranteed by `sockaddr_storage`.
    let sin = unsafe { &*(sa as *const _ as *const libc::sockaddr_in) };`
- Derive `Debug`, `Clone`, and `PartialEq` on `RecvBatch` and implement `Default` to satisfy clippy's `new_without_default` lint.
  - Before: `/// A batch of received packets.
pub struct RecvBatch {
    pub bufs: [[u8; PKT_BUF_SIZE]; BATCH_SIZE],
    pub lens: [usize; BATCH_SIZE],
    pub addrs: [Option<SocketAddr>; BATCH_SIZE],
    pub count: usize,
}`
  - After: `/// A batch of received packets.
#[derive(Debug, Clone, PartialEq)]
pub struct RecvBatch {
    /// Raw packet data buffers.
    pub bufs: [[u8; PKT_BUF_SIZE]; BATCH_SIZE],
    /// Byte length of each received packet.
    pub lens: [usize; BATCH_SIZE],
    /// Source address of each received packet.
    pub addrs: [Option<SocketAddr>; BATCH_SIZE],
    /// Number of valid entries in this batch.
    pub count: usize,
}

impl Default for RecvBatch {
    fn default() -> Self {
        Self::new()
    }
}`
- Add a doc comment to the macOS `send_packet` and to `RecvBatch::new()`.
  - Before: `#[cfg(target_os = "macos")]
pub fn send_packet(sock: &UdpSocket, buf: &[u8], addr: SocketAddr) -> io::Result<usize> {`
  - After: `/// Send a single packet (macOS fallback via `send_to`).
#[cfg(target_os = "macos")]
pub fn send_packet(sock: &UdpSocket, buf: &[u8], addr: SocketAddr) -> io::Result<usize> {`
