use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, RoundingFrom,
};
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::natural::Natural;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    f32s, f32s_var_1, f64s, f64s_var_1, pairs_of_finite_f32_and_rounding_mode_var_1,
    pairs_of_finite_f64_and_rounding_mode_var_1,
};
use malachite_test::inputs::natural::{
    f32s_exactly_equal_to_natural, f32s_var_2, f32s_var_3, f64s_exactly_equal_to_natural,
    f64s_var_2, f64s_var_3,
};

macro_rules! float_properties {
    (
        $f: ident,
        $pairs_of_float_and_rounding_mode_var_1: ident,
        $floats: ident,
        $floats_exactly_equal_to_natural: ident,
        $floats_var_1: ident,
        $floats_var_2: ident,
        $floats_var_3: ident,
        $rounding_from_float_properties: ident,
        $from_float_properties: ident,
        $checked_from_float_properties: ident,
        $convertible_from_float_properties: ident
    ) => {
        #[test]
        fn $rounding_from_float_properties() {
            test_properties($pairs_of_float_and_rounding_mode_var_1, |&(f, rm)| {
                let n = Natural::rounding_from(f, rm);
                assert!(n.is_valid());
            });

            test_properties($floats_exactly_equal_to_natural, |&f| {
                let n = Natural::rounding_from(f, RoundingMode::Exact);
                assert!(n.is_valid());
                assert_eq!(n, Natural::rounding_from(f, RoundingMode::Floor));
                assert_eq!(n, Natural::rounding_from(f, RoundingMode::Ceiling));
                assert_eq!(n, Natural::rounding_from(f, RoundingMode::Down));
                assert_eq!(n, Natural::rounding_from(f, RoundingMode::Up));
                assert_eq!(n, Natural::rounding_from(f, RoundingMode::Nearest));
                assert_eq!($f::rounding_from(n, RoundingMode::Exact), f);
            });

            test_properties($floats_var_2, |&f| {
                let n_floor = Natural::rounding_from(f, RoundingMode::Floor);
                assert!(n_floor.is_valid());
                let n_ceiling = &n_floor + Natural::ONE;
                assert_eq!(n_ceiling, Natural::rounding_from(f, RoundingMode::Ceiling));
                assert_eq!(n_floor, Natural::rounding_from(f, RoundingMode::Down));
                assert_eq!(n_ceiling, Natural::rounding_from(f, RoundingMode::Up));
                let n_nearest = Natural::rounding_from(f, RoundingMode::Nearest);
                assert!(n_nearest == n_floor || n_nearest == n_ceiling);
                assert_ne!($f::from(n_nearest), f);
            });

            test_properties($floats_var_3, |&f| {
                let floor = Natural::rounding_from(f, RoundingMode::Floor);
                let ceiling = &floor + Natural::ONE;
                let nearest = Natural::rounding_from(f, RoundingMode::Nearest);
                assert_eq!(nearest, if floor.even() { floor } else { ceiling });
            });
        }

        #[test]
        fn $from_float_properties() {
            test_properties($floats_var_1, |&f| {
                let n = Natural::from(f);
                assert!(n.is_valid());
                assert_eq!(n, Natural::rounding_from(f, RoundingMode::Nearest));
            });

            test_properties($floats_exactly_equal_to_natural, |&f| {
                let n = Natural::from(f);
                assert!(n.is_valid());
                assert_eq!($f::from(n), f);
            });

            test_properties($floats_var_2, |&f| {
                let n_floor = Natural::rounding_from(f, RoundingMode::Floor);
                assert!(n_floor.is_valid());
                let n_ceiling = &n_floor + Natural::ONE;
                let n_nearest = Natural::from(f);
                assert!(n_nearest == n_floor || n_nearest == n_ceiling);
            });

            test_properties($floats_var_3, |&f| {
                let floor = Natural::rounding_from(f, RoundingMode::Floor);
                let ceiling = &floor + Natural::ONE;
                let nearest = Natural::from(f);
                assert_eq!(nearest, if floor.even() { floor } else { ceiling });
            });
        }

        #[test]
        fn $checked_from_float_properties() {
            test_properties($floats, |&f| {
                let on = Natural::checked_from(f);
                assert!(on.map_or(true, |n| n.is_valid()));
            });

            test_properties($floats_exactly_equal_to_natural, |&f| {
                let n = Natural::exact_from(f);
                assert!(n.is_valid());
                assert_eq!(n, Natural::rounding_from(f, RoundingMode::Exact));
                assert_eq!($f::rounding_from(n, RoundingMode::Exact), f);
            });

            test_properties($floats_var_2, |&f| {
                assert!(Natural::checked_from(f).is_none());
            });

            test_properties($floats_var_3, |&f| {
                assert!(Natural::checked_from(f).is_none());
            });
        }

        #[test]
        fn $convertible_from_float_properties() {
            test_properties($floats, |&f| {
                Natural::convertible_from(f);
            });

            test_properties($floats_exactly_equal_to_natural, |&f| {
                assert!(Natural::convertible_from(f));
            });

            test_properties($floats_var_2, |&f| {
                assert!(!Natural::convertible_from(f));
            });

            test_properties($floats_var_3, |&f| {
                assert!(!Natural::convertible_from(f));
            });
        }
    };
}

float_properties!(
    f32,
    pairs_of_finite_f32_and_rounding_mode_var_1,
    f32s,
    f32s_exactly_equal_to_natural,
    f32s_var_1,
    f32s_var_2,
    f32s_var_3,
    rounding_from_f32_properties,
    from_f32_properties,
    checked_from_f32_properties,
    convertible_from_f32_properties
);
float_properties!(
    f64,
    pairs_of_finite_f64_and_rounding_mode_var_1,
    f64s,
    f64s_exactly_equal_to_natural,
    f64s_var_1,
    f64s_var_2,
    f64s_var_3,
    rounding_from_f64_properties,
    from_f64_properties,
    checked_from_f64_properties,
    convertible_from_f64_properties
);
