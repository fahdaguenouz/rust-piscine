pub fn get_diamond(c: char) -> Vec<String> {
    let mut result = Vec::new();
    
    let target = (c as u8 - b'A') as usize;
    
    for i in 0..=target {
        let letter = (b'A' + i as u8) as char;
        let space = target - i;
        
        if i == 0 {
            let line = format!("{}{}", " ".repeat(space), letter);
            result.push(line);
        } else {
            let inner_spaces = 2 * i - 1;
            let line = format!("{}{}{}{}", 
                " ".repeat(space), 
                letter, 
                " ".repeat(inner_spaces), 
                letter
            );
            result.push(line);
        }
    }
    
    for i in (0..target).rev() {
        let letter = (b'A' + i as u8) as char;
        let space = target - i;
        
        if i == 0 {
            let line = format!("{}{}", " ".repeat(space), letter);
            result.push(line);
        } else {
            let inner_spaces = 2 * i - 1;
            let line = format!("{}{}{}{}", 
                " ".repeat(space), 
                letter, 
                " ".repeat(inner_spaces), 
                letter
            );
            result.push(line);
        }
    }
    
    result
}