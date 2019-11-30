use malachite_nz::natural::arithmetic::eq_mod::{_limbs_eq_mod_naive, limbs_eq_mod};
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

use common::test_properties;
use malachite_test::inputs::base::{
    triples_of_unsigned_vec_var_55, triples_of_unsigned_vec_var_56, triples_of_unsigned_vec_var_57,
};

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_eq_mod() {
    let test = |xs: &[Limb], ys: &[Limb], modulus: &[Limb], equal: bool| {
        assert_eq!(limbs_eq_mod(xs, ys, modulus), equal);
        assert_eq!(_limbs_eq_mod_naive(xs, ys, modulus), equal);
    };
    // xs.len() >= ys.len()
    // y_len == 0 in limbs_eq_mod_greater
    test(&[6, 7], &[], &[2], true);
    test(&[5, 7], &[], &[2], false);
    // xs.len() < ys.len()
    test(&[], &[6, 7], &[2], true);
    test(&[], &[5, 7], &[2], false);
    // y_len > 0 in limbs_eq_mod_greater
    // xs != ys in limbs_eq_mod_greater
    // x_0.eq_mod_power_of_two(y_0, u64::from(m_trailing_zeros))
    // y_len == 1 in limbs_eq_mod_greater
    // m_len == 1 in limbs_eq_mod_greater
    // x_len < BMOD_1_TO_MOD_1_THRESHOLD first time; [7] [3] [2]
    test(&[7], &[3], &[2], true);
    // !x_0.eq_mod_power_of_two(y_0, u64::from(m_trailing_zeros)) in limbs_eq_mod_greater
    test(&[7], &[4], &[2], false);
    test(&[6, 7], &[4], &[2], true);
    test(&[7, 7], &[4], &[2], false);
    test(&[6, 7], &[3], &[2], false);
    test(&[7, 7], &[3], &[2], true);
    test(&[2, 2], &[7], &[13], true);
    test(&[100, 101, 102], &[1_238], &[10], true);
    test(&[100, 101, 102], &[1_239], &[10], false);
    test(&[123, 456], &[636], &[789], true);
    test(&[123, 456], &[1_000], &[789], false);
    // y_len > 1 in limbs_eq_mod_greater
    // limbs_cmp(xs, ys) == Ordering::Less in limbs_eq_mod_greater
    test(&[1, 1, 1], &[1, 0, 3], &[0, 7], true);
    test(&[0, 1, 1], &[1, 0, 3], &[0, 7], false);
    // limbs_cmp(xs, ys) >= Ordering::Equal in limbs_eq_mod_greater
    test(&[1, 3], &[1, 1, 2], &[0, 5], true);
    // xs == ys in limbs_eq_mod_greater
    test(&[1], &[1], &[0, 1], true);
    test(&[], &[0, 1], &[0, 1], true);
    test(&[1], &[3], &[1], true);
    // m_len > 1 in limbs_eq_mod_greater
    // m_len == 2 && m_0 != 0 in limbs_eq_mod_greater
    // m_1 >= 1 << m_trailing_zeros in limbs_eq_mod_greater
    test(&[1], &[2], &[1, 1], false);
    // m_len != 2 || m_0 == 0 in limbs_eq_mod_greater
    test(&[1, 1], &[1], &[0, 1], true);
    // m_1 < 1 << m_trailing_zeros in limbs_eq_mod_greater
    // x_len < BMOD_1_TO_MOD_1_THRESHOLD second time in limbs_eq_mod_greater
    test(&[0, 1], &[2], &[2, 1], false);
    // x_len >= BMOD_1_TO_MOD_1_THRESHOLD first time
    // y_0 >= m_0 first time
    test(
        &[
            957355272, 2717966866, 2284391330, 238149753, 3607703304, 23463007, 1388955612,
            3269479240, 881285075, 2493741919, 360635652, 2851492229, 3590429614, 2528168680,
            215334077, 3509222230, 1825157855, 3737409852, 4151389929, 2692167062, 1409227805,
            2060445344, 1453537438, 3186146035, 1159656442, 954576963, 2935313630, 2288694644,
            400433986, 3182217800, 3929694465, 3346806449, 131165877,
        ],
        &[1529684314],
        &[1469269654],
        false,
    );
    // y_0 < m_0 first time
    test(
        &[
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            511, 0, 0, 0, 4227858432, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            511, 0, 0, 0, 3221225472, 63, 0, 0, 0, 0, 0, 0, 0, 4294443008, 4294967295, 4294967295,
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295,
        ],
        &[4294963200],
        &[4294967295],
        false,
    );
    // x_len >= BMOD_1_TO_MOD_1_THRESHOLD second time in limbs_eq_mod_greater
    // y_0 < m_0 second time in limbs_eq_mod_greater
    test(&[6; 40], &[2], &[2, 1], false);
    // y_0 >= m_0 second time in limbs_eq_mod_greater
    test(&[6; 40], &[2147483650], &[2, 1], false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_fail_1() {
    limbs_eq_mod(&[1, 3], &[1, 1, 2], &[]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_fail_2() {
    limbs_eq_mod(&[], &[], &[0, 5]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_fail_3() {
    limbs_eq_mod(&[1, 0], &[1, 1, 2], &[0, 5]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_fail_4() {
    limbs_eq_mod(&[1, 3], &[1, 1, 0], &[0, 5]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_fail_5() {
    limbs_eq_mod(&[1, 3], &[1, 1, 2], &[5, 0]);
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
