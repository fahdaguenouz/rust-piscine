pub fn arrange_phrase(phrase: &str) -> String {
    
    
    let mut data: Vec<(String,String)> = vec![];
    let words = phrase.split_whitespace();
    for v in words{
        let mut word = String::new();
        let mut  nbr=String::new();

        for chr in v.chars(){
            if is_digit(chr)  {
                nbr=chr.to_string()
            } else if !is_digit(chr) {
                word.push(chr);
            }
        }
        data.push((word.clone(), nbr));
        
    }
     data.sort_by_key(|item| item.1.clone());

    
     data.into_iter().map(|(word, _)| word).collect::<Vec<String>>().join(" ")
    
    
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}