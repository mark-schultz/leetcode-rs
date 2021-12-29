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

/// Median of the (sorted) array [nums, elem] in O(log n) time
/// Assume that nums is sorted
fn median_sorted_array_single_elem(nums: &[i32], elem: i32) -> f64 {
    let idx = match nums.binary_search(&elem) {
        Ok(idx) => idx,
        Err(idx) => idx,
    };
    // If we didn't care about efficiency, would just be a (sorted) array
    // of nums + the other elem. That would take O(n) time to construst.
    // This is O(1) to access.
    let fake_nums = |i: usize| match i.cmp(&idx) {
        Ordering::Less => nums[i],
        Ordering::Equal => elem,
        Ordering::Greater => nums[i - 1],
    };
    let n = nums.len();
    let total_len = n + 1;
    if total_len % 2 == 1 {
        fake_nums((total_len - 1) / 2) as f64
    } else {
        (fake_nums((total_len - 2) / 2) + fake_nums((total_len) / 2)) as f64 / 2.
    }
}
fn median_sorted_array_two_elems(nums: &[i32], elem1: i32, elem2: i32) -> f64 {
    let (elem1, elem2) = (i32::min(elem1, elem2), i32::max(elem1, elem2));
    let idx1 = match nums.binary_search(&elem1) {
        Ok(idx) => idx,
        Err(idx) => idx,
    };
    let idx2 = match nums.binary_search(&elem2) {
        Ok(idx) => idx,
        Err(idx) => idx,
    };
    let fake_nums = |i: usize| match (i.cmp(&idx1), i.cmp(&idx2)) {
        (Ordering::Less, Ordering::Less) => nums[i],
        (Ordering::Equal, _) => elem1,
        (Ordering::Greater, Ordering::Less) => nums[i - 1],
        (Ordering::Greater, Ordering::Equal) => elem2,
        (Ordering::Greater, Ordering::Greater) => nums[i - 2],
        _ => unreachable!("elem1 <= elem2"),
    };
    let n = nums.len();
    let total_len = n + 2;
    if total_len % 2 == 1 {
        fake_nums((total_len - 1) / 2) as f64
    } else {
        (fake_nums((total_len - 2) / 2) + fake_nums((total_len) / 2)) as f64 / 2.
    }
}

pub fn find_median_sorted_arrays(nums1: &[i32], nums2: &[i32]) -> f64 {
    let (nums1_size, nums2_size) = (nums1.len(), nums2.len());
    match (nums1_size < 3, nums2_size < 3) {
        (true, true) => {
            // O(1) size instance
            let mut concat = [nums1, nums2].concat();
            concat.sort();
            median_sorted_array(&concat)
        }
        (false, true) => find_median_sorted_arrays(nums2, nums1), // Reduce to case below
        (true, false) => match nums1_size {
            0 => median_sorted_array(nums2),
            1 => median_sorted_array_single_elem(nums2, nums1[0]),
            2 => median_sorted_array_two_elems(nums2, nums1[0], nums1[1]),
            _ => unreachable!("nums1_size < 2"),
        },
        (false, false) => {
            let (nums1_l, nums1_r) = nums1.split_at(nums1_size / 2);
            let (nums2_l, nums2_r) = nums2.split_at(nums2_size / 2);
            if nums1_l.last().unwrap() < nums2_l.last().unwrap() {
                // nums1's left half contained in nums2's left half
                // reduce search to nums1's right half and nums2's left half
                find_median_sorted_arrays(nums1_r, nums2_l)
            } else {
                // reduce search to nums1's left half and nums2's right half
                find_median_sorted_arrays(nums1_l, nums2_r)
            }
        }
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    fn slow_find_median_sorted_arrays(nums1: &[i32], nums2: &[i32]) -> f64 {
        let mut big_arr = [nums1, nums2].concat();
        big_arr.sort();
        median_sorted_array(&big_arr)
    }

    fn test_io(l1: Vec<i32>, l2: Vec<i32>) {
        let med1 = slow_find_median_sorted_arrays(&l1, &l2);
        let med2 = find_median_sorted_arrays(&l1, &l2);
        assert_eq!(med1, med2);
    }
    #[test]
    fn test_ex_1() {
        let l1 = vec![1, 3];
        let l2 = vec![2];
        test_io(l1, l2);
    }
    #[test]
    fn test_ex_2() {
        let l1 = vec![1, 2];
        let l2 = vec![3, 4];
        test_io(l1, l2);
    }
    #[test]
    fn test_custom_1() {
        let l1 = vec![1, 1, 1, 1];
        let l2 = vec![3, 3];
        test_io(l1, l2);
    }
    #[test]
    fn test_case_1() {
        let l1 = vec![1, 2];
        let l2 = vec![-1, 3];
        test_io(l1, l2);
    }
    #[test]
    fn test_median_sorted() {
        assert_eq!(median_sorted_array(&[1, 2, 3]), 2.);
        assert_eq!(median_sorted_array(&[1, 2, 3, 4]), (2. + 3.) / 2.);
    }
    #[test]
    fn test_median_sorted_single_elem_even() {
        //! Even total len
        assert_eq!(
            median_sorted_array_single_elem(&[2, 3, 4], 1),
            (2. + 3.) / 2.
        );
        assert_eq!(
            median_sorted_array_single_elem(&[1, 2, 3], 4),
            (2. + 3.) / 2.
        );
        assert_eq!(
            median_sorted_array_single_elem(&[1, 2, 4], 3),
            (2. + 3.) / 2.
        );
    }
    #[test]
    fn test_median_sorted_single_elem_odd() {
        //! Odd total len
        assert_eq!(median_sorted_array_single_elem(&[1, 2, 3, 4], 5), 3.);
        assert_eq!(median_sorted_array_single_elem(&[1, 3, 4, 5], 2), 3.);
        assert_eq!(median_sorted_array_single_elem(&[2, 3, 4, 5], 1), 3.);
    }
}
