
#[derive(Debug)]
pub enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

pub fn check_ms(message: &str) -> MyResult<&str, &'static str> {
    let content = message.trim();
    if !content.is_empty() && !content.contains("stupid") {
        MyResult::Ok(content)
    } else {
        MyResult::Err("ERROR: illegal")
    }
}