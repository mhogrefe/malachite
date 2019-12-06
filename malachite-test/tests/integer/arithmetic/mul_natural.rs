use std::str::FromStr;

use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use rug;

use common::test_properties;
use malachite_test::common::{
    integer_to_rug_integer, natural_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_natural};
use malachite_test::inputs::natural::naturals;

#[test]
fn test_mul_natural() {
    let test = |u, v, out| {
        let mut n = Integer::from_str(u).unwrap();
        n *= Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Integer::from_str(u).unwrap();
        n *= &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap() * Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() * Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap() * &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() * &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(v).unwrap() * Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(v).unwrap() * &Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(v).unwrap() * Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(v).unwrap() * &Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = rug::Integer::from_str(u).unwrap() * rug::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "0");
    test("123", "0", "0");
    test("-123", "0", "0");
    test("1", "123", "123");
    test("-1", "123", "-123");
    test("123", "1", "123");
    test("-123", "1", "-123");
    test("123", "456", "56088");
    test("-123", "456", "-56088");
    test("0", "1000000000000", "0");
    test("1000000000000", "0", "0");
    test("-1000000000000", "0", "0");
    test("1", "1000000000000", "1000000000000");
    test("-1", "1000000000000", "-1000000000000");
    test("1000000000000", "1", "1000000000000");
    test("-1000000000000", "1", "-1000000000000");
    test("1000000000000", "123", "123000000000000");
    test("-1000000000000", "123", "-123000000000000");
    test("123", "1000000000000", "123000000000000");
    test("-123", "1000000000000", "-123000000000000");
    test("123456789000", "987654321000", "121932631112635269000000");
    test("-123456789000", "987654321000", "-121932631112635269000000");
    test("4294967295", "2", "8589934590");
    test("-4294967295", "2", "-8589934590");
    test("4294967295", "4294967295", "18446744065119617025");
    test("-4294967295", "4294967295", "-18446744065119617025");
}

#[test]
fn mul_natural_properties() {
    test_properties(pairs_of_integer_and_natural, |&(ref x, ref y)| {
        let product_val_val = x.clone() * y.clone();
        let product_val_ref = x.clone() * y;
        let product_ref_val = x * y.clone();
        let product = x * y;
        assert!(product_val_val.is_valid());
        assert!(product_val_ref.is_valid());
        assert!(product_ref_val.is_valid());
        assert!(product.is_valid());
        assert_eq!(product_val_val, product);
        assert_eq!(product_val_ref, product);
        assert_eq!(product_ref_val, product);

        assert_eq!(y * x, product);
        assert_eq!(y * x.clone(), product);
        assert_eq!(y.clone() * x, product);
        assert_eq!(y.clone() * x.clone(), product);

        let mut mut_x = x.clone();
        mut_x *= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, product);
        let mut mut_x = x.clone();
        mut_x *= y;
        assert_eq!(mut_x, product);
        assert!(mut_x.is_valid());

        let mut mut_x = integer_to_rug_integer(x);
        mut_x *= natural_to_rug_integer(y);
        assert_eq!(rug_integer_to_integer(&mut_x), product);

        assert_eq!(
            rug_integer_to_integer(&(integer_to_rug_integer(x) * natural_to_rug_integer(y))),
            product
        );
        assert_eq!(y * x, product);
        //TODO assert_eq!((product / x).unwrap(), y);
        //TODO assert_eq!((product / y).unwrap(), x);

        assert_eq!(-x * y, -&product);
        assert_eq!(x * -y, -product);
    });

    #[allow(unknown_lints, erasing_op)]
    test_properties(integers, |x| {
        assert_eq!(x * Natural::ZERO, 0 as Limb);
        assert_eq!(Natural::ZERO * x, 0 as Limb);
        assert_eq!(x * Natural::ONE, *x);
        assert_eq!(Natural::ONE * x, *x);
    });

    #[allow(unknown_lints, erasing_op)]
    test_properties(naturals, |x| {
        assert_eq!(x * Integer::ZERO, 0 as Limb);
        assert_eq!(Integer::ZERO * x, 0 as Limb);
        assert_eq!(x * Integer::ONE, *x);
        assert_eq!(Integer::ONE * x, *x);
        assert_eq!(x * Integer::NEGATIVE_ONE, -x);
        assert_eq!(Integer::NEGATIVE_ONE * x, -x);
    });
}
