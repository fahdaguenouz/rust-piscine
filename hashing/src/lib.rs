 use std::collections::HashMap;
pub fn mean(list: &[i32]) -> f64 {
     let sum: i32 = list.iter().sum();
    sum as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
   let mut sorted = list.to_vec();
    sorted.sort();
    let half =  sorted.len() / 2;
    // println!("{:?}",sorted);
    if list.len()%2==0{
        list[half]
    }else{
        (sorted[half - 1] + sorted[half+1])  / 2
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut nbs = HashMap::new();

    for &num in list {
        *nbs.entry(num).or_insert(0) += 1;
    }
    let mut res = list[0];
    let mut max = 0;

    for (&num, &count) in nbs.iter() {
        if count > max {
            max = count;
            res = num;
        }
    }

    res
}