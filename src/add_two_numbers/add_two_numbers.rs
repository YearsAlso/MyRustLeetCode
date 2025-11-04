/*
给你两个 非空 的链表，表示两个非负的整数。它们每位数字都是按照 逆序 的方式存储的，并且每个节点只能存储 一位 数字。

请你将两个数相加，并以相同形式返回一个表示和的链表。

你可以假设除了数字 0 之外，这两个数都不会以 0 开头。
*/
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>,mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry > 0 {
            let mut sum = carry;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next;
            }
            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next;
            }
            carry = sum / 10;
            tail.next = Some(Box::new(ListNode::new(sum % 10)));
            tail = tail.next.as_mut().unwrap();
        }

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &v in vec.iter().rev() {
            let mut node = Box::new(ListNode::new(v));
            node.next = current;
            current = Some(node);
        }
        current
    }

    fn from_list(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut node = list;
        while let Some(n) = node {
            result.push(n.val);
            node = n.next;
        }
        result
    }

    #[test]
    fn test_example_1() {
        let l1 = to_list(vec![2,4,3]);
        let l2 = to_list(vec![5,6,4]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(from_list(result), vec![7,0,8]);
    }

    #[test]
    fn test_carry() {
        let l1 = to_list(vec![9,9,9]);
        let l2 = to_list(vec![1]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(from_list(result), vec![0,0,0,1]);
    }

    #[test]
    fn test_diff_length() {
        let l1 = to_list(vec![2,4]);
        let l2 = to_list(vec![5,6,4]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(from_list(result), vec![7,0,5]);
    }

    #[test]
    fn test_all_zero() {
        let l1 = to_list(vec![0]);
        let l2 = to_list(vec![0]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(from_list(result), vec![0]);
    }
}