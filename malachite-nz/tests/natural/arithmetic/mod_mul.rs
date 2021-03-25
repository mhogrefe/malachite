use malachite_base::num::arithmetic::traits::{ModIsReduced, ModMul, ModMulAssign};
#[cfg(feature = "32_bit_limbs")]
use malachite_nz_test_util::natural::arithmetic::mod_mul::{
    limbs_mod_mul_two_limbs_naive, limbs_precompute_mod_mul_two_limbs_alt,
};
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::arithmetic::mod_mul::{
    _limbs_mod_mul_two_limbs, _limbs_precompute_mod_mul_two_limbs,
};
use malachite_nz::natural::Natural;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_precompute_mod_mul_two_limbs() {
    let test = |m_1, m_0, inv_2, inv_1, inv_0| {
        assert_eq!(
            _limbs_precompute_mod_mul_two_limbs(m_1, m_0),
            (inv_2, inv_1, inv_0)
        );
        assert_eq!(
            limbs_precompute_mod_mul_two_limbs_alt(m_1, m_0),
            (inv_2, inv_1, inv_0)
        );
    };
    test(1, 1, u32::MAX, 0, u32::MAX);
    test(1, 2, u32::MAX - 1, 3, 0xfffffff8);
    test(123, 456, 34918433, 1162528328, 1277088208);
    test(u32::MAX, u32::MAX - 1, 1, 0, 2);
    test(u32::MAX, u32::MAX, 1, 0, 1);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_mul_two_limbs() {
    let test = |x_1, x_0, y_1, y_0, m_1, m_0, r_1, r_0| {
        let (inv_2, inv_1, inv_0) = _limbs_precompute_mod_mul_two_limbs(m_1, m_0);
        assert_eq!(
            _limbs_mod_mul_two_limbs(x_1, x_0, y_1, y_0, m_1, m_0, inv_2, inv_1, inv_0),
            (r_1, r_0)
        );
        assert_eq!(
            limbs_mod_mul_two_limbs_naive(x_1, x_0, y_1, y_0, m_1, m_0),
            (r_1, r_0)
        );
    };
    test(0, 0, 0, 0, 1, 1, 0, 0);
    test(1, 0, 0, 1, 1, 1, 1, 0);
    test(123, 456, 654, 321, 789, 876, 213, 4164192732);
    test(123, 456, 789, 876, u32::MAX, u32::MAX, 467532, 496503);
}

#[test]
fn test_mod_mul() {
    let test = |u, v, m, out| {
        assert!(Natural::from_str(u)
            .unwrap()
            .mod_is_reduced(&Natural::from_str(m).unwrap()));
        assert!(Natural::from_str(v)
            .unwrap()
            .mod_is_reduced(&Natural::from_str(m).unwrap()));

        let mut n = Natural::from_str(u).unwrap();
        n.mod_mul_assign(Natural::from_str(v).unwrap(), Natural::from_str(m).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert!(n.mod_is_reduced(&Natural::from_str(m).unwrap()));

        let mut n = Natural::from_str(u).unwrap();
        n.mod_mul_assign(
            &Natural::from_str(v).unwrap(),
            Natural::from_str(m).unwrap(),
        );
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n.mod_mul_assign(
            Natural::from_str(v).unwrap(),
            &Natural::from_str(m).unwrap(),
        );
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n.mod_mul_assign(
            &Natural::from_str(v).unwrap(),
            &Natural::from_str(m).unwrap(),
        );
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u)
            .unwrap()
            .mod_mul(Natural::from_str(v).unwrap(), Natural::from_str(m).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap())
            .mod_mul(Natural::from_str(v).unwrap(), Natural::from_str(m).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap().mod_mul(
            &Natural::from_str(v).unwrap(),
            Natural::from_str(m).unwrap(),
        );
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).mod_mul(
            &Natural::from_str(v).unwrap(),
            Natural::from_str(m).unwrap(),
        );
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap().mod_mul(
            Natural::from_str(v).unwrap(),
            &Natural::from_str(m).unwrap(),
        );
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).mod_mul(
            Natural::from_str(v).unwrap(),
            &Natural::from_str(m).unwrap(),
        );
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap().mod_mul(
            &Natural::from_str(v).unwrap(),
            &Natural::from_str(m).unwrap(),
        );
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).mod_mul(
            &Natural::from_str(v).unwrap(),
            &Natural::from_str(m).unwrap(),
        );
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        assert_eq!(
            (Natural::from_str(u).unwrap() * Natural::from_str(v).unwrap()
                % Natural::from_str(m).unwrap())
            .to_string(),
            out
        );
    };
    test("0", "0", "1", "0");
    test("1", "0", "32", "0");
    test("1", "2", "32", "2");
    test("3", "4", "15", "12");
    test("7", "6", "10", "2");
    test("10", "14", "16", "12");
    test("1", "123", "128", "123");
    test("123", "1", "128", "123");
    test("123", "456", "512", "280");
    test("1000000000", "2000000000", "4294967296", "1321730048");
    test("1000000000", "2000000000", "4294967297", "856068761");
    test(
        "1000000000000000",
        "2000000000000000",
        "1000000000000000000000001",
        "999999999999999998000001",
    );
}
