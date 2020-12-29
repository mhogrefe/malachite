use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;

#[test]
fn test_exhaustive_bools() {
    assert_eq!(exhaustive_bools().collect_vec(), &[false, true]);
}
