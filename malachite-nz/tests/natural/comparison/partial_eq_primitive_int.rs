use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{CheckedFrom, ExactFrom};
use malachite_base::test_util::generators::{signed_pair_gen_var_7, unsigned_pair_gen_var_27};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::common::{natural_to_biguint, natural_to_rug_integer};
use malachite_nz::test_util::generators::{natural_signed_pair_gen, natural_unsigned_pair_gen};
use malachite_nz::test_util::natural::comparison::partial_eq_primitive_int::*;
use num::BigUint;
use rug;
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_eq_u32() {
    let test = |u, v: u32, out| {
        assert_eq!(Natural::from_str(u).unwrap() == v, out);
        assert_eq!(
            num_partial_eq_unsigned(&BigUint::from_str(u).unwrap(), v),
            out
        );
        assert_eq!(rug::Integer::from_str(u).unwrap() == v, out);

        assert_eq!(v == Natural::from_str(u).unwrap(), out);
        assert_eq!(v == rug::Integer::from_str(u).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("123", 5, false);
    test("1000000000000", 123, false);
}

#[test]
fn test_partial_eq_u64() {
    let test = |u, v: u64, out| {
        assert_eq!(Natural::from_str(u).unwrap() == v, out);
        assert_eq!(
            num_partial_eq_unsigned(&BigUint::from_str(u).unwrap(), v),
            out
        );
        assert_eq!(rug::Integer::from_str(u).unwrap() == v, out);

        assert_eq!(v == Natural::from_str(u).unwrap(), out);
        assert_eq!(v == rug::Integer::from_str(u).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("123", 5, false);
    test("1000000000000", 1000000000000, true);
    test("1000000000000", 1000000000001, false);
    test("1000000000000000000000000", 1000000000000, false);
}

#[test]
fn test_partial_eq_i32() {
    let test = |u, v: i32, out| {
        assert_eq!(Natural::from_str(u).unwrap() == v, out);
        assert_eq!(rug::Integer::from_str(u).unwrap() == v, out);

        assert_eq!(v == Natural::from_str(u).unwrap(), out);
        assert_eq!(v == rug::Integer::from_str(u).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("123", 5, false);
    test("1000000000000", 123, false);
}

#[test]
fn test_partial_eq_i64() {
    let test = |u, v: i64, out| {
        assert_eq!(Natural::from_str(u).unwrap() == v, out);
        assert_eq!(rug::Integer::from_str(u).unwrap() == v, out);

        assert_eq!(v == Natural::from_str(u).unwrap(), out);
        assert_eq!(v == rug::Integer::from_str(u).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("123", 5, false);
    test("1000000000000", 1000000000000, true);
    test("1000000000000", 1000000000001, false);
    test("1000000000000000000000000", 1000000000000, false);
}

// Extra refs necessary for type inference
#[allow(clippy::cmp_owned, clippy::op_ref, clippy::trait_duplication_in_bounds)]
fn partial_eq_primitive_int_properties_helper_unsigned<
    T: PartialEq<Natural> + PartialEq<rug::Integer> + PrimitiveUnsigned,
>()
where
    BigUint: From<T>,
    Natural: From<T> + PartialEq<T> + PartialOrd<T>,
    rug::Integer: PartialEq<T>,
{
    natural_unsigned_pair_gen::<T>().test_properties(|(n, u)| {
        let eq = n == u;
        assert_eq!(num_partial_eq_unsigned(&natural_to_biguint(&n), u), eq);
        assert_eq!(natural_to_rug_integer(&n) == u, eq);
        assert_eq!(&n == &Natural::from(u), eq);

        assert_eq!(u == n, eq);
        assert_eq!(u == natural_to_rug_integer(&n), eq);
        assert_eq!(&Natural::from(u) == &n, eq);
        assert_eq!(n.partial_cmp(&u) == Some(Ordering::Equal), eq);
    });

    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        assert_eq!(Natural::from(x) == y, x == y);
        assert_eq!(x == Natural::from(y), x == y);
    });
}

// Extra refs necessary for type inference
#[allow(clippy::op_ref, clippy::trait_duplication_in_bounds)]
fn partial_eq_primitive_int_properties_helper_signed<
    T: PartialEq<Natural> + PartialEq<rug::Integer> + PrimitiveSigned,
>()
where
    Integer: From<T>,
    Natural: CheckedFrom<T> + PartialEq<T> + PartialOrd<T>,
    rug::Integer: PartialEq<T>,
{
    natural_signed_pair_gen::<T>().test_properties(|(n, i)| {
        let eq = n == i;
        assert_eq!(natural_to_rug_integer(&n) == i, eq);
        assert_eq!(&n == &Integer::from(i), eq);

        assert_eq!(i == n, eq);
        assert_eq!(i == natural_to_rug_integer(&n), eq);
        assert_eq!(&Integer::from(i) == &n, eq);
        assert_eq!(n.partial_cmp(&i) == Some(Ordering::Equal), eq);
    });

    signed_pair_gen_var_7::<T>().test_properties(|(x, y)| {
        assert_eq!(Natural::exact_from(x) == y, x == y);
        assert_eq!(x == Natural::exact_from(y), x == y);
    });
}

#[test]
fn partial_eq_primitive_int_properties() {
    apply_fn_to_unsigneds!(partial_eq_primitive_int_properties_helper_unsigned);
    apply_fn_to_signeds!(partial_eq_primitive_int_properties_helper_signed);
}
