struct Solution;

// impl Solution {
//     pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
//         let num_str: String = digits.iter().map(|d| d.to_string()).collect();

//         let mut num: u128 = num_str.parse().unwrap();

//         num += 1;

//         num.to_string()
//             .chars()
//             .map(|c| c.to_digit(10).unwrap() as i32)
//             .collect()
//     }
// }

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for i in (0..digits.len()).rev() {
            if digits[i] < 9 {
                digits[i] += 1;
                return digits;
            }
            digits[i] = 0;
        }
        let mut result = vec![1];
        result.extend(digits);
        result
    }
}

fn main() {
    let digits = vec![9, 9, 9];
    println!("{:?}", Solution::plus_one(digits));
}
