use std::iter;

use crate::response::Response;

pub type Code<'a> = &'a [usize];

pub fn universe<'a>(_length: usize, _base: usize) -> Vec<Code<'a>> {
    // todo
    vec![
        &[0, 0],
        &[0, 1],
        &[1, 0],
        &[1, 1],
    ]
}

pub fn compare(left: Code, right: Code) -> Response {
    let mut left_mask: Vec<bool> = iter::repeat(true).take(left.len())
        .collect();
    let mut result = Response(0, 0, 0);

    // count matching, "correct" digits
    for (index, value) in left.iter().enumerate() {
        if *value == right[index] {
            result.0 += 1;
            left_mask[index] = false;
        }
    }

    // count misplaced digits
    let mut right_mask = left_mask.clone();
    for (left_index, left_value) in left.iter().enumerate() {
        if !left_mask[left_index] {
            continue;
        }
        // search the right code for the first available matching digit
        for (right_index, right_value) in right.iter().enumerate() {
            if right_mask[right_index] && left_value == right_value {
                result.1 += 1;
                right_mask[right_index] = false;
                break;
            }
        }
    }

    // wrong digits are all that remain
    result.2 = left.len() - result.0 - result.1;

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_all_correct() {
        assert_eq!(
            compare(&[0], &[0]),
            Response(1, 0, 0),
            "Matching codes must generate a Response indicating that all \
            digits are equal"
        );
    }

    #[test]
    fn test_compare_none_correct() {
        assert_eq!(
            compare(&[0], &[1]),
            Response(0, 0, 1),
            "Disjoint codes must generate a Response indicating neither \
            correct nor misplaced digits"
        )
    }

    #[test]
    fn test_compare_misplaced_digits() {
        assert_eq!(
            compare(&[0, 1], &[1, 0]),
            Response(0, 2, 0),
            "Codes with equal digits in unequal positions must generate a 
            Response indicating the number of misplaced digits"
        )
    }

    #[test]
    fn test_compare_precedence() {
        assert_eq!(
            compare(&[0, 1], &[0, 0]),
            Response(1, 0, 1),
            "Any digit which matches exactly cannot count towards misplaced \
            digits in a Response (in this instance, the 0 on the left matches \
            a 0 on the right, so it cannot count as a misplaced match of the \
            other 0 on the right, as well)"
        )
    }

    #[test]
    fn test_compare_misplace_exhaustion() {
        assert_eq!(
            compare(&[0, 2, 2], &[1, 0, 0]),
            Response(0, 1, 2),
            "Any digit which counts towards the number of misplaced digits in \
            a Response may count only once (in this instance, the 0 on the \
            left matches only _one_ of the 0s on the right)"
        )
    }
}
