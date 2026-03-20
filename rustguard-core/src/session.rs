use rustguard_crypto::{self as crypto};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// A transport session derived from a completed handshake.
///
/// Each side gets a sending key and a receiving key.
/// The initiator sends with key_send and receives with key_recv.
/// The responder has these reversed.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct TransportSession {
    /// Our sender index (the other side uses this as receiver_index).
    #[zeroize(skip)]
    pub our_index: u32,
    /// Their sender index (we use this as receiver_index in incoming packets).
    #[zeroize(skip)]
    pub their_index: u32,
    /// Key for encrypting outgoing packets.
    key_send: [u8; 32],
    /// Key for decrypting incoming packets.
    key_recv: [u8; 32],
    /// Outgoing nonce counter.
    #[zeroize(skip)]
    send_counter: u64,
}

impl TransportSession {
    pub fn new(
        our_index: u32,
        their_index: u32,
        key_send: [u8; 32],
        key_recv: [u8; 32],
    ) -> Self {
        Self {
            our_index,
            their_index,
            key_send,
            key_recv,
            send_counter: 0,
        }
    }

    /// Encrypt a plaintext packet for transport.
    /// Returns (counter, ciphertext) — caller wraps this in a Transport message.
    pub fn encrypt(&mut self, plaintext: &[u8]) -> (u64, Vec<u8>) {
        let counter = self.send_counter;
        self.send_counter = self
            .send_counter
            .checked_add(1)
            .expect("nonce counter overflow — rekey required");
        let ciphertext = crypto::seal(&self.key_send, counter, &[], plaintext);
        (counter, ciphertext)
    }

    /// Decrypt an incoming transport packet.
    /// Caller provides the counter from the Transport header.
    pub fn decrypt(&self, counter: u64, ciphertext: &[u8]) -> Option<Vec<u8>> {
        crypto::open(&self.key_recv, counter, &[], ciphertext)
    }

    pub fn send_counter(&self) -> u64 {
        self.send_counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_session_pair() -> (TransportSession, TransportSession) {
        let key_a = [0x11u8; 32];
        let key_b = [0x22u8; 32];

        let initiator = TransportSession::new(1, 2, key_a, key_b);
        let responder = TransportSession::new(2, 1, key_b, key_a);
        (initiator, responder)
    }

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let (mut initiator, responder) = make_session_pair();
        let plaintext = b"ping from the trenches";

        let (counter, ciphertext) = initiator.encrypt(plaintext);
        assert_eq!(ciphertext.len(), plaintext.len() + crypto::AEAD_TAG_LEN);

        let decrypted = responder.decrypt(counter, &ciphertext).unwrap();
        assert_eq!(&decrypted, plaintext);
    }

    #[test]
    fn counter_increments() {
        let (mut initiator, _) = make_session_pair();
        assert_eq!(initiator.send_counter(), 0);
        initiator.encrypt(b"one");
        assert_eq!(initiator.send_counter(), 1);
        initiator.encrypt(b"two");
        assert_eq!(initiator.send_counter(), 2);
    }

    #[test]
    fn wrong_counter_fails() {
        let (mut initiator, responder) = make_session_pair();
        let (counter, ciphertext) = initiator.encrypt(b"data");
        assert!(responder.decrypt(counter + 1, &ciphertext).is_none());
    }

    #[test]
    fn bidirectional() {
        let (mut initiator, mut responder) = make_session_pair();

        let (c1, ct1) = initiator.encrypt(b"hello");
        let (c2, ct2) = responder.encrypt(b"world");

        assert_eq!(responder.decrypt(c1, &ct1).unwrap(), b"hello");
        assert_eq!(initiator.decrypt(c2, &ct2).unwrap(), b"world");
    }
}
