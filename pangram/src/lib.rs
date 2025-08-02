use std::collections::HashSet;
pub fn is_pangram(s: &str) -> bool {
    let mut ch = HashSet::new();
    for c in s.chars() {
        println!("{c},{:?}",ch);
        if c.is_ascii_alphabetic() {
            ch.insert(c.to_ascii_lowercase());
        }
    }
    ch.len() == 26
}