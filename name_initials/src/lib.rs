pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
   for elem in  names.iter(){
    let words: Vec<&str> =elem.split(' ').collect();
   let ch1 = &words[0][0..1];
    let  ch2 = &words[1][0..1];
    res.push(format!("{}. {}.", ch1, ch2));
   }
   res
} 

