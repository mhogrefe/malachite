use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{DivisibleBy, EqMod, Mod};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::integer::arithmetic::eq_mod::{
    limbs_eq_neg_limb_mod_limb, limbs_pos_eq_neg_limb_mod, limbs_pos_eq_neg_limb_mod_ref,
    limbs_pos_eq_neg_mod, limbs_pos_eq_neg_mod_limb, limbs_pos_eq_neg_mod_ref,
    limbs_pos_limb_eq_neg_limb_mod,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::common::{integer_to_rug_integer, natural_to_rug_integer};
use malachite_test::inputs::base::{
    triples_of_limb_limb_and_limb_vec_var_2, triples_of_limb_vec_limb_and_limb_vec_var_4,
    triples_of_limb_vec_limb_and_limb_vec_var_5, triples_of_limb_vec_limb_vec_and_limb_var_11,
    triples_of_limb_vec_limb_vec_and_limb_var_12, triples_of_limb_vec_var_58,
    triples_of_limb_vec_var_59, triples_of_unsigned_unsigned_and_unsigned_vec_var_1,
    triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1,
    triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8, triples_of_unsigned_vec_var_55,
};
use malachite_test::inputs::integer::{
    pairs_of_integer_and_natural, pairs_of_integers, triples_of_integer_integer_and_natural,
    triples_of_integer_integer_and_natural_var_1, triples_of_integer_integer_and_natural_var_2,
};

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_eq_neg_limb_mod_limb() {
    let test = |limbs: &[Limb], limb: Limb, modulus: Limb, equal: bool| {
        assert_eq!(limbs_eq_neg_limb_mod_limb(limbs, limb, modulus), equal);
    };
    test(&[6, 7], 4, 2, true);
    test(&[7, 7], 4, 2, false);
    test(&[6, 7], 3, 2, false);
    test(&[7, 7], 3, 2, true);
    test(&[2, 2], 6, 13, true);
    test(&[100, 101, 102], 1_232, 10, true);
    test(&[100, 101, 102], 1_233, 10, false);
    test(&[123, 456], 153, 789, true);
    test(&[123, 456], 1_000, 789, false);
    test(&[0xffff_ffff, 0xffff_ffff], 101, 2, true);
    test(&[0xffff_ffff, 0xffff_ffff], 100, 2, false);
    test(&[0xffff_ffff, 0xffff_ffff], 111, 3, true);
    test(&[0xffff_ffff, 0xffff_ffff], 110, 3, false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_neg_limb_mod_limb_fail() {
    limbs_eq_neg_limb_mod_limb(&[10], 10, 15);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_pos_limb_eq_neg_limb_mod() {
    let test = |x: Limb, y: Limb, modulus: &[Limb], equal: bool| {
        assert_eq!(limbs_pos_limb_eq_neg_limb_mod(x, y, modulus), equal);
        let x = Integer::from(x);
        let y = -Natural::from(y);
        let modulus = Natural::from_limbs_asc(modulus);
        assert_eq!((&x).eq_mod(&y, &modulus), equal);
        let modulus = Integer::from(modulus);
        assert_eq!(
            x == y || modulus != 0 && (&x).mod_op(&modulus) == (&y).mod_op(&modulus),
            equal
        );
        assert_eq!((x - y).divisible_by(modulus), equal);
    };
    test(1, 1, &[1, 1], false);
    test(1, 1, &[2, 1], false);
    test(1, 1, &[1, 0, 1], false);
    test(0xffff_ffff, 0xffff_ffff, &[0xffff_fffe, 1], true);
    test(0xffff_ffff, 0xffff_ffff, &[0xffff_fffe, 1, 2], false);
    test(0xffff_ffff, 0xffff_ffff, &[0xffff_fffe, 2], false);
    test(0xabcd_dbca, 0x641f_efdf, &[0xfed_cba9, 1], true);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_pos_eq_neg_limb_mod() {
    let test = |xs: &[Limb], y: Limb, modulus: &[Limb], equal: bool| {
        let mut mut_modulus = modulus.to_vec();
        assert_eq!(limbs_pos_eq_neg_limb_mod(xs, y, &mut mut_modulus), equal);
        assert_eq!(limbs_pos_eq_neg_limb_mod_ref(xs, y, modulus), equal);
        let x = Integer::from(Natural::from_limbs_asc(xs));
        let y = -Natural::from(y);
        let modulus = Natural::from_limbs_asc(modulus);
        assert_eq!((&x).eq_mod(&y, &modulus), equal);
        let modulus = Integer::from(modulus);
        assert_eq!(
            x == y || modulus != 0 && (&x).mod_op(&modulus) == (&y).mod_op(&modulus),
            equal
        );
        assert_eq!((x - y).divisible_by(modulus), equal);
    };
    // !xs[0].wrapping_neg().eq_mod_power_of_two(y, u64::from(twos))
    test(&[1, 2], 2, &[2, 1], false);
    // xs[0].wrapping_neg().eq_mod_power_of_two(y, u64::from(twos))
    // m_len == 2 && m_0 != 0
    // m_1 < 1 << twos
    // x_len < BMOD_1_TO_MOD_1_THRESHOLD
    test(&[2, 2], 2, &[2, 1], true);
    // m_1 >= 1 << twos
    test(&[0, 1], 1, &[1, 1], true);
    // m_len > 2 || m_0 == 0
    test(&[0, 1], 1, &[1, 0, 1], false);
    // x_len >= BMOD_1_TO_MOD_1_THRESHOLD
    // y < m_0
    test(
        &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 1,
        ],
        2,
        &[2, 1],
        false,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_limb_mod_fail_1() {
    limbs_pos_eq_neg_limb_mod(&[1], 1, &mut [0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_limb_mod_fail_2() {
    limbs_pos_eq_neg_limb_mod(&[1, 1], 1, &mut [1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_limb_mod_fail_3() {
    limbs_pos_eq_neg_limb_mod(&[1, 0], 1, &mut [0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_limb_mod_fail_4() {
    limbs_pos_eq_neg_limb_mod(&[1, 1], 0, &mut [0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_limb_mod_fail_5() {
    limbs_pos_eq_neg_limb_mod(&[1, 1], 1, &mut [1, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_limb_mod_ref_fail_1() {
    limbs_pos_eq_neg_limb_mod_ref(&[1], 1, &[0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_limb_mod_ref_fail_2() {
    limbs_pos_eq_neg_limb_mod_ref(&[1, 1], 1, &[1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_limb_mod_ref_fail_3() {
    limbs_pos_eq_neg_limb_mod_ref(&[1, 0], 1, &[0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_limb_mod_ref_fail_4() {
    limbs_pos_eq_neg_limb_mod_ref(&[1, 1], 0, &[0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_limb_mod_ref_fail_5() {
    limbs_pos_eq_neg_limb_mod_ref(&[1, 1], 1, &[1, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_pos_eq_neg_mod_limb() {
    let test = |xs: &[Limb], ys: &[Limb], modulus: Limb, equal: bool| {
        assert_eq!(limbs_pos_eq_neg_mod_limb(xs, ys, modulus), equal);
        let x = Integer::from(Natural::from_limbs_asc(xs));
        let y = -Natural::from_limbs_asc(ys);
        let modulus = Natural::from(modulus);
        assert_eq!((&x).eq_mod(&y, &modulus), equal);
        let modulus = Integer::from(modulus);
        assert_eq!(
            x == y || modulus != 0 && (&x).mod_op(&modulus) == (&y).mod_op(&modulus),
            equal
        );
        assert_eq!((x - y).divisible_by(modulus), equal);
    };
    // xs[0].wrapping_neg().eq_mod_power_of_two(ys[0], u64::from(modulus.trailing_zeros()))
    //      in limbs_pos_eq_mod_neg_limb_greater
    test(&[0, 1], &[0, 1], 1, true);
    test(&[0, 1], &[0, 1], 2, true);
    test(&[0, 1], &[6, 1], 2, true);
    // !xs[0].wrapping_neg().eq_mod_power_of_two(ys[0], u64::from(modulus.trailing_zeros()))
    //      in limbs_pos_eq_mod_neg_limb_greater
    test(&[0, 1], &[7, 1], 2, false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_limb_fail_1() {
    limbs_pos_eq_neg_mod_limb(&[1], &[3, 4], 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_limb_fail_2() {
    limbs_pos_eq_neg_mod_limb(&[1, 1], &[4], 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_limb_fail_3() {
    limbs_pos_eq_neg_mod_limb(&[1, 0], &[3, 4], 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_limb_fail_4() {
    limbs_pos_eq_neg_mod_limb(&[1, 1], &[3, 0], 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_limb_fail_5() {
    limbs_pos_eq_neg_mod_limb(&[1, 1], &[3, 4], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_pos_eq_neg_mod() {
    let test = |xs: &[Limb], ys: &[Limb], modulus: &[Limb], equal: bool| {
        let mut mut_modulus = modulus.to_vec();
        assert_eq!(limbs_pos_eq_neg_mod(xs, ys, &mut mut_modulus), equal);
        assert_eq!(limbs_pos_eq_neg_mod_ref(xs, ys, modulus), equal);
        let x = Integer::from(Natural::from_limbs_asc(xs));
        let y = -Natural::from_limbs_asc(ys);
        let modulus = Natural::from_limbs_asc(modulus);
        assert_eq!((&x).eq_mod(&y, &modulus), equal);
        let modulus = Integer::from(modulus);
        assert_eq!(
            x == y || modulus != 0 && (&x).mod_op(&modulus) == (&y).mod_op(&modulus),
            equal
        );
        assert_eq!((x - y).divisible_by(modulus), equal);
    };
    // !xs[0].wrapping_neg().eq_mod_power_of_two(ys[0], u64::from(modulus[0].trailing_zeros()))
    //      in limbs_pos_eq_neg_mod_greater
    test(&[1, 2], &[3, 4], &[0, 1], false);
    test(&[0, 0, 1], &[0, 1], &[1, 1], true);
    // xs[0].wrapping_neg().eq_mod_power_of_two(ys[0], u64::from(modulus[0].trailing_zeros()))
    //      in limbs_pos_eq_neg_mod_greater
    test(
        &[
            936369948, 322455623, 3632895046, 978349680, 17000327, 2833388987, 2719643819,
            4166701038,
        ],
        &[
            2342728269, 2320695303, 2977562202, 4108534583, 1505907268, 3739165110, 101046064,
            1901445664,
        ],
        &[
            602975281, 3649288173, 1789153785, 3864060421, 3382875975, 610141130,
        ],
        false,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_fail_1() {
    limbs_pos_eq_neg_mod(&[1], &[1, 0, 3], &mut [0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_fail_2() {
    limbs_pos_eq_neg_mod(&[1, 1, 1], &[1], &mut [0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_fail_3() {
    limbs_pos_eq_neg_mod(&[1, 1, 1], &[1, 0, 3], &mut [7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_fail_4() {
    limbs_pos_eq_neg_mod(&[1, 1, 0], &[1, 0, 3], &mut [0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_fail_5() {
    limbs_pos_eq_neg_mod(&[1, 1, 1], &[1, 0, 0], &mut [0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_fail_6() {
    limbs_pos_eq_neg_mod(&[1, 1, 1], &[1, 0, 3], &mut [7, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_ref_fail_1() {
    limbs_pos_eq_neg_mod_ref(&[1], &[1, 0, 3], &[0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_ref_fail_2() {
    limbs_pos_eq_neg_mod_ref(&[1, 1, 1], &[1], &[0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_ref_fail_3() {
    limbs_pos_eq_neg_mod_ref(&[1, 1, 1], &[1, 0, 3], &[7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_ref_fail_4() {
    limbs_pos_eq_neg_mod_ref(&[1, 1, 0], &[1, 0, 3], &[0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_ref_fail_5() {
    limbs_pos_eq_neg_mod_ref(&[1, 1, 1], &[1, 0, 0], &[0, 7]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_eq_neg_mod_ref_fail_6() {
    limbs_pos_eq_neg_mod_ref(&[1, 1, 1], &[1, 0, 3], &[7, 0]);
}

#[test]
fn test_eq_mod() {
    let test = |x, y, modulus, out| {
        assert_eq!(
            Integer::from_str(x).unwrap().eq_mod(
                Integer::from_str(y).unwrap(),
                Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            Integer::from_str(x).unwrap().eq_mod(
                Integer::from_str(y).unwrap(),
                &Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            Integer::from_str(x).unwrap().eq_mod(
                &Integer::from_str(y).unwrap(),
                Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            Integer::from_str(x).unwrap().eq_mod(
                &Integer::from_str(y).unwrap(),
                &Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            (&Integer::from_str(x).unwrap()).eq_mod(
                Integer::from_str(y).unwrap(),
                Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            (&Integer::from_str(x).unwrap()).eq_mod(
                Integer::from_str(y).unwrap(),
                &Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            (&Integer::from_str(x).unwrap()).eq_mod(
                &Integer::from_str(y).unwrap(),
                Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            (&Integer::from_str(x).unwrap()).eq_mod(
                &Integer::from_str(y).unwrap(),
                &Natural::from_str(modulus).unwrap()
            ),
            out
        );

        assert_eq!(
            Integer::from_str(y).unwrap().eq_mod(
                Integer::from_str(x).unwrap(),
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

    test("0", "-1", "0", false);
    test("57", "-57", "0", false);
    test("57", "-58", "0", false);
    test("1000000000000", "-57", "0", false);
    test("0", "-256", "256", true);
    test("0", "-256", "512", false);
    test("13", "-27", "10", true);
    test("13", "-28", "10", false);
    test("29", "-27", "1", true);
    test("29", "-27", "2", true);
    test("29", "-27", "4", true);
    test("29", "-27", "8", true);
    test("29", "-27", "16", false);
    test("29", "-27", "3", false);
    test("999999999999", "-1", "4096", true);
    test("999999999999", "-1", "8192", false);
    test("12345678987654321", "-679", "1000", true);
    test("12345678987654321", "-680", "1000", false);
    test("1000000001234", "-999999999766", "1000", true);
    test("1000000001234", "-999999999767", "1000", false);
    test("1000000001234", "-999999998766", "1000000000000", true);
    test("1000000001234", "-999999998767", "1000000000000", false);

    test("-1", "0", "0", false);
    test("-57", "57", "0", false);
    test("-57", "58", "0", false);
    test("-1000000000000", "57", "0", false);
    test("-256", "0", "256", true);
    test("-256", "0", "512", false);
    test("-13", "27", "10", true);
    test("-13", "28", "10", false);
    test("-29", "27", "1", true);
    test("-29", "27", "2", true);
    test("-29", "27", "4", true);
    test("-29", "27", "8", true);
    test("-29", "27", "16", false);
    test("-29", "27", "3", false);
    test("-999999999999", "1", "4096", true);
    test("-999999999999", "1", "8192", false);
    test("-12345678987654321", "679", "1000", true);
    test("-12345678987654321", "680", "1000", false);
    test("-1000000001234", "999999999766", "1000", true);
    test("-1000000001234", "999999999767", "1000", false);
    test("-1000000001234", "999999998766", "1000000000000", true);
    test("-1000000001234", "999999998767", "1000000000000", false);

    test("-57", "-57", "0", true);
    test("-57", "-58", "0", false);
    test("-1000000000000", "-57", "0", false);
    test("-13", "-23", "10", true);
    test("-13", "-24", "10", false);
    test("-13", "-21", "1", true);
    test("-13", "-21", "2", true);
    test("-13", "-21", "4", true);
    test("-13", "-21", "8", true);
    test("-13", "-21", "16", false);
    test("-13", "-21", "3", false);
    test("-1000000000001", "-1", "4096", true);
    test("-1000000000001", "-1", "8192", false);
    test("-12345678987654321", "-321", "1000", true);
    test("-12345678987654321", "-322", "1000", false);
    test("-1234", "-1234", "1000000000000", true);
    test("-1234", "-1235", "1000000000000", false);
    test("-1000000001234", "-1000000002234", "1000", true);
    test("-1000000001234", "-1000000002235", "1000", false);
    test("-1000000001234", "-1234", "1000000000000", true);
    test("-1000000001234", "-1235", "1000000000000", false);
    test("-1000000001234", "-5000000001234", "1000000000000", true);
    test("-1000000001234", "-5000000001235", "1000000000000", false);
}

#[test]
fn limbs_eq_neg_limb_mod_limb_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1,
        |&(ref limbs, limb, modulus)| {
            let equal = limbs_eq_neg_limb_mod_limb(limbs, limb, modulus);
            assert_eq!(
                (-Natural::from_limbs_asc(limbs))
                    .eq_mod(Integer::from(limb), Natural::from(modulus)),
                equal
            );
        },
    );
}

#[test]
fn limbs_pos_limb_eq_neg_limb_mod_properties() {
    test_properties(
        triples_of_unsigned_unsigned_and_unsigned_vec_var_1,
        |&(x, y, ref modulus)| {
            let equal = limbs_pos_limb_eq_neg_limb_mod(x, y, modulus);
            let x = Integer::from(x);
            let y = -Natural::from(y);
            let modulus = Natural::from_limbs_asc(modulus);
            assert_eq!((&x).eq_mod(&y, &modulus), equal);
            let modulus = Integer::from(modulus);
            assert_eq!(
                x == y || modulus != 0 && (&x).mod_op(&modulus) == (&y).mod_op(&modulus),
                equal
            );
            assert_eq!((x - y).divisible_by(modulus), equal);
        },
    );

    test_properties(
        triples_of_limb_limb_and_limb_vec_var_2,
        |&(x, y, ref modulus)| {
            assert!(!limbs_pos_limb_eq_neg_limb_mod(x, y, modulus));
            let x = Integer::from(x);
            let y = -Natural::from(y);
            let modulus = Natural::from_limbs_asc(modulus);
            assert!(!(&x).eq_mod(&y, &modulus));
            let modulus = Integer::from(modulus);
            assert!(x != y && (modulus == 0 || (&x).mod_op(&modulus) != (&y).mod_op(&modulus)));
            assert!(!(x - y).divisible_by(modulus));
        },
    );
}

#[test]
fn limbs_pos_eq_neg_limb_mod_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_and_unsigned_vec_var_1,
        |&(ref xs, y, ref modulus)| {
            let equal = limbs_pos_eq_neg_limb_mod_ref(xs, y, modulus);
            let mut mut_modulus = modulus.clone();
            assert_eq!(limbs_pos_eq_neg_limb_mod(xs, y, &mut mut_modulus), equal);
            let x = Integer::from(Natural::from_limbs_asc(xs));
            let y = -Natural::from(y);
            let modulus = Natural::from_limbs_asc(modulus);
            assert_eq!((&x).eq_mod(&y, &modulus), equal);
            let modulus = Integer::from(modulus);
            assert_eq!(
                x == y || modulus != 0 && (&x).mod_op(&modulus) == (&y).mod_op(&modulus),
                equal
            );
            assert_eq!((x - y).divisible_by(modulus), equal);
        },
    );

    test_properties(
        triples_of_limb_vec_limb_and_limb_vec_var_4,
        |&(ref xs, y, ref modulus)| {
            assert!(limbs_pos_eq_neg_limb_mod_ref(xs, y, modulus));
            let mut mut_modulus = modulus.clone();
            assert!(limbs_pos_eq_neg_limb_mod(xs, y, &mut mut_modulus));
            let x = Integer::from(Natural::from_limbs_asc(xs));
            let y = -Natural::from(y);
            let modulus = Natural::from_limbs_asc(modulus);
            assert!((&x).eq_mod(&y, &modulus));
            let modulus = Integer::from(modulus);
            assert!(x == y || modulus != 0 && (&x).mod_op(&modulus) == (&y).mod_op(&modulus));
            assert!((x - y).divisible_by(modulus));
        },
    );

    test_properties(
        triples_of_limb_vec_limb_and_limb_vec_var_5,
        |&(ref xs, y, ref modulus)| {
            assert!(!limbs_pos_eq_neg_limb_mod_ref(xs, y, modulus));
            let mut mut_modulus = modulus.clone();
            assert!(!limbs_pos_eq_neg_limb_mod(xs, y, &mut mut_modulus));
            let x = Integer::from(Natural::from_limbs_asc(xs));
            let y = -Natural::from(y);
            let modulus = Natural::from_limbs_asc(modulus);
            assert!(!(&x).eq_mod(&y, &modulus));
            let modulus = Integer::from(modulus);
            assert!(x != y && (modulus == 0 || (&x).mod_op(&modulus) != (&y).mod_op(&modulus)));
            assert!(!(x - y).divisible_by(modulus));
        },
    );
}

#[test]
fn limbs_pos_eq_neg_mod_limb_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_8,
        |&(ref xs, ref ys, modulus)| {
            let equal = limbs_pos_eq_neg_mod_limb(xs, ys, modulus);
            let x = Integer::from(Natural::from_limbs_asc(xs));
            let y = -Natural::from_limbs_asc(ys);
            let modulus = Natural::from(modulus);
            assert_eq!((&x).eq_mod(&y, &modulus), equal);
            let modulus = Integer::from(modulus);
            assert_eq!(
                x == y || modulus != 0 && (&x).mod_op(&modulus) == (&y).mod_op(&modulus),
                equal
            );
            assert_eq!((x - y).divisible_by(modulus), equal);
        },
    );

    test_properties(
        triples_of_limb_vec_limb_vec_and_limb_var_11,
        |&(ref xs, ref ys, modulus)| {
            assert!(limbs_pos_eq_neg_mod_limb(xs, ys, modulus));
            let x = Integer::from(Natural::from_limbs_asc(xs));
            let y = -Natural::from_limbs_asc(ys);
            let modulus = Natural::from(modulus);
            assert!((&x).eq_mod(&y, &modulus));
            let modulus = Integer::from(modulus);
            assert!(x == y || modulus != 0 && (&x).mod_op(&modulus) == (&y).mod_op(&modulus));
            assert!((x - y).divisible_by(modulus));
        },
    );

    test_properties(
        triples_of_limb_vec_limb_vec_and_limb_var_12,
        |&(ref xs, ref ys, modulus)| {
            assert!(!limbs_pos_eq_neg_mod_limb(xs, ys, modulus));
            let x = Integer::from(Natural::from_limbs_asc(xs));
            let y = -Natural::from_limbs_asc(ys);
            let modulus = Natural::from(modulus);
            assert!(!(&x).eq_mod(&y, &modulus));
            let modulus = Integer::from(modulus);
            assert!(x != y && (modulus == 0 || (&x).mod_op(&modulus) != (&y).mod_op(&modulus)));
            assert!(!(x - y).divisible_by(modulus));
        },
    );
}

#[test]
fn limbs_pos_eq_neg_mod_properties() {
    test_properties(
        triples_of_unsigned_vec_var_55,
        |&(ref xs, ref ys, ref modulus)| {
            let equal = limbs_pos_eq_neg_mod_ref(xs, ys, modulus);
            let mut mut_modulus = modulus.clone();
            assert_eq!(limbs_pos_eq_neg_mod(xs, ys, &mut mut_modulus), equal);
            let x = Integer::from(Natural::from_limbs_asc(xs));
            let y = -Natural::from_limbs_asc(ys);
            let modulus = Natural::from_limbs_asc(modulus);
            assert_eq!((&x).eq_mod(&y, &modulus), equal);
            let modulus = Integer::from(modulus);
            assert_eq!(
                x == y || modulus != 0 && (&x).mod_op(&modulus) == (&y).mod_op(&modulus),
                equal
            );
            assert_eq!((x - y).divisible_by(modulus), equal);
        },
    );

    test_properties(
        triples_of_limb_vec_var_58,
        |&(ref xs, ref ys, ref modulus)| {
            assert!(limbs_pos_eq_neg_mod_ref(xs, ys, modulus));
            let mut mut_modulus = modulus.clone();
            assert!(limbs_pos_eq_neg_mod_ref(xs, ys, &mut mut_modulus));
            let x = Integer::from(Natural::from_limbs_asc(xs));
            let y = -Natural::from_limbs_asc(ys);
            let modulus = Natural::from_limbs_asc(modulus);
            assert!((&x).eq_mod(&y, &modulus));
            let modulus = Integer::from(modulus);
            assert!(x == y || modulus != 0 && (&x).mod_op(&modulus) == (&y).mod_op(&modulus));
            assert!((x - y).divisible_by(modulus));
        },
    );

    test_properties(
        triples_of_limb_vec_var_59,
        |&(ref xs, ref ys, ref modulus)| {
            assert!(!limbs_pos_eq_neg_mod_ref(xs, ys, modulus));
            let mut mut_modulus = modulus.clone();
            assert!(!limbs_pos_eq_neg_mod_ref(xs, ys, &mut mut_modulus));
            let x = Integer::from(Natural::from_limbs_asc(xs));
            let y = -Natural::from_limbs_asc(ys);
            let modulus = Natural::from_limbs_asc(modulus);
            assert!(!(&x).eq_mod(&y, &modulus));
            let modulus = Integer::from(modulus);
            assert!(x != y && (modulus == 0 || (&x).mod_op(&modulus) != (&y).mod_op(&modulus)));
            assert!(!(x - y).divisible_by(modulus));
        },
    );
}

#[test]
fn eq_mod_properties() {
    test_properties(
        triples_of_integer_integer_and_natural,
        |&(ref x, ref y, ref modulus)| {
            let equal = x.eq_mod(y, modulus);
            assert_eq!(y.eq_mod(x, modulus), equal);

            assert_eq!(x.eq_mod(y, modulus.clone()), equal);
            assert_eq!(x.eq_mod(y.clone(), modulus), equal);
            assert_eq!(x.eq_mod(y.clone(), modulus.clone()), equal);
            assert_eq!(x.clone().eq_mod(y, modulus), equal);
            assert_eq!(x.clone().eq_mod(y, modulus.clone()), equal);
            assert_eq!(x.clone().eq_mod(y.clone(), modulus), equal);
            assert_eq!(x.clone().eq_mod(y.clone(), modulus.clone()), equal);

            assert_eq!((-x).eq_mod(-y, modulus), equal);
            assert_eq!((x - y).divisible_by(Integer::from(modulus)), equal);
            assert_eq!((y - x).divisible_by(Integer::from(modulus)), equal);
            assert_eq!(
                integer_to_rug_integer(x)
                    .is_congruent(&integer_to_rug_integer(y), &natural_to_rug_integer(modulus)),
                equal
            );
        },
    );

    test_properties(
        triples_of_integer_integer_and_natural_var_1,
        |&(ref x, ref y, ref modulus)| {
            assert!(x.eq_mod(y, modulus));
            assert!(y.eq_mod(x, modulus));
            assert!(integer_to_rug_integer(x)
                .is_congruent(&integer_to_rug_integer(y), &natural_to_rug_integer(modulus)));
        },
    );

    test_properties(
        triples_of_integer_integer_and_natural_var_2,
        |&(ref x, ref y, ref modulus)| {
            assert!(!x.eq_mod(y, modulus));
            assert!(!y.eq_mod(x, modulus));
            assert!(!integer_to_rug_integer(x)
                .is_congruent(&integer_to_rug_integer(y), &natural_to_rug_integer(modulus)));
        },
    );

    test_properties(pairs_of_integers, |&(ref x, ref y)| {
        assert!(x.eq_mod(y, Natural::ONE));
        assert_eq!(x.eq_mod(y, Natural::ZERO), x == y);
    });

    test_properties(pairs_of_integer_and_natural, |&(ref x, ref y)| {
        assert_eq!(x.eq_mod(Integer::ZERO, y), x.divisible_by(Integer::from(y)));
        assert!(x.eq_mod(x, y));
    });
}
