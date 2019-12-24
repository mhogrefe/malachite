use malachite_test::common::test_properties;
use malachite_test::hash::hash;
use malachite_test::inputs::natural::naturals;

#[test]
fn hash_properties() {
    test_properties(naturals, |x| {
        assert_eq!(hash(x), hash(&x.clone()));
    });
}
