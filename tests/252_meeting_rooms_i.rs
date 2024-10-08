// 252. Meeting Rooms I
// From: https://leetcode.com/problems/meeting-rooms/
//
// Given an array of meeting time intervals consisting of start and end times [[s1,e1],[s2,e2],...] (si < ei), determine if a person could attend all meetings.
//
// Example 1:
//
// Input: [[0,30],[5,10],[15,20]]
// Output: false
//
// Example 2:
//
// Input: [[7,10],[2,4]]
// Output: true
//
// Solved in 6 minutes, 59 seconds
//
// NOTE: This Leetcode problem is only available to premium customers, so I had to infer
// the Rust method signature used officially

#[test]
fn official_example_1() {
    let input = vec![(0,30), (5,10), (15,20)];
    let output = Solution::can_attend_all(input);
    assert_eq!(output, false);
}

#[test]
fn official_example_2() {
    let input = vec![(7,10), (2,4)];
    let output = Solution::can_attend_all(input);
    assert_eq!(output, true);
}

struct Solution;

//

impl Solution {
    fn can_attend_all(meetings: Vec<(i32, i32)>) -> bool {
        if meetings.is_empty() {
            return true;
        }

        for i in 0..meetings.len()-1 {
            for j in i+1..meetings.len() {
                let (i_start, i_end) = meetings[i];
                let (j_start, j_end) = meetings[j];
                if (j_start > i_start && j_start < i_end) || (j_end > i_start && j_end < i_end) {
                    return false;
                }
            }
        }

        return true;
    }
}