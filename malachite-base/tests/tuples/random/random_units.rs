use malachite_base::tuples::random::random_units;

#[test]
fn test_random_units() {
    assert_eq!(random_units().take(20).collect::<Vec<_>>(), &[(); 20]);
}
