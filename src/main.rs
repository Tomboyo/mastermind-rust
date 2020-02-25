#[cfg(test)]
#[macro_use]
extern crate maplit;

mod code;
mod options;
mod response;
mod tree;

use crate::tree::Tree;
use crate::tree::rank;

fn main() {
    let options = options::from_stdin();
    let rank = |tree: &Tree| rank::by_depth(tree) as f64;
    let best_tree = tree::generate_exhaustively(
        options.code_length,
        options.code_base,
        &rank
    );
    
    println!("{:?}", best_tree);
}
