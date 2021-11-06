fn q03(s: &str) -> usize {
    // 将字符串首尾拼接
    let s2 = s.repeat(2);
    
    let mut s_iter = s2.chars();
    let mut last_char = s_iter.next().unwrap();
    let mut size = 1;
    let mut sum = 0;
    for c in s_iter {
        if c != last_char {
            size += 1;
        } else {
            if size > sum {
                sum = size;
            }
            size = 1;
        }
        last_char = c;
    }
    sum
}

#[test]
fn test_q03() {
    let res = q03("1212111212");
    dbg!(res);
    assert_eq!(res, 9);
}
