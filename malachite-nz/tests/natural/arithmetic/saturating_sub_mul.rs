use malachite_base::num::arithmetic::traits::{SaturatingSubMul, SaturatingSubMulAssign};
use malachite_nz::natural::Natural;
use std::str::FromStr;

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
