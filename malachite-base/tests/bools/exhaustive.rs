use malachite_base::bools::exhaustive::exhaustive_bools;

#[test]
fn test_exhaustive_bools() {
    assert_eq!(exhaustive_bools().collect::<Vec<_>>(), &[false, true]);
}
