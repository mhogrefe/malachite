use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::sets::exhaustive::exhaustive_hash_sets;
use malachite_base::tuples::exhaustive::exhaustive_units;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

fn exhaustive_hash_sets_helper<I: Clone + Iterator>(xs: I, out: &[HashSet<I::Item>])
where
    I::Item: Clone + Debug + Eq + Hash,
{
    let xss = exhaustive_hash_sets(xs).take(20).collect_vec();
    assert_eq!(xss.into_iter().collect_vec().as_slice(), out);
}

fn exhaustive_hash_sets_small_helper<I: Clone + Iterator>(
    xs: I,
    out_len: usize,
    out: &[HashSet<I::Item>],
) where
    I::Item: Clone + Debug + Eq + Hash,
{
    let xss = exhaustive_hash_sets(xs);
    let xss_prefix = xss.clone().take(20).collect_vec();
    assert_eq!(xss_prefix.into_iter().collect_vec().as_slice(), out);
    assert_eq!(xss.count(), out_len);
}

#[test]
fn test_exhaustive_hash_sets() {
    exhaustive_hash_sets_small_helper(nevers(), 1, &[hashset! {}]);
    exhaustive_hash_sets_small_helper(exhaustive_units(), 2, &[hashset! {}, hashset! {()}]);
    exhaustive_hash_sets_small_helper(
        exhaustive_bools(),
        4,
        &[hashset! {}, hashset! {false}, hashset! {true}, hashset! {false, true}],
    );
    exhaustive_hash_sets_small_helper(
        1..=6,
        64,
        &[
            hashset! {},
            hashset! {1},
            hashset! {2},
            hashset! {1, 2},
            hashset! {3},
            hashset! {1, 3},
            hashset! {2, 3},
            hashset! {1, 2, 3},
            hashset! {4},
            hashset! {1, 4},
            hashset! {2, 4},
            hashset! {1, 2, 4},
            hashset! {3, 4},
            hashset! {1, 3, 4},
            hashset! {2, 3, 4},
            hashset! {1, 2, 3, 4},
            hashset! {5},
            hashset! {1, 5},
            hashset! {2, 5},
            hashset! {1, 2, 5},
        ],
    );
    exhaustive_hash_sets_small_helper(
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
    exhaustive_hash_sets_helper(
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
}
