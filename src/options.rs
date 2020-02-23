pub struct Options {
    pub code_length: usize,
    pub code_base: usize,
}

pub fn from_stdin() -> Options {
    Options { code_length: 0, code_base: 0 }
}
