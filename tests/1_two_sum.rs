// 1. Two Integer Sum
// From: https://leetcode.com/problems/two-sum/
//
// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
//
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
//
// You can return the answer in any order.
//
// Example 1:
//
// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
//
// Example 2:
//
// Input: nums = [3,2,4], target = 6
// Output: [1,2]
//
// Example 3:
//
// Input: nums = [3,3], target = 6
// Output: [0,1]
//
// Constraints:
//
// 2 <= nums.length <= 104
// -109 <= nums[i] <= 109
// -109 <= target <= 109
// Only one valid answer exists.
//
// Follow-up: Can you come up with an algorithm that is less than O(n2) time complexity?
//
// This implementation took 18 minutes 52 seconds to finish

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut output = Vec::new();

        let mut map = HashMap::new();

        for i in 0..nums.len() {
            let i_val = nums[i];
            if !map.contains_key(&i_val) {
                map.insert(i_val, i);
            }

            let target_value = target - i_val;
            if let Some(j) = map.get(&target_value) {
                let j = *j;
                if j == i {
                    continue;
                }
                // found solution
                output.push(j as i32);
                output.push(i as i32);
                return output;
            }
        }

        panic!("Could not find solution");
    }
}

#[test]
fn example_1() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let output = Solution::two_sum(nums, target);
    assert_eq!(output, vec![0, 1]);
}

#[test]
fn example_2() {
    let nums = vec![3, 2, 4];
    let target = 6;
    let output = Solution::two_sum(nums, target);
    assert_eq!(output, vec![1, 2]);
}

#[test]
fn example_3() {
    let nums = vec![3, 3];
    let target = 6;
    let output = Solution::two_sum(nums, target);
    assert_eq!(output, vec![0, 1]);
}