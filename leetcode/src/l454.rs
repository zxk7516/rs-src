struct Solution;

impl Solution {
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        let mut sum = 0;
        let mut part1 = std::collections::HashMap::new();

        for v1 in nums1.iter() {
            for v2 in nums2.iter() {
                *part1.entry(-v1 - v2).or_insert(0) += 1;
            }
        }
        for v3 in nums3.iter() {
            for v4 in nums4.iter() {
                sum += *part1.entry(v3 + v4).or_default();
            }
        }
        sum
    }
}

#[test]
fn test_it() {
    assert_eq!(
        Solution::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]),
        2
    )
}
