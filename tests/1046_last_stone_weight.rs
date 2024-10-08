// 1046. Last Stone Weight
// From https://leetcode.com/problems/last-stone-weight
//
// You are given an array of integers stones where stones[i] is the weight of the ith stone.
//
// We are playing a game with the stones. On each turn, we choose the heaviest two stones and smash them together. Suppose the heaviest two stones have weights x and y with x <= y. The result of this smash is:
//
// If x == y, both stones are destroyed, and
// If x != y, the stone of weight x is destroyed, and the stone of weight y has new weight y - x.
// At the end of the game, there is at most one stone left.
//
// Return the weight of the last remaining stone. If there are no stones left, return 0.
//
// Example 1:
//
// Input: stones = [2,7,4,1,8,1]
// Output: 1
// Explanation:
// We combine 7 and 8 to get 1 so the array converts to [2,4,1,1,1] then,
// we combine 2 and 4 to get 2 so the array converts to [2,1,1,1] then,
// we combine 2 and 1 to get 1 so the array converts to [1,1,1] then,
// we combine 1 and 1 to get 0 so the array converts to [1] then that's the value of the last stone.
//
// Example 2:
//
// Input: stones = [1]
// Output: 1
//
// Constraints:
//
// 1 <= stones.length <= 30
// 1 <= stones[i] <= 1000
//
// Solved in 11 minutes, 51 seconds

struct Solution;
impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut stones = stones;
        loop {
            if stones.is_empty() {
                return 0;
            }
            if stones.len() == 1 {
                return stones[0];
            }
            stones = smash(stones);
        }
    }
}

fn smash(mut stones: Vec<i32>) -> Vec<i32> {
    let mut first_index = 0;
    let mut first_heaviest = stones[first_index];
    let mut second_index = 1;
    let mut second_heaviest = stones[second_index];

    if first_heaviest < second_heaviest {
        std::mem::swap(&mut first_index, &mut second_index);
        std::mem::swap(&mut first_heaviest, &mut second_heaviest);
    }

    for index in 2..stones.len() {
        let value = stones[index];
        if value > first_heaviest {
            second_heaviest = first_heaviest;
            second_index = first_index;
            first_heaviest = value;
            first_index = index;
            continue;
        }
        if value > second_heaviest {
            second_heaviest = value;
            second_index = index;
        }
    }

    stones.remove(first_index);

    if first_index < second_index {
        second_index -= 1;
    }
    stones.remove(second_index);

    if first_heaviest == second_heaviest {
        return stones;
    }

    let after_smash = first_heaviest - second_heaviest;
    stones.push(after_smash);

    stones
}