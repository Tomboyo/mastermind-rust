#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
// ( correct pegs, misplaced pegs, unused pegs )
pub struct Response(pub usize, pub usize, pub usize);

pub fn is_correct(r: &Response) -> bool {
    return r.1 == 0 && r.2 == 0
}
