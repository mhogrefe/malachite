use malachite_base::round::RoundingMode;

use malachite_test::common::{test_eq_helper, test_properties_no_limit_exhaustive_no_special};
use malachite_test::inputs::base::{
    pairs_of_rounding_modes, rounding_modes, triples_of_rounding_modes,
};

#[test]
fn test_eq() {
    let strings = vec!["Down", "Up", "Floor", "Ceiling", "Nearest", "Exact"];
    test_eq_helper::<RoundingMode>(&strings);
}

#[test]
fn eq_properties() {
    test_properties_no_limit_exhaustive_no_special(pairs_of_rounding_modes, |&(x, y)| {
        assert_eq!(x == y, y == x);
    });

    test_properties_no_limit_exhaustive_no_special(rounding_modes, |&rm| {
        assert_eq!(rm, rm);
    });

    test_properties_no_limit_exhaustive_no_special(triples_of_rounding_modes, |&(x, y, z)| {
        if x == y && x == z {
            assert_eq!(x, z);
        }
    });
}
