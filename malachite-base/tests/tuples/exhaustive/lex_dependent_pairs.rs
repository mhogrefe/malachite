use core::hash::Hash;
use itertools::{chain, Itertools};
use malachite_base::tuples::exhaustive::{
    lex_dependent_pairs, lex_dependent_pairs_stop_after_empty_ys,
    ExhaustiveDependentPairsYsGenerator,
};
use std::collections::HashMap;
use std::fmt::Debug;
use std::iter::{once, repeat, Cloned};
use std::slice::Iter;

#[derive(Clone, Debug)]
struct DPGeneratorFromMap<X: Clone + Eq + Hash, Y: 'static + Clone> {
    map: HashMap<X, &'static [Y]>,
}

impl<X: Clone + Eq + Hash, Y: 'static + Clone>
    ExhaustiveDependentPairsYsGenerator<X, Y, Cloned<Iter<'static, Y>>>
    for DPGeneratorFromMap<X, Y>
{
    #[inline]
    fn get_ys(&self, x: &X) -> Cloned<Iter<'static, Y>> {
        self.map[x].iter().cloned()
    }
}

fn lex_dependent_pairs_helper<I: Iterator, Y>(
    xs: I,
    map: HashMap<I::Item, &'static [Y]>,
    out: &[(I::Item, Y)],
) where
    I::Item: Clone + Debug + Eq + Hash,
    Y: Clone + Debug + Eq,
{
    let xss = lex_dependent_pairs(xs, DPGeneratorFromMap { map })
        .take(20)
        .collect_vec();
    assert_eq!(xss.as_slice(), out);
}

#[test]
fn test_lex_dependent_pairs() {
    lex_dependent_pairs_helper(
        [1, 2, 3].iter().cloned(),
        hashmap! {
            1 => &[100, 101, 102][..],
            2 => &[200, 201, 202][..],
            3 => &[300, 301, 302][..]
        },
        &[
            (1, 100),
            (1, 101),
            (1, 102),
            (2, 200),
            (2, 201),
            (2, 202),
            (3, 300),
            (3, 301),
            (3, 302),
        ],
    );
    lex_dependent_pairs_helper(
        ["cat", "dog", "mouse", "dog", "cat"].iter().cloned(),
        hashmap! { "cat" => &[2, 3, 4][..], "dog" => &[20][..], "mouse" => &[30, 40][..] },
        &[
            ("cat", 2),
            ("cat", 3),
            ("cat", 4),
            ("dog", 20),
            ("mouse", 30),
            ("mouse", 40),
            ("dog", 20),
            ("cat", 2),
            ("cat", 3),
            ("cat", 4),
        ],
    );
    lex_dependent_pairs_helper(
        [1, 2, 3, 2, 3, 2, 2].iter().cloned(),
        hashmap! { 1 => &[100, 101, 102][..], 2 => &[][..], 3 => &[300, 301, 302][..] },
        &[
            (1, 100),
            (1, 101),
            (1, 102),
            (3, 300),
            (3, 301),
            (3, 302),
            (3, 300),
            (3, 301),
            (3, 302),
        ],
    );
    lex_dependent_pairs_helper(
        [].iter().cloned(),
        hashmap! { 1 => &[100, 101, 102][..], 2 => &[][..], 3 => &[300, 301, 302][..] },
        &[],
    );
    lex_dependent_pairs_helper(
        [2, 2, 2, 2, 2].iter().cloned(),
        hashmap! { 1 => &[100, 101, 102][..], 2 => &[][..], 3 => &[300, 301, 302][..] },
        &[],
    );
}

fn lex_dependent_pairs_stop_after_empty_ys_helper<I: Iterator, Y>(
    xs: I,
    map: HashMap<I::Item, &'static [Y]>,
    out: &[(I::Item, Y)],
) where
    I::Item: Clone + Debug + Eq + Hash,
    Y: Clone + Debug + Eq,
{
    let xss = lex_dependent_pairs_stop_after_empty_ys(xs, DPGeneratorFromMap { map })
        .take(20)
        .collect_vec();
    assert_eq!(xss.as_slice(), out);
}

#[test]
fn test_lex_dependent_pairs_stop_after_empty_ys() {
    lex_dependent_pairs_stop_after_empty_ys_helper(
        [1, 2, 3].iter().cloned(),
        hashmap! {
            1 => &[100, 101, 102][..],
            2 => &[200, 201, 202][..],
            3 => &[300, 301, 302][..]
        },
        &[
            (1, 100),
            (1, 101),
            (1, 102),
            (2, 200),
            (2, 201),
            (2, 202),
            (3, 300),
            (3, 301),
            (3, 302),
        ],
    );
    lex_dependent_pairs_stop_after_empty_ys_helper(
        ["cat", "dog", "mouse", "dog", "cat"].iter().cloned(),
        hashmap! { "cat" => &[2, 3, 4][..], "dog" => &[20][..], "mouse" => &[30, 40][..] },
        &[
            ("cat", 2),
            ("cat", 3),
            ("cat", 4),
            ("dog", 20),
            ("mouse", 30),
            ("mouse", 40),
            ("dog", 20),
            ("cat", 2),
            ("cat", 3),
            ("cat", 4),
        ],
    );
    // Notice difference from `lex_dependent_pairs`
    lex_dependent_pairs_stop_after_empty_ys_helper(
        [1, 2, 3, 2, 3, 2, 2].iter().cloned(),
        hashmap! { 1 => &[100, 101, 102][..], 2 => &[][..], 3 => &[300, 301, 302][..] },
        &[(1, 100), (1, 101), (1, 102)],
    );
    lex_dependent_pairs_stop_after_empty_ys_helper(
        [].iter().cloned(),
        hashmap! { 1 => &[100, 101, 102][..], 2 => &[][..], 3 => &[300, 301, 302][..] },
        &[],
    );
    lex_dependent_pairs_stop_after_empty_ys_helper(
        [2, 2, 2, 2, 2].iter().cloned(),
        hashmap! { 1 => &[100, 101, 102][..], 2 => &[][..], 3 => &[300, 301, 302][..] },
        &[],
    );
    // With `lex_dependent_pairs` this would hang
    lex_dependent_pairs_stop_after_empty_ys_helper(
        chain(once(3), repeat(2)),
        hashmap! { 1 => &[100, 101, 102][..], 2 => &[][..], 3 => &[300, 301, 302][..] },
        &[(3, 300), (3, 301), (3, 302)],
    );
}
