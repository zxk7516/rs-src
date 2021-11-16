pub enum Token<'a> {
    Header { typ: HeaderType, val: &'a str },
}

pub enum HeaderType {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

pub trait Tokenizer<'a> {
    fn next(&mut self) -> Option<Token<'a>>;
}

pub struct TokenizerImpl<'a> {
    input: &'a str,
    pos: usize,
}
