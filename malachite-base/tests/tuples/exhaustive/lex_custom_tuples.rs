use itertools::Itertools;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::iter::once;

use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::orderings::exhaustive::exhaustive_orderings;
use malachite_base::tuples::exhaustive::{
    lex_pairs, lex_triples_from_single, lex_triples_xxy, lex_triples_xyx,
};

fn lex_triples_xxy_helper<
    X: Clone + Debug + Eq,
    I: Clone + Iterator<Item = X>,
    Y: Clone + Debug + Eq,
    J: Clone + Iterator<Item = Y>,
>(
    xs: I,
    ys: J,
    out_len: usize,
    out: &[(X, X, Y)],
) {
    let ts = lex_triples_xxy(xs, ys);
    let ts_prefix = ts.clone().take(20).collect_vec();
    assert_eq!(ts_prefix.as_slice(), out);
    assert_eq!(ts.count(), out_len);
}

#[test]
fn test_lex_triples_xxy() {
    lex_triples_xxy_helper(nevers(), nevers(), 0, &[]);
    lex_triples_xxy_helper(nevers(), 0..4, 0, &[]);
    lex_triples_xxy_helper(once('a'), once(1), 1, &[('a', 'a', 1)]);
    lex_triples_xxy_helper(
        once('a'),
        0..4,
        4,
        &[('a', 'a', 0), ('a', 'a', 1), ('a', 'a', 2), ('a', 'a', 3)],
    );
    lex_triples_xxy_helper(
        exhaustive_unsigneds::<u8>(),
        'a'..'e',
        1 << 18,
        &[
            (0, 0, 'a'),
            (0, 0, 'b'),
            (0, 0, 'c'),
            (0, 0, 'd'),
            (0, 1, 'a'),
            (0, 1, 'b'),
            (0, 1, 'c'),
            (0, 1, 'd'),
            (0, 2, 'a'),
            (0, 2, 'b'),
            (0, 2, 'c'),
            (0, 2, 'd'),
            (0, 3, 'a'),
            (0, 3, 'b'),
            (0, 3, 'c'),
            (0, 3, 'd'),
            (0, 4, 'a'),
            (0, 4, 'b'),
            (0, 4, 'c'),
            (0, 4, 'd'),
        ],
    );
    lex_triples_xxy_helper(
        exhaustive_bools(),
        0..4,
        16,
        &[
            (false, false, 0),
            (false, false, 1),
            (false, false, 2),
            (false, false, 3),
            (false, true, 0),
            (false, true, 1),
            (false, true, 2),
            (false, true, 3),
            (true, false, 0),
            (true, false, 1),
            (true, false, 2),
            (true, false, 3),
            (true, true, 0),
            (true, true, 1),
            (true, true, 2),
            (true, true, 3),
        ],
    );
    lex_triples_xxy_helper(
        'a'..'f',
        0..3,
        75,
        &[
            ('a', 'a', 0),
            ('a', 'a', 1),
            ('a', 'a', 2),
            ('a', 'b', 0),
            ('a', 'b', 1),
            ('a', 'b', 2),
            ('a', 'c', 0),
            ('a', 'c', 1),
            ('a', 'c', 2),
            ('a', 'd', 0),
            ('a', 'd', 1),
            ('a', 'd', 2),
            ('a', 'e', 0),
            ('a', 'e', 1),
            ('a', 'e', 2),
            ('b', 'a', 0),
            ('b', 'a', 1),
            ('b', 'a', 2),
            ('b', 'b', 0),
            ('b', 'b', 1),
        ],
    );
    lex_triples_xxy_helper(
        ['a', 'b', 'c'].iter().cloned(),
        exhaustive_orderings(),
        27,
        &[
            ('a', 'a', Ordering::Equal),
            ('a', 'a', Ordering::Less),
            ('a', 'a', Ordering::Greater),
            ('a', 'b', Ordering::Equal),
            ('a', 'b', Ordering::Less),
            ('a', 'b', Ordering::Greater),
            ('a', 'c', Ordering::Equal),
            ('a', 'c', Ordering::Less),
            ('a', 'c', Ordering::Greater),
            ('b', 'a', Ordering::Equal),
            ('b', 'a', Ordering::Less),
            ('b', 'a', Ordering::Greater),
            ('b', 'b', Ordering::Equal),
            ('b', 'b', Ordering::Less),
            ('b', 'b', Ordering::Greater),
            ('b', 'c', Ordering::Equal),
            ('b', 'c', Ordering::Less),
            ('b', 'c', Ordering::Greater),
            ('c', 'a', Ordering::Equal),
            ('c', 'a', Ordering::Less),
        ],
    );
    lex_triples_xxy_helper(
        lex_pairs(exhaustive_orderings(), exhaustive_bools()),
        lex_triples_from_single([Ordering::Less, Ordering::Greater].iter().cloned()),
        288,
        &[
            (
                (Ordering::Equal, false),
                (Ordering::Equal, false),
                (Ordering::Less, Ordering::Less, Ordering::Less),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Equal, false),
                (Ordering::Less, Ordering::Less, Ordering::Greater),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Equal, false),
                (Ordering::Less, Ordering::Greater, Ordering::Less),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Equal, false),
                (Ordering::Less, Ordering::Greater, Ordering::Greater),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Equal, false),
                (Ordering::Greater, Ordering::Less, Ordering::Less),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Equal, false),
                (Ordering::Greater, Ordering::Less, Ordering::Greater),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Equal, false),
                (Ordering::Greater, Ordering::Greater, Ordering::Less),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Equal, false),
                (Ordering::Greater, Ordering::Greater, Ordering::Greater),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Equal, true),
                (Ordering::Less, Ordering::Less, Ordering::Less),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Equal, true),
                (Ordering::Less, Ordering::Less, Ordering::Greater),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Equal, true),
                (Ordering::Less, Ordering::Greater, Ordering::Less),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Equal, true),
                (Ordering::Less, Ordering::Greater, Ordering::Greater),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Equal, true),
                (Ordering::Greater, Ordering::Less, Ordering::Less),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Equal, true),
                (Ordering::Greater, Ordering::Less, Ordering::Greater),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Equal, true),
                (Ordering::Greater, Ordering::Greater, Ordering::Less),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Equal, true),
                (Ordering::Greater, Ordering::Greater, Ordering::Greater),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Less, false),
                (Ordering::Less, Ordering::Less, Ordering::Less),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Less, false),
                (Ordering::Less, Ordering::Less, Ordering::Greater),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Less, false),
                (Ordering::Less, Ordering::Greater, Ordering::Less),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Less, false),
                (Ordering::Less, Ordering::Greater, Ordering::Greater),
            ),
        ],
    );
}

