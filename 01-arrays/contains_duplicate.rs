use std::collections::HashSet;

struct Solution;

impl Solution {
    // The above only works if it's sorted and the numbers are repeated
    // one after another
    // pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    //     let mut num = nums[0];
    //     let mut value = false;

    //     for i in 1..nums.len() {
    //         if num == nums[i] {
    //             value = true;
    //         } else {
    //             num = nums[i];
    //         }
    //     }
    //     value
    // }

    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::new();

        for &num in nums.iter() {
            if seen.contains(&num) {
                return true;
            }
            seen.insert(num);
        }
        false
    }
}

fn main() {
    println!("{}", Solution::contains_duplicate(vec![1, 2, 3, 1])); // true
    println!("{}", Solution::contains_duplicate(vec![1, 2, 3, 4])); // false
    println!(
        "{}",
        Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2])
    ); // true
}
