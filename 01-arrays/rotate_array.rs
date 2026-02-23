struct Solution;

impl Solution {
    // pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    //     for _ in 0..k {
    //         let last = nums.pop().unwrap();
    //         nums.insert(0, last);
    //     }
    // }

    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        if nums.is_empty() {
            return;
        }

        let n = nums.len();
        let k = (k as usize) % n;
        if k == 0 {
            return;
        }

        fn reverse_range(nums: &mut [i32], start: usize, end: usize) {
            let mut left = start;
            let mut right = end - 1;
            while left < right {
                nums.swap(left, right);
                left += 1;
                right -= 1;
            }
        }

        reverse_range(nums, 0, n);
        reverse_range(nums, 0, k);
        reverse_range(nums, k, n);
    }
}

fn main() {
    let tests = vec![
        (vec![1, 2, 3, 4, 5, 6, 7], 3),
        (vec![-1, -100, 3, 99], 2),
        (vec![1], 10),
        (vec![1, 2], 0),
        (vec![1, 2, 3], 4),
    ];
    for (mut arr, k) in tests {
        println!("Before= {:?}, k= {}", arr, k);
        Solution::rotate(&mut arr, k);
        println!("After= {:?}, k= {}", arr, k);
    }
}
