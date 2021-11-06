pub fn last_lenght_of_word(str: String) -> usize {
    str.split_whitespace().last().map(|s| s.len()).unwrap_or(0)
}
