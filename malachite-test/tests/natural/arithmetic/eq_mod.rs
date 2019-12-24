use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{DivisibleBy, EqMod};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::eq_mod::{
    _combined_limbs_eq_limb_mod_limb, _limbs_eq_limb_mod_naive_1, _limbs_eq_limb_mod_naive_2,
    _limbs_eq_mod_limb_naive_1, _limbs_eq_mod_limb_naive_2, _limbs_eq_mod_naive_1,
    _limbs_eq_mod_naive_2, limbs_eq_limb_mod, limbs_eq_limb_mod_limb, limbs_eq_limb_mod_ref_ref,
    limbs_eq_limb_mod_ref_val, limbs_eq_limb_mod_val_ref, limbs_eq_mod_limb_ref_ref,
    limbs_eq_mod_limb_ref_val, limbs_eq_mod_limb_val_ref, limbs_eq_mod_ref_ref_ref,
    limbs_eq_mod_ref_ref_val, limbs_eq_mod_ref_val_ref, limbs_eq_mod_ref_val_val,
};
use malachite_nz::natural::arithmetic::mod_op::limbs_mod_limb;
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

use malachite_test::common::natural_to_rug_integer;
use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    triples_of_limb_vec_limb_and_positive_limb_var_3,
    triples_of_limb_vec_limb_and_positive_limb_var_4,
    triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1,
    triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1,
    triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_2,
    triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_3,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_10,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_9, triples_of_unsigned_vec_var_55,
    triples_of_unsigned_vec_var_56, triples_of_unsigned_vec_var_57,
};
use malachite_test::inputs::natural::{
    pairs_of_naturals, triples_of_naturals, triples_of_naturals_var_2, triples_of_naturals_var_3,
};

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_eq_limb_mod_limb() {
    let test = |limbs: &[Limb], limb: Limb, modulus: Limb, equal: bool| {
        assert_eq!(limbs_eq_limb_mod_limb(limbs, limb, modulus), equal);
        assert_eq!(limbs_mod_limb(limbs, modulus) == limb % modulus, equal);
        assert_eq!(
            _combined_limbs_eq_limb_mod_limb(limbs, limb, modulus),
            equal
        );
    };
    test(&[6, 7], 4, 2, true);
    test(&[7, 7], 4, 2, false);
    test(&[6, 7], 3, 2, false);
    test(&[7, 7], 3, 2, true);
    test(&[2, 2], 7, 13, true);
    test(&[100, 101, 102], 1_238, 10, true);
    test(&[100, 101, 102], 1_239, 10, false);
    test(&[123, 456], 636, 789, true);
    test(&[123, 456], 1_000, 789, false);
    test(&[0xffff_ffff, 0xffff_ffff], 101, 2, true);
    test(&[0xffff_ffff, 0xffff_ffff], 100, 2, false);
    test(&[0xffff_ffff, 0xffff_ffff], 120, 3, true);
    test(&[0xffff_ffff, 0xffff_ffff], 110, 3, false);
    test(
        &[
            957355272, 2717966866, 2284391330, 238149753, 3607703304, 23463007, 1388955612,
            3269479240, 881285075, 2493741919, 360635652, 2851492229, 3590429614, 2528168680,
            215334077, 3509222230, 1825157855, 3737409852, 4151389929, 2692167062, 1409227805,
            2060445344, 1453537438, 3186146035, 1159656442, 954576963, 2935313630, 2288694644,
            400433986, 3182217800, 3929694465, 3346806449, 131165877,
        ],
        1529684314,
        1469269654,
        false,
    );
    test(
        &[
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            511, 0, 0, 0, 4227858432, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            511, 0, 0, 0, 3221225472, 63, 0, 0, 0, 0, 0, 0, 0, 4294443008, 4294967295, 4294967295,
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295,
        ],
        4294963200,
        4294967295,
        false,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_limb_mod_limb_fail_1() {
    limbs_eq_limb_mod_limb(&[10], 10, 15);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_limb_mod_limb_fail_2() {
    limbs_eq_limb_mod_limb(&[6, 7], 4, 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_eq_limb_mod() {
    let test = |xs: &[Limb], y: Limb, modulus: &[Limb], equal: bool| {
        let mut mut_xs = xs.to_vec();
        let mut mut_modulus = modulus.to_vec();
        assert_eq!(limbs_eq_limb_mod(&mut mut_xs, y, &mut mut_modulus), equal);
        let mut mut_xs = xs.to_vec();
        assert_eq!(limbs_eq_limb_mod_val_ref(&mut mut_xs, y, modulus), equal);
        let mut mut_modulus = modulus.to_vec();
        assert_eq!(limbs_eq_limb_mod_ref_val(xs, y, &mut mut_modulus), equal);
        assert_eq!(limbs_eq_limb_mod_ref_ref(xs, y, modulus), equal);
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
    limbs_eq_limb_mod(&mut [1], 1, &mut [0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_limb_mod_fail_2() {
    limbs_eq_limb_mod(&mut [1, 1], 1, &mut [1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_limb_mod_fail_3() {
    limbs_eq_limb_mod(&mut [1, 0], 1, &mut [0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_limb_mod_fail_4() {
    limbs_eq_limb_mod(&mut [1, 1], 0, &mut [0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_limb_mod_fail_5() {
    limbs_eq_limb_mod(&mut [1, 1], 1, &mut [1, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_limb_mod_val_ref_fail_1() {
    limbs_eq_limb_mod_val_ref(&mut [1], 1, &[0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_limb_mod_val_ref_fail_2() {
    limbs_eq_limb_mod_val_ref(&mut [1, 1], 1, &[1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_limb_mod_val_ref_fail_3() {
    limbs_eq_limb_mod_val_ref(&mut [1, 0], 1, &[0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_limb_mod_val_ref_fail_4() {
    limbs_eq_limb_mod_val_ref(&mut [1, 1], 0, &[0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_limb_mod_val_ref_fail_5() {
    limbs_eq_limb_mod_val_ref(&mut [1, 1], 1, &[1, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_limb_mod_ref_val_fail_1() {
    limbs_eq_limb_mod_ref_val(&[1], 1, &mut [0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_limb_mod_ref_val_fail_2() {
    limbs_eq_limb_mod_ref_val(&[1, 1], 1, &mut [1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_limb_mod_ref_val_fail_3() {
    limbs_eq_limb_mod_ref_val(&[1, 0], 1, &mut [0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_limb_mod_ref_val_fail_4() {
    limbs_eq_limb_mod_ref_val(&[1, 1], 0, &mut [0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_limb_mod_ref_val_fail_5() {
    limbs_eq_limb_mod_ref_val(&[1, 1], 1, &mut [1, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_limb_mod_ref_ref_fail_1() {
    limbs_eq_limb_mod_ref_ref(&[1], 1, &[0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_limb_mod_ref_ref_fail_2() {
    limbs_eq_limb_mod_ref_ref(&[1, 1], 1, &[1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_limb_mod_ref_ref_fail_3() {
    limbs_eq_limb_mod_ref_ref(&[1, 0], 1, &[0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_limb_mod_ref_ref_fail_4() {
    limbs_eq_limb_mod_ref_ref(&[1, 1], 0, &[0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_limb_mod_ref_ref_fail_5() {
    limbs_eq_limb_mod_ref_ref(&[1, 1], 1, &[1, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_eq_mod_limb() {
    let test = |xs: &[Limb], ys: &[Limb], modulus: Limb, equal: bool| {
        let mut mut_xs = xs.to_vec();
        assert_eq!(limbs_eq_mod_limb_val_ref(&mut mut_xs, ys, modulus), equal);
        let mut mut_ys = ys.to_vec();
        assert_eq!(limbs_eq_mod_limb_ref_val(xs, &mut mut_ys, modulus), equal);
        assert_eq!(limbs_eq_mod_limb_ref_ref(xs, ys, modulus), equal);
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
fn limbs_eq_mod_limb_val_ref_fail_1() {
    limbs_eq_mod_limb_val_ref(&mut [1], &[3, 4], 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_limb_val_ref_fail_2() {
    limbs_eq_mod_limb_val_ref(&mut [1, 1], &[4], 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_limb_val_ref_fail_3() {
    limbs_eq_mod_limb_val_ref(&mut [1, 0], &[3, 4], 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_limb_val_ref_fail_4() {
    limbs_eq_mod_limb_val_ref(&mut [1, 1], &[3, 0], 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_limb_val_ref_fail_5() {
    limbs_eq_mod_limb_val_ref(&mut [1, 1], &[3, 4], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_limb_ref_val_fail_1() {
    limbs_eq_mod_limb_ref_val(&[1], &mut [3, 4], 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_limb_ref_val_fail_2() {
    limbs_eq_mod_limb_ref_val(&[1, 1], &mut [4], 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_limb_ref_val_fail_3() {
    limbs_eq_mod_limb_ref_val(&[1, 0], &mut [3, 4], 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_limb_ref_val_fail_4() {
    limbs_eq_mod_limb_ref_val(&[1, 1], &mut [3, 0], 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_limb_ref_val_fail_5() {
    limbs_eq_mod_limb_ref_val(&[1, 1], &mut [3, 4], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_limb_ref_ref_fail_1() {
    limbs_eq_mod_limb_ref_ref(&[1], &[3, 4], 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_limb_ref_ref_fail_2() {
    limbs_eq_mod_limb_ref_ref(&[1, 1], &[4], 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_limb_ref_ref_fail_3() {
    limbs_eq_mod_limb_ref_ref(&[1, 0], &[3, 4], 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_limb_ref_ref_fail_4() {
    limbs_eq_mod_limb_ref_ref(&[1, 1], &[3, 0], 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_limb_ref_ref_fail_5() {
    limbs_eq_mod_limb_ref_ref(&[1, 1], &[3, 4], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_eq_mod_ref_ref_ref() {
    let test = |xs: &[Limb], ys: &[Limb], modulus: &[Limb], equal: bool| {
        let mut mut_ys = ys.to_vec();
        let mut mut_modulus = modulus.to_vec();
        assert_eq!(
            limbs_eq_mod_ref_val_val(xs, &mut mut_ys, &mut mut_modulus),
            equal
        );
        let mut mut_ys = ys.to_vec();
        assert_eq!(limbs_eq_mod_ref_val_ref(xs, &mut mut_ys, modulus), equal);
        let mut mut_modulus = modulus.to_vec();
        assert_eq!(limbs_eq_mod_ref_ref_val(xs, ys, &mut mut_modulus), equal);
        assert_eq!(limbs_eq_mod_ref_ref_ref(xs, ys, modulus), equal);
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
fn limbs_eq_mod_ref_val_val_fail_1() {
    limbs_eq_mod_ref_val_val(&[1], &mut [1, 0, 3], &mut [0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_ref_val_val_fail_2() {
    limbs_eq_mod_ref_val_val(&[1, 1, 1], &mut [1], &mut [0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_ref_val_val_fail_3() {
    limbs_eq_mod_ref_val_val(&[1, 1, 1], &mut [1, 0, 3], &mut [7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_ref_val_val_fail_4() {
    limbs_eq_mod_ref_val_val(&[1, 1, 0], &mut [1, 0, 3], &mut [0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_ref_val_val_fail_5() {
    limbs_eq_mod_ref_val_val(&[1, 1, 1], &mut [1, 0, 0], &mut [0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_ref_val_val_fail_6() {
    limbs_eq_mod_ref_val_val(&[1, 1, 1], &mut [1, 0, 3], &mut [7, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_ref_val_ref_fail_1() {
    limbs_eq_mod_ref_val_ref(&[1], &mut [1, 0, 3], &[0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_ref_val_ref_fail_2() {
    limbs_eq_mod_ref_val_ref(&[1, 1, 1], &mut [1], &[0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_ref_val_ref_fail_3() {
    limbs_eq_mod_ref_val_ref(&[1, 1, 1], &mut [1, 0, 3], &[7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_ref_val_ref_fail_4() {
    limbs_eq_mod_ref_val_ref(&[1, 1, 0], &mut [1, 0, 3], &[0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_ref_val_ref_fail_5() {
    limbs_eq_mod_ref_val_ref(&[1, 1, 1], &mut [1, 0, 0], &[0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_ref_val_ref_fail_6() {
    limbs_eq_mod_ref_val_ref(&[1, 1, 1], &mut [1, 0, 3], &[7, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_ref_ref_val_fail_1() {
    limbs_eq_mod_ref_ref_val(&[1], &[1, 0, 3], &mut [0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_ref_ref_val_fail_2() {
    limbs_eq_mod_ref_ref_val(&[1, 1, 1], &[1], &mut [0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_ref_ref_val_fail_3() {
    limbs_eq_mod_ref_ref_val(&[1, 1, 1], &[1, 0, 3], &mut [7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_ref_ref_val_fail_4() {
    limbs_eq_mod_ref_ref_val(&[1, 1, 0], &[1, 0, 3], &mut [0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_ref_ref_val_fail_5() {
    limbs_eq_mod_ref_ref_val(&[1, 1, 1], &[1, 0, 0], &mut [0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_ref_ref_val_fail_6() {
    limbs_eq_mod_ref_ref_val(&[1, 1, 1], &[1, 0, 3], &mut [7, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_ref_ref_ref_fail_1() {
    limbs_eq_mod_ref_ref_ref(&[1], &[1, 0, 3], &[0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_ref_ref_ref_fail_2() {
    limbs_eq_mod_ref_ref_ref(&[1, 1, 1], &[1], &[0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_ref_ref_ref_fail_3() {
    limbs_eq_mod_ref_ref_ref(&[1, 1, 1], &[1, 0, 3], &[7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_ref_ref_ref_fail_4() {
    limbs_eq_mod_ref_ref_ref(&[1, 1, 0], &[1, 0, 3], &[0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_ref_ref_ref_fail_5() {
    limbs_eq_mod_ref_ref_ref(&[1, 1, 1], &[1, 0, 0], &[0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_mod_ref_ref_ref_fail_6() {
    limbs_eq_mod_ref_ref_ref(&[1, 1, 1], &[1, 0, 3], &[7, 0]);
}

#[test]
fn test_eq_mod() {
    let test = |x, y, modulus, out| {
        assert_eq!(
            Natural::from_str(x).unwrap().eq_mod(
                Natural::from_str(y).unwrap(),
                Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            Natural::from_str(x).unwrap().eq_mod(
                Natural::from_str(y).unwrap(),
                &Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            Natural::from_str(x).unwrap().eq_mod(
                &Natural::from_str(y).unwrap(),
                Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            Natural::from_str(x).unwrap().eq_mod(
                &Natural::from_str(y).unwrap(),
                &Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            (&Natural::from_str(x).unwrap()).eq_mod(
                Natural::from_str(y).unwrap(),
                Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            (&Natural::from_str(x).unwrap()).eq_mod(
                Natural::from_str(y).unwrap(),
                &Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            (&Natural::from_str(x).unwrap()).eq_mod(
                &Natural::from_str(y).unwrap(),
                Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            (&Natural::from_str(x).unwrap()).eq_mod(
                &Natural::from_str(y).unwrap(),
                &Natural::from_str(modulus).unwrap()
            ),
            out
        );

        assert_eq!(
            Natural::from_str(y).unwrap().eq_mod(
                Natural::from_str(x).unwrap(),
                Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            rug::Integer::from_str(x).unwrap().is_congruent(
                &rug::Integer::from_str(y).unwrap(),
                &rug::Integer::from_str(modulus).unwrap()
            ),
            out
        );
    };
    test("0", "0", "0", true);
    test("0", "1", "0", false);
    test("57", "57", "0", true);
    test("57", "58", "0", false);
    test("1000000000000", "57", "0", false);
    test("0", "256", "256", true);
    test("0", "256", "512", false);
    test("13", "23", "10", true);
    test("13", "24", "10", false);
    test("13", "21", "1", true);
    test("13", "21", "2", true);
    test("13", "21", "4", true);
    test("13", "21", "8", true);
    test("13", "21", "16", false);
    test("13", "21", "3", false);
    test("1000000000001", "1", "4096", true);
    test("1000000000001", "1", "8192", false);
    test("12345678987654321", "321", "1000", true);
    test("12345678987654321", "322", "1000", false);
    test("1234", "1234", "1000000000000", true);
    test("1234", "1235", "1000000000000", false);
    test("1000000001234", "1000000002234", "1000", true);
    test("1000000001234", "1000000002235", "1000", false);
    test("1000000001234", "1234", "1000000000000", true);
    test("1000000001234", "1235", "1000000000000", false);
    test("1000000001234", "5000000001234", "1000000000000", true);
    test("1000000001234", "5000000001235", "1000000000000", false);
}

#[test]
fn limbs_eq_limb_mod_limb_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1,
        |&(ref limbs, limb, modulus)| {
            let equal = limbs_eq_limb_mod_limb(limbs, limb, modulus);
            assert_eq!(
                Natural::from_limbs_asc(limbs).eq_mod(Natural::from(limb), Natural::from(modulus)),
                equal
            );
            assert_eq!(limbs_mod_limb(limbs, modulus) == limb % modulus, equal);
            assert_eq!(
                _combined_limbs_eq_limb_mod_limb(limbs, limb, modulus),
                equal
            );
        },
    );

    test_properties(
        triples_of_limb_vec_limb_and_positive_limb_var_3,
        |&(ref limbs, limb, modulus)| {
            assert!(limbs_eq_limb_mod_limb(limbs, limb, modulus));
            assert!(
                Natural::from_limbs_asc(limbs).eq_mod(Natural::from(limb), Natural::from(modulus))
            );
            assert_eq!(limbs_mod_limb(limbs, modulus), limb % modulus);
            assert!(_combined_limbs_eq_limb_mod_limb(limbs, limb, modulus));
        },
    );

    test_properties(
        triples_of_limb_vec_limb_and_positive_limb_var_4,
        |&(ref limbs, limb, modulus)| {
            assert!(!limbs_eq_limb_mod_limb(limbs, limb, modulus));
            assert!(
                !Natural::from_limbs_asc(limbs).eq_mod(Natural::from(limb), Natural::from(modulus))
            );
            assert_ne!(limbs_mod_limb(limbs, modulus), limb % modulus);
            assert!(!_combined_limbs_eq_limb_mod_limb(limbs, limb, modulus));
        },
    );
}

#[test]
fn limbs_eq_limb_mod_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1,
        |&(ref xs, y, ref modulus)| {
            let equal = limbs_eq_limb_mod_ref_ref(xs, y, modulus);
            let mut mut_xs = xs.clone();
            let mut mut_modulus = modulus.clone();
            assert_eq!(limbs_eq_limb_mod(&mut mut_xs, y, &mut mut_modulus), equal);
            let mut mut_xs = xs.clone();
            assert_eq!(limbs_eq_limb_mod_val_ref(&mut mut_xs, y, modulus), equal);
            let mut mut_modulus = modulus.clone();
            assert_eq!(limbs_eq_limb_mod_ref_val(xs, y, &mut mut_modulus), equal);
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
            assert!(limbs_eq_limb_mod_ref_ref(xs, y, modulus));
            assert!(_limbs_eq_limb_mod_naive_1(xs, y, modulus));
            assert!(_limbs_eq_limb_mod_naive_2(xs, y, modulus));
        },
    );

    test_properties(
        triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_3,
        |&(ref xs, y, ref modulus)| {
            assert!(!Natural::from_limbs_asc(xs)
                .eq_mod(Natural::from(y), Natural::from_limbs_asc(modulus)));
            assert!(!limbs_eq_limb_mod_ref_ref(xs, y, modulus));
            assert!(!_limbs_eq_limb_mod_naive_1(xs, y, modulus));
            assert!(!_limbs_eq_limb_mod_naive_2(xs, y, modulus));
        },
    );
}

#[test]
fn limbs_eq_mod_limb_ref_ref_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8,
        |&(ref xs, ref ys, modulus)| {
            let equal = limbs_eq_mod_limb_ref_ref(xs, ys, modulus);
            let mut mut_xs = xs.clone();
            assert_eq!(limbs_eq_mod_limb_val_ref(&mut mut_xs, ys, modulus), equal);
            let mut mut_ys = ys.clone();
            assert_eq!(limbs_eq_mod_limb_ref_val(xs, &mut mut_ys, modulus), equal);
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
            assert!(limbs_eq_mod_limb_ref_ref(xs, ys, modulus));
            assert!(_limbs_eq_mod_limb_naive_1(xs, ys, modulus));
            assert!(_limbs_eq_mod_limb_naive_2(xs, ys, modulus));
        },
    );

    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_10,
        |&(ref xs, ref ys, modulus)| {
            assert!(!Natural::from_limbs_asc(xs)
                .eq_mod(Natural::from_limbs_asc(ys), Natural::from(modulus)));
            assert!(!limbs_eq_mod_limb_ref_ref(xs, ys, modulus));
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
            let equal = limbs_eq_mod_ref_ref_ref(xs, ys, modulus);
            let mut mut_ys = ys.clone();
            let mut mut_modulus = modulus.clone();
            assert_eq!(
                limbs_eq_mod_ref_val_val(xs, &mut mut_ys, &mut mut_modulus),
                equal
            );
            let mut mut_ys = ys.clone();
            assert_eq!(limbs_eq_mod_ref_val_ref(xs, &mut mut_ys, modulus), equal);
            let mut mut_modulus = modulus.clone();
            assert_eq!(limbs_eq_mod_ref_ref_val(xs, ys, &mut mut_modulus), equal);
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
            assert!(limbs_eq_mod_ref_ref_ref(xs, ys, modulus));
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
            assert!(!limbs_eq_mod_ref_ref_ref(xs, ys, modulus));
            assert!(!_limbs_eq_mod_naive_1(xs, ys, modulus));
            assert!(!_limbs_eq_mod_naive_2(xs, ys, modulus));
        },
    );
}

#[test]
fn eq_mod_properties() {
    test_properties(triples_of_naturals, |&(ref x, ref y, ref modulus)| {
        let equal = x.eq_mod(y, modulus);
        assert_eq!(y.eq_mod(x, modulus), equal);

        assert_eq!(x.eq_mod(y, modulus.clone()), equal);
        assert_eq!(x.eq_mod(y.clone(), modulus), equal);
        assert_eq!(x.eq_mod(y.clone(), modulus.clone()), equal);
        assert_eq!(x.clone().eq_mod(y, modulus), equal);
        assert_eq!(x.clone().eq_mod(y, modulus.clone()), equal);
        assert_eq!(x.clone().eq_mod(y.clone(), modulus), equal);
        assert_eq!(x.clone().eq_mod(y.clone(), modulus.clone()), equal);

        assert_eq!(
            (Integer::from(x) - Integer::from(y)).divisible_by(Integer::from(modulus)),
            equal
        );
        assert_eq!(
            (Integer::from(y) - Integer::from(x)).divisible_by(Integer::from(modulus)),
            equal
        );
        assert_eq!(
            natural_to_rug_integer(x)
                .is_congruent(&natural_to_rug_integer(y), &natural_to_rug_integer(modulus)),
            equal
        );
    });

    test_properties(triples_of_naturals_var_2, |&(ref x, ref y, ref modulus)| {
        assert!(x.eq_mod(y, modulus));
        assert!(y.eq_mod(x, modulus));
        assert!(natural_to_rug_integer(x)
            .is_congruent(&natural_to_rug_integer(y), &natural_to_rug_integer(modulus)));
    });

    test_properties(triples_of_naturals_var_3, |&(ref x, ref y, ref modulus)| {
        assert!(!x.eq_mod(y, modulus));
        assert!(!y.eq_mod(x, modulus));
        assert!(!natural_to_rug_integer(x)
            .is_congruent(&natural_to_rug_integer(y), &natural_to_rug_integer(modulus)));
    });

    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        assert!(x.eq_mod(y, Natural::ONE));
        assert_eq!(x.eq_mod(Natural::ZERO, y), x.divisible_by(y));
        assert!(x.eq_mod(x, y));
        assert_eq!(x.eq_mod(y, Natural::ZERO), x == y);
    });
}
