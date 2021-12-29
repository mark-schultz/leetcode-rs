fn reverse_helper(x: i32) -> Option<i32> {
    let mut output: i32 = 0;
    let sign = x.signum();
    let mut x = x.checked_abs()?;
    while x > 0 {
        output = output.checked_mul(10)?;
        output = output.checked_add(x % 10)?;
        x = x.checked_div(10)?;
    }
    output.checked_mul(sign)
}

pub fn reverse(x: i32) -> i32 {
    if let Some(y) = reverse_helper(x) {
        y
    } else {
        0
    }
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
        assert_eq!(5, reverse(5));
    }
    #[test]
    fn test_5() {
        assert_eq!(0, reverse(1534236469));
    }
}
