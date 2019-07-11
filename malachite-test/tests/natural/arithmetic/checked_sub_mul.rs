use common::test_properties;
use malachite_base::num::arithmetic::traits::{CheckedSub, CheckedSubMul};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::Natural;
use malachite_test::inputs::natural::{naturals, pairs_of_naturals, triples_of_naturals};
use std::str::FromStr;

#[test]
fn test_checked_sub_mul() {
    let test = |u, v, w, out| {
        let on = Natural::from_str(u)
            .unwrap()
            .checked_sub_mul(Natural::from_str(v).unwrap(), Natural::from_str(w).unwrap());
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = Natural::from_str(u).unwrap().checked_sub_mul(
            Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
        );
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = Natural::from_str(u).unwrap().checked_sub_mul(
            &Natural::from_str(v).unwrap(),
            Natural::from_str(w).unwrap(),
        );
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = Natural::from_str(u).unwrap().checked_sub_mul(
            &Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
        );
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = (&Natural::from_str(u).unwrap()).checked_sub_mul(
            &Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
        );
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));
    };
    test("0", "0", "0", "Some(0)");
    test("0", "0", "123", "Some(0)");
    test("123", "0", "5", "Some(123)");
    test("123", "5", "1", "Some(118)");
    test("123", "5", "100", "None");
    test("10", "3", "4", "None");
    test("15", "3", "4", "Some(3)");
    test("1000000000000", "0", "123", "Some(1000000000000)");
    test("1000000000000", "1", "123", "Some(999999999877)");
    test("1000000000000", "123", "1", "Some(999999999877)");
    test("1000000000000", "123", "100", "Some(999999987700)");
    test("1000000000000", "100", "123", "Some(999999987700)");
    test("1000000000000", "65536", "65536", "Some(995705032704)");
    test("1000000000000", "1000000000000", "0", "Some(1000000000000)");
    test("1000000000000", "1000000000000", "1", "Some(0)");
    test("1000000000000", "1000000000000", "100", "None");
    test("0", "1000000000000", "100", "None");
    test("4294967296", "1", "1", "Some(4294967295)");
    test("3902609153", "88817093856604", "1", "None");
}

#[test]
fn checked_sub_mul_properties() {
    test_properties(triples_of_naturals, |&(ref a, ref b, ref c)| {
        let result = a.checked_sub_mul(b, c);
        assert!(result.as_ref().map_or(true, |n| n.is_valid()));

        let result_alt = a.clone().checked_sub_mul(b, c);
        assert!(result_alt.as_ref().map_or(true, |n| n.is_valid()));
        assert_eq!(result_alt, result);

        let result_alt = a.clone().checked_sub_mul(b, c.clone());
        assert!(result_alt.as_ref().map_or(true, |n| n.is_valid()));
        assert_eq!(result_alt, result);

        let result_alt = a.clone().checked_sub_mul(b.clone(), c);
        assert!(result_alt.as_ref().map_or(true, |n| n.is_valid()));
        assert_eq!(result_alt, result);

        let result_alt = a.clone().checked_sub_mul(b.clone(), c.clone());
        assert!(result_alt.as_ref().map_or(true, |n| n.is_valid()));
        assert_eq!(result_alt, result);

        assert_eq!(a.checked_sub(b * c), result);
    });

    test_properties(naturals, |n| {
        assert_eq!(n.checked_sub_mul(n, &Natural::ONE), Some(Natural::ZERO));
    });

    test_properties(pairs_of_naturals, |&(ref a, ref b)| {
        assert_eq!(a.checked_sub_mul(&Natural::ZERO, b).as_ref(), Some(a));
        assert_eq!(a.checked_sub_mul(b, &Natural::ZERO).as_ref(), Some(a));
        assert_eq!(a.checked_sub_mul(&Natural::ONE, b), a.checked_sub(b));
        assert_eq!(a.checked_sub_mul(b, &Natural::ONE), a.checked_sub(b));
    });
}
