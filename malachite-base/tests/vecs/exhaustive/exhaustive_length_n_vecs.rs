use std::fmt::Debug;
use std::iter::{empty, once};

use malachite_base::nevers::nevers;
use malachite_base::vecs::exhaustive::exhaustive_length_2_vecs;

fn exhaustive_length_2_vecs_helper<T, I: Iterator<Item = T>, J: Iterator<Item = T>>(
    xs: I,
    ys: J,
    out: &[&[T]],
) where
    I::Item: Clone + Debug + Eq,
{
    let xss = exhaustive_length_2_vecs(xs, ys)
        .take(20)
        .collect::<Vec<_>>();
    assert_eq!(
        xss.iter().map(Vec::as_slice).collect::<Vec<_>>().as_slice(),
        out
    );
}

#[test]
fn test_exhaustive_length_2_vecs() {
    exhaustive_length_2_vecs_helper(nevers(), nevers(), &[]);
    exhaustive_length_2_vecs_helper(empty(), 0..4, &[]);
    exhaustive_length_2_vecs_helper(once(0), once(1), &[&[0, 1]]);
    exhaustive_length_2_vecs_helper(once(0), 0..4, &[&[0, 0], &[0, 1], &[0, 2], &[0, 3]]);
    exhaustive_length_2_vecs_helper(
        0..2,
        0..4,
        &[
            &[0, 0],
            &[0, 1],
            &[1, 0],
            &[1, 1],
            &[0, 2],
            &[0, 3],
            &[1, 2],
            &[1, 3],
        ],
    );
    exhaustive_length_2_vecs_helper(
        0..5,
        0..3,
        &[
            &[0, 0],
            &[0, 1],
            &[1, 0],
            &[1, 1],
            &[0, 2],
            &[1, 2],
            &[2, 0],
            &[2, 1],
            &[3, 0],
            &[3, 1],
            &[2, 2],
            &[3, 2],
            &[4, 0],
            &[4, 1],
            &[4, 2],
        ],
    );
    exhaustive_length_2_vecs_helper(
        ['a', 'b', 'c'].iter().cloned(),
        ['x', 'y', 'z'].iter().cloned(),
        &[
            &['a', 'x'],
            &['a', 'y'],
            &['b', 'x'],
            &['b', 'y'],
            &['a', 'z'],
            &['b', 'z'],
            &['c', 'x'],
            &['c', 'y'],
            &['c', 'z'],
        ],
    );
}
