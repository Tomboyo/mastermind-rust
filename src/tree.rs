pub mod rank;

use std::collections::BTreeMap;

use crate::code::Code;
use crate::response::Response;

#[derive(Debug, PartialEq, Eq)]
pub struct Tree<'a> {
    guess: &'a Code<'a>,
    children: BTreeMap<Response, Option<Tree<'a>>>,
}

pub fn generate<'a, F>(
    guesses: Vec<&'a Code>,
    answers: Vec<&'a Code>,
    rank: F
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
            let map = answers_by_response(&guess, &answers);

            Tree { guess: guess, children: BTreeMap::new() }
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
            map.entry(guess.compare(answer))
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
        let universe = Code::universe(2, 2);
        // prefer trees based on their guess; 0,0 is "best".
        let rank = |tree: &Tree| match tree.guess {
            Code { data: [0, 0] } => 0f64,
            Code { data: [0, 1] } => 1f64,
            Code { data: [1, 0] } => 2f64,
            Code { data: [1, 1] } => 3f64,
            x => panic!("Unexpected test code {:?}", x)
        };

        let actual = generate(
            universe.iter().collect(), // guesses = universe
            universe.iter().collect(), // answers = universe,
            rank
        );
        
        /*
        Expect the following tree
        (00, { (2,0) => (),
               (0,0) => (11, { (2,0) => () }),
               (1,0) => (01, { (2,0) => (),
                               (0,2) => (10, { (2,0) => () })
                             })})
        */
        // todo: consider tree owning code and deriving clone for code
        let c11 = Code::new(&[1, 1]);
        let c01 = Code::new(&[0, 1]);
        let c10 = Code::new(&[1, 0]);
        let expected = Tree {
            guess: &Code::new(&[0, 0]),
            children: btreemap![
                Response(2, 0) => None,
                Response(0, 0) => Some(Tree {
                    guess: &c11,
                    children: btreemap![
                        Response(2, 0) => None,
                    ]
                }),
                Response(1, 0) => Some(Tree {
                    guess: &c01,
                    children: btreemap![
                        Response(2, 0) => None,
                        Response(0, 2) => Some(Tree {
                            guess: &c10,
                            children: btreemap![
                                Response(2, 0) => None,
                            ]
                        })
                    ]
                }),
            ],
        };

        assert_eq!(actual, expected);
    }
}
