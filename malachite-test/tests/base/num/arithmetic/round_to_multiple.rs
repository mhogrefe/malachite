use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{CheckedFrom, ConvertibleFrom, WrappingFrom};
use malachite_base::rounding_mode::RoundingMode;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signed_and_rounding_mode, pairs_of_signeds, pairs_of_signeds_var_3,
    pairs_of_unsigned_and_positive_unsigned_var_1, pairs_of_unsigned_and_rounding_mode,
    pairs_of_unsigneds, triples_of_signed_signed_and_rounding_mode_var_1,
    triples_of_unsigned_unsigned_and_rounding_mode_var_1,
};

fn round_to_multiple_properties_unsigned_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(
        triples_of_unsigned_unsigned_and_rounding_mode_var_1::<T>,
        |&(x, y, rm)| {
            let rounded = x.round_to_multiple(y, rm);

            let mut mut_x = x;
            mut_x.round_to_multiple_assign(y, rm);
            assert_eq!(mut_x, rounded);

            assert!(rounded.divisible_by(y));
            match rm {
                RoundingMode::Floor | RoundingMode::Down => assert!(rounded <= x),
                RoundingMode::Ceiling | RoundingMode::Up => assert!(rounded >= x),
                RoundingMode::Exact => assert_eq!(rounded, x),
                RoundingMode::Nearest => {
                    if y == T::ZERO {
                        assert_eq!(rounded, T::ZERO);
                    } else {
                        let mut closest = None;
                        let mut second_closest = None;
                        if rounded <= x {
                            if let Some(above) = rounded.checked_add(y) {
                                closest = Some(x - rounded);
                                second_closest = Some(above - x);
                            }
                        } else if let Some(below) = rounded.checked_sub(y) {
                            closest = Some(rounded - x);
                            second_closest = Some(x - below);
                        }
                        if let (Some(closest), Some(second_closest)) = (closest, second_closest) {
                            assert!(closest <= second_closest);
                            if closest == second_closest {
                                assert!(rounded.div_exact(y).even());
                            }
                        }
                    }
                }
            }
        },
    );

    test_properties(pairs_of_unsigneds::<T>, |&(x, y)| {
        if let Some(product) = x.checked_mul(y) {
            assert_eq!(product.round_to_multiple(y, RoundingMode::Down), product);
            assert_eq!(product.round_to_multiple(y, RoundingMode::Up), product);
            assert_eq!(product.round_to_multiple(y, RoundingMode::Floor), product);
            assert_eq!(product.round_to_multiple(y, RoundingMode::Ceiling), product);
            assert_eq!(product.round_to_multiple(y, RoundingMode::Nearest), product);
            assert_eq!(product.round_to_multiple(y, RoundingMode::Exact), product);
        }
    });

    test_properties(
        pairs_of_unsigned_and_positive_unsigned_var_1::<T>,
        |&(x, y)| {
            let down = x.round_to_multiple(y, RoundingMode::Down);
            if let Some(up) = down.checked_add(y) {
                assert_eq!(x.round_to_multiple(y, RoundingMode::Up), up);
                assert_eq!(x.round_to_multiple(y, RoundingMode::Floor), down);
                assert_eq!(x.round_to_multiple(y, RoundingMode::Ceiling), up);
                let nearest = x.round_to_multiple(y, RoundingMode::Nearest);
                assert!(nearest == down || nearest == up);
            }
        },
    );

    test_properties(pairs_of_unsigned_and_rounding_mode::<T>, |&(n, rm)| {
        if rm == RoundingMode::Floor || rm == RoundingMode::Down || rm == RoundingMode::Nearest {
            assert_eq!(n.round_to_multiple(T::ZERO, rm), T::ZERO);
        }
        assert_eq!(T::ZERO.round_to_multiple(n, rm), T::ZERO);
        assert_eq!(n.round_to_multiple(T::ONE, rm), n);
        assert_eq!(n.round_to_multiple(n, rm), n);
    });
}

