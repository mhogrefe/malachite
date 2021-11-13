use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::lex_unique_vecs_min_length;
use std::fmt::Debug;

fn lex_unique_vecs_min_length_helper<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    let xss = lex_unique_vecs_min_length(min_length, xs)
        .take(20)
        .collect_vec();
    assert_eq!(xss.iter().map(Vec::as_slice).collect_vec().as_slice(), out);
}

fn lex_unique_vecs_min_length_small_helper<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
    out_len: usize,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    let xss = lex_unique_vecs_min_length(min_length, xs);
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
fn test_lex_unique_vecs_min_length() {
    lex_unique_vecs_min_length_small_helper(0, nevers(), 1, &[&[]]);
    lex_unique_vecs_min_length_small_helper(4, nevers(), 0, &[]);
    lex_unique_vecs_min_length_small_helper(0, exhaustive_units(), 2, &[&[], &[()]]);
    lex_unique_vecs_min_length_small_helper(5, exhaustive_units(), 0, &[]);
    lex_unique_vecs_min_length_small_helper(
        0,
        exhaustive_bools(),
        5,
        &[&[], &[false], &[false, true], &[true], &[true, false]],
    );
    lex_unique_vecs_min_length_small_helper(
        1,
        exhaustive_bools(),
        4,
        &[&[false], &[false, true], &[true], &[true, false]],
    );
    lex_unique_vecs_min_length_small_helper(
        0,
        'a'..='c',
        16,
        &[
            &[],
            &['a'],
            &['a', 'b'],
            &['a', 'b', 'c'],
            &['a', 'c'],
            &['a', 'c', 'b'],
            &['b'],
            &['b', 'a'],
            &['b', 'a', 'c'],
            &['b', 'c'],
            &['b', 'c', 'a'],
            &['c'],
            &['c', 'a'],
            &['c', 'a', 'b'],
            &['c', 'b'],
            &['c', 'b', 'a'],
        ],
    );
    lex_unique_vecs_min_length_small_helper(
        2,
        'a'..='c',
        12,
        &[
            &['a', 'b'],
            &['a', 'b', 'c'],
            &['a', 'c'],
            &['a', 'c', 'b'],
            &['b', 'a'],
            &['b', 'a', 'c'],
            &['b', 'c'],
            &['b', 'c', 'a'],
            &['c', 'a'],
            &['c', 'a', 'b'],
            &['c', 'b'],
            &['c', 'b', 'a'],
        ],
    );
    lex_unique_vecs_min_length_helper(
        0,
        exhaustive_ascii_chars(),
        &[
            &[],
            &['a'],
            &['a', 'b'],
            &['a', 'b', 'c'],
            &['a', 'b', 'c', 'd'],
            &['a', 'b', 'c', 'd', 'e'],
            &['a', 'b', 'c', 'd', 'e', 'f'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q'],
            &[
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r',
            ],
            &[
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's',
            ],
        ],
    );
    lex_unique_vecs_min_length_helper(
        3,
        exhaustive_ascii_chars(),
        &[
            &['a', 'b', 'c'],
            &['a', 'b', 'c', 'd'],
            &['a', 'b', 'c', 'd', 'e'],
            &['a', 'b', 'c', 'd', 'e', 'f'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q'],
            &[
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r',
            ],
            &[
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's',
            ],
            &[
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't',
            ],
            &[
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u',
            ],
            &[
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v',
            ],
        ],
    );
}
