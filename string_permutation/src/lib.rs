 use std::collections::HashMap;
pub fn is_permutation(s1: &str, s2: &str) -> bool {
   
    let mut w1=HashMap::new();
    let mut w2=HashMap::new();

    for n1 in s1.chars(){
        // println!("{}",n1);
        w1.insert(n1,1);
    }
    // println!("{:?}",w1);

    for n2 in s2.chars(){
        w2.insert(n2,1);
    }

    w2==w1
}