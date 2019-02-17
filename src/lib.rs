mod tests;

/// The Haystack trait is the 'target' of the KMP algorithm provided by this library.
/// It provides the pattern_table method (part of the KMP algorithm) and the contains_needle method.
/// Haystack is implemented on all types that can be converted to a &[u8], such as Byte slices, str and Strings.
pub trait Haystack {
	/// Produce a 'pattern table' for use with the Knuth Morris Pratt algorithm
    fn pattern_table(needle: &[u8]) -> Vec<usize> {
        let mut i = 0;
        let mut j = 1;
        let mut arr = vec![0; needle.len()];
        while j < needle.len() {
            if needle[i] == needle[j] {
                i += 1;
                arr[j] = i;
                j += 1;
            } else {
                if i != 0 {
                    i = arr[i - 1];
                } else {
                    arr[j] = i;
                    j += 1;
                }
            }
        }
        arr
    }

    /// Returns true if this Haystack contains needle.
    fn contains_needle<N: AsRef<[u8]>>(&self, needle: N) -> bool;
}

impl<H: AsRef<[u8]>> Haystack for H {
    fn contains_needle<N: AsRef<[u8]>>(&self, needle: N) -> bool {
        let needle = needle.as_ref();
        let pattern_table = Self::pattern_table(needle);
        let haystack = &self.as_ref();

        let mut haystack_c = 0usize;
        let mut needle_c = 0usize;

        let haystack_len = haystack.len();
        let needle_len = needle.len();

        while haystack_c < haystack_len {
            if haystack[haystack_c] == needle[needle_c] {
                haystack_c += 1;
                needle_c += 1;
            }
            if needle_c == needle_len {
                return true;
            } else {
                if haystack_c < haystack_len && haystack[haystack_c] != needle[needle_c] {
                    if needle_c != 0 {
                        needle_c = pattern_table[needle_c - 1];
                    } else {
                        haystack_c += 1;
                    }
                }
            }
        }
        false
    }
}
