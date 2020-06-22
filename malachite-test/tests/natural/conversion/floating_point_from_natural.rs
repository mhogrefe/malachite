use malachite_base::crement::Crementable;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, RoundingFrom,
};
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::natural::Natural;

use malachite_test::common::test_properties;
use malachite_test::inputs::natural::{
    naturals, naturals_exactly_equal_to_f32, naturals_exactly_equal_to_f64,
    naturals_not_exactly_equal_to_f32, naturals_not_exactly_equal_to_f64, naturals_var_2_f32,
    naturals_var_2_f64, pairs_of_natural_and_rounding_mode_var_1_f32,
    pairs_of_natural_and_rounding_mode_var_1_f64,
};

macro_rules! float_properties {
    (
        $f: ident,
        $pairs_of_natural_and_rounding_mode_var_1: ident,
        $naturals_exactly_equal_to_float: ident,
        $naturals_not_exactly_equal_to_float: ident,
        $naturals_var_2: ident,
        $float_rounding_from_natural_properties: ident,
        $float_from_natural_properties: ident,
        $float_checked_from_natural_properties: ident,
        $float_convertible_from_natural_properties: ident,
    ) => {
        #[test]
        fn $float_rounding_from_natural_properties() {
            test_properties($pairs_of_natural_and_rounding_mode_var_1, |&(ref n, rm)| {
                let f = $f::rounding_from(n, rm);
                assert_eq!($f::rounding_from(n.clone(), rm), f);
            });

            test_properties($naturals_exactly_equal_to_float, |n| {
                let f = $f::rounding_from(n, RoundingMode::Exact);
                assert_eq!($f::rounding_from(n.clone(), RoundingMode::Exact), f);
                assert_eq!(f, $f::rounding_from(n, RoundingMode::Floor));
                assert_eq!(f, $f::rounding_from(n, RoundingMode::Ceiling));
                assert_eq!(f, $f::rounding_from(n, RoundingMode::Down));
                assert_eq!(f, $f::rounding_from(n, RoundingMode::Up));
                assert_eq!(f, $f::rounding_from(n, RoundingMode::Nearest));
                assert_eq!(f, $f::rounding_from(n.clone(), RoundingMode::Floor));
                assert_eq!(f, $f::rounding_from(n.clone(), RoundingMode::Ceiling));
                assert_eq!(f, $f::rounding_from(n.clone(), RoundingMode::Down));
                assert_eq!(f, $f::rounding_from(n.clone(), RoundingMode::Up));
                assert_eq!(f, $f::rounding_from(n.clone(), RoundingMode::Nearest));
                assert_eq!(Natural::rounding_from(f, RoundingMode::Exact), *n);
            });

            test_properties($naturals_not_exactly_equal_to_float, |n| {
                let f_below = $f::rounding_from(n, RoundingMode::Floor);
                assert_eq!($f::rounding_from(n.clone(), RoundingMode::Floor), f_below);
                let mut f_above = f_below;
                f_above.increment();
                assert_eq!(f_above, $f::rounding_from(n, RoundingMode::Ceiling));
                assert_eq!(f_above, $f::rounding_from(n.clone(), RoundingMode::Ceiling));
                assert_eq!(f_below, $f::rounding_from(n, RoundingMode::Down));
                assert_eq!(f_below, $f::rounding_from(n.clone(), RoundingMode::Down));
                assert_eq!(f_above, $f::rounding_from(n, RoundingMode::Up));
                assert_eq!(f_above, $f::rounding_from(n.clone(), RoundingMode::Up));
                let f_nearest = $f::rounding_from(n, RoundingMode::Nearest);
                assert_eq!(
                    $f::rounding_from(n.clone(), RoundingMode::Nearest),
                    f_nearest
                );
                assert!(f_nearest == f_below || f_nearest == f_above);
                assert_ne!(Natural::from(f_nearest), *n);
            });

            test_properties($naturals_var_2, |n| {
                let floor = $f::rounding_from(n, RoundingMode::Floor);
                let mut ceiling = floor;
                ceiling.increment();
                let nearest = $f::rounding_from(n, RoundingMode::Nearest);
                assert_eq!(
                    nearest,
                    if floor.to_bits().even() {
                        floor
                    } else {
                        ceiling
                    }
                );
            });
        }

        #[test]
        fn $float_from_natural_properties() {
            test_properties(naturals, |n| {
                let f = $f::from(n);
                assert_eq!($f::from(n.clone()), f);
                assert_eq!($f::rounding_from(n, RoundingMode::Nearest), f);
            });

            test_properties($naturals_exactly_equal_to_float, |n| {
                let f = $f::from(n);
                assert_eq!($f::from(n.clone()), f);
                assert_eq!(Natural::from(f), *n);
            });

            test_properties($naturals_not_exactly_equal_to_float, |n| {
                let f_below = $f::rounding_from(n, RoundingMode::Floor);
                assert_eq!($f::rounding_from(n.clone(), RoundingMode::Floor), f_below);
                let mut f_above = f_below;
                f_above.increment();
                let f_nearest = $f::from(n);
                assert_eq!($f::from(n.clone()), f_nearest);
                assert!(f_nearest == f_below || f_nearest == f_above);
                assert_ne!(Natural::from(f_nearest), *n);
            });

            test_properties($naturals_var_2, |n| {
                let floor = $f::rounding_from(n, RoundingMode::Floor);
                let mut ceiling = floor;
                ceiling.increment();
                let nearest = $f::from(n);
                assert_eq!(
                    nearest,
                    if floor.to_bits().even() {
                        floor
                    } else {
                        ceiling
                    }
                );
            });
        }

        #[test]
        fn $float_checked_from_natural_properties() {
            test_properties(naturals, |n| {
                let of = $f::checked_from(n);
                assert_eq!($f::checked_from(n.clone()), of);
            });

            test_properties($naturals_exactly_equal_to_float, |n| {
                let f = $f::exact_from(n);
                assert_eq!($f::exact_from(n.clone()), f);
                assert_eq!(f, $f::rounding_from(n, RoundingMode::Exact));
                assert_eq!(Natural::rounding_from(f, RoundingMode::Exact), *n);
            });

            test_properties($naturals_not_exactly_equal_to_float, |n| {
                assert!($f::checked_from(n).is_none());
            });

            test_properties($naturals_var_2, |n| {
                assert!($f::checked_from(n).is_none());
            });
        }

        #[test]
        fn $float_convertible_from_natural_properties() {
            test_properties(naturals, |n| {
                $f::convertible_from(n);
            });

            test_properties($naturals_exactly_equal_to_float, |n| {
                assert!($f::convertible_from(n));
            });

            test_properties($naturals_not_exactly_equal_to_float, |n| {
                assert!(!$f::convertible_from(n));
            });

            test_properties($naturals_var_2, |n| {
                assert!(!$f::convertible_from(n));
            });
        }
    };
}

float_properties!(
    f32,
    pairs_of_natural_and_rounding_mode_var_1_f32,
    naturals_exactly_equal_to_f32,
    naturals_not_exactly_equal_to_f32,
    naturals_var_2_f32,
    f32_rounding_from_natural_properties,
    f32_from_natural_properties,
    f32_checked_from_natural_properties,
    f32_convertible_from_natural_properties,
);
float_properties!(
    f64,
    pairs_of_natural_and_rounding_mode_var_1_f64,
    naturals_exactly_equal_to_f64,
    naturals_not_exactly_equal_to_f64,
    naturals_var_2_f64,
    f64_rounding_from_natural_properties,
    f64_from_natural_properties,
    f64_checked_from_natural_properties,
    f64_convertible_from_natural_properties,
);
