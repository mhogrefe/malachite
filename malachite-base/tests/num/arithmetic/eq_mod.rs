// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::test_util::generators::{
    signed_pair_gen, signed_quadruple_gen, signed_triple_gen, signed_triple_gen_var_4,
    signed_triple_gen_var_5, unsigned_pair_gen_var_27, unsigned_quadruple_gen_var_10,
    unsigned_triple_gen_var_19, unsigned_triple_gen_var_7, unsigned_triple_gen_var_8,
};

#[test]
fn test_eq_mod() {
    fn test<T: PrimitiveInt>(x: T, y: T, m: T, out: bool) {
        assert_eq!(x.eq_mod(y, m), out);
    }
    test::<u8>(0, 0, 0, true);
    test::<u16>(0, 1, 0, false);
    test::<u32>(57, 57, 0, true);
    test::<u64>(57, 58, 0, false);
    test::<u128>(1000000000000, 57, 0, false);
    test::<usize>(0, 256, 256, true);
    test::<u16>(0, 256, 512, false);
    test::<u16>(13, 23, 10, true);
    test::<u32>(13, 24, 10, false);
    test::<u64>(13, 21, 1, true);
    test::<u128>(13, 21, 2, true);
    test::<usize>(13, 21, 4, true);
    test::<u8>(13, 21, 8, true);
    test::<u16>(13, 21, 16, false);
    test::<u32>(13, 21, 3, false);
    test::<u64>(1000000000001, 1, 4096, true);
    test::<u128>(1000000000001, 1, 8192, false);
    test::<u64>(12345678987654321, 321, 1000, true);
    test::<u64>(12345678987654321, 322, 1000, false);
    test::<u64>(1234, 1234, 1000000000000, true);
    test::<u64>(1234, 1235, 1000000000000, false);
    test::<u64>(1000000001234, 1000000002234, 1000, true);
    test::<u64>(1000000001234, 1000000002235, 1000, false);
    test::<u64>(1000000001234, 1234, 1000000000000, true);
    test::<u64>(1000000001234, 1235, 1000000000000, false);
    test::<u64>(1000000001234, 5000000001234, 1000000000000, true);
    test::<u64>(1000000001234, 5000000001235, 1000000000000, false);

    test::<i8>(0, -1, 0, false);
    test::<i16>(57, -57, 0, false);
    test::<i32>(57, -58, 0, false);
    test::<i64>(1000000000000, -57, 0, false);
    test::<i128>(0, -256, 256, true);
    test::<isize>(0, -256, 512, false);
    test::<i8>(13, -27, 10, true);
    test::<i16>(13, -28, 10, false);
    test::<i32>(29, -27, 1, true);
    test::<i64>(29, -27, 2, true);
    test::<i128>(29, -27, 4, true);
    test::<isize>(29, -27, 8, true);
    test::<i8>(29, -27, 16, false);
    test::<i16>(29, -27, 3, false);
    test::<i64>(999999999999, -1, 4096, true);
    test::<i64>(999999999999, -1, 8192, false);
    test::<i64>(12345678987654321, -679, 1000, true);
    test::<i64>(12345678987654321, -680, 1000, false);
    test::<i64>(1000000001234, -999999999766, 1000, true);
    test::<i64>(1000000001234, -999999999767, 1000, false);
    test::<i64>(1000000001234, -999999998766, 1000000000000, true);
    test::<i64>(1000000001234, -999999998767, 1000000000000, false);

    test::<i16>(-57, 57, 0, false);
    test::<i32>(-57, 58, 0, false);
    test::<i64>(-1000000000000, 57, 0, false);
    test::<i8>(-13, 27, 10, true);
    test::<i16>(-13, 28, 10, false);
    test::<i32>(-29, 27, 1, true);
    test::<i64>(-29, 27, 2, true);
    test::<i128>(-29, 27, 4, true);
    test::<isize>(-29, 27, 8, true);
    test::<i8>(-29, 27, 16, false);
    test::<i16>(-29, 27, 3, false);
    test::<i64>(-999999999999, 1, 4096, true);
    test::<i64>(-999999999999, 1, 8192, false);
    test::<i64>(-12345678987654321, 679, 1000, true);
    test::<i64>(-12345678987654321, 680, 1000, false);
    test::<i64>(-1000000001234, 999999999766, 1000, true);
    test::<i64>(-1000000001234, 999999999767, 1000, false);
    test::<i64>(-1000000001234, 999999998766, 1000000000000, true);
    test::<i64>(-1000000001234, 999999998767, 1000000000000, false);

    test::<i32>(-57, -57, 0, true);
    test::<i64>(-57, -58, 0, false);
    test::<i128>(-1000000000000, -57, 0, false);
    test::<i16>(-13, -23, 10, true);
    test::<i32>(-13, -24, 10, false);
    test::<i64>(-13, -21, 1, true);
    test::<i128>(-13, -21, 2, true);
    test::<isize>(-13, -21, 4, true);
    test::<i8>(-13, -21, 8, true);
    test::<i16>(-13, -21, 16, false);
    test::<i32>(-13, -21, 3, false);
    test::<i64>(-1000000000001, -1, 4096, true);
    test::<i128>(-1000000000001, -1, 8192, false);
    test::<i64>(-12345678987654321, -321, 1000, true);
    test::<i64>(-12345678987654321, -322, 1000, false);
    test::<i64>(-1234, -1234, 1000000000000, true);
    test::<i64>(-1234, -1235, 1000000000000, false);
    test::<i64>(-1000000001234, -1000000002234, 1000, true);
    test::<i64>(-1000000001234, -1000000002235, 1000, false);
    test::<i64>(-1000000001234, -1234, 1000000000000, true);
    test::<i64>(-1000000001234, -1235, 1000000000000, false);
    test::<i64>(-1000000001234, -5000000001234, 1000000000000, true);
    test::<i64>(-1000000001234, -5000000001235, 1000000000000, false);

    test::<isize>(0, 256, -256, true);
    test::<i16>(0, 256, -512, false);
    test::<i16>(13, 23, -10, true);
    test::<i32>(13, 24, -10, false);
    test::<i64>(13, 21, -1, true);
    test::<i128>(13, 21, -2, true);
    test::<isize>(13, 21, -4, true);
    test::<i8>(13, 21, -8, true);
    test::<i16>(13, 21, -16, false);
    test::<i32>(13, 21, -3, false);
    test::<i64>(1000000000001, 1, -4096, true);
    test::<i128>(1000000000001, 1, -8192, false);
    test::<i64>(12345678987654321, 321, -1000, true);
    test::<i64>(12345678987654321, 322, -1000, false);
    test::<i64>(1234, 1234, -1000000000000, true);
    test::<i64>(1234, 1235, -1000000000000, false);
    test::<i64>(1000000001234, 1000000002234, -1000, true);
    test::<i64>(1000000001234, 1000000002235, -1000, false);
    test::<i64>(1000000001234, 1234, -1000000000000, true);
    test::<i64>(1000000001234, 1235, -1000000000000, false);
    test::<i64>(1000000001234, 5000000001234, -1000000000000, true);
    test::<i64>(1000000001234, 5000000001235, -1000000000000, false);

    test::<i128>(0, -256, -256, true);
    test::<isize>(0, -256, -512, false);
    test::<i8>(13, -27, -10, true);
    test::<i16>(13, -28, -10, false);
    test::<i32>(29, -27, -1, true);
    test::<i64>(29, -27, -2, true);
    test::<i128>(29, -27, -4, true);
    test::<isize>(29, -27, -8, true);
    test::<i8>(29, -27, -16, false);
    test::<i16>(29, -27, -3, false);
    test::<i64>(999999999999, -1, -4096, true);
    test::<i64>(999999999999, -1, -8192, false);
    test::<i64>(12345678987654321, -679, -1000, true);
    test::<i64>(12345678987654321, -680, -1000, false);
    test::<i64>(1000000001234, -999999999766, -1000, true);
    test::<i64>(1000000001234, -999999999767, -1000, false);
    test::<i64>(1000000001234, -999999998766, -1000000000000, true);
    test::<i64>(1000000001234, -999999998767, -1000000000000, false);

    test::<i8>(-13, 27, -10, true);
    test::<i16>(-13, 28, -10, false);
    test::<i32>(-29, 27, -1, true);
    test::<i64>(-29, 27, -2, true);
    test::<i128>(-29, 27, -4, true);
    test::<isize>(-29, 27, -8, true);
    test::<i8>(-29, 27, -16, false);
    test::<i16>(-29, 27, -3, false);
    test::<i64>(-999999999999, 1, -4096, true);
    test::<i64>(-999999999999, 1, -8192, false);
    test::<i64>(-12345678987654321, 679, -1000, true);
    test::<i64>(-12345678987654321, 680, -1000, false);
    test::<i64>(-1000000001234, 999999999766, -1000, true);
    test::<i64>(-1000000001234, 999999999767, -1000, false);
    test::<i64>(-1000000001234, 999999998766, -1000000000000, true);
    test::<i64>(-1000000001234, 999999998767, -1000000000000, false);

    test::<i16>(-13, -23, -10, true);
    test::<i32>(-13, -24, -10, false);
    test::<i64>(-13, -21, -1, true);
    test::<i128>(-13, -21, -2, true);
    test::<isize>(-13, -21, -4, true);
    test::<i8>(-13, -21, -8, true);
    test::<i16>(-13, -21, -16, false);
    test::<i32>(-13, -21, -3, false);
    test::<i64>(-1000000000001, -1, -4096, true);
    test::<i128>(-1000000000001, -1, -8192, false);
    test::<i64>(-12345678987654321, -321, -1000, true);
    test::<i64>(-12345678987654321, -322, -1000, false);
    test::<i64>(-1234, -1234, -1000000000000, true);
    test::<i64>(-1234, -1235, -1000000000000, false);
    test::<i64>(-1000000001234, -1000000002234, -1000, true);
    test::<i64>(-1000000001234, -1000000002235, -1000, false);
    test::<i64>(-1000000001234, -1234, -1000000000000, true);
    test::<i64>(-1000000001234, -1235, -1000000000000, false);
    test::<i64>(-1000000001234, -5000000001234, -1000000000000, true);
    test::<i64>(-1000000001234, -5000000001235, -1000000000000, false);
}

