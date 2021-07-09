use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = vec![];
    let mut lookup: HashMap<i32, i32> = HashMap::new();

    for (idx, each) in nums.iter().enumerate() {
       if lookup.contains_key(&(target - each)) {
           result.push(lookup[&((target-each) as i32)]);
           result.push(idx as i32);
           return result;
       } else {
           lookup.insert(*each, idx as i32);
       }
    }
    return result;
}