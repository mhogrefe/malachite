use std::ops::{Shl, Shr};

use malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShr, ShlRound, ShlRoundAssign, ShrRound,
};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signed_and_rounding_mode, pairs_of_unsigned_and_rounding_mode,
    triples_of_signed_small_signed_and_rounding_mode_var_2,
    triples_of_unsigned_small_signed_and_rounding_mode_var_2,
};

fn shl_round_unsigned_signed_helper<T: PrimitiveUnsigned + Rand, U: PrimitiveSigned + Rand>()
where
    T: Shr<U, Output = T>
        + Shl<U, Output = T>
        + ShrRound<U, Output = T>
        + ShlRound<U, Output = T>
        + ShlRoundAssign<U>
        + ArithmeticCheckedShr<U, Output = T>,
    U::UnsignedOfEqualWidth: Rand,
    U: WrappingFrom<<U as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(
        triples_of_unsigned_small_signed_and_rounding_mode_var_2::<T, U>,
        |&(n, i, rm)| {
            let mut mut_n = n;
            mut_n.shl_round_assign(i, rm);
            let shifted = mut_n;

            assert_eq!(n.shl_round(i, rm), shifted);
            if i < U::ZERO {
                assert!(shifted <= n);
            }
            if i != U::MIN {
                assert_eq!(n.shr_round(-i, rm), shifted);
            }
        },
    );

    test_properties(pairs_of_unsigned_and_rounding_mode::<T>, |&(n, rm)| {
        assert_eq!(n.shl_round(U::ZERO, rm), n);
    });

    test_properties(pairs_of_signed_and_rounding_mode::<U>, |&(u, rm)| {
        assert_eq!(T::ZERO.shl_round(u, rm), T::ZERO);
    });
}

fn shl_round_signed_signed_helper<T: PrimitiveSigned + Rand, U: PrimitiveSigned + Rand>()
where
    T: Shr<U, Output = T>
        + Shl<U, Output = T>
        + ShrRound<U, Output = T>
        + ShlRound<U, Output = T>
        + ShlRoundAssign<U>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
    U::UnsignedOfEqualWidth: Rand,
    U: WrappingFrom<<U as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(
        triples_of_signed_small_signed_and_rounding_mode_var_2::<T, U>,
        |&(n, i, rm)| {
            let mut mut_n = n;
            mut_n.shl_round_assign(i, rm);
            let shifted = mut_n;

            assert_eq!(n.shl_round(i, rm), shifted);
            if i < U::ZERO {
                assert!(shifted.le_abs(&n));
            }
            if i != U::MIN {
                assert_eq!(n.shr_round(-i, rm), shifted);
            }
        },
    );

    test_properties(pairs_of_signed_and_rounding_mode::<T>, |&(n, rm)| {
        assert_eq!(n.shl_round(U::ZERO, rm), n);
    });

    test_properties(pairs_of_signed_and_rounding_mode::<U>, |&(u, rm)| {
        assert_eq!(T::ZERO.shl_round(u, rm), T::ZERO);
    });
}

#[test]
fn shl_round_properties() {
    shl_round_unsigned_signed_helper::<u8, i8>();
    shl_round_unsigned_signed_helper::<u8, i16>();
    shl_round_unsigned_signed_helper::<u8, i32>();
    shl_round_unsigned_signed_helper::<u8, i64>();
    shl_round_unsigned_signed_helper::<u8, isize>();
    shl_round_unsigned_signed_helper::<u16, i8>();
    shl_round_unsigned_signed_helper::<u16, i16>();
    shl_round_unsigned_signed_helper::<u16, i32>();
    shl_round_unsigned_signed_helper::<u16, i64>();
    shl_round_unsigned_signed_helper::<u16, isize>();
    shl_round_unsigned_signed_helper::<u32, i8>();
    shl_round_unsigned_signed_helper::<u32, i16>();
    shl_round_unsigned_signed_helper::<u32, i32>();
    shl_round_unsigned_signed_helper::<u32, i64>();
    shl_round_unsigned_signed_helper::<u32, isize>();
    shl_round_unsigned_signed_helper::<u64, i8>();
    shl_round_unsigned_signed_helper::<u64, i16>();
    shl_round_unsigned_signed_helper::<u64, i32>();
    shl_round_unsigned_signed_helper::<u64, i64>();
    shl_round_unsigned_signed_helper::<u64, isize>();
    shl_round_unsigned_signed_helper::<usize, i8>();
    shl_round_unsigned_signed_helper::<usize, i16>();
    shl_round_unsigned_signed_helper::<usize, i32>();
    shl_round_unsigned_signed_helper::<usize, i64>();
    shl_round_unsigned_signed_helper::<usize, isize>();

    shl_round_signed_signed_helper::<i8, i8>();
    shl_round_signed_signed_helper::<i8, i16>();
    shl_round_signed_signed_helper::<i8, i32>();
    shl_round_signed_signed_helper::<i8, i64>();
    shl_round_signed_signed_helper::<i8, isize>();
    shl_round_signed_signed_helper::<i16, i8>();
    shl_round_signed_signed_helper::<i16, i16>();
    shl_round_signed_signed_helper::<i16, i32>();
    shl_round_signed_signed_helper::<i16, i64>();
    shl_round_signed_signed_helper::<i16, isize>();
    shl_round_signed_signed_helper::<i32, i8>();
    shl_round_signed_signed_helper::<i32, i16>();
    shl_round_signed_signed_helper::<i32, i32>();
    shl_round_signed_signed_helper::<i32, i64>();
    shl_round_signed_signed_helper::<i32, isize>();
    shl_round_signed_signed_helper::<i64, i8>();
    shl_round_signed_signed_helper::<i64, i16>();
    shl_round_signed_signed_helper::<i64, i32>();
    shl_round_signed_signed_helper::<i64, i64>();
    shl_round_signed_signed_helper::<i64, isize>();
    shl_round_signed_signed_helper::<isize, i8>();
    shl_round_signed_signed_helper::<isize, i16>();
    shl_round_signed_signed_helper::<isize, i32>();
    shl_round_signed_signed_helper::<isize, i64>();
    shl_round_signed_signed_helper::<isize, isize>();
}
