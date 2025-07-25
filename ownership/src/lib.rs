pub fn first_subword(mut s: String) -> String {
    let mut res=String::new();
    for (i,letter) in s.chars().enumerate(){
        if i != 0 && (letter == '_'||letter.is_uppercase()){
            break;
        }else{
            res+=&letter.to_string();
        }
        // println!("{}",letter);
    }
res

}