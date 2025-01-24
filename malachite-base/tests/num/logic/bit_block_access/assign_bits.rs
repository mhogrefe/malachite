// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::logic::traits::BitBlockAccess;
use malachite_base::test_util::generators::{
    signed_unsigned_pair_gen_var_1, signed_unsigned_unsigned_triple_gen_var_1,
    signed_unsigned_unsigned_unsigned_quadruple_gen_var_1, unsigned_pair_gen_var_2,
    unsigned_pair_gen_var_7, unsigned_quadruple_gen_var_1, unsigned_triple_gen_var_4,
};
use malachite_base::test_util::num::logic::bit_block_access::assign_bits_naive;
use std::cmp::min;
use std::panic::catch_unwind;

#[test]
pub fn test_assign_bits_unsigned() {
    fn test<T: BitBlockAccess<Bits = T> + PrimitiveUnsigned>(
        x_in: T,
        start: u64,
        end: u64,
        bits: T,
        x_out: T,
    ) {
        let mut x = x_in;
        x.assign_bits(start, end, &bits);
        assert_eq!(x, x_out);

        let mut x = x_in;
        assign_bits_naive(&mut x, start, end, &bits);
        assert_eq!(x, x_out);
    }
    // - assign partially
    test(0xab5du16, 4, 8, 0xc, 0xabcd);
    test(0x5bcdu16, 12, 100, 0xa, 0xabcd);
    test(0xabcdu16, 5, 9, 10, 43853);
    test(0xabcdu16, 5, 5, 123, 0xabcd);
    // - assign zeros above width
    test(0xabcdu16, 100, 200, 0, 0xabcd);
    test(0xabcdu16, 8, 24, 0, 0xcd);
    // - assign everything
    test(0xabcdu16, 0, 100, 0x1234, 0x1234);

    test(0xab5du64, 4, 8, 0xc, 0xabcd);
    test(0x5bcdu64, 12, 100, 0xa, 0xabcd);
    test(0xabcdu64, 5, 9, 10, 43853);
    test(0xabcdu64, 5, 5, 123, 0xabcd);
    test(0xabcdu64, 100, 200, 0, 0xabcd);
    test(0xabcdu64, 0, 100, 0x1234, 0x1234);
}

#[test]
pub fn test_assign_bits_signed() {
    fn test<T: BitBlockAccess<Bits = U> + PrimitiveSigned, U: PrimitiveUnsigned>(
        x_in: T,
        start: u64,
        end: u64,
        bits: U,
        x_out: T,
    ) {
        let mut x = x_in;
        x.assign_bits(start, end, &bits);
        assert_eq!(x, x_out);

        let mut x = x_in;
        assign_bits_naive(&mut x, start, end, &bits);
        assert_eq!(x, x_out);
    }
    // - *self >= 0
    test(0x2b5di16, 4, 8, 0xc, 0x2bcd);
    // - *self < 0
    // - assign within width
    test(-0x5413i16, 4, 8, 0xc, -0x5433);
    test(-0x54a3i16, 5, 9, 14, -21539);
    test(-0x5433i16, 5, 5, 0, -0x5433);
    // - assign ones above width
    test(-0x5433i16, 100, 104, 0xf, -0x5433);
    // - assign everything
    test(-57i8, 0, 8, 0xff, -1);

    test(0x2b5di64, 4, 8, 0xc, 0x2bcd);
    test(-0x5413i64, 4, 8, 0xc, -0x5433);
    test(-0x54a3i64, 5, 9, 14, -21539);
    test(-0x5433i64, 5, 5, 0, -0x5433);
    test(-0x5433i64, 100, 104, 0xf, -0x5433);
    test(-57i64, 0, 64, u64::MAX, -1);
}

fn assign_bits_fail_helper_unsigned<T: BitBlockAccess<Bits = T> + PrimitiveUnsigned>() {
    assert_panic!(T::exact_from(100).assign_bits(10, 5, &T::exact_from(3)));
    assert_panic!(T::exact_from(100).assign_bits(3, T::WIDTH + 3, &T::MAX));
}

fn assign_bits_fail_helper_signed<
    U: PrimitiveUnsigned,
    S: BitBlockAccess<Bits = U> + PrimitiveSigned,
