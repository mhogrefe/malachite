use std::str::FromStr;

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_nz::integer::Integer;
use num::BigInt;
use rand::Rand;
use rug;

use malachite_test::common::{integer_to_bigint, integer_to_rug_integer, test_properties};
use malachite_test::inputs::base::{pairs_of_signeds, pairs_of_unsigneds};
use malachite_test::inputs::integer::{pairs_of_integer_and_signed, pairs_of_integer_and_unsigned};
use malachite_test::integer::comparison::partial_eq_primitive_integer::num_partial_eq_primitive;

#[test]
fn test_partial_eq_u32() {
    let test = |u, v: u32, out| {
        assert_eq!(Integer::from_str(u).unwrap() == v, out);
        assert_eq!(
            num_partial_eq_primitive(&BigInt::from_str(u).unwrap(), v),
            out
        );
        assert_eq!(rug::Integer::from_str(u).unwrap() == v, out);

        assert_eq!(v == Integer::from_str(u).unwrap(), out);
        assert_eq!(v == rug::Integer::from_str(u).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("-123", 123, false);
    test("123", 5, false);
    test("-123", 5, false);
    test("1000000000000", 123, false);
    test("-1000000000000", 123, false);
}

#[test]
fn test_partial_eq_u64() {
    let test = |u, v: u64, out| {
        assert_eq!(Integer::from_str(u).unwrap() == v, out);
        assert_eq!(
            num_partial_eq_primitive(&BigInt::from_str(u).unwrap(), v),
            out
        );
        assert_eq!(rug::Integer::from_str(u).unwrap() == v, out);

        assert_eq!(v == Integer::from_str(u).unwrap(), out);
        assert_eq!(v == rug::Integer::from_str(u).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("-123", 123, false);
    test("123", 5, false);
    test("-123", 5, false);
    test("1000000000000", 1000000000000, true);
    test("-1000000000000", 1000000000000, false);
    test("1000000000000", 1000000000001, false);
    test("-1000000000000", 1000000000001, false);
    test("1000000000000000000000000", 1000000000000, false);
    test("-1000000000000000000000000", 1000000000000, false);
}

#[test]
fn test_partial_eq_i32() {
    let test = |u, v: i32, out| {
        assert_eq!(Integer::from_str(u).unwrap() == v, out);
        assert_eq!(rug::Integer::from_str(u).unwrap() == v, out);

        assert_eq!(v == Integer::from_str(u).unwrap(), out);
        assert_eq!(v == rug::Integer::from_str(u).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("-123", -123, true);
    test("-123", 123, false);
    test("123", 5, false);
    test("-123", -5, false);
    test("1000000000000", 123, false);
    test("-1000000000000", -123, false);
}

#[test]
fn test_partial_eq_i64() {
    let test = |u, v: i64, out| {
        assert_eq!(Integer::from_str(u).unwrap() == v, out);
        assert_eq!(rug::Integer::from_str(u).unwrap() == v, out);

        assert_eq!(v == Integer::from_str(u).unwrap(), out);
        assert_eq!(v == rug::Integer::from_str(u).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("-123", -123, true);
    test("-123", 123, false);
    test("123", 5, false);
    test("-123", -5, false);
    test("1000000000000", 1000000000000, true);
    test("-1000000000000", -1000000000000, true);
    test("1000000000000", 1000000000001, false);
    test("-1000000000000", -1000000000001, false);
    test("1000000000000000000000000", 1000000000000, false);
    test("-1000000000000000000000000", -1000000000000, false);
}

fn partial_eq_primitive_integer_properties_helper_unsigned<
    T: PartialEq<Integer> + PartialEq<rug::Integer> + PrimitiveUnsigned + Rand,
>()
where
    Integer: From<T> + PartialEq<T>,
    BigInt: From<T>,
    rug::Integer: PartialEq<T>,
{
    test_properties(pairs_of_integer_and_unsigned::<T>, |&(ref n, u)| {
        let eq = *n == u;
        assert_eq!(num_partial_eq_primitive(&integer_to_bigint(n), u), eq);
        assert_eq!(integer_to_rug_integer(n) == u, eq);
        assert_eq!(n == &Integer::from(u), eq);

        assert_eq!(u == *n, eq);
        assert_eq!(u == integer_to_rug_integer(n), eq);
        assert_eq!(&Integer::from(u) == n, eq);
    });

    test_properties(pairs_of_unsigneds::<T>, |&(x, y)| {
        assert_eq!(Integer::from(x) == y, x == y);
        assert_eq!(x == Integer::from(y), x == y);
    });
}

fn partial_eq_primitive_integer_properties_helper_signed<
    T: PartialEq<Integer> + PartialEq<rug::Integer> + PrimitiveSigned + Rand,
>()
where
    Integer: From<T> + PartialEq<T>,
    rug::Integer: PartialEq<T>,
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(pairs_of_integer_and_signed::<T>, |&(ref n, i)| {
        let eq = *n == i;
        assert_eq!(integer_to_rug_integer(n) == i, eq);
        assert_eq!(n == &Integer::from(i), eq);

        assert_eq!(i == *n, eq);
        assert_eq!(i == integer_to_rug_integer(n), eq);
        assert_eq!(&Integer::from(i) == n, eq);
    });

    test_properties(pairs_of_signeds::<T>, |&(x, y)| {
        assert_eq!(Integer::from(x) == y, x == y);
        assert_eq!(x == Integer::from(y), x == y);
    });
}

#[test]
fn partial_eq_primitive_integer_properties() {
    partial_eq_primitive_integer_properties_helper_unsigned::<u8>();
    partial_eq_primitive_integer_properties_helper_unsigned::<u16>();
    partial_eq_primitive_integer_properties_helper_unsigned::<u32>();
    partial_eq_primitive_integer_properties_helper_unsigned::<u64>();
    partial_eq_primitive_integer_properties_helper_unsigned::<usize>();
    partial_eq_primitive_integer_properties_helper_signed::<i8>();
    partial_eq_primitive_integer_properties_helper_signed::<i16>();
    partial_eq_primitive_integer_properties_helper_signed::<i32>();
    partial_eq_primitive_integer_properties_helper_signed::<i64>();
    partial_eq_primitive_integer_properties_helper_signed::<isize>();
}
