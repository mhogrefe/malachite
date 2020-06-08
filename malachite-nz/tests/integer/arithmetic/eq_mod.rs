use std::str::FromStr;

use malachite_base::num::arithmetic::traits::EqMod;
#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::arithmetic::traits::{DivisibleBy, Mod};

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::integer::arithmetic::eq_mod::{
    limbs_eq_neg_limb_mod_limb, limbs_pos_eq_neg_limb_mod, limbs_pos_eq_neg_limb_mod_ref,
    limbs_pos_eq_neg_mod, limbs_pos_eq_neg_mod_limb, limbs_pos_eq_neg_mod_ref,
    limbs_pos_limb_eq_neg_limb_mod,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_eq_neg_limb_mod_limb() {
    let test = |xs: &[Limb], y: Limb, m: Limb, equal: bool| {
        assert_eq!(limbs_eq_neg_limb_mod_limb(xs, y, m), equal);
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
    test(&[u32::MAX, u32::MAX], 101, 2, true);
    test(&[u32::MAX, u32::MAX], 100, 2, false);
    test(&[u32::MAX, u32::MAX], 111, 3, true);
    test(&[u32::MAX, u32::MAX], 110, 3, false);
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
    let test = |x: Limb, y: Limb, ms: &[Limb], equal: bool| {
        assert_eq!(limbs_pos_limb_eq_neg_limb_mod(x, y, ms), equal);
        let x = Integer::from(x);
        let y = -Natural::from(y);
        let m = Natural::from_limbs_asc(ms);
        assert_eq!((&x).eq_mod(&y, &m), equal);
        let m = Integer::from(m);
        assert_eq!(
            x == y || m != 0 && (&x).mod_op(&m) == (&y).mod_op(&m),
            equal
        );
        assert_eq!((x - y).divisible_by(m), equal);
    };
    test(1, 1, &[1, 1], false);
    test(1, 1, &[2, 1], false);
    test(1, 1, &[1, 0, 1], false);
    test(u32::MAX, u32::MAX, &[u32::MAX - 1, 1], true);
    test(u32::MAX, u32::MAX, &[u32::MAX - 1, 1, 2], false);
    test(u32::MAX, u32::MAX, &[u32::MAX - 1, 2], false);
    test(0xabcd_dbca, 0x641f_efdf, &[0xfed_cba9, 1], true);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_pos_eq_neg_limb_mod() {
    let test = |xs: &[Limb], y: Limb, ms: &[Limb], equal: bool| {
        let mut mut_ms = ms.to_vec();
        assert_eq!(limbs_pos_eq_neg_limb_mod(xs, y, &mut mut_ms), equal);
        assert_eq!(limbs_pos_eq_neg_limb_mod_ref(xs, y, ms), equal);
        let x = Integer::from(Natural::from_limbs_asc(xs));
        let y = -Natural::from(y);
        let m = Natural::from_limbs_asc(ms);
        assert_eq!((&x).eq_mod(&y, &m), equal);
        let m = Integer::from(m);
        assert_eq!(
            x == y || m != 0 && (&x).mod_op(&m) == (&y).mod_op(&m),
            equal
        );
        assert_eq!((x - y).divisible_by(m), equal);
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
    let test = |xs: &[Limb], ys: &[Limb], m: Limb, equal: bool| {
        assert_eq!(limbs_pos_eq_neg_mod_limb(xs, ys, m), equal);
        let x = Integer::from(Natural::from_limbs_asc(xs));
        let y = -Natural::from_limbs_asc(ys);
        let m = Natural::from(m);
        assert_eq!((&x).eq_mod(&y, &m), equal);
        let m = Integer::from(m);
        assert_eq!(
            x == y || m != 0 && (&x).mod_op(&m) == (&y).mod_op(&m),
            equal
        );
        assert_eq!((x - y).divisible_by(m), equal);
    };
    // xs[0].wrapping_neg().eq_mod_power_of_two(ys[0], u64::from(m.trailing_zeros()))
    //      in limbs_pos_eq_mod_neg_limb_greater
    test(&[0, 1], &[0, 1], 1, true);
    test(&[0, 1], &[0, 1], 2, true);
    test(&[0, 1], &[6, 1], 2, true);
    // !xs[0].wrapping_neg().eq_mod_power_of_two(ys[0], u64::from(m.trailing_zeros()))
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
    let test = |xs: &[Limb], ys: &[Limb], ms: &[Limb], equal: bool| {
        let mut mut_ms = ms.to_vec();
        assert_eq!(limbs_pos_eq_neg_mod(xs, ys, &mut mut_ms), equal);
        assert_eq!(limbs_pos_eq_neg_mod_ref(xs, ys, ms), equal);
        let x = Integer::from(Natural::from_limbs_asc(xs));
        let y = -Natural::from_limbs_asc(ys);
        let m = Natural::from_limbs_asc(ms);
        assert_eq!((&x).eq_mod(&y, &m), equal);
        let m = Integer::from(m);
        assert_eq!(
            x == y || m != 0 && (&x).mod_op(&m) == (&y).mod_op(&m),
            equal
        );
        assert_eq!((x - y).divisible_by(m), equal);
    };
    // !xs[0].wrapping_neg().eq_mod_power_of_two(ys[0], u64::from(ms[0].trailing_zeros()))
    //      in limbs_pos_eq_neg_mod_greater
    test(&[1, 2], &[3, 4], &[0, 1], false);
    test(&[0, 0, 1], &[0, 1], &[1, 1], true);
    // xs[0].wrapping_neg().eq_mod_power_of_two(ys[0], u64::from(ms[0].trailing_zeros()))
    //      in limbs_pos_eq_neg_mod_greater
    test(
        &[
            936_369_948,
            322_455_623,
            3_632_895_046,
            978_349_680,
            17_000_327,
            2_833_388_987,
            2_719_643_819,
            4_166_701_038,
        ],
        &[
            2_342_728_269,
            2_320_695_303,
            2_977_562_202,
            4_108_534_583,
            1_505_907_268,
            3_739_165_110,
            101_046_064,
            1_901_445_664,
        ],
        &[
            602_975_281,
            3_649_288_173,
            1_789_153_785,
            3_864_060_421,
            3_382_875_975,
            610_141_130,
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
    let test = |x, y, m, out| {
        assert_eq!(
            Integer::from_str(x)
                .unwrap()
                .eq_mod(Integer::from_str(y).unwrap(), Natural::from_str(m).unwrap()),
            out
        );
        assert_eq!(
            Integer::from_str(x).unwrap().eq_mod(
                Integer::from_str(y).unwrap(),
                &Natural::from_str(m).unwrap()
            ),
            out
        );
        assert_eq!(
            Integer::from_str(x).unwrap().eq_mod(
                &Integer::from_str(y).unwrap(),
                Natural::from_str(m).unwrap()
            ),
            out
        );
        assert_eq!(
            Integer::from_str(x).unwrap().eq_mod(
                &Integer::from_str(y).unwrap(),
                &Natural::from_str(m).unwrap()
            ),
            out
        );
        assert_eq!(
            (&Integer::from_str(x).unwrap())
                .eq_mod(Integer::from_str(y).unwrap(), Natural::from_str(m).unwrap()),
            out
        );
        assert_eq!(
            (&Integer::from_str(x).unwrap()).eq_mod(
                Integer::from_str(y).unwrap(),
                &Natural::from_str(m).unwrap()
            ),
            out
        );
        assert_eq!(
            (&Integer::from_str(x).unwrap()).eq_mod(
                &Integer::from_str(y).unwrap(),
                Natural::from_str(m).unwrap()
            ),
            out
        );
        assert_eq!(
            (&Integer::from_str(x).unwrap()).eq_mod(
                &Integer::from_str(y).unwrap(),
                &Natural::from_str(m).unwrap()
            ),
            out
        );

        assert_eq!(
            Integer::from_str(y)
                .unwrap()
                .eq_mod(Integer::from_str(x).unwrap(), Natural::from_str(m).unwrap()),
            out
        );
        assert_eq!(
            rug::Integer::from_str(x).unwrap().is_congruent(
                &rug::Integer::from_str(y).unwrap(),
                &rug::Integer::from_str(m).unwrap()
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
