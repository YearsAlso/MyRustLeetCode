pub struct Solution;

// 给定两个大小分别为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。请你找出并返回这两个正序数组的 中位数 。
//
// 算法的时间复杂度应该为 O(log (m+n)) 。
//
//
//
// 示例 1：
//
// 输入：nums1 = [1,3], nums2 = [2]
// 输出：2.00000
// 解释：合并数组 = [1,2,3] ，中位数 2
// 示例 2：
//
// 输入：nums1 = [1,2], nums2 = [3,4]
// 输出：2.50000
// 解释：合并数组 = [1,2,3,4] ，中位数 (2 + 3) / 2 = 2.5

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // TODO: 根据奇偶数，返回中位数
        let sum_len = nums1.len() + nums2.len();
        let mut new_arr = [nums1, nums2].concat();
        new_arr.sort();
        if sum_len % 2 == 0 {
            (new_arr[sum_len / 2 - 1] + new_arr[sum_len / 2]) as f64 / 2.0
        } else {
            new_arr[sum_len / 2] as f64
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
        2.00000
    );
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
        2.50000
    );
}