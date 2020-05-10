use malachite_test::common::test_properties_no_limit_exhaustive_no_special;
use malachite_test::inputs::base::rounding_modes;

#[test]
fn neg_properties() {
    test_properties_no_limit_exhaustive_no_special(rounding_modes, |&rm| {
        assert_eq!(-(-rm), rm);
    });
}
