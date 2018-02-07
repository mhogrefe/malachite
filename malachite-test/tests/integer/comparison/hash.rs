use common::test_properties;
use malachite_test::hash::hash;
use malachite_test::inputs::integer::integers;

#[test]
fn hash_properties() {
    test_properties(integers, |x| {
        assert_eq!(hash(x), hash(&x.clone()));
    });
}
