pub fn capitalize_first(input: &str) -> String {
     let mut c = input.chars();
    match c.next() {
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

pub fn title_case(input: &str) -> String {
  input.split_whitespace().map(|word| {
        capitalize_first(word)
  }).collect::<Vec<String>>().join(" ")
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