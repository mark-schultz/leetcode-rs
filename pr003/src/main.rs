use std::collections::HashMap;

/// Easy quadratic-time solution
///     - From each (of the O(n)) starting points, compute longest substring starting there
///     - Easily O(n) time using hashing
/// Linear time sol --- sliding window
///     - Dyn prog where T[i] = longest unique substring ending at ith idx.
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max_len = 0;
    let mut char_counts: HashMap<u8, u8> = HashMap::new();
    let mut lower_idx = 0;
    let byte_str = s.into_bytes();
    for upper_idx in 0..byte_str.len() {
        let new_byte = byte_str[upper_idx];
        if let Some(mut count) = char_counts.remove(&new_byte) {
            // Raise lower index until count is zero
            while count > 0 {
                if byte_str[lower_idx] == new_byte {
                    count -= 1;
                }
                lower_idx += 1;
            }
        }
        char_counts.insert(new_byte, 1);
        max_len = i32::max(max_len, (upper_idx - lower_idx + 1) as i32);
    }
    max_len
}

fn main() {}

mod tests {
    use super::*;
    fn run_test(string: String, length: i32) {
        assert_eq!(length_of_longest_substring(string), length);
    }
    #[test]
    fn test_ex_1() {
        let s = String::from("abcabcbb");
        let len = 3;
        run_test(s, len);
    }
    #[test]
    fn test_ex_2() {
        let s = String::from("bbbbb");
        let len = 1;
        run_test(s, len);
    }
    #[test]
    fn test_ex_3() {
        let s = String::from("pwwkew");
        let len = 3;
        run_test(s, len);
    }
}
