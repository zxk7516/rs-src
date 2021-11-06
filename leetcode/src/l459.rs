struct Solution {}

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let s2 = s.repeat(2);
        let sub = &s2[1..s2.len() - 1];
        sub.contains(&s)
    }
}
