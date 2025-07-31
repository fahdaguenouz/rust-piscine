

pub fn check_ms(message: &str) -> Result<&str, &'static str> {
    let content = message.trim();
    if !content.is_empty() && !content.contains("stupid") {
        Ok(content)
    } else {
        Err("ERROR: illegal")
    }
}