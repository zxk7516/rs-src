struct Solution;

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n < 3 {
            return false;
        }
        let mut last = i32::MIN;
        let mut sta: Vec<i32> = vec![];
        for i in (0..n).rev() {
            if nums[i] < last {
                return true;
            }
            while !sta.is_empty() && sta.last().unwrap() < &nums[i] {
                last = sta.pop().unwrap();
            }
            sta.push(nums[i]);
        }
        false
    }
}

#[test]
fn test_it() {
    assert_eq!(Solution::find132pattern(vec![1, 2, 3, 4]), false);
    assert_eq!(Solution::find132pattern(vec![3, 1, 4, 2]), true);
    assert_eq!(Solution::find132pattern(vec![-1, 3, 2, 0]), true);
}
