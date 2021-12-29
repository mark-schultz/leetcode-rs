use std::cmp::Ordering;
use std::str;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum CenterType {
    Single,
    Double,
}

#[derive(Copy, Clone, Debug)]
struct SubStr<'a> {
    c: usize,
    j: usize,
    s: &'a str,
    center: CenterType,
}

impl<'a> SubStr<'a> {
    fn try_new(s: &'a str, c: usize, j: usize, center: CenterType) -> Option<SubStr<'a>> {
        let b = match center {
            CenterType::Single => 0,
            CenterType::Double => 1,
        };
        if c >= j && c + j + b < s.len() {
            Some(SubStr { c, j, s, center })
        } else {
            None
        }
    }
    fn len(self) -> usize {
        2 * self.j + {
            match self.center {
                CenterType::Single => 1,
                CenterType::Double => 2,
            }
        }
    }
}

impl<'a> PartialEq for SubStr<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center && self.j == other.j && self.s == other.s
    }
}

impl<'a> PartialOrd for SubStr<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.s != other.s {
            None
        } else {
            self.len().partial_cmp(&other.len())
        }
    }
}

impl<'a> ToString for SubStr<'a> {
    fn to_string(&self) -> String {
        let b = match self.center {
            CenterType::Single => 0,
            CenterType::Double => 1,
        };
        let left_idx = self.c - self.j;
        let right_idx = self.c + self.j + b;
        str::from_utf8(&self.s.as_bytes()[left_idx..=right_idx])
            .unwrap()
            .to_string()
    }
}

fn main() {}

pub fn longest_palindrome(s: String) -> String {
    // Centered at single char
    let mut max_substr = None;
    let s_bytes = s.as_bytes();
    for c in 0..s.len() {
        let mut j = 0;
        loop {
            let new_substr = SubStr::try_new(&s, c, j, CenterType::Single);
            if new_substr == None || s_bytes[c - j] != s_bytes[c + j] {
                break;
            } else if new_substr > max_substr {
                max_substr = new_substr;
            }
            j += 1;
        }
    }
    // Centered at two char
    for c in 0..s.len() {
        let mut j = 0;
        loop {
            let new_substr = SubStr::try_new(&s, c, j, CenterType::Double);
            if new_substr == None || s_bytes[c - j] != s_bytes[c + j + 1] {
                break;
            } else if new_substr > max_substr {
                max_substr = new_substr;
            }
            j += 1;
        }
    }
    max_substr.unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ex_1() {
        let s = String::from("babad");
        let output = String::from("bab");
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
        let output = String::from("a");
        assert_eq!(output, longest_palindrome(s));
    }
    #[test]
    fn test_7() {
        let s = String::from("tattarrattat");
        let output = String::from("tattarrattat");
        assert_eq!(output, longest_palindrome(s));
    }
}
