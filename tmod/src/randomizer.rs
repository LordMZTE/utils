use core::ops::DerefMut;
use std::ops::Deref;

use rand::RngCore;

pub struct BufRandomizer {
    /// this is a buffer of bits for random booleans
    bit_buf: u64,
    /// this indicates how many bits have been read from `bit_buf`.
    /// `bit_buf` is not simply checked against 0 to prevent slight bias to false
    shift_counter: u8,
    rand: Box<dyn RngCore>,
}

impl DerefMut for BufRandomizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rand
    }
}

impl Deref for BufRandomizer {
    type Target = Box<dyn RngCore>;
    fn deref(&self) -> &Self::Target {
        &self.rand
    }
}

impl BufRandomizer {
    pub fn new(rand: Box<dyn RngCore>) -> Self {
        BufRandomizer {
            bit_buf: 0,
            shift_counter: 0xff, //not initialized with 0 to prevent bit_buf from being all 0
            rand,
        }
    }

    pub fn next_bool(&mut self) -> bool {
        if self.shift_counter >= 64 {
            self.bit_buf = self.next_u64();
            self.shift_counter = 0;
        }
        let out = self.bit_buf % 2 == 0;
        self.bit_buf >>= 1;
        self.shift_counter += 1;
        out
    }

    pub fn rand_char_case(&mut self, c: &char) -> Option<char> {
        if self.next_bool() {
            c.to_uppercase().to_string()
        } else {
            c.to_lowercase().to_string()
        }
        .chars()
        .next()
    }

    pub fn rand_string_case(&mut self, s: &str) -> String {
        s.chars()
            .into_iter()
            .map(|c| self.rand_char_case(&c).unwrap())
            .collect()
    }
}
