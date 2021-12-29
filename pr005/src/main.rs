use std::str;

fn main() {}

pub fn longest_palindrome(s: String) -> String {
    let s = s.as_bytes();
    // Centered at single char
    let mut max_j_single = None;
    let mut max_c_single = None;
    for c in 0..s.len() {
        let mut j = 0;
        while c >= j && c + j < s.len() {
            if s[c - j] == s[c + j] {
                if let Some(max_j) = max_j_single {
                    if j >= max_j {
                        max_j_single = Some(j);
                        max_c_single = Some(c);
                    }
                } else {
                    max_j_single = Some(j);
                    max_c_single = Some(c);
                }
            } else {
                break;
            }
            j += 1;
        }
    }
    // Centered at two char
    let mut max_j_double = None;
    let mut max_c_double = None;
    for c in 0..(s.len() - 1) {
        let mut j = 0;
        while c >= j && c + j + 1 < s.len() {
            if s[c - j] == s[c + j + 1] {
                if let Some(max_j) = max_j_single {
                    if j >= max_j {
                        max_j_double = Some(j);
                        max_c_double = Some(c);
                    }
                } else {
                    max_j_double = Some(j);
                    max_c_double = Some(c);
                }
            } else {
                break;
            }
            j += 1;
        }
    }
    match (max_j_single, max_j_double) {
        (Some(i), Some(j)) => {
            if 1 + 2 * i > 2 + 2 * j {
                let c = max_c_single.unwrap();
                let j = i;
                str::from_utf8(&s[c - j..=c + j]).unwrap().to_string()
            } else {
                let c = max_c_double.unwrap();
                str::from_utf8(&s[c - j..=c + j + 1]).unwrap().to_string()
            }
        }
        (Some(i), None) => {
            let c = max_c_single.unwrap();
            let j = i;
            str::from_utf8(&s[c - j..=c + j]).unwrap().to_string()
        }
        (None, Some(j)) => {
            let c = max_c_double.unwrap();
            str::from_utf8(&s[c - j..=c + j + 1]).unwrap().to_string()
        }
        (None, None) => unreachable!(),
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
    #[test]
    fn test_5() {
        let s = String::from("a");
        let output = String::from("a");
        assert_eq!(output, longest_palindrome(s));
    }
    #[test]
    fn test_6() {
        let s = String::from("ac");
        let output = String::from("c");
        assert_eq!(output, longest_palindrome(s));
    }
}
