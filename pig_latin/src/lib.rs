pub fn pig_latin(text: &str) -> String {
    let mut res = String::new();
    let mut  v = "aeiou";

    let first = text.chars().nth(0).unwrap();
    if v.contains(first) {
        return format!("{text}ay");
    }
    if text.starts_with("qu") {
        if let Some(rest) = text.strip_prefix("q") {
            for ch in rest.chars() {
                res.push(ch);
            }
           return  format!("{res}qay");
        }
    }
    let mut found = false;
    let mut first=String::new();
    let mut last=String::new();
    for c in text.chars(){
        if v.contains(c)||found{
            found=true;
            first.push(c);
        }else if !found{
            last.push(c);
        }
    }
    format!("{first}{last}ay")

    
}
