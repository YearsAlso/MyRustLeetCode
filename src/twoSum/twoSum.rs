
use std::iter::Map;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut sub_map = std::collections::HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        if let Some(&j) = sub_map.get(&(target - num)) {
            return vec![j as i32, i as i32];
        }
        sub_map.insert(num, i);
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);

        let nums = vec![3, 2, 4];
        let target = 6;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![1, 2]);

        let nums = vec![3, 3];
        let target = 6;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }
}

pub mod two_sum {
    pub mod two_sum;
}