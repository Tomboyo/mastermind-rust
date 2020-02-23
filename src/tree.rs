pub mod rank;

use crate::code::Code;

#[derive(Debug)]
pub struct Tree { /* todo */ }

pub fn generate<'a>(
    guesses: Vec<&'a Code>,
    answers: Vec<&'a Code>
) -> impl Iterator<Item = &'a Tree> {
    Generator { guesses, answers }.take(3)
}

pub struct Generator<'a> {
    guesses: Vec<&'a Code>,
    answers: Vec<&'a Code>,
}

impl<'a> Iterator for Generator<'a> {
    type Item = &'a Tree;

    fn next(&mut self) -> Option<Self::Item> {
        Some(&Tree {})
    }
}

pub fn select<'a, 'b: 'a, F>(
    left: &'b Tree,
    right: &'b Tree,
    rank: F
) -> &'a Tree
where F: Fn(&Tree) -> f64 {
    if rank(left) <= rank(right) {
        left
    } else {
        right
    }
}

