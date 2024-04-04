
#[cfg(test)]
mod test_trim {
    use verify_string::{INVISIBILITIES, left_trim, right_trim, trim};

    #[test]
    pub fn test_left_trim() {
        // space is invisible character
        let name = " abcd ";
        assert_eq!("abcd ", left_trim(name, INVISIBILITIES));

        // invisible unicode in random order
        let name = "  	abcd ";
        assert_eq!("abcd ", left_trim(name, INVISIBILITIES));
    }

    #[test]
    pub fn test_right_trim() {
        // space is invisible character
        let name = " abcd ";
        assert_eq!(" abcd", right_trim(name, INVISIBILITIES));

        // invisible unicode in random order
        let name = " abcd   	";
        assert_eq!(" abcd", right_trim(name, INVISIBILITIES));
    }

    #[test]
    pub fn test_trim() {
        // space is invisible character
        let name = " abcd ";
        assert_eq!("abcd", trim(name, INVISIBILITIES));

        // invisible unicode in random order
        let name = "  	abcd   	";
        assert_eq!("abcd", trim(name, INVISIBILITIES));
    }

}