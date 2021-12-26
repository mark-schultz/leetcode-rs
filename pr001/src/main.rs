fn main() {
    println!("Hello, world!");
}

/// O(n^2) trivial solution
pub fn two_sum_quadratic(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    panic!("Invalid Two Sum Instance")
}

/// O(n log n) solution
/// 1. Sort
/// 2. Single scan (from both ends simultaneously) through array to find right pair of indices
///
/// Need to replace i32's with (usize, i32) to track initial indicies of the elements after sorting.

pub fn two_sum_quasilinear(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let len = nums.len();
    let mut zipped: Vec<(usize, i32)> = nums.into_iter().enumerate().take(len).collect();
    zipped.sort_by(|(_, b), (_, d)| b.cmp(d));
    let mut upper = len - 1;
    for lower in 0..len {
        while zipped[lower].1 + zipped[upper].1 > target {
            upper -= 1;
        }
        if zipped[lower].1 + zipped[upper].1 == target {
            return vec![zipped[lower].0 as i32, zipped[upper].0 as i32];
        }
    }
    panic!("Invalid Two Sum Instance")
}

use std::collections::HashMap;

/// O(n) solution (hashing)
pub fn two_sum_linear(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash = HashMap::new();
    for (idx, val) in nums.iter().enumerate() {
        hash.insert(target - val, idx as i32);
    }
    for (idx, val) in nums.iter().enumerate() {
        if let Some(other_idx) = hash.get(val) {
            if (idx as i32) != *other_idx {
                return vec![idx as i32, *other_idx];
            }
        }
    }
    panic!("Invalid Two Sum Instance")
}

mod tests {
    use super::*;
    fn run_test_quadratic(nums: Vec<i32>, target: i32, output: Vec<i32>) {
        assert_eq!(two_sum_quadratic(nums, target), output);
    }
    fn run_test_quasilinear(nums: Vec<i32>, target: i32, output: Vec<i32>) {
        assert_eq!(two_sum_quasilinear(nums, target), output);
    }
    fn run_test_linear(nums: Vec<i32>, target: i32, output: Vec<i32>) {
        assert_eq!(two_sum_linear(nums, target), output);
    }

    #[test]
    fn test_quadratic() {
        let nums1 = vec![2, 7, 11, 15];
        let target1 = 9;
        let output1 = vec![0, 1];
        let nums2 = vec![3, 2, 4];
        let target2 = 6;
        let output2 = vec![1, 2];
        let nums3 = vec![3, 3];
        let target3 = 6;
        let output3 = vec![0, 1];
        run_test_quadratic(nums1, target1, output1);
        run_test_quadratic(nums2, target2, output2);
        run_test_quadratic(nums3, target3, output3);
    }

    #[test]
    fn test_quasilinear() {
        let nums1 = vec![2, 7, 11, 15];
        let target1 = 9;
        let output1 = vec![0, 1];
        let nums2 = vec![3, 2, 4];
        let target2 = 6;
        let output2 = vec![1, 2];
        let nums3 = vec![3, 3];
        let target3 = 6;
        let output3 = vec![0, 1];
        run_test_quasilinear(nums1, target1, output1);
        run_test_quasilinear(nums2, target2, output2);
        run_test_quasilinear(nums3, target3, output3);
    }

    #[test]
    fn test_linear() {
        let nums1 = vec![2, 7, 11, 15];
        let target1 = 9;
        let output1 = vec![0, 1];
        let nums2 = vec![3, 2, 4];
        let target2 = 6;
        let output2 = vec![1, 2];
        let nums3 = vec![3, 3];
        let target3 = 6;
        let output3 = vec![0, 1];
        run_test_linear(nums1, target1, output1);
        run_test_linear(nums2, target2, output2);
        run_test_linear(nums3, target3, output3);
    }
}
