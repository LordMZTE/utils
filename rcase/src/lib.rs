use delegate::delegate;

use rand::RngCore;

pub struct BufRandomizer {
    bit_buf: u64,
    rand: Box<dyn RngCore>,
}

impl RngCore for BufRandomizer {
    delegate! {
        to self.rand {
            fn next_u32(&mut self) -> u32;
            fn next_u64(&mut self) -> u64;
            fn fill_bytes(&mut self, dest: &mut [u8]);
            fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error>;
        }
    }
}

impl BufRandomizer {
    pub fn new(rand: Box<dyn RngCore>) -> Box<Self> {
        Box::new(BufRandomizer {
            bit_buf: 0,
            rand,
        })
    }

    pub fn next_bool(&mut self) -> bool {
        if self.bit_buf == 0 {
            self.bit_buf = self.next_u64();
        }
        let out = self.bit_buf % 2 == 0;
        self.bit_buf >>= 1;
        out
    }

    pub fn rand_char_case(&mut self, c: &char) -> Option<char> {
        if self.next_bool() {
            c.to_uppercase().to_string()
        } else {
            c.to_lowercase().to_string()
        }.chars().next()
    }

    pub fn rand_string_case(&mut self, s: &str) -> String {
        s.chars().into_iter().map(|c| self.rand_char_case(&c).unwrap()).collect()
    }
}