fn lex_triples_xyx_helper<
    X: Clone + Debug + Eq,
    I: Clone + Iterator<Item = X>,
    Y: Clone + Debug + Eq,
    J: Clone + Iterator<Item = Y>,
>(
    xs: I,
    ys: J,
    out_len: usize,
    out: &[(X, Y, X)],
) {
    let ts = lex_triples_xyx(xs, ys);
    let ts_prefix = ts.clone().take(20).collect_vec();
    assert_eq!(ts_prefix.as_slice(), out);
    assert_eq!(ts.count(), out_len);
}

#[test]
fn test_lex_triples_xyx() {
    lex_triples_xyx_helper(nevers(), nevers(), 0, &[]);
    lex_triples_xyx_helper(nevers(), 0..4, 0, &[]);
    lex_triples_xyx_helper(once('a'), once(1), 1, &[('a', 1, 'a')]);
    lex_triples_xyx_helper(
        once('a'),
        0..4,
        4,
        &[('a', 0, 'a'), ('a', 1, 'a'), ('a', 2, 'a'), ('a', 3, 'a')],
    );
    lex_triples_xyx_helper(
        exhaustive_unsigneds::<u8>(),
        'a'..'e',
        1 << 18,
        &[
            (0, 'a', 0),
            (0, 'a', 1),
            (0, 'a', 2),
            (0, 'a', 3),
            (0, 'a', 4),
            (0, 'a', 5),
            (0, 'a', 6),
            (0, 'a', 7),
            (0, 'a', 8),
            (0, 'a', 9),
            (0, 'a', 10),
            (0, 'a', 11),
            (0, 'a', 12),
            (0, 'a', 13),
            (0, 'a', 14),
            (0, 'a', 15),
            (0, 'a', 16),
            (0, 'a', 17),
            (0, 'a', 18),
            (0, 'a', 19),
        ],
    );
    lex_triples_xyx_helper(
        exhaustive_bools(),
        0..4,
        16,
        &[
            (false, 0, false),
            (false, 0, true),
            (false, 1, false),
            (false, 1, true),
            (false, 2, false),
            (false, 2, true),
            (false, 3, false),
            (false, 3, true),
            (true, 0, false),
            (true, 0, true),
            (true, 1, false),
            (true, 1, true),
            (true, 2, false),
            (true, 2, true),
            (true, 3, false),
            (true, 3, true),
        ],
    );
    lex_triples_xyx_helper(
        'a'..'f',
        0..3,
        75,
        &[
            ('a', 0, 'a'),
            ('a', 0, 'b'),
            ('a', 0, 'c'),
            ('a', 0, 'd'),
            ('a', 0, 'e'),
            ('a', 1, 'a'),
            ('a', 1, 'b'),
            ('a', 1, 'c'),
            ('a', 1, 'd'),
            ('a', 1, 'e'),
            ('a', 2, 'a'),
            ('a', 2, 'b'),
            ('a', 2, 'c'),
            ('a', 2, 'd'),
            ('a', 2, 'e'),
            ('b', 0, 'a'),
            ('b', 0, 'b'),
            ('b', 0, 'c'),
            ('b', 0, 'd'),
            ('b', 0, 'e'),
        ],
    );
    lex_triples_xyx_helper(
        ['a', 'b', 'c'].iter().cloned(),
        exhaustive_orderings(),
        27,
        &[
            ('a', Ordering::Equal, 'a'),
            ('a', Ordering::Equal, 'b'),
            ('a', Ordering::Equal, 'c'),
            ('a', Ordering::Less, 'a'),
            ('a', Ordering::Less, 'b'),
            ('a', Ordering::Less, 'c'),
            ('a', Ordering::Greater, 'a'),
            ('a', Ordering::Greater, 'b'),
            ('a', Ordering::Greater, 'c'),
            ('b', Ordering::Equal, 'a'),
            ('b', Ordering::Equal, 'b'),
            ('b', Ordering::Equal, 'c'),
            ('b', Ordering::Less, 'a'),
            ('b', Ordering::Less, 'b'),
            ('b', Ordering::Less, 'c'),
            ('b', Ordering::Greater, 'a'),
            ('b', Ordering::Greater, 'b'),
            ('b', Ordering::Greater, 'c'),
            ('c', Ordering::Equal, 'a'),
            ('c', Ordering::Equal, 'b'),
        ],
    );
    lex_triples_xyx_helper(
        lex_pairs(exhaustive_orderings(), exhaustive_bools()),
        lex_triples_from_single([Ordering::Less, Ordering::Greater].iter().cloned()),
        288,
        &[
            (
                (Ordering::Equal, false),
                (Ordering::Less, Ordering::Less, Ordering::Less),
                (Ordering::Equal, false),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Less, Ordering::Less, Ordering::Less),
                (Ordering::Equal, true),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Less, Ordering::Less, Ordering::Less),
                (Ordering::Less, false),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Less, Ordering::Less, Ordering::Less),
                (Ordering::Less, true),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Less, Ordering::Less, Ordering::Less),
                (Ordering::Greater, false),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Less, Ordering::Less, Ordering::Less),
                (Ordering::Greater, true),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Less, Ordering::Less, Ordering::Greater),
                (Ordering::Equal, false),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Less, Ordering::Less, Ordering::Greater),
                (Ordering::Equal, true),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Less, Ordering::Less, Ordering::Greater),
                (Ordering::Less, false),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Less, Ordering::Less, Ordering::Greater),
                (Ordering::Less, true),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Less, Ordering::Less, Ordering::Greater),
                (Ordering::Greater, false),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Less, Ordering::Less, Ordering::Greater),
                (Ordering::Greater, true),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Less, Ordering::Greater, Ordering::Less),
                (Ordering::Equal, false),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Less, Ordering::Greater, Ordering::Less),
                (Ordering::Equal, true),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Less, Ordering::Greater, Ordering::Less),
                (Ordering::Less, false),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Less, Ordering::Greater, Ordering::Less),
                (Ordering::Less, true),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Less, Ordering::Greater, Ordering::Less),
                (Ordering::Greater, false),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Less, Ordering::Greater, Ordering::Less),
                (Ordering::Greater, true),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Less, Ordering::Greater, Ordering::Greater),
                (Ordering::Equal, false),
            ),
            (
                (Ordering::Equal, false),
                (Ordering::Less, Ordering::Greater, Ordering::Greater),
                (Ordering::Equal, true),
            ),
        ],
    );
}
