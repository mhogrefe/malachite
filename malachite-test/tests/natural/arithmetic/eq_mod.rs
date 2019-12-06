use malachite_base::num::arithmetic::traits::EqMod;
use malachite_nz::natural::arithmetic::eq_mod::{
    _limbs_eq_limb_mod_naive_1, _limbs_eq_limb_mod_naive_2, _limbs_eq_mod_limb_naive_1,
    _limbs_eq_mod_limb_naive_2, _limbs_eq_mod_naive_1, _limbs_eq_mod_naive_2, limbs_eq_limb_mod,
    limbs_eq_mod, limbs_eq_mod_limb,
};
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

use common::test_properties;
use malachite_test::inputs::base::{
    triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1,
    triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_2,
    triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_3,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_10,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_9, triples_of_unsigned_vec_var_55,
    triples_of_unsigned_vec_var_56, triples_of_unsigned_vec_var_57,
};

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_eq_limb_mod() {
    let test = |xs: &[Limb], y: Limb, modulus: &[Limb], equal: bool| {
        assert_eq!(limbs_eq_limb_mod(xs, y, modulus), equal);
        assert_eq!(_limbs_eq_limb_mod_naive_1(xs, y, modulus), equal);
        assert_eq!(_limbs_eq_limb_mod_naive_2(xs, y, modulus), equal);
    };
    // xs[0].eq_mod_power_of_two(y, u64::from(m_trailing_zeros))
    // m_len != 2 || m_0 == 0
    test(&[1, 1], 1, &[0, 1], true);
    // m_len == 2 && m_0 != 0
    // m_1 < 1 << m_trailing_zeros
    // x_len < BMOD_1_TO_MOD_1_THRESHOLD
    test(&[0, 1], 2, &[2, 1], false);
    // x_len >= BMOD_1_TO_MOD_1_THRESHOLD
    // y_0 < m_0
    test(&[6; 40], 2, &[2, 1], false);
    // y_0 >= m_0
    test(&[6; 40], 2147483650, &[2, 1], false);
    // !xs[0].eq_mod_power_of_two(y, u64::from(m_trailing_zeros))
    test(&[0, 1], 1, &[0, 1], false);
    // m_1 >= 1 << m_trailing_zeros
    test(&[0, 1], 1, &[1, 1], false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_limb_mod_fail_1() {
    limbs_eq_limb_mod(&[1], 1, &[0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_limb_mod_fail_2() {
    limbs_eq_limb_mod(&[1, 1], 1, &[1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_limb_mod_fail_3() {
    limbs_eq_limb_mod(&[1, 0], 1, &[0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_limb_mod_fail_4() {
    limbs_eq_limb_mod(&[1, 1], 0, &[0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_limb_mod_fail_5() {
    limbs_eq_limb_mod(&[1, 1], 1, &[1, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_eq_mod_limb() {
    let test = |xs: &[Limb], ys: &[Limb], modulus: Limb, equal: bool| {
        assert_eq!(limbs_eq_mod_limb(xs, ys, modulus), equal);
        assert_eq!(_limbs_eq_mod_limb_naive_1(xs, ys, modulus), equal);
        assert_eq!(_limbs_eq_mod_limb_naive_2(xs, ys, modulus), equal);
    };
    // xs != ys in limbs_eq_mod_limb_greater
    // xs[0].eq_mod_power_of_two(ys[0], u64::from(modulus.trailing_zeros()))
    //      in limbs_eq_mod_limb_greater
    // limbs_cmp(xs, ys) < Ordering::Equal in limbs_eq_mod_limb_greater
    // scratch.len() > 1 in limbs_eq_mod_limb_greater
    test(&[1, 1], &[3, 4], 5, true);
    // xs == ys in limbs_eq_mod_limb_greater
    test(&[0, 1], &[0, 1], 1, true);
    // limbs_cmp(xs, ys) >= Ordering::Equal in limbs_eq_mod_limb_greater
    test(&[0, 0, 1], &[0, 1], 1, true);
    // scratch.len() == 1 in limbs_eq_mod_limb_greater
    test(&[0, 1], &[1, 1], 1, true);
    // !xs[0].eq_mod_power_of_two(ys[0], u64::from(modulus.trailing_zeros()))
    //      in limbs_eq_mod_limb_greater
    test(&[0, 1], &[1, 1], 2, false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_limb_fail_1() {
    limbs_eq_mod_limb(&[1], &[3, 4], 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_limb_fail_2() {
    limbs_eq_mod_limb(&[1, 1], &[4], 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_limb_fail_3() {
    limbs_eq_mod_limb(&[1, 0], &[3, 4], 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_limb_fail_4() {
    limbs_eq_mod_limb(&[1, 1], &[3, 0], 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_limb_fail_5() {
    limbs_eq_mod_limb(&[1, 1], &[3, 4], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_eq_mod() {
    let test = |xs: &[Limb], ys: &[Limb], modulus: &[Limb], equal: bool| {
        assert_eq!(limbs_eq_mod(xs, ys, modulus), equal);
        assert_eq!(_limbs_eq_mod_naive_1(xs, ys, modulus), equal);
        assert_eq!(_limbs_eq_mod_naive_2(xs, ys, modulus), equal);
    };
    // xs != ys in limbs_eq_mod_greater
    // xs[0].eq_mod_power_of_two(ys[0], u64::from(modulus[0].trailing_zeros()))
    //      in limbs_eq_mod_greater
    // limbs_cmp(xs, ys) == Ordering::Less
    test(&[1, 1, 1], &[1, 0, 3], &[0, 7], true);
    // !xs[0].eq_mod_power_of_two(ys[0], u64::from(modulus[0].trailing_zeros()))
    //      in limbs_eq_mod_greater
    test(&[0, 1, 1], &[1, 0, 3], &[0, 7], false);
    // limbs_cmp(xs, ys) >= Ordering::Equal
    test(&[1, 3], &[1, 1, 2], &[0, 5], true);
    // xs == ys in limbs_eq_mod_greater
    test(&[0, 1], &[0, 1], &[0, 1], true);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_fail_1() {
    limbs_eq_mod(&[1], &[1, 0, 3], &[0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_fail_2() {
    limbs_eq_mod(&[1, 1, 1], &[1], &[0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_fail_3() {
    limbs_eq_mod(&[1, 1, 1], &[1, 0, 3], &[7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_fail_4() {
    limbs_eq_mod(&[1, 1, 0], &[1, 0, 3], &[0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_fail_5() {
    limbs_eq_mod(&[1, 1, 1], &[1, 0, 0], &[0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_fail_6() {
    limbs_eq_mod(&[1, 1, 1], &[1, 0, 3], &[7, 0]);
}

#[test]
fn limbs_eq_limb_mod_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1,
        |&(ref xs, y, ref modulus)| {
            let equal = limbs_eq_limb_mod(xs, y, modulus);
            assert_eq!(
                Natural::from_limbs_asc(xs)
                    .eq_mod(Natural::from(y), Natural::from_limbs_asc(modulus)),
                equal
            );
            assert_eq!(_limbs_eq_limb_mod_naive_1(xs, y, modulus), equal);
            assert_eq!(_limbs_eq_limb_mod_naive_2(xs, y, modulus), equal);
        },
    );

    test_properties(
        triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_2,
        |&(ref xs, y, ref modulus)| {
            assert!(Natural::from_limbs_asc(xs)
                .eq_mod(Natural::from(y), Natural::from_limbs_asc(modulus)));
            assert!(limbs_eq_limb_mod(xs, y, modulus));
            assert!(_limbs_eq_limb_mod_naive_1(xs, y, modulus));
            assert!(_limbs_eq_limb_mod_naive_2(xs, y, modulus));
        },
    );

    test_properties(
        triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_3,
        |&(ref xs, y, ref modulus)| {
            assert!(!Natural::from_limbs_asc(xs)
                .eq_mod(Natural::from(y), Natural::from_limbs_asc(modulus)));
            assert!(!limbs_eq_limb_mod(xs, y, modulus));
            assert!(!_limbs_eq_limb_mod_naive_1(xs, y, modulus));
            assert!(!_limbs_eq_limb_mod_naive_2(xs, y, modulus));
        },
    );
}

#[test]
fn limbs_eq_mod_limb_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8,
        |&(ref xs, ref ys, modulus)| {
            let equal = limbs_eq_mod_limb(xs, ys, modulus);
            assert_eq!(
                Natural::from_limbs_asc(xs)
                    .eq_mod(Natural::from_limbs_asc(ys), Natural::from(modulus)),
                equal
            );
            assert_eq!(_limbs_eq_mod_limb_naive_1(xs, ys, modulus), equal);
            assert_eq!(_limbs_eq_mod_limb_naive_2(xs, ys, modulus), equal);
        },
    );

    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_9,
        |&(ref xs, ref ys, modulus)| {
            assert!(Natural::from_limbs_asc(xs)
                .eq_mod(Natural::from_limbs_asc(ys), Natural::from(modulus)));
            assert!(limbs_eq_mod_limb(xs, ys, modulus));
            assert!(_limbs_eq_mod_limb_naive_1(xs, ys, modulus));
            assert!(_limbs_eq_mod_limb_naive_2(xs, ys, modulus));
        },
    );

    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_10,
        |&(ref xs, ref ys, modulus)| {
            assert!(!Natural::from_limbs_asc(xs)
                .eq_mod(Natural::from_limbs_asc(ys), Natural::from(modulus)));
            assert!(!limbs_eq_mod_limb(xs, ys, modulus));
            assert!(!_limbs_eq_mod_limb_naive_1(xs, ys, modulus));
            assert!(!_limbs_eq_mod_limb_naive_2(xs, ys, modulus));
        },
    );
}

#[test]
fn limbs_eq_mod_properties() {
    test_properties(
        triples_of_unsigned_vec_var_55,
        |&(ref xs, ref ys, ref modulus)| {
            let equal = limbs_eq_mod(xs, ys, modulus);
            assert_eq!(
                Natural::from_limbs_asc(xs).eq_mod(
                    Natural::from_limbs_asc(ys),
                    Natural::from_limbs_asc(modulus)
                ),
                equal
            );
            assert_eq!(_limbs_eq_mod_naive_1(xs, ys, modulus), equal);
            assert_eq!(_limbs_eq_mod_naive_2(xs, ys, modulus), equal);
        },
    );

    test_properties(
        triples_of_unsigned_vec_var_56,
        |&(ref xs, ref ys, ref modulus)| {
            assert!(Natural::from_limbs_asc(xs).eq_mod(
                Natural::from_limbs_asc(ys),
                Natural::from_limbs_asc(modulus)
            ));
            assert!(limbs_eq_mod(xs, ys, modulus));
            assert!(_limbs_eq_mod_naive_1(xs, ys, modulus));
            assert!(_limbs_eq_mod_naive_2(xs, ys, modulus));
        },
    );

    test_properties(
        triples_of_unsigned_vec_var_57,
        |&(ref xs, ref ys, ref modulus)| {
            assert!(!Natural::from_limbs_asc(xs).eq_mod(
                Natural::from_limbs_asc(ys),
                Natural::from_limbs_asc(modulus)
            ));
            assert!(!limbs_eq_mod(xs, ys, modulus));
            assert!(!_limbs_eq_mod_naive_1(xs, ys, modulus));
            assert!(!_limbs_eq_mod_naive_2(xs, ys, modulus));
        },
    );
}
