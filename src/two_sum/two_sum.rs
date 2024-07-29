struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        for (i, &num) in nums.iter().enumerate() {
            map.insert(num, i as i32);
        }
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&(target - num)) {
                if i as i32 != j {
                    return vec![i as i32, j];
                }
            }
        }
        vec![]
    }
}
