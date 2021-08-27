use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::nevers::nevers;
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::shortlex_vecs_length_inclusive_range;
use std::fmt::Debug;

fn shortlex_vecs_length_inclusive_range_small_helper<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
    out_len: usize,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    let xss = shortlex_vecs_length_inclusive_range(a, b, xs);
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
fn test_shortlex_vecs_length_inclusive_range() {
    shortlex_vecs_length_inclusive_range_small_helper(0, 4, nevers(), 1, &[&[]]);
    shortlex_vecs_length_inclusive_range_small_helper(6, 9, nevers(), 0, &[]);
    shortlex_vecs_length_inclusive_range_small_helper(
        0,
        4,
        exhaustive_units(),
        5,
        &[&[], &[()], &[(), ()], &[(), (), ()], &[(), (), (), ()]],
    );
    shortlex_vecs_length_inclusive_range_small_helper(
        1,
        1,
        exhaustive_bools(),
        2,
        &[&[false], &[true]],
    );
    shortlex_vecs_length_inclusive_range_small_helper(
        0,
        2,
        exhaustive_bools(),
        7,
        &[&[], &[false], &[true], &[false, false], &[false, true], &[true, false], &[true, true]],
    );
    shortlex_vecs_length_inclusive_range_small_helper(
        2,
        3,
        exhaustive_bools(),
        12,
        &[
            &[false, false],
            &[false, true],
            &[true, false],
            &[true, true],
            &[false, false, false],
            &[false, false, true],
            &[false, true, false],
            &[false, true, true],
            &[true, false, false],
            &[true, false, true],
            &[true, true, false],
            &[true, true, true],
        ],
    );
    shortlex_vecs_length_inclusive_range_small_helper(
        5,
        7,
        'a'..='c',
        3159,
        &[
            &['a', 'a', 'a', 'a', 'a'],
            &['a', 'a', 'a', 'a', 'b'],
            &['a', 'a', 'a', 'a', 'c'],
            &['a', 'a', 'a', 'b', 'a'],
            &['a', 'a', 'a', 'b', 'b'],
            &['a', 'a', 'a', 'b', 'c'],
            &['a', 'a', 'a', 'c', 'a'],
            &['a', 'a', 'a', 'c', 'b'],
            &['a', 'a', 'a', 'c', 'c'],
            &['a', 'a', 'b', 'a', 'a'],
            &['a', 'a', 'b', 'a', 'b'],
            &['a', 'a', 'b', 'a', 'c'],
            &['a', 'a', 'b', 'b', 'a'],
            &['a', 'a', 'b', 'b', 'b'],
            &['a', 'a', 'b', 'b', 'c'],
            &['a', 'a', 'b', 'c', 'a'],
            &['a', 'a', 'b', 'c', 'b'],
            &['a', 'a', 'b', 'c', 'c'],
            &['a', 'a', 'c', 'a', 'a'],
            &['a', 'a', 'c', 'a', 'b'],
        ],
    );
}

#[test]
#[should_panic]
fn shortlex_vecs_length_inclusive_range_fail() {
    shortlex_vecs_length_inclusive_range(1, 0, exhaustive_bools());
}