fn eq_mod_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_triple_gen_var_19::<T>().test_properties(|(x, y, m)| {
        let equal = x.eq_mod(y, m);
        assert_eq!(y.eq_mod(x, m), equal);
    });

    unsigned_triple_gen_var_7::<T>().test_properties(|(x, y, m)| {
        assert!(x.eq_mod(y, m));
        assert!(y.eq_mod(x, m));
    });

    unsigned_triple_gen_var_8::<T>().test_properties(|(x, y, m)| {
        assert!(!x.eq_mod(y, m));
        assert!(!y.eq_mod(x, m));
    });

    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        assert!(x.eq_mod(y, T::ONE));
        assert_eq!(x.eq_mod(y, T::ZERO), x == y);
        assert_eq!(x.eq_mod(T::ZERO, y), x.divisible_by(y));
        assert!(x.eq_mod(x, y));
    });

    unsigned_quadruple_gen_var_10::<T>().test_properties(|(x, y, z, m)| {
        if x.eq_mod(y, m) && y.eq_mod(z, m) {
            assert!(x.eq_mod(z, m));
        }
    });
}

fn eq_mod_properties_helper_signed<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>,
>() {
    signed_triple_gen::<S>().test_properties(|(x, y, m)| {
        let equal = x.eq_mod(y, m);
        assert_eq!(y.eq_mod(x, m), equal);

        if x != S::MIN && y != S::MIN {
            assert_eq!((-x).eq_mod(-y, m), equal);
        }
        if m != S::MIN {
            assert_eq!(x.eq_mod(y, -m), equal);
        }
    });

    signed_triple_gen_var_4::<U, S>().test_properties(|(x, y, m)| {
        assert!(x.eq_mod(y, m));
        assert!(y.eq_mod(x, m));
    });

    signed_triple_gen_var_5::<S>().test_properties(|(x, y, m)| {
        assert!(!x.eq_mod(y, m));
        assert!(!y.eq_mod(x, m));
    });

    signed_pair_gen::<S>().test_properties(|(x, y)| {
        assert!(x.eq_mod(y, S::ONE));
        assert_eq!(x.eq_mod(y, S::ZERO), x == y);
        assert_eq!(x.eq_mod(S::ZERO, y), x.divisible_by(y));
        assert!(x.eq_mod(x, y));
    });

    signed_quadruple_gen::<S>().test_properties(|(x, y, z, m)| {
        if x.eq_mod(y, m) && y.eq_mod(z, m) {
            assert!(x.eq_mod(z, m));
        }
    });
}

#[test]
fn eq_mod_properties() {
    apply_fn_to_unsigneds!(eq_mod_properties_helper_unsigned);
    apply_fn_to_unsigned_signed_pairs!(eq_mod_properties_helper_signed);
}
