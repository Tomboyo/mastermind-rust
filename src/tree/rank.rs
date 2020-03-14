use num_traits::ToPrimitive;

use crate::tree::RefTree;
#[cfg(test)] use crate::response::Response;

pub fn by_depth(tree: &RefTree) -> f64 {
    by_depth_u32(tree).to_f64().unwrap() // todo
}

fn by_depth_u32(tree: &RefTree) -> u32 {
    let mut max = 1u32;

    for (_, child) in tree.children.iter() {
        match child {
            None       => continue,
            Some(tree) => {
                max = std::cmp::max(max, 1 + by_depth_u32(tree))
            }
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_by_depth_u32() {
        let arbitrary = vec![0];
        assert_eq!(3u32, by_depth_u32(
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
    fn test_by_depth_u32_minimum_value() {
        assert_eq!(1u32, by_depth_u32(
            &RefTree { guess: &vec![0], children: btreemap![] }
        ));
    }
}
