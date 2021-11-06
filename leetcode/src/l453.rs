struct Solution {}

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let min = nums.iter().min().unwrap_or(&0);
        let sum: i32 = nums.iter().map(|n| n - min).sum();
        sum / (nums.len() as i32 - 1)
    }
}
