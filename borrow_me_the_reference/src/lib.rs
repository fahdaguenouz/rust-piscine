pub fn delete_and_backspace(s: &mut String) {

    while s.contains("+")||s.contains("-"){

        let mut i=0;
        while i<s.len(){
        let  ch= s.chars().nth(i);
           if ch == Some('-'){
            s.remove(i);
            s.remove(i-1);
            i-=1;
           }else if ch==Some('+'){
            if i<s.len()-1 && s.chars().nth(i+1)!=Some('+'){
                s.remove(i+1);
                s.remove(i);
                
            }
           }
        //    println!("{:?}",ch);
           i+=1;
        }
    }
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