
/*
Given two binary strings a and b, return their sum as a binary string.

Example 1:

Input: a = "11", b = "1"
Output: "100"
Example 2:

Input: a = "1010", b = "1011"
Output: "10101"
 */
pub struct Solution {

}

impl Solution {
    fn binary_string_to_i32(num: String) -> i32 {
        return i32::from_str_radix(&*num, 2).unwrap();
    }

    fn i32_to_binary_string(num: i32) -> String {
        return format!("{:b}", num);
    }

    pub fn add_binary(&self, a: String, b: String) -> String {
        /*
        &self     - just want to read the data in the struct
        &mut self - want to change the instance we've called the method on
        self      - the method transforms self into something else
                    and you want to prevent the caller from using the original
                    instance after the transformation.

         */
        let num1 = Solution::binary_string_to_i32(a);
        let num2 = Solution::binary_string_to_i32(b);
        let total = num1 + num2;
        println!("{0} + {1} = {2}", num1, num2, total);
        return Solution::i32_to_binary_string(total);
    }
}