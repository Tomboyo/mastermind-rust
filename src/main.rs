#![feature(test)]

extern crate clap;
#[cfg(test)] #[macro_use]
extern crate maplit;
extern crate num_traits;
extern crate test;

mod code;
mod options;
mod response;
mod tree;

use crate::tree::rank;

fn main() {
    let options = options::from_stdin();

    let best_tree = tree::generate_exhaustively(
        options.code_length,
        options.code_base,
        &rank::by_depth
    );

    println!("{:?}", best_tree);
}
