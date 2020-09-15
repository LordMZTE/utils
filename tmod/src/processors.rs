use std::{cell::RefCell, string::String};

use buf_rand::BufRand;

use crate::TextProcessor;

pub struct CaseRandomizer {
    randomizer: RefCell<BufRand>,
}

impl TextProcessor for CaseRandomizer {
    fn process(&self, txt: &str) -> String {
        self.randomizer.borrow_mut().rand_string_case(txt)
    }
}

impl CaseRandomizer {
    pub fn new() -> Self {
        CaseRandomizer {
            randomizer: RefCell::new(BufRand::new(Box::new(rand::thread_rng()))),
        }
    }
}
