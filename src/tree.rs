pub mod rank;

use std::collections::BTreeMap;

use crate::code;
use crate::code::Code;
use crate::response;
use crate::response::Response;

#[derive(Debug, PartialEq, Eq)]
pub struct Tree {
    guess: Code,
    children: BTreeMap<Response, Option<Tree>>,
}

pub fn generate_exhaustively<F>(
    code_length: usize,
    code_base: usize,
    rank: &F,
) -> Tree
where F: Fn(&Tree) -> f64 {
    let universe = code::universe(code_length, code_base);
    generate(
        universe.iter().cloned().collect(),
        universe.iter().cloned().collect(),
        rank
    )
}

pub fn generate<'a, F>(
    guesses: Vec<Code>,
    answers: Vec<Code>,
    rank: &F
) -> Tree
where F: Fn(&Tree) -> f64 {
    guesses.iter()
        .map(|guess| {
            let mut children = BTreeMap::new();
            for (response, remaining_answers) in answers_by_response(&guess, &answers) {
                if response::is_correct(&response) {
                    // terminal case
                    children.insert(response, None);
                } else {
                    // recursive case
                    let remaining_guesses = guesses.iter()
                        .cloned()
                        .filter(|x| x != guess)
                        .collect();
                    children.insert(
                        response,
                        Some(generate(
                            remaining_guesses,
                            remaining_answers,
                            rank)));
                }
            }

            let guess = guess.clone();
            Tree { guess, children }
        })
        .fold(None, |acc, tree| match acc {
            None => Some(tree),
            Some(best) => Some(select(best, tree, &rank))
        })
        .expect("The should be at least one tree")
}

fn answers_by_response(
    guess: &Code,
    answers: &Vec<Code>,
) -> BTreeMap<Response, Vec<Code>> {
    answers.iter()
        .fold(BTreeMap::new(), |mut map, answer| {
            map.entry(code::compare(guess, answer))
                .or_insert(Vec::new())
                .push(answer.clone());
            map
        })
}

fn select<F>(
    left: Tree,
    right: Tree,
    rank: &F
) -> Tree
where F: Fn(&Tree) -> f64 {
    if rank(&left) <= rank(&right) {
        left
    } else {
        right
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;
    use crate::tree::rank;

    #[test]
    fn test_generate() {
        // prefer trees based on their guess; 0,0 is "best".
        let rank = |tree: &Tree| match &tree.guess[..] {
            &[0, 0] => 0f64,
            &[0, 1] => 1f64,
            x => panic!("Unexpected test code {:?}", x)
        };

        let actual = generate(
            vec![vec![0, 0], vec![0, 1]],
            vec![vec![0, 0], vec![0, 1]],
            &rank
        );

        let expected = Tree {
            guess: vec![0, 0],
            children: btreemap![
                Response(2, 0, 0) => None,
                Response(1, 0, 1) => Some(Tree {
                    guess: vec![0, 1],
                    children: btreemap![Response(2, 0, 0) => None]
                }),
            ],
        };

        assert_eq!(actual, expected);
    }

    #[bench]
    fn test_generate_exhausively(bencher: &mut Bencher) {
        let rank = |tree: &Tree| rank::by_depth(tree) as f64;
        bencher.iter(|| generate_exhaustively(2, 2, &rank))
    }
}
