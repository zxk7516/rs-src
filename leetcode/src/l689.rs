struct Solution;

impl Solution {
    pub fn max_sum_of_three_subarrays(
        nums: Vec<i32>,
        k: i32,
    ) -> Vec<i32> {
        let k = k as usize;
        let size = nums.len();
        let n = size - k + 1;
        let mut sub_arr_sum = vec![0; n];
        let mut sum = 0;
        for i in 0..size {
            sum = sum + nums[i];
            if i >= k {
                sum = sum - nums[i - k];
            }
            if i >= k - 1 {
                sub_arr_sum[i - k + 1] = sum;
            }
        }
        let mut left = vec![0; n];
        let mut right = vec![0; n];

        let mut max_index = 0;
        for i in (0..n).rev() {
            if sub_arr_sum[max_index] <= sub_arr_sum[i] {
                max_index = i;
            }
            right[i] = max_index;
        }
        let mut res: Vec<i64> = vec![-1; 3];
        for i in k..(n - k) {
            if res[0] == -1
                || sub_arr_sum[res[0] as usize]
                    + sub_arr_sum[res[1] as usize]
                    + sub_arr_sum[res[2] as usize]
                    < sub_arr_sum[i]
                        + sub_arr_sum[left[i - k] as usize]
                        + sub_arr_sum[right[i + k] as usize]
            {
                res[0] = left[i - k] as i64;
                res[1] = i as i64;
                res[2] = right[i + k] as i64;
            }
        }
        res.iter().map(|i| *i as i32).collect()
    }
}
