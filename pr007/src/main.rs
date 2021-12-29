pub fn reverse(x: i32) -> i32 {
    let mut output = 0;
    let sign = x.signum();
    let mut x = x.abs();
    while x > 0 {
        output *= 10;
        output += x % 10;
        x /= 10;
    }
    output * sign
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ex_1() {
        assert_eq!(321, reverse(123));
    }
    #[test]
    fn test_ex_2() {
        assert_eq!(-123, reverse(-321));
    }
    #[test]
    fn test_ex_3() {
        assert_eq!(21, reverse(120));
    }
    #[test]
    fn test_4() {
        assert_eq!(5, reverse(5))
    }
}
