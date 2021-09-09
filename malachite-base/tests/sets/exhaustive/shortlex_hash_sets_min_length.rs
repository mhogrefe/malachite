use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::sets::exhaustive::shortlex_hash_sets_min_length;
use malachite_base::tuples::exhaustive::exhaustive_units;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

fn shortlex_hash_sets_min_length_helper<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
    out: &[HashSet<I::Item>],
) where
    I::Item: Clone + Debug + Eq + Hash,
{
    let xss = shortlex_hash_sets_min_length(min_length, xs)
        .take(20)
        .collect_vec();
    assert_eq!(xss.into_iter().collect_vec().as_slice(), out);
}

fn shortlex_hash_sets_min_length_small_helper<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
    out_len: usize,
    out: &[HashSet<I::Item>],
) where
    I::Item: Clone + Debug + Eq + Hash,
{
    let xss = shortlex_hash_sets_min_length(min_length, xs);
    let xss_prefix = xss.clone().take(20).collect_vec();
    assert_eq!(xss_prefix.into_iter().collect_vec().as_slice(), out);
    assert_eq!(xss.count(), out_len);
}

#[test]
fn test_shortlex_hash_sets_min_length() {
    shortlex_hash_sets_min_length_small_helper(0, nevers(), 1, &[hashset! {}]);
    shortlex_hash_sets_min_length_small_helper(4, nevers(), 0, &[]);
    shortlex_hash_sets_min_length_small_helper(
        0,
        exhaustive_units(),
        2,
        &[hashset! {}, hashset! {()}],
    );
    shortlex_hash_sets_min_length_small_helper(5, exhaustive_units(), 0, &[]);
    shortlex_hash_sets_min_length_small_helper(
        0,
        exhaustive_bools(),
        4,
        &[hashset! {}, hashset! {false}, hashset! {true}, hashset! {false, true}],
    );
    shortlex_hash_sets_min_length_small_helper(
        1,
        exhaustive_bools(),
        3,
        &[hashset! {false}, hashset! {true}, hashset! {false, true}],
    );
    shortlex_hash_sets_min_length_small_helper(
        0,
        'a'..='c',
        8,
        &[
            hashset! {},
            hashset! {'a'},
            hashset! {'b'},
            hashset! {'c'},
            hashset! {'a', 'b'},
            hashset! {'a', 'c'},
            hashset! {'b', 'c'},
            hashset! {'a', 'b', 'c'},
        ],
    );
    shortlex_hash_sets_min_length_small_helper(
        2,
        'a'..='c',
        4,
        &[hashset! {'a', 'b'}, hashset! {'a', 'c'}, hashset! {'b', 'c'}, hashset! {'a', 'b', 'c'}],
    );
    shortlex_hash_sets_min_length_helper(
        0,
        exhaustive_ascii_chars(),
        &[
            hashset! {},
            hashset! {'a'},
            hashset! {'b'},
            hashset! {'c'},
            hashset! {'d'},
            hashset! {'e'},
            hashset! {'f'},
            hashset! {'g'},
            hashset! {'h'},
            hashset! {'i'},
            hashset! {'j'},
            hashset! {'k'},
            hashset! {'l'},
            hashset! {'m'},
            hashset! {'n'},
            hashset! {'o'},
            hashset! {'p'},
            hashset! {'q'},
            hashset! {'r'},
            hashset! {'s'},
        ],
    );
    shortlex_hash_sets_min_length_helper(
        3,
        exhaustive_ascii_chars(),
        &[
            hashset! {'a', 'b', 'c'},
            hashset! {'a', 'b', 'd'},
            hashset! {'a', 'b', 'e'},
            hashset! {'a', 'b', 'f'},
            hashset! {'a', 'b', 'g'},
            hashset! {'a', 'b', 'h'},
            hashset! {'a', 'b', 'i'},
            hashset! {'a', 'b', 'j'},
            hashset! {'a', 'b', 'k'},
            hashset! {'a', 'b', 'l'},
            hashset! {'a', 'b', 'm'},
            hashset! {'a', 'b', 'n'},
            hashset! {'a', 'b', 'o'},
            hashset! {'a', 'b', 'p'},
            hashset! {'a', 'b', 'q'},
            hashset! {'a', 'b', 'r'},
            hashset! {'a', 'b', 's'},
            hashset! {'a', 'b', 't'},
            hashset! {'a', 'b', 'u'},
            hashset! {'a', 'b', 'v'},
        ],
    );
}
