use std::fmt::Debug;

use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::nevers::nevers;
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::shortlex_vecs_length_range;

fn shortlex_vecs_length_range_small_helper<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
    out_len: usize,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    let xss = shortlex_vecs_length_range(a, b, xs);
    let xss_prefix = xss.clone().take(20).collect::<Vec<_>>();
    assert_eq!(
        xss_prefix
            .iter()
            .map(Vec::as_slice)
            .collect::<Vec<_>>()
            .as_slice(),
        out
    );
    assert_eq!(xss.count(), out_len);
}

#[test]
fn test_shortlex_vecs_length_range() {
    shortlex_vecs_length_range_small_helper(0, 5, nevers(), 1, &[&[]]);
    shortlex_vecs_length_range_small_helper(6, 10, nevers(), 0, &[]);
    shortlex_vecs_length_range_small_helper(
        0,
        5,
        exhaustive_units(),
        5,
        &[&[], &[()], &[(), ()], &[(), (), ()], &[(), (), (), ()]],
    );
    shortlex_vecs_length_range_small_helper(1, 1, exhaustive_bools(), 0, &[]);
    shortlex_vecs_length_range_small_helper(
        0,
        3,
        exhaustive_bools(),
        7,
        &[
            &[],
            &[false],
            &[true],
            &[false, false],
            &[false, true],
            &[true, false],
            &[true, true],
        ],
    );
    shortlex_vecs_length_range_small_helper(
        2,
        4,
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
    shortlex_vecs_length_range_small_helper(
        5,
        8,
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
fn shortlex_vecs_length_range_fail() {
    shortlex_vecs_length_range(1, 0, exhaustive_bools());
}
