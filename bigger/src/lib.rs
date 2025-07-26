use std::collections::HashMap;
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
let mut max=0;
    for nbr in h{
        if nbr.1.abs()>max{
            max=nbr.1;
        }
    }
max

}