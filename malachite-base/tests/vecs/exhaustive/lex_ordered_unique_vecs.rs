use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::lex_ordered_unique_vecs;
use std::fmt::Debug;

fn lex_ordered_unique_vecs_helper<I: Clone + Iterator>(xs: I, out: &[&[I::Item]])
where
    I::Item: Clone + Debug + Eq,
{
    let xss = lex_ordered_unique_vecs(xs).take(20).collect_vec();
    assert_eq!(xss.iter().map(Vec::as_slice).collect_vec().as_slice(), out);
}

fn lex_ordered_unique_vecs_small_helper<I: Clone + Iterator>(
    xs: I,
    out_len: usize,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    let xss = lex_ordered_unique_vecs(xs);
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
fn test_lex_ordered_unique_vecs() {
    lex_ordered_unique_vecs_small_helper(nevers(), 1, &[&[]]);
    lex_ordered_unique_vecs_small_helper(exhaustive_units(), 2, &[&[], &[()]]);
    lex_ordered_unique_vecs_small_helper(
        exhaustive_bools(),
        4,
        &[&[], &[false], &[false, true], &[true]],
    );
    lex_ordered_unique_vecs_small_helper(
        1..=6,
        64,
        &[
            &[],
            &[1],
            &[1, 2],
            &[1, 2, 3],
            &[1, 2, 3, 4],
            &[1, 2, 3, 4, 5],
            &[1, 2, 3, 4, 5, 6],
            &[1, 2, 3, 4, 6],
            &[1, 2, 3, 5],
            &[1, 2, 3, 5, 6],
            &[1, 2, 3, 6],
            &[1, 2, 4],
            &[1, 2, 4, 5],
            &[1, 2, 4, 5, 6],
            &[1, 2, 4, 6],
            &[1, 2, 5],
            &[1, 2, 5, 6],
            &[1, 2, 6],
            &[1, 3],
            &[1, 3, 4],
        ],
    );
    lex_ordered_unique_vecs_small_helper(
        'a'..='c',
        8,
        &[&[], &['a'], &['a', 'b'], &['a', 'b', 'c'], &['a', 'c'], &['b'], &['b', 'c'], &['c']],
    );
    lex_ordered_unique_vecs_helper(
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
}
