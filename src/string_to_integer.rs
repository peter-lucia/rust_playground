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
use std::cmp;

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


        let mut _start = 0;
        let mut _found = false;
        let mut _end = trimmed.len() - 1;
        for i in 0..trimmed.len() {
            let c: char = trimmed.chars().nth(i).unwrap();
            if !_found && !c.is_numeric() {
                _start += 1;
            } else {
                _found = true;
            }

            if _found && !c.is_numeric() {
                _end = i-1;
                break;
            }
        }
        let mut trimmed_2 = (&trimmed[_start..=_end]).to_string();

        let leading_digit = trimmed.chars()
            .nth(
                cmp::max(_start as i32-1, 0) as usize)
            .unwrap();

        if leading_digit == '-' {
            is_neg = true;
        }
        let size_of_remaining = trimmed_2.len();
        let mut result: i64 = 0;
        let mut power_of_ten = 0;
        for i in (0..size_of_remaining).rev() {
            let digit = trimmed_2.chars().nth(i).unwrap().to_digit(10).unwrap() as i64;
            result += (digit) * (10 as i64).pow(power_of_ten);
            if result > 2147483647 {
                result = 2147483648;
                break;
            }
            power_of_ten += 1;
        }
        if is_neg {
            return -result as i32;
        }
        return result as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use super::MyAtoi;

    #[test]
    fn test_my_atoi_1() {
        let solution = Solution{};
        assert_eq!(solution.my_atoi("  -42".to_string()), -42);
    }

    #[test]
    fn test_my_atoi_2() {
        let solution = Solution{};
        assert_eq!(solution.my_atoi("4193 with words".to_string()), 4193);
    }

    #[test]
    fn test_my_atoi_3() {
        let solution = Solution{};
        assert_eq!(solution.my_atoi("words and 987".to_string()), 987);
    }

    #[test]
    fn test_my_atoi_4() {
        let solution = Solution{};
        assert_eq!(solution.my_atoi("-91283472332".to_string()), -2147483648);
    }
}
