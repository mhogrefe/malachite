// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::BitBlockAccess;
use malachite_base::test_util::generators::{
    signed_unsigned_pair_gen_var_1, signed_unsigned_unsigned_triple_gen_var_2, unsigned_gen_var_5,
    unsigned_pair_gen_var_2, unsigned_pair_gen_var_7, unsigned_triple_gen_var_5,
};
use malachite_base::test_util::num::logic::bit_block_access::get_bits_naive;
use std::panic::catch_unwind;

#[test]
pub fn test_get_bits_unsigned() {
    fn test<T: BitBlockAccess<Bits = T> + PrimitiveUnsigned>(x: T, start: u64, end: u64, out: T) {
        assert_eq!(x.get_bits(start, end), out);
        assert_eq!(get_bits_naive::<T, T>(&x, start, end), out);
    }
    test(0xabcdu16, 4, 8, 0xc);
    test(0xabcdu16, 12, 100, 0xa);
    test(0xabcdu16, 5, 9, 14);
    test(0xabcdu16, 5, 5, 0);
    test(0xabcdu16, 100, 200, 0);

    test(0xabcdu64, 4, 8, 0xc);
    test(0xabcdu64, 12, 100, 0xa);
    test(0xabcdu64, 5, 9, 14);
    test(0xabcdu64, 5, 5, 0);
    test(0xabcdu64, 100, 200, 0);
}

#[test]
pub fn test_get_bits_signed() {
    fn test<T: BitBlockAccess<Bits = U> + PrimitiveSigned, U: PrimitiveUnsigned>(
        x: T,
        start: u64,
        end: u64,
        out: U,
    ) {
        assert_eq!(x.get_bits(start, end), out);
        assert_eq!(get_bits_naive::<T, U>(&x, start, end), out);
    }
    test(-0x5433i16, 4, 8, 0xc);
    test(-0x5433i16, 5, 9, 14);
    test(-0x5433i16, 5, 5, 0);
    test(-0x5433i16, 100, 104, 0xf);

    test(-0x5433i64, 4, 8, 0xc);
    test(-0x5433i64, 5, 9, 14);
    test(-0x5433i64, 5, 5, 0);
    test(-0x5433i64, 100, 104, 0xf);

    test(-1i8, 0, 8, 0xff);
}

fn get_bits_fail_helper<T: PrimitiveInt>() {
    assert_panic!(T::exact_from(100).get_bits(10, 5));
}

fn get_bits_fail_helper_signed<T: PrimitiveSigned>() {
    assert_panic!(T::exact_from(-100).get_bits(100, 300));
}

#[test]
fn get_bits_fail() {
    apply_fn_to_primitive_ints!(get_bits_fail_helper);
    apply_fn_to_signeds!(get_bits_fail_helper_signed);
}

fn properties_helper_unsigned<T: BitBlockAccess<Bits = T> + PrimitiveUnsigned>() {
    unsigned_triple_gen_var_5::<T, _>().test_properties(|(n, start, end)| {
        let bits = n.get_bits(start, end);
        assert_eq!(get_bits_naive::<T, T>(&n, start, end), bits);
        assert!(bits <= n);
        assert_eq!(n.get_bits(start + T::WIDTH, end + T::WIDTH), T::ZERO);
        let mut n_alt = n;
        n_alt.assign_bits(start, end, &bits);
        assert_eq!(n_alt, n);
    });

    unsigned_pair_gen_var_2::<T, u64>().test_properties(|(n, start)| {
        assert_eq!(n.get_bits(start, start), T::ZERO);
    });

    unsigned_pair_gen_var_7().test_properties(|(start, end)| {
        assert_eq!(T::ZERO.get_bits(start, end), T::ZERO);
    });
}

fn properties_helper_signed<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: BitBlockAccess<Bits = U> + PrimitiveSigned + WrappingFrom<U>,
>() {
    signed_unsigned_unsigned_triple_gen_var_2::<U, S, u64>().test_properties(|(n, start, end)| {
        let bits = n.get_bits(start, end);
        assert_eq!(get_bits_naive::<S, U>(&n, start, end), bits);
        let mut n_alt = n;
        n_alt.assign_bits(start, end, &bits);
        assert_eq!(n_alt, n);
    });

    signed_unsigned_pair_gen_var_1::<S, _>().test_properties(|(n, start)| {
        assert_eq!(n.get_bits(start, start), U::ZERO);
        assert_eq!(
            n.get_bits(start + S::WIDTH, start + (S::WIDTH << 1)),
            if n >= S::ZERO { U::ZERO } else { U::MAX }
        );
    });

    unsigned_pair_gen_var_7().test_properties(|(start, end)| {
        assert_eq!(S::ZERO.get_bits(start, end), U::ZERO);
    });

    unsigned_gen_var_5().test_properties(|start| {
        assert_eq!(S::NEGATIVE_ONE.get_bits(start, start + S::WIDTH), U::MAX);
    });
}

#[test]
fn get_bits_properties() {
    apply_fn_to_unsigneds!(properties_helper_unsigned);
    apply_fn_to_unsigned_signed_pairs!(properties_helper_signed);
}
