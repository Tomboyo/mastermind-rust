pub mod rank;

use std::collections::BTreeMap;

use crate::code;
use crate::code::Code;
use crate::response;
use crate::response::Response;

#[derive(Debug, PartialEq, Eq)]
pub struct Tree<'a> {
    guess: &'a Code<'a>,
    children: BTreeMap<Response, Option<Tree<'a>>>,
}

pub fn generate<'a, F>(
    guesses: &Vec<&'a Code>,
    answers: &Vec<&'a Code>,
    rank: &F
) -> Tree<'a>
where F: Fn(&Tree) -> f64 {
    guesses.iter()
        // map each guess to (guess, answers-by-response)
        // map each (g, abr) to a tree
        //    map abr to a collection
        //        (term)  if r is correct, append an empty tree
        //        (recur) otherwise, Tree { guess, generate(...) }
        // then reduce the trees down to the optimal one.
        .map(|guess| {
            let mut children = BTreeMap::new();
            for (response, remaining_answers) in answers_by_response(&guess, &answers) {
                if response::is_correct(&response) {
                    // terminal case
                    children.insert(response, None);
                } else {
                    // recursive case
                    let remaining_guesses = guesses.iter()
                        .map(|x| *x)
                        .filter(|x| x != guess)
                        .collect();
                    children.insert(
                        response,
                        Some(generate(
                            &remaining_guesses,
                            &remaining_answers,
                            rank)));
                }
            }
            
            Tree { guess, children }
        }) //dummy
        .fold(None, |acc, tree| match acc {
            None => Some(tree),
            Some(best) => Some(select(best, tree, &rank))
        })
        .expect("The should be at least one tree")
}

fn answers_by_response<'a>(
    guess: &Code,
    answers: &Vec<&'a Code>,
) -> BTreeMap<Response, Vec<&'a Code<'a>>> {
    answers.iter()
        .fold(BTreeMap::new(), |mut map, answer| {
            map.entry(code::compare(guess, answer))
                .or_insert(Vec::new())
                .push(answer);
            map
        })
}

pub fn select<'a, F>(
    left: Tree<'a>,
    right: Tree<'a>,
    rank: &F
) -> Tree<'a>
where F: Fn(&Tree) -> f64 {
    if rank(&left) <= rank(&right) {
        left
    } else {
        right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let c00 = &[0, 0][..];
        let c01 = &[0, 1][..];

        // prefer trees based on their guess; 0,0 is "best".
        let rank = |tree: &Tree| match tree.guess {
            &[0, 0] => 0f64,
            &[0, 1] => 1f64,
            x => panic!("Unexpected test code {:?}", x)
        };

        let actual = generate(
            &vec![&c00, &c01],
            &vec![&c00, &c01],
            &rank
        );

        let expected = Tree {
            guess: &c00,
            children: btreemap![
                Response(2, 0, 0) => None,
                Response(1, 0, 1) => Some(Tree {
                    guess: &c01,
                    children: btreemap![Response(2, 0, 0) => None]
                }),
            ],
        };

        assert_eq!(actual, expected);
    }
}
