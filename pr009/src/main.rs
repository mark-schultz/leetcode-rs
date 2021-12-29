fn main() {}

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        false
    } else {
        let mut rev = 0;
        let mut x_copy = x.clone();
        while x_copy > 0 {
            rev *= 10;
            rev += x_copy % 10;
            x_copy /= 10
        }
        x == rev
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ex_1() {
        assert_eq!(is_palindrome(121), true);
    }
    #[test]
    fn test_ex_2() {
        assert_eq!(is_palindrome(-121), false);
    }
    #[test]
    fn test_ex_3() {
        assert_eq!(is_palindrome(10), false);
    }
}
