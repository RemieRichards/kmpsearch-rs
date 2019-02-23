#[cfg(test)]
mod tests {
    use crate::Haystack;

    #[test]
    /// Calls contains_needle on a str, using a str literal, The haystack contains the needle.
    fn contains_needle_tests() {
        //on str - match
        let res = "The quick brown fox jumps over the lazy dog".contains_needle("fox");
        assert_eq!(res, true);

        //on str - no match
        let res = "The quick brown fox jumps over the lazy dog".contains_needle("cat");
        assert_eq!(res, false);

        //on String - match
        let res = "The quick brown fox jumps over the lazy dog"
            .to_string()
            .contains_needle("fox");
        assert_eq!(res, true);

        //on String - no match
        let res = "The quick brown fox jumps over the lazy dog"
            .to_string()
            .contains_needle("cat");
        assert_eq!(res, false);

        //on bytes - match
        let res = b"DEADBEEF".contains_needle(b"BEEF");
        assert_eq!(res, true);
    }

    #[test]
    fn index_tests() {
        //First index of needle, where the entire haystack is the needle
        let res = "fox".first_indexof_needle("fox");
        assert_eq!(res.unwrap(), 0);

        //First index of needle, where the needle is in the haystack
        let res = "The quick brown fox jumps over the lazy dog".first_indexof_needle("fox");
        assert_eq!(res.unwrap(), 16);

        //Last index of needle, where the entire haystack is the needle
        let res = "fox".last_indexof_needle("fox");
        assert_eq!(res.unwrap(), 0);

        //Last index of needle, where the needle is in the haystack twice
        let res = "That fox is a cool fox, he knows magic".last_indexof_needle("fox");
        assert_eq!(res.unwrap(), 19);

        //All indexes of needle in haystack
        let res = "That fox is a cool fox, he knows magic".indexesof_needle("fox");
        assert_eq!(res.unwrap(), vec![5, 19]);

        //All indexes of needle in haystack, but haystack doesn't contain needle
        let res = "There are no orange fluffy animals here".indexesof_needle("fox");
        assert_eq!(res, None);
    }
}
