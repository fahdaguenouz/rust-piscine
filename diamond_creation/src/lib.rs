
pub fn get_diamond(c: char) -> Vec<String> {
    let mut result = Vec::new();
    
    let target = (c as u8 - b'A') as usize;
    
    let total_width = if target == 0 { 1 } else { 2 * target + 1 };
    
    for i in 0..=target {
        let letter = (b'A' + i as u8) as char;
        let leading_spaces = target - i;
        
        let line = if i == 0 {
            format!("{}{}{}", 
                " ".repeat(leading_spaces), 
                letter,
                " ".repeat(leading_spaces)
            )
        } else {
            let inner_spaces = 2 * i - 1;
            let content = format!("{}{}{}{}", 
                " ".repeat(leading_spaces), 
                letter, 
                " ".repeat(inner_spaces), 
                letter
            );
            format!("{}{}", content, " ".repeat(leading_spaces))
        };
        
        result.push(line);
    }
    
    for i in (0..target).rev() {
        let letter = (b'A' + i as u8) as char;
        let leading_spaces = target - i;
        
        let line = if i == 0 {
            format!("{}{}{}", 
                " ".repeat(leading_spaces), 
                letter,
                " ".repeat(leading_spaces)
            )
        } else {
            let inner_spaces = 2 * i - 1;
            let content = format!("{}{}{}{}", 
                " ".repeat(leading_spaces), 
                letter, 
                " ".repeat(inner_spaces), 
                letter
            );
            format!("{}{}", content, " ".repeat(leading_spaces))
        };
        
        result.push(line);
    }
    
    result
}