use crate::response::Response;

// consider making Code an alias for &[usize] instead
#[derive(Debug, PartialEq, Eq)]
pub struct Code<'a> {
    pub data: &'a [usize],
}

impl<'a> Code<'a> {
    pub fn universe(length: usize, base: usize) -> Vec<Code<'a>> {
        // todo
        vec![
            Code::new(&[0, 0]),
            Code::new(&[0, 1]),
            Code::new(&[1, 0]),
            Code::new(&[1, 1]),
        ]
    }

    pub fn new(data: &'a [usize]) -> Self {
        Code { data }
    }

    pub fn compare(&self, other: &Code) -> Response {
        // TODO
        Response(0, 0)
    }
}
