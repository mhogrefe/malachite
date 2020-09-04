use std::panic::catch_unwind;

use malachite_base_test_util::num::logic::bit_block_access::{assign_bits_naive, get_bits_naive};

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::logic::traits::BitBlockAccess;

#[test]
pub fn test_get_bits_unsigned() {
    fn test<T: PrimitiveUnsigned>(x: T, start: u64, end: u64, out: T)
    where
        T: BitBlockAccess<Bits = T>,
    {
        assert_eq!(x.get_bits(start, end), out);
        assert_eq!(get_bits_naive::<T, T>(&x, start, end), out)
    };
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
    fn test<T: PrimitiveSigned, U: PrimitiveUnsigned>(x: T, start: u64, end: u64, out: U)
    where
        T: BitBlockAccess<Bits = U>,
    {
        assert_eq!(x.get_bits(start, end), out);
        assert_eq!(get_bits_naive::<T, U>(&x, start, end), out)
    };
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

#[test]
pub fn test_assign_bits_unsigned() {
    fn test<T: PrimitiveUnsigned>(x_in: T, start: u64, end: u64, bits: T, x_out: T)
    where
        T: BitBlockAccess<Bits = T>,
    {
        let mut x = x_in;
        x.assign_bits(start, end, &bits);
        assert_eq!(x, x_out);

        let mut x = x_in;
        assign_bits_naive(&mut x, start, end, &bits);
        assert_eq!(x, x_out);
    };
    // assign partially
    test(0xab5du16, 4, 8, 0xc, 0xabcd);
    test(0x5bcdu16, 12, 100, 0xa, 0xabcd);
    test(0xabcdu16, 5, 9, 10, 43_853);
    test(0xabcdu16, 5, 5, 123, 0xabcd);
    // assign zeros above width
    test(0xabcdu16, 100, 200, 0, 0xabcd);
    test(0xabcdu16, 8, 24, 0, 0xcd);
    // assign everything
    test(0xabcdu16, 0, 100, 0x1234, 0x1234);

    test(0xab5du64, 4, 8, 0xc, 0xabcd);
    test(0x5bcdu64, 12, 100, 0xa, 0xabcd);
    test(0xabcdu64, 5, 9, 10, 43_853);
    test(0xabcdu64, 5, 5, 123, 0xabcd);
    test(0xabcdu64, 100, 200, 0, 0xabcd);
    test(0xabcdu64, 0, 100, 0x1234, 0x1234);
}

#[test]
pub fn test_assign_bits_signed() {
    fn test<T: PrimitiveSigned, U: PrimitiveUnsigned>(
        x_in: T,
        start: u64,
        end: u64,
        bits: U,
        x_out: T,
    ) where
        T: BitBlockAccess<Bits = U>,
    {
        let mut x = x_in;
        x.assign_bits(start, end, &bits);
        assert_eq!(x, x_out);

        let mut x = x_in;
        assign_bits_naive(&mut x, start, end, &bits);
        assert_eq!(x, x_out);
    };
    // *self >= 0
    test(0x2b5di16, 4, 8, 0xc, 0x2bcd);
    // *self < 0
    // assign within width
    test(-0x5413i16, 4, 8, 0xc, -0x5433);
    test(-0x54a3i16, 5, 9, 14, -21_539);
    test(-0x5433i16, 5, 5, 0, -0x5433);
    // assign ones above width
    test(-0x5433i16, 100, 104, 0xf, -0x5433);
    // assign everything
    test(-57i8, 0, 8, 0xff, -1);

    test(0x2b5di64, 4, 8, 0xc, 0x2bcd);
    test(-0x5413i64, 4, 8, 0xc, -0x5433);
    test(-0x54a3i64, 5, 9, 14, -21_539);
    test(-0x5433i64, 5, 5, 0, -0x5433);
    test(-0x5433i64, 100, 104, 0xf, -0x5433);
    test(-57i64, 0, 64, u64::MAX, -1);
}

fn assign_bits_fail_helper_unsigned<T: PrimitiveUnsigned>()
where
    T: BitBlockAccess<Bits = T>,
{
    assert_panic!(T::exact_from(100).assign_bits(10, 5, &T::exact_from(3)));
    assert_panic!(T::exact_from(100).assign_bits(3, T::WIDTH + 3, &T::MAX));
}

fn assign_bits_fail_helper_signed<U: PrimitiveUnsigned, S: PrimitiveSigned>()
where
    S: BitBlockAccess<Bits = U>,
{
    assert_panic!(S::exact_from(100).assign_bits(7, 5, &U::exact_from(3)));
    assert_panic!(S::exact_from(100).assign_bits(0, S::WIDTH, &U::MAX));
    assert_panic!(S::exact_from(-100).assign_bits(0, S::WIDTH + 1, &U::ZERO));
    assert_panic!(S::exact_from(-100).assign_bits(S::WIDTH + 1, S::WIDTH + 2, &U::ZERO));
    assert_panic!({
        let half_width = S::WIDTH >> 1;
        S::exact_from(-100).assign_bits(half_width, 3 * half_width - 4, &U::ZERO)
    });
}

#[test]
fn assign_bits_fail() {
    apply_fn_to_unsigneds!(assign_bits_fail_helper_unsigned);
    apply_fn_to_unsigned_signed_pairs!(assign_bits_fail_helper_signed);
}
