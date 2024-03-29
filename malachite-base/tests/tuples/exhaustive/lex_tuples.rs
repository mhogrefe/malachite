use crate::extra_variadic::{lex_triples, lex_triples_from_single};
use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::options::exhaustive::exhaustive_somes;
use malachite_base::orderings::exhaustive::exhaustive_orderings;
use malachite_base::tuples::exhaustive::{lex_pairs, lex_pairs_from_single};
use std::cmp::Ordering;
use std::fmt::Debug;
use std::iter::once;

fn lex_pairs_helper<
    X: Clone + Debug + Eq,
    I: Clone + Iterator<Item = X>,
    Y: Clone + Debug + Eq,
    J: Clone + Iterator<Item = Y>,
>(
    xs: I,
    ys: J,
    out_len: usize,
    out: &[(X, Y)],
) {
    let ps = lex_pairs(xs, ys);
    let ps_prefix = ps.clone().take(20).collect_vec();
    assert_eq!(ps_prefix.as_slice(), out);
    assert_eq!(ps.count(), out_len);
}

#[test]
fn test_lex_pairs() {
    lex_pairs_helper(nevers(), nevers(), 0, &[]);
    lex_pairs_helper(nevers(), 0..4, 0, &[]);
    lex_pairs_helper(once('a'), once(1), 1, &[('a', 1)]);
    lex_pairs_helper(
        once('a'),
        0..4,
        4,
        &[('a', 0), ('a', 1), ('a', 2), ('a', 3)],
    );
    lex_pairs_helper(
        exhaustive_unsigneds::<u8>(),
        'a'..'e',
        1024,
        &[
            (0, 'a'),
            (0, 'b'),
            (0, 'c'),
            (0, 'd'),
            (1, 'a'),
            (1, 'b'),
            (1, 'c'),
            (1, 'd'),
            (2, 'a'),
            (2, 'b'),
            (2, 'c'),
            (2, 'd'),
            (3, 'a'),
            (3, 'b'),
            (3, 'c'),
            (3, 'd'),
            (4, 'a'),
            (4, 'b'),
            (4, 'c'),
            (4, 'd'),
        ],
    );
    lex_pairs_helper(
        exhaustive_bools(),
        0..4,
        8,
        &[
            (false, 0),
            (false, 1),
            (false, 2),
            (false, 3),
            (true, 0),
            (true, 1),
            (true, 2),
            (true, 3),
        ],
    );
    lex_pairs_helper(
        'a'..'f',
        0..3,
        15,
        &[
            ('a', 0),
            ('a', 1),
            ('a', 2),
            ('b', 0),
            ('b', 1),
            ('b', 2),
            ('c', 0),
            ('c', 1),
            ('c', 2),
            ('d', 0),
            ('d', 1),
            ('d', 2),
            ('e', 0),
            ('e', 1),
            ('e', 2),
        ],
    );
    lex_pairs_helper(
        ['a', 'b', 'c'].iter().cloned(),
        exhaustive_orderings(),
        9,
        &[
            ('a', Ordering::Equal),
            ('a', Ordering::Less),
            ('a', Ordering::Greater),
            ('b', Ordering::Equal),
            ('b', Ordering::Less),
            ('b', Ordering::Greater),
            ('c', Ordering::Equal),
            ('c', Ordering::Less),
            ('c', Ordering::Greater),
        ],
    );
    lex_pairs_helper(
        lex_pairs(exhaustive_orderings(), exhaustive_bools()),
        lex_triples_from_single([Ordering::Less, Ordering::Greater].iter().cloned()),
        48,
        &[
            (
                (Ordering::Equal, false),
                (Ordering::Less, Ordering::Less, Ordering::Less),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Less, Ordering::Less, Ordering::Greater),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Less, Ordering::Greater, Ordering::Less),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Less, Ordering::Greater, Ordering::Greater),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Greater, Ordering::Less, Ordering::Less),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Greater, Ordering::Less, Ordering::Greater),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Greater, Ordering::Greater, Ordering::Less),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Greater, Ordering::Greater, Ordering::Greater),
            ),
            (
                (Ordering::Equal, true),
                (Ordering::Less, Ordering::Less, Ordering::Less),
            ),
            (
                (Ordering::Equal, true),
                (Ordering::Less, Ordering::Less, Ordering::Greater),
            ),
            (
                (Ordering::Equal, true),
                (Ordering::Less, Ordering::Greater, Ordering::Less),
            ),
            (
                (Ordering::Equal, true),
                (Ordering::Less, Ordering::Greater, Ordering::Greater),
            ),
            (
                (Ordering::Equal, true),
                (Ordering::Greater, Ordering::Less, Ordering::Less),
            ),
            (
                (Ordering::Equal, true),
                (Ordering::Greater, Ordering::Less, Ordering::Greater),
            ),
            (
                (Ordering::Equal, true),
                (Ordering::Greater, Ordering::Greater, Ordering::Less),
            ),
            (
                (Ordering::Equal, true),
                (Ordering::Greater, Ordering::Greater, Ordering::Greater),
            ),
            (
                (Ordering::Less, false),
                (Ordering::Less, Ordering::Less, Ordering::Less),
            ),
            (
                (Ordering::Less, false),
                (Ordering::Less, Ordering::Less, Ordering::Greater),
            ),
            (
                (Ordering::Less, false),
                (Ordering::Less, Ordering::Greater, Ordering::Less),
            ),
            (
                (Ordering::Less, false),
                (Ordering::Less, Ordering::Greater, Ordering::Greater),
            ),
        ],
    );
}

