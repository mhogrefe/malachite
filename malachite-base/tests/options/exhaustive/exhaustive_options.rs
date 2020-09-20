use std::fmt::Debug;

use malachite_base::nevers::Never;
use malachite_base::options::exhaustive::exhaustive_options;

fn exhaustive_options_helper<T: Clone + Debug + Eq>(xs: &[T], out: &[Option<T>]) {
    assert_eq!(
        exhaustive_options(xs.iter().cloned())
            .collect::<Vec<_>>()
            .as_slice(),
        out
    );
}

#[test]
fn test_exhaustive_options() {
    exhaustive_options_helper::<Never>(&[], &[None]);
    exhaustive_options_helper(&[5], &[None, Some(5)]);
    exhaustive_options_helper(&[1, 2, 3], &[None, Some(1), Some(2), Some(3)]);
    exhaustive_options_helper(
        &[Some(2), None, Some(5)],
        &[None, Some(Some(2)), Some(None), Some(Some(5))],
    );
}
