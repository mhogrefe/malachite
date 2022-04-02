use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::common::test_cmp_helper;
use malachite_base::test_util::generators::{
    rounding_mode_gen, rounding_mode_pair_gen, rounding_mode_triple_gen,
};
use std::cmp::Ordering;

#[test]
fn test_cmp() {
    test_cmp_helper::<RoundingMode>(&["Down", "Up", "Floor", "Ceiling", "Nearest", "Exact"]);
}

#[test]
fn cmp_properties() {
    rounding_mode_pair_gen().test_properties(|(x, y)| {
        let ord = x.cmp(&y);
        assert_eq!(y.cmp(&x).reverse(), ord);
    });

    rounding_mode_gen().test_properties(|x| {
        assert_eq!(x.cmp(&x), Ordering::Equal);
    });

    rounding_mode_triple_gen().test_properties(|(x, y, z)| {
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    });
}
