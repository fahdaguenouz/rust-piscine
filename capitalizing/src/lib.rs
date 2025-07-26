pub fn capitalize_first(input: &str) -> String {
     let mut c = input.chars();
    match c.next() {
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let mut word = String::new();

    for c in input.chars() {
        if c.is_whitespace() {
            result.push(c);
            capitalize_next = true;
        } else if capitalize_next {
            result.extend(c.to_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    if !word.is_empty() {
        result.push_str(&capitalize_first(&word));
    }

    result
}
pub fn change_case(input: &str) -> String {
    let mut res = String::new();
    for t in input.chars() {
        if t.is_uppercase() {
            res.extend(t.to_lowercase());
        } else {
            res.extend(t.to_uppercase());
        }
    }
    res
}