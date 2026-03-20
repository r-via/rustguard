//! Peer state management.
//!
//! Each peer has a public key, optional endpoint, allowed IPs,
//! and an active transport session (after handshake).

use std::net::SocketAddr;

use rustguard_core::session::TransportSession;
use rustguard_crypto::PublicKey;

use crate::config::{CidrAddr, PeerConfig};

/// Runtime state for a WireGuard peer.
pub struct Peer {
    pub public_key: PublicKey,
    pub endpoint: Option<SocketAddr>,
    pub allowed_ips: Vec<CidrAddr>,
    pub persistent_keepalive: Option<u16>,
    /// Active transport session, set after successful handshake.
    pub session: Option<TransportSession>,
}

impl Peer {
    pub fn from_config(config: &PeerConfig) -> Self {
        Self {
            public_key: PublicKey::from_bytes(config.public_key),
            endpoint: config.endpoint,
            allowed_ips: config.allowed_ips.clone(),
            persistent_keepalive: config.persistent_keepalive,
            session: None,
        }
    }

    /// Check if this peer is allowed to send/receive packets for the given IP.
    pub fn allows_ip(&self, ip: std::net::Ipv4Addr) -> bool {
        self.allowed_ips.iter().any(|cidr| cidr.contains(ip))
    }

    /// Whether this peer has a completed handshake and can transport data.
    pub fn has_session(&self) -> bool {
        self.session.is_some()
    }
}
