pub fn vulnerable_copy(
    dest: &mut [u8],
    src: &[u8],
    n: usize,
) {
    let mut i = 0;
    while i < n {
        dest[i] = src[i];
        i += i;
    }
    let s = "abc";
    s.len();
    s.chars();
}
