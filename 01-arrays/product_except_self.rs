struct Solution;

impl Solution {
    //     pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    //         let n = nums.len();
    //         let mut result = vec![1; n];

    //         for i in 0..n {
    //             let mut product = 1;
    //             for j in 0..i {
    //                 product *= nums[j];
    //             }
    //             for j in (i + 1)..n {
    //                 product *= nums[j];
    //             }
    //             result[i] = product;
    //         }

    //         result
    //     }
    //
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![1; n];

        for i in 1..n {
            result[i] = nums[i - 1] * result[i - 1];
        }

        let mut right_product = 1;

        for i in (0..n).rev() {
            result[i] *= right_product;
            right_product *= nums[i];
        }

        result
    }
}

fn main() {
    let result = Solution::product_except_self(vec![1, 2, 3]);
    println!("{:?}", result);
}
