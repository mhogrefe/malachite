use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::exhaustive_unique_vecs;
use std::fmt::Debug;

fn exhaustive_unique_vecs_helper<I: Clone + Iterator>(xs: I, out: &[&[I::Item]])
where
    I::Item: Clone + Debug + Eq,
{
    let xss = exhaustive_unique_vecs(xs).take(20).collect_vec();
    assert_eq!(xss.iter().map(Vec::as_slice).collect_vec().as_slice(), out);
}

fn exhaustive_unique_vecs_small_helper<I: Clone + Iterator>(
    xs: I,
    out_len: usize,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    let xss = exhaustive_unique_vecs(xs);
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
fn test_exhaustive_unique_vecs() {
    exhaustive_unique_vecs_small_helper(nevers(), 1, &[&[]]);
    exhaustive_unique_vecs_small_helper(exhaustive_units(), 2, &[&[], &[()]]);
    exhaustive_unique_vecs_small_helper(
        exhaustive_bools(),
        5,
        &[&[], &[false], &[true], &[false, true], &[true, false]],
    );
    exhaustive_unique_vecs_small_helper(
        1..=6,
        1957,
        &[
            &[],
            &[1],
            &[2],
            &[3],
            &[1, 2],
            &[1, 3],
            &[2, 1],
            &[1, 2, 3],
            &[3, 1],
            &[2, 3],
            &[3, 2],
            &[4],
            &[1, 3, 2],
            &[1, 4],
            &[2, 1, 3],
            &[3, 4],
            &[2, 3, 1],
            &[4, 1],
            &[3, 1, 2],
            &[2, 4],
        ],
    );
    exhaustive_unique_vecs_small_helper(
        'a'..='c',
        16,
        &[
            &[],
            &['a'],
            &['b'],
            &['c'],
            &['a', 'b'],
            &['a', 'c'],
            &['b', 'a'],
            &['a', 'b', 'c'],
            &['c', 'a'],
            &['b', 'c'],
            &['c', 'b'],
            &['a', 'c', 'b'],
            &['b', 'a', 'c'],
            &['b', 'c', 'a'],
            &['c', 'a', 'b'],
            &['c', 'b', 'a'],
        ],
    );
    exhaustive_unique_vecs_helper(
        exhaustive_ascii_chars(),
        &[
            &[],
            &['a'],
            &['b'],
            &['c'],
            &['a', 'b'],
            &['a', 'c'],
            &['b', 'a'],
            &['a', 'b', 'c'],
            &['c', 'a'],
            &['b', 'c'],
            &['c', 'b'],
            &['d'],
            &['a', 'c', 'b'],
            &['a', 'd'],
            &['b', 'a', 'c'],
            &['c', 'd'],
            &['b', 'c', 'a'],
            &['d', 'a'],
            &['c', 'a', 'b'],
            &['b', 'd'],
        ],
    );
}
