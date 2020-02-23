pub struct Options {
    pub code_length: u32,
    pub code_base: u32,
}

pub fn from_stdin() -> Options {
    Options { code_length: 0, code_base: 0 }
}
