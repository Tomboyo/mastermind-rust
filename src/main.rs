#![feature(test)]

extern crate clap;
extern crate num_traits;
extern crate test;

#[cfg(test)] #[macro_use]
extern crate maplit;

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
