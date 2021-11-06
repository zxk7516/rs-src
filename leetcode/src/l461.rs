use std::arch::x86_64::_popcnt32;

struct Solution;

impl Solution {
    pub fn hamming_distance(
        x: i32,
        y: i32,
    ) -> i32 {
        let res = unsafe { _popcnt32(x ^ y) };
        let _res = (x^y).count_ones();
        res
    }
}
