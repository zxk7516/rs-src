struct Solution;

impl Solution {
    pub fn remove_occurrences(
        s: String,
        part: String,
    ) -> String {
        let mut s = s;
        let mut i = 0;
        while i < s.len() {
            println!("{}", s);
            if s[i..].starts_with(&part) {
                // let _: String = s.drain(i..i + part.len()).collect();
                s.drain(i..i + part.len());
                i = 0;
            } else {
                i = i + 1;
            }
        }
        s
    }
}

#[test]
fn test_it() {
    assert_eq!(
        Solution::remove_occurrences("daabcbaabcbc".to_string(), "abc".to_string()),
        "dab".to_string()
    );
}
