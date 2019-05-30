use std::str::FromStr;

use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use rug;

use common::test_properties;
use malachite_test::common::{
    integer_to_rug_integer, natural_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_natural, pairs_of_integer_and_unsigned,
};
use malachite_test::inputs::natural::naturals;

#[test]
fn test_add_natural() {
    let test = |u, v, out| {
        let mut n = Integer::from_str(u).unwrap();
        n += Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Integer::from_str(u).unwrap();
        n += &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap() + Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() + Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap() + &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() + &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(v).unwrap() + Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(v).unwrap() + Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(v).unwrap() + &Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(v).unwrap() + &Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = rug::Integer::from_str(u).unwrap() + rug::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "123");
    test("123", "0", "123");
    test("123", "456", "579");
    test("1000000000000", "123", "1000000000123");
    test("123", "1000000000000", "1000000000123");
    test("12345678987654321", "314159265358979", "12659838253013300");
    test("-123", "0", "-123");
    test("-123", "123", "0");
    test("-456", "123", "-333");
    test("-123", "1000000000000", "999999999877");
    test("-1000000000000", "123", "-999999999877");
    test("-314159265358979", "12345678987654321", "12031519722295342");
}

#[test]
fn add_natural_properties() {
    test_properties(pairs_of_integer_and_natural, |&(ref x, ref y)| {
        let sum_val_val = x.clone() + y.clone();
        let sum_val_ref = x.clone() + y;
        let sum_ref_val = x + y.clone();
        let sum = x + y;
        assert!(sum_val_val.is_valid());
        assert!(sum_val_ref.is_valid());
        assert!(sum_ref_val.is_valid());
        assert!(sum.is_valid());
        assert_eq!(sum_val_val, sum);
        assert_eq!(sum_val_ref, sum);
        assert_eq!(sum_ref_val, sum);

        assert_eq!(y + x, sum);
        assert_eq!(y + x.clone(), sum);
        assert_eq!(y.clone() + x, sum);
        assert_eq!(y.clone() + x.clone(), sum);

        let mut mut_x = x.clone();
        mut_x += y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, sum);
        let mut mut_x = x.clone();
        mut_x += y;
        assert_eq!(mut_x, sum);
        assert!(mut_x.is_valid());

        let mut mut_x = integer_to_rug_integer(x);
        mut_x += natural_to_rug_integer(y);
        assert_eq!(rug_integer_to_integer(&mut_x), sum);

        assert_eq!(
            rug_integer_to_integer(&(integer_to_rug_integer(x) + natural_to_rug_integer(y))),
            sum
        );
        assert_eq!(&sum - x, *y);
        assert_eq!(sum - y, *x);
    });

    test_properties(naturals, |x| {
        assert_eq!(x + Integer::ZERO, *x);
        assert_eq!(Integer::ZERO + x, *x);
    });

    test_properties(integers, |x| {
        assert_eq!(x + Natural::ZERO, *x);
        assert_eq!(Natural::ZERO + x, *x);
    });

    test_properties(pairs_of_integer_and_unsigned::<Limb>, |&(ref x, y)| {
        let sum = x + Natural::from(y);
        assert_eq!(x + y, sum);
        assert_eq!(y + x, sum);
    });
}
