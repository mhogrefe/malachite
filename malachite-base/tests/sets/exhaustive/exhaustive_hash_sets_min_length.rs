use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::sets::exhaustive::exhaustive_hash_sets_min_length;
use malachite_base::tuples::exhaustive::exhaustive_units;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

fn exhaustive_hash_sets_min_length_helper<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
    out: &[HashSet<I::Item>],
) where
    I::Item: Clone + Debug + Eq + Hash,
{
    let xss = exhaustive_hash_sets_min_length(min_length, xs)
        .take(20)
        .collect_vec();
    assert_eq!(xss.into_iter().collect_vec().as_slice(), out);
}

fn exhaustive_hash_sets_min_length_small_helper<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
    out_len: usize,
    out: &[HashSet<I::Item>],
) where
    I::Item: Clone + Debug + Eq + Hash,
{
    let xss = exhaustive_hash_sets_min_length(min_length, xs);
    let xss_prefix = xss.clone().take(20).collect_vec();
    assert_eq!(xss_prefix.into_iter().collect_vec().as_slice(), out);
    assert_eq!(xss.count(), out_len);
}

#[test]
fn test_exhaustive_hash_sets_min_length() {
    exhaustive_hash_sets_min_length_small_helper(0, nevers(), 1, &[hashset! {}]);
    exhaustive_hash_sets_min_length_small_helper(4, nevers(), 0, &[]);
    exhaustive_hash_sets_min_length_small_helper(
        0,
        exhaustive_units(),
        2,
        &[hashset! {}, hashset! {()}],
    );
    exhaustive_hash_sets_min_length_small_helper(5, exhaustive_units(), 0, &[]);
    exhaustive_hash_sets_min_length_small_helper(
        0,
        exhaustive_bools(),
        4,
        &[hashset! {}, hashset! {false}, hashset! {true}, hashset! {false, true}],
    );
    exhaustive_hash_sets_min_length_small_helper(
        1,
        exhaustive_bools(),
        3,
        &[hashset! {false}, hashset! {true}, hashset! {false, true}],
    );
    exhaustive_hash_sets_min_length_small_helper(
        0,
        'a'..='c',
        8,
        &[
            hashset! {},
            hashset! {'a'},
            hashset! {'b'},
            hashset! {'a', 'b'},
            hashset! {'c'},
            hashset! {'a', 'c'},
            hashset! {'b', 'c'},
            hashset! {'a', 'b', 'c'},
        ],
    );
    exhaustive_hash_sets_min_length_small_helper(
        2,
        'a'..='c',
        4,
        &[hashset! {'a', 'b'}, hashset! {'a', 'c'}, hashset! {'b', 'c'}, hashset! {'a', 'b', 'c'}],
    );
    exhaustive_hash_sets_min_length_helper(
        0,
        exhaustive_ascii_chars(),
        &[
            hashset! {},
            hashset! {'a'},
            hashset! {'b'},
            hashset! {'a', 'b'},
            hashset! {'c'},
            hashset! {'a', 'c'},
            hashset! {'b', 'c'},
            hashset! {'a', 'b', 'c'},
            hashset! {'d'},
            hashset! {'a', 'd'},
            hashset! {'b', 'd'},
            hashset! {'a', 'b', 'd'},
            hashset! {'c', 'd'},
            hashset! {'a', 'c', 'd'},
            hashset! {'b', 'c', 'd'},
            hashset! {'a', 'b', 'c', 'd'},
            hashset! {'e'},
            hashset! {'a', 'e'},
            hashset! {'b', 'e'},
            hashset! {'a', 'b', 'e'},
        ],
    );
    exhaustive_hash_sets_min_length_helper(
        3,
        exhaustive_ascii_chars(),
        &[
            hashset! {'a', 'b', 'c'},
            hashset! {'a', 'b', 'd'},
            hashset! {'a', 'c', 'd'},
            hashset! {'b', 'c', 'd'},
            hashset! {'a', 'b', 'c', 'd'},
            hashset! {'a', 'b', 'e'},
            hashset! {'a', 'c', 'e'},
            hashset! {'b', 'c', 'e'},
            hashset! {'a', 'b', 'c', 'e'},
            hashset! {'a', 'd', 'e'},
            hashset! {'b', 'd', 'e'},
            hashset! {'a', 'b', 'd', 'e'},
            hashset! {'c', 'd', 'e'},
            hashset! {'a', 'c', 'd', 'e'},
            hashset! {'b', 'c', 'd', 'e'},
            hashset! {'a', 'b', 'c', 'd', 'e'},
            hashset! {'a', 'b', 'f'},
            hashset! {'a', 'c', 'f'},
            hashset! {'b', 'c', 'f'},
            hashset! {'a', 'b', 'c', 'f'},
        ],
    );
}
