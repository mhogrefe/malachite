use malachite_base::num::arithmetic::traits::{
    Abs, CoprimeWith, JacobiSymbol, KroneckerSymbol, LegendreSymbol, ModPowerOf2, Sign,
};
use malachite_base::num::basic::traits::{NegativeOne, One, Two, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::WrappingInto;
use malachite_base::test_util::generators::{signed_pair_gen, signed_pair_gen_var_8};
use malachite_nz::integer::Integer;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_nz::test_util::generators::{
    integer_gen, integer_gen_var_9, integer_pair_gen, integer_pair_gen_var_4,
    integer_pair_gen_var_5, integer_pair_gen_var_6, integer_triple_gen, integer_triple_gen_var_2,
    integer_triple_gen_var_3,
};
use std::borrow::Cow;
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_jacobi_symbol() {
    fn test(u: &str, v: &str, s: i8) {
        let a = Integer::from_str(u).unwrap();
        let n = Integer::from_str(v).unwrap();

        assert_eq!(a.clone().legendre_symbol(n.clone()), s);
        assert_eq!(a.clone().legendre_symbol(&n), s);
        assert_eq!((&a).legendre_symbol(n.clone()), s);
        assert_eq!((&a).legendre_symbol(&n), s);

        assert_eq!(a.clone().jacobi_symbol(n.clone()), s);
        assert_eq!(a.clone().jacobi_symbol(&n), s);
        assert_eq!((&a).jacobi_symbol(n.clone()), s);
        assert_eq!((&a).jacobi_symbol(&n), s);

        assert_eq!(a.clone().kronecker_symbol(n.clone()), s);
        assert_eq!(a.clone().kronecker_symbol(&n), s);
        assert_eq!((&a).kronecker_symbol(n.clone()), s);
        assert_eq!((&a).kronecker_symbol(&n), s);
    }
    test("0", "1", 1);
    test("0", "3", 0);
    test("1", "3", 1);
    test("2", "3", -1);
    test("0", "5", 0);
    test("1", "5", 1);
    test("2", "5", -1);
    test("3", "5", -1);
    test("4", "5", 1);
    test("0", "7", 0);
    test("1", "7", 1);
    test("2", "7", 1);
    test("3", "7", -1);
    test("4", "7", 1);
    test("5", "7", -1);
    test("6", "7", -1);
    test("0", "9", 0);
    test("1", "9", 1);
    test("2", "9", 1);
    test("3", "9", 0);
    test("4", "9", 1);
    test("5", "9", 1);
    test("6", "9", 0);
    test("7", "9", 1);
    test("8", "9", 1);

    test("7", "7", 0);
    test("8", "7", 1);
    test("9", "7", 1);
    test("10", "7", -1);
    test("11", "7", 1);
    test("12", "7", -1);
    test("13", "7", -1);
    test("9", "9", 0);
    test("10", "9", 1);
    test("11", "9", 1);
    test("12", "9", 0);
    test("13", "9", 1);
    test("14", "9", 1);
    test("15", "9", 0);
    test("16", "9", 1);
    test("17", "9", 1);

    test("-7", "7", 0);
    test("-6", "7", 1);
    test("-5", "7", 1);
    test("-4", "7", -1);
    test("-3", "7", 1);
    test("-2", "7", -1);
    test("-1", "7", -1);
    test("-9", "9", 0);
    test("-8", "9", 1);
    test("-7", "9", 1);
    test("-6", "9", 0);
    test("-5", "9", 1);
    test("-4", "9", 1);
    test("-3", "9", 0);
    test("-2", "9", 1);
    test("-1", "9", 1);

    test("1001", "9907", -1);
    test("10908", "9907", -1);
    test("-8906", "9907", -1);
}

#[test]
fn jacobi_symbol_fail_1() {
    assert_panic!(Integer::ONE.jacobi_symbol(Integer::TWO));
}

#[test]
fn jacobi_symbol_fail_2() {
    assert_panic!(Integer::ONE.jacobi_symbol(Integer::NEGATIVE_ONE));
}

// Odd n is already tested in test_jacobi_symbol, so here we just test even n
#[test]
fn test_kronecker_symbol() {
    fn test(u: &str, v: &str, s: i8) {
        let a = Integer::from_str(u).unwrap();
        let n = Integer::from_str(v).unwrap();

        assert_eq!(a.clone().kronecker_symbol(n.clone()), s);
        assert_eq!(a.clone().kronecker_symbol(&n), s);
        assert_eq!((&a).kronecker_symbol(n.clone()), s);
        assert_eq!((&a).kronecker_symbol(&n), s);
    }
    test("0", "2", 0);
    test("1", "2", 1);
    test("2", "2", 0);
    test("3", "2", -1);
    test("4", "2", 0);
    test("5", "2", -1);
    test("6", "2", 0);
    test("7", "2", 1);
    test("0", "4", 0);
    test("1", "4", 1);
    test("2", "4", 0);
    test("3", "4", 1);
    test("0", "6", 0);
    test("1", "6", 1);
    test("2", "6", 0);
    test("3", "6", 0);
    test("4", "6", 0);
    test("5", "6", 1);
    test("6", "6", 0);
    test("7", "6", 1);
    test("8", "6", 0);
    test("9", "6", 0);
    test("10", "6", 0);
    test("11", "6", 1);
    test("12", "6", 0);
    test("13", "6", -1);
    test("14", "6", 0);
    test("15", "6", 0);
    test("16", "6", 0);
    test("17", "6", -1);
    test("18", "6", 0);
    test("19", "6", -1);
    test("20", "6", 0);
    test("21", "6", 0);
    test("22", "6", 0);
    test("23", "6", -1);

    test("-1", "2", 1);
    test("-2", "2", 0);
    test("-3", "2", -1);
    test("-4", "2", 0);
    test("-5", "2", -1);
    test("-6", "2", 0);
    test("-7", "2", 1);
    test("-1", "4", 1);
    test("-2", "4", 0);
    test("-3", "4", 1);
    test("-1", "6", -1);
    test("-2", "6", 0);
    test("-3", "6", 0);
    test("-4", "6", 0);
    test("-5", "6", -1);
    test("-6", "6", 0);
    test("-7", "6", -1);
    test("-8", "6", 0);
    test("-9", "6", 0);
    test("-10", "6", 0);
    test("-11", "6", -1);
    test("-12", "6", 0);
    test("-13", "6", 1);
    test("-14", "6", 0);
    test("-15", "6", 0);
    test("-16", "6", 0);
    test("-17", "6", 1);
    test("-18", "6", 0);
    test("-19", "6", 1);
    test("-20", "6", 0);
    test("-21", "6", 0);
    test("-22", "6", 0);
    test("-23", "6", 1);

    test("0", "-2", 0);
    test("1", "-2", 1);
    test("2", "-2", 0);
    test("3", "-2", -1);
    test("4", "-2", 0);
    test("5", "-2", -1);
    test("6", "-2", 0);
    test("7", "-2", 1);
    test("0", "-4", 0);
    test("1", "-4", 1);
    test("2", "-4", 0);
    test("3", "-4", 1);
    test("0", "-6", 0);
    test("1", "-6", 1);
    test("2", "-6", 0);
    test("3", "-6", 0);
    test("4", "-6", 0);
    test("5", "-6", 1);
    test("6", "-6", 0);
    test("7", "-6", 1);
    test("8", "-6", 0);
    test("9", "-6", 0);
    test("10", "-6", 0);
    test("11", "-6", 1);
    test("12", "-6", 0);
    test("13", "-6", -1);
    test("14", "-6", 0);
    test("15", "-6", 0);
    test("16", "-6", 0);
    test("17", "-6", -1);
    test("18", "-6", 0);
    test("19", "-6", -1);
    test("20", "-6", 0);
    test("21", "-6", 0);
    test("22", "-6", 0);
    test("23", "-6", -1);

    test("-1", "-2", -1);
    test("-2", "-2", 0);
    test("-3", "-2", 1);
    test("-4", "-2", 0);
    test("-5", "-2", 1);
    test("-6", "-2", 0);
    test("-7", "-2", -1);
    test("-1", "-4", -1);
    test("-2", "-4", 0);
    test("-3", "-4", -1);
    test("-1", "-6", 1);
    test("-2", "-6", 0);
    test("-3", "-6", 0);
    test("-4", "-6", 0);
    test("-5", "-6", 1);
    test("-6", "-6", 0);
    test("-7", "-6", 1);
    test("-8", "-6", 0);
    test("-9", "-6", 0);
    test("-10", "-6", 0);
    test("-11", "-6", 1);
    test("-12", "-6", 0);
    test("-13", "-6", -1);
    test("-14", "-6", 0);
    test("-15", "-6", 0);
    test("-16", "-6", 0);
    test("-17", "-6", -1);
    test("-18", "-6", 0);
    test("-19", "-6", -1);
    test("-20", "-6", 0);
    test("-21", "-6", 0);
    test("-22", "-6", 0);
    test("-23", "-6", -1);

    test("1001", "-9908", -1);
    test("10909", "-9908", -1);
    test("-8907", "-9908", 1);
}

#[test]
fn jacobi_symbol_properties() {
    integer_pair_gen_var_4().test_properties(|(a, n)| {
        let s = (&a).jacobi_symbol(&n);
        assert_eq!((&a).legendre_symbol(&n), s);
        assert_eq!((&a).kronecker_symbol(&n), s);
        assert!(s.le_abs(&1i8));

        assert_eq!((&a + &n).jacobi_symbol(&n), s);
        assert_eq!((&a - &n).jacobi_symbol(&n), s);
        assert_eq!(
            s != 0,
            a.unsigned_abs_ref().coprime_with(n.unsigned_abs_ref())
        );
        let n_mod_8: u8 = (&(&n).mod_power_of_2(3)).wrapping_into();
        assert_eq!(
            (&a << 1u32).jacobi_symbol(&n),
            if n_mod_8 == 1 || n_mod_8 == 7 { s } else { -s }
        );
        let n_mod_4: u8 = (&(&n).mod_power_of_2(2)).wrapping_into();
        assert_eq!((-a).jacobi_symbol(n), if n_mod_4 == 1 { s } else { -s });
    });

    integer_pair_gen_var_6().test_properties(|(m, n)| {
        let n_mod_4: u8 = (&(&n).mod_power_of_2(2)).wrapping_into();
        let m_mod_4: u8 = (&(&m).mod_power_of_2(2)).wrapping_into();
        assert_eq!(
            (&m).jacobi_symbol(&n) * n.jacobi_symbol(m),
            if n_mod_4 == 1 || m_mod_4 == 1 { 1 } else { -1 }
        );
    });

    integer_triple_gen_var_2().test_properties(|(a, b, n)| {
        assert_eq!(
            (&a * &b).jacobi_symbol(&n),
            a.jacobi_symbol(&n) * b.jacobi_symbol(n)
        );
    });

    integer_triple_gen_var_3().test_properties(|(a, m, n)| {
        assert_eq!(
            (&a).jacobi_symbol(&m * &n),
            (&a).jacobi_symbol(m) * a.jacobi_symbol(n)
        );
    });

    integer_gen_var_9().test_properties(|n| {
        if n != 1u32 {
            assert_eq!(Integer::ZERO.jacobi_symbol(&n), 0);
            assert_eq!((&n).jacobi_symbol(&n), 0);
        }
        assert_eq!(Integer::ONE.jacobi_symbol(&n), 1);
        assert_eq!((&n).jacobi_symbol(Integer::ONE), 1);
        let n_mod_4: u8 = (&(&n).mod_power_of_2(2)).wrapping_into();
        assert_eq!(
            Integer::NEGATIVE_ONE.jacobi_symbol(&n),
            if n_mod_4 == 1 { 1 } else { -1 }
        );
        let n_mod_8: u8 = (&(&n).mod_power_of_2(3)).wrapping_into();
        assert_eq!(
            Integer::TWO.jacobi_symbol(n),
            if n_mod_8 == 1 || n_mod_8 == 7 { 1 } else { -1 }
        );
    });

    signed_pair_gen_var_8::<Limb, SignedLimb>().test_properties(|(x, y)| {
        assert_eq!(
            Integer::from(x).jacobi_symbol(Integer::from(y)),
            x.jacobi_symbol(y)
        );
    });
}

#[test]
fn kronecker_symbol_properties() {
    integer_pair_gen().test_properties(|(a, n)| {
        let s = (&a).kronecker_symbol(&n);
        assert!(s.le_abs(&1i8));

        assert_eq!(
            s != 0,
            a.unsigned_abs_ref().coprime_with(n.unsigned_abs_ref())
        );
        let n_mod_4: u8 = (&(&n).mod_power_of_2(2)).wrapping_into();
        if n_mod_4 == 2 {
            let four_n = &n << 2u32;
            let b = &a + &four_n;
            if n > 0u32 || a.sign() == b.sign() {
                assert_eq!(b.kronecker_symbol(&n), s);
            }
            let b = &a - four_n;
            if n > 0u32 || a.sign() == b.sign() {
                assert_eq!(b.kronecker_symbol(&n), s);
            }
        } else {
            let b = &a + &n;
            if n > 0u32 || a.sign() == b.sign() {
                assert_eq!(b.kronecker_symbol(&n), s);
            }
            let b = &a - &n;
            if n > 0u32 || a.sign() == b.sign() {
                assert_eq!(b.kronecker_symbol(&n), s);
            }
        }
        let a_mod_4: u8 = (&(&a).mod_power_of_2(2)).wrapping_into();
        if a != 0u32 && a_mod_4 != 3 {
            let abs_a = (&a).abs();
            if a_mod_4 == 2 {
                let four_abs_a = abs_a << 2u32;
                assert_eq!((&a).kronecker_symbol(&n + &four_abs_a), s);
                assert_eq!((&a).kronecker_symbol(&n - four_abs_a), s);
            } else {
                assert_eq!((&a).kronecker_symbol(&n + &abs_a), s);
                assert_eq!((&a).kronecker_symbol(&n - abs_a), s);
            }
        }

        let m = a;
        let m_odd = if m == 0u32 {
            Integer::ONE
        } else {
            &m >> m.trailing_zeros().unwrap()
        };
        let m_odd_mod_4: u8 = (&(&m_odd).mod_power_of_2(2)).wrapping_into();
        let m_star = if m_odd_mod_4 == 1 {
            Cow::Borrowed(&m)
        } else {
            Cow::Owned(-&m)
        };
        assert_eq!(
            m_star.as_ref().kronecker_symbol(&n),
            n.kronecker_symbol(m.abs())
        );
    });

    integer_triple_gen().test_properties(|(x, y, z)| {
        if !(z == -1i32 && (x == 0u32 && y < 0u32 || x < 0u32 && y == 0u32)) {
            assert_eq!(
                (&x * &y).kronecker_symbol(&z),
                (&x).kronecker_symbol(&z) * (&y).kronecker_symbol(&z)
            );
        }
        let y_odd_mod_4: u8 = if y == 0u32 {
            0
        } else {
            (&(&y >> y.trailing_zeros().unwrap()).mod_power_of_2(2)).wrapping_into()
        };
        let z_odd_mod_4: u8 = if z == 0u32 {
            0
        } else {
            (&(&z >> z.trailing_zeros().unwrap()).mod_power_of_2(2)).wrapping_into()
        };
        if !(x == -1i32 && (y == 0u32 && z_odd_mod_4 == 3 || y_odd_mod_4 == 3 && z == 0u32)) {
            assert_eq!(
                (&x).kronecker_symbol(&y * &z),
                (&x).kronecker_symbol(y) * x.kronecker_symbol(z)
            );
        }
    });

    integer_pair_gen_var_5().test_properties(|(m, n)| {
        let n_odd = if n == 0u32 {
            Integer::ONE
        } else {
            &n >> n.trailing_zeros().unwrap()
        };
        let m_odd = if m == 0u32 {
            Integer::ONE
        } else {
            &m >> m.trailing_zeros().unwrap()
        };
        let n_odd_mod_4: u8 = (&n_odd.mod_power_of_2(2)).wrapping_into();
        let m_odd_mod_4: u8 = (&m_odd.mod_power_of_2(2)).wrapping_into();
        let p = if n_odd_mod_4 == 1 || m_odd_mod_4 == 1 {
            1
        } else {
            -1
        };
        assert_eq!(
            (&m).kronecker_symbol(&n) * (&n).kronecker_symbol(&m),
            if m < 0u32 && n < 0u32 { -p } else { p }
        );
        assert_eq!((&m).kronecker_symbol(&n) * n.kronecker_symbol(m.abs()), p);
    });

    integer_gen().test_properties(|n| {
        if n != 1u32 && n != -1i32 {
            assert_eq!(Integer::ZERO.kronecker_symbol(&n), 0);
            assert_eq!((&n).kronecker_symbol(&n), 0);
        }
        assert_eq!(Integer::ONE.kronecker_symbol(&n), 1);
        assert_eq!((&n).kronecker_symbol(Integer::ONE), 1);
        let n_odd = if n == 0u32 {
            Integer::ONE
        } else {
            &n >> n.trailing_zeros().unwrap()
        };
        let n_odd_mod_4: u8 = (&n_odd.mod_power_of_2(2)).wrapping_into();
        assert_eq!(
            Integer::NEGATIVE_ONE.kronecker_symbol(&n),
            if n_odd_mod_4 == 1 { 1 } else { -1 }
        );
        assert_eq!(
            (&n).kronecker_symbol(Integer::NEGATIVE_ONE),
            if n >= 0u32 { 1 } else { -1 }
        );
    });

    signed_pair_gen::<SignedLimb>().test_properties(|(x, y)| {
        assert_eq!(
            Integer::from(x).kronecker_symbol(Integer::from(y)),
            x.kronecker_symbol(y)
        );
    });
}
