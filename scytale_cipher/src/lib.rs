pub fn scytale_cipher(message: &str, i: usize) -> String {
    let len = message.len();
    if len == 0 || i == 0 {
        return String::new();
    }

    let cols = (len + i - 1) / i;
    println!("{cols},{len}");
    let mut table = message.to_string();

    while table.len() < cols * i {
        table.push(' ');
        println!("{table},{:?}",table.len());
    }

    let mut res = String::new();

    for row in 0..i {
        for col in 0..cols {
            let idx = col * i + row;
            if let Some(ch) = table.chars().nth(idx) {
                res.push(ch);
            }
        }
    }

    res.trim().to_string()
}
