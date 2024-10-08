// 49. Group Anagrams
// From: https://leetcode.com/problems/group-anagrams
//
// Given an array of strings strs, group the anagrams together. You can return the answer in any order.
//
// Example 1:
//
// Input: strs = ["eat","tea","tan","ate","nat","bat"]
//
// Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
//
// Explanation:
//
// There is no string in strs that can be rearranged to form "bat".
// The strings "nat" and "tan" are anagrams as they can be rearranged to form each other.
// The strings "ate", "eat", and "tea" are anagrams as they can be rearranged to form each other.
//
// Example 2:
//
// Input: strs = [""]
//
// Output: [[""]]
//
// Example 3:
//
// Input: strs = ["a"]
//
// Output: [["a"]]
//
// Constraints:
//
// 1 <= strs.length <= 104
// 0 <= strs[i].length <= 100
// strs[i] consists of lowercase English letters.
//
// solved in 14 minutes 24 seconds

struct Solution;

//

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        const A_INDEX: u8 = 'a' as u8;
        let mut map: HashMap<Vec<u32>, Vec<String>> = HashMap::new();
        for word in strs {
            let mut ana_key: Vec<u32> = vec![0; 26];
            for c in word.chars() {
                let c_index = c as u8;
                let index = (c_index - A_INDEX) as usize;
                ana_key[index] += 1;
            }

            if !map.contains_key(&ana_key) {
                map.insert(ana_key.clone(), Vec::new());
            }
            let list = map.get_mut(&ana_key).unwrap();
            list.push(word);
        }

        let mut output = Vec::new();
        for (key, value) in map {
            output.push(value);
        }
        output
    }
}