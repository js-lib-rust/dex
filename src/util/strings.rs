pub fn to_titlecase(string: &str) -> String {
    if let Some(first_char) = string.chars().next() {
        let rest = &string[first_char.len_utf8()..];
            format!("{}{}.", first_char.to_lowercase(), rest)
    } else {
        String::new()
    }
}

pub fn first_word(string: &str) -> &str {
    string.split(',').next().unwrap_or(string).trim()
}