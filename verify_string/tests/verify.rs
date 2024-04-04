#[cfg(test)]
mod test_verify {
    use verify_string::{
        regex_not_start_end_char, regex_some_char, regex_start_end_char, LengthVerifierBuilder,
        RegexVerifierBuilder, Verifier, VerifyString, WordVerifierBuilder, INVISIBILITIES,
    };

    #[test]
    pub fn builder_length() {
        let rule = LengthVerifierBuilder::default()
            .range(0..=10)
            .build()
            .expect("build rule");
        // Length is 10
        assert!(!rule.verify("1234567890").is_err());

        // Length is 11
        assert!(rule.verify("12345678901").is_err());

        // Length is 0
        assert!(!rule.verify("").is_err());
    }

    #[test]
    pub fn builder_word() {
        let rule = WordVerifierBuilder::default()
            .profanities(vec!["fuck".to_string(), "18".to_string()])
            .build()
            .expect("build rule");

        // No profanity
        assert!(!rule.verify("Mason").is_err());

        // profanity
        assert!(rule.verify("alkdjflksdfuckdslfjdsklfjl").is_err());

        // profanity
        assert!(rule.verify("aldklfnie17a18sdjfl").is_err());
    }

    #[test]
    pub fn builder_regex_some_char() {
        let regex = regex_some_char(INVISIBILITIES).expect("make regex failed");
        let rule = RegexVerifierBuilder::default()
            .regex(regex)
            .build()
            .expect("build rule");

        // No invisible characters
        assert!(!rule.verify("Mason").is_err());

        // Contains invisible characters(U+17B4)
        assert!(rule.verify("឴Mason").is_err());

        // Contains invisible characters(U+17B4)
        assert!(rule.verify("Ma឴son").is_err());

        // Contains invisible characters(U+17B4)
        assert!(rule.verify("Mason឴").is_err());

        // Contains invisible character(U+E01EE)
        assert!(rule.verify("Mas󠇮on").is_err());
    }

    #[test]
    pub fn builder_regex_not_start_end_char() {
        let regex = regex_not_start_end_char(INVISIBILITIES).expect("make regex failed");
        let rule = RegexVerifierBuilder::default()
            .regex(regex)
            .build()
            .expect("build rule");

        // No invisible characters
        assert!(!rule.verify("Mason").is_err());

        // Contains invisible characters start(U+E01E6)
        assert!(!rule.verify("󠇦Mason").is_err());

        // Contains invisible characters end(U+E01E6)
        assert!(!rule.verify("Mason󠇦").is_err());

        // Contains invisible characters not start_end (U+E01E6)
        assert!(rule.verify("Ma󠇦son").is_err());
    }

    #[test]
    pub fn builder_regex_start_end_char() {
        let regex = regex_start_end_char(INVISIBILITIES).expect("make regex failed");
        let rule = RegexVerifierBuilder::default()
            .regex(regex)
            .build()
            .expect("build rule");

        // No invisible characters
        assert!(!rule.verify("Mason").is_err());

        // contains invalid characters start(U+E01E6)
        assert!(rule.verify("󠇦Mason").is_err());

        // contains invalid characters end(U+E01E6)
        assert!(rule.verify("Mason󠇦").is_err());

        // contains invalid characters middle(U+E01E6)
        assert!(!rule.verify("Ma󠇦son").is_err());
    }
    #[test]
    pub fn verify_string() {
        let length = LengthVerifierBuilder::default()
            .range(0..=10)
            .build()
            .expect("build length rule");

        let start_end_regex = regex_start_end_char(&INVISIBILITIES).expect("make regex failed");
        let start_end = RegexVerifierBuilder::default()
            .regex(start_end_regex)
            .build()
            .expect("build invisible_unicode rule");

        let mut invisible_chars = INVISIBILITIES.to_vec();
        // Remove white space
        invisible_chars.retain(|&c| !c.is_whitespace());

        let not_start_end_regex =
            regex_not_start_end_char(&invisible_chars).expect("make regex failed");
        let not_start_end = RegexVerifierBuilder::default()
            .regex(not_start_end_regex)
            .build()
            .expect("build invisible_unicode rule");

        let profanity = WordVerifierBuilder::default()
            .profanities(vec!["fuck".to_string(), "18".to_string()])
            .build()
            .expect("build profanity rule");

        let mut verifier = VerifyString::default();
        verifier
            .with_verifier(length)
            .with_verifier(start_end)
            .with_verifier(not_start_end)
            .with_verifier(profanity);

        // valid string
        assert!(verifier.verify("Mason"));

        // valid string, middle space
        assert!(verifier.verify("Mas Mas"));

        // valid string, middle (U+00A0)
        assert!(verifier.verify("Mas Mas"));

        // valid string, middle (U+2002)
        assert!(verifier.verify("Mas Mas"));

        // invalid string(U+17B4)
        assert!(!verifier.verify("Mason឴"));

        // invalid string(profanity)
        assert!(!verifier.verify("18Mason"));

        // invalid string, start space
        assert!(!verifier.verify(" Mason"));

        // invalid string, end space
        assert!(!verifier.verify("Mason "));

        // invalid string, middle (U+115F)
        assert!(!verifier.verify("MasᅟMas"));
    }
}
