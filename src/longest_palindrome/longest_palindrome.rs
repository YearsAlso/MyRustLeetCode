pub struct Solution;

// 给你一个字符串 s，找到 s 中最长的 回文 子串。
//
//
//
// 示例 1：
//
// 输入：s = "babad"
// 输出："bab"
// 解释："aba" 同样是符合题意的答案。
// 示例 2：
//
// 输入：s = "cbbd"
// 输出："bb"
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        // TODO: 建立字符和索引的映射关系
        let s = s.chars().collect::<Vec<_>>();
        let n = s.len();
        let mut dp = vec![vec![false; n]; n];
        let mut max_len = 0;
        let mut start = 0;
        for i in (0..n).rev() {
            for j in i..n {
                dp[i][j] = s[i] == s[j] && (j - i < 3 || dp[i + 1][j - 1]);
                if dp[i][j] && j - i + 1 > max_len {
                    max_len = j - i + 1;
                    start = i;
                }
            }
        }

        println!("{}", s[start..start + max_len].iter().collect::<String>());

        let result: String = s[start..start + max_len].iter().collect();
        println!("{}", result);

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab".to_string());
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb".to_string());
    }
}