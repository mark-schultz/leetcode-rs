use std::str;

fn main() {}

pub fn longest_palindrome(s: String) -> String {
    let s = s.as_bytes();
    // Centered at single char
    let mut max_j_single = 0;
    let mut max_c_single = 0;
    for c in 0..s.len() {
        let mut j = 0;
        while c >= j && c + j < s.len() {
            if s[c - j] == s[c + j] {
                if j >= max_j_single {
                    max_j_single = j;
                    max_c_single = c;
                }
            } else {
                break;
            }
            j += 1;
        }
    }
    // Centered at two char
    let mut max_j_double = 0;
    let mut max_c_double = 0;
    for c in 0..(s.len() - 1) {
        let mut j = 0;
        while c >= j && c + j + 1 < s.len() {
            if s[c - j] == s[c + j + 1] {
                if j >= max_j_double {
                    max_j_double = j;
                    max_c_double = c;
                }
            } else {
                break;
            }
            j += 1;
        }
    }
    if 1 + 2 * max_j_single > 2 * max_j_double + 2 {
        let (c, j) = (max_c_single, max_j_single);
        str::from_utf8(&s[c - j..=c + j]).unwrap().to_string()
    } else {
        let (c, j) = (max_c_double, max_j_double);
        str::from_utf8(&s[c - j..=c + j + 1]).unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ex_1() {
        let s = String::from("babad");
        let output = String::from("aba");
        assert_eq!(output, longest_palindrome(s));
    }
    #[test]
    fn test_ex_2() {
        let s = String::from("cbbd");
        let output = String::from("bb");
        assert_eq!(output, longest_palindrome(s));
    }
    #[test]
    fn test_3() {
        let s = String::from("aaabcdef");
        let output = String::from("aaa");
        assert_eq!(output, longest_palindrome(s));
    }
    #[test]
    fn test_4() {
        let s = String::from("abcdefgaaaa");
        let output = String::from("aaaa");
        assert_eq!(output, longest_palindrome(s));
    }
}
