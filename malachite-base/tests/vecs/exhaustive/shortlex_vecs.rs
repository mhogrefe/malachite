use std::fmt::Debug;

use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::shortlex_vecs;

fn shortlex_vecs_helper<I: Clone + Iterator>(xs: I, out: &[&[I::Item]])
where
    I::Item: Clone + Debug + Eq,
{
    let xss = shortlex_vecs(xs).take(20).collect::<Vec<_>>();
    assert_eq!(
        xss.iter().map(Vec::as_slice).collect::<Vec<_>>().as_slice(),
        out
    );
}

#[test]
fn test_shortlex_vecs() {
    shortlex_vecs_helper(nevers(), &[&[]]);
    shortlex_vecs_helper(
        exhaustive_units(),
        &[
            &[],
            &[()],
            &[(), ()],
            &[(), (), ()],
            &[(), (), (), ()],
            &[(), (), (), (), ()],
            &[(), (), (), (), (), ()],
            &[(), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(); 16],
            &[(); 17],
            &[(); 18],
            &[(); 19],
        ],
    );
    shortlex_vecs_helper(
        exhaustive_bools(),
        &[
            &[],
            &[false],
            &[true],
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
            &[false, false, false, false],
            &[false, false, false, true],
            &[false, false, true, false],
            &[false, false, true, true],
            &[false, true, false, false],
        ],
    );
    shortlex_vecs_helper(
        'a'..='c',
        &[
            &[],
            &['a'],
            &['b'],
            &['c'],
            &['a', 'a'],
            &['a', 'b'],
            &['a', 'c'],
            &['b', 'a'],
            &['b', 'b'],
            &['b', 'c'],
            &['c', 'a'],
            &['c', 'b'],
            &['c', 'c'],
            &['a', 'a', 'a'],
            &['a', 'a', 'b'],
            &['a', 'a', 'c'],
            &['a', 'b', 'a'],
            &['a', 'b', 'b'],
            &['a', 'b', 'c'],
            &['a', 'c', 'a'],
        ],
    );
    shortlex_vecs_helper(
        exhaustive_ascii_chars(),
        &[
            &[],
            &['a'],
            &['b'],
            &['c'],
            &['d'],
            &['e'],
            &['f'],
            &['g'],
            &['h'],
            &['i'],
            &['j'],
            &['k'],
            &['l'],
            &['m'],
            &['n'],
            &['o'],
            &['p'],
            &['q'],
            &['r'],
            &['s'],
        ],
    );
}
