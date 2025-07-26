 use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut w1 = HashMap::new();
    let mut w2 = HashMap::new();

    for c in s1.chars() {
        *w1.entry(c).or_insert(0) += 1;
    }

    for c in s2.chars() {
        *w2.entry(c).or_insert(0) += 1;
    }

    // println!("{:?}", w1);
    // println!("{:?}", w2);

    w1 == w2
}