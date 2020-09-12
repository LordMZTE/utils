pub mod processors;
pub mod randomizer;

pub trait TextProcessor {
    fn process(&self, txt: &str) -> String;
}

pub fn apply_processors(processors: &Vec<&Box<dyn TextProcessor>>, input: &str) -> String {
    let mut txt = input.to_owned();
    for proc in processors {
        txt = proc.process(&txt);
    }
    txt
}