>() {
    assert_panic!(S::exact_from(100).assign_bits(7, 5, &U::exact_from(3)));
    assert_panic!(S::exact_from(100).assign_bits(0, S::WIDTH, &U::MAX));
    assert_panic!(S::exact_from(-100).assign_bits(0, S::WIDTH + 1, &U::ZERO));
    assert_panic!(S::exact_from(-100).assign_bits(S::WIDTH + 1, S::WIDTH + 2, &U::ZERO));
    assert_panic!({
        let half_width = S::WIDTH >> 1;
        S::exact_from(-100).assign_bits(half_width, 3 * half_width - 4, &U::ZERO);
    });
}

#[test]
fn assign_bits_fail() {
    apply_fn_to_unsigneds!(assign_bits_fail_helper_unsigned);
    apply_fn_to_unsigned_signed_pairs!(assign_bits_fail_helper_signed);
}

fn properties_helper_unsigned<T: BitBlockAccess<Bits = T> + PrimitiveUnsigned>() {
    unsigned_quadruple_gen_var_1::<T, T>().test_properties(|(n, start, end, bits)| {
        let mut mut_n = n;
        mut_n.assign_bits(start, end, &bits);
        let mut mut_n_alt = mut_n;
        mut_n_alt.assign_bits(start, end, &bits);
        assert_eq!(mut_n_alt, mut_n);
        let mut mut_n_alt = n;
        assign_bits_naive::<T, T>(&mut mut_n_alt, start, end, &bits);
        assert_eq!(mut_n_alt, mut_n);
        assert_eq!(mut_n.get_bits(start, end), bits.mod_power_of_2(end - start));
    });

    unsigned_triple_gen_var_4::<T, u64>().test_properties(|(n, bits, start)| {
        let mut mut_n = n;
        mut_n.assign_bits(start, start, &bits);
        assert_eq!(mut_n, n);
    });

    unsigned_pair_gen_var_2::<T, u64>().test_properties(|(n, start)| {
        let mut mut_n = n;
        mut_n.assign_bits(start + T::WIDTH, start + (T::WIDTH << 1), &T::ZERO);
        assert_eq!(mut_n, n);
    });

    unsigned_pair_gen_var_7::<u64>().test_properties(|(start, end)| {
        let mut n = T::ZERO;
        n.assign_bits(start, end, &T::ZERO);
        assert_eq!(n, T::ZERO);
    });
}

fn properties_helper_signed<
    U: BitBlockAccess<Bits = U> + PrimitiveUnsigned,
    T: BitBlockAccess<Bits = U> + PrimitiveSigned + UnsignedAbs<Output = U>,
>() {
    signed_unsigned_unsigned_unsigned_quadruple_gen_var_1::<T, U>().test_properties(
        |(n, start, end, bits)| {
            let mut mut_n = n;
            mut_n.assign_bits(start, end, &bits);
            let mut mut_n_alt = mut_n;
            mut_n_alt.assign_bits(start, end, &bits);
            assert_eq!(mut_n_alt, mut_n);
            let mut mut_n_alt = n;
            assign_bits_naive::<T, U>(&mut mut_n_alt, start, end, &bits);
            assert_eq!(mut_n_alt, mut_n);
            assert_eq!(mut_n.get_bits(start, end), bits.mod_power_of_2(end - start));
            assert_eq!(mut_n >= T::ZERO, n >= T::ZERO);
        },
    );

    signed_unsigned_unsigned_triple_gen_var_1::<T, U, u64>().test_properties(|(n, bits, start)| {
        let mut mut_n = n;
        mut_n.assign_bits(start, start, &bits);
        assert_eq!(mut_n, n);
    });

    signed_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(n, start)| {
        let mut mut_n = n;
        mut_n.assign_bits(
            start + T::WIDTH - 1,
            start + (T::WIDTH << 1) - 1,
            &(if n >= T::ZERO { U::ZERO } else { U::MAX }),
        );
        assert_eq!(mut_n, n);
    });

    unsigned_pair_gen_var_7().test_properties(|(start, end)| {
        let mut n = T::ZERO;
        n.assign_bits(start, end, &U::ZERO);
        assert_eq!(n, T::ZERO);

        let mut n = T::NEGATIVE_ONE;
        n.assign_bits(start, min(end, start.saturating_add(T::WIDTH)), &U::MAX);
        assert_eq!(n, T::NEGATIVE_ONE);
    });
}

#[test]
fn assign_bits_properties() {
    apply_fn_to_unsigneds!(properties_helper_unsigned);
    apply_fn_to_unsigned_signed_pairs!(properties_helper_signed);
}
