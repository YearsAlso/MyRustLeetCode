pub struct Solution;

/***
给定一个字符串 s ，请你找出其中不含有重复字符的 最长 子串 的长度。

示例 1:

输入: s = "abcabcbb"
输出: 3
解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
示例 2:

输入: s = "bbbbb"
输出: 1
解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
示例 3:

输入: s = "pwwkew"
输出: 3
解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
     请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。

 */

use std::collections::HashMap;
impl Solution {
    pub fn length_of_longest_substring_old(s: String) -> i32 {
        let current_string = s.into_bytes();
        let len = current_string.len();
        let mut start_window: i32 = 0;
        let mut stop_window: i32 = 0;
        let mut char_set: [i32; 256] = [-1; 256];
        let mut current_str_len: i32 = 0;
        let mut result: i32 = 0;
        if len == 0 {
            return 0;
        }
        for i in 0..len {
            // 相同字符素索引
            let same_char_index = char_set[current_string[i] as usize];
            // 如果索引位置大于0，则说明有重复字符
            if same_char_index > 0 {
                // 获取之前这个字符的索引位置，并且将之前的字符串索引位置和当前索引位置进行比较，如果当前索引位置小于之前索引位置，则更新当前索引位置为之前索引位置
                let history_substring_length = stop_window - start_window;
                // 如果上次的字符串长度大于结果，则更新结果
                if history_substring_length > result {
                    result = history_substring_length;
                }
                // 将上次重复的位置后一位取出
                let new_start_window: i32 = same_char_index + 1;
                let new_stop_window: i32 = i as i32;

                current_str_len = new_stop_window - new_start_window;

                start_window = new_start_window;

            }
            // 如果索引位置小于0，则说明没有重复字符
            else {
                current_str_len += 1;
                stop_window = i as i32;
            }

            char_set[current_string[i] as usize] = i as i32;
        }

        if current_str_len > result {
            result = current_str_len;
        }

        return result;
    }

    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_pos: HashMap<char, usize> = HashMap::new();
        let mut left: usize = 0;
        let mut max_len: usize = 0;

        for (i, ch) in s.chars().enumerate() {
            if let Some(&prev) = last_pos.get(&ch) {
                if prev >= left {
                    left = prev + 1;
                }
            }
            let curr_len = i - left + 1;
            if curr_len > max_len {
                max_len = curr_len;
            }
            last_pos.insert(ch, i);
        }

        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test case 1: Normal case with repeated characters
    /// Input: "abcabcbb"
    /// Expected output: 3 (substring "abc")
    fn test_normal_case_with_repetition() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
    }

    #[test]
    /// Test case 2: All same characters
    /// Input: "bbbbb"
    /// Expected output: 1 (substring "b")
    fn test_all_same_characters() {
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
    }

    #[test]
    /// Test case 3: Complex case with non-adjacent repetition
    /// Input: "pwwkew"
    /// Expected output: 3 (substring "wke")
    fn test_complex_case() {
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }

    #[test]
    /// Test case 4: Empty string
    /// Input: ""
    /// Expected output: 0
    fn test_empty_string() {
        assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
    }

    #[test]
    /// Test case 5: Single character
    /// Input: "a"
    /// Expected output: 1
    fn test_single_character() {
        assert_eq!(Solution::length_of_longest_substring("a".to_string()), 1);
    }

    #[test]
    /// Test case 6: No repeated characters
    /// Input: "abcdef"
    /// Expected output: 6 (entire string)
    fn test_no_repeated_characters() {
        assert_eq!(
            Solution::length_of_longest_substring("abcdef".to_string()),
            6
        );
    }

    #[test]
    /// Test case 7: Repetition at beginning and end
    /// Input: "aab"
    /// Expected output: 2 (substring "ab")
    fn test_repetition_at_boundaries() {
        assert_eq!(Solution::length_of_longest_substring("aab".to_string()), 2);
    }

    #[test]
    /// Test case 8: Chinese characters
    /// Input: "你好世界"
    /// Expected output: 4 (all different Chinese characters)
    fn test_chinese_characters() {
        assert_eq!(Solution::length_of_longest_substring("dvdf".to_string()), 3);
    }
}
