use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::nevers::nevers;
use malachite_base::test_util::vecs::exhaustive::exhaustive_vecs_small_helper_helper;
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::shortlex_unique_vecs_length_inclusive_range;
use std::fmt::Debug;

fn shortlex_unique_vecs_length_inclusive_range_small_helper<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
    out_len: usize,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_small_helper_helper(
        shortlex_unique_vecs_length_inclusive_range(a, b, xs),
        out_len,
        out,
    );
}

#[test]
fn test_shortlex_unique_vecs_length_inclusive_range() {
    shortlex_unique_vecs_length_inclusive_range_small_helper(0, 4, nevers(), 1, &[&[]]);
    shortlex_unique_vecs_length_inclusive_range_small_helper(6, 9, nevers(), 0, &[]);
    shortlex_unique_vecs_length_inclusive_range_small_helper(
        0,
        4,
        exhaustive_units(),
        2,
        &[&[], &[()]],
    );
    shortlex_unique_vecs_length_inclusive_range_small_helper(1, 0, exhaustive_bools(), 0, &[]);
    shortlex_unique_vecs_length_inclusive_range_small_helper(
        0,
        1,
        exhaustive_bools(),
        3,
        &[&[], &[false], &[true]],
    );
    shortlex_unique_vecs_length_inclusive_range_small_helper(
        2,
        3,
        exhaustive_bools(),
        2,
        &[&[false, true], &[true, false]],
    );
    shortlex_unique_vecs_length_inclusive_range_small_helper(
        1,
        1,
        'a'..='c',
        3,
        &[&['a'], &['b'], &['c']],
    );
}
