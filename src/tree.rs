pub mod rank;
mod morphology;

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

#[derive(Debug, PartialEq, Eq)]
pub struct RefTree<'a> {
    guess: &'a Code,
    children: BTreeMap<Response, Option<RefTree<'a>>>,
}

pub fn generate_exhaustively<F>(
    code_length: usize,
    code_base: usize,
    rank: &F,
) -> Tree
where F: Fn(&RefTree) -> usize {
    let universe = code::universe(code_length, code_base);
    generate(
        universe.iter().collect(),
        universe.iter().collect(),
        rank,
        universe.len() + 1 // sentinel -- larger than any optimal tree
    )
    .expect("There should be at least one tree")
    .to_tree()
}

/// guesses: list of codes available to guess
/// answers: list of codes which may be answers based on guesses made so far
/// rank: a function to rank a tree by its depth, lower numbers are better.
/// optimal_rank: A ranking which must be beaten by any candidate subtree. If a
///     subtree meets or exceeds this rank, it is discarded. If no trees can
///     meet this rank, we return None.
fn generate<'a, F>(
    guesses: Vec<&'a Code>,
    answers: Vec<&'a Code>,
    rank: &F,
    optimal_rank: usize
) -> Option<RefTree<'a>>
where F: Fn(&RefTree<'a>) -> usize {
    if optimal_rank == 0 {
        // In order for the parent tree to beat the optimal_rank, this subtree
        // must already have differentiated all answers, which is impossible.
        return None
    }

    let mut cache = morphology::IsomorphCache::new();
    // local_optimal_rank <= optimal_rank and shrinks as optimal trees are found
    let mut local_optimal_rank = optimal_rank;
    let mut optimal_tree = None;

    'guesses: for guess in &guesses {
        let morph = morphology::answers_by_response(
            guess,
            answers.iter().copied());
        
        if !cache.is_new_morph(&morph) {
            continue;
        }

        let mut children = BTreeMap::new();
        for (response, remaining_answers) in morph {
            if response::is_correct(&response) {
                children.insert(response, None);
            } else {
                let remaining_guesses = guesses.iter()
                    .cloned()
                    .filter(|x| x != guess)
                    .collect();
                let child = generate(
                    remaining_guesses,
                    remaining_answers,
                    rank,
                    local_optimal_rank - 1);

                // - If generate(...) returns None, then the optimal worst-case
                // tree decending the current tree exceeds the desired rank, and
                // therefore the current tree rooted at `guess` does as well. we
                // do not need to evaluate further answer-by-response groups.
                // - Otherwise if generate(...) returns Some, then we need to
                // continue evaluating answer-by-response groups to determine if
                // this tree is optimal or not.
                match child {
                    None => continue 'guesses,
                    Some(_) => children.insert(response, child),
                };
            }
        }
        
        let candidate = RefTree { guess, children };
        let candidate_rank = rank(&candidate);
        if candidate_rank < local_optimal_rank {
            optimal_tree = Some(candidate);
            // Shrink local_optimal_rank to save time evaluating remaining
            // guesses
            local_optimal_rank = candidate_rank;
        }
    }

    optimal_tree
}

impl<'a> RefTree<'a> {
    fn to_tree(&self) -> Tree {
        Tree {
            guess: self.guess.to_vec(),
            children: self.children.iter()
                .map(|(response, opt_ref_tree)| match opt_ref_tree {
                    None => (response.clone(), None),
                    Some(ref_tree) => (response.clone(), Some(ref_tree.to_tree())),
                })
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;
    use crate::tree::rank;

    #[test]
    fn test_generate() {
        let c00 = vec![0, 0];
        let c01 = vec![0, 1];

        // prefer trees based on their guess; 0,0 is "best".
        let rank = |tree: &RefTree| match &tree.guess[..] {
            &[0, 0] => 0,
            &[0, 1] => 1,
            x => panic!("Unexpected test code {:?}", x)
        };

        let actual = generate(
            vec![&c00, &c01],
            vec![&c00, &c01],
            &rank,
            3
        );

        let expected = RefTree {
            guess: &c00,
            children: btreemap![
                Response(2, 0, 0) => None,
                Response(1, 0, 1) => Some(RefTree {
                    guess: &c01,
                    children: btreemap![Response(2, 0, 0) => None]
                }),
            ],
        };

        assert_eq!(actual, Some(expected));
    }

    #[bench]
    fn test_generate_exhaustively_2_2(bencher: &mut Bencher) {
        let rank = |tree: &RefTree| rank::by_depth(tree);
        bencher.iter(|| generate_exhaustively(2, 2, &rank))
    }

    #[bench]
    fn test_generate_exhaustively_3_2(bencher: &mut Bencher) {
        let rank = |tree: &RefTree| rank::by_depth(tree);
        bencher.iter(|| generate_exhaustively(3, 2, &rank))
    }

    #[bench]
    fn test_generate_exhaustively_3_3(bencher: &mut Bencher) {
        let rank = |tree: &RefTree| rank::by_depth(tree);
        bencher.iter(|| generate_exhaustively(3, 3, &rank))
    }
}
