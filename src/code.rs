use crate::response::Response;

pub type Code<'a> = &'a [usize];

pub fn universe<'a>(_length: usize, _base: usize) -> Vec<Code<'a>> {
    // todo
    vec![
        &[0, 0],
        &[0, 1],
        &[1, 0],
        &[1, 1],
    ]
}

pub fn compare(_left: &Code, _right: &Code) -> Response {
    // TODO
    Response(0, 0)
}
