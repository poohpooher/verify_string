/// Trim the given characters from the left side of the string.
pub fn left_trim<'a>(name: &'a str, chars: &[char]) -> &'a str {
    name.trim_start_matches(chars)
}

/// Trim the given characters from the right side of the string.
pub fn right_trim<'a>(name: &'a str, chars: &[char]) -> &'a str {
    name.trim_end_matches(chars)
}

/// Trim the given characters from both sides of the string.
pub fn trim<'a>(name: &'a str, chars: &[char]) -> &'a str {
    name.trim_matches(chars)
}