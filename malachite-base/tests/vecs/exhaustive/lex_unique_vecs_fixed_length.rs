use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::lex_unique_vecs_fixed_length;
use std::fmt::Debug;

fn lex_unique_vecs_fixed_length_helper<I: Iterator>(len: u64, xs: I, out: &[&[I::Item]])
where
    I::Item: Clone + Debug + Eq,
{
    let xss = lex_unique_vecs_fixed_length(len, xs).take(20).collect_vec();
    assert_eq!(xss.iter().map(Vec::as_slice).collect_vec().as_slice(), out);
}

fn lex_unique_vecs_fixed_length_small_helper<I: Clone + Iterator>(
    len: u64,
    xs: I,
    out_len: usize,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    let xss = lex_unique_vecs_fixed_length(len, xs);
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
fn test_lex_unique_vecs_fixed_length() {
    // This demonstrates that 0 ^ 0 == 1:
    lex_unique_vecs_fixed_length_small_helper(0, nevers(), 1, &[&[]]);
    lex_unique_vecs_fixed_length_small_helper(1, nevers(), 0, &[]);
    lex_unique_vecs_fixed_length_small_helper(2, nevers(), 0, &[]);
    lex_unique_vecs_fixed_length_small_helper(5, nevers(), 0, &[]);
    lex_unique_vecs_fixed_length_small_helper(1, exhaustive_units(), 1, &[&[()]]);
    lex_unique_vecs_fixed_length_small_helper(2, exhaustive_units(), 0, &[]);
    lex_unique_vecs_fixed_length_small_helper(5, exhaustive_units(), 0, &[]);
    lex_unique_vecs_fixed_length_small_helper(0, exhaustive_unsigneds::<u8>(), 1, &[&[]]);
    lex_unique_vecs_fixed_length_small_helper(
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
    lex_unique_vecs_fixed_length_helper(
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
    lex_unique_vecs_fixed_length_small_helper(
        2,
        exhaustive_unsigneds::<u8>(),
        65280,
        &[
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
            &[0, 20],
        ],
    );
    lex_unique_vecs_fixed_length_helper(
        3,
        exhaustive_unsigneds::<u8>(),
        &[
            &[0, 1, 2],
            &[0, 1, 3],
            &[0, 1, 4],
            &[0, 1, 5],
            &[0, 1, 6],
            &[0, 1, 7],
            &[0, 1, 8],
            &[0, 1, 9],
            &[0, 1, 10],
            &[0, 1, 11],
            &[0, 1, 12],
            &[0, 1, 13],
            &[0, 1, 14],
            &[0, 1, 15],
            &[0, 1, 16],
            &[0, 1, 17],
            &[0, 1, 18],
            &[0, 1, 19],
            &[0, 1, 20],
            &[0, 1, 21],
        ],
    );
    lex_unique_vecs_fixed_length_small_helper(
        2,
        exhaustive_ascii_chars(),
        16256,
        &[
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
            &['a', 'u'],
        ],
    );
    lex_unique_vecs_fixed_length_small_helper(1, exhaustive_bools(), 2, &[&[false], &[true]]);
    lex_unique_vecs_fixed_length_small_helper(
        2,
        exhaustive_bools(),
        2,
        &[&[false, true], &[true, false]],
    );
    lex_unique_vecs_fixed_length_small_helper(4, exhaustive_bools(), 0, &[]);
    lex_unique_vecs_fixed_length_small_helper(
        4,
        1..=6,
        360,
        &[
            &[1, 2, 3, 4],
            &[1, 2, 3, 5],
            &[1, 2, 3, 6],
            &[1, 2, 4, 3],
            &[1, 2, 4, 5],
            &[1, 2, 4, 6],
            &[1, 2, 5, 3],
            &[1, 2, 5, 4],
            &[1, 2, 5, 6],
            &[1, 2, 6, 3],
            &[1, 2, 6, 4],
            &[1, 2, 6, 5],
            &[1, 3, 2, 4],
            &[1, 3, 2, 5],
            &[1, 3, 2, 6],
            &[1, 3, 4, 2],
            &[1, 3, 4, 5],
            &[1, 3, 4, 6],
            &[1, 3, 5, 2],
            &[1, 3, 5, 4],
        ],
    );
    lex_unique_vecs_fixed_length_helper(
        2,
        lex_unique_vecs_fixed_length(2, exhaustive_unsigneds::<u8>()),
        &[
            &[vec![0, 1], vec![0, 2]],
            &[vec![0, 1], vec![0, 3]],
            &[vec![0, 1], vec![0, 4]],
            &[vec![0, 1], vec![0, 5]],
            &[vec![0, 1], vec![0, 6]],
            &[vec![0, 1], vec![0, 7]],
            &[vec![0, 1], vec![0, 8]],
            &[vec![0, 1], vec![0, 9]],
            &[vec![0, 1], vec![0, 10]],
            &[vec![0, 1], vec![0, 11]],
            &[vec![0, 1], vec![0, 12]],
            &[vec![0, 1], vec![0, 13]],
            &[vec![0, 1], vec![0, 14]],
            &[vec![0, 1], vec![0, 15]],
            &[vec![0, 1], vec![0, 16]],
            &[vec![0, 1], vec![0, 17]],
            &[vec![0, 1], vec![0, 18]],
            &[vec![0, 1], vec![0, 19]],
            &[vec![0, 1], vec![0, 20]],
            &[vec![0, 1], vec![0, 21]],
        ],
    );
}
