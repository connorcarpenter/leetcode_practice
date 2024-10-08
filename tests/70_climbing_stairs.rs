// 70. Climbing Stairs
// From: https://leetcode.com/problems/climbing-stairs
//
// You are climbing a staircase. It takes n steps to reach the top.
//
// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
//
// Example 1:
//
// Input: n = 2
// Output: 2
// Explanation: There are two ways to climb to the top.
// 1. 1 step + 1 step
// 2. 2 steps
//
// Example 2:
//
// Input: n = 3
// Output: 3
// Explanation: There are three ways to climb to the top.
// 1. 1 step + 1 step + 1 step
// 2. 1 step + 2 steps
// 3. 2 steps + 1 step
//
// Constraints:
//
// 1 <= n <= 45
//
// Solved in 40 minutes, 57 seconds
//
// NOTES:
// Initially I sought to solve this essentially through a recursive bread-first search, starting from the first step forward, cancelling a step if we went too far
// that produced the correct result, but leetcode.com told me it took too long to execute .. that makes sense, as my internal test had n == 44 taking a couple seconds to run
// I thought to just try to solve it by stepping backwards from n, though it yielded the same result, it wasn't any faster
//
// Implementing the backwards step function, I realized that there was no need to keep around a "current_steps" value like I was doing while walking forward to make sure we didn't overstep the target
// and seeing the simplicity step(n - 1) + step(n - 2) reminded me of solving Fibonacci with memo-ization, just save the result of step(n) in a hash map, so you never need to re-calculate it
// That was the trick that yielded the fastest performance.. This is a Fibonacci sequence problem in disguise!

struct Solution;

#[test]
fn official_example_1() {
    let output = Solution::climb_stairs(2);
    assert_eq!(output, 2);
}

#[test]
fn official_example_2() {
    let output = Solution::climb_stairs(3);
    assert_eq!(output, 3);
}

#[test]
fn custom_example() {
    let output = Solution::climb_stairs(44);
    assert_eq!(output, 1134903170);
}

//

use std::collections::HashMap;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut map = HashMap::new();
        map.insert(1, 1);
        map.insert(2, 2);

        return step(&mut map, n);
    }
}

fn step(map: &mut HashMap<i32, i32>, n: i32) -> i32 {

    if let Some(ways) = map.get(&n) {
        return *ways;
    }

    let ways_1 = step(map, n - 1);
    let ways_2 = step(map, n - 2);
    let result = ways_1 + ways_2;

    map.insert(n, result);

    result
}