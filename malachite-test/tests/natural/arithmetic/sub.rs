use common::test_properties;
use malachite_base::num::Zero;
use malachite_nz::natural::Natural;
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::natural::{naturals, pairs_of_naturals_var_1};
use num::BigUint;
use rug;
use std::str::FromStr;

#[test]
fn test_sub_assign_natural() {
    let test = |u, v, out| {
        let mut n = Natural::from_str(u).unwrap();
        n -= &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0", "0");
    test("123", "0", "123");
    test("456", "123", "333");
    test("1000000000000", "123", "999999999877");
    test("12345678987654321", "314159265358979", "12031519722295342");
    test("4294967296", "1", "4294967295");
    test("4294967295", "4294967295", "0");
    test("4294967296", "4294967295", "1");
    test("4294967296", "4294967296", "0");
    test("18446744073709551616", "1", "18446744073709551615");
    test("18446744073709551615", "18446744073709551615", "0");
    test("18446744073709551616", "18446744073709551615", "1");
}

#[test]
#[should_panic(expected = "Cannot subtract a Natural from a smaller Natural")]
fn sub_assign_fail() {
    let mut x = Natural::from_str("123").unwrap();
    x -= &Natural::from_str("456").unwrap();
}

#[test]
fn test_sub_natural() {
    let test = |u, v, out| {
        let n = Natural::from_str(u).unwrap() - &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() - &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigUint::from_str(u).unwrap() - BigUint::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() - rug::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("123", "0", "123");
    test("456", "123", "333");
    test("1000000000000", "123", "999999999877");
    test("12345678987654321", "314159265358979", "12031519722295342",);
    test("4294967296", "1", "4294967295");
    test("4294967295", "4294967295", "0");
    test("4294967296", "4294967295", "1");
    test("4294967296", "4294967296", "0");
    test("18446744073709551616", "1", "18446744073709551615");
    test("18446744073709551615", "18446744073709551615", "0");
    test("18446744073709551616", "18446744073709551615", "1");
    test("70734740290631708", "282942734368", "70734457347897340",);
}

#[test]
#[should_panic(expected = "Cannot subtract a Natural from a smaller Natural")]
#[allow(unused_must_use)]
fn sub_fail_1() {
    Natural::from(123_u32) - &Natural::from(456_u32);
}

#[test]
#[should_panic(expected = "Cannot subtract a Natural from a smaller Natural. self: 123, other: 456")]
#[allow(unused_must_use)]
fn sub_fail_2() {
    &Natural::from(123_u32) - &Natural::from(456_u32);
}

#[test]
fn sub_properties() {
    test_properties(pairs_of_naturals_var_1, |&(ref x, ref y)| {
        let mut mut_x = x.clone();
        mut_x -= y;
        assert!(mut_x.is_valid());
        let difference = mut_x;
        
        let mut rug_x = natural_to_rug_integer(x);
        rug_x -= natural_to_rug_integer(y);
        assert_eq!(rug_integer_to_natural(&rug_x), difference);

        let difference_alt = x - y;
        assert_eq!(difference_alt, difference);
        assert!(difference_alt.is_valid());

        let difference_alt = x.clone() - y;
        assert_eq!(difference_alt, difference);
        assert!(difference_alt.is_valid());

        assert_eq!(
            biguint_to_natural(&(natural_to_biguint(x) - natural_to_biguint(y))),
            difference
        );
        assert_eq!(
            rug_integer_to_natural(&(natural_to_rug_integer(x) - natural_to_rug_integer(y))),
            difference
        );

        assert!(difference <= *x);
        assert_eq!(difference + y, *x);
    });

    #[allow(unknown_lints, identity_op, eq_op)]
    test_properties(naturals, |x| {
        assert_eq!(x - &Natural::ZERO, *x);
        assert_eq!(x - x, Natural::ZERO);
    });
}
