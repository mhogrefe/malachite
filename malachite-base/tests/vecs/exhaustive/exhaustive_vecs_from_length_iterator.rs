use itertools::Itertools;
use std::fmt::Debug;
use std::iter::empty;

use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::vecs::exhaustive::exhaustive_vecs_from_length_iterator;

fn exhaustive_vecs_from_element_iterator_helper<I: Iterator<Item = u64>, J: Clone + Iterator>(
    lengths: I,
    xs: J,
    out: &[&[J::Item]],
) where
    J::Item: Clone + Debug + Eq,
{
    let xss = exhaustive_vecs_from_length_iterator(lengths, xs)
        .take(20)
        .collect_vec();
    assert_eq!(xss.iter().map(Vec::as_slice).collect_vec().as_slice(), out);
}

#[test]
fn test_exhaustive_vecs_from_element_iterator() {
    exhaustive_vecs_from_element_iterator_helper(empty(), exhaustive_bools(), &[]);
    exhaustive_vecs_from_element_iterator_helper(
        [2, 1, 2].iter().cloned(),
        exhaustive_bools(),
        &[
            &[false, false],
            &[false],
            &[false, true],
            &[false, false],
            &[true, false],
            &[true],
            &[true, true],
            &[false, true],
            &[true, false],
            &[true, true],
        ],
    );
    exhaustive_vecs_from_element_iterator_helper(
        exhaustive_unsigneds::<u64>().map(|u| u << 1),
        exhaustive_bools(),
        &[
            &[],
            &[false, false],
            &[false, true],
            &[false, false, false, false, false, false],
            &[true, false],
            &[false, false, false, false],
            &[true, true],
            &[false, false, false, false, false, false, false, false],
            &[false, false, false, true],
            &[false, false, false, false, false, true],
            &[false, false, true, false],
            &[false, false, false, false, false, false, false, true],
            &[false, false, true, true],
            &[false, false, false, false, true, false],
            &[false, true, false, false],
            &[
                false, false, false, false, false, false, false, false, false, false, false, false,
            ],
            &[false, true, false, true],
            &[false, false, false, false, true, true],
            &[false, true, true, false],
            &[false, false, false, false, false, false, true, false],
        ],
    );
    exhaustive_vecs_from_element_iterator_helper(
        [2, 1, 0, 2].iter().cloned(),
        exhaustive_bools(),
        &[
            &[false, false],
            &[false],
            &[false, true],
            &[],
            &[true, false],
            &[true],
            &[true, true],
            &[false, false],
            &[false, true],
            &[true, false],
            &[true, true],
        ],
    );
    exhaustive_vecs_from_element_iterator_helper(empty(), exhaustive_unsigneds::<u32>(), &[]);
    exhaustive_vecs_from_element_iterator_helper(
        [2, 1, 2].iter().cloned(),
        exhaustive_unsigneds::<u32>(),
        &[
            &[0, 0],
            &[0],
            &[0, 1],
            &[0, 0],
            &[1, 0],
            &[1],
            &[1, 1],
            &[0, 2],
            &[0, 3],
            &[2],
            &[1, 2],
            &[0, 1],
            &[1, 3],
            &[3],
            &[2, 0],
            &[4],
            &[2, 1],
            &[5],
            &[3, 0],
            &[1, 0],
        ],
    );
    exhaustive_vecs_from_element_iterator_helper(
        exhaustive_unsigneds::<u64>().map(|u| u << 1),
        exhaustive_unsigneds::<u32>(),
        &[
            &[],
            &[0, 0],
            &[0, 1],
            &[0, 0, 0, 0, 0, 0],
            &[1, 0],
            &[0, 0, 0, 0],
            &[1, 1],
            &[0, 0, 0, 0, 0, 0, 0, 0],
            &[0, 2],
            &[0, 0, 0, 1],
            &[0, 3],
            &[0, 0, 0, 0, 0, 1],
            &[1, 2],
            &[0, 0, 1, 0],
            &[1, 3],
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            &[2, 0],
            &[0, 0, 1, 1],
            &[2, 1],
            &[0, 0, 0, 0, 1, 0],
        ],
    );
    exhaustive_vecs_from_element_iterator_helper(
        [2, 1, 0, 2].iter().cloned(),
        exhaustive_unsigneds::<u32>(),
        &[
            &[0, 0],
            &[0],
            &[0, 1],
            &[],
            &[1, 0],
            &[1],
            &[1, 1],
            &[0, 0],
            &[0, 2],
            &[2],
            &[0, 3],
            &[0, 1],
            &[1, 2],
            &[3],
            &[1, 3],
            &[4],
            &[2, 0],
            &[5],
            &[2, 1],
            &[1, 0],
        ],
    );
    // Stops after first empty ys
    exhaustive_vecs_from_element_iterator_helper(
        [0, 0, 1, 0].iter().cloned(),
        nevers(),
        &[&[], &[]],
    );
}
