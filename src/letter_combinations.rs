/*
Given a string containing digits from 2-9 inclusive,
return all possible letter combinations that the number could represent.
Return the answer in any order.
A mapping of digit to letters (just like on the telephone buttons) is
given below. Note that 1 does not map to any letters.

Input: digits = "23"
Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]

Input: digits = ""
Output: []

Input: digits = "2"
Output: ["a","b","c"]
 */
use std::collections::HashMap;

pub struct Solution {
    pub digits: String,
    pub combinations: Vec<String>,

}

// See the python solution found here: https://leetcode.com/problems/letter-combinations-of-a-phone-number/solution/
impl Solution {
    fn backtrack(&mut self, index: i32, path: Vec<String>) {
        if path.len() == self.digits.len() {
            self.combinations.push(path.join(""));
        }

    }
    pub fn letter_combinations(&self, digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return vec![];
        }
        let mut lookup: HashMap<i32, String> = HashMap::new();
        lookup.insert(1, "".to_string());
        lookup.insert(2, "abc".to_string());
        lookup.insert(3, "def".to_string());
        lookup.insert(4, "ghi".to_string());
        lookup.insert(5, "jkl".to_string());
        lookup.insert(6, "mno".to_string());
        lookup.insert(7, "pqrs".to_string());
        lookup.insert(8, "tuv".to_string());
        lookup.insert(9, "wxyz".to_string());
        let result: Vec<String> = Vec::new();
        return result;
    }
}