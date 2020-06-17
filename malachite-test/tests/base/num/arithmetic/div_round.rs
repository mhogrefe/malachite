use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::rounding_mode::RoundingMode;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_nonzero_signed_and_rounding_mode, pairs_of_positive_unsigned_and_rounding_mode,
    pairs_of_signed_and_nonzero_signed, pairs_of_signed_and_rounding_mode,
    pairs_of_signed_and_rounding_mode_var_1, pairs_of_signed_and_rounding_mode_var_2,
    pairs_of_signeds_var_3, pairs_of_unsigned_and_positive_unsigned,
    pairs_of_unsigned_and_positive_unsigned_var_1, pairs_of_unsigned_and_rounding_mode,
    triples_of_signed_nonzero_signed_and_rounding_mode_var_1,
    triples_of_unsigned_positive_unsigned_and_rounding_mode_var_1,
};

fn unsigned_div_round_properties_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(
        triples_of_unsigned_positive_unsigned_and_rounding_mode_var_1::<T>,
        |&(x, y, rm)| {
            let mut mut_x = x.clone();
            mut_x.div_round_assign(y, rm);
            let q = mut_x;

            assert_eq!(x.div_round(y, rm), q);
            assert!(q <= x);
        },
    );

    test_properties(
        pairs_of_unsigned_and_positive_unsigned::<T, T>,
        |&(x, y)| {
            if let Some(left_multiplied) = x.checked_mul(y) {
                assert_eq!(left_multiplied.div_round(y, RoundingMode::Down), x);
                assert_eq!(left_multiplied.div_round(y, RoundingMode::Up), x);
                assert_eq!(left_multiplied.div_round(y, RoundingMode::Floor), x);
                assert_eq!(left_multiplied.div_round(y, RoundingMode::Ceiling), x);
                assert_eq!(left_multiplied.div_round(y, RoundingMode::Nearest), x);
                assert_eq!(left_multiplied.div_round(y, RoundingMode::Exact), x);
            }

            assert_eq!(
                x.ceiling_div_neg_mod(y).0,
                x.div_round(y, RoundingMode::Ceiling)
            );
        },
    );

    // TODO test using Rationals
    test_properties(
        pairs_of_unsigned_and_positive_unsigned_var_1::<T>,
        |&(x, y)| {
            let down = x.div_round(y, RoundingMode::Down);
            let up = down + T::ONE;
            assert_eq!(x.div_round(y, RoundingMode::Up), up);
            assert_eq!(x.div_round(y, RoundingMode::Floor), down);
            assert_eq!(x.div_round(y, RoundingMode::Ceiling), up);
            let nearest = x.div_round(y, RoundingMode::Nearest);
            assert!(nearest == down || nearest == up);
        },
    );

    test_properties(pairs_of_unsigned_and_rounding_mode::<T>, |&(x, rm)| {
        assert_eq!(x.div_round(T::ONE, rm), x);
    });

    test_properties(
        pairs_of_positive_unsigned_and_rounding_mode::<T>,
        |&(x, rm)| {
            assert_eq!(T::ZERO.div_round(x, rm), T::ZERO);
            assert_eq!(x.div_round(x, rm), T::ONE);
        },
    );
}

fn signed_div_round_properties_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(
        triples_of_signed_nonzero_signed_and_rounding_mode_var_1::<T>,
        |&(x, y, rm)| {
            let mut mut_x = x.clone();
            mut_x.div_round_assign(y, rm);
            let q = mut_x;

            assert_eq!(x.div_round(y, rm), q);

            assert!(q.le_abs(&x));
            if x != T::MIN {
                assert_eq!(-(-x).div_round(y, -rm), q);
            }
            if y != T::MIN && (x != T::MIN || (y != T::ONE && y != T::NEGATIVE_ONE)) {
                assert_eq!(-x.div_round(-y, -rm), q);
            }
        },
    );

    test_properties(pairs_of_signed_and_nonzero_signed::<T, T>, |&(x, y)| {
        if let Some(left_multiplied) = x.checked_mul(y) {
            assert_eq!(left_multiplied.div_round(y, RoundingMode::Down), x);
            assert_eq!(left_multiplied.div_round(y, RoundingMode::Up), x);
            assert_eq!(left_multiplied.div_round(y, RoundingMode::Floor), x);
            assert_eq!(left_multiplied.div_round(y, RoundingMode::Ceiling), x);
            assert_eq!(left_multiplied.div_round(y, RoundingMode::Nearest), x);
            assert_eq!(left_multiplied.div_round(y, RoundingMode::Exact), x);
        }
    });

    // TODO test using Rationals
    test_properties(pairs_of_signeds_var_3::<T>, |&(x, y)| {
        let down = x.div_round(y, RoundingMode::Down);
        let up = if (x >= T::ZERO) == (y >= T::ZERO) {
            down + T::ONE
        } else {
            down - T::ONE
        };
        let floor = x.div_round(y, RoundingMode::Floor);
        let ceiling = floor + T::ONE;
        assert_eq!(x.div_round(y, RoundingMode::Up), up);
        assert_eq!(x.div_round(y, RoundingMode::Ceiling), ceiling);
        let nearest = x.div_round(y, RoundingMode::Nearest);
        assert!(nearest == down || nearest == up);
    });

    test_properties(pairs_of_signed_and_rounding_mode::<T>, |&(x, rm)| {
        assert_eq!(x.div_round(T::ONE, rm), x);
    });

    test_properties(pairs_of_signed_and_rounding_mode_var_1::<T>, |&(x, rm)| {
        assert_eq!(x.div_round(T::NEGATIVE_ONE, rm), -x);
    });

    test_properties(
        pairs_of_nonzero_signed_and_rounding_mode::<T>,
        |&(x, rm)| {
            assert_eq!(T::ZERO.div_round(x, rm), T::ZERO);
            assert_eq!(x.div_round(x, rm), T::ONE);
        },
    );

    test_properties(pairs_of_signed_and_rounding_mode_var_2::<T>, |&(x, rm)| {
        assert_eq!(x.div_round(-x, rm), T::NEGATIVE_ONE);
        assert_eq!((-x).div_round(x, rm), T::NEGATIVE_ONE);
    });
}

#[test]
fn div_round_properties() {
    unsigned_div_round_properties_helper::<u8>();
    unsigned_div_round_properties_helper::<u16>();
    unsigned_div_round_properties_helper::<u32>();
    unsigned_div_round_properties_helper::<u64>();
    unsigned_div_round_properties_helper::<usize>();

    signed_div_round_properties_helper::<i8>();
    signed_div_round_properties_helper::<i16>();
    signed_div_round_properties_helper::<i32>();
    signed_div_round_properties_helper::<i64>();
    signed_div_round_properties_helper::<isize>();
}
