// disemvowel.rs

pub fn disemvowel(s: &str) -> String {
    s.chars()
        .filter(|c| !"aeiouAEIOU".contains(*c))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::disemvowel;

    #[test]
    fn hello_world() {
        let input = "Hello, world!";
        let expected = "Hll, wrld!";

        assert_eq!(expected, disemvowel(input));
    }

    #[test]
    fn empty() {
        assert_eq!("", disemvowel(""));
    }

    #[test]
    fn no_vowels() {
        assert_eq!("pqrst", disemvowel("pqrst"));
    }

    #[test]
    fn all_vowels() {
        assert_eq!("", disemvowel("aeiouAEIOUOIEAuoiea"));
    }

    #[test]
    fn morris_minnesota() {
        assert_eq!("Mrrs, Mnnst", disemvowel("Morris, Minnesota"));
    }
}