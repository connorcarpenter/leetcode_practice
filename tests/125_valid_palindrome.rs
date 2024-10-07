// 125. Valid Palindrome
// From: https://leetcode.com/problems/valid-palindrome
//
// A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters,
// it reads the same forward and backward. Alphanumeric characters include letters and numbers.
//
// Given a string s, return true if it is a palindrome, or false otherwise.
//
// Example 1:
//
// Input: s = "A man, a plan, a canal: Panama"
// Output: true
// Explanation: "amanaplanacanalpanama" is a palindrome.
//
// Example 2:
//
// Input: s = "race a car"
// Output: false
// Explanation: "raceacar" is not a palindrome.
//
// Example 3:
//
// Input: s = " "
// Output: true
// Explanation: s is an empty string "" after removing non-alphanumeric characters.
// Since an empty string reads the same forward and backward, it is a palindrome.
//
// Constraints:
//
// 1 <= s.length <= 2 * 105
// s consists only of printable ASCII characters.

#[test]
fn official_example_1() {
    let input = "A man, a plan, a canal: Panama".to_string();
    let output = Solution::is_palindrome(input);
    assert_eq!(output, true);
}

#[test]
fn official_example_2() {
    let input = "race a car".to_string();
    let output = Solution::is_palindrome(input);
    assert_eq!(output, false);
}

#[test]
fn official_example_3() {
    let input = " ".to_string();
    let output = Solution::is_palindrome(input);
    assert_eq!(output, true);
}

struct Solution;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = convert_to_lowercase_alphanumeric(s).into_bytes();
        let len = s.len();
        let chars_to_check = if len % 2 == 0 { len / 2 } else { len + 1 / 2 };
        for i in 0..chars_to_check {
            if s[i] != s[len-1-i] {
                return false;
            }
        }
        return true;
    }
}

#[test]
fn custom_test_alphanumeric_conversions() {
    assert_eq!(convert_to_lowercase_alphanumeric("WHATSUP123".to_string()), "whatsup123".to_string());
    assert_eq!(convert_to_lowercase_alphanumeric("Yo dawg! -@$(&*!! damn".to_string()), "yodawgdamn".to_string());
}

fn convert_to_lowercase_alphanumeric(s: String) -> String {
    let s = s.to_ascii_lowercase();
    let mut output = "".to_string();

    for c in s.chars() {
        if c.is_alphanumeric() {
            output.push(c);
        }
    }

    output
}