fn round_to_multiple_properties_signed_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>
        + ConvertibleFrom<<T as UnsignedAbs>::Output>
        + CheckedFrom<<T as UnsignedAbs>::Output>,
    <T as UnsignedAbs>::Output: PrimitiveUnsigned,
{
    test_properties(
        triples_of_signed_signed_and_rounding_mode_var_1::<T>,
        |&(x, y, rm)| {
            let rounded = x.round_to_multiple(y, rm);

            let mut mut_x = x;
            mut_x.round_to_multiple_assign(y, rm);
            assert_eq!(mut_x, rounded);

            assert!(rounded.divisible_by(y));
            match rm {
                RoundingMode::Floor => assert!(rounded <= x),
                RoundingMode::Ceiling => assert!(rounded >= x),
                RoundingMode::Down => assert!(rounded.le_abs(&x)),
                RoundingMode::Up => assert!(rounded.ge_abs(&x)),
                RoundingMode::Exact => assert_eq!(rounded, x),
                RoundingMode::Nearest => {
                    if y == T::ZERO {
                        assert_eq!(rounded, T::ZERO);
                    } else {
                        let mut closest = None;
                        let mut second_closest = None;
                        let (o_above, o_below) = if y >= T::ZERO {
                            (rounded.checked_add(y), rounded.checked_sub(y))
                        } else {
                            (rounded.checked_sub(y), rounded.checked_add(y))
                        };
                        if rounded <= x {
                            if let Some(above) = o_above {
                                closest = Some(x - rounded);
                                second_closest = Some(above - x);
                            }
                        } else if let Some(below) = o_below {
                            closest = Some(rounded - x);
                            second_closest = Some(x - below);
                        }
                        if let (Some(closest), Some(second_closest)) = (closest, second_closest) {
                            assert!(closest <= second_closest, "{} {} {} {}", T::NAME, x, y, rm);
                            if closest == second_closest {
                                assert!(rounded.div_exact(y).even());
                            }
                        }
                    }
                }
            }
        },
    );

    test_properties(pairs_of_signeds::<T>, |&(x, y)| {
        if let Some(product) = x.checked_mul(y) {
            assert_eq!(product.round_to_multiple(y, RoundingMode::Down), product);
            assert_eq!(product.round_to_multiple(y, RoundingMode::Up), product);
            assert_eq!(product.round_to_multiple(y, RoundingMode::Floor), product);
            assert_eq!(product.round_to_multiple(y, RoundingMode::Ceiling), product);
            assert_eq!(product.round_to_multiple(y, RoundingMode::Nearest), product);
            assert_eq!(product.round_to_multiple(y, RoundingMode::Exact), product);
        }
    });

    test_properties(pairs_of_signeds_var_3::<T>, |&(x, y)| {
        let down = x.round_to_multiple(y, RoundingMode::Down);
        if let Some(up) = if (x >= T::ZERO) == (y >= T::ZERO) {
            down.checked_add(y)
        } else {
            down.checked_sub(y)
        } {
            assert_eq!(x.round_to_multiple(y, RoundingMode::Up), up);
            if x >= T::ZERO {
                assert_eq!(x.round_to_multiple(y, RoundingMode::Floor), down);
                assert_eq!(x.round_to_multiple(y, RoundingMode::Ceiling), up);
            } else {
                assert_eq!(x.round_to_multiple(y, RoundingMode::Floor), up);
                assert_eq!(x.round_to_multiple(y, RoundingMode::Ceiling), down);
            }
            let nearest = x.round_to_multiple(y, RoundingMode::Nearest);
            assert!(nearest == down || nearest == up);
        }
    });

    test_properties(pairs_of_signed_and_rounding_mode::<T>, |&(n, rm)| {
        if rm == RoundingMode::Down
            || rm == RoundingMode::Nearest
            || rm
                == if n >= T::ZERO {
                    RoundingMode::Floor
                } else {
                    RoundingMode::Ceiling
                }
        {
            assert_eq!(n.round_to_multiple(T::ZERO, rm), T::ZERO);
        }
        assert_eq!(T::ZERO.round_to_multiple(n, rm), T::ZERO);
        assert_eq!(n.round_to_multiple(T::ONE, rm), n);
        assert_eq!(n.round_to_multiple(n, rm), n);
    });
}

#[test]
fn round_to_multiple_properties() {
    round_to_multiple_properties_unsigned_helper::<u8>();
    round_to_multiple_properties_unsigned_helper::<u16>();
    round_to_multiple_properties_unsigned_helper::<u32>();
    round_to_multiple_properties_unsigned_helper::<u64>();
    round_to_multiple_properties_unsigned_helper::<usize>();
    round_to_multiple_properties_signed_helper::<i8>();
    round_to_multiple_properties_signed_helper::<i16>();
    round_to_multiple_properties_signed_helper::<i32>();
    round_to_multiple_properties_signed_helper::<i64>();
    round_to_multiple_properties_signed_helper::<isize>();
}
