// Copyright © 2026 William Youmans
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL as published by the Free Software Foundation; either version
// 3 of the License, or (at your option any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::factorization::traits::IsSquare;
use malachite_base::test_util::generators::{
    signed_gen, signed_gen_var_4, signed_gen_var_10, unsigned_gen, unsigned_gen_var_21,
    unsigned_pair_gen_var_27,
};

fn is_square_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(x.is_square(), x.checked_sqrt().is_some());
    });

    unsigned_gen_var_21::<T>().test_properties(|x| {
        assert!(x.square().is_square());
    });

    // 1 < x < 2^32 avoids overflow and consecutive squares (0, 1).
    unsigned_pair_gen_var_27::<T>().test_properties(|(mut x, mut y)| {
        // test non squares in interval (x^2, (x+1)^2)
        x.mod_power_of_2_assign(T::WIDTH >> 1);
        y.mod_power_of_2_assign(T::WIDTH >> 1);
        if x != T::ZERO {
            let sqr = x.square();
            let non_sqr = sqr + (y % (x << 1u32)) + T::ONE;
            assert!(!non_sqr.is_square());
        }
    });
}

fn is_square_properties_helper_signed<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U> + UnsignedAbs<Output = U>,
>() {
    signed_gen::<S>().test_properties(|x| {
        assert_eq!(x.is_square(), x >= S::ZERO && x.unsigned_abs().is_square());
    });

    signed_gen_var_10::<U, S>().test_properties(|x| {
        assert!(x.square().is_square());
    });

    // test negative signed integers are non square and positive signed integer squares are squares.
    signed_gen_var_4::<S>().test_properties(|x| {
        assert!(!x.is_square());
    });
}

#[test]
fn is_square_properties() {
    apply_fn_to_unsigneds!(is_square_properties_helper_unsigned);
    apply_fn_to_unsigned_signed_pairs!(is_square_properties_helper_signed);
}

#[test]
fn test_is_square() {
    assert!(0u8.is_square());
    assert!(0u16.is_square());
    assert!(0u32.is_square());
    assert!(0u64.is_square());

    assert!(1u64.is_square());
    assert!(4u64.is_square());
    assert!(9u64.is_square());
    assert!(16u64.is_square());
    assert!(25u64.is_square());

    assert!(0i8.is_square());
    assert!(0i16.is_square());
    assert!(0i32.is_square());
    assert!(0i64.is_square());

    assert!(1i64.is_square());
    assert!(4i64.is_square());
    assert!(9i64.is_square());
    assert!(16i64.is_square());
    assert!(25i64.is_square());

    assert!(!(-1i64).is_square());
    assert!(!(-4i64).is_square());
    assert!(!(-9i64).is_square());
    assert!(!(-16i64).is_square());
    assert!(!(-25i64).is_square());
}
