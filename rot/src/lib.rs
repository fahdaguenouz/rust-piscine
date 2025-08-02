pub fn rotate(input: &str, key: i8) -> String {
    let mut res=String::new();
    for ch in input.chars(){
     if ch.is_ascii_lowercase() {
            let mut pos = ch as i8 - b'a' as i8 + key;
            if pos < 0 {
                pos += 26;
            }
            res.push((b'a' + (pos as u8 % 26)) as char);
        } else if ch.is_ascii_uppercase() {
            let mut pos = ch as i8 - b'A' as i8 + key;
            if pos < 0 {
                pos += 26;
            }
            res.push((b'A' + (pos as u8 % 26)) as char);
        } else {
            res.push(ch);
        }
    }
   res
}
