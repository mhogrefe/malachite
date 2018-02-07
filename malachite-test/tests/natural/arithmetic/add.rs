use common::test_properties;
use malachite_base::num::Zero;
use malachite_nz::natural::Natural;
use malachite_test::common::{biguint_to_natural, natural_to_biguint, natural_to_rug_integer,
                             rug_integer_to_natural};
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_unsigned, pairs_of_naturals,
                                      triples_of_naturals};
use num::BigUint;
use rug;
use std::str::FromStr;

#[test]
fn test_add() {
    let test = |u, v, out| {
        let mut n = Natural::from_str(u).unwrap();
        n += Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n += &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap() + Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() + Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap() + &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() + &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigUint::from_str(u).unwrap() + BigUint::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);

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
    test(
        "1000000000000",
        "1000000000000000000000000",
        "1000000000001000000000000",
    );
}

#[test]
fn add_properties() {
    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
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

        let mut mut_x = x.clone();
        mut_x += y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, sum);
        let mut mut_x = x.clone();
        mut_x += y;
        assert_eq!(mut_x, sum);
        assert!(mut_x.is_valid());

        let mut mut_x = natural_to_rug_integer(x);
        mut_x += natural_to_rug_integer(y);
        assert_eq!(rug_integer_to_natural(&mut_x), sum);

        assert_eq!(
            biguint_to_natural(&(natural_to_biguint(x) + natural_to_biguint(y))),
            sum
        );
        assert_eq!(
            rug_integer_to_natural(&(natural_to_rug_integer(x) + natural_to_rug_integer(y))),
            sum
        );
        assert_eq!(y + x, sum);
        assert_eq!((&sum - x).unwrap(), *y);
        assert_eq!((&sum - y).unwrap(), *x);

        assert!(sum >= *x);
        assert!(sum >= *y);
    });

    test_properties(
        pairs_of_natural_and_unsigned,
        |&(ref x, y): &(Natural, u32)| {
            let sum = x + Natural::from(y);
            assert_eq!(x + y, sum);
            assert_eq!(y + x, sum);
        },
    );

    test_properties(naturals, |x| {
        assert_eq!(x + Natural::ZERO, *x);
        assert_eq!(Natural::ZERO + x, *x);
        assert_eq!(x + x, x << 1);
    });

    test_properties(triples_of_naturals, |&(ref x, ref y, ref z)| {
        assert_eq!((x + y) + z, x + (y + z));
    });
}
