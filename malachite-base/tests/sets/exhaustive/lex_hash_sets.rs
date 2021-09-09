use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::sets::exhaustive::lex_hash_sets;
use malachite_base::tuples::exhaustive::exhaustive_units;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

fn lex_hash_sets_helper<I: Clone + Iterator>(xs: I, out: &[HashSet<I::Item>])
where
    I::Item: Clone + Debug + Eq + Hash,
{
    let xss = lex_hash_sets(xs).take(20).collect_vec();
    assert_eq!(xss.into_iter().collect_vec().as_slice(), out);
}

fn lex_hash_sets_small_helper<I: Clone + Iterator>(xs: I, out_len: usize, out: &[HashSet<I::Item>])
where
    I::Item: Clone + Debug + Eq + Hash,
{
    let xss = lex_hash_sets(xs);
    let xss_prefix = xss.clone().take(20).collect_vec();
    assert_eq!(xss_prefix.into_iter().collect_vec().as_slice(), out);
    assert_eq!(xss.count(), out_len);
}

#[test]
fn test_lex_hash_sets() {
    lex_hash_sets_small_helper(nevers(), 1, &[hashset! {}]);
    lex_hash_sets_small_helper(exhaustive_units(), 2, &[hashset! {}, hashset! {()}]);
    lex_hash_sets_small_helper(
        exhaustive_bools(),
        4,
        &[hashset! {}, hashset! {false}, hashset! {false, true}, hashset! {true}],
    );
    lex_hash_sets_small_helper(
        1..=6,
        64,
        &[
            hashset! {},
            hashset! {1},
            hashset! {1, 2},
            hashset! {1, 2, 3},
            hashset! {1, 2, 3, 4},
            hashset! {1, 2, 3, 4, 5},
            hashset! {1, 2, 3, 4, 5, 6},
            hashset! {1, 2, 3, 4, 6},
            hashset! {1, 2, 3, 5},
            hashset! {1, 2, 3, 5, 6},
            hashset! {1, 2, 3, 6},
            hashset! {1, 2, 4},
            hashset! {1, 2, 4, 5},
            hashset! {1, 2, 4, 5, 6},
            hashset! {1, 2, 4, 6},
            hashset! {1, 2, 5},
            hashset! {1, 2, 5, 6},
            hashset! {1, 2, 6},
            hashset! {1, 3},
            hashset! {1, 3, 4},
        ],
    );
    lex_hash_sets_small_helper(
        'a'..='c',
        8,
        &[
            hashset! {},
            hashset! {'a'},
            hashset! {'a', 'b'},
            hashset! {'a', 'b', 'c'},
            hashset! {'a', 'c'},
            hashset! {'b'},
            hashset! {'b', 'c'},
            hashset! {'c'},
        ],
    );
    lex_hash_sets_helper(
        exhaustive_ascii_chars(),
        &[
            hashset! {},
            hashset! {'a'},
            hashset! {'a', 'b'},
            hashset! {'a', 'b', 'c'},
            hashset! {'a', 'b', 'c', 'd'},
            hashset! {'a', 'b', 'c', 'd', 'e'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f', 'g'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n'},
            hashset! {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o'},
            hashset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'
            },
            hashset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q'
            },
            hashset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r',
            },
            hashset! {
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's',
            },
        ],
    );
}
