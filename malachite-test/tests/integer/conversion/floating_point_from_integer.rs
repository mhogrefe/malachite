use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, RoundingFrom,
};
use malachite_base::num::float::PrimitiveFloat;
use malachite_base::rounding_modes::RoundingMode;
use malachite_nz::integer::Integer;
use malachite_test::common::test_properties;
use malachite_test::inputs::integer::{
    integers, integers_exactly_equal_to_f32, integers_exactly_equal_to_f64,
    integers_not_exactly_equal_to_f32, integers_not_exactly_equal_to_f64, integers_var_1_f32,
    integers_var_1_f64, pairs_of_integer_and_rounding_mode_var_1_f32,
    pairs_of_integer_and_rounding_mode_var_1_f64,
};

macro_rules! float_properties {
    (
        $f: ident,
        $pairs_of_integer_and_rounding_mode_var_1: ident,
        $integers_exactly_equal_to_float: ident,
        $integers_not_exactly_equal_to_float: ident,
        $integers_var_1: ident,
        $float_rounding_from_integer_properties: ident,
        $float_from_integer_properties: ident,
        $float_checked_from_integer_properties: ident,
        $float_convertible_from_integer_properties: ident,
    ) => {
        #[test]
        fn $float_rounding_from_integer_properties() {
            test_properties($pairs_of_integer_and_rounding_mode_var_1, |&(ref n, rm)| {
                let f = $f::rounding_from(n, rm);
                assert_eq!($f::rounding_from(n.clone(), rm), f);
                assert_eq!($f::rounding_from(-n, -rm), -f);
            });

            test_properties($integers_exactly_equal_to_float, |n| {
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
                assert_eq!(Integer::rounding_from(f, RoundingMode::Exact), *n);
            });

            test_properties($integers_not_exactly_equal_to_float, |n| {
                let f_below = $f::rounding_from(n, RoundingMode::Floor);
                assert_eq!($f::rounding_from(n.clone(), RoundingMode::Floor), f_below);
                let f_above = f_below.next_higher();
                assert_eq!(f_above, $f::rounding_from(n, RoundingMode::Ceiling));
                assert_eq!(f_above, $f::rounding_from(n.clone(), RoundingMode::Ceiling));
                if *n >= 0 {
                    assert_eq!(f_below, $f::rounding_from(n, RoundingMode::Down));
                    assert_eq!(f_below, $f::rounding_from(n.clone(), RoundingMode::Down));
                    assert_eq!(f_above, $f::rounding_from(n, RoundingMode::Up));
                    assert_eq!(f_above, $f::rounding_from(n.clone(), RoundingMode::Up));
                } else {
                    assert_eq!(f_above, $f::rounding_from(n, RoundingMode::Down));
                    assert_eq!(f_above, $f::rounding_from(n.clone(), RoundingMode::Down));
                    assert_eq!(f_below, $f::rounding_from(n, RoundingMode::Up));
                    assert_eq!(f_below, $f::rounding_from(n.clone(), RoundingMode::Up));
                }
                let f_nearest = $f::rounding_from(n, RoundingMode::Nearest);
                assert_eq!(
                    $f::rounding_from(n.clone(), RoundingMode::Nearest),
                    f_nearest
                );
                assert!(f_nearest == f_below || f_nearest == f_above);
                assert_ne!(Integer::from(f_nearest), *n);
            });
            test_properties($integers_var_1, |n| {
                let floor = $f::rounding_from(n, RoundingMode::Floor);
                let ceiling = floor.next_higher();
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
        fn $float_from_integer_properties() {
            test_properties(integers, |n| {
                let f = $f::from(n);
                assert_eq!($f::from(n.clone()), f);
                assert_eq!($f::rounding_from(n, RoundingMode::Nearest), f);
                assert_eq!($f::from(-n), -f);
            });

            test_properties($integers_exactly_equal_to_float, |n| {
                let f = $f::from(n);
                assert_eq!($f::from(n.clone()), f);
                assert_eq!(Integer::from(f), *n);
            });

            test_properties($integers_not_exactly_equal_to_float, |n| {
                let f_below = $f::rounding_from(n, RoundingMode::Floor);
                assert_eq!($f::rounding_from(n.clone(), RoundingMode::Floor), f_below);
                let f_above = f_below.next_higher();
                let f_nearest = $f::from(n);
                assert_eq!($f::from(n.clone()), f_nearest);
                assert!(f_nearest == f_below || f_nearest == f_above);
                assert_ne!(Integer::from(f_nearest), *n);
            });

            test_properties($integers_var_1, |n| {
                let floor = $f::rounding_from(n, RoundingMode::Floor);
                let ceiling = floor.next_higher();
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
        fn $float_checked_from_integer_properties() {
            test_properties(integers, |n| {
                let of = $f::checked_from(n);
                assert_eq!($f::checked_from(n.clone()), of);
                assert_eq!($f::checked_from(-n), of.map(|f| -f));
            });

            test_properties($integers_exactly_equal_to_float, |n| {
                let f = $f::exact_from(n);
                assert_eq!($f::exact_from(n.clone()), f);
                assert_eq!(f, $f::rounding_from(n, RoundingMode::Exact));
                assert_eq!(Integer::rounding_from(f, RoundingMode::Exact), *n);
            });

            test_properties($integers_not_exactly_equal_to_float, |n| {
                assert!($f::checked_from(n).is_none());
            });

            test_properties($integers_var_1, |n| {
                assert!($f::checked_from(n).is_none());
            });
        }

        #[test]
        fn $float_convertible_from_integer_properties() {
            test_properties(integers, |n| {
                $f::convertible_from(n);
            });

            test_properties($integers_exactly_equal_to_float, |n| {
                assert!($f::convertible_from(n));
            });

            test_properties($integers_not_exactly_equal_to_float, |n| {
                assert!(!$f::convertible_from(n));
            });

            test_properties($integers_var_1, |n| {
                assert!(!$f::convertible_from(n));
            });
        }
    };
}

float_properties!(
    f32,
    pairs_of_integer_and_rounding_mode_var_1_f32,
    integers_exactly_equal_to_f32,
    integers_not_exactly_equal_to_f32,
    integers_var_1_f32,
    f32_rounding_from_integer_properties,
    f32_from_integer_properties,
    f32_checked_from_integer_properties,
    f32_convertible_from_integer_properties,
);
float_properties!(
    f64,
    pairs_of_integer_and_rounding_mode_var_1_f64,
    integers_exactly_equal_to_f64,
    integers_not_exactly_equal_to_f64,
    integers_var_1_f64,
    f64_rounding_from_integer_properties,
    f64_from_integer_properties,
    f64_checked_from_integer_properties,
    f64_convertible_from_integer_properties,
);
