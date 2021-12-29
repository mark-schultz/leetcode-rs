use std::cmp::Ordering;

#[inline]
fn median_sorted_array(nums: &[i32]) -> f64 {
    let n = nums.len();
    if n % 2 == 1 {
        nums[(n - 1) / 2] as f64
    } else {
        (nums[(n - 2) / 2] + nums[n / 2]) as f64 / 2.
    }
}

#[inline]
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut nums = [nums1, nums2].concat();
    nums.sort();
    median_sorted_array(&nums)
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_ex_1() {
        let l1 = vec![1, 3];
        let l2 = vec![2];
        assert_eq!(find_median_sorted_arrays(l1, l2), 2.);
    }
    #[test]
    fn test_ex_2() {
        let l1 = vec![1, 2];
        let l2 = vec![3, 4];
        assert_eq!(find_median_sorted_arrays(l1, l2), 5. / 2.);
    }

    #[test]
    fn test_custom_1() {
        let l1 = vec![1, 1, 1, 1];
        let l2 = vec![3, 3];
        assert_eq!(find_median_sorted_arrays(l1, l2), 1.);
    }
    #[test]
    fn test_case_1() {
        let l1 = vec![1, 2];
        let l2 = vec![-1, 3];
        assert_eq!(find_median_sorted_arrays(l1, l2), 3. / 2.);
    }
}
