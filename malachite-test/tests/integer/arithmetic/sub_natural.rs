use std::str::FromStr;

use malachite_base::num::traits::Zero;
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
fn test_sub_natural() {
    let test = |i, j, out| {
        let mut n = Integer::from_str(i).unwrap();
        n -= Natural::from_str(j).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Integer::from_str(i).unwrap();
        n -= &Natural::from_str(j).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(i).unwrap() - Natural::from_str(j).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(i).unwrap() - Natural::from_str(j).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(i).unwrap() - &Natural::from_str(j).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(i).unwrap() - &Natural::from_str(j).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(j).unwrap() - Integer::from_str(i).unwrap();
        assert_eq!((-&n).to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(j).unwrap() - &Integer::from_str(i).unwrap();
        assert_eq!((-&n).to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(j).unwrap() - Integer::from_str(i).unwrap();
        assert_eq!((-&n).to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(j).unwrap() - &Integer::from_str(i).unwrap();
        assert_eq!((-&n).to_string(), out);
        assert!(n.is_valid());

        let n = rug::Integer::from_str(i).unwrap() - rug::Integer::from_str(j).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("123", "0", "123");
    test("0", "123", "-123");
    test("123", "123", "0");
    test("123", "456", "-333");
    test("1000000000000", "123", "999999999877");
    test("123", "1000000000000", "-999999999877");
    test("12345678987654321", "314159265358979", "12031519722295342");
}

#[test]
fn sub_natural_properties() {
    test_properties(pairs_of_integer_and_natural, |&(ref x, ref y)| {
        let difference_val_val = x.clone() - y.clone();
        let difference_val_ref = x.clone() - y;
        let difference_ref_val = x - y.clone();
        let difference = x - y;
        assert!(difference_val_val.is_valid());
        assert!(difference_val_ref.is_valid());
        assert!(difference_ref_val.is_valid());
        assert!(difference.is_valid());
        assert_eq!(difference_val_val, difference);
        assert_eq!(difference_val_ref, difference);
        assert_eq!(difference_ref_val, difference);

        assert_eq!(y - x, -&difference);
        assert_eq!(y - x.clone(), -&difference);
        assert_eq!(y.clone() - x, -&difference);
        assert_eq!(y.clone() - x.clone(), -&difference);

        let mut mut_x = x.clone();
        mut_x -= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, difference);
        let mut mut_x = x.clone();
        mut_x -= y;
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, difference);

        let mut mut_x = integer_to_rug_integer(x);
        mut_x -= natural_to_rug_integer(y);
        assert_eq!(rug_integer_to_integer(&mut_x), difference);

        assert_eq!(
            rug_integer_to_integer(&(integer_to_rug_integer(x) - natural_to_rug_integer(y))),
            difference
        );
        assert_eq!(&difference + y, *x);
        assert_eq!(x - difference, *y);
    });

    test_properties(naturals, |x| {
        assert_eq!(x - Integer::ZERO, *x);
        assert_eq!(Integer::ZERO - x, -x);
    });

    test_properties(integers, |x| {
        assert_eq!(x - Natural::ZERO, *x);
        assert_eq!(Natural::ZERO - x, -x);
    });

    test_properties(pairs_of_integer_and_unsigned::<Limb>, |&(ref x, y)| {
        assert_eq!(x - y, x - Natural::from(y));
        assert_eq!(y - x, Natural::from(y) - x);
    });
}
