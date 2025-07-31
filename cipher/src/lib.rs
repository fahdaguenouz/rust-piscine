use std::collections::HashMap;
#[derive(Debug, PartialEq)]
pub struct CipherError {
   pub expected:String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let cipher = gencipher(original);
    if cipher ==ciphered{
        Ok(())
    }else{
        Err(CipherError {
            expected: format!("{}", cipher),
        })
    }
}

fn gencipher(string:&str)->String{
    let mut cipher =String::new();
     let map: HashMap<char, char> = [
        ('A', 'Z'), ('B', 'Y'), ('C', 'X'), ('D', 'W'),
        ('E', 'V'), ('F', 'U'), ('G', 'T'), ('H', 'S'),
        ('I', 'R'), ('J', 'Q'), ('K', 'P'), ('L', 'O'),
        ('M', 'N'), ('N', 'M'), ('O', 'L'), ('P', 'K'),
        ('Q', 'J'), ('R', 'I'), ('S', 'H'), ('T', 'G'),
        ('U', 'F'), ('V', 'E'), ('W', 'D'), ('X', 'C'),
        ('Y', 'B'), ('Z', 'A'),
    ].iter().cloned().collect();

     for ch in string.chars() {
       if ch.is_ascii_uppercase() {
           
            if let Some(&mapped) = map.get(&ch) {
                cipher.push(mapped);
            }
        }else if ch.is_ascii_lowercase() {
            let upper = ch.to_ascii_uppercase();
            if let Some(&mapped) = map.get(&upper) {
                cipher.push(mapped.to_ascii_lowercase());
            }
        } else {
            cipher.push(ch);
        }
        
    }
    cipher
}