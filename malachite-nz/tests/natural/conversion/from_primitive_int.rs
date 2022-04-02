use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, SaturatingFrom,
};
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::common::{biguint_to_natural, rug_integer_to_natural};
use num::BigUint;
use rug;

#[test]
fn test_from_u32() {
    let test = |u: u32, out| {
        let x = Natural::from(u);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(BigUint::from(u).to_string(), out);
        assert_eq!(rug::Integer::from(u).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(u32::MAX, "4294967295");
}

#[test]
fn test_from_u64() {
    let test = |u: u64, out| {
        let x = Natural::from(u);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(BigUint::from(u).to_string(), out);
        assert_eq!(rug::Integer::from(u).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(u64::MAX, "18446744073709551615");
}

#[test]
fn test_checked_from_i32() {
    let test = |i: i32, out| {
        let on = Natural::checked_from(i);
        assert!(on.as_ref().map_or(true, Natural::is_valid));
        assert_eq!(on.to_debug_string(), out);
    };
    test(0, "Some(0)");
    test(123, "Some(123)");
    test(-123, "None");
    test(i32::MAX, "Some(2147483647)");
    test(i32::MIN, "None");
}

#[test]
fn test_exact_from_i32() {
    let test = |i: i32, out| {
        let x = Natural::exact_from(i);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(i32::MAX, "2147483647");
}

#[test]
#[should_panic]
fn exact_from_i32_fail_1() {
    Natural::exact_from(-123i32);
}

#[test]
#[should_panic]
fn exact_from_i32_fail_2() {
    Natural::exact_from(i32::MIN);
}

#[test]
fn test_saturating_from_i32() {
    let test = |i: i32, out| {
        let x = Natural::saturating_from(i);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(-123, "0");
    test(i32::MAX, "2147483647");
    test(i32::MIN, "0");
}

#[test]
fn test_convertible_from_i32() {
    let test = |i: i32, out| {
        assert_eq!(Natural::convertible_from(i), out);
    };
    test(0, true);
    test(123, true);
    test(-123, false);
    test(i32::MAX, true);
    test(i32::MIN, false);
}

#[test]
fn test_checked_from_i64() {
    let test = |i: i64, out| {
        let on = Natural::checked_from(i);
        assert!(on.as_ref().map_or(true, Natural::is_valid));
        assert_eq!(on.to_debug_string(), out);
    };
    test(0, "Some(0)");
    test(123, "Some(123)");
    test(-123, "None");
    test(i64::MAX, "Some(9223372036854775807)");
    test(i64::MIN, "None");
}

#[test]
fn test_saturating_from_i64() {
    let test = |i: i64, out| {
        let x = Natural::saturating_from(i);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(-123, "0");
    test(i64::MAX, "9223372036854775807");
    test(i64::MIN, "0");
}

#[test]
fn test_convertible_from_i64() {
    let test = |i: i64, out| {
        assert_eq!(Natural::convertible_from(i), out);
    };
    test(0, true);
    test(123, true);
    test(-123, false);
    test(i64::MAX, true);
    test(i64::MIN, false);
}

fn unsigned_properties<T: PrimitiveUnsigned>()
where
    Natural: From<T>,
    for<'a> T: CheckedFrom<&'a Natural>,
    u128: ExactFrom<T>,
{
    unsigned_gen::<T>().test_properties(|u| {
        let n = Natural::from(u);
        assert!(n.is_valid());
        assert_eq!(T::exact_from(&n), u);
        let n_alt: Natural = From::from(u128::exact_from(u));
        assert_eq!(n_alt, n);
    });
}

fn signed_properties<T: PrimitiveSigned>()
where
    Natural: CheckedFrom<T> + ConvertibleFrom<T> + SaturatingFrom<T>,
    for<'a> T: CheckedFrom<&'a Natural>,
    i128: ExactFrom<T>,
{
    signed_gen::<T>().test_properties(|i| {
        let on = Natural::checked_from(i);
        assert!(on.as_ref().map_or(true, Natural::is_valid));
        assert_eq!(on.is_some(), i >= T::ZERO);
        assert_eq!(Natural::convertible_from(i), i >= T::ZERO);
        let n = Natural::saturating_from(i);
        assert!(n.is_valid());
        if let Some(x) = on.as_ref() {
            assert_eq!(*x, n);
            assert_eq!(T::exact_from(x), i);
            let n_alt: Natural = ExactFrom::exact_from(i128::exact_from(i));
            assert_eq!(n_alt, n);
        } else {
            assert_eq!(n, 0);
        }
    });
}

#[test]
fn from_primitive_int_properties() {
    unsigned_gen::<u32>().test_properties(|u| {
        let n = Natural::from(u);
        assert_eq!(biguint_to_natural(&BigUint::from(u)), n);
        assert_eq!(rug_integer_to_natural(&rug::Integer::from(u)), n);
    });

    unsigned_gen::<u64>().test_properties(|u| {
        let n = Natural::from(u);
        assert_eq!(biguint_to_natural(&BigUint::from(u)), n);
        assert_eq!(rug_integer_to_natural(&rug::Integer::from(u)), n);
    });

    apply_fn_to_unsigneds!(unsigned_properties);
    apply_fn_to_signeds!(signed_properties);
}
