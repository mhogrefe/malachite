// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ModIsReduced, ModMul, ModNeg, ModPow, ModPowAssign, Parity,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::unsigned_triple_gen_var_15;
use malachite_nz::natural::arithmetic::mod_pow::{
    limbs_mod_pow, limbs_mod_pow_odd, limbs_mod_pow_odd_scratch_len,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_pair_gen_var_5, natural_pair_gen_var_8, natural_quadruple_gen_var_2,
    natural_quadruple_gen_var_3, natural_triple_gen_var_5, unsigned_vec_quadruple_gen_var_6,
    unsigned_vec_quadruple_gen_var_7,
};
use malachite_nz::test_util::natural::arithmetic::mod_pow::simple_binary_mod_pow;
use num::BigUint;
use std::panic::catch_unwind;
use std::str::FromStr;

fn verify_limbs_mod_pow(out: &[Limb], xs: &[Limb], es: &[Limb], ms: &[Limb], out_out: &[Limb]) {
    let exp = Natural::from_limbs_asc(es);
    let m = Natural::from_limbs_asc(ms);
    let x = Natural::from_limbs_asc(xs) % &m;
    let expected = (&x).mod_pow(&exp, &m);
    assert!(expected.mod_is_reduced(&m));
    assert_eq!(simple_binary_mod_pow(&x, &exp, &m), expected);
    let n = ms.len();
    assert_eq!(Natural::from_limbs_asc(&out_out[..n]), expected);
    assert_eq!(&out_out[n..], &out[n..]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_pow_odd() {
    let test = |out: &[Limb], xs: &[Limb], es: &[Limb], ms: &[Limb], out_out: &[Limb]| {
        let out_old = out;
        let mut out = out_old.to_vec();
        let mut scratch = vec![0; limbs_mod_pow_odd_scratch_len(ms.len())];
        limbs_mod_pow_odd(&mut out, xs, es, ms, &mut scratch);
        assert_eq!(out, out_out);
        verify_limbs_mod_pow(out_old, xs, es, ms, &out);
    };
    // - ms_len < REDC_1_TO_REDC_N_THRESHOLD
    // - ms_len == 1 in to_redc
    // - end >= len in get_bits
    // - width >= windowsize
    // - bit_index != 0
    // - !limbs_get_bit(es, bit_index - 1)
    // - bit_index >= windowsize
    // - limbs_cmp_same_length(out, ms) == Less
    test(&[10; 3], &[3], &[20], &[105], &[51, 10, 10]);
    // - ms_len != 1 in to_redc
    // - end < len in get_bits
    // - bit_index < windowsize
    test(
        &[10; 3],
        &[123, 456],
        &[789, 987],
        &[135, 797],
        &[2939877551, 399, 10],
    );
    // - limbs_cmp_same_length(out, ms) != Less
    test(&[10; 3], &[3], &[2], &[9], &[0, 10, 10]);
    // - ms_len >= REDC_1_TO_REDC_N_THRESHOLD
    // - REDC_1_TO_REDC_N_THRESHOLD <= ms_len < MUL_TOOM22_THRESHOLD in select_fns
    test(
        &[10; 102],
        &[
            15231838, 2040644427, 3019562008, 1849879669, 3035273653, 1126993058, 47231998,
            1272966966, 808826123, 2613679871, 3880209905, 533657556, 462055026, 1995462791,
            2669440556, 964537932, 1111256412, 3618168224, 2814460867, 3500086570, 4007554324,
            423675806, 4263852014, 939174181, 3372315870, 2404528019, 2462982568,
        ],
        &[4284714037, 4294504709, 3097020148, 1061316867],
        &[
            2895738059, 2222742945, 2200913266, 2474343961, 674884792, 3868751732, 3089689534,
            2430632097, 2488368713, 3062061319, 4067596626, 511264582, 1407628892, 2272907418,
            959402877, 3744259866, 3311299232, 2145791174, 43238938, 3833638835, 4110565129,
            4008973369, 104651202, 677542079, 758926920, 1101277253, 3001021931, 998015387,
            1742463066, 558329360, 2693367111, 3480565457, 3612553333, 1301555794, 519337581,
            3300908468, 3530322699, 2448060428, 4064907740, 2664586778, 1523503672, 3213102458,
            1257460118, 2076514388, 4071643827, 3592057262, 1402398574, 3017877057, 2055576849,
            732723305, 1414294091, 4223824602, 3201010967, 1895660785, 1135353915, 2148174613,
            4048519927, 121851082, 2198350375, 1783615559, 372909943, 614317668, 3372914919,
            2755021756, 4017557367, 2133054830, 1948457571, 1938410224, 3325075631, 3634267258,
            51102224, 2458277488, 3478591381, 1824685083, 3648635504, 481889501, 1100046228,
            4153007238, 3393775448, 288899148, 1691342799, 541535815, 3936683203, 3579439411,
            1281616696, 2097613162, 1182389689, 2985495353, 2822637621, 3933095442, 4138905254,
            4281995886, 1468542161, 503073229, 1438766160, 1146230629, 3365322628, 594370818,
            3970846533, 2993627984,
        ],
        &[
            3220228332, 1289044619, 4090486882, 2935175502, 1036421691, 322080634, 2520677041,
            782484792, 3367131407, 251234797, 3793043809, 2859808531, 1341835713, 1429457528,
            2859236711, 988242793, 3516913319, 3085390135, 1767135385, 3505108006, 941241196,
            1517169544, 581305334, 1456461198, 1628771306, 864062263, 818489872, 3761869500,
            808518712, 1576582852, 3807006337, 2323525681, 106379360, 3251953336, 2046814458,
            2437108016, 1297677798, 129047745, 3490510139, 1806687715, 3213874668, 139669025,
            919576894, 2643933393, 3980690050, 2652575541, 1927756447, 2123306615, 440563656,
            2040387516, 4243697106, 428296898, 915674737, 498045820, 3130354008, 1193458405,
            3406781642, 3252972448, 201621523, 1862360144, 2039264045, 2329252476, 2686526115,
            1116276280, 287363272, 4049536496, 3211163886, 2780240991, 3185623463, 3699825246,
            260938527, 3712407400, 50158619, 4082561437, 3981632240, 1645303993, 1388449088,
            1723061239, 1436532661, 1932705168, 3620788285, 4096149072, 2319194551, 2203579540,
            1766526917, 783820005, 3067485228, 3442142247, 2486918849, 2955962844, 1280634082,
            3907853014, 1337542963, 2795904203, 1989841069, 973220557, 1353182775, 2705903522,
            3829991598, 71670781, 10, 10,
        ],
    );
    // - ms_len >= MUL_TOOM22_THRESHOLD in select_fns
    test(
        &[10; 132],
        &[
            2688238368, 1370389694, 2050030377, 2515208601, 2857390881, 4197651524, 1239390266,
            1406670778, 2579838638, 295836633, 3196840122, 3842197805, 3093150937, 3921979512,
            3334864271, 4051787844, 1715354699, 784724006, 2641707538, 91367600, 564566325,
            4232021563, 3303269258, 1546428796, 4081815008, 2772422919, 3080061263, 3655857709,
            3221167157, 3188437627, 3509421900, 2117096697, 399342008, 595809629, 3677310060,
            4115179023, 755358759, 2356175810, 1130123131, 1730880525, 295144730, 3749456557,
        ],
        &[
            870678421, 2796837091, 3293178107, 1768925464, 1619766842, 2289477468, 1997089941,
            1122217361, 1351469882, 2292919231, 1820507033, 811208831, 2962958283, 49325855,
            904973661, 2650666234, 38738475, 1350510862, 3541603511, 1957000434, 626956257,
            3679451040, 455567141, 2358641221, 1772224239, 663265109,
        ],
        &[
            3910996289, 2816958662, 2867701269, 2489588705, 4139766686, 68347912, 3545464089,
            186840093, 4192695275, 3384626828, 1867478774, 3430502357, 2070259035, 1818903078,
            185487092, 1945164940, 198992488, 1340594809, 2884280378, 2659028966, 2731883802,
            3386600963, 4226259957, 1351345124, 1596030101, 1412857735, 3371378007, 1255307044,
            3627261742, 2728165048, 3740045608, 3893125603, 2353417837, 3173894525, 1712654005,
            3756974619, 1870314396, 3071976119, 399143608, 2618882156, 2758650080, 1057786871,
            2222605504, 3375739680, 696795589, 3386974205, 359484891, 309512373, 299872079,
            2278512560, 532043407, 1466190502, 733728197, 3362102523, 2180739566, 3829805290,
            397079472, 3354014956, 1906213944, 3156244881, 3388567106, 369361961, 264909755,
            3405203581, 2313150087, 3156935606, 1429648656, 3898358330, 1257746430, 1594676943,
            170660532, 2745162133, 2864414636, 2142084396, 2569588942, 3853728956, 2233612974,
            152936585, 1575814183, 519325569, 2275674013, 1085948185, 2072284470, 3541789119,
            4034514248, 1610140625, 3875476257, 1035874049, 3543420948, 1561693735, 966255080,
            936126918, 2523487827, 2895401694, 1142029713, 862454352, 3620226631, 69690788,
            3246184999, 485393839, 3514148744, 2848543979, 4007221509, 175089578, 1517243713,
            1261753161, 3257856227, 116328541, 2289524271, 3087481870, 3446271192, 1201816042,
            3910458324, 1249156998, 1230723249, 3753644481, 1117866811, 983288816, 2732615778,
            743509902, 3225264110, 1171181988, 946574108, 1804691841, 3423828869, 959970900,
            2507443216, 395931811, 1259542577, 2396067278,
        ],
        &[
            1025852633, 79270077, 3301788792, 1386637604, 192346455, 1456906572, 69195851,
            3129723136, 2519645061, 2541961631, 1794686607, 4106028394, 2701350434, 1348632655,
            1867572334, 1249803339, 1440960895, 250478100, 2889620165, 961879123, 1291616398,
            432573079, 867602404, 778350117, 4216286390, 4226436012, 2329382459, 2871255036,
            2542841218, 1342004849, 1363690216, 22108213, 4073323393, 4067883423, 3008421638,
            2102737805, 3312566592, 3539262183, 4034305910, 1392816118, 2717447275, 2310567812,
            2894277860, 2383199219, 4103937620, 708098321, 2949556562, 4211952216, 3158171510,
            4137870672, 3903904439, 4282380128, 855731951, 659589583, 2557106859, 576388003,
            313616280, 331520688, 1277940989, 2708897033, 533029719, 487352515, 1949824586,
            505072740, 4038013897, 2298668550, 1178798567, 1512613139, 4068988335, 2153062284,
            4118780280, 597996449, 1606074452, 2335555420, 2054577350, 3045549370, 2701737868,
            3618701098, 3636119053, 1005071332, 2303326104, 51535250, 3397405829, 3833534865,
            2425051659, 2190540066, 4180788716, 2608530236, 1439369848, 2112154998, 2289724846,
            2610833560, 808802563, 2996847623, 3253345649, 2123523552, 4214461433, 1492789177,
            3467294232, 2909754357, 3226703424, 619540384, 1914094063, 1208434472, 3653698470,
            753691175, 2501672947, 4171705775, 3829738927, 1900544129, 2509951656, 1518574100,
            2809954004, 2621465363, 2267697036, 3991643575, 869359274, 502886394, 1517544442,
            868396157, 2801850027, 190888001, 4244506669, 3019225248, 1504559895, 3027372440,
            1152465509, 3085485694, 2286582782, 1487765908, 10, 10,
        ],
    );
    // - xs longer than ms
    test(&[10; 3], &[123, 456], &[20], &[105], &[36, 10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_pow_odd_fail_1() {
    let out = &mut [10];
    let mut scratch = vec![0; limbs_mod_pow_odd_scratch_len(2)];
    limbs_mod_pow_odd(out, &[123, 456], &[789, 987], &[135, 797], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_pow_odd_fail_2() {
    let out = &mut [10; 3];
    let mut scratch = vec![0; limbs_mod_pow_odd_scratch_len(2)];
    limbs_mod_pow_odd(out, &[], &[789, 987], &[135, 797], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_pow_odd_fail_3() {
    let out = &mut [10; 3];
    let mut scratch = vec![0; limbs_mod_pow_odd_scratch_len(2)];
    limbs_mod_pow_odd(out, &[123, 456], &[], &[135, 797], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_pow_odd_fail_4() {
    let out = &mut [10; 3];
    let mut scratch = vec![0; limbs_mod_pow_odd_scratch_len(2)];
    limbs_mod_pow_odd(out, &[123, 456], &[789, 987], &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_pow_odd_fail_5() {
    let out = &mut [10; 3];
    let mut scratch = vec![0; limbs_mod_pow_odd_scratch_len(2)];
    limbs_mod_pow_odd(out, &[123, 456], &[789, 987], &[136, 797], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_pow_odd_fail_6() {
    let out = &mut [10; 3];
    let mut scratch = vec![0; limbs_mod_pow_odd_scratch_len(1)];
    limbs_mod_pow_odd(out, &[3], &[0], &[9], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_pow_odd_fail_7() {
    let out = &mut [10; 3];
    let mut scratch = vec![0; limbs_mod_pow_odd_scratch_len(1)];
    limbs_mod_pow_odd(out, &[3], &[1], &[9], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_pow() {
    let test = |out: &[Limb], xs: &[Limb], es: &[Limb], ms: &[Limb], out_out: &[Limb]| {
        let out_old = out;
        let mut out = out_old.to_vec();
        limbs_mod_pow(&mut out, xs, es, ms);
        assert_eq!(out, out_out);
        verify_limbs_mod_pow(out_old, xs, es, ms, &out);
    };
    // - ms[0].odd()
    // - ms_zero_len == 0
    test(&[10; 3], &[3], &[20], &[105], &[51, 10, 10]);
    test(&[10; 3], &[4], &[20], &[105], &[16, 10, 10]);
    test(&[10; 3], &[4], &[1000000], &[3], &[1, 10, 10]);
    // - ms[0].even()
    // - ms_zero_len != 0
    // - xs_len >= ms_zero_len
    // - xs[0].odd()
    // - do_pow_low
    // - ms_nonzero_len >= ms_zero_len
    // - ms_twos != 0
    test(&[10; 3], &[3], &[1000000], &[4], &[1, 10, 10]);
    // - xs[0].even()
    // - es_len == 1
    // - es[0].wrapping_mul(bits) >= t
    // - !do_pow_low
    test(&[10; 3], &[4], &[1000000], &[6], &[4, 10, 10]);
    // - ms[ms_zero_len] == 0
    // - xs_len < ms_zero_len
    // - ms_nonzero_len < ms_zero_len
    test(&[10; 3], &[4], &[1000000], &[0, 6], &[0, 4, 10]);
    // - es_len > 1
    test(&[10; 3], &[4], &[1, 1], &[0, 6], &[0, 4, 10]);
    // - ms_twos == 0
    test(&[10; 4], &[1], &[2], &[0, 1], &[1, 0, 10, 10]);
    // - es[0].wrapping_mul(bits) < t
    test(&[10; 4], &[2], &[2], &[0, 1], &[4, 0, 10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_pow_fail_1() {
    let out = &mut [10];
    limbs_mod_pow(out, &[123, 456], &[789, 987], &[135, 797]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_pow_fail_2() {
    let out = &mut [10; 3];
    limbs_mod_pow(out, &[], &[789, 987], &[135, 797]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_pow_fail_3() {
    let out = &mut [10; 3];
    limbs_mod_pow(out, &[123, 456], &[], &[135, 797]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_pow_fail_4() {
    let out = &mut [10; 3];
    limbs_mod_pow(out, &[123, 456], &[789, 987], &[]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_pow_fail_5() {
    let out = &mut [10; 3];
    limbs_mod_pow(out, &[3], &[0], &[9]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_pow_fail_6() {
    let out = &mut [10; 3];
    limbs_mod_pow(out, &[3], &[1], &[9]);
}

#[test]
fn test_mod_pow() {
    let test = |r, s, t, out| {
        let u = Natural::from_str(r).unwrap();
        let exp = Natural::from_str(s).unwrap();
        let m = Natural::from_str(t).unwrap();

        assert!(u.mod_is_reduced(&m));

        let mut n = u.clone();
        n.mod_pow_assign(exp.clone(), m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert!(n.mod_is_reduced(&m));

        let mut n = u.clone();
        n.mod_pow_assign(&exp, m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n.mod_pow_assign(exp.clone(), &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n.mod_pow_assign(&exp, &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_pow(exp.clone(), m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_pow(exp.clone(), m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_pow(&exp, m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_pow(&exp, m.clone());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_pow(exp.clone(), &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_pow(exp.clone(), &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_pow(&exp, &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_pow(&exp, &m);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0", "1", "0");
    test("0", "0", "10", "1");
    test("0", "1", "10", "0");
    test("2", "10", "10", "4");
    test("4", "13", "497", "445");
    test("10", "1000", "30", "10");
    test("2", "340", "341", "1");
    test("5", "216", "217", "1");
    test("2", "1000000", "1000000000", "747109376");
    test(
        "1234567890",
        "1000000000",
        "12345678987654321",
        "10973935643347062",
    );
}

#[test]
fn mod_pow_fail() {
    assert_panic!(Natural::ZERO.mod_pow(Natural::from(3u32), Natural::ZERO));
    assert_panic!(Natural::from(30u32).mod_pow(Natural::from(3u32), Natural::ONE));

    assert_panic!(Natural::ZERO.mod_pow(Natural::from(3u32), &Natural::ZERO));
    assert_panic!(Natural::from(30u32).mod_pow(Natural::from(3u32), &Natural::ONE));

    assert_panic!(Natural::ZERO.mod_pow(&Natural::from(3u32), Natural::ZERO));
    assert_panic!(Natural::from(30u32).mod_pow(&Natural::from(3u32), Natural::ONE));

    assert_panic!(Natural::ZERO.mod_pow(&Natural::from(3u32), &Natural::ZERO));
    assert_panic!(Natural::from(30u32).mod_pow(&Natural::from(3u32), &Natural::ONE));

    assert_panic!((&Natural::ZERO).mod_pow(Natural::from(3u32), Natural::ZERO));
    assert_panic!((&Natural::from(30u32)).mod_pow(Natural::from(3u32), Natural::ONE));

    assert_panic!((&Natural::ZERO).mod_pow(Natural::from(3u32), &Natural::ZERO));
    assert_panic!((&Natural::from(30u32)).mod_pow(Natural::from(3u32), &Natural::ONE));

    assert_panic!((&Natural::ZERO).mod_pow(&Natural::from(3u32), Natural::ZERO));
    assert_panic!((&Natural::from(30u32)).mod_pow(&Natural::from(3u32), Natural::ONE));

    assert_panic!((&Natural::ZERO).mod_pow(&Natural::from(3u32), &Natural::ZERO));
    assert_panic!((&Natural::from(30u32)).mod_pow(&Natural::from(3u32), &Natural::ONE));

    assert_panic!({
        let mut x = Natural::ZERO;
        x.mod_pow_assign(Natural::from(3u32), Natural::ZERO);
    });
    assert_panic!({
        let mut x = Natural::from(30u32);
        x.mod_pow_assign(Natural::from(3u32), Natural::ONE);
    });

    assert_panic!({
        let mut x = Natural::ZERO;
        x.mod_pow_assign(Natural::from(3u32), &Natural::ZERO);
    });
    assert_panic!({
        let mut x = Natural::from(30u32);
        x.mod_pow_assign(Natural::from(3u32), Natural::ONE);
    });

    assert_panic!({
        let mut x = Natural::ZERO;
        x.mod_pow_assign(&Natural::from(3u32), Natural::ZERO);
    });
    assert_panic!({
        let mut x = Natural::from(30u32);
        x.mod_pow_assign(Natural::from(3u32), Natural::ONE);
    });

    assert_panic!({
        let mut x = Natural::ZERO;
        x.mod_pow_assign(&Natural::from(3u32), &Natural::ZERO);
    });
    assert_panic!({
        let mut x = Natural::from(30u32);
        x.mod_pow_assign(Natural::from(3u32), Natural::ONE);
    });
}

#[test]
fn limbs_mod_pow_odd_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_quadruple_gen_var_7().test_properties_with_config(
        &config,
        |(mut out, xs, es, ms)| {
            let out_old = out.clone();
            let mut scratch = vec![0; limbs_mod_pow_odd_scratch_len(ms.len())];
            limbs_mod_pow_odd(&mut out, &xs, &es, &ms, &mut scratch);
            verify_limbs_mod_pow(&out_old, &xs, &es, &ms, &out);
        },
    );
}

#[test]
fn limbs_mod_pow_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_quadruple_gen_var_6().test_properties_with_config(
        &config,
        |(mut out, xs, es, ms)| {
            let out_old = out.clone();
            limbs_mod_pow(&mut out, &xs, &es, &ms);
            verify_limbs_mod_pow(&out_old, &xs, &es, &ms, &out);
        },
    );
}

#[test]
fn mod_pow_properties() {
    natural_triple_gen_var_5().test_properties(|(x, exp, m)| {
        assert!(x.mod_is_reduced(&m));
        let power_val_val_val = x.clone().mod_pow(exp.clone(), m.clone());
        let power_val_ref_val = x.clone().mod_pow(&exp, m.clone());
        let power_ref_val_val = (&x).mod_pow(exp.clone(), m.clone());
        let power_ref_ref_val = (&x).mod_pow(&exp, m.clone());
        let power_val_val_ref = x.clone().mod_pow(exp.clone(), &m);
        let power_val_ref_ref = x.clone().mod_pow(&exp, &m);
        let power_ref_val_ref = (&x).mod_pow(exp.clone(), &m);
        let power = (&x).mod_pow(&exp, &m);
        assert!(power_val_val_val.is_valid());
        assert!(power_val_ref_val.is_valid());
        assert!(power_ref_val_val.is_valid());
        assert!(power_val_val_ref.is_valid());
        assert!(power_val_val_ref.is_valid());
        assert!(power_val_ref_ref.is_valid());
        assert!(power_ref_val_ref.is_valid());
        assert!(power.is_valid());
        assert!(power.mod_is_reduced(&m));
        assert_eq!(power_val_val_val, power);
        assert_eq!(power_val_ref_val, power);
        assert_eq!(power_ref_val_val, power);
        assert_eq!(power_ref_ref_val, power);
        assert_eq!(power_val_val_ref, power);
        assert_eq!(power_val_ref_ref, power);
        assert_eq!(power_ref_val_ref, power);

        let mut mut_x = x.clone();
        mut_x.mod_pow_assign(exp.clone(), m.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, power);
        let mut mut_x = x.clone();
        mut_x.mod_pow_assign(&exp, m.clone());
        assert_eq!(mut_x, power);
        assert!(mut_x.is_valid());
        let mut mut_x = x.clone();
        mut_x.mod_pow_assign(exp.clone(), &m);
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, power);
        let mut mut_x = x.clone();
        mut_x.mod_pow_assign(&exp, &m);
        assert_eq!(mut_x, power);
        assert!(mut_x.is_valid());

        let num_power = BigUint::from(&x).modpow(&BigUint::from(&exp), &BigUint::from(&m));
        assert_eq!(Natural::from(&num_power), power);
        let rug_power = rug::Integer::from(&x)
            .pow_mod(&rug::Integer::from(&exp), &rug::Integer::from(&m))
            .unwrap();
        assert_eq!(Natural::exact_from(&rug_power), power);

        if exp.even() {
            assert_eq!(x.mod_neg(&m).mod_pow(exp, m), power);
        } else {
            assert_eq!(x.mod_neg(&m).mod_pow(exp, &m), power.mod_neg(m));
        }
    });

    natural_pair_gen_var_5().test_properties(|(exp, m)| {
        assert_eq!(
            Natural::ZERO.mod_pow(&exp, &m),
            Natural::from(exp == 0 && m != 1),
        );
        if m != 1 {
            assert_eq!(Natural::ONE.mod_pow(exp, m), 1);
        }
    });

    natural_pair_gen_var_8().test_properties(|(ref x, ref m)| {
        assert_eq!(x.mod_pow(Natural::ZERO, m), Natural::from(*m != 1));
        assert_eq!(x.mod_pow(Natural::ONE, m), *x);
        assert_eq!(x.mod_pow(Natural::TWO, m), x.mod_mul(x, m));
    });

    natural_quadruple_gen_var_2().test_properties(|(ref x, ref y, ref exp, ref m)| {
        assert_eq!(
            x.mod_mul(y, m).mod_pow(exp, m),
            x.mod_pow(exp, m).mod_mul(y.mod_pow(exp, m), m)
        );
    });

    natural_quadruple_gen_var_3().test_properties(|(ref x, ref e, ref f, ref m)| {
        assert_eq!(
            x.mod_pow(e + f, m),
            x.mod_pow(e, m).mod_mul(x.mod_pow(f, m), m)
        );
        assert_eq!(x.mod_pow(e * f, m), x.mod_pow(e, m).mod_pow(f, m));
    });

    unsigned_triple_gen_var_15::<Limb, u64>().test_properties(|(x, exp, m)| {
        assert_eq!(
            x.mod_pow(exp, m),
            Natural::from(x).mod_pow(Natural::from(exp), Natural::from(m))
        );
    });
}
