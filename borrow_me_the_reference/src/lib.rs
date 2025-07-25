pub fn delete_and_backspace(s: &mut String) {
    let mut ch: Vec<char> = s.chars().collect();
    let mut copy = String::new();
    let mut count: i32 = 0;
    let mut i: usize = 0;
 
    while i < ch.len() {
        let c = ch[i];
        println!("{i} {c}");
        
        if c == '-' && !copy.is_empty() {
            println!("{copy}");
            copy.pop(); 
            i += 1;
            println!("{copy}");
        } else if c == '+' {
            count += 1;
            i += 1;
        } else if count > 0 {
            count -= 1;
            i += 1; 
        } else {
            copy.push(c);
            i += 1;
        }
    }
    *s = copy;
}
pub fn do_operations(v: &mut [String]) {
    for nbr  in v.iter_mut(){
         if let Some(pos) = nbr.find('+') {
            let left = &nbr[..pos];
            let right = &nbr[pos + 1..];
            if let (Ok(a), Ok(b)) = (left.trim().parse::<i32>(), right.trim().parse::<i32>()) {
                *nbr = (a + b).to_string();
            }
        } else if let Some(pos) = nbr.find('-') {
            let left = &nbr[..pos];
            let right = &nbr[pos + 1..];
            if let (Ok(a), Ok(b)) = (left.trim().parse::<i32>(), right.trim().parse::<i32>()) {
                *nbr = (a - b).to_string();
            }
        }
    }
}