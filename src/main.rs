mod code;
mod options;
mod tree;

use crate::code::Code;
use crate::tree::rank;

fn main() {
    let options = options::from_stdin();
    let universe = code::universe(options.code_length, options.code_base);
    let init_guesses: Vec<&Code> = universe.iter().collect();
    let init_answers: Vec<&Code> = universe.iter().collect();
    let mut tree_iter = tree::generate(init_guesses, init_answers);
    
    let mut best_tree = tree_iter.next().expect("At least one tree must exist");
    for candidate_tree in tree_iter {
        best_tree = tree::select(best_tree, candidate_tree, rank::by_depth);
    }

    println!("{:?}", best_tree);
}
