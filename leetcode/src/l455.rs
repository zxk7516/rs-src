struct Solution;

impl Solution {
    pub fn find_content_children(
        g: Vec<i32>,
        s: Vec<i32>,
    ) -> i32 {
        let mut g = g;
        g.sort();
        let mut s = s;
        s.sort();
        let lg = g.len();
        let ls = s.len();
        let mut i = 0;
        let mut j = 0;
        let mut sum = 0;
        while i < lg && j < ls {
            if g[i] <= s[j] {
                i += 1;
                j += 1;
                sum += 1;
            } else {
                j += 1;
            }
        }
        sum
    }
}
#[test]
fn test_it() {
    assert_eq!(
        Solution::find_content_children(vec![1, 2, 3], vec![1, 1]),
        1
    );
    assert_eq!(
        Solution::find_content_children(vec![1, 2], vec![1, 2, 3]),
        2
    );
}
