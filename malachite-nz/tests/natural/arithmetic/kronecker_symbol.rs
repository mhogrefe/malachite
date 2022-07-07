use malachite_base::num::arithmetic::traits::{
    CoprimeWith, JacobiSymbol, KroneckerSymbol, LegendreSymbol, ModPowerOf2,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::WrappingInto;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{unsigned_pair_gen_var_27, unsigned_pair_gen_var_40};
use malachite_nz::natural::arithmetic::kronecker_symbol::jacobi_symbol_simple;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_gen, natural_gen_var_8, natural_pair_gen, natural_pair_gen_var_12,
    natural_pair_gen_var_13, natural_pair_gen_var_14, natural_pair_gen_var_4, natural_triple_gen,
    natural_triple_gen_var_8, natural_triple_gen_var_9,
};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_jacobi_symbol() {
    fn test(u: &str, v: &str, s: i8) {
        let a = Natural::from_str(u).unwrap();
        let n = Natural::from_str(v).unwrap();

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

        assert_eq!(jacobi_symbol_simple(a, n), s);
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

    test("1001", "9907", -1);
    test("10908", "9907", -1);
}

#[test]
fn jacobi_symbol_fail() {
    assert_panic!(Natural::ONE.jacobi_symbol(Natural::TWO));
}

// Odd n is already tested in test_jacobi_symbol, so here we just test even n
#[test]
fn test_kronecker_symbol() {
    fn test(u: &str, v: &str, s: i8) {
        let a = Natural::from_str(u).unwrap();
        let n = Natural::from_str(v).unwrap();

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

    test("1001", "9908", -1);
    test("10909", "9908", -1);
}

#[test]
fn jacobi_symbol_properties() {
    natural_pair_gen_var_12().test_properties(|(a, n)| {
        let s = (&a).jacobi_symbol(&n);
        assert_eq!((&a).jacobi_symbol(n.clone()), s);
        assert_eq!(a.clone().jacobi_symbol(&n), s);
        assert_eq!(a.clone().jacobi_symbol(n.clone()), s);

        assert_eq!((&a).legendre_symbol(&n), s);
        assert_eq!((&a).legendre_symbol(n.clone()), s);
        assert_eq!(a.clone().legendre_symbol(&n), s);
        assert_eq!(a.clone().legendre_symbol(n.clone()), s);

        assert_eq!((&a).kronecker_symbol(&n), s);
        assert_eq!((&a).kronecker_symbol(n.clone()), s);
        assert_eq!(a.clone().kronecker_symbol(&n), s);
        assert_eq!(a.clone().kronecker_symbol(n.clone()), s);

        assert_eq!(jacobi_symbol_simple(a.clone(), n.clone()), s);
        assert!(s.le_abs(&1i8));

        assert_eq!((&a + &n).jacobi_symbol(&n), s);
        if a >= n {
            assert_eq!((&a - &n).jacobi_symbol(&n), s);
        }
        assert_eq!(s != 0, (&a).coprime_with(&n));
        let n_mod_8: u8 = (&(&n).mod_power_of_2(3)).wrapping_into();
        assert_eq!(
            (a << 1u32).jacobi_symbol(n),
            if n_mod_8 == 1 || n_mod_8 == 7 { s } else { -s }
        );
    });

    natural_pair_gen_var_13().test_properties(|(m, n)| {
        let n_mod_4: u8 = (&(&n).mod_power_of_2(2)).wrapping_into();
        let m_mod_4: u8 = (&(&m).mod_power_of_2(2)).wrapping_into();
        assert_eq!(
            (&m).jacobi_symbol(&n) * n.jacobi_symbol(m),
            if n_mod_4 == 1 || m_mod_4 == 1 { 1 } else { -1 }
        );
    });

    natural_triple_gen_var_8().test_properties(|(a, b, n)| {
        assert_eq!(
            (&a * &b).jacobi_symbol(&n),
            a.jacobi_symbol(&n) * b.jacobi_symbol(n)
        );
    });

    natural_triple_gen_var_9().test_properties(|(a, m, n)| {
        assert_eq!(
            (&a).jacobi_symbol(&m * &n),
            (&a).jacobi_symbol(m) * a.jacobi_symbol(n)
        );
    });

    natural_gen_var_8().test_properties(|n| {
        if n != 1u32 {
            assert_eq!(Natural::ZERO.jacobi_symbol(&n), 0);
            assert_eq!((&n).jacobi_symbol(&n), 0);
        }
        assert_eq!(Natural::ONE.jacobi_symbol(&n), 1);
        assert_eq!((&n).jacobi_symbol(Natural::ONE), 1);
        let n_mod_4: u8 = (&(&n).mod_power_of_2(2)).wrapping_into();
        assert_eq!(
            (&n - Natural::ONE).jacobi_symbol(&n),
            if n_mod_4 == 1 { 1 } else { -1 }
        );
        let n_mod_8: u8 = (&(&n).mod_power_of_2(3)).wrapping_into();
        assert_eq!(
            Natural::TWO.jacobi_symbol(&n),
            if n_mod_8 == 1 || n_mod_8 == 7 { 1 } else { -1 }
        );
    });

    unsigned_pair_gen_var_40::<Limb>().test_properties(|(x, y)| {
        assert_eq!(
            Natural::from(x).jacobi_symbol(Natural::from(y)),
            x.jacobi_symbol(y)
        );
    });
}

fn kronecker_symbol_properties_helper(a: Natural, n: Natural) {
    let s = (&a).kronecker_symbol(&n);
    assert!(s.le_abs(&1i8));

    assert_eq!(s != 0, (&a).coprime_with(&n));
    let n_mod_4: u8 = (&(&n).mod_power_of_2(2)).wrapping_into();
    if n_mod_4 == 2 {
        let four_n = &n << 2u32;
        assert_eq!((&a + &four_n).kronecker_symbol(&n), s);
        if a >= four_n {
            assert_eq!((&a - four_n).kronecker_symbol(&n), s);
        }
    } else {
        assert_eq!((&a + &n).kronecker_symbol(&n), s);
        if a >= n {
            assert_eq!((&a - &n).kronecker_symbol(&n), s);
        }
    }
    let a_mod_4: u8 = (&(&a).mod_power_of_2(2)).wrapping_into();
    if a != 0u32 && a_mod_4 != 3 {
        if a_mod_4 == 2 {
            let four_a = &a << 2u32;
            assert_eq!((&a).kronecker_symbol(&n + &four_a), s);
            if n >= four_a {
                assert_eq!((&a).kronecker_symbol(&n - four_a), s);
            }
        } else {
            assert_eq!((&a).kronecker_symbol(&n + &a), s);
            if n >= a {
                let diff = n - &a;
                assert_eq!((a).kronecker_symbol(diff), s);
            }
        }
    }
}

#[test]
fn kronecker_symbol_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_bits_n", 2048);
    config.insert("mean_stripe_n", 512 << Limb::LOG_WIDTH);
    natural_pair_gen().test_properties_with_config(&config, |(x, y)| {
        kronecker_symbol_properties_helper(x, y);
    });

    natural_pair_gen_var_4().test_properties_with_config(&config, |(x, y)| {
        kronecker_symbol_properties_helper(x, y);
    });

    natural_triple_gen().test_properties(|(x, y, z)| {
        assert_eq!(
            (&x * &y).kronecker_symbol(&z),
            (&x).kronecker_symbol(&z) * (&y).kronecker_symbol(&z)
        );
        assert_eq!(
            (&x).kronecker_symbol(&y * &z),
            (&x).kronecker_symbol(y) * x.kronecker_symbol(z)
        );
    });

    natural_pair_gen_var_14().test_properties(|(m, n)| {
        let n_odd = if n == 0u32 {
            Natural::ONE
        } else {
            &n >> n.trailing_zeros().unwrap()
        };
        let m_odd = if m == 0u32 {
            Natural::ONE
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
        assert_eq!((&m).kronecker_symbol(&n) * n.kronecker_symbol(m), p);
    });

    natural_gen().test_properties(|n| {
        if n != 1u32 {
            assert_eq!(Natural::ZERO.kronecker_symbol(&n), 0);
            assert_eq!((&n).kronecker_symbol(&n), 0);
        }
        assert_eq!(Natural::ONE.kronecker_symbol(&n), 1);
        assert_eq!(n.kronecker_symbol(Natural::ONE), 1);
    });

    unsigned_pair_gen_var_27::<Limb>().test_properties(|(x, y)| {
        assert_eq!(
            Natural::from(x).kronecker_symbol(Natural::from(y)),
            x.kronecker_symbol(y)
        );
    });
}
