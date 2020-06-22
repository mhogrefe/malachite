use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, RoundingFrom,
};
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::integer::Integer;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    finite_f32s, finite_f64s, pairs_of_finite_f32_and_rounding_mode_var_2,
    pairs_of_finite_f64_and_rounding_mode_var_2,
};
use malachite_test::inputs::integer::{
    f32s_exactly_equal_to_integer, f32s_var_4, f32s_var_5, f64s_exactly_equal_to_integer,
    f64s_var_4, f64s_var_5,
};

macro_rules! float_properties {
    (
        $f: ident,
        $finite_floats: ident,
        $pairs_of_float_and_rounding_mode_var_2: ident,
        $floats_exactly_equal_to_integer: ident,
        $floats_var_4: ident,
        $floats_var_5: ident,
        $rounding_from_float_properties: ident,
        $from_float_properties: ident,
        $checked_from_float_properties: ident,
        $convertible_from_float_properties: ident,
    ) => {
        #[test]
        fn $rounding_from_float_properties() {
            test_properties($pairs_of_float_and_rounding_mode_var_2, |&(f, rm)| {
                let n = Integer::rounding_from(f, rm);
                assert!(n.is_valid());
                assert_eq!(Integer::rounding_from(-f, -rm), -n);
            });

            test_properties($floats_exactly_equal_to_integer, |&f| {
                let n = Integer::rounding_from(f, RoundingMode::Exact);
                assert!(n.is_valid());
                assert_eq!(n, Integer::rounding_from(f, RoundingMode::Floor));
                assert_eq!(n, Integer::rounding_from(f, RoundingMode::Ceiling));
                assert_eq!(n, Integer::rounding_from(f, RoundingMode::Down));
                assert_eq!(n, Integer::rounding_from(f, RoundingMode::Up));
                assert_eq!(n, Integer::rounding_from(f, RoundingMode::Nearest));
                assert_eq!($f::rounding_from(n, RoundingMode::Exact), f);
            });

            test_properties($floats_var_4, |&f| {
                let n_floor = Integer::rounding_from(f, RoundingMode::Floor);
                assert!(n_floor.is_valid());
                let n_ceiling = &n_floor + Integer::ONE;
                assert_eq!(n_ceiling, Integer::rounding_from(f, RoundingMode::Ceiling));
                if f >= 0.0 {
                    assert_eq!(n_floor, Integer::rounding_from(f, RoundingMode::Down));
                    assert_eq!(n_ceiling, Integer::rounding_from(f, RoundingMode::Up));
                } else {
                    assert_eq!(n_ceiling, Integer::rounding_from(f, RoundingMode::Down));
                    assert_eq!(n_floor, Integer::rounding_from(f, RoundingMode::Up));
                }
                let n_nearest = Integer::rounding_from(f, RoundingMode::Nearest);
                assert!(n_nearest == n_floor || n_nearest == n_ceiling);
                assert_ne!($f::from(n_nearest), f);
            });

            test_properties($floats_var_5, |&f| {
                let floor = Integer::rounding_from(f, RoundingMode::Floor);
                let ceiling = &floor + Integer::ONE;
                let nearest = Integer::rounding_from(f, RoundingMode::Nearest);
                assert_eq!(nearest, if floor.even() { floor } else { ceiling });
            });
        }

        #[test]
        fn $from_float_properties() {
            test_properties($finite_floats, |&f| {
                let n = Integer::from(f);
                assert!(n.is_valid());
                assert_eq!(n, Integer::rounding_from(f, RoundingMode::Nearest));
                assert_eq!(Integer::from(-f), -n);
            });

            test_properties($floats_exactly_equal_to_integer, |&f| {
                let n = Integer::from(f);
                assert!(n.is_valid());
                assert_eq!($f::from(n), f);
            });

            test_properties($floats_var_4, |&f| {
                let n_floor = Integer::rounding_from(f, RoundingMode::Floor);
                assert!(n_floor.is_valid());
                let n_ceiling = &n_floor + Integer::ONE;
                let n_nearest = Integer::from(f);
                assert!(n_nearest == n_floor || n_nearest == n_ceiling);
            });

            test_properties($floats_var_5, |&f| {
                let floor = Integer::rounding_from(f, RoundingMode::Floor);
                let ceiling = &floor + Integer::ONE;
                let nearest = Integer::from(f);
                assert_eq!(nearest, if floor.even() { floor } else { ceiling });
            });
        }

        #[test]
        fn $checked_from_float_properties() {
            test_properties($finite_floats, |&f| {
                let on = Integer::checked_from(f);
                assert!(on.as_ref().map_or(true, |n| n.is_valid()));
                assert_eq!(Integer::checked_from(-f), on.map(|n| -n));
            });

            test_properties($floats_exactly_equal_to_integer, |&f| {
                let n = Integer::exact_from(f);
                assert!(n.is_valid());
                assert_eq!(n, Integer::rounding_from(f, RoundingMode::Exact));
                assert_eq!($f::rounding_from(n, RoundingMode::Exact), f);
            });

            test_properties($floats_var_4, |&f| {
                assert!(Integer::checked_from(f).is_none());
            });

            test_properties($floats_var_5, |&f| {
                assert!(Integer::checked_from(f).is_none());
            });
        }

        #[test]
        fn $convertible_from_float_properties() {
            test_properties($finite_floats, |&f| {
                Integer::convertible_from(f);
            });

            test_properties($floats_exactly_equal_to_integer, |&f| {
                assert!(Integer::convertible_from(f));
            });

            test_properties($floats_var_4, |&f| {
                assert!(!Integer::convertible_from(f));
            });

            test_properties($floats_var_5, |&f| {
                assert!(!Integer::convertible_from(f));
            });
        }
    };
}

float_properties!(
    f32,
    finite_f32s,
    pairs_of_finite_f32_and_rounding_mode_var_2,
    f32s_exactly_equal_to_integer,
    f32s_var_4,
    f32s_var_5,
    rounding_from_f32_properties,
    from_f32_properties,
    checked_from_f32_properties,
    convertible_from_f32_properties,
);
float_properties!(
    f64,
    finite_f64s,
    pairs_of_finite_f64_and_rounding_mode_var_2,
    f64s_exactly_equal_to_integer,
    f64s_var_4,
    f64s_var_5,
    rounding_from_f64_properties,
    from_f64_properties,
    checked_from_f64_properties,
    convertible_from_f64_properties,
);
