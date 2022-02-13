//Given an integer array nums of unique elements, return all possible subsets (
//the power set).
//
// The solution set must not contain duplicate subsets. Return the solution in
//any order.
//
//
// Example 1:
//
//
//Input: nums = [1,2,3]
//Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
//
//
// Example 2:
//
//
//Input: nums = [0]
//Output: [[],[0]]
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 10
// -10 <= nums[i] <= 10
// All the numbers of nums are unique.
use std::collections::HashSet;
use std::iter::FromIterator;

struct Solution {

}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // Approach:
        // Iterate over each element in nums
        // Add each element to each existing subset in the result, creating
        // a new item in the powerset

        let mut result: Vec<Vec<i32>> = vec![];
        result.push(vec![]);

        for num in nums {
            for subset in result.clone() {
                let mut new_subset = subset.clone();
                new_subset.push(num);
                result.push(new_subset);
            }
        }
        return result;
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let input = vec![1, 2, 3];
        let expected_result= vec![
            vec![],
            vec![1],
            vec![2],
            vec![1,2],
            vec![3],
            vec![1,3],
            vec![2,3],
            vec![1,2,3],
            ];
        let actual_result = Solution::subsets(input);
        assert_eq!(actual_result, expected_result);
    }
}
