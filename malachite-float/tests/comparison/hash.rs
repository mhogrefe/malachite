use malachite_base::test_util::hash::hash;
use malachite_float::test_util::generators::float_gen;
use malachite_float::ComparableFloatRef;

#[test]
fn hash_properties() {
    float_gen().test_properties(|x| {
        assert_eq!(
            hash(&ComparableFloatRef(&x)),
            hash(&ComparableFloatRef(&x.clone()))
        );
    });
}
