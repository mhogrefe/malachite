use malachite_base::nevers::nevers;

#[test]
fn test_nevers() {
    assert_eq!(nevers().collect::<Vec<_>>(), &[]);
}
