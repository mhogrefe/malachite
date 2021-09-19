use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::nevers::nevers;
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::exhaustive_ordered_unique_vecs_length_inclusive_range;
use std::fmt::Debug;

fn exhaustive_ordered_unique_vecs_length_inclusive_range_small_helper<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
    out_len: usize,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    let xss = exhaustive_ordered_unique_vecs_length_inclusive_range(a, b, xs);
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
fn test_exhaustive_ordered_unique_vecs_length_inclusive_range() {
    exhaustive_ordered_unique_vecs_length_inclusive_range_small_helper(0, 4, nevers(), 1, &[&[]]);
    exhaustive_ordered_unique_vecs_length_inclusive_range_small_helper(6, 9, nevers(), 0, &[]);
    exhaustive_ordered_unique_vecs_length_inclusive_range_small_helper(
        0,
        4,
        exhaustive_units(),
        2,
        &[&[], &[()]],
    );
    exhaustive_ordered_unique_vecs_length_inclusive_range_small_helper(
        1,
        0,
        exhaustive_bools(),
        0,
        &[],
    );
    exhaustive_ordered_unique_vecs_length_inclusive_range_small_helper(
        0,
        1,
        exhaustive_bools(),
        3,
        &[&[], &[false], &[true]],
    );
    exhaustive_ordered_unique_vecs_length_inclusive_range_small_helper(
        2,
        3,
        exhaustive_bools(),
        1,
        &[&[false, true]],
    );
    exhaustive_ordered_unique_vecs_length_inclusive_range_small_helper(
        1,
        1,
        'a'..='c',
        3,
        &[&['a'], &['b'], &['c']],
    );
}
