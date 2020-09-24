use std::fmt::Debug;

use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::exhaustive_fixed_length_vecs_from_single;

fn exhaustive_fixed_length_vecs_from_single_helper<I: Iterator>(
    len: usize,
    xs: I,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    let xss = exhaustive_fixed_length_vecs_from_single(len, xs)
        .take(20)
        .collect::<Vec<_>>();
    assert_eq!(
        xss.iter().map(Vec::as_slice).collect::<Vec<_>>().as_slice(),
        out
    );
}

#[test]
fn test_exhaustive_fixed_length_vecs_from_single() {
    // This demonstrates that 0 ^ 0 == 1:
    exhaustive_fixed_length_vecs_from_single_helper(0, nevers(), &[&[]]);
    exhaustive_fixed_length_vecs_from_single_helper(1, nevers(), &[]);
    exhaustive_fixed_length_vecs_from_single_helper(2, nevers(), &[]);
    exhaustive_fixed_length_vecs_from_single_helper(5, nevers(), &[]);
    exhaustive_fixed_length_vecs_from_single_helper(1, exhaustive_units(), &[&[()]]);
    exhaustive_fixed_length_vecs_from_single_helper(2, exhaustive_units(), &[&[(), ()]]);
    exhaustive_fixed_length_vecs_from_single_helper(5, exhaustive_units(), &[&[(); 5]]);
    exhaustive_fixed_length_vecs_from_single_helper(0, exhaustive_unsigneds::<u8>(), &[&[]]);
    exhaustive_fixed_length_vecs_from_single_helper(
        1,
        exhaustive_unsigneds::<u8>(),
        &[
            &[0],
            &[1],
            &[2],
            &[3],
            &[4],
            &[5],
            &[6],
            &[7],
            &[8],
            &[9],
            &[10],
            &[11],
            &[12],
            &[13],
            &[14],
            &[15],
            &[16],
            &[17],
            &[18],
            &[19],
        ],
    );
    exhaustive_fixed_length_vecs_from_single_helper(
        2,
        exhaustive_unsigneds::<u8>(),
        &[
            &[0, 0],
            &[0, 1],
            &[1, 0],
            &[1, 1],
            &[0, 2],
            &[0, 3],
            &[1, 2],
            &[1, 3],
            &[2, 0],
            &[2, 1],
            &[3, 0],
            &[3, 1],
            &[2, 2],
            &[2, 3],
            &[3, 2],
            &[3, 3],
            &[0, 4],
            &[0, 5],
            &[1, 4],
            &[1, 5],
        ],
    );
    exhaustive_fixed_length_vecs_from_single_helper(
        3,
        exhaustive_unsigneds::<u8>(),
        &[
            &[0, 0, 0],
            &[0, 0, 1],
            &[0, 1, 0],
            &[0, 1, 1],
            &[1, 0, 0],
            &[1, 0, 1],
            &[1, 1, 0],
            &[1, 1, 1],
            &[0, 0, 2],
            &[0, 0, 3],
            &[0, 1, 2],
            &[0, 1, 3],
            &[1, 0, 2],
            &[1, 0, 3],
            &[1, 1, 2],
            &[1, 1, 3],
            &[0, 2, 0],
            &[0, 2, 1],
            &[0, 3, 0],
            &[0, 3, 1],
        ],
    );
    exhaustive_fixed_length_vecs_from_single_helper(
        2,
        exhaustive_ascii_chars(),
        &[
            &['a', 'a'],
            &['a', 'b'],
            &['b', 'a'],
            &['b', 'b'],
            &['a', 'c'],
            &['a', 'd'],
            &['b', 'c'],
            &['b', 'd'],
            &['c', 'a'],
            &['c', 'b'],
            &['d', 'a'],
            &['d', 'b'],
            &['c', 'c'],
            &['c', 'd'],
            &['d', 'c'],
            &['d', 'd'],
            &['a', 'e'],
            &['a', 'f'],
            &['b', 'e'],
            &['b', 'f'],
        ],
    );
    exhaustive_fixed_length_vecs_from_single_helper(1, exhaustive_bools(), &[&[false], &[true]]);
    exhaustive_fixed_length_vecs_from_single_helper(
        2,
        exhaustive_bools(),
        &[
            &[false, false],
            &[false, true],
            &[true, false],
            &[true, true],
        ],
    );
    exhaustive_fixed_length_vecs_from_single_helper(
        4,
        exhaustive_bools(),
        &[
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
            &[true, true, true, true],
        ],
    );
}
