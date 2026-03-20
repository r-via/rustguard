//! WireGuard configuration file parser.
//!
//! Parses the standard INI-style wg0.conf format that every WireGuard
//! implementation understands. Muscle memory compatible.

use base64::prelude::*;
use std::net::{Ipv4Addr, SocketAddr};
use std::path::Path;
use std::{fs, io};

/// Parsed WireGuard configuration.
#[derive(Debug)]
pub struct Config {
    pub interface: InterfaceConfig,
    pub peers: Vec<PeerConfig>,
}

#[derive(Debug)]
pub struct InterfaceConfig {
    pub private_key: [u8; 32],
    pub listen_port: u16,
    pub address: Ipv4Addr,
    pub netmask: Ipv4Addr,
}

#[derive(Debug)]
pub struct PeerConfig {
    pub public_key: [u8; 32],
    pub endpoint: Option<SocketAddr>,
    pub allowed_ips: Vec<CidrAddr>,
    pub persistent_keepalive: Option<u16>,
}

/// An IP address with prefix length (CIDR notation).
#[derive(Debug, Clone)]
pub struct CidrAddr {
    pub addr: Ipv4Addr,
    pub prefix_len: u8,
}

impl CidrAddr {
    /// Check if a given IP falls within this CIDR range.
    pub fn contains(&self, ip: Ipv4Addr) -> bool {
        if self.prefix_len == 0 {
            return true;
        }
        let mask = if self.prefix_len >= 32 {
            u32::MAX
        } else {
            u32::MAX << (32 - self.prefix_len)
        };
        let net: u32 = self.addr.into();
        let target: u32 = ip.into();
        (net & mask) == (target & mask)
    }
}

impl Config {
    pub fn from_file(path: &Path) -> io::Result<Self> {
        let contents = fs::read_to_string(path)?;
        Self::parse(&contents)
    }

    pub fn parse(input: &str) -> io::Result<Self> {
        let interface;
        let mut peers = Vec::new();
        let mut current_section: Option<&str> = None;

        // Accumulate key-value pairs per section.
        let mut iface_kvs: Vec<(&str, &str)> = Vec::new();
        let mut current_peer_kvs: Vec<(&str, &str)> = Vec::new();

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if line.eq_ignore_ascii_case("[Interface]") {
                current_section = Some("interface");
                continue;
            }
            if line.eq_ignore_ascii_case("[Peer]") {
                // Flush previous peer if any.
                if current_section == Some("peer") && !current_peer_kvs.is_empty() {
                    peers.push(parse_peer(&current_peer_kvs)?);
                    current_peer_kvs.clear();
                }
                current_section = Some("peer");
                continue;
            }

            let (key, value) = line.split_once('=').ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, format!("bad line: {line}"))
            })?;
            let key = key.trim();
            let value = value.trim();

            match current_section {
                Some("interface") => iface_kvs.push((key, value)),
                Some("peer") => current_peer_kvs.push((key, value)),
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("key outside section: {line}"),
                    ))
                }
            }
        }

        // Flush last peer.
        if !current_peer_kvs.is_empty() {
            peers.push(parse_peer(&current_peer_kvs)?);
        }

        interface = Some(parse_interface(&iface_kvs)?);

        Ok(Config {
            interface: interface
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "no [Interface]"))?,
            peers,
        })
    }
}

fn decode_key(s: &str) -> io::Result<[u8; 32]> {
    let bytes = BASE64_STANDARD
        .decode(s)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("bad base64: {e}")))?;
    bytes
        .try_into()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "key must be 32 bytes"))
}

fn parse_interface(kvs: &[(&str, &str)]) -> io::Result<InterfaceConfig> {
    let mut private_key = None;
    let mut listen_port = 51820u16;
    let mut address = None;
    let mut netmask = Ipv4Addr::new(255, 255, 255, 0);

    for &(key, value) in kvs {
        match key.to_ascii_lowercase().as_str() {
            "privatekey" => private_key = Some(decode_key(value)?),
            "listenport" => {
                listen_port = value.parse().map_err(|e| {
                    io::Error::new(io::ErrorKind::InvalidData, format!("bad port: {e}"))
                })?
            }
            "address" => {
                // Parse "10.0.0.1/24" format.
                let (addr_str, prefix) = value
                    .split_once('/')
                    .unwrap_or((value, "24"));
                address = Some(addr_str.parse::<Ipv4Addr>().map_err(|e| {
                    io::Error::new(io::ErrorKind::InvalidData, format!("bad address: {e}"))
                })?);
                let prefix_len: u8 = prefix.parse().map_err(|e| {
                    io::Error::new(io::ErrorKind::InvalidData, format!("bad prefix: {e}"))
                })?;
                netmask = prefix_to_netmask(prefix_len);
            }
            // Silently skip PostUp/PostDown/DNS/Table/etc for now.
            _ => {}
        }
    }

    Ok(InterfaceConfig {
        private_key: private_key
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing PrivateKey"))?,
        listen_port,
        address: address
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing Address"))?,
        netmask,
    })
}

