use common::test_properties;
use malachite_base::num::Zero;
use malachite_nz::natural::Natural;
use malachite_test::common::{biguint_to_natural, natural_to_biguint, natural_to_rug_integer,
                             rug_integer_to_natural};
use malachite_test::inputs::base::unsigneds;
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_unsigned};
use malachite_test::natural::arithmetic::add_u32::num_add_u32;
use num::BigUint;
use rug::{self, Assign};
use std::str::FromStr;

#[test]
fn test_add_u32() {
    let test = |u, v: u32, out| {
        let mut n = Natural::from_str(u).unwrap();
        n += v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from_str(u).unwrap();
        n += v;
        assert_eq!(n.to_string(), out);

        let n = Natural::from_str(u).unwrap() + v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = num_add_u32(BigUint::from_str(u).unwrap(), v);
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() + v;
        assert_eq!(n.to_string(), out);

        let n = &Natural::from_str(u).unwrap() + v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v + Natural::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v + rug::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);

        let n = v + &Natural::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from(0);
        n.assign(v + &rug::Integer::from_str(u).unwrap());
        assert_eq!(n.to_string(), out);
    };
    test("0", 0, "0");
    test("0", 123, "123");
    test("123", 0, "123");
    test("123", 456, "579");
    test("1000000000000", 123, "1000000000123");
    test("4294967295", 1, "4294967296");
    test("18446744073709551615", 1, "18446744073709551616");
}

#[test]
fn add_u32_properties() {
    test_properties(
        pairs_of_natural_and_unsigned,
        |&(ref n, u): &(Natural, u32)| {
            let mut mut_n = n.clone();
            mut_n += u;
            assert!(mut_n.is_valid());
            let sum = mut_n;

            let mut rug_n = natural_to_rug_integer(n);
            rug_n += u;
            assert_eq!(rug_integer_to_natural(&rug_n), sum);

            let sum_alt = n + u;
            assert!(sum_alt.is_valid());
            assert_eq!(sum_alt, sum);

            let sum_alt = n.clone() + u;
            assert!(sum_alt.is_valid());
            assert_eq!(sum_alt, sum);

            let sum_alt = u + n;
            assert!(sum_alt.is_valid());
            assert_eq!(sum_alt, sum);

            let sum_alt = u + n.clone();
            assert!(sum_alt.is_valid());
            assert_eq!(sum_alt, sum);

            assert_eq!(n + Natural::from(u), sum);
            assert_eq!(Natural::from(u) + n, sum);

            assert_eq!(
                biguint_to_natural(&num_add_u32(natural_to_biguint(n), u)),
                sum
            );
            assert_eq!(
                rug_integer_to_natural(&(natural_to_rug_integer(n) + u)),
                sum
            );

            assert!(sum >= *n);
            assert!(sum >= u);
            assert_eq!((&sum - u).as_ref(), Some(n));
            assert_eq!(sum - n, Some(Natural::from(u)));
        },
    );

    #[allow(unknown_lints, identity_op)]
    test_properties(naturals, |n| {
        assert_eq!(n + 0u32, *n);
        assert_eq!(0u32 + n, *n);
    });

    test_properties(unsigneds, |&u: &u32| {
        assert_eq!(Natural::ZERO + u, u);
        assert_eq!(u + Natural::ZERO, u);
    });
}
