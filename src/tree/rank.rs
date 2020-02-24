#[cfg(test)]
use crate::response::Response;
use crate::tree::Tree;

pub fn by_depth(tree: &Tree) -> u32 {
    let mut max = 1u32;

    for (_, child) in tree.children.iter() {
        match child {
            None       => continue,
            Some(tree) => {
                max = std::cmp::max(max, 1 + by_depth(tree))
            }
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_by_depth() {
        let arbitrary_guess = &&[0][..];

        assert_eq!(3, by_depth(
            &Tree {
                guess: arbitrary_guess,
                children: btreemap![
                    Response(2, 0, 0) => None,
                    Response(0, 0, 0) => Some(Tree {
                        guess: arbitrary_guess,
                        children: btreemap![
                            Response(2, 0, 0) => None
                        ]
                    }),
                    Response(1, 0, 0) => Some(Tree {
                        guess: arbitrary_guess,
                        children: btreemap![
                            Response(2, 0, 0) => None,
                            Response(1, 0, 0) => Some(Tree {
                                guess: arbitrary_guess,
                                children: btreemap![
                                    Response(2, 0, 0) => None
                                ]
                            })
                        ]
                    })
                ],
            }
        ));
    }

    #[test]
    fn test_by_depth_minimum_value() {
        assert_eq!(1, by_depth(
            &Tree { guess: &&[0][..], children: btreemap![] }
        ));
    }
}
