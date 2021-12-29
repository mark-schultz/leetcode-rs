/// Given string abcdefg, and number of rows:
///  1. convert to "zig zag" representation
///  2. stitch things back together row-wise
///
/// 1-row: 0123456
/// 2-row: 0 2 4 6 -> 0246 135
///        1 3 5
///
/// 3-row: 0   4   8
///        1 3 5 7 9  -> 048 13579 26
///        2   6
///
/// 4-row: 0     6     12
///        1   5 7   11
///        2 4   8 10    -> 06`12` 157`11` 248`10` 39
///        3     9
///
/// 5-row: 0       8           16
///        1     7 9        15 17
///        2   6   10    14    18 -> 08`16` 179`15``17` 26`10``14``18` 35`11``13`19
///        3 5     11 13       19
///        4       12          20
///
/// 6-row: 0         10             20
///        1       9 11          19
///        2     8   12       18
///        3   7     13    17
///        4 6       14 16
///        5         15

pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        s
    } else {
        let mut output = String::new();
        output.reserve(s.len());
        let byte_str = s.as_bytes();
        for curr_row in 0..num_rows {
            if curr_row == 0 || curr_row == num_rows - 1 {
                for i in [curr_row as usize]
                    .iter()
                    .cycle()
                    .enumerate()
                    .map(|(i, val)| val + 2 * i * (num_rows as usize - 1))
                {
                    if i >= s.len() {
                        break;
                    } else {
                        output.push(byte_str[i] as char);
                    }
                }
            } else {
                for i in [[curr_row as usize, (2 * (num_rows - 1) - curr_row) as usize]]
                    .iter()
                    .cycle()
                    .enumerate()
                    .flat_map(|(i, val)| val.map(|j| j + 2 * i * (num_rows - 1) as usize))
                {
                    if i >= s.len() {
                        break;
                    } else {
                        output.push(byte_str[i] as char);
                    }
                }
            }
        }
        output
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ex_1() {
        let s = String::from("PAYPALISHIRING");
        assert_eq!(convert(s, 3), String::from("PAHNAPLSIIGYIR"));
        // P   A   H   N
        // A P L S I I G -> PAHN APLSIIG YIR
        // Y   I   R
        //
        // PAHN AALLIIGG YIR"
    }
    #[test]
    fn test_ex_2() {
        let s = String::from("PAYPALISHIRING");
        assert_eq!(convert(s, 4), String::from("PINALSIGYAHRPI"));
    }
    #[test]
    fn test_ex_3() {
        let s = String::from("A");
        assert_eq!(convert(s, 1), String::from("A"));
    }
}
