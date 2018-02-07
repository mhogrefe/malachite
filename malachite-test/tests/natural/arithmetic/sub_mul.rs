use common::test_properties;
use malachite_base::num::{One, Zero};
use malachite_base::num::{SubMul, SubMulAssign};
use malachite_nz::natural::Natural;
use malachite_test::inputs::natural::{pairs_of_naturals, triples_of_naturals};
use std::str::FromStr;

#[test]
fn test_sub_mul_assign() {
    let test = |u, v, w, out: &str| {
        let mut n = Natural::from_str(u).unwrap();
        n.sub_mul_assign(
            &Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
        );
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0", "0", "0");
    test("0", "0", "123", "0");
    test("123", "0", "5", "123");
    test("123", "5", "1", "118");
    test("15", "3", "4", "3");
    test("1000000000000", "0", "123", "1000000000000");
    test("1000000000000", "1", "123", "999999999877");
    test("1000000000000", "123", "1", "999999999877");
    test("1000000000000", "123", "100", "999999987700");
    test("1000000000000", "100", "123", "999999987700");
    test("1000000000000", "65536", "65536", "995705032704");
    test("1000000000000", "1000000000000", "0", "1000000000000");
    test("1000000000000", "1000000000000", "1", "0");
    test("4294967296", "1", "1", "4294967295");
    test(
        "1000000000000000000000000",
        "1000000000000",
        "1000000000000",
        "0",
    );
    test(
        "1000000000001000000000000",
        "1000000000000",
        "1000000000000",
        "1000000000000",
    );
}

#[test]
#[should_panic(expected = "Natural sub_mul_assign cannot have a negative result")]
fn sub_mul_assign_fail_1() {
    let mut x = Natural::from_str("123").unwrap();
    x.sub_mul_assign(
        &Natural::from_str("5").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic(expected = "Natural sub_mul_assign cannot have a negative result")]
fn sub_mul_assign_fail_2() {
    let mut x = Natural::from_str("1000000000000").unwrap();
    x.sub_mul_assign(
        &Natural::from_str("1000000000000").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
fn test_sub_mul() {
    let test = |u, v, w, out| {
        let on = Natural::from_str(u).unwrap().sub_mul(
            &Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
        );
        assert_eq!(format!("{:?}", on), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = (&Natural::from_str(u).unwrap()).sub_mul(
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
fn sub_mul_properties() {
    test_properties(triples_of_naturals, |&(ref a, ref b, ref c)| {
        let result = if *a >= b * c {
            let mut mut_a = a.clone();
            mut_a.sub_mul_assign(b, c);
            assert!(mut_a.is_valid());
            Some(mut_a)
        } else {
            None
        };

        let result_alt = a.sub_mul(b, c);
        assert!(result_alt.as_ref().map_or(true, |n| n.is_valid()));
        assert_eq!(result_alt, result);

        let result_alt = a.clone().sub_mul(b, c);
        assert!(result_alt.as_ref().map_or(true, |n| n.is_valid()));
        assert_eq!(result_alt, result);

        assert_eq!(a - &(b * c), result);
    });

    test_properties(pairs_of_naturals, |&(ref a, ref b)| {
        assert_eq!(a.sub_mul(&Natural::ZERO, b).as_ref(), Some(a));
        assert_eq!(a.sub_mul(b, &Natural::ZERO).as_ref(), Some(a));
        assert_eq!(a.sub_mul(&Natural::ONE, b), a - b);
        assert_eq!(a.sub_mul(b, &Natural::ONE), a - b);
    });
}
