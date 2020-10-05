use std::{cell::RefCell, string::String};

use buf_rand::BufRand;
use rand::prelude::ThreadRng;

use crate::TextProcessor;

pub struct CaseRandomizer {
    randomizer: RefCell<BufRand<ThreadRng>>,
}

impl TextProcessor for CaseRandomizer {
    fn process(&self, txt: &str) -> String {
        self.randomizer.borrow_mut().rand_string_case(txt)
    }
}

impl CaseRandomizer {
    pub fn new() -> Self {
        CaseRandomizer {
            randomizer: RefCell::new(BufRand::new(rand::thread_rng())),
        }
    }
}
