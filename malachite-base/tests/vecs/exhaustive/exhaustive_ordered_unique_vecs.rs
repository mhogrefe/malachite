use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::exhaustive_ordered_unique_vecs;
use std::fmt::Debug;

fn exhaustive_ordered_unique_vecs_helper<I: Clone + Iterator>(xs: I, out: &[&[I::Item]])
where
    I::Item: Clone + Debug + Eq,
{
    let xss = exhaustive_ordered_unique_vecs(xs).take(20).collect_vec();
    assert_eq!(xss.iter().map(Vec::as_slice).collect_vec().as_slice(), out);
}

fn exhaustive_ordered_unique_vecs_small_helper<I: Clone + Iterator>(
    xs: I,
    out_len: usize,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    let xss = exhaustive_ordered_unique_vecs(xs);
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
fn test_exhaustive_ordered_unique_vecs() {
    exhaustive_ordered_unique_vecs_small_helper(nevers(), 1, &[&[]]);
    exhaustive_ordered_unique_vecs_small_helper(exhaustive_units(), 2, &[&[], &[()]]);
    exhaustive_ordered_unique_vecs_small_helper(
        exhaustive_bools(),
        4,
        &[&[], &[false], &[true], &[false, true]],
    );
    exhaustive_ordered_unique_vecs_small_helper(
        1..=6,
        64,
        &[
            &[],
            &[1],
            &[2],
            &[1, 2],
            &[3],
            &[1, 3],
            &[2, 3],
            &[1, 2, 3],
            &[4],
            &[1, 4],
            &[2, 4],
            &[1, 2, 4],
            &[3, 4],
            &[1, 3, 4],
            &[2, 3, 4],
            &[1, 2, 3, 4],
            &[5],
            &[1, 5],
            &[2, 5],
            &[1, 2, 5],
        ],
    );
    exhaustive_ordered_unique_vecs_small_helper(
        'a'..='c',
        8,
        &[&[], &['a'], &['b'], &['a', 'b'], &['c'], &['a', 'c'], &['b', 'c'], &['a', 'b', 'c']],
    );
    exhaustive_ordered_unique_vecs_helper(
        exhaustive_ascii_chars(),
        &[
            &[],
            &['a'],
            &['b'],
            &['a', 'b'],
            &['c'],
            &['a', 'c'],
            &['b', 'c'],
            &['a', 'b', 'c'],
            &['d'],
            &['a', 'd'],
            &['b', 'd'],
            &['a', 'b', 'd'],
            &['c', 'd'],
            &['a', 'c', 'd'],
            &['b', 'c', 'd'],
            &['a', 'b', 'c', 'd'],
            &['e'],
            &['a', 'e'],
            &['b', 'e'],
            &['a', 'b', 'e'],
        ],
    );
}
