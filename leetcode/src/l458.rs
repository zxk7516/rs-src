struct Solution;

impl Solution {
    pub fn poor_pigs(
        buckets: i32,
        minutes_to_die: i32,
        minutes_to_test: i32,
    ) -> i32 {
        let states = minutes_to_test / minutes_to_die + 1;
        let buckets = buckets as f64;
        let states = states as f64;
        (buckets.log2() as f64 / states.log2()).ceil() as i32
    }
}
