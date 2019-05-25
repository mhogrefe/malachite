use std::str::FromStr;

use malachite_base::num::traits::{CheckedSub, One, SubMul, SubMulAssign, Zero};
use malachite_nz::natural::Natural;

use common::test_properties;
use malachite_test::inputs::natural::{
    naturals, pairs_of_naturals, pairs_of_naturals_var_1, triples_of_naturals_var_1,
};

#[test]
fn test_sub_mul() {
    let test = |u, v, w, out: &str| {
        let mut n = Natural::from_str(u).unwrap();
        n.sub_mul_assign(Natural::from_str(v).unwrap(), Natural::from_str(w).unwrap());
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let mut n = Natural::from_str(u).unwrap();
        n.sub_mul_assign(
            Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
        );
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let mut n = Natural::from_str(u).unwrap();
        n.sub_mul_assign(
            &Natural::from_str(v).unwrap(),
            Natural::from_str(w).unwrap(),
        );
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let mut n = Natural::from_str(u).unwrap();
        n.sub_mul_assign(
            &Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
        );
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = Natural::from_str(u)
            .unwrap()
            .sub_mul(Natural::from_str(v).unwrap(), Natural::from_str(w).unwrap());
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = Natural::from_str(u).unwrap().sub_mul(
            Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
        );
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = Natural::from_str(u).unwrap().sub_mul(
            &Natural::from_str(v).unwrap(),
            Natural::from_str(w).unwrap(),
        );
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = Natural::from_str(u).unwrap().sub_mul(
            &Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
        );
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = (&Natural::from_str(u).unwrap()).sub_mul(
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
#[should_panic]
fn sub_mul_assign_fail_1() {
    let mut x = Natural::from_str("123").unwrap();
    x.sub_mul_assign(
        Natural::from_str("5").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_fail_2() {
    let mut x = Natural::from_str("1000000000000").unwrap();
    x.sub_mul_assign(
        Natural::from_str("1000000000000").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_val_ref_fail_1() {
    let mut x = Natural::from_str("123").unwrap();
    x.sub_mul_assign(
        Natural::from_str("5").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_val_ref_fail_2() {
    let mut x = Natural::from_str("1000000000000").unwrap();
    x.sub_mul_assign(
        Natural::from_str("1000000000000").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_ref_val_fail_1() {
    let mut x = Natural::from_str("123").unwrap();
    x.sub_mul_assign(
        &Natural::from_str("5").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_ref_val_fail_2() {
    let mut x = Natural::from_str("1000000000000").unwrap();
    x.sub_mul_assign(
        &Natural::from_str("1000000000000").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_ref_ref_fail_1() {
    let mut x = Natural::from_str("123").unwrap();
    x.sub_mul_assign(
        &Natural::from_str("5").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_ref_ref_fail_2() {
    let mut x = Natural::from_str("1000000000000").unwrap();
    x.sub_mul_assign(
        &Natural::from_str("1000000000000").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_fail_1() {
    Natural::from_str("123").unwrap().sub_mul(
        Natural::from_str("5").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_fail_2() {
    Natural::from_str("1000000000000").unwrap().sub_mul(
        Natural::from_str("1000000000000").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_val_val_ref_fail_1() {
    Natural::from_str("123").unwrap().sub_mul(
        Natural::from_str("5").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_val_val_ref_fail_2() {
    Natural::from_str("1000000000000").unwrap().sub_mul(
        Natural::from_str("1000000000000").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_val_ref_val_fail_1() {
    Natural::from_str("123").unwrap().sub_mul(
        &Natural::from_str("5").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_val_ref_val_fail_2() {
    Natural::from_str("1000000000000").unwrap().sub_mul(
        &Natural::from_str("1000000000000").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_val_ref_ref_fail_1() {
    Natural::from_str("123").unwrap().sub_mul(
        &Natural::from_str("5").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_val_ref_ref_fail_2() {
    Natural::from_str("1000000000000").unwrap().sub_mul(
        &Natural::from_str("1000000000000").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_ref_ref_ref_fail_1() {
    (&Natural::from_str("123").unwrap()).sub_mul(
        &Natural::from_str("5").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_ref_ref_ref_fail_2() {
    (&Natural::from_str("1000000000000").unwrap()).sub_mul(
        &Natural::from_str("1000000000000").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
fn sub_mul_properties() {
    test_properties(triples_of_naturals_var_1, |&(ref a, ref b, ref c)| {
        let mut mut_a = a.clone();
        mut_a.sub_mul_assign(b, c);
        assert!(mut_a.is_valid());
        let result = mut_a;

        let mut mut_a = a.clone();
        mut_a.sub_mul_assign(b, c.clone());
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let mut mut_a = a.clone();
        mut_a.sub_mul_assign(b.clone(), c);
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let mut mut_a = a.clone();
        mut_a.sub_mul_assign(b.clone(), c.clone());
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let result_alt = a.sub_mul(b, c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().sub_mul(b, c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().sub_mul(b, c.clone());
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().sub_mul(b.clone(), c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().sub_mul(b.clone(), c.clone());
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!(a - b * c, result);
        assert_eq!(a.checked_sub(b * c), Some(result));
    });

    test_properties(naturals, |n| {
        assert_eq!(n.sub_mul(n, &Natural::ONE), Natural::ZERO);
    });

    test_properties(pairs_of_naturals, |&(ref a, ref b)| {
        assert_eq!(a.sub_mul(&Natural::ZERO, b), *a);
        assert_eq!(a.sub_mul(b, &Natural::ZERO), *a);
        assert_eq!((a * b).sub_mul(a, b), Natural::ZERO);
    });

    test_properties(pairs_of_naturals_var_1, |&(ref a, ref b)| {
        assert_eq!(a.sub_mul(&Natural::ONE, b), a - b);
        assert_eq!(a.sub_mul(b, &Natural::ONE), a - b);
    });
}
