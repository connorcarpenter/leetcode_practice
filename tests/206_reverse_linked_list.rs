// 206. Reverse Linked List
// From: https://leetcode.com/problems/reverse-linked-list
//
// Given the head of a singly linked list, reverse the list, and return the reversed list.

// Example 1:
//
// Input: head = [1,2,3,4,5]
// Output: [5,4,3,2,1]
//
// Example 2:
//
// Input: head = [1,2]
// Output: [2,1]
//
// Example 3:
//
// Input: head = []
// Output: []
//
// Constraints:
//
// The number of nodes in the list is the range [0, 5000].
// -5000 <= Node.val <= 5000
//
// Follow up: A linked list can be reversed either iteratively or recursively. Could you implement both?
//
// Solved in 42 minutes, 51 seconds (test cases and helper functions for them took 20 minutes alone ..)

#[test]
fn official_example_1() {
    // Input: head = [1,2,3,4,5]
    // Output: [5,4,3,2,1]
    let input_values = vec![1,2,3,4,5];

    let head = new_list(input_values);

    let output_head = Solution::reverse_list(head);

    let output_values = vec![5,4,3,2,1];
    eval_values(output_head, output_values);
}

#[test]
fn official_example_2() {
    // Input: head = [1,2]
    // Output: [2,1]
    let input_values = vec![1,2];

    let head = new_list(input_values);

    let output_head = Solution::reverse_list(head);

    let output_values = vec![2,1];
    eval_values(output_head, output_values);
}

#[test]
fn official_example_3() {
    // Input: head = []
    // Output: []
    let input_values = vec![];

    let head = new_list(input_values);

    let output_head = Solution::reverse_list(head);

    let output_values = vec![];
    eval_values(output_head, output_values);
}

fn new_list(input_values: Vec<i32>) -> Option<Box<ListNode>> {
    if input_values.is_empty() {
        return None;
    }

    let last_index = input_values.len() - 1;
    let mut head = Box::new(ListNode::new(0));
    let mut cursor = &mut head;

    for (index, value) in input_values.iter().enumerate() {
        cursor.val = *value;

        if index == last_index {
            break;
        }

        cursor.next = Some(Box::new(ListNode::new(0)));
        cursor = cursor.next.as_mut().unwrap();
    }

    Some(head)
}

fn eval_values(mut output_head: Option<Box<ListNode>>, output_values: Vec<i32>) {
    for output_value in output_values {
        assert!(output_head.is_some());
        let output_head_node = output_head.as_mut().unwrap();
        assert_eq!(output_head_node.val, output_value);
        output_head = output_head_node.next.take();
    }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    Self {
      next: None,
      val
    }
  }
}

struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let Some(mut head) = head else {
            return None;
        };

        let mut stack = Vec::new();
        loop {
            if head.next.is_none() {
                stack.push(head);
                break;
            }
            let mut child = head.next.take().unwrap();
            std::mem::swap(&mut head, &mut child);
            stack.push(child);
        }

        let mut head = stack.pop().unwrap();
        let mut cursor = &mut head;
        loop {
            if stack.is_empty() {
                return Some(head);
            }

            let next = stack.pop().unwrap();
            cursor.next = Some(next);
            cursor = cursor.next.as_mut().unwrap();
        }
    }
}