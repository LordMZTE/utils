use clap::{load_yaml, App};
use itertools::Itertools;
use tmod::{apply_processors, processors::CaseRandomizer, TextProcessor};

fn main() {
    //TODO add some more processors
    let processors: &[(&str, Box<dyn TextProcessor>)] =
        &[("randomize", Box::new(CaseRandomizer::new()))];

    let yaml = load_yaml!("args.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let active_processors = {
        let mut buf = vec![];
        for p in processors {
            if matches.is_present(p.0) {
                buf.push(&p.1);
            }
        }
        buf
    };

    if matches.is_present("text") {
        print!(
            "{}",
            apply_processors(
                &active_processors,
                &matches
                    .values_of("text")
                    .unwrap()
                    .into_iter()
                    .intersperse(" ")
                    .collect::<String>()
            )
        );
    } else {
        loop {
            let mut buf = String::new();

            if std::io::stdin().read_line(&mut buf).is_err() || buf.is_empty() {
                break;
            }

            print!("{}", apply_processors(&active_processors, &buf));
        }
    }
}
