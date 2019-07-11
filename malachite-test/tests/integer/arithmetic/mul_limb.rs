use common::test_properties;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::platform::{DoubleLimb, Limb};
use malachite_test::common::{bigint_to_integer, integer_to_bigint};
#[cfg(feature = "32_bit_limbs")]
use malachite_test::common::{integer_to_rug_integer, rug_integer_to_integer};
use malachite_test::inputs::base::{pairs_of_unsigneds, unsigneds};
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_unsigned};
use malachite_test::inputs::natural::pairs_of_natural_and_unsigned;
use malachite_test::integer::arithmetic::mul_limb::num_mul_limb;
use num::BigInt;
#[cfg(feature = "32_bit_limbs")]
use rug::{self, Assign};
use std::str::FromStr;

#[test]
fn test_add_limb() {
    let test = |u, v: Limb, out| {
        let mut n = Integer::from_str(u).unwrap();
        n *= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        #[cfg(feature = "32_bit_limbs")]
        {
            let mut n = rug::Integer::from_str(u).unwrap();
            n *= v;
            assert_eq!(n.to_string(), out);
        }

        let n = Integer::from_str(u).unwrap() * v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = num_mul_limb(BigInt::from_str(u).unwrap(), v);
        assert_eq!(n.to_string(), out);

        #[cfg(feature = "32_bit_limbs")]
        {
            let n = rug::Integer::from_str(u).unwrap() * v;
            assert_eq!(n.to_string(), out);
        }

        let n = &Integer::from_str(u).unwrap() * v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v * Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        #[cfg(feature = "32_bit_limbs")]
        {
            let n = v * rug::Integer::from_str(u).unwrap();
            assert_eq!(n.to_string(), out);
        }

        let n = v * &Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        #[cfg(feature = "32_bit_limbs")]
        {
            let mut n = rug::Integer::from(0);
            n.assign(v * &rug::Integer::from_str(u).unwrap());
            assert_eq!(n.to_string(), out);
        }
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
    test("-1", 123, "-123");
    test("-123", 1, "-123");
    test("-123", 456, "-56088");
    test("-1000000000000", 0, "0");
    test("-1000000000000", 1, "-1000000000000");
    test("-1000000000000", 123, "-123000000000000");
    test("-4294967295", 2, "-8589934590");
    test("-4294967296", 2, "-8589934592");
    test("-18446744073709551615", 2, "-36893488147419103230");
}

#[test]
fn mul_limb_properties() {
    test_properties(
        pairs_of_integer_and_unsigned,
        |&(ref n, u): &(Integer, Limb)| {
            let mut mut_n = n.clone();
            mut_n *= u;
            assert!(mut_n.is_valid());
            let product = mut_n;

            #[cfg(feature = "32_bit_limbs")]
            {
                let mut rug_n = integer_to_rug_integer(n);
                rug_n *= u;
                assert_eq!(rug_integer_to_integer(&rug_n), product);
            }

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

            let product_alt = n * Integer::from(u);
            assert_eq!(product_alt, product);
            let product_alt = Integer::from(u) * n.clone();
            assert_eq!(product_alt, product);

            assert_eq!(
                bigint_to_integer(&num_mul_limb(integer_to_bigint(n), u)),
                product
            );
            #[cfg(feature = "32_bit_limbs")]
            {
                assert_eq!(
                    rug_integer_to_integer(&(integer_to_rug_integer(n) * u)),
                    product
                );
            }

            assert_eq!((-n) * u, -(n * u));
            if u != 0 {
                assert_eq!(product / u, *n);
            }
        },
    );

    #[allow(unknown_lints, erasing_op, identity_op)]
    test_properties(integers, |n| {
        assert_eq!(n * 0 as Limb, 0 as Limb);
        assert_eq!(0 as Limb * n, 0 as Limb);
        assert_eq!(n * 1 as Limb, *n);
        assert_eq!(1 as Limb * n, *n);
        assert_eq!(n * 2 as Limb, n << 1);
        assert_eq!(2 as Limb * n, n << 1);
    });

    test_properties(unsigneds, |&u: &Limb| {
        assert_eq!(Integer::ZERO * u, 0 as Limb);
        assert_eq!(u * Integer::ZERO, 0 as Limb);
        assert_eq!(Integer::ONE * u, u);
        assert_eq!(u * Integer::ONE, u);
    });

    test_properties(pairs_of_unsigneds::<Limb>, |&(x, y)| {
        let product = Integer::from(DoubleLimb::from(x) * DoubleLimb::from(y));
        assert_eq!(product, Integer::from(x) * y);
        assert_eq!(product, x * Integer::from(y));
    });

    test_properties(pairs_of_natural_and_unsigned::<Limb>, |&(ref n, u)| {
        assert_eq!(n * u, Integer::from(n) * u);
    });
}
