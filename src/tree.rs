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
where F: Fn(&RefTree) -> f64 {
    let universe = code::universe(code_length, code_base);
    generate(
        universe.iter().collect(),
        universe.iter().collect(),
        rank
    ).to_tree()
}

fn generate<'a, F>(
    guesses: Vec<&'a Code>,
    answers: Vec<&'a Code>,
    rank: &F
) -> RefTree<'a>
where F: Fn(&RefTree<'a>) -> f64 {
    let mut cache = morphology::IsomorphCache::new();
    let mut best = None;
    let mut best_rank = (answers.len() + 1) as f64;

    for guess in &guesses {
        let morph = morphology::answers_by_response(
            guess,
            answers.iter().copied());
        
        if !cache.is_new_morph(&morph) {
            continue;
        }

        let candidate = subtree(guess, &guesses, morph, rank, best_rank);
        if let Some(x) = candidate {
            let candidate_rank = rank(&x);
            if candidate_rank < best_rank {
                best = Some(x);
                best_rank = candidate_rank;
            }
        }
    }
    
    best.expect("There should be at least one tree")
}

// TODO: RefTree::new(...)?
// calculate the children for each answers-by-response group given a set of
// guesses. If any of the childrens' rank is >= the threshold, derivation halts
// and an empty is returned.
fn subtree<'a, F>(
    guess: &'a Code,
    guesses: &[&'a Code],
    morph: BTreeMap<Response, Vec<&'a Code>>,
    rank: &F,
    threshold: f64
) -> Option<RefTree<'a>>
where F: Fn(&RefTree<'a>) ->f64 {
    let mut children = BTreeMap::new();
    for (response, remaining_answers) in morph {
        if response::is_correct(&response) {
            children.insert(response, None);
        } else {
            let remaining_guesses = guesses.iter()
                .cloned()
                .filter(|x| *x != guess)
                .collect();
            let child = generate(
                remaining_guesses,
                remaining_answers,
                rank);
            if rank(&child) + 1.0 < threshold {
                children.insert(response, Some(child));
            } else {
                // A best worst-case is already too bad, so quit now
                return None;
            }
        }
    }
    Some(RefTree { guess, children })
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
            &[0, 0] => 0f64,
            &[0, 1] => 1f64,
            x => panic!("Unexpected test code {:?}", x)
        };

        let actual = generate(
            vec![&c00, &c01],
            vec![&c00, &c01],
            &rank
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

        assert_eq!(actual, expected);
    }

    #[bench]
    fn test_generate_exhausively_2_2(bencher: &mut Bencher) {
        let rank = |tree: &RefTree| rank::by_depth(tree) as f64;
        bencher.iter(|| generate_exhaustively(2, 2, &rank))
    }

    #[bench]
    fn test_generate_exhausively_3_2(bencher: &mut Bencher) {
        let rank = |tree: &RefTree| rank::by_depth(tree) as f64;
        bencher.iter(|| generate_exhaustively(3, 2, &rank))
    }
}
