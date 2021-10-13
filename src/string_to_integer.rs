// Input: s = "42"
// Output: 42
// Explanation: The underlined characters are what is read in, the caret is the current reader position.
// Step 1: "42" (no characters read because there is no leading whitespace)
//          ^
// Step 2: "42" (no characters read because there is neither a '-' nor '+')
//          ^
// Step 3: "42" ("42" is read in)
//            ^
// The parsed integer is 42.
// Since 42 is in the range [-231, 231 - 1], the final result is 42.

// Input: s = "   -42"
// Output: -42
// Explanation:
// Step 1: "   -42" (leading whitespace is read and ignored)
//             ^
// Step 2: "   -42" ('-' is read, so the result should be negative)
//              ^
// Step 3: "   -42" ("42" is read in)
//                ^
// The parsed integer is -42.
// Since -42 is in the range [-231, 231 - 1], the final result is -42.

pub struct Solution {

}

pub trait MyAtoi {
    /*
    pub is implied in traits
     */
    fn my_atoi(&self, s: String) -> i32;
}

impl MyAtoi for Solution {
    fn my_atoi(&self, s: String) -> i32 {
        let mut trimmed = s.trim().to_string();
        let mut is_neg = false;
        if trimmed.len() == 0 {
            return 0;
        }

        if trimmed.chars().nth(0).unwrap() == '-' {
            is_neg = true;
            trimmed.remove(0);
        }
        let size_of_remaining = trimmed.len();
        let mut result = 0;
        let mut power_of_ten = 0;
        for i in (0..size_of_remaining).rev() {
            let digit = trimmed.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;
            result += (digit) * (10 as i32).pow(power_of_ten);
            power_of_ten += 1;
        }
        if is_neg {
            return -result;
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use super::MyAtoi;

    #[test]
    fn test_my_atoi() {
        let solution = Solution{};
        assert_eq!(solution.my_atoi("  -42".to_string()), -42);
    }
}
