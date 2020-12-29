use itertools::Itertools;
use malachite_base::tuples::exhaustive::exhaustive_units;

#[test]
fn test_exhaustive_units() {
    assert_eq!(exhaustive_units().collect_vec(), &[()]);
}
