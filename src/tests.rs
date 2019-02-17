#[cfg(test)]
mod tests {
    use crate::Haystack;

    #[test]
    /// Calls contains_needle on a str, using a binary literal, The haystack contains the needle.
    fn contains_needle_on_str() {
        let res = "The quick brown fox jumps over the lazy dog".contains_needle(b"fox");
        assert_eq!(res, true);
    }

    #[test]
    /// Calls contains_needle on a str, using a binary literal, The haystack does not contain the needle.
    fn doesnt_contains_needle_on_str() {
        let res = "The quick brown fox jumps over the lazy dog".contains_needle(b"cat");
        assert_eq!(res, false);
    }
	
	#[test]
    /// Calls contains_needle on a String, using a binary literal, The haystack contains the needle.
    fn contains_needle_on_string() {
        let res = "The quick brown fox jumps over the lazy dog".to_string().contains_needle(b"fox");
        assert_eq!(res, true);
    }
	
	#[test]
    /// Calls contains_needle on a String, using a binary literal, The haystack does not contain the needle.
    fn doesnt_contains_needle_on_string() {
        let res = "The quick brown fox jumps over the lazy dog".to_string().contains_needle(b"cat");
        assert_eq!(res, false);
    }

    #[test]
	/// Calls contains_needle on a slice of bytes ([u8]), using a binary literal, The haystack contains the needle.
    fn bytes() {
        let res = b"DEADBEEF".contains_needle(b"BEEF");
        assert_eq!(res, true);
    }
}
