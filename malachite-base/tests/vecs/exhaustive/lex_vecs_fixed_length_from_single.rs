use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::lex_vecs_fixed_length_from_single;
use std::fmt::Debug;

fn lex_vecs_fixed_length_from_single_helper<I: Iterator>(len: u64, xs: I, out: &[&[I::Item]])
where
    I::Item: Clone + Debug + Eq,
{
    let xss = lex_vecs_fixed_length_from_single(len, xs)
        .take(20)
        .collect_vec();
    assert_eq!(xss.iter().map(Vec::as_slice).collect_vec().as_slice(), out);
}

fn lex_vecs_fixed_length_from_single_small_helper<I: Clone + Iterator>(
    len: u64,
    xs: I,
    out_len: usize,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    let xss = lex_vecs_fixed_length_from_single(len, xs);
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
fn test_lex_vecs_fixed_length_from_single() {
    // This demonstrates that 0 ^ 0 == 1:
    lex_vecs_fixed_length_from_single_small_helper(0, nevers(), 1, &[&[]]);
    lex_vecs_fixed_length_from_single_small_helper(1, nevers(), 0, &[]);
    lex_vecs_fixed_length_from_single_small_helper(2, nevers(), 0, &[]);
    lex_vecs_fixed_length_from_single_small_helper(5, nevers(), 0, &[]);
    lex_vecs_fixed_length_from_single_small_helper(1, exhaustive_units(), 1, &[&[()]]);
    lex_vecs_fixed_length_from_single_small_helper(2, exhaustive_units(), 1, &[&[(), ()]]);
    lex_vecs_fixed_length_from_single_small_helper(5, exhaustive_units(), 1, &[&[(); 5]]);
    lex_vecs_fixed_length_from_single_small_helper(0, exhaustive_unsigneds::<u8>(), 1, &[&[]]);
    lex_vecs_fixed_length_from_single_small_helper(
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
    lex_vecs_fixed_length_from_single_helper(
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
    lex_vecs_fixed_length_from_single_small_helper(
        2,
        exhaustive_unsigneds::<u8>(),
        0x10000,
        &[
            &[0, 0],
            &[0, 1],
            &[0, 2],
            &[0, 3],
            &[0, 4],
            &[0, 5],
            &[0, 6],
            &[0, 7],
            &[0, 8],
            &[0, 9],
            &[0, 10],
            &[0, 11],
            &[0, 12],
            &[0, 13],
            &[0, 14],
            &[0, 15],
            &[0, 16],
            &[0, 17],
            &[0, 18],
            &[0, 19],
        ],
    );
    lex_vecs_fixed_length_from_single_helper(
        3,
        exhaustive_unsigneds::<u8>(),
        &[
            &[0, 0, 0],
            &[0, 0, 1],
            &[0, 0, 2],
            &[0, 0, 3],
            &[0, 0, 4],
            &[0, 0, 5],
            &[0, 0, 6],
            &[0, 0, 7],
            &[0, 0, 8],
            &[0, 0, 9],
            &[0, 0, 10],
            &[0, 0, 11],
            &[0, 0, 12],
            &[0, 0, 13],
            &[0, 0, 14],
            &[0, 0, 15],
            &[0, 0, 16],
            &[0, 0, 17],
            &[0, 0, 18],
            &[0, 0, 19],
        ],
    );
    lex_vecs_fixed_length_from_single_small_helper(
        2,
        exhaustive_ascii_chars(),
        0x4000,
        &[
            &['a', 'a'],
            &['a', 'b'],
            &['a', 'c'],
            &['a', 'd'],
            &['a', 'e'],
            &['a', 'f'],
            &['a', 'g'],
            &['a', 'h'],
            &['a', 'i'],
            &['a', 'j'],
            &['a', 'k'],
            &['a', 'l'],
            &['a', 'm'],
            &['a', 'n'],
            &['a', 'o'],
            &['a', 'p'],
            &['a', 'q'],
            &['a', 'r'],
            &['a', 's'],
            &['a', 't'],
        ],
    );
    lex_vecs_fixed_length_from_single_small_helper(1, exhaustive_bools(), 2, &[&[false], &[true]]);
    lex_vecs_fixed_length_from_single_small_helper(
        2,
        exhaustive_bools(),
        4,
        &[&[false, false], &[false, true], &[true, false], &[true, true]],
    );
    lex_vecs_fixed_length_from_single_small_helper(
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
    lex_vecs_fixed_length_from_single_small_helper(
        10,
        exhaustive_bools(),
        1024,
        &[
            &[false, false, false, false, false, false, false, false, false, false],
            &[false, false, false, false, false, false, false, false, false, true],
            &[false, false, false, false, false, false, false, false, true, false],
            &[false, false, false, false, false, false, false, false, true, true],
            &[false, false, false, false, false, false, false, true, false, false],
            &[false, false, false, false, false, false, false, true, false, true],
            &[false, false, false, false, false, false, false, true, true, false],
            &[false, false, false, false, false, false, false, true, true, true],
            &[false, false, false, false, false, false, true, false, false, false],
            &[false, false, false, false, false, false, true, false, false, true],
            &[false, false, false, false, false, false, true, false, true, false],
            &[false, false, false, false, false, false, true, false, true, true],
            &[false, false, false, false, false, false, true, true, false, false],
            &[false, false, false, false, false, false, true, true, false, true],
            &[false, false, false, false, false, false, true, true, true, false],
            &[false, false, false, false, false, false, true, true, true, true],
            &[false, false, false, false, false, true, false, false, false, false],
            &[false, false, false, false, false, true, false, false, false, true],
            &[false, false, false, false, false, true, false, false, true, false],
            &[false, false, false, false, false, true, false, false, true, true],
        ],
    );
    lex_vecs_fixed_length_from_single_small_helper(
        10,
        0..3,
        59049,
        &[
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 2],
            &[0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
            &[0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
            &[0, 0, 0, 0, 0, 0, 0, 0, 1, 2],
            &[0, 0, 0, 0, 0, 0, 0, 0, 2, 0],
            &[0, 0, 0, 0, 0, 0, 0, 0, 2, 1],
            &[0, 0, 0, 0, 0, 0, 0, 0, 2, 2],
            &[0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
            &[0, 0, 0, 0, 0, 0, 0, 1, 0, 2],
            &[0, 0, 0, 0, 0, 0, 0, 1, 1, 0],
            &[0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
            &[0, 0, 0, 0, 0, 0, 0, 1, 1, 2],
            &[0, 0, 0, 0, 0, 0, 0, 1, 2, 0],
            &[0, 0, 0, 0, 0, 0, 0, 1, 2, 1],
            &[0, 0, 0, 0, 0, 0, 0, 1, 2, 2],
            &[0, 0, 0, 0, 0, 0, 0, 2, 0, 0],
            &[0, 0, 0, 0, 0, 0, 0, 2, 0, 1],
        ],
    );
    lex_vecs_fixed_length_from_single_helper(
        2,
        lex_vecs_fixed_length_from_single(2, exhaustive_unsigneds::<u8>()),
        &[
            &[vec![0, 0], vec![0, 0]],
            &[vec![0, 0], vec![0, 1]],
            &[vec![0, 0], vec![0, 2]],
            &[vec![0, 0], vec![0, 3]],
            &[vec![0, 0], vec![0, 4]],
            &[vec![0, 0], vec![0, 5]],
            &[vec![0, 0], vec![0, 6]],
            &[vec![0, 0], vec![0, 7]],
            &[vec![0, 0], vec![0, 8]],
            &[vec![0, 0], vec![0, 9]],
            &[vec![0, 0], vec![0, 10]],
            &[vec![0, 0], vec![0, 11]],
            &[vec![0, 0], vec![0, 12]],
            &[vec![0, 0], vec![0, 13]],
            &[vec![0, 0], vec![0, 14]],
            &[vec![0, 0], vec![0, 15]],
            &[vec![0, 0], vec![0, 16]],
            &[vec![0, 0], vec![0, 17]],
            &[vec![0, 0], vec![0, 18]],
            &[vec![0, 0], vec![0, 19]],
        ],
    );
}
