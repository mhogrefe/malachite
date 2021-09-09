use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::nevers::nevers;
use malachite_base::sets::exhaustive::shortlex_hash_sets_length_range;
use malachite_base::tuples::exhaustive::exhaustive_units;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

fn shortlex_hash_sets_length_range_small_helper<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
    out_len: usize,
    out: &[HashSet<I::Item>],
) where
    I::Item: Clone + Debug + Eq + Hash,
{
    let xss = shortlex_hash_sets_length_range(a, b, xs);
    let xss_prefix = xss.clone().take(20).collect_vec();
    assert_eq!(xss_prefix.into_iter().collect_vec().as_slice(), out);
    assert_eq!(xss.count(), out_len);
}

#[test]
fn test_shortlex_hash_sets_length_range() {
    shortlex_hash_sets_length_range_small_helper(0, 5, nevers(), 1, &[hashset! {}]);
    shortlex_hash_sets_length_range_small_helper(6, 10, nevers(), 0, &[]);
    shortlex_hash_sets_length_range_small_helper(
        0,
        5,
        exhaustive_units(),
        2,
        &[hashset! {}, hashset! {()}],
    );
    shortlex_hash_sets_length_range_small_helper(1, 0, exhaustive_bools(), 0, &[]);
    shortlex_hash_sets_length_range_small_helper(1, 1, exhaustive_bools(), 0, &[]);
    shortlex_hash_sets_length_range_small_helper(
        0,
        2,
        exhaustive_bools(),
        3,
        &[hashset! {}, hashset! {false}, hashset! {true}],
    );
    shortlex_hash_sets_length_range_small_helper(
        2,
        4,
        exhaustive_bools(),
        1,
        &[hashset! {false, true}],
    );
    shortlex_hash_sets_length_range_small_helper(
        1,
        2,
        'a'..='c',
        3,
        &[hashset! {'a'}, hashset! {'b'}, hashset! {'c'}],
    );
}
