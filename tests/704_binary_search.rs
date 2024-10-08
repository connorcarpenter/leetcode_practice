// 704. Binary Search
// From: https://leetcode.com/problems/binary-search
//
// Given an array of integers nums which is sorted in ascending order, and an integer target, write a function to
// search target in nums. If target exists, then return its index. Otherwise, return -1.
//
// You must write an algorithm with O(log n) runtime complexity.
//
// Example 1:
//
// Input: nums = [-1,0,3,5,9,12], target = 9
// Output: 4
// Explanation: 9 exists in nums and its index is 4
//
// Example 2:
//
// Input: nums = [-1,0,3,5,9,12], target = 2
// Output: -1
// Explanation: 2 does not exist in nums so return -1
//
// Constraints:
//
// 1 <= nums.length <= 104
// -104 < nums[i], target < 104
// All the integers in nums are unique.
// nums is sorted in ascending order.
//
// This implementation took 22 minutes 27 seconds to finish

#[test]
fn official_example_1() {
    let nums = vec![-1,0,3,5,9,12];
    let target = 9;
    let output = Solution::search(nums, target);
    assert_eq!(output, 4);
}

#[test]
fn official_example_2() {
    let nums = vec![-1,0,3,5,9,12];
    let target = 2;
    let output = Solution::search(nums, target);
    assert_eq!(output, -1);
}

#[test]
fn custom_example_1() {
    let nums = vec![5];
    let target = -5;
    let output = Solution::search(nums, target);
    assert_eq!(output, -1);
}

struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {

        if nums.len() == 0 {
            return -1;
        }

        let mut left = 0;
        let mut right = nums.len() - 1;

        loop {
            let range = right - left;

            if range <= 1 {
                if nums[left] == target {
                    return left as i32;
                }
                if nums[right] == target {
                    return right as i32;
                }
                return -1;
            }

            let index = left + (range / 2);
            let value = nums[index];

            if value == target {
                return index as i32;
            } else if value > target {
                // go left
                right = index;
            } else if value < target {
                // go right
                left = index;
            } else {
                panic!("not possible");
            }
        }
    }
}