use anyhow::{anyhow, Error};
use derive_builder::Builder;
use regex::Regex;
use crate::Verifier;

#[derive(Debug, Clone, Builder)]
pub struct RegexVerifier {
    regex: Regex,
}

impl Verifier for RegexVerifier {
    fn verify(&self, name: &str) -> Result<(), Error> {
        if self.regex.is_match(name) {
            return Err(anyhow!("regex matched"));
        }

        Ok(())
    }
}

/// Create a regex that matches any of the characters in the provided list.
pub fn regex_some_char(chars: &[char]) -> Result<Regex, regex::Error> {
    let s = format!(r"[{}]", String::from_iter(chars));

    Regex::new(s.as_str())
}

/// Check if a character is present in a string, excluding the beginning and end
pub fn regex_not_start_end_char(chars: &[char]) -> Result<Regex, regex::Error> {
    let s = format!(r".[{}].", String::from_iter(chars));

    Regex::new(s.as_str())
}

/// Check if a character is present at the beginning or end of a string
pub fn regex_start_end_char(chars: &[char]) -> Result<Regex, regex::Error> {
    let c = String::from_iter(chars);
    let s = format!(r"^[{}].|.[{}]$", c, c);

    Regex::new(s.as_str())
}