struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);

        while left < right {
            let mid = left + (right - left) / 2;

            if nums[mid] > nums[right] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        nums[left]
    }
}

fn main() {
    let test_cases = vec![
        (vec![3, 4, 5, 1, 2], 1),
        (vec![4, 5, 6, 7, 0, 1, 2], 0),
        (vec![11, 13, 15, 17], 11),
        (vec![1], 1),
        (vec![2, 1], 1),
        (vec![1, 2], 1),
        (vec![5, 1, 2, 3, 4], 1),
        (vec![2, 3, 4, 5, 1], 1),
        (vec![1, 2, 3, 4, 5], 1),
        (vec![3, 1, 2], 1),
    ];

    println!(
        "Running {} Test cases for find minimum...\n",
        test_cases.len()
    );
    for (i, (input, expected)) in test_cases.into_iter().enumerate() {
        let result = Solution::find_min(input.clone());
        let passed = result == expected;
        let status = if passed { "✅ PASS" } else { "❌ FAILED" };

        println!(
            "Test {}: {}\n Input: {:?}\n Expected: {}\n Got: {}\n",
            i + 1,
            status,
            input,
            expected,
            result
        );

        if !passed {
            eprintln!("⚠️  Test {} failed!", i + 1);
        }
    }
    println!("All tests completed!");
}
