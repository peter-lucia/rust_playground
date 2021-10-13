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

pub struct LetterCombinations {
    digits: String,
    combinations: Vec<String>,
    lookup: HashMap<i32, String>,
}

// See the python solution found here: https://leetcode.com/problems/letter-combinations-of-a-phone-number/solution/
impl LetterCombinations {
    pub fn new() -> LetterCombinations {
        return LetterCombinations {
            digits: "".to_string(),
            combinations: vec![],
            lookup: [
                (1, "".to_string()),
                (2, "abc".to_string()),
                (3, "def".to_string()),
                (4, "ghi".to_string()),
                (5, "jkl".to_string()),
                (6, "mno".to_string()),
                (7, "pqrs".to_string()),
                (8, "tuv".to_string()),
                (9, "wxyz".to_string()),
            ].iter().cloned().collect(),
        }
    }

    fn convert_string_i32(&self, str: String) -> i32 {
        return str.parse::<i32>().unwrap();
    }

    fn backtrack(&mut self, index: i32, mut path: Vec<String>) {
        if path.len() == self.digits.len() {
            self.combinations.push(path.join(""));
            return;
        }
        let possible_letters: String =
            self.lookup.get(&self.convert_string_i32(self.digits.chars()
                .nth(index as usize).unwrap().to_string()))
                .unwrap().to_string();
        // let possible_letters.unwrap().to_string();
        for letter in possible_letters.chars() {
            path.push(letter.to_string());
            // need to call path.to_vec() to get a new copy of path
            self.backtrack(index + 1, path.to_vec());
            path.pop();
        }

    }
    pub fn letter_combinations(&mut self, digits: String) -> Vec<String> {
        self.digits = digits;
        if self.digits.len() == 0 {
            return vec![];
        }
        self.backtrack(0, Vec::new());
        // return a copy of self.combinations
        return self.combinations.to_vec();
    }
}