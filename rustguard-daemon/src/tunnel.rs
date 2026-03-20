//! The tunnel: where TUN meets UDP meets crypto.
//!
//! Two threads:
//!   1. TUN -> UDP: read IP packets from TUN, find peer by dest IP, encrypt, send UDP
//!   2. UDP -> TUN: read UDP datagrams, handshake or decrypt, write IP packets to TUN
//!
//! Simple. No tokio. Two threads and a prayer.

use std::io;
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

use rustguard_core::handshake;
use rustguard_core::messages::{
    Initiation, Response, Transport, INITIATION_SIZE, MSG_INITIATION, MSG_RESPONSE, MSG_TRANSPORT,
    RESPONSE_SIZE, TRANSPORT_HEADER_SIZE,
};
use rustguard_crypto::StaticSecret;
use rustguard_tun::{Tun, TunConfig};

use crate::config::Config;
use crate::peer::Peer;

/// Shared state between the two tunnel threads.
struct TunnelState {
    our_static: StaticSecret,
    peers: Vec<Peer>,
    /// Maps sender_index -> peer index for routing incoming handshake responses.
    pending_handshakes:
        Vec<(u32, handshake::InitiatorHandshake)>,
}

/// Start the tunnel. Blocks until error or shutdown.
pub fn run(config: Config) -> io::Result<()> {
    let our_static = StaticSecret::from_bytes(config.interface.private_key);
    let _our_public = our_static.public_key();

    // Create TUN device.
    // For point-to-point, use the first peer's first allowed IP as destination
    // (or a dummy). Real wg-quick would set up routes, but this gets us going.
    let dest = config
        .peers
        .first()
        .and_then(|p| p.allowed_ips.first())
        .map(|c| c.addr)
        .unwrap_or(Ipv4Addr::new(10, 0, 0, 2));

    let tun = Arc::new(Tun::create(&TunConfig {
        name: None,
        mtu: 1420,
        address: config.interface.address,
        destination: dest,
        netmask: config.interface.netmask,
    })?);

    println!("interface: {}", tun.name());
    println!(
        "listening on 0.0.0.0:{}",
        config.interface.listen_port
    );

    // Bind UDP socket.
    let udp = Arc::new(
        UdpSocket::bind(SocketAddr::from((
            Ipv4Addr::UNSPECIFIED,
            config.interface.listen_port,
        )))
        .map_err(|e| io::Error::new(e.kind(), format!("bind UDP: {e}")))?,
    );

    // Build peer list.
    let peers: Vec<Peer> = config.peers.iter().map(Peer::from_config).collect();

    for peer in &peers {
        println!(
            "peer: {} endpoint={:?} allowed_ips={:?}",
            base64_key(peer.public_key.as_bytes()),
            peer.endpoint,
            peer.allowed_ips.iter().map(|c| format!("{}/{}", c.addr, c.prefix_len)).collect::<Vec<_>>(),
        );
    }

    // Add routes for AllowedIPs through the TUN interface.
    let tun_name = tun.name().to_string();
    for peer_config in &config.peers {
        for cidr in &peer_config.allowed_ips {
            let route = if cidr.prefix_len == 32 {
                format!("{}", cidr.addr)
            } else {
                format!("{}/{}", cidr.addr, cidr.prefix_len)
            };
            let result = Command::new("route")
                .args(["-n", "add", "-net", &route, "-interface", &tun_name])
                .output();
            match result {
                Ok(out) if out.status.success() => {
                    println!("route add {route} -> {tun_name}");
                }
                Ok(out) => {
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    eprintln!("route add {route} failed: {stderr}");
                }
                Err(e) => eprintln!("route command failed: {e}"),
            }
        }
    }

    let state = Arc::new(Mutex::new(TunnelState {
        our_static,
        peers,
        pending_handshakes: Vec::new(),
    }));

    // Initiate handshakes with all peers that have endpoints.
    {
        let mut st = state.lock().unwrap();
        for i in 0..st.peers.len() {
            if let Some(endpoint) = st.peers[i].endpoint {
                let sender_index = (i as u32) + 1;
                let (init_msg, init_state) = handshake::create_initiation(
                    &st.our_static,
                    &st.peers[i].public_key,
                    sender_index,
                );
                st.pending_handshakes.push((sender_index, init_state));

                let wire = init_msg.to_bytes();
                if let Err(e) = udp.send_to(&wire, endpoint) {
                    eprintln!("failed to send handshake to {endpoint}: {e}");
                } else {
                    println!("sent handshake initiation to {endpoint}");
                }
            }
        }
    }

    // Thread 1: TUN -> UDP (outbound).
    let tun_out = Arc::clone(&tun);
    let udp_out = Arc::clone(&udp);
    let state_out = Arc::clone(&state);
    let outbound = thread::spawn(move || {
        let mut buf = [0u8; 1500];
        loop {
            let n = match tun_out.read(&mut buf) {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("TUN read error: {e}");
                    continue;
                }
            };

            if n < 20 {
                continue; // Too short for IPv4.
            }

            // Extract destination IP from IPv4 header.
            let dst_ip = Ipv4Addr::new(buf[16], buf[17], buf[18], buf[19]);

            let mut st = state_out.lock().unwrap();

            // Find peer by allowed IPs.
            let peer_idx = st.peers.iter().position(|p| p.allows_ip(dst_ip));
            let Some(idx) = peer_idx else {
                continue; // No peer for this destination.
            };

            let peer = &mut st.peers[idx];
            let endpoint = match peer.endpoint {
                Some(ep) => ep,
                None => continue, // Can't send without endpoint.
            };

            if let Some(session) = &mut peer.session {
                let (counter, ciphertext) = session.encrypt(&buf[..n]);
                let transport = Transport {
                    receiver_index: session.their_index,
                    counter,
                    payload: ciphertext,
                };
                let wire = transport.to_bytes();
                if let Err(e) = udp_out.send_to(&wire, endpoint) {
                    eprintln!("UDP send error: {e}");
                }
            }
            // If no session yet, packet is dropped. Handshake is in progress.
        }
    });

    // Thread 2: UDP -> TUN (inbound).
    let tun_in = Arc::clone(&tun);
    let udp_in = Arc::clone(&udp);
    let state_in = Arc::clone(&state);
    let inbound = thread::spawn(move || {
        let mut buf = [0u8; 2048];
        loop {
            let (n, src_addr) = match udp_in.recv_from(&mut buf) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("UDP recv error: {e}");
                    continue;
                }
            };

            if n < 4 {
                continue;
            }

            let msg_type = u32::from_le_bytes(buf[..4].try_into().unwrap());

            match msg_type {
                MSG_INITIATION if n >= INITIATION_SIZE => {
                    let msg = Initiation::from_bytes(
                        buf[..INITIATION_SIZE].try_into().unwrap(),
                    );
                    let mut st = state_in.lock().unwrap();
                    let responder_index = rand_u32();

                    let result = handshake::process_initiation(
                        &st.our_static,
                        &msg,
                        responder_index,
                    );

                    if let Some((peer_pubkey, resp_msg, session)) = result {
                        // Find the peer by public key.
                        if let Some(peer) = st
                            .peers
                            .iter_mut()
                            .find(|p| p.public_key == peer_pubkey)
                        {
                            peer.session = Some(session);
                            peer.endpoint = Some(src_addr);
                            let wire = resp_msg.to_bytes();
                            if let Err(e) = udp_in.send_to(&wire, src_addr) {
                                eprintln!("failed to send response: {e}");
                            } else {
                                println!(
                                    "handshake complete with {} (they initiated)",
                                    base64_key(peer_pubkey.as_bytes()),
                                );
                            }
                        }
                    }
                }

                MSG_RESPONSE if n >= RESPONSE_SIZE => {
                    let msg = Response::from_bytes(
                        buf[..RESPONSE_SIZE].try_into().unwrap(),
                    );
                    let mut st = state_in.lock().unwrap();

                    // Find the pending handshake by receiver_index.
                    let pos = st
                        .pending_handshakes
                        .iter()
                        .position(|(idx, _)| *idx == msg.receiver_index);

                    if let Some(pos) = pos {
                        let (_, init_state) = st.pending_handshakes.remove(pos);
                        let result = handshake::process_response(
                            init_state,
                            &st.our_static,
                            &msg,
                        );

                        if let Some(session) = result {
                            // Find peer by sender_index mapping.
                            let peer_idx = (msg.receiver_index - 1) as usize;
                            if peer_idx < st.peers.len() {
                                st.peers[peer_idx].session = Some(session);
                                st.peers[peer_idx].endpoint = Some(src_addr);
                                println!(
                                    "handshake complete with {} (we initiated)",
                                    base64_key(st.peers[peer_idx].public_key.as_bytes()),
                                );
                            }
                        }
                    }
                }

                MSG_TRANSPORT if n >= TRANSPORT_HEADER_SIZE => {
                    let msg = match Transport::from_bytes(&buf[..n]) {
                        Some(m) => m,
                        None => continue,
                    };
                    let mut st = state_in.lock().unwrap();

                    // Find peer whose session has our_index matching receiver_index.
                    let peer = st.peers.iter_mut().find(|p| {
                        p.session
                            .as_ref()
                            .is_some_and(|s| s.our_index == msg.receiver_index)
                    });

                    if let Some(peer) = peer {
                        // Update endpoint (roaming).
                        peer.endpoint = Some(src_addr);

                        if let Some(session) = &peer.session {
                            if let Some(plaintext) = session.decrypt(msg.counter, &msg.payload)
                            {
                                drop(st); // Release lock before TUN write.
                                if let Err(e) = tun_in.write(&plaintext) {
                                    eprintln!("TUN write error: {e}");
                                }
                            }
                        }
                    }
                }

                _ => {} // Unknown message type, drop.
            }
        }
    });

    outbound.join().unwrap();
    inbound.join().unwrap();
    Ok(())
}

fn rand_u32() -> u32 {
    use std::time::SystemTime;
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos()
}

fn base64_key(key: &[u8; 32]) -> String {
    use base64::prelude::*;
    BASE64_STANDARD.encode(key)
}
