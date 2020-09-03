use rcase::randomize_case;
use std::io;
use std::env::args;

fn main() {
    let mut rng = rand::thread_rng();
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
                },
                None => {
                    buf.pop();
                    break;
                },
            }
        }

        print!("{}", randomize_case(buf, &mut rng))
    } else {
        loop {
            let mut buf = String::new();
            if let Err(_) = io::stdin().read_line(&mut buf) {
                continue;
            }

            if buf.is_empty() {
                break;
            }
            print!("{}", randomize_case(buf, &mut rng));
        }
    }
}
