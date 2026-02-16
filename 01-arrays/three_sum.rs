struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        if n < 3 {
            return vec![];
        }

        nums.sort_unstable();
        let mut result = Vec::with_capacity(n / 2);

        for i in 0..n - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            if nums[i] > 0 {
                break;
            }

            let (mut right, mut left) = (i + 1, n - 1);
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum < 0 {
                    left += 1;
                } else if sum > 0 {
                    right -= 1;
                } else {
                    result.push(vec![nums[i], nums[left], nums[right]]);

                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }

                    left += 1;
                    right -= 1;
                }
            }
        }
        result
    }
}

fn main() {
    // Test cases: (input, expected_output)
    let test_cases = vec![
        (
            vec![-1, 0, 1, 2, -1, -4],
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
        ),
        (vec![0, 1, 1], vec![]),
        (vec![0, 0, 0], vec![vec![0, 0, 0]]),
        (vec![], vec![]),
        (vec![1, 2], vec![]),
        (vec![0, 0, 0, 0], vec![vec![0, 0, 0]]),
        (vec![-2, 0, 1, 1, 2], vec![vec![-2, 0, 2], vec![-2, 1, 1]]),
        (vec![1, 2, -2, -1], vec![]),
        (vec![-1, 0, 1, 0], vec![vec![-1, 0, 1]]),
    ];

    println!("Running {} test cases for 3Sum...\n", test_cases.len());

    for (i, (input, expected)) in test_cases.into_iter().enumerate() {
        let mut input_clone = input.clone();
        input_clone.sort_unstable(); // normalize for comparison

        let mut result = Solution::three_sum(input);
        // Sort each triplet and the whole result for consistent comparison
        for triplet in &mut result {
            triplet.sort_unstable();
        }
        result.sort_unstable();

        let mut expected_normalized = expected;
        for triplet in &mut expected_normalized {
            triplet.sort_unstable();
        }
        expected_normalized.sort_unstable();

        let passed = result == expected_normalized;
        let status = if passed { "✅ PASS" } else { "❌ FAIL" };

        println!(
            "Test {}: {}\n  Input: {:?}\n  Expected: {:?}\n  Got: {:?}\n",
            i + 1,
            status,
            input_clone,
            expected_normalized,
            result
        );

        if !passed {
            eprintln!("⚠️  Test {} failed!", i + 1);
        }
    }

    println!("All tests completed!");
}
