use common::LARGE_LIMIT;
use malachite_base::traits::{One, Zero};
use malachite_base::traits::{SubMul, SubMulAssign};
use malachite_nz::natural::Natural;
use malachite_test::common::GenerationMode;
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
    // a.sub_mul_assign(&b, c); a is valid.
    // a.sub_mul(&b, c) is valid.
    // (&a).sub_mul(&b, c) is valid.
    // a.sub_mul_assign(&b, c), a.sub_mul(&b, c), and (&a).sub_mul(&b, c) give the same result.
    // a.sub_mul(&b, c) is equivalent to a - b * c.
    let three_naturals = |mut a: Natural, b: Natural, c: Natural| {
        let old_a = a.clone();
        if a >= &b * &c {
            a.sub_mul_assign(&b, &c);
            assert!(a.is_valid());
        }
        let oa = if old_a >= &b * &c { Some(a) } else { None };

        let a2 = old_a.clone();
        let result = (&a2).sub_mul(&b, &c);
        assert!(result.clone().map_or(true, |n| n.is_valid()));
        assert_eq!(result, oa);

        let result = a2.sub_mul(&b, &c);
        assert!(result.clone().map_or(true, |n| n.is_valid()));
        assert_eq!(result, oa);

        assert_eq!(old_a - &(b * c), oa);
    };

    // a.sub_mul(b, 0) == Some(a)
    // a.sub_mul(b, 1) == a - b
    let two_naturals = |a: &Natural, b: &Natural| {
        assert_eq!(a.sub_mul(&Natural::ZERO, b), Some(a.clone()));
        assert_eq!(a.sub_mul(b, &Natural::ZERO), Some(a.clone()));
        assert_eq!(a.sub_mul(&Natural::ONE, b), a - b);
        assert_eq!(a.sub_mul(b, &Natural::ONE), a - b);
    };

    for (a, b, c) in triples_of_naturals(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        three_naturals(a, b, c);
    }

    for (a, b, c) in triples_of_naturals(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        three_naturals(a, b, c);
    }

    for (a, b) in pairs_of_naturals(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        two_naturals(&a, &b);
    }

    for (a, b) in pairs_of_naturals(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        two_naturals(&a, &b);
    }
}
