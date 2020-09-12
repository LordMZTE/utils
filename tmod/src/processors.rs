use std::{cell::RefCell, string::String};

use crate::randomizer::BufRandomizer;
use crate::TextProcessor;

pub struct CaseRandomizer {
    randomizer: RefCell<BufRandomizer>,
}

impl TextProcessor for CaseRandomizer {
    fn process(&self, txt: &str) -> String {
        self.randomizer.borrow_mut().rand_string_case(txt)
    }
}

impl CaseRandomizer {
    pub fn new() -> Self {
        CaseRandomizer {
            randomizer: RefCell::new(BufRandomizer::new(Box::new(rand::thread_rng()))),
        }
    }
}
