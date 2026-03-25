// SPDX-License-Identifier: GPL-2.0

//! Anti-replay sliding window (2048-bit bitmap).
//!
//! Same algorithm as IPsec (RFC 6479) and kernel WireGuard.
//! Ported from rustguard-core/src/replay.rs — pure arithmetic, no deps.

const WINDOW_SIZE: u64 = 2048;
const BITMAP_LEN: usize = (WINDOW_SIZE / 64) as usize; // 32 u64s

pub(crate) struct ReplayWindow {
    top: u64,
    bitmap: [u64; BITMAP_LEN],
}

impl ReplayWindow {
    pub(crate) fn new() -> Self {
        Self {
            top: 0,
            bitmap: [0; BITMAP_LEN],
        }
    }

    /// Check if a counter would be acceptable (without marking it).
    pub(crate) fn check(&self, counter: u64) -> bool {
        if self.top == 0 && self.bitmap == [0; BITMAP_LEN] {
            return true;
        }
        if counter > self.top {
            return true;
        }
        let age = self.top - counter;
        if age >= WINDOW_SIZE {
            return false;
        }
        let idx = age as usize;
        let word = idx / 64;
        let bit = idx % 64;
        self.bitmap[word] & (1u64 << bit) == 0
    }

    /// Mark a counter as seen. Only call after authentication succeeds.
    pub(crate) fn update(&mut self, counter: u64) {
        if self.top == 0 && self.bitmap == [0; BITMAP_LEN] {
            self.top = counter;
            self.set_bit(0);
            return;
        }
        if counter > self.top {
            let shift = counter - self.top;
            self.shift_window(shift);
            self.top = counter;
            self.set_bit(0);
            return;
        }
        let age = self.top - counter;
        if age < WINDOW_SIZE {
            let idx = age as usize;
            let word = idx / 64;
            let bit = idx % 64;
            self.bitmap[word] |= 1u64 << bit;
        }
    }

    fn set_bit(&mut self, idx: usize) {
        let word = idx / 64;
        let bit = idx % 64;
        self.bitmap[word] |= 1u64 << bit;
    }

    fn shift_window(&mut self, shift: u64) {
        if shift >= WINDOW_SIZE {
            self.bitmap = [0; BITMAP_LEN];
            return;
        }
        let word_shift = (shift / 64) as usize;
        let bit_shift = (shift % 64) as u32;

        if word_shift > 0 {
            self.bitmap.copy_within(..BITMAP_LEN - word_shift, word_shift);
            self.bitmap[..word_shift].fill(0);
        }
        if bit_shift > 0 {
            let mut carry = 0u64;
            for word in self.bitmap.iter_mut().rev() {
                let new_carry = *word << (64 - bit_shift);
                *word = (*word >> bit_shift) | carry;
                carry = new_carry;
            }
        }
    }
}
