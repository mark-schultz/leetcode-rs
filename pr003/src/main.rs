use std::collections::HashSet;

/// Easy quadratic-time solution
///     - From each (of the O(n)) starting points, compute longest substring starting there
///     - Easily O(n) time using hashing
/// Linear time sol --- sliding window
///     - Dyn prog where T[i] = longest unique substring ending at ith idx.
///     - Each element occuring in substring w/ len stored in T[i] has unique chars,
///         can be stored with a HashSet<u8> rather than HashMap<u8>.
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max_len = 0;
    let mut seen_chars: HashSet<u8> = HashSet::new();
    let mut lower_idx = 0;
    let byte_str = s.into_bytes();
    for upper_idx in 0..byte_str.len() {
        let new_byte = byte_str[upper_idx];
        if seen_chars.remove(&new_byte) {
            while byte_str[lower_idx] != new_byte {
                // Brings lower_idx up to last occurance of new_byte
                seen_chars.remove(&byte_str[lower_idx]);
                lower_idx += 1;
            }
            // Makes the lower_idx right after the last occurance of new_byte
            lower_idx += 1;
        }
        seen_chars.insert(new_byte);
        max_len = i32::max(max_len, seen_chars.len() as i32);
    }
    max_len
}

fn main() {}

#[cfg(test)]
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
    #[test]
    fn test_tst_case_1() {
        let s = String::from("tmmzuxt");
        let len = 5;
        run_test(s, len);
    }
}
