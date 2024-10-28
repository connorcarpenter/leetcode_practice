// 3 Longest Substring Without Repeating Characters
//
// Hint:
// Given a string s, find the length of the longest substring without repeating characters.
//
// Example 1:
//
// Input: s = "abcabcbb"
// Output: 3
// Explanation: The answer is "abc", with the length of 3.

#[test]
fn example_1() {
    let input = "abcabcbb".to_string();
    let output = Solution::length_of_longest_substring(input);
    assert_eq!(output, 3);
}

// Example 2:
//
// Input: s = "bbbbb"
// Output: 1
// Explanation: The answer is "b", with the length of 1.

#[test]
fn example_2() {
    let input = "bbbbb".to_string();
    let output = Solution::length_of_longest_substring(input);
    assert_eq!(output, 1);
}

// Example 3:
//
// Input: s = "pwwkew"
// Output: 3
// Explanation: The answer is "wke", with the length of 3.
// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.

#[test]
fn example_3() {
    let input = "pwwkew".to_string();
    let output = Solution::length_of_longest_substring(input);
    assert_eq!(output, 3);
}

#[test]
fn example_4() {
    let input = " ".to_string();
    let output = Solution::length_of_longest_substring(input);
    assert_eq!(output, 1);
}

#[test]
fn example_5() {
    let input = "abba".to_string();
    let output = Solution::length_of_longest_substring(input);
    assert_eq!(output, 2);
}

// Constraints:
//
// 0 <= s.length <= 5 * 104
// s consists of English letters, digits, symbols and spaces.
//
// First brute force solution solved in 12:30
//
//  Solved in 47 minutes, 2 seconds

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_c = 0;

        let mut char_set: HashMap<char, i32> = HashMap::new();

        let mut substr_start_i = 0;
        let mut current_i = 0;

        for c in s.chars() {

            if char_set.contains_key(&c) {
                // found duplicate..
                let first_dup_i = char_set.remove(&c).unwrap();
                substr_start_i = first_dup_i + 1;

                let mut chars_to_delete: Vec<char> = Vec::new();
                for (k, v) in char_set.iter_mut() {
                    if *v < first_dup_i {
                        chars_to_delete.push(*k);
                    }
                }
                for ctd in chars_to_delete {
                    char_set.remove(&ctd);
                }
            } else {
                // no duplicate yet
            }

            char_set.insert(c, current_i);

            let current_l = current_i - substr_start_i + 1;
            max_c = max_c.max(current_l);

            current_i += 1;
        }

        max_c
    }
}