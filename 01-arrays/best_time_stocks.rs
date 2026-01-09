struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }

        let mut min_price = prices[0];
        let mut max_profit = 0;

        for i in 1..prices.len() {
            if min_price > prices[i] {
                min_price = prices[i];
            } else {
                let profit = prices[i] - min_price;
                if max_profit < profit {
                    max_profit = profit;
                }
            }
        }

        max_profit
    }
}

fn main() {
    let test_cases = vec![
        (vec![7, 1, 5, 3, 6, 4], 5),
        (vec![7, 6, 4, 3, 1], 0),
        (vec![1, 2, 3, 4, 5], 4),
        (vec![5], 0),
        (vec![2, 1], 0),
        (vec![1, 2], 1),
        (vec![3, 2, 6, 5, 0, 3], 4),
        (vec![1, 1, 1, 1], 0),
    ];

    println!("Running {} test cases", test_cases.len());

    for (i, (prices, expected)) in test_cases.into_iter().enumerate() {
        let result = Solution::max_profit(prices.clone());
        let status = if result == expected {
            "Passed"
        } else {
            "Failed"
        };

        println!(
            "Test {}: {}\n Input: {:?}\n Expected: {}, Got: {}\n",
            i + 1,
            status,
            prices,
            expected,
            result
        );
    }
    println!("All tests completed!");
}
