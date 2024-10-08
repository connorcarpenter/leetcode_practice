// 11. Container With Most Water
// From: https://leetcode.com/problems/container-with-most-water/
//
// You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).
//
// Find two lines that together with the x-axis form a container, such that the container contains the most water.
//
// Return the maximum amount of water a container can store.
//
// Notice that you may not slant the container.
//
// Example 1:
//
// Input: height = [1,8,6,2,5,4,8,3,7]
// Output: 49
// Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the container can contain is 49.
//
// Example 2:
//
// Input: height = [1,1]
// Output: 1
//
// Constraints:
//
// n == height.length
// 2 <= n <= 105
// 0 <= height[i] <= 104
//
// Solved in 50 minutes, 47 seconds

// Brute Force Solution 11 min 12 seconds
// Optimized Brute Force in 30 min 55 seconds
// Two Pointer Solution in 17 min 0 seconds

#[test]
fn official_example_1() {
    let input = vec![1,8,6,2,5,4,8,3,7];
    let output = Solution::max_area(input);
    assert_eq!(output, 49);
}

struct Solution;

//

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let heights = height;
        let height = 0;
        let mut max_area: i32 = 0;

        let mut left = 0;
        let mut left_h = heights[left];
        let mut max_left_h = 0;

        let mut right = heights.len() - 1;
        let mut right_h = heights[right];
        let mut max_right_h = 0;
        let mut first = true;

        loop {

            if first {
                first = false;
            } else {
                if left_h < right_h {
                    // left moves forward

                    left += 1;
                    if left == right {
                        break;
                    }

                    left_h = heights[left];

                    if left_h > max_left_h {
                        max_left_h = left_h;
                    } else {
                        continue;
                    }
                } else {
                    // right moves backward
                    right -= 1;
                    if left == right {
                        break;
                    }

                    right_h = heights[right];

                    if right_h > max_right_h {
                        max_right_h = right_h;
                    } else {
                        continue;
                    }
                }
            }

            let width = (right - left) as i32;
            let height = left_h.min(right_h);
            let area = width * height;

            println!("({}, {}) -> ({}, {}) = {}", left, left_h, right, right_h, area);

            if area > max_area {
                max_area = area;
            }
        }

        return max_area;
    }
}

// Optimized Brute Force in 30 min 55 seconds
// impl Solution {
//     pub fn max_area(height: Vec<i32>) -> i32 {
//         let heights = height;
//         let height = 0;
//
//         let mut max_area: i32 = 0;
//         let mut i_max_height = 0;
//
//         'outer: for i in 0..heights.len() - 1 {
//
//             let i_val = heights[i];
//             if i_val > i_max_height {
//                 i_max_height = i_val;
//             } else {
//                 // definitely going to be less than before
//                 continue 'outer;
//             }
//
//             let mut j_max_height = 0;
//
//             'inner: for j in 0..heights.len()-1-i {
//
//                 let rj = heights.len()-1 - j;
//                 //println!("i: {}, rj: {}", i, rj);
//                 let j_val = heights[rj];
//
//                 println!("testing ({}, {}) -> ({}, {})", i, i_val, rj, j_val);
//
//                 if j_val > j_max_height {
//                     j_max_height = j_val;
//                 } else {
//                     // definitely going to be less than before
//                     continue 'inner;
//                 }
//
//                 let width = (rj - i) as i32;
//                 let height = i_val.min(j_val);
//                 let area = width * height;
//                 if area > max_area {
//                     max_area = area;
//                 }
//
//                 if j_val >= i_val {
//                     // can't get any fuller than this
//                     break 'inner;
//                 }
//             }
//         }
//
//         return max_area;
//     }
// }

// Brute Force Solution 11 min 12 seconds
// impl Solution {
//     pub fn max_area(height: Vec<i32>) -> i32 {
//         let heights = height;
//         let height = 0;
//
//         let mut max_area: i32 = 0;
//
//         for i in 0..heights.len() - 1 {
//             for j in i+1..heights.len() {
//                 let i_val = heights[i];
//                 let j_val = heights[j];
//
//                 let width = (j-i) as i32;
//                 let height = i_val.min(j_val);
//                 let area = width * height;
//                 if area > max_area {
//                     max_area = area;
//                 }
//             }
//         }
//
//         return max_area;
//     }
// }