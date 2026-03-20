use std::path::PathBuf;
use std::process;

use rustguard_daemon::config::Config;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("up") => cmd_up(&args[2..]),
        Some("genkey") => cmd_genkey(),
        Some("pubkey") => cmd_pubkey(),
        Some(cmd) => {
            eprintln!("unknown command: {cmd}");
            usage();
            process::exit(1);
        }
        None => {
            usage();
            process::exit(1);
        }
    }
}

fn usage() {
    eprintln!("usage: rustguard <command>");
    eprintln!();
    eprintln!("commands:");
    eprintln!("  up <config>   bring up a WireGuard interface");
    eprintln!("  genkey        generate a private key");
    eprintln!("  pubkey        derive public key from private key on stdin");
}

fn cmd_up(args: &[String]) {
    let config_path = match args.first() {
        Some(p) => PathBuf::from(p),
        None => {
            eprintln!("usage: rustguard up <config>");
            process::exit(1);
        }
    };

    let config = match Config::from_file(&config_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("failed to read config: {e}");
            process::exit(1);
        }
    };

    println!("rustguard starting...");
    if let Err(e) = rustguard_daemon::tunnel::run(config) {
        eprintln!("tunnel error: {e}");
        process::exit(1);
    }
}

fn cmd_genkey() {
    use base64::prelude::*;
    let secret = rustguard_crypto::StaticSecret::random();
    println!("{}", BASE64_STANDARD.encode(secret.to_bytes()));
}

fn cmd_pubkey() {
    use base64::prelude::*;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("failed to read stdin");
    let bytes = BASE64_STANDARD
        .decode(input.trim())
        .expect("invalid base64");
    let key: [u8; 32] = bytes.try_into().expect("key must be 32 bytes");
    let secret = rustguard_crypto::StaticSecret::from_bytes(key);
    let public = secret.public_key();
    println!("{}", BASE64_STANDARD.encode(public.as_bytes()));
}
