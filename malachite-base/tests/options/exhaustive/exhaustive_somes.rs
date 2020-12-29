use std::fmt::Debug;

use itertools::Itertools;
use malachite_base::nevers::Never;
use malachite_base::options::exhaustive::exhaustive_somes;

fn exhaustive_somes_helper<T: Clone + Debug + Eq>(xs: &[T], out: &[Option<T>]) {
    assert_eq!(
        exhaustive_somes(xs.iter().cloned())
            .collect_vec()
            .as_slice(),
        out
    );
}

#[test]
fn test_exhaustive_somes() {
    exhaustive_somes_helper::<Never>(&[], &[]);
    exhaustive_somes_helper(&[5], &[Some(5)]);
    exhaustive_somes_helper(&[1, 2, 3], &[Some(1), Some(2), Some(3)]);
    exhaustive_somes_helper(
        &[Some(2), None, Some(5)],
        &[Some(Some(2)), Some(None), Some(Some(5))],
    );
}
