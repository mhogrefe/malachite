use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{
    SaturatingSub, SaturatingSubMul, SaturatingSubMulAssign,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use common::test_properties;
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_unsigned, pairs_of_naturals,
    triples_of_natural_natural_and_unsigned,
};

#[test]
fn test_sub_mul_limb() {
    let test = |u, v, c: Limb, out: &str| {
        let mut n = Natural::from_str(u).unwrap();
        n.saturating_sub_mul_assign(Natural::from_str(v).unwrap(), c);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n.saturating_sub_mul_assign(&Natural::from_str(v).unwrap(), c);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u)
            .unwrap()
            .saturating_sub_mul(Natural::from_str(v).unwrap(), c);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u)
            .unwrap()
            .saturating_sub_mul(&Natural::from_str(v).unwrap(), c);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n =
            (&Natural::from_str(u).unwrap()).saturating_sub_mul(Natural::from_str(v).unwrap(), c);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n =
            (&Natural::from_str(u).unwrap()).saturating_sub_mul(&Natural::from_str(v).unwrap(), c);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0", 0, "0");
    test("0", "0", 123, "0");
    test("123", "0", 5, "123");
    test("123", "5", 1, "118");
    test("123", "5", 100, "0");
    test("10", "3", 4, "0");
    test("15", "3", 4, "3");
    test("1000000000000", "0", 123, "1000000000000");
    test("1000000000000", "1", 123, "999999999877");
    test("1000000000000", "123", 1, "999999999877");
    test("1000000000000", "123", 100, "999999987700");
    test("1000000000000", "100", 123, "999999987700");
    test("1000000000000", "65536", 0x1_0000, "995705032704");
    test("1000000000000", "1000000000000", 0, "1000000000000");
    test("1000000000000", "1000000000000", 1, "0");
    test("1000000000000", "1000000000000", 100, "0");
    test("0", "1000000000000", 100, "0");
    test("4294967296", "1", 1, "4294967295");
    test("3902609153", "88817093856604", 1, "0");
    test(
        "1000000000000000000000",
        "1000000000000",
        1_000_000_000,
        "0",
    );
}

#[test]
fn saturating_sub_mul_limb_properties() {
    test_properties(
        triples_of_natural_natural_and_unsigned,
        |&(ref a, ref b, c): &(Natural, Natural, Limb)| {
            let mut mut_a = a.clone();
            mut_a.saturating_sub_mul_assign(b, c);
            assert!(mut_a.is_valid());
            let result = mut_a;

            let mut mut_a = a.clone();
            mut_a.saturating_sub_mul_assign(b.clone(), c);
            assert!(mut_a.is_valid());
            assert_eq!(mut_a, result);

            let result_alt = a.saturating_sub_mul(b, c);
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = a.saturating_sub_mul(b, c.clone());
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = a.saturating_sub_mul(b.clone(), c);
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = a.saturating_sub_mul(b.clone(), c.clone());
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            assert_eq!(a.saturating_sub(b * c), result);
            assert_eq!(a.saturating_sub_mul(b, &Natural::from(c)), result);
            assert!(result <= *a);
        },
    );

    test_properties(naturals, |n| {
        assert_eq!(n.saturating_sub_mul(n, 1 as Limb), Natural::ZERO);
    });

    test_properties(
        pairs_of_natural_and_unsigned,
        |&(ref n, c): &(Natural, Limb)| {
            assert_eq!(Natural::ZERO.saturating_sub_mul(n, c), Natural::ZERO);
            assert_eq!(n.saturating_sub_mul(Natural::ZERO, c), *n);
            assert_eq!(n.saturating_sub_mul(Natural::ONE, c), n.saturating_sub(c));
            assert_eq!((n * c).saturating_sub_mul(n, c), Natural::ZERO);
        },
    );

    test_properties(pairs_of_naturals, |&(ref a, ref b)| {
        assert_eq!(a.saturating_sub_mul(b, 0 as Limb), *a);
        assert_eq!(a.saturating_sub_mul(b, 1 as Limb), a.saturating_sub(b));
    });
}
