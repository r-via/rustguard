//! Persistent state: save and restore enrolled peers across restarts.
//!
//! State file is a simple JSON array of enrolled peers.
//! Private keys are NOT stored — regenerated on restart.
//! Only peer public keys and assigned IPs are persisted.

use std::io;
use std::net::Ipv4Addr;
use std::path::{Path, PathBuf};

/// A persisted peer — just enough to re-enroll on restart.
#[derive(Clone)]
pub struct PersistedPeer {
    pub public_key: [u8; 32],
    pub assigned_ip: Ipv4Addr,
}

/// Default state file path.
pub fn default_state_path() -> PathBuf {
    PathBuf::from("/var/lib/rustguard/peers.state")
}

/// Save peers to the state file.
/// Format: one peer per line, "base64_pubkey ip"
pub fn save(path: &Path, peers: &[PersistedPeer]) -> io::Result<()> {
    use base64::prelude::*;

    // Ensure parent directory exists.
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut content = String::new();
    for peer in peers {
        let key = BASE64_STANDARD.encode(peer.public_key);
        content.push_str(&format!("{} {}\n", key, peer.assigned_ip));
    }

    // Atomic write: write to temp file, then rename.
    let tmp = path.with_extension("tmp");
    std::fs::write(&tmp, &content)?;
    std::fs::rename(&tmp, path)?;

    Ok(())
}

/// Load peers from the state file.
pub fn load(path: &Path) -> io::Result<Vec<PersistedPeer>> {
    use base64::prelude::*;

    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e),
    };

    let mut peers = Vec::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (key_str, ip_str) = line.split_once(' ').ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, format!("bad state line: {line}"))
        })?;

        let key_bytes = BASE64_STANDARD.decode(key_str).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("bad key: {e}"))
        })?;
        let public_key: [u8; 32] = key_bytes.try_into().map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "key must be 32 bytes")
        })?;
        let assigned_ip: Ipv4Addr = ip_str.parse().map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("bad ip: {e}"))
        })?;

        peers.push(PersistedPeer {
            public_key,
            assigned_ip,
        });
    }

    Ok(peers)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn save_and_load_roundtrip() {
        let dir = std::env::temp_dir().join("rustguard-test-state");
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("peers.state");

        let peers = vec![
            PersistedPeer {
                public_key: [0x42; 32],
                assigned_ip: Ipv4Addr::new(10, 150, 0, 2),
            },
            PersistedPeer {
                public_key: [0x99; 32],
                assigned_ip: Ipv4Addr::new(10, 150, 0, 3),
            },
        ];

        save(&path, &peers).unwrap();
        let loaded = load(&path).unwrap();
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].public_key, [0x42; 32]);
        assert_eq!(loaded[0].assigned_ip, Ipv4Addr::new(10, 150, 0, 2));
        assert_eq!(loaded[1].assigned_ip, Ipv4Addr::new(10, 150, 0, 3));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn load_nonexistent_returns_empty() {
        let loaded = load(Path::new("/tmp/rustguard-nonexistent-state")).unwrap();
        assert!(loaded.is_empty());
    }
}
