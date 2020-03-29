use std::collections::BTreeMap;
use std::collections::HashSet;

use crate::code;
use crate::code::Code;
use crate::response::Response;

#[derive(Debug)]
pub struct IsomorphCache {
    cache: HashSet<BTreeMap<Response, usize>> // len(answers)-by-response
}

impl IsomorphCache {
    pub fn new() -> IsomorphCache {
        IsomorphCache {
            cache: HashSet::new(),
        }
    }

    // This algorithm makes the assumption that we can detect two isomorphic
    // answers_by_response instances based on the Responses and numbers of
    // answers per response only. We assume this because the relationship of a
    // guess to a set of answers is what determines the structure (= depth) of a
    // tree. Equal responses with equally-sized answer sets imply we have
    // garnered the "same kind" of information from a guess. This assumes a
    // candidate guess is made at the same level of tree generation as a given
    // morph.
    pub fn is_new_morph(
        &mut self,
        morphology: &BTreeMap<Response, Vec<&Code>>
    ) -> bool {
        let value: BTreeMap<Response, usize> = morphology.iter()
            .map(|(k, v)| (k.clone(), v.len()))
            .collect();
        if self.cache.contains(&value) {
            false
        } else {
            self.cache.insert(value);
            true
        }
    }
}

pub fn answers_by_response<'a, I> (
    guess: &Code,
    answers: I,
) -> BTreeMap<Response, Vec<&'a Code>>
where I: Iterator<Item = &'a Code> {
    answers.fold(BTreeMap::new(), |mut map, answer| {
        map.entry(code::compare(guess, answer))
            .or_insert_with(Vec::new)
            .push(answer);
        map
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answers_by_response() {
        let c01: Code = vec![0, 1];
        let c10: Code = vec![1, 0];
        let c11: Code = vec![1, 1];

        assert_eq!(
            answers_by_response(
                &c11,
                vec![&c01, &c10, &c11].into_iter()),
            btreemap![
                Response(2, 0, 0) => vec![&c11],
                Response(1, 0, 1) => vec![&c01, &c10],
            ]);
    }

    #[test]
    fn test_isomorph_cache_is_new_morph() {
        let c00 = vec![0, 0];
        let c11 = vec![1, 1];
        let c22 = vec![2, 2];
        let universe = code::universe(2, 2);
        let answers: Vec<&Code> = universe.iter().collect();
        let mut cache = IsomorphCache::new();

        assert!(
            cache.is_new_morph(
                &answers_by_response(&c00, answers.iter().copied())),
            "The first entry of a morphology is always new");

        assert!(
            !cache.is_new_morph(
                &answers_by_response(&c11, answers.iter().copied())),
            "Guess 11 against answers [00, 01, 10, 11] is isomorphic to guess
            00, which is already cached.");

        assert!(
            cache.is_new_morph(
                &answers_by_response(&c22, answers.iter().copied())),
            "Guess 22 against answers [00, 01, 01, 11] is not isomorphic to
            guess 00, and so it is new.");
    }
}
