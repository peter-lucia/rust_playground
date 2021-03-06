// declare remove_element as a module
mod remove_element;
mod two_sum;
mod add_binary;
mod reverse_integer;
use reverse_integer::Reverse;
use crate::string_to_integer::MyAtoi;

// Reverse is the trait for the Solution struct
mod letter_combinations;
mod string_to_integer;
mod get_concatenation;
mod buy_sell_stock;
mod graph;
mod subsets_78;
mod time_formatter;
mod maximum_depth_binary_tree_104;

fn main() {
    if false {
        let solution = reverse_integer::Solution{};
        println!("Hello, world!");
        println!("The reverse of {0} is {1}", -123, solution.reverse(-123));
        println!("The reverse of {0} is {1} since we restrict the answer to < 2^31",
                 1534236469, solution.reverse(1534236469));
        assert_eq!(-321, solution.reverse(-123));
        assert_eq!(0, solution.reverse(1534236469));
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

    if false {
        let solution = add_binary::Solution{};
        assert_eq!(String::from("100"),
                   solution.add_binary(String::from("11"),
                                          String::from("1")));
        assert_eq!(String::from("10101"),
                   solution.add_binary(String::from("1010"),
                                          String::from("1011")));
    }
    if false {
        let mut solution = letter_combinations::LetterCombinations::new();
        assert_eq!(vec!["ad".to_string(),
                        "ae".into(),
                        "af".into(),
                        "bd".into(),
                        "be".into(),
                        "bf".into(),
                        "cd".into(),
                        "ce".into(),
                        "cf".into()],
            solution.letter_combinations("23".to_string()));
    }
    if false {
        let solution = string_to_integer::Solution{};
        assert_eq!(solution.my_atoi("  -42".to_string()), -42);
    }
}
