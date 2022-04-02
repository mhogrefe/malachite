use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{CheckedFrom, ExactFrom};
use malachite_base::test_util::generators::{signed_gen, signed_gen_var_2, unsigned_gen};
use malachite_nz::natural::Natural;
use malachite_q::test_util::common::rug_rational_to_rational;
use malachite_q::Rational;
use rug;

#[test]
fn test_from_u32() {
    let test = |u: u32, out| {
        let x = Rational::from(u);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
        assert_eq!(rug::Rational::from(u).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(u32::MAX, "4294967295");
}

#[test]
fn test_from_u64() {
    let test = |u: u64, out| {
        let x = Rational::from(u);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
        assert_eq!(rug::Rational::from(u).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(u64::MAX, "18446744073709551615");
}

#[test]
fn test_from_i32() {
    let test = |i: i32, out| {
        let x = Rational::from(i);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
        assert_eq!(rug::Rational::from(i).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(-123, "-123");
    test(i32::MIN, "-2147483648");
    test(i32::MAX, "2147483647");
}

#[test]
fn test_from_i64() {
    let test = |i: i64, out| {
        let x = Rational::from(i);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
        assert_eq!(rug::Rational::from(i).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(-123, "-123");
    test(i64::MIN, "-9223372036854775808");
    test(i64::MAX, "9223372036854775807");
}

fn from_unsigned_properties_helper<T: for<'a> CheckedFrom<&'a Rational> + PrimitiveUnsigned>()
where
    Rational: From<T>,
    Natural: From<T>,
    u128: CheckedFrom<T>,
    rug::Integer: From<T>,
{
    unsigned_gen::<T>().test_properties(|u| {
        let n = Rational::from(u);
        assert!(n.is_valid());
        assert_eq!(T::exact_from(&n), u);
        let alt_n: Rational = From::from(Natural::from(u));
        assert_eq!(alt_n, n);
        let alt_n: Rational = From::from(u128::exact_from(u));
        assert_eq!(alt_n, n);
        assert_eq!(rug_rational_to_rational(&rug::Rational::from(u)), n);
    });
}

fn from_signed_properties_helper<T: for<'a> CheckedFrom<&'a Rational> + PrimitiveSigned>()
where
    Rational: From<T>,
    Natural: CheckedFrom<T>,
    i128: CheckedFrom<T>,
    rug::Integer: From<T>,
{
    signed_gen::<T>().test_properties(|i| {
        let n = Rational::from(i);
        assert!(n.is_valid());
        assert_eq!(T::exact_from(&n), i);
        let alt_n: Rational = From::from(i128::exact_from(i));
        assert_eq!(alt_n, n);
        assert_eq!(rug_rational_to_rational(&rug::Rational::from(i)), n);
    });

    signed_gen_var_2::<T>().test_properties(|i| {
        let n: Rational = From::from(Natural::exact_from(i));
        assert_eq!(n, Rational::from(i));
    });
}

#[test]
fn from_primitive_int_properties() {
    apply_fn_to_unsigneds!(from_unsigned_properties_helper);
    apply_fn_to_signeds!(from_signed_properties_helper);
}
