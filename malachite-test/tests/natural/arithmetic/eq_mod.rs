#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::arithmetic::eq_mod::{
    _limbs_eq_limb_mod_naive, _limbs_eq_mod_limb_naive, limbs_eq_limb_mod, limbs_eq_mod_limb,
};
use malachite_nz::natural::arithmetic::eq_mod::{_limbs_eq_mod_naive, limbs_eq_mod};
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

use common::test_properties;
use malachite_test::inputs::base::{
    triples_of_unsigned_vec_var_55, triples_of_unsigned_vec_var_56, triples_of_unsigned_vec_var_57,
};

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_eq_limb_mod() {
    let test = |xs: &[Limb], y: Limb, modulus: &[Limb], equal: bool| {
        assert_eq!(limbs_eq_limb_mod(xs, y, modulus), equal);
        assert_eq!(_limbs_eq_limb_mod_naive(xs, y, modulus), equal);
    };
    // m_len != 2 || m_0 == 0
    test(&[1, 1], 1, &[0, 1], true);
    // m_1 < 1 << m_trailing_zeros
    // x_len < BMOD_1_TO_MOD_1_THRESHOLD
    test(&[0, 1], 2, &[2, 1], false);
    // x_len >= BMOD_1_TO_MOD_1_THRESHOLD
    // y_0 < m_0
    test(&[6; 40], 2, &[2, 1], false);
    // y_0 >= m_0
    test(&[6; 40], 2147483650, &[2, 1], false);
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
        assert_eq!(_limbs_eq_mod_limb_naive(xs, ys, modulus), equal);
    };
    test(&[1, 1], &[3, 4], 5, true);
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
        assert_eq!(_limbs_eq_mod_naive(xs, ys, modulus), equal);
    };
    // limbs_cmp(xs, ys) == Ordering::Less
    test(&[1, 1, 1], &[1, 0, 3], &[0, 7], true);
    test(&[0, 1, 1], &[1, 0, 3], &[0, 7], false);
    // limbs_cmp(xs, ys) >= Ordering::Equal
    test(&[1, 3], &[1, 1, 2], &[0, 5], true);
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
fn limbs_eq_mod_properties() {
    test_properties(
        triples_of_unsigned_vec_var_55,
        |&(ref xs, ref ys, ref modulus)| {
            let equal = limbs_eq_mod(xs, ys, modulus);
            //TODO assert_eq!(
            //      Natural::from_limbs_asc(xs).eq_mod(
            //              Natural::from_limbs_asc(ys),
            //              Natural::from_limbs_asc(ys)
            //      ),
            //      equal
            // );
            assert_eq!(_limbs_eq_mod_naive(xs, ys, modulus), equal);
        },
    );

    test_properties(
        triples_of_unsigned_vec_var_56,
        |&(ref xs, ref ys, ref modulus)| {
            assert!(limbs_eq_mod(xs, ys, modulus));
            //TODO assert!(
            //      Natural::from_limbs_asc(xs).eq_mod(
            //              Natural::from_limbs_asc(ys),
            //              Natural::from_limbs_asc(ys)
            //      )
            // );
            assert!(_limbs_eq_mod_naive(xs, ys, modulus));
        },
    );

    test_properties(
        triples_of_unsigned_vec_var_57,
        |&(ref xs, ref ys, ref modulus)| {
            assert!(!limbs_eq_mod(xs, ys, modulus));
            //TODO assert!(
            //      !Natural::from_limbs_asc(xs).eq_mod(
            //              Natural::from_limbs_asc(ys),
            //              Natural::from_limbs_asc(ys)
            //      )
            // );
            assert!(!_limbs_eq_mod_naive(xs, ys, modulus));
        },
    );
}
