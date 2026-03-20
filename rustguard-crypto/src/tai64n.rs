use std::time::SystemTime;

/// TAI64N timestamp — 12 bytes: 8 bytes seconds (TAI64) + 4 bytes nanoseconds.
///
/// Used in WireGuard handshake initiation to prevent replay.
/// Each new handshake must have a strictly greater timestamp than the last.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tai64n([u8; 12]);

/// TAI is ahead of UTC by 10 seconds (1970 epoch) + 2^62 offset.
const TAI64_EPOCH_OFFSET: u64 = 0x4000_0000_0000_000a;

impl Tai64n {
    /// Create a timestamp for "now".
    pub fn now() -> Self {
        let duration = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("system clock before UNIX epoch");

        let secs = duration.as_secs() + TAI64_EPOCH_OFFSET;
        let nanos = duration.subsec_nanos();

        let mut buf = [0u8; 12];
        buf[..8].copy_from_slice(&secs.to_be_bytes());
        buf[8..].copy_from_slice(&nanos.to_be_bytes());
        Self(buf)
    }

    pub fn as_bytes(&self) -> &[u8; 12] {
        &self.0
    }

    pub fn from_bytes(bytes: [u8; 12]) -> Self {
        Self(bytes)
    }

    /// Returns true if `self` is strictly after `other`.
    pub fn is_after(&self, other: &Tai64n) -> bool {
        self.0 > other.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn timestamps_are_monotonic() {
        let t1 = Tai64n::now();
        thread::sleep(Duration::from_millis(10));
        let t2 = Tai64n::now();
        assert!(t2.is_after(&t1));
    }

    #[test]
    fn roundtrip() {
        let t = Tai64n::now();
        let bytes = *t.as_bytes();
        let restored = Tai64n::from_bytes(bytes);
        assert_eq!(t, restored);
    }
}
