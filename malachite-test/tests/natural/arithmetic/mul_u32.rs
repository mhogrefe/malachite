use common::test_properties;
use malachite_base::num::{One, Zero};
use malachite_nz::natural::Natural;
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::base::unsigneds;
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_unsigned};
use malachite_test::natural::arithmetic::mul_u32::num_mul_u32;
use num::BigUint;
use rug::{self, Assign};
use std::str::FromStr;

#[test]
fn test_add_u32() {
    let test = |u, v: u32, out| {
        let mut n = Natural::from_str(u).unwrap();
        n *= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from_str(u).unwrap();
        n *= v;
        assert_eq!(n.to_string(), out);

        let n = Natural::from_str(u).unwrap() * v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = num_mul_u32(BigUint::from_str(u).unwrap(), v);
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() * v;
        assert_eq!(n.to_string(), out);

        let n = &Natural::from_str(u).unwrap() * v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v * Natural::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v * rug::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);

        let n = v * &Natural::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from(0);
        n.assign(v * &rug::Integer::from_str(u).unwrap());
        assert_eq!(n.to_string(), out);
    };
    test("0", 0, "0");
    test("0", 123, "0");
    test("123", 0, "0");
    test("1", 123, "123");
    test("123", 1, "123");
    test("123", 456, "56088");
    test("1000000000000", 0, "0");
    test("1000000000000", 1, "1000000000000");
    test("1000000000000", 123, "123000000000000");
    test("4294967295", 2, "8589934590");
    test("18446744073709551615", 2, "36893488147419103230");
}

#[test]
fn mul_u32_properties() {
    test_properties(
        pairs_of_natural_and_unsigned,
        |&(ref n, u): &(Natural, u32)| {
            let mut mut_n = n.clone();
            mut_n *= u;
            assert!(mut_n.is_valid());
            let product = mut_n;

            let mut rug_n = natural_to_rug_integer(n);
            rug_n *= u;
            assert_eq!(rug_integer_to_natural(&rug_n), product);

            let product_alt = n * u;
            assert!(product_alt.is_valid());
            assert_eq!(product_alt, product);

            let product_alt = n.clone() * u;
            assert!(product_alt.is_valid());
            assert_eq!(product_alt, product);

            let product_alt = u * n;
            assert!(product_alt.is_valid());
            assert_eq!(product_alt, product);

            let product_alt = u * n.clone();
            assert!(product_alt.is_valid());
            assert_eq!(product_alt, product);

            assert_eq!(n * Natural::from(u), product);
            assert_eq!(Natural::from(u) * n, product);

            assert_eq!(
                biguint_to_natural(&num_mul_u32(natural_to_biguint(n), u)),
                product
            );
            assert_eq!(
                rug_integer_to_natural(&(natural_to_rug_integer(n) * u)),
                product
            );

            if *n != 0 && u != 0 {
                assert!(product >= *n);
                assert!(product >= u);
            }
            //TODO assert_eq!(product / u, n);
            //TODO assert_eq!(product / n, u);
        },
    );

    #[allow(unknown_lints, erasing_op, identity_op)]
    test_properties(naturals, |n| {
        assert_eq!(n * 0u32, 0);
        assert_eq!(0u32 * n, 0);
        assert_eq!(n * 1u32, *n);
        assert_eq!(1u32 * n, *n);
        assert_eq!(n * 2u32, n << 1);
        assert_eq!(2u32 * n, n << 1);
    });

    test_properties(unsigneds, |&u: &u32| {
        assert_eq!(Natural::ZERO * u, 0);
        assert_eq!(u * Natural::ZERO, 0);
        assert_eq!(Natural::ONE * u, u);
        assert_eq!(u * Natural::ONE, u);
    });
}
