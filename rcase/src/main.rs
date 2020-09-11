use std::env::args;
use std::io;

use rcase::BufRandomizer;

fn main() {
    let mut rng = BufRandomizer::new(Box::new(rand::thread_rng()));
    let mut input = args();

    if input.len() > 1 {
        let mut buf = String::new();

        //remove first one.
        input.next();
        loop {
            match input.next() {
                Some(w) => {
                    buf.push_str(w.as_str());
                    buf.push(' ');
                }
                None => {
                    buf.pop();
                    break;
                }
            }
        }

        print!("{}", rng.rand_string_case(&buf))
    } else {
        loop {
            let mut buf = String::new();
            if io::stdin().read_line(&mut buf).is_err() {
                continue;
            }

            if buf.is_empty() {
                break;
            }
            print!("{}", rng.rand_string_case(&buf));
        }
    }
}
