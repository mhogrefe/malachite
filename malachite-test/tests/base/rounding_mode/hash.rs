use common::test_properties_no_limit_exhaustive;
use malachite_test::hash::hash;
use malachite_test::inputs::base::rounding_modes;

#[test]
#[allow(unknown_lints, clone_on_copy)]
fn hash_properties() {
    test_properties_no_limit_exhaustive(rounding_modes, |rm| {
        assert_eq!(hash(&rm), hash(&rm.clone()));
    });
}
