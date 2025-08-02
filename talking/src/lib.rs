pub fn talking(text: &str) -> &str {
    let trimmed = text.trim();

    if trimmed.is_empty() {
        "Just say something!"
    } else if trimmed.ends_with('?') && trimmed == trimmed.to_uppercase() && trimmed.chars().any(|c| c.is_alphabetic()) {
        "Quiet, I am thinking!"
    } else if trimmed == trimmed.to_uppercase() && trimmed.chars().any(|c| c.is_alphabetic()) {
        "There is no need to yell, calm down!"
    } else if trimmed.ends_with('?') {
        "Sure."
    } else {
        "Interesting"
    }
    //  match true {
    //     _ if trimmed.is_empty() => "Just say something!",
    //     _ if trimmed.ends_with('?') && trimmed == trimmed.to_uppercase() && trimmed.chars().any(|c| c.is_alphabetic()) => {
    //         "Quiet, I am thinking!"
    //     }
    //     _ if trimmed == trimmed.to_uppercase() && trimmed.chars().any(|c| c.is_alphabetic()) => {
    //         "There is no need to yell, calm down!"
    //     }
    //     _ if trimmed.ends_with('?') => "Sure.",
    //     _ => "Interesting",
    // }
}
