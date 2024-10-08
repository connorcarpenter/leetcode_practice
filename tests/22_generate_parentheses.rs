// 22. Generate Parentheses
// From: https://leetcode.com/problems/generate-parentheses
//
// Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
//
// Example 1:
//
// Input: n = 3
// Output: ["((()))","(()())","(())()","()(())","()()()"]
//
// Example 2:
//
// Input: n = 1
// Output: ["()"]
//
// Constraints:
//
// 1 <= n <= 8
//
// Solved in 22 minutes, 31 seconds

struct Solution;

// Example 3:
// Input: n = 2
// Output: ["(())", "()()"]

#[test]
fn custom_example_1() {
    let output = Solution::generate_parenthesis(2);
    assert_eq!(output, vec!["(())".to_string(), "()()".to_string()]);
}

//

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut output = Vec::new();
        let mut current_string = "(".to_string();
        let opens = 1;
        let closes = 0;

        if opens + closes < n {
            generate_and_add(current_string.clone(), opens, closes, false, n, &mut output);
        }
        generate_and_add(current_string.clone(), opens, closes, true, n, &mut output);

        output
    }
}

fn generate_and_add(mut current_string: String, mut opens: i32, mut closes: i32, next_is_close: bool, target: i32, output: &mut Vec<String>) {
    if next_is_close {
        current_string.push(')');
        closes += 1;
        opens -= 1;
        if closes == target {
            output.push(current_string);
            return;
        } else {
            if opens + closes < target {
                generate_and_add(current_string.clone(), opens, closes, false, target, output);
            }
            if opens > 0 {
                generate_and_add(current_string.clone(), opens, closes, true, target, output);
            }
        }
    } else {
        current_string.push('(');
        opens += 1;
        if opens + closes < target {
            generate_and_add(current_string.clone(), opens, closes, false, target, output);
        }
        generate_and_add(current_string.clone(), opens, closes, true, target, output);
    }
}