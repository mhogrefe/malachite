use malachite_base::num::arithmetic::traits::EqMod;
#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::arithmetic::traits::{DivisibleBy, Mod};
use std::str::FromStr;

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
    test(&[100, 101, 102], 1232, 10, true);
    test(&[100, 101, 102], 1233, 10, false);
    test(&[123, 456], 153, 789, true);
    test(&[123, 456], 1000, 789, false);
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
    test(0xabcddbca, 0x641fefdf, &[0xfedcba9, 1], true);
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
    // !xs[0].wrapping_neg().eq_mod_power_of_2(y, u64::from(twos))
    test(&[1, 2], 2, &[2, 1], false);
    // xs[0].wrapping_neg().eq_mod_power_of_2(y, u64::from(twos))
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
    // xs[0].wrapping_neg().eq_mod_power_of_2(ys[0], u64::from(m.trailing_zeros()))
    //      in limbs_pos_eq_mod_neg_limb_greater
    test(&[0, 1], &[0, 1], 1, true);
    test(&[0, 1], &[0, 1], 2, true);
    test(&[0, 1], &[6, 1], 2, true);
    // !xs[0].wrapping_neg().eq_mod_power_of_2(ys[0], u64::from(m.trailing_zeros()))
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
    // !xs[0].wrapping_neg().eq_mod_power_of_2(ys[0], u64::from(ms[0].trailing_zeros()))
    //      in limbs_pos_eq_neg_mod_greater
    test(&[1, 2], &[3, 4], &[0, 1], false);
    test(&[0, 0, 1], &[0, 1], &[1, 1], true);
    // xs[0].wrapping_neg().eq_mod_power_of_2(ys[0], u64::from(ms[0].trailing_zeros()))
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
        &[602975281, 3649288173, 1789153785, 3864060421, 3382875975, 610141130],
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
    let test = |r, s, t, out| {
        let u = Integer::from_str(r).unwrap();
        let v = Integer::from_str(s).unwrap();
        let w = Natural::from_str(t).unwrap();

        assert_eq!(u.clone().eq_mod(v.clone(), w.clone()), out);
        assert_eq!(u.clone().eq_mod(v.clone(), &w), out);
        assert_eq!(u.clone().eq_mod(&v, w.clone()), out);
        assert_eq!(u.clone().eq_mod(&v, &w), out);
        assert_eq!((&u).eq_mod(v.clone(), w.clone()), out);
        assert_eq!((&u).eq_mod(v.clone(), &w), out);
        assert_eq!((&u).eq_mod(&v, w.clone()), out);
        assert_eq!((&u).eq_mod(&v, &w), out);
        assert_eq!(v.eq_mod(u, w), out);
        assert_eq!(
            rug::Integer::from_str(r).unwrap().is_congruent(
                &rug::Integer::from_str(s).unwrap(),
                &rug::Integer::from_str(t).unwrap()
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
