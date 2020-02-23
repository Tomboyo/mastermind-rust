#[cfg(test)]
#[macro_use]
extern crate maplit;

mod code;
mod options;
mod response;
mod tree;

use crate::code::Code;
use crate::tree::rank;

fn main() {
    let options = options::from_stdin();
    let universe = Code::universe(options.code_length, options.code_base);
    let init_guesses: Vec<&Code> = universe.iter().collect();
    let init_answers: Vec<&Code> = universe.iter().collect();
    let best_tree = tree::generate(init_guesses, init_answers, rank::by_depth);
    println!("{:?}", best_tree);
}
