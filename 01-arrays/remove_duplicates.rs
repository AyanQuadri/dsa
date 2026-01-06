struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut k = 0;

        for i in 1..nums.len() {
            if nums[k] != nums[i] {
                k += 1;
                nums[k] = nums[i];
            }
        }
        (k + 1) as i32
    }
}

fn main() {
    let mut vecc = vec![0, 0, 1, 1, 1, 1, 2, 2, 3, 3, 4, 4];
    let k = Solution::remove_duplicates(&mut vecc);
    println!("{}", k);
    println!("{:?}", &vecc[..k as usize]);
}
