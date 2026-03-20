/// Smoke test: create a utun, read packets, echo them back.
///
/// Run with: sudo cargo run --example tun_echo -p rustguard-tun
///
/// Then in another terminal:
///   ping 10.0.0.2
///
/// You should see the ping requests and replies logged.
use rustguard_tun::{Tun, TunConfig};
use std::net::Ipv4Addr;

fn main() {
    let config = TunConfig {
        name: None,
        mtu: 1420,
        address: Ipv4Addr::new(10, 0, 0, 1),
        destination: Ipv4Addr::new(10, 0, 0, 2),
        netmask: Ipv4Addr::new(255, 255, 255, 255),
    };

    let tun = Tun::create(&config).expect("failed to create TUN (need root?)");
    println!("created interface: {}", tun.name());
    println!("address: 10.0.0.1 -> 10.0.0.2");
    println!("waiting for packets... (try: ping 10.0.0.2)");
    println!();

    let mut buf = [0u8; 1500];
    loop {
        match tun.read(&mut buf) {
            Ok(n) => {
                let version = buf[0] >> 4;
                let proto = if version == 4 && n >= 20 { buf[9] } else { 0 };
                let proto_name = match proto {
                    1 => "ICMP",
                    6 => "TCP",
                    17 => "UDP",
                    _ => "???",
                };
                println!(
                    "read {n} bytes: IPv{version} {proto_name} src={}.{}.{}.{} dst={}.{}.{}.{}",
                    buf[12], buf[13], buf[14], buf[15],
                    buf[16], buf[17], buf[18], buf[19],
                );

                // For ICMP echo request, swap src/dst and change type to reply.
                if proto == 1 && n >= 28 && buf[20] == 8 {
                    // Swap source and destination IP.
                    let mut reply = buf[..n].to_vec();
                    let (left, right) = reply.split_at_mut(16);
                    left[12..16].swap_with_slice(&mut right[..4]);
                    // Change ICMP type from 8 (request) to 0 (reply).
                    reply[20] = 0;
                    // Recompute ICMP checksum.
                    reply[22] = 0;
                    reply[23] = 0;
                    let cksum = icmp_checksum(&reply[20..n]);
                    reply[22] = (cksum >> 8) as u8;
                    reply[23] = (cksum & 0xff) as u8;
                    // Recompute IP header checksum.
                    reply[10] = 0;
                    reply[11] = 0;
                    let ip_cksum = ip_checksum(&reply[..20]);
                    reply[10] = (ip_cksum >> 8) as u8;
                    reply[11] = (ip_cksum & 0xff) as u8;

                    if let Err(e) = tun.write(&reply) {
                        eprintln!("write error: {e}");
                    } else {
                        println!("  -> sent ICMP echo reply");
                    }
                }
            }
            Err(e) => {
                eprintln!("read error: {e}");
                break;
            }
        }
    }
}

fn icmp_checksum(data: &[u8]) -> u16 {
    checksum_fold(data)
}

fn ip_checksum(header: &[u8]) -> u16 {
    checksum_fold(header)
}

fn checksum_fold(data: &[u8]) -> u16 {
    let mut sum: u32 = 0;
    let mut i = 0;
    while i + 1 < data.len() {
        sum += u16::from_be_bytes([data[i], data[i + 1]]) as u32;
        i += 2;
    }
    if i < data.len() {
        sum += (data[i] as u32) << 8;
    }
    while sum >> 16 != 0 {
        sum = (sum & 0xffff) + (sum >> 16);
    }
    !(sum as u16)
}
