// declare remove_element as a module
mod remove_element;
mod two_sum;
mod add_binary;

fn main() {
    if false {
        println!("Hello, world!");
        println!("The reverse of {0} is {1}", -123, reverse(-123));
        println!("The reverse of {0} is {1}", 1534236469, reverse(1534236469));
    }

    if false {
        let mut nums = vec![1, 1, 2, 3];
        remove_element::remove_element(&mut nums, 1);
        println!("values = {0:?}", nums);
    }

    if false {
        let nums = vec![2, 7, 11, 15];
        let result = two_sum::two_sum(nums, 9);
        println!("Indices adding up to 9 = {0:?}", result);
    }

    if true {
        let solution = add_binary::Solution{};
        assert_eq!(String::from("100"),
                   solution.add_binary(String::from("11"),
                                          String::from("1")));
        assert_eq!(String::from("10101"),
                   solution.add_binary(String::from("1010"),
                                          String::from("1011")));
    }
}

pub fn reverse(mut x: i32) -> i32 {
    /// Reverses an integer, e.g. 123 becomes 321 or -123 becomes -321
    ///
    // example: let x = 32

    let mut is_neg: bool = false;
    if x < 0 {
        is_neg = true;
    }
    x = x.abs();
    let mut values: Vec<i32> = Vec::new();

    // vec = {2, 3}
    let mut i: u32 = 0;
    while (10 as i32).pow(i) <= x {
        values.push(x % 10);
        x /= 10;
    }
    // debug
    // println!("values = {0:?}", values);
    // reverse the vector
    values.reverse();


    let mut result: i64 = 0;

    // result = 0
    // result += 2*10**0
    // result += 3*10**1
    i = 0;
    for v in values {
        result += (v as i64)*(10 as i64).pow(i);
        if result > (2 as i64).pow(31) - 1 {
            return 0;
        }
        i += 1;
    }

    return if is_neg {-result as i32} else {result as i32};
}