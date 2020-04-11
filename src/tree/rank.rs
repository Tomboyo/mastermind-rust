use crate::tree::RefTree;
#[cfg(test)] use crate::response::Response;

pub fn by_depth(tree: &RefTree) -> usize {
    tree.children.values().fold(1, |max, child| {
        match child {
            None => max,
            Some(x) => std::cmp::max(max, 1 + by_depth(x)),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_by_depth() {
        let arbitrary = vec![0];
        assert_eq!(3, by_depth(
            &RefTree {
                guess: &arbitrary,
                children: btreemap![
                    Response(2, 0, 0) => None,
                    Response(0, 0, 0) => Some(RefTree {
                        guess: &arbitrary,
                        children: btreemap![
                            Response(2, 0, 0) => None
                        ]
                    }),
                    Response(1, 0, 0) => Some(RefTree {
                        guess: &arbitrary,
                        children: btreemap![
                            Response(2, 0, 0) => None,
                            Response(1, 0, 0) => Some(RefTree {
                                guess: &arbitrary,
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
            &RefTree { guess: &vec![0], children: btreemap![] }
        ));
    }
}
