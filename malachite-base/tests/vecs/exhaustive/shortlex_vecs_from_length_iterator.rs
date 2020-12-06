use std::fmt::Debug;
use std::iter::empty;

use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::vecs::exhaustive::shortlex_vecs_from_length_iterator;

fn shortlex_vecs_from_element_iterator_helper<I: Iterator<Item = u64>, J: Clone + Iterator>(
    lengths: I,
    xs: J,
    out: &[&[J::Item]],
) where
    J::Item: Clone + Debug + Eq,
{
    let xss = shortlex_vecs_from_length_iterator(lengths, xs)
        .take(20)
        .collect::<Vec<_>>();
    assert_eq!(
        xss.iter().map(Vec::as_slice).collect::<Vec<_>>().as_slice(),
        out
    );
}

#[test]
fn test_shortlex_vecs_from_element_iterator() {
    shortlex_vecs_from_element_iterator_helper(empty(), exhaustive_bools(), &[]);
    shortlex_vecs_from_element_iterator_helper(
        [2, 1, 2].iter().cloned(),
        exhaustive_bools(),
        &[
            &[false, false],
            &[false, true],
            &[true, false],
            &[true, true],
            &[false],
            &[true],
            &[false, false],
            &[false, true],
            &[true, false],
            &[true, true],
        ],
    );
    shortlex_vecs_from_element_iterator_helper(
        exhaustive_unsigneds::<u64>().map(|u| u << 1),
        exhaustive_bools(),
        &[
            &[],
            &[false, false],
            &[false, true],
            &[true, false],
            &[true, true],
            &[false, false, false, false],
            &[false, false, false, true],
            &[false, false, true, false],
            &[false, false, true, true],
            &[false, true, false, false],
            &[false, true, false, true],
            &[false, true, true, false],
            &[false, true, true, true],
            &[true, false, false, false],
            &[true, false, false, true],
            &[true, false, true, false],
            &[true, false, true, true],
            &[true, true, false, false],
            &[true, true, false, true],
            &[true, true, true, false],
        ],
    );
    shortlex_vecs_from_element_iterator_helper(
        [2, 1, 0, 2].iter().cloned(),
        exhaustive_bools(),
        &[
            &[false, false],
            &[false, true],
            &[true, false],
            &[true, true],
            &[false],
            &[true],
            &[],
            &[false, false],
            &[false, true],
            &[true, false],
            &[true, true],
        ],
    );
    // Stops after first empty ys
    shortlex_vecs_from_element_iterator_helper([0, 0, 1, 0].iter().cloned(), nevers(), &[&[], &[]]);
}
