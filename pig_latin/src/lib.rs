pub fn pig_latin(text: &str) -> String {
    let mut res = String::new();
    let  v = "aeiou";

    let first = text.chars().nth(0).unwrap();
    if v.contains(first) {
        return format!("{text}ay");
    }
    let chars: Vec<char> = text.chars().collect();
    if !v.contains(chars[0]) && chars[1] == 'q' && chars[2] == 'u' {
        let last: String = chars[0..3].iter().collect();
        let rest: String = chars[3..].iter().collect();
        return format!("{rest}{last}ay");
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
