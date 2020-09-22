//!
#![warn(missing_debug_implementations, rust_2018_idioms)]

#[derive(Debug)]
pub struct StrSplit<'r, 'd> {
    remainder: Option<&'r str>,
    delimiter: &'d str,
}

// str -> [char]
// &str -> &[char]
// String -> Vec<char>
//
// String -> &str  trival (cheap -- AsRef)
// &str -> String doing heap alloc and copy things over (expensive -- memmove)

impl<'r, 'd> StrSplit<'r, 'd> {
    pub fn new(haystack: &'r str, delimiter: &'d str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'r> Iterator for StrSplit<'r, '_> {
    type Item = &'r str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some(next_delim) = remainder.find(self.delimiter) {
            let until_delimiter = &remainder[..next_delim];
            *remainder = &remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
    }
}

fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, &format! {"{}", c})
        .next()
        .expect("StrSplit always gives at lease one result")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let haystack = "a b c d e";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn tail() {
        let haystack = "a b c d ";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", ""])
    }

    #[test]
    fn until_char_test() {
        assert_eq!(until_char("hello world", 'o'), "hell");
    }
}
