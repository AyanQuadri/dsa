struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut current_max = nums[0];
        let mut global_max = nums[0];

        for i in 1..nums.len() {
            current_max = nums[i].max(current_max + nums[i]);
            global_max = current_max.max(global_max);
        }
        global_max
    }
}

fn main() {
    println!("{}", Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4])); // 6
    println!("{}", Solution::max_sub_array(vec![1]));                      // 1
    println!("{}", Solution::max_sub_array(vec![5,4,-1,7,8]));            // 23
    println!("{}", Solution::max_sub_array(vec![-1,-2,-3]));              // -1
}
