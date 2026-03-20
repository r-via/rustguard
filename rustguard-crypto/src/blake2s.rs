use blake2::digest::consts::U32;
use blake2::digest::CtOutput;
use blake2::{Blake2s256, Blake2sMac, Digest};
use blake2::digest::Mac;

/// BLAKE2s-256 hash.
pub fn hash(data: &[&[u8]]) -> [u8; 32] {
    let mut hasher = Blake2s256::new();
    for chunk in data {
        hasher.update(chunk);
    }
    hasher.finalize().into()
}

/// BLAKE2s-256 keyed MAC (used for MAC1/MAC2 in WireGuard).
pub fn mac(key: &[u8], data: &[&[u8]]) -> [u8; 32] {
    let mut m = Blake2sMac::<U32>::new_from_slice(key)
        .expect("BLAKE2s accepts any key length up to 32 bytes");
    for chunk in data {
        m.update(chunk);
    }
    let result: CtOutput<Blake2sMac<U32>> = m.finalize();
    result.into_bytes().into()
}

/// HKDF using HMAC-BLAKE2s, as specified by WireGuard.
///
/// WireGuard's HKDF always uses BLAKE2s as the hash.
/// It extracts with HMAC(key, input) then expands to 1-3 outputs.
///
/// Returns (output1, output2, output3) — callers use what they need.
pub fn hkdf(key: &[u8; 32], input: &[u8]) -> ([u8; 32], [u8; 32], [u8; 32]) {
    // Extract: PRK = HMAC(key, input)
    let prk = hmac_blake2s(key, input);

    // Expand: T1 = HMAC(PRK, 0x01)
    let t1 = hmac_blake2s(&prk, &[0x01]);

    // T2 = HMAC(PRK, T1 || 0x02)
    let mut t2_input = [0u8; 33];
    t2_input[..32].copy_from_slice(&t1);
    t2_input[32] = 0x02;
    let t2 = hmac_blake2s(&prk, &t2_input);

    // T3 = HMAC(PRK, T2 || 0x03)
    let mut t3_input = [0u8; 33];
    t3_input[..32].copy_from_slice(&t2);
    t3_input[32] = 0x03;
    let t3 = hmac_blake2s(&prk, &t3_input);

    (t1, t2, t3)
}

/// HMAC-BLAKE2s as used in WireGuard's KDF.
/// HMAC(key, msg) = BLAKE2s(key, msg) using BLAKE2s's built-in keying.
fn hmac_blake2s(key: &[u8; 32], data: &[u8]) -> [u8; 32] {
    mac(key, &[data])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_deterministic() {
        let h1 = hash(&[b"hello"]);
        let h2 = hash(&[b"hello"]);
        assert_eq!(h1, h2);
    }

    #[test]
    fn hash_different_inputs() {
        let h1 = hash(&[b"hello"]);
        let h2 = hash(&[b"world"]);
        assert_ne!(h1, h2);
    }

    #[test]
    fn hash_multi_chunk() {
        let h1 = hash(&[b"hello", b"world"]);
        let h2 = hash(&[b"helloworld"]);
        assert_eq!(h1, h2);
    }

    #[test]
    fn mac_differs_with_key() {
        let m1 = mac(&[0x01; 32], &[b"data"]);
        let m2 = mac(&[0x02; 32], &[b"data"]);
        assert_ne!(m1, m2);
    }

    #[test]
    fn hkdf_produces_different_outputs() {
        let key = [0x42u8; 32];
        let (t1, t2, t3) = hkdf(&key, b"input");
        assert_ne!(t1, t2);
        assert_ne!(t2, t3);
        assert_ne!(t1, t3);
    }

    #[test]
    fn hkdf_deterministic() {
        let key = [0x42u8; 32];
        let (a1, a2, a3) = hkdf(&key, b"input");
        let (b1, b2, b3) = hkdf(&key, b"input");
        assert_eq!(a1, b1);
        assert_eq!(a2, b2);
        assert_eq!(a3, b3);
    }
}
