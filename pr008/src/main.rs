/// atoi, clamp result to be in i32 range

pub fn my_atoi(s: String) -> i32 {
    let mut chars = s.trim_start().chars();
    let mut sign = 1;
    let mut output: i32 = 0;
    match chars.next() {
        Some(d) => match d {
            '+' => {}
            '-' => sign = -1,
            i @ _ => {
                if let Some(x) = i.to_digit(10) {
                    output += x as i32;
                } else {
                    return 0;
                }
            }
        },
        None => return 0,
    }
    for chr in chars {
        if let Some(x) = chr.to_digit(10) {
            output = output.saturating_mul(10);
            output = output.saturating_add(sign * x as i32);
        } else {
            break;
        }
    }
    output
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ex_1() {
        assert_eq!(42, my_atoi(String::from("42")));
    }
    #[test]
    fn test_ex_2() {
        assert_eq!(-42, my_atoi(String::from("   -42")));
    }
    #[test]
    fn test_ex_3() {
        assert_eq!(4193, my_atoi(String::from("4193 with words")));
    }
    #[test]
    fn test_4() {
        assert_eq!(-2147483648, my_atoi(String::from("-91283472332")));
    }
}
