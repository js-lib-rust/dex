pub fn uppercase_first_char(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => {
            let capitalized_first = first.to_uppercase().to_string();
            let rest: String = chars.collect();
            capitalized_first + &rest
        }
        None => String::new(),
    }
}

pub fn lowercase_first_char(s: &str) -> String {
    let mut chars = s.chars();
    if let Some(first_char) = chars.next() {
        format!("{}{}", first_char.to_lowercase(), chars.as_str())
    } else {
        s.to_string()
    }
}

pub fn first_word(s: &str) -> &str {
    s.split(',').next().unwrap_or(s).trim()
}

pub fn starts_with_uppercase(s: &str) -> bool {
    match s.chars().next() {
        Some(first_char) => first_char.is_uppercase(),
        None => false,
    }
}

pub fn ends_with_punctuation(s: &str) -> bool {
    match s.chars().last() {
        Some(first_char) => match first_char {
            '.' | '!' | '?' => true,
            _ => false,
        },
        None => false,
    }
}

pub fn _remove_last_char(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    s[..s.len() - 1].to_string()
}

pub fn trim_end_punctuation(s: &str) -> String {
    s.trim_end_matches(|c: char| c.is_ascii_punctuation())
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    // uppercase_first_char

    #[test]
    fn test_empty_string_on_uppercase_first_char() {
        assert_eq!(uppercase_first_char(""), "");
    }

    #[test]
    fn test_single_char_on_uppercase_first_char() {
        assert_eq!(uppercase_first_char("a"), "A");
    }

    #[test]
    fn test_multiple_chars_on_uppercase_first_char() {
        assert_eq!(uppercase_first_char("hello"), "Hello");
    }

    #[test]
    fn test_mixed_case_on_uppercase_first_char() {
        assert_eq!(uppercase_first_char("hELLo"), "HELLo");
    }

    #[test]
    fn test_with_numbers_on_uppercase_first_char() {
        assert_eq!(uppercase_first_char("123hello"), "123hello");
    }

    #[test]
    fn test_with_symbols_on_uppercase_first_char() {
        assert_eq!(uppercase_first_char("!hello"), "!hello");
    }

    // remove_last_char

    #[test]
    fn test_empty_string_on_remove_last_char() {
        assert_eq!(_remove_last_char(""), "");
    }

    #[test]
    fn test_single_char_string_on_remove_last_char() {
        assert_eq!(_remove_last_char("a"), "");
    }

    #[test]
    fn test_multiple_char_string_on_remove_last_char() {
        assert_eq!(_remove_last_char("hello"), "hell");
    }

    #[test]
    fn test_string_with_spaces_on_remove_last_char() {
        assert_eq!(_remove_last_char("hello world"), "hello worl");
    }

    #[test]
    fn test_string_with_numbers_on_remove_last_char() {
        assert_eq!(_remove_last_char("12345"), "1234");
    }

    // lowercase_first_char

    #[test]
    fn test_empty_string_on_lowercase_first_char() {
        assert_eq!(lowercase_first_char(""), "");
    }

    #[test]
    fn test_single_char_on_lowercase_first_char() {
        assert_eq!(lowercase_first_char("A"), "a");
    }

    #[test]
    fn test_multiple_chars_on_lowercase_first_char() {
        assert_eq!(lowercase_first_char("Hello"), "hello");
    }

    #[test]
    fn test_already_lowercase_on_lowercase_first_char() {
        assert_eq!(lowercase_first_char("hello"), "hello");
    }

    #[test]
    fn test_mixed_case_on_lowercase_first_char() {
        assert_eq!(lowercase_first_char("HeLlO"), "heLlO");
    }

    // ends_with_punctuation

    #[test]
    fn test_empty_string_on_ends_with_punctuation() {
        assert_eq!(ends_with_punctuation(""), false);
    }

    #[test]
    fn test_ends_with_period_on_ends_with_punctuation() {
        assert_eq!(ends_with_punctuation("hello."), true);
    }

    #[test]
    fn test_ends_with_comma_on_ends_with_punctuation() {
        assert_eq!(ends_with_punctuation("hello,"), false);
    }
    #[test]
    fn test_ends_with_parenthesis_on_ends_with_punctuation() {
        assert_eq!(ends_with_punctuation("(hello)"), false);
    }

    #[test]
    fn test_ends_with_question_mark_on_ends_with_punctuation() {
        assert_eq!(ends_with_punctuation("hello?"), true);
    }

    #[test]
    fn test_ends_with_exclamation_mark_on_ends_with_punctuation() {
        assert_eq!(ends_with_punctuation("hello!"), true);
    }

    #[test]
    fn test_ends_with_letter_on_ends_with_punctuation() {
        assert_eq!(ends_with_punctuation("hello"), false);
    }

    #[test]
    fn test_ends_with_number_on_ends_with_punctuation() {
        assert_eq!(ends_with_punctuation("hello123"), false);
    }

    #[test]
    fn test_ends_with_space_on_ends_with_punctuation() {
        assert_eq!(ends_with_punctuation("hello "), false);
    }

    #[test]
    fn test_ends_with_multiple_punctuation_on_ends_with_punctuation() {
        assert_eq!(ends_with_punctuation("hello!!"), true);
    }

    #[test]
    fn test_ends_with_mixed_punctuation_on_ends_with_punctuation() {
        assert_eq!(ends_with_punctuation("hello.1"), false);
    }

    // trim_end_punctuation

    #[test]
    fn test_empty_string_on_trim_end_punctuation() {
        assert_eq!(trim_end_punctuation(""), "");
    }

    #[test]
    fn test_no_punctuation_on_trim_end_punctuation() {
        assert_eq!(trim_end_punctuation("hello world"), "hello world");
    }

    #[test]
    fn test_punctuation_at_end_on_trim_end_punctuation() {
        assert_eq!(trim_end_punctuation("hello, world!"), "hello, world");
    }

    #[test]
    fn test_multiple_punctuation_on_trim_end_punctuation() {
        assert_eq!(trim_end_punctuation("hello!!! world???"), "hello!!! world");
    }

    #[test]
    fn test_punctuation_in_middle_on_trim_end_punctuation() {
        assert_eq!(trim_end_punctuation("hello, world!"), "hello, world");
    }

    #[test]
    fn test_only_punctuation_on_trim_end_punctuation() {
        assert_eq!(trim_end_punctuation("!!!"), "");
    }

    #[test]
    fn test_mixed_characters_on_trim_end_punctuation() {
        assert_eq!(trim_end_punctuation("abc.def!ghi?"), "abc.def!ghi");
    }

    #[test]
    fn test_unicode_punctuation_on_trim_end_punctuation() {
        assert_eq!(trim_end_punctuation("你好，世界!"), "你好，世界");
    }

    #[test]
    fn test_leading_and_trailing_punctuation_on_trim_end_punctuation() {
        assert_eq!(trim_end_punctuation("!hello, world!"), "!hello, world");
    }
}
