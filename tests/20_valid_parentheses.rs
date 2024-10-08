// 20. Valid Parentheses
// From: https://leetcode.com/problems/valid-parentheses
//
// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
//
// An input string is valid if:
//
// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.
// Every close bracket has a corresponding open bracket of the same type.
//
// Example 1:
//
// Input: s = "()"
//
// Output: true
//
// Example 2:
//
// Input: s = "()[]{}"
//
// Output: true
//
// Example 3:
//
// Input: s = "(]"
//
// Output: false
//
// Example 4:
//
// Input: s = "([])"
//
// Output: true
//
// Constraints:
//
// 1 <= s.length <= 104
// s consists of parentheses only '()[]{}'.
//
// Solved in 17 minutes, 44 seconds

#[test]
fn official_example_1() {
    let input = "()".to_string();
    let output = Solution::is_valid(input);
    assert_eq!(output, true);
}

#[test]
fn official_example_2() {
    let input = "()[]{}".to_string();
    let output = Solution::is_valid(input);
    assert_eq!(output, true);
}

#[test]
fn official_example_3() {
    let input = "(]".to_string();
    let output = Solution::is_valid(input);
    assert_eq!(output, false);
}

#[test]
fn official_example_4() {
    let input = "([])".to_string();
    let output = Solution::is_valid(input);
    assert_eq!(output, true);
}

#[test]
fn custom_example_1() {
    let input = "([)]".to_string();
    let output = Solution::is_valid(input);
    assert_eq!(output, false);
}

pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {

        let mut stack = Vec::new();

        for c in s.chars() {
            if is_open(c) {
                stack.push(c);
            } else {
                if let Some(last_char) = stack.last() {
                    let last_char = *last_char;
                    if open_matches_closed(last_char, c) {
                        stack.pop();
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        if !stack.is_empty() {
            return false;
        }

        true
    }
}

fn is_open(c: char) -> bool {
    match c {
        '(' | '[' | '{' => true,
        _ => false,
    }
}

fn open_matches_closed(open: char, close: char) -> bool {
    match open {
        '(' => close == ')',
        '[' => close == ']',
        '{' => close == '}',
        _ => panic!("unknown character!"),
    }
}