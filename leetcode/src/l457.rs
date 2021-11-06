struct Solution;

impl Solution {
    pub fn circular_array_loop(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        let nr = &nums;
        let n = nums.len();

        let next = |cur: usize| {
            if nr[cur] < 0 {
                let n = n as i64;
                (((cur as i64 + nr[cur] as i64) % n + n) % n) as usize
            } else {
                ((cur + nr[cur] as usize) % n + n) % n
            }
        };
        for i in 0..n {
            if nums[i] == 0 {
                continue;
            }
            let mut slow = i;
            let mut fast = next(i);
            while nums[slow] * nums[fast] > 0 && nums[slow] * nums[next(fast)] > 0 {
                if slow == fast {
                    if slow != next(slow) {
                        return true;
                    } else {
                        break;
                    }
                }
                slow = next(slow);
                fast = next(next(fast));
            }

            let mut add = i;
            let mut tmp;
            while nr[add] * nr[next(add)] > 0 {
                tmp = add;
                add = next(add);
                unsafe {
                    let ptr = nums.as_ptr().add(tmp) as *mut i32;
                    *ptr = 0;
                }
                // nums[tmp] = 0
            }
        }

        false
    }
}
