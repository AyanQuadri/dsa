// struct Solution;

// impl Solution {
//     pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//         for i in 0..nums.len() {
//             for j in (i + 1)..nums.len() {
//                 if nums[i] + nums[j] == target {
//                     println!("{} + {} = {}", nums[i], nums[j], nums[i] + nums[j]);
//                     return vec![i as i32, j as i32];
//                 }
//             }
//         }
//         vec![]
//     }
// }

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&j) = map.get(&complement) {
                return vec![j as i32, i as i32];
            }
            map.insert(num, i);
        }
        vec![]
    }
}

fn main() {
    let result = Solution::two_sum(vec![11, 15, 2, 7], 9);
    println!("{:?}", result);
}
