use rand::RngCore;

pub fn randomize_case(s: String, rand: &mut dyn RngCore) -> String {
    let mut bits = rand.next_u32();
    let mut out = String::new();

    for c in s.chars() {
        out.push_str(if bits % 2 == 0 {c.to_uppercase().to_string()} else {c.to_lowercase().to_string()}.as_str());
        bits >>= 1;
        if bits == 0 {
            bits = rand.next_u32();
        }
    }

    out
}