fn lex_triples_helper<
    X: Clone + Debug + Eq,
    I: Clone + Iterator<Item = X>,
    Y: Clone + Debug + Eq,
    J: Clone + Iterator<Item = Y>,
    Z: Clone + Debug + Eq,
    K: Clone + Iterator<Item = Z>,
>(
    xs: I,
    ys: J,
    zs: K,
    out_len: usize,
    out: &[(X, Y, Z)],
) {
    let ts = lex_triples(xs, ys, zs);
    let ts_prefix = ts.clone().take(20).collect_vec();
    assert_eq!(ts_prefix.as_slice(), out);
    assert_eq!(ts.count(), out_len);
}

#[test]
fn test_lex_triples() {
    lex_triples_helper(nevers(), nevers(), nevers(), 0, &[]);
    lex_triples_helper(nevers(), 0..4, 'a'..'f', 0, &[]);
    lex_triples_helper(once('a'), once(false), once(5), 1, &[('a', false, 5)]);
    lex_triples_helper(
        once('a'),
        once(false),
        0..4,
        4,
        &[('a', false, 0), ('a', false, 1), ('a', false, 2), ('a', false, 3)],
    );
    lex_triples_helper(
        exhaustive_unsigneds::<u8>(),
        lex_pairs_from_single(exhaustive_bools()),
        'a'..'e',
        4096,
        &[
            (0, (false, false), 'a'),
            (0, (false, false), 'b'),
            (0, (false, false), 'c'),
            (0, (false, false), 'd'),
            (0, (false, true), 'a'),
            (0, (false, true), 'b'),
            (0, (false, true), 'c'),
            (0, (false, true), 'd'),
            (0, (true, false), 'a'),
            (0, (true, false), 'b'),
            (0, (true, false), 'c'),
            (0, (true, false), 'd'),
            (0, (true, true), 'a'),
            (0, (true, true), 'b'),
            (0, (true, true), 'c'),
            (0, (true, true), 'd'),
            (1, (false, false), 'a'),
            (1, (false, false), 'b'),
            (1, (false, false), 'c'),
            (1, (false, false), 'd'),
        ],
    );
    lex_triples_helper(
        exhaustive_bools(),
        0..3,
        'a'..'d',
        18,
        &[
            (false, 0, 'a'),
            (false, 0, 'b'),
            (false, 0, 'c'),
            (false, 1, 'a'),
            (false, 1, 'b'),
            (false, 1, 'c'),
            (false, 2, 'a'),
            (false, 2, 'b'),
            (false, 2, 'c'),
            (true, 0, 'a'),
            (true, 0, 'b'),
            (true, 0, 'c'),
            (true, 1, 'a'),
            (true, 1, 'b'),
            (true, 1, 'c'),
            (true, 2, 'a'),
            (true, 2, 'b'),
            (true, 2, 'c'),
        ],
    );
    lex_triples_helper(
        0..11,
        exhaustive_somes(0..12),
        'a'..'n',
        1716,
        &[
            (0, Some(0), 'a'),
            (0, Some(0), 'b'),
            (0, Some(0), 'c'),
            (0, Some(0), 'd'),
            (0, Some(0), 'e'),
            (0, Some(0), 'f'),
            (0, Some(0), 'g'),
            (0, Some(0), 'h'),
            (0, Some(0), 'i'),
            (0, Some(0), 'j'),
            (0, Some(0), 'k'),
            (0, Some(0), 'l'),
            (0, Some(0), 'm'),
            (0, Some(1), 'a'),
            (0, Some(1), 'b'),
            (0, Some(1), 'c'),
            (0, Some(1), 'd'),
            (0, Some(1), 'e'),
            (0, Some(1), 'f'),
            (0, Some(1), 'g'),
        ],
    );
    lex_triples_helper(
        ['a', 'b', 'c'].iter().cloned(),
        ["xx", "yy", "zz"].iter().cloned(),
        0..3,
        27,
        &[
            ('a', "xx", 0),
            ('a', "xx", 1),
            ('a', "xx", 2),
            ('a', "yy", 0),
            ('a', "yy", 1),
            ('a', "yy", 2),
            ('a', "zz", 0),
            ('a', "zz", 1),
            ('a', "zz", 2),
            ('b', "xx", 0),
            ('b', "xx", 1),
            ('b', "xx", 2),
            ('b', "yy", 0),
            ('b', "yy", 1),
            ('b', "yy", 2),
            ('b', "zz", 0),
            ('b', "zz", 1),
            ('b', "zz", 2),
            ('c', "xx", 0),
            ('c', "xx", 1),
        ],
    );
}