fn parse_peer(kvs: &[(&str, &str)]) -> io::Result<PeerConfig> {
    let mut public_key = None;
    let mut endpoint = None;
    let mut allowed_ips = Vec::new();
    let mut persistent_keepalive = None;

    for &(key, value) in kvs {
        match key.to_ascii_lowercase().as_str() {
            "publickey" => public_key = Some(decode_key(value)?),
            "endpoint" => {
                endpoint = Some(value.parse::<SocketAddr>().map_err(|e| {
                    io::Error::new(io::ErrorKind::InvalidData, format!("bad endpoint: {e}"))
                })?)
            }
            "allowedips" => {
                for cidr in value.split(',') {
                    allowed_ips.push(parse_cidr(cidr.trim())?);
                }
            }
            "persistentkeepalive" => {
                persistent_keepalive = Some(value.parse::<u16>().map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("bad keepalive: {e}"),
                    )
                })?)
            }
            _ => {}
        }
    }

    Ok(PeerConfig {
        public_key: public_key
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing PublicKey"))?,
        endpoint,
        allowed_ips,
        persistent_keepalive,
    })
}

fn parse_cidr(s: &str) -> io::Result<CidrAddr> {
    let (addr_str, prefix_str) = s.split_once('/').unwrap_or((s, "32"));
    let addr = addr_str.parse::<Ipv4Addr>().map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("bad CIDR addr: {e}"))
    })?;
    let prefix_len = prefix_str.parse::<u8>().map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidData, format!("bad CIDR prefix: {e}"))
    })?;
    Ok(CidrAddr { addr, prefix_len })
}

fn prefix_to_netmask(prefix: u8) -> Ipv4Addr {
    if prefix == 0 {
        Ipv4Addr::new(0, 0, 0, 0)
    } else if prefix >= 32 {
        Ipv4Addr::new(255, 255, 255, 255)
    } else {
        Ipv4Addr::from(u32::MAX << (32 - prefix))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_CONFIG: &str = r#"
[Interface]
PrivateKey = cGiPH7CqyNOCaW6ykZLH9K3Bt0enk5rDiTcv1O3A+JA=
ListenPort = 51820
Address = 10.0.0.1/24

[Peer]
PublicKey = HhMN8JntZEa8iF6bc+BdJD8MGD9shwefov5Gt+95Ky8=
AllowedIPs = 10.0.0.2/32, 192.168.1.0/24
Endpoint = 203.0.113.1:51820
PersistentKeepalive = 25

[Peer]
PublicKey = tNwlJsp4WIaPpeCYNsleoE8QzJFVBLHYEHLgHVQ4NyQ=
AllowedIPs = 10.0.0.3/32
"#;

    #[test]
    fn parse_full_config() {
        let config = Config::parse(SAMPLE_CONFIG).unwrap();
        assert_eq!(config.interface.listen_port, 51820);
        assert_eq!(config.interface.address, Ipv4Addr::new(10, 0, 0, 1));
        assert_eq!(config.interface.netmask, Ipv4Addr::new(255, 255, 255, 0));
        assert_eq!(config.peers.len(), 2);

        let peer1 = &config.peers[0];
        assert_eq!(
            peer1.endpoint,
            Some("203.0.113.1:51820".parse().unwrap())
        );
        assert_eq!(peer1.allowed_ips.len(), 2);
        assert_eq!(peer1.persistent_keepalive, Some(25));

        let peer2 = &config.peers[1];
        assert!(peer2.endpoint.is_none());
        assert_eq!(peer2.allowed_ips.len(), 1);
    }

    #[test]
    fn cidr_contains() {
        let cidr = CidrAddr {
            addr: Ipv4Addr::new(10, 0, 0, 0),
            prefix_len: 24,
        };
        assert!(cidr.contains(Ipv4Addr::new(10, 0, 0, 1)));
        assert!(cidr.contains(Ipv4Addr::new(10, 0, 0, 254)));
        assert!(!cidr.contains(Ipv4Addr::new(10, 0, 1, 1)));
    }

    #[test]
    fn cidr_contains_host() {
        let cidr = CidrAddr {
            addr: Ipv4Addr::new(10, 0, 0, 2),
            prefix_len: 32,
        };
        assert!(cidr.contains(Ipv4Addr::new(10, 0, 0, 2)));
        assert!(!cidr.contains(Ipv4Addr::new(10, 0, 0, 3)));
    }

    #[test]
    fn cidr_contains_default_route() {
        let cidr = CidrAddr {
            addr: Ipv4Addr::new(0, 0, 0, 0),
            prefix_len: 0,
        };
        assert!(cidr.contains(Ipv4Addr::new(1, 2, 3, 4)));
        assert!(cidr.contains(Ipv4Addr::new(10, 0, 0, 1)));
    }
}
