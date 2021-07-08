use std::collections::HashSet;

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    /// Removes all occurrences of an element from a vector
    /// and returns the size of the resultant vector
    /// For reference, see
    /// https://leetcode.com/problems/remove-element/
    ///

    let mut i = 0;
    while i < nums.len() {
        if nums[i] == val {
            nums.remove(i);
        } else {
            i += 1;
        }
    }
    return nums.len() as i32;
}
