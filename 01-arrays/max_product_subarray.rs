struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut max_so_far = nums[0];
        let mut min_so_far = nums[0];
        let mut result = nums[0];

        for i in 1..nums.len() {
            let current = nums[i];

            let candidate = [current, max_so_far * current, min_so_far * current];

            let new_max = *candidate.iter().max().unwrap();
            let new_min = *candidate.iter().min().unwrap();

            max_so_far = new_max;
            min_so_far = new_min;

            if max_so_far > result {
                result = max_so_far;
            }
        }
        result
    }
}

fn main() {
    assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
    assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
    assert_eq!(Solution::max_product(vec![-2, -3, -1]), 6);
    assert_eq!(Solution::max_product(vec![5]), 5);
    assert_eq!(Solution::max_product(vec![-1]), -1);
    assert_eq!(Solution::max_product(vec![-2, 3, -4]), 24);
    assert_eq!(Solution::max_product(vec![2, -5, -2, -4, 3]), 24);
    assert_eq!(Solution::max_product(vec![1, 2, 3, 4]), 24);
    assert_eq!(Solution::max_product(vec![0, 2]), 2);
    assert_eq!(Solution::max_product(vec![0, -2, 0, -1]), 0);
    assert_eq!(Solution::max_product(vec![-1, -2, -3]), 6);
    println!("✅ All tests passed!");
}
