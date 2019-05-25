use std::str::FromStr;

use malachite_base::num::traits::{
    One, SaturatingSub, SaturatingSubMul, SaturatingSubMulAssign, Zero,
};
use malachite_nz::natural::Natural;

use common::test_properties;
use malachite_test::inputs::natural::{naturals, pairs_of_naturals, triples_of_naturals};

#[test]
fn test_saturating_sub_mul() {
    let test = |u, v, w, out: &str| {
        let mut n = Natural::from_str(u).unwrap();
        n.saturating_sub_mul_assign(Natural::from_str(v).unwrap(), Natural::from_str(w).unwrap());
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let mut n = Natural::from_str(u).unwrap();
        n.saturating_sub_mul_assign(
            Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
        );
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let mut n = Natural::from_str(u).unwrap();
        n.saturating_sub_mul_assign(
            &Natural::from_str(v).unwrap(),
            Natural::from_str(w).unwrap(),
        );
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let mut n = Natural::from_str(u).unwrap();
        n.saturating_sub_mul_assign(
            &Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
        );
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = Natural::from_str(u)
            .unwrap()
            .saturating_sub_mul(Natural::from_str(v).unwrap(), Natural::from_str(w).unwrap());
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = Natural::from_str(u).unwrap().saturating_sub_mul(
            Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
        );
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = Natural::from_str(u).unwrap().saturating_sub_mul(
            &Natural::from_str(v).unwrap(),
            Natural::from_str(w).unwrap(),
        );
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = Natural::from_str(u).unwrap().saturating_sub_mul(
            &Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
        );
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = (&Natural::from_str(u).unwrap()).saturating_sub_mul(
            &Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
        );
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0", "0");
    test("0", "0", "123", "0");
    test("123", "0", "5", "123");
    test("123", "5", "1", "118");
    test("123", "5", "100", "0");
    test("10", "3", "4", "0");
    test("15", "3", "4", "3");
    test("1000000000000", "0", "123", "1000000000000");
    test("1000000000000", "1", "123", "999999999877");
    test("1000000000000", "123", "1", "999999999877");
    test("1000000000000", "123", "100", "999999987700");
    test("1000000000000", "100", "123", "999999987700");
    test("1000000000000", "65536", "65536", "995705032704");
    test("1000000000000", "1000000000000", "0", "1000000000000");
    test("1000000000000", "1000000000000", "1", "0");
    test("1000000000000", "1000000000000", "100", "0");
    test("0", "1000000000000", "100", "0");
    test("4294967296", "1", "1", "4294967295");
    test("3902609153", "88817093856604", "1", "0");
}

#[test]
fn saturating_sub_mul_properties() {
    test_properties(triples_of_naturals, |&(ref a, ref b, ref c)| {
        let mut mut_a = a.clone();
        mut_a.saturating_sub_mul_assign(b, c);
        assert!(mut_a.is_valid());
        let result = mut_a;

        let mut mut_a = a.clone();
        mut_a.saturating_sub_mul_assign(b, c.clone());
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let mut mut_a = a.clone();
        mut_a.saturating_sub_mul_assign(b.clone(), c);
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let mut mut_a = a.clone();
        mut_a.saturating_sub_mul_assign(b.clone(), c.clone());
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let result_alt = a.saturating_sub_mul(b, c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().saturating_sub_mul(b, c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().saturating_sub_mul(b, c.clone());
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().saturating_sub_mul(b.clone(), c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().saturating_sub_mul(b.clone(), c.clone());
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!(a.saturating_sub(b * c), result);
        assert!(result <= *a);
    });

    test_properties(naturals, |n| {
        assert_eq!(n.saturating_sub_mul(n, &Natural::ONE), Natural::ZERO);
    });

    test_properties(pairs_of_naturals, |&(ref a, ref b)| {
        assert_eq!(Natural::ZERO.saturating_sub_mul(a, b), Natural::ZERO);
        assert_eq!(a.saturating_sub_mul(&Natural::ZERO, b), *a);
        assert_eq!(a.saturating_sub_mul(b, &Natural::ZERO), *a);
        assert_eq!((a * b).saturating_sub_mul(a, b), Natural::ZERO);
        assert_eq!(a.saturating_sub_mul(&Natural::ONE, b), a.saturating_sub(b));
        assert_eq!(a.saturating_sub_mul(b, &Natural::ONE), a.saturating_sub(b));
    });
}
