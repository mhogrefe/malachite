use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::exhaustive_fixed_length_vecs_from_single;
use std::fmt::Debug;

fn exhaustive_fixed_length_vecs_from_single_helper<I: Iterator>(len: u64, xs: I, out: &[&[I::Item]])
where
    I::Item: Clone + Debug + Eq,
{
    let xss = exhaustive_fixed_length_vecs_from_single(len, xs)
        .take(20)
        .collect_vec();
    assert_eq!(xss.iter().map(Vec::as_slice).collect_vec().as_slice(), out);
}

fn exhaustive_fixed_length_vecs_from_single_finite_helper<I: Clone + Iterator>(
    len: u64,
    xs: I,
    out_len: usize,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    let xss = exhaustive_fixed_length_vecs_from_single(len, xs);
    let xss_prefix = xss.clone().take(20).collect_vec();
    assert_eq!(
        xss_prefix
            .iter()
            .map(Vec::as_slice)
            .collect_vec()
            .as_slice(),
        out
    );
    assert_eq!(xss.count(), out_len);
}

#[test]
fn test_exhaustive_fixed_length_vecs_from_single() {
    // This demonstrates that 0 ^ 0 == 1:
    exhaustive_fixed_length_vecs_from_single_finite_helper(0, nevers(), 1, &[&[]]);
    exhaustive_fixed_length_vecs_from_single_finite_helper(1, nevers(), 0, &[]);
    exhaustive_fixed_length_vecs_from_single_finite_helper(2, nevers(), 0, &[]);
    exhaustive_fixed_length_vecs_from_single_finite_helper(5, nevers(), 0, &[]);
    exhaustive_fixed_length_vecs_from_single_finite_helper(1, exhaustive_units(), 1, &[&[()]]);
    exhaustive_fixed_length_vecs_from_single_finite_helper(2, exhaustive_units(), 1, &[&[(), ()]]);
    exhaustive_fixed_length_vecs_from_single_finite_helper(5, exhaustive_units(), 1, &[&[(); 5]]);
    exhaustive_fixed_length_vecs_from_single_finite_helper(
        0,
        exhaustive_unsigneds::<u8>(),
        1,
        &[&[]],
    );
    exhaustive_fixed_length_vecs_from_single_finite_helper(
        1,
        exhaustive_unsigneds::<u8>(),
        256,
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
        1,
        exhaustive_unsigneds::<u64>(),
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
    exhaustive_fixed_length_vecs_from_single_finite_helper(
        2,
        exhaustive_ascii_chars(),
        0x4000,
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
    exhaustive_fixed_length_vecs_from_single_finite_helper(
        1,
        exhaustive_bools(),
        2,
        &[&[false], &[true]],
    );
    exhaustive_fixed_length_vecs_from_single_finite_helper(
        2,
        exhaustive_bools(),
        4,
        &[
            &[false, false],
            &[false, true],
            &[true, false],
            &[true, true],
        ],
    );
    exhaustive_fixed_length_vecs_from_single_finite_helper(
        4,
        exhaustive_bools(),
        16,
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
    exhaustive_fixed_length_vecs_from_single_finite_helper(
        10,
        exhaustive_bools(),
        1024,
        &[
            &[
                false, false, false, false, false, false, false, false, false, false,
            ],
            &[
                false, false, false, false, false, false, false, false, false, true,
            ],
            &[
                false, false, false, false, false, false, false, false, true, false,
            ],
            &[
                false, false, false, false, false, false, false, false, true, true,
            ],
            &[
                false, false, false, false, false, false, false, true, false, false,
            ],
            &[
                false, false, false, false, false, false, false, true, false, true,
            ],
            &[
                false, false, false, false, false, false, false, true, true, false,
            ],
            &[
                false, false, false, false, false, false, false, true, true, true,
            ],
            &[
                false, false, false, false, false, false, true, false, false, false,
            ],
            &[
                false, false, false, false, false, false, true, false, false, true,
            ],
            &[
                false, false, false, false, false, false, true, false, true, false,
            ],
            &[
                false, false, false, false, false, false, true, false, true, true,
            ],
            &[
                false, false, false, false, false, false, true, true, false, false,
            ],
            &[
                false, false, false, false, false, false, true, true, false, true,
            ],
            &[
                false, false, false, false, false, false, true, true, true, false,
            ],
            &[
                false, false, false, false, false, false, true, true, true, true,
            ],
            &[
                false, false, false, false, false, true, false, false, false, false,
            ],
            &[
                false, false, false, false, false, true, false, false, false, true,
            ],
            &[
                false, false, false, false, false, true, false, false, true, false,
            ],
            &[
                false, false, false, false, false, true, false, false, true, true,
            ],
        ],
    );
    exhaustive_fixed_length_vecs_from_single_finite_helper(
        10,
        0..3,
        59049,
        &[
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            &[0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
            &[0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
            &[0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
            &[0, 0, 0, 0, 0, 0, 0, 1, 1, 0],
            &[0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
            &[0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
            &[0, 0, 0, 0, 0, 0, 1, 0, 0, 1],
            &[0, 0, 0, 0, 0, 0, 1, 0, 1, 0],
            &[0, 0, 0, 0, 0, 0, 1, 0, 1, 1],
            &[0, 0, 0, 0, 0, 0, 1, 1, 0, 0],
            &[0, 0, 0, 0, 0, 0, 1, 1, 0, 1],
            &[0, 0, 0, 0, 0, 0, 1, 1, 1, 0],
            &[0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
            &[0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
            &[0, 0, 0, 0, 0, 1, 0, 0, 0, 1],
            &[0, 0, 0, 0, 0, 1, 0, 0, 1, 0],
            &[0, 0, 0, 0, 0, 1, 0, 0, 1, 1],
        ],
    );
    exhaustive_fixed_length_vecs_from_single_helper(
        2,
        exhaustive_fixed_length_vecs_from_single(2, exhaustive_unsigneds::<u8>()),
        &[
            &[vec![0, 0], vec![0, 0]],
            &[vec![0, 0], vec![0, 1]],
            &[vec![0, 1], vec![0, 0]],
            &[vec![0, 1], vec![0, 1]],
            &[vec![0, 0], vec![1, 0]],
            &[vec![0, 0], vec![1, 1]],
            &[vec![0, 1], vec![1, 0]],
            &[vec![0, 1], vec![1, 1]],
            &[vec![1, 0], vec![0, 0]],
            &[vec![1, 0], vec![0, 1]],
            &[vec![1, 1], vec![0, 0]],
            &[vec![1, 1], vec![0, 1]],
            &[vec![1, 0], vec![1, 0]],
            &[vec![1, 0], vec![1, 1]],
            &[vec![1, 1], vec![1, 0]],
            &[vec![1, 1], vec![1, 1]],
            &[vec![0, 0], vec![0, 2]],
            &[vec![0, 0], vec![0, 3]],
            &[vec![0, 1], vec![0, 2]],
            &[vec![0, 1], vec![0, 3]],
        ],
    );
}
