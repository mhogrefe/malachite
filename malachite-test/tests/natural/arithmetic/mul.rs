use common::test_properties_custom_scale;
use malachite_base::num::{One, Zero};
use malachite_nz::natural::arithmetic::mul::{
    _limbs_mul_to_out_basecase, _limbs_mul_to_out_toom_22, _limbs_mul_to_out_toom_22_scratch_size,
    _limbs_mul_to_out_toom_32, _limbs_mul_to_out_toom_32_scratch_size, _limbs_mul_to_out_toom_33,
    _limbs_mul_to_out_toom_33_scratch_size, _limbs_mul_to_out_toom_42,
    _limbs_mul_to_out_toom_42_scratch_size, _limbs_mul_to_out_toom_43,
    _limbs_mul_to_out_toom_43_scratch_size, mpn_mul,
};
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::arithmetic::mul::{MUL_TOOM22_THRESHOLD, MUL_TOOM33_THRESHOLD};
use malachite_nz::natural::Natural;
use malachite_nz::platform::{DoubleLimb, Limb};
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::base::{
    pairs_of_unsigneds, triples_of_unsigned_vec_var_10, triples_of_unsigned_vec_var_11,
    triples_of_unsigned_vec_var_12, triples_of_unsigned_vec_var_13, triples_of_unsigned_vec_var_14,
    triples_of_unsigned_vec_var_15,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_unsigned, pairs_of_naturals, triples_of_naturals,
};
use num::BigUint;
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_to_out() {
    let test = |xs, ys, out_before: &[Limb], highest_result_limb, out_after| {
        let mut out = out_before.to_vec();
        _limbs_mul_to_out_basecase(&mut out, xs, ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        assert_eq!(mpn_mul(&mut out, xs, ys), highest_result_limb);
        assert_eq!(out, out_after);
    };
    test(&[2], &[3], &[10, 10, 10], 0, vec![6, 0, 10]);
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        &[5, 5, 5, 5, 5, 5, 5, 5],
        0,
        vec![1, 3, 6, 5, 3, 0, 5, 5],
    );
    test(
        &[1, 2, 3],
        &[6, 7],
        &[0, 0, 0, 0, 0],
        0,
        vec![6, 19, 32, 21, 0],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10, 10, 10, 10],
        0,
        vec![10_200, 20_402, 30_605, 20_402, 10_200, 0, 10],
    );
    test(
        &[0xffff_ffff],
        &[1],
        &[10, 10, 10],
        0,
        vec![0xffff_ffff, 0, 10],
    );
    test(
        &[0xffff_ffff],
        &[0xffff_ffff],
        &[10, 10, 10, 10],
        0xffff_fffe,
        vec![1, 0xffff_fffe, 10, 10],
    );
    test(
        &[0xffff_ffff, 0xffff_ffff, 0xffff_ffff],
        &[0xffff_ffff, 0xffff_ffff, 0xffff_ffff],
        &[10, 10, 10, 10, 10, 10],
        0xffff_ffff,
        vec![1, 0, 0, 0xffff_fffe, 0xffff_ffff, 0xffff_ffff],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: out_limbs.len() >= xs_len + ys_len")]
fn limbs_mul_to_out_fail_1() {
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_to_out_basecase(&mut out, &[6, 7, 8], &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: xs_len >= ys_len")]
fn limbs_mul_to_out_fail_2() {
    let mut out = vec![10, 10, 10, 10, 10];
    _limbs_mul_to_out_basecase(&mut out, &[6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: `(left != right)")]
fn limbs_mul_to_out_fail_3() {
    let mut out = vec![10, 10, 10];
    _limbs_mul_to_out_basecase(&mut out, &[6, 7], &[]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_to_out_toom_22() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut scratch = vec![0; _limbs_mul_to_out_toom_22_scratch_size(xs.len())];
        let mut out = out_before.to_vec();
        _limbs_mul_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        _limbs_mul_to_out_toom_22(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    // s != n
    // !(xs0[s] == 0 && limbs_cmp_same_length(&xs0[..s], xs1) == Ordering::Less)
    // t != n
    // limbs_test_zero(&ys0[t..]) && limbs_cmp_same_length(&ys0[..t], ys1) != Ordering::Less
    // s <= t
    // !v_neg_1_neg
    // carry <= 2
    test(
        vec![2, 3, 4],
        vec![3, 4, 5],
        vec![10, 10, 10, 10, 10, 10],
        vec![6, 17, 34, 31, 20, 0],
    );
    // xs0[s] == 0 && limbs_cmp_same_length(&xs0[..s], xs1) == Ordering::Less
    // v_neg_1_neg
    //test(&[2, 0, 4], &[3, 4, 5], &[10, 10, 10, 10, 10, 10], vec![6, 8, 22, 16, 20, 0]);
    test(
        vec![1, 1, 1],
        vec![1, 2, 3],
        vec![5, 5, 5, 5, 5, 5, 5, 5],
        vec![1, 3, 6, 5, 3, 0, 5, 5],
    );
    // s == n
    // limbs_cmp_same_length(ys0, ys1) != Ordering::Less
    // t == n
    // limbs_cmp_same_length(ys0, ys1) == Ordering::Less
    test(
        vec![1, 1, 1, 1],
        vec![1, 2, 3, 4],
        vec![5, 5, 5, 5, 5, 5, 5, 5],
        vec![1, 3, 6, 10, 9, 7, 4, 0],
    );
    // limbs_cmp_same_length(&a0[..n], &a1[..n]) == Ordering::Less
    // limbs_cmp_same_length(&b0[..n], &b1[..n]) != Ordering::Less
    test(
        vec![1, 2, 3, 4],
        vec![1, 1, 1, 1],
        vec![5, 5, 5, 5, 5, 5, 5, 5],
        vec![1, 3, 6, 10, 9, 7, 4, 0],
    );
    // limbs_test_zero(&ys0[t..]) && limbs_cmp_same_length(&ys0[..t], ys1) == Ordering::Less
    test(
        vec![1, 2, 3, 4, 5],
        vec![1, 0, 0, 4],
        vec![5, 5, 5, 5, 5, 5, 5, 5, 5],
        vec![1, 2, 3, 8, 13, 12, 16, 20, 0],
    );
    // s > t
    // limbs_mul_to_out_basecase in limbs_mul_to_out_toom_22_recursive
    test(
        vec![1, 1, 1, 1],
        vec![1, 2, 3],
        vec![5, 5, 5, 5, 5, 5, 5, 5],
        vec![1, 3, 6, 6, 5, 3, 0, 5],
    );
    test(
        vec![100, 101, 102],
        vec![102, 101, 100],
        vec![10, 10, 10, 10, 10, 10, 10],
        vec![10_200, 20_402, 30_605, 20_402, 10_200, 0, 10],
    );
    // limbs_mul_to_out_basecase in limbs_mul_same_length_to_out_toom_22_recursive
    test(
        vec![0xffff_ffff, 0xffff_ffff, 0xffff_ffff],
        vec![0xffff_ffff, 0xffff_ffff, 0xffff_ffff],
        vec![10, 10, 10, 10, 10, 10],
        vec![1, 0, 0, 0xffff_fffe, 0xffff_ffff, 0xffff_ffff],
    );
    let xs = vec![
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294950911, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 536870911, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ];
    let ys = vec![
        4294967295, 4294967295, 4294963199, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        268435455, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
    ];
    let out_len = xs.len() + ys.len();
    // carry > 2
    test(
        xs,
        ys,
        vec![10; out_len],
        vec![
            1, 0, 4096, 0, 0, 0, 0, 0, 16384, 0, 67108864, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            4026531840, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295, 4294967295, 3758095359, 4294967295, 4294967295, 4294966783, 4294967295,
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295, 4294967295, 4294967295, 33554431, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
    );
    let limit = 2 * MUL_TOOM22_THRESHOLD;
    let mut long_xs = Vec::new();
    let mut long_ys = Vec::new();
    for i in 0..limit {
        long_xs.push(i as Limb + 1);
        long_ys.push((limit - i) as Limb);
    }
    let long_out_limbs = vec![10; 2 * limit];
    // limbs_mul_to_out_toom_22 in limbs_mul_same_length_to_out_toom_22_recursive
    test(
        long_xs,
        long_ys,
        long_out_limbs,
        vec![
            60, 179, 356, 590, 880, 1225, 1624, 2076, 2580, 3135, 3740, 4394, 5096, 5845, 6640,
            7480, 8364, 9291, 10260, 11270, 12320, 13409, 14536, 15700, 16900, 18135, 19404, 20706,
            22040, 23405, 24800, 26224, 27676, 29155, 30660, 32190, 33744, 35321, 36920, 38540,
            40180, 41839, 43516, 45210, 46920, 48645, 50384, 52136, 53900, 55675, 57460, 59254,
            61056, 62865, 64680, 66500, 68324, 70151, 71980, 73810, 71980, 70151, 68324, 66500,
            64680, 62865, 61056, 59254, 57460, 55675, 53900, 52136, 50384, 48645, 46920, 45210,
            43516, 41839, 40180, 38540, 36920, 35321, 33744, 32190, 30660, 29155, 27676, 26224,
            24800, 23405, 22040, 20706, 19404, 18135, 16900, 15700, 14536, 13409, 12320, 11270,
            10260, 9291, 8364, 7480, 6640, 5845, 5096, 4394, 3740, 3135, 2580, 2076, 1624, 1225,
            880, 590, 356, 179, 60, 0,
        ],
    );
    let mut long_xs = Vec::new();
    let mut long_ys = Vec::new();
    for i in 0..limit + 2 {
        long_xs.push(i as Limb + 1);
    }
    for i in 0..limit + 1 {
        long_ys.push((limit + 1 - i) as Limb);
    }
    let long_out_limbs = vec![10; 2 * limit + 3];
    // limbs_mul_to_out_toom_22 in limbs_mul_to_out_toom_22_recursive
    test(
        long_xs,
        long_ys,
        long_out_limbs,
        vec![
            61, 182, 362, 600, 895, 1246, 1652, 2112, 2625, 3190, 3806, 4472, 5187, 5950, 6760,
            7616, 8517, 9462, 10450, 11480, 12551, 13662, 14812, 16000, 17225, 18486, 19782, 21112,
            22475, 23870, 25296, 26752, 28237, 29750, 31290, 32856, 34447, 36062, 37700, 39360,
            41041, 42742, 44462, 46200, 47955, 49726, 51512, 53312, 55125, 56950, 58786, 60632,
            62487, 64350, 66220, 68096, 69977, 71862, 73750, 75640, 77531, 79422, 77470, 75520,
            73573, 71630, 69692, 67760, 65835, 63918, 62010, 60112, 58225, 56350, 54488, 52640,
            50807, 48990, 47190, 45408, 43645, 41902, 40180, 38480, 36803, 35150, 33522, 31920,
            30345, 28798, 27280, 25792, 24335, 22910, 21518, 20160, 18837, 17550, 16300, 15088,
            13915, 12782, 11690, 10640, 9633, 8670, 7752, 6880, 6055, 5278, 4550, 3872, 3245, 2670,
            2148, 1680, 1267, 910, 610, 368, 185, 62, 0,
        ],
    );
    let mut long_xs = Vec::new();
    let mut long_ys = Vec::new();
    assert_eq!(MUL_TOOM22_THRESHOLD, 30);
    // xs_len == 76, ys_len == 68 satisfy s > t && t >= MUL_TOOM22_THRESHOLD && 4 * s >= 5 * t
    for i in 0..76 {
        long_xs.push(i as Limb + 1);
    }
    for i in 0..68 {
        long_ys.push((68 - i) as Limb);
    }
    let long_out_limbs = vec![10; 144];
    // limbs_mul_to_out_toom_32 in limbs_mul_to_out_toom_22_recursive
    test(
        long_xs,
        long_ys,
        long_out_limbs,
        vec![
            68, 203, 404, 670, 1000, 1393, 1848, 2364, 2940, 3575, 4268, 5018, 5824, 6685, 7600,
            8568, 9588, 10659, 11780, 12950, 14168, 15433, 16744, 18100, 19500, 20943, 22428,
            23954, 25520, 27125, 28768, 30448, 32164, 33915, 35700, 37518, 39368, 41249, 43160,
            45100, 47068, 49063, 51084, 53130, 55200, 57293, 59408, 61544, 63700, 65875, 68068,
            70278, 72504, 74745, 77000, 79268, 81548, 83839, 86140, 88450, 90768, 93093, 95424,
            97760, 100100, 102443, 104788, 107134, 109480, 111826, 114172, 116518, 118864, 121210,
            123556, 125902, 123012, 120131, 117260, 114400, 111552, 108717, 105896, 103090, 100300,
            97527, 94772, 92036, 89320, 86625, 83952, 81302, 78676, 76075, 73500, 70952, 68432,
            65941, 63480, 61050, 58652, 56287, 53956, 51660, 49400, 47177, 44992, 42846, 40740,
            38675, 36652, 34672, 32736, 30845, 29000, 27202, 25452, 23751, 22100, 20500, 18952,
            17457, 16016, 14630, 13300, 12027, 10812, 9656, 8560, 7525, 6552, 5642, 4796, 4015,
            3300, 2652, 2072, 1561, 1120, 750, 452, 227, 76, 0,
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: s > 0 && (s == n || s == n - 1)")]
fn limbs_mul_to_out_toom_22_fail_1() {
    let mut scratch = vec![];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_to_out_toom_22(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: xs_len >= ys_len")]
fn limbs_mul_to_out_toom_22_fail_2() {
    let mut scratch = vec![];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_to_out_toom_22(&mut out, &[6, 7, 8], &[1, 2, 3, 4], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: 0 < t && t <= s")]
fn limbs_mul_to_out_toom_22_fail_3() {
    let mut scratch = vec![];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_to_out_toom_22(&mut out, &[6, 7, 8, 9], &[1, 2], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: 0 < t && t <= s")]
fn limbs_mul_to_out_toom_22_fail_4() {
    let mut scratch = vec![];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_to_out_toom_22(&mut out, &[6, 7, 8], &[1, 2], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: mid <= len")]
fn limbs_mul_to_out_toom_22_fail_5() {
    let mut scratch = vec![];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_to_out_toom_22(&mut out, &[6, 7, 8], &[1, 2, 3], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: ys_len >= n")]
fn limbs_mul_to_out_toom_22_fail_6() {
    let mut scratch = vec![];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_to_out_toom_22(&mut out, &[6, 7, 8], &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: mid <= len")]
fn limbs_mul_to_out_toom_22_fail_7() {
    let mut scratch = vec![];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_to_out_toom_22(&mut out, &[6, 7, 8], &[1, 2, 3], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_to_out_toom_32() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        _limbs_mul_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; _limbs_mul_to_out_toom_32_scratch_size(xs.len(), ys.len())];
        _limbs_mul_to_out_toom_32(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    // !(ap1_hi == 0 && limbs_cmp_same_length(ap1, xs1) == Ordering::Less)
    // t == n
    // limbs_cmp_same_length(ys0, ys1) == Ordering::Less
    // ap1_hi != 1 and ap1_hi != 2
    // !bp1_hi
    // hi == 0 first time
    // v_neg_1_neg
    // s <= t
    // s + t > n
    // hi >= 0 second time
    test(
        vec![2, 3, 4, 5, 6, 7],
        vec![3, 4, 5, 6],
        vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10],
        vec![6, 17, 34, 58, 76, 94, 88, 71, 42, 0],
    );
    // limbs_cmp_same_length(ys0, ys1) != Ordering::Less
    // ap1_hi == 2
    // bp1_hi
    // !v_neg_1_neg
    test(
        vec![
            0xffff_ffff,
            0xffff_ffff,
            0xffff_ffff,
            0xffff_ffff,
            0xffff_ffff,
            0xffff_ffff,
        ],
        vec![0xffff_ffff, 0xffff_ffff, 0xffff_ffff, 0xffff_ffff],
        vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10],
        vec![
            1, 0, 0, 0, 4294967295, 4294967295, 4294967294, 4294967295, 4294967295, 4294967295,
        ],
    );
    // ap1_hi == 0 && limbs_cmp_same_length(ap1, xs1) == Ordering::Less
    test(
        vec![0, 0, 1, 1, 0, 1],
        vec![0, 0, 0, 1],
        vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10],
        vec![0, 0, 0, 0, 0, 1, 1, 0, 1, 0],
    );
    // t != n
    // limbs_test_zero(&ys0[t..]) && limbs_cmp_same_length(&ys0[..t], ys1) == Ordering::Less
    // s + t <= n
    test(
        vec![0, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 1],
        vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    );
    // !(limbs_test_zero(&ys0[t..]) && limbs_cmp_same_length(&ys0[..t], ys1) == Ordering::Less)
    test(
        vec![0, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 1, 0, 1],
        vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0],
    );
    // s > t
    test(
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![9, 8, 7, 6, 5],
        vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10],
        vec![9, 26, 50, 80, 115, 150, 185, 220, 255, 200, 146, 94, 45, 0],
    );
    // ap1_hi == 1
    test(
        vec![
            2543705880, 1859419010, 3343322808, 1165039137, 1872701663, 1957510151, 1589243046,
        ],
        vec![1919189400, 1295801997, 354566481, 1212146910, 1886225431],
        vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10],
        vec![
            1753714240, 1114397484, 4100081063, 2352383720, 667557204, 920036609, 2291920497,
            3338154324, 3806846000, 1880963052, 291601955, 697949587, 10, 10,
        ],
    );
    // hi != 0 first time
    test(
        vec![
            706760835, 4153647095, 3843998199, 2077172825, 1158686949, 3157624247,
        ],
        vec![2847735618, 2779635711, 2471732382, 2655639495],
        vec![10; 10],
        vec![
            2814066374, 2022835469, 2101335047, 312674723, 2952296274, 1055977952, 590716674,
            290888444, 3944399226, 1952404077,
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: ys_len + 2 <= xs_len && xs_len + 6 <= 3 * ys_len")]
fn limbs_mul_to_out_toom_32_fail_1() {
    let mut scratch = vec![0; _limbs_mul_to_out_toom_32_scratch_size(1, 1)];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_to_out_toom_32(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: xs_len >= ys_len")]
fn limbs_mul_to_out_toom_32_fail_2() {
    let mut scratch = vec![0; _limbs_mul_to_out_toom_32_scratch_size(3, 4)];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_to_out_toom_32(&mut out, &[6, 7, 8], &[1, 2, 3, 4], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: ys_len + 2 <= xs_len && xs_len + 6 <= 3 * ys_len")]
fn limbs_mul_to_out_toom_32_fail_3() {
    let mut scratch = vec![0; _limbs_mul_to_out_toom_32_scratch_size(5, 4)];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_to_out_toom_32(&mut out, &[6, 7, 8, 9, 10], &[1, 2, 3, 4], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: ys_len + 2 <= xs_len && xs_len + 6 <= 3 * ys_len")]
fn limbs_mul_to_out_toom_32_fail_4() {
    let mut scratch = vec![0; _limbs_mul_to_out_toom_32_scratch_size(6, 3)];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_to_out_toom_32(&mut out, &[6, 7, 8, 9, 10, 11], &[1, 2, 3], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: ys_len + 2 <= xs_len && xs_len + 6 <= 3 * ys_len")]
fn limbs_mul_to_out_toom_32_fail_5() {
    let mut scratch = vec![0; _limbs_mul_to_out_toom_32_scratch_size(3, 0)];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_to_out_toom_32(&mut out, &[6, 7, 8], &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: out_limbs.len() >= xs_len + ys_len")]
fn limbs_mul_to_out_toom_32_fail_6() {
    let mut scratch = vec![0; _limbs_mul_to_out_toom_32_scratch_size(6, 4)];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_to_out_toom_32(&mut out, &[6, 7, 8, 9, 10, 11], &[1, 2, 3, 4], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_to_out_toom_33() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        _limbs_mul_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; _limbs_mul_to_out_toom_33_scratch_size(xs.len())];
        _limbs_mul_to_out_toom_33(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    // carry == 0 && limbs_cmp_same_length(&gp[..n], xs_1) == Ordering::Less
    // s != n
    // carry == 0 && limbs_cmp_same_length(&gp[..n], ys_1) == Ordering::Less
    // t != n
    // s <= t
    // !v_neg_1
    // two_r <= k + 1
    // _limbs_mul_to_out_basecase in _limbs_mul_to_out_toom_33_recursive
    test(
        vec![2, 3, 4, 5, 6],
        vec![3, 4, 5, 6, 7],
        vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10],
        vec![6, 17, 34, 58, 90, 94, 88, 71, 42, 0],
    );
    // s > t
    test(
        vec![2, 3, 4, 5, 6, 7],
        vec![3, 4, 5, 6, 7],
        vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10],
        vec![6, 17, 34, 58, 90, 115, 116, 106, 84, 49, 0],
    );
    // v_neg_1
    // two_r > k + 1
    test(
        vec![2, 3, 4, 5, 6, 7, 8, 9, 10],
        vec![3, 4, 5, 6, 7, 8, 9, 10],
        vec![10; 17],
        vec![
            6, 17, 34, 58, 90, 131, 182, 244, 296, 315, 320, 310, 284, 241, 180, 100, 0,
        ],
    );
    test(
        vec![3, 4, 5, 6, 7],
        vec![2, 3, 4, 5, 6],
        vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10],
        vec![6, 17, 34, 58, 90, 94, 88, 71, 42, 0],
    );
    // !(carry == 0 && limbs_cmp_same_length(&gp[..n], xs_1) == Ordering::Less)
    // !(carry == 0 && limbs_cmp_same_length(&gp[..n], ys_1) == Ordering::Less)
    test(
        vec![
            0xffff_ffff,
            0xffff_ffff,
            0xffff_ffff,
            0xffff_ffff,
            0xffff_ffff,
        ],
        vec![
            0xffff_ffff,
            0xffff_ffff,
            0xffff_ffff,
            0xffff_ffff,
            0xffff_ffff,
        ],
        vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10],
        vec![
            1, 0, 0, 0, 0, 4294967294, 4294967295, 4294967295, 4294967295, 4294967295,
        ],
    );
    let limit = 3 * MUL_TOOM22_THRESHOLD - 5;
    let mut long_xs = Vec::new();
    let mut long_ys = Vec::new();
    for i in 0..limit {
        long_xs.push(i as Limb + 1);
        long_ys.push((limit - i) as Limb);
    }
    let long_out_limbs = vec![10; 2 * limit];
    // _limbs_mul_to_out_toom_22 in _limbs_mul_to_out_toom_33_recursive
    test(
        long_xs,
        long_ys,
        long_out_limbs,
        vec![
            85, 254, 506, 840, 1255, 1750, 2324, 2976, 3705, 4510, 5390, 6344, 7371, 8470, 9640,
            10880, 12189, 13566, 15010, 16520, 18095, 19734, 21436, 23200, 25025, 26910, 28854,
            30856, 32915, 35030, 37200, 39424, 41701, 44030, 46410, 48840, 51319, 53846, 56420,
            59040, 61705, 64414, 67166, 69960, 72795, 75670, 78584, 81536, 84525, 87550, 90610,
            93704, 96831, 99990, 103180, 106400, 109649, 112926, 116230, 119560, 122915, 126294,
            129696, 133120, 136565, 140030, 143514, 147016, 150535, 154070, 157620, 161184, 164761,
            168350, 171950, 175560, 179179, 182806, 186440, 190080, 193725, 197374, 201026, 204680,
            208335, 204680, 201026, 197374, 193725, 190080, 186440, 182806, 179179, 175560, 171950,
            168350, 164761, 161184, 157620, 154070, 150535, 147016, 143514, 140030, 136565, 133120,
            129696, 126294, 122915, 119560, 116230, 112926, 109649, 106400, 103180, 99990, 96831,
            93704, 90610, 87550, 84525, 81536, 78584, 75670, 72795, 69960, 67166, 64414, 61705,
            59040, 56420, 53846, 51319, 48840, 46410, 44030, 41701, 39424, 37200, 35030, 32915,
            30856, 28854, 26910, 25025, 23200, 21436, 19734, 18095, 16520, 15010, 13566, 12189,
            10880, 9640, 8470, 7371, 6344, 5390, 4510, 3705, 2976, 2324, 1750, 1255, 840, 506, 254,
            85, 0,
        ],
    );
    let limit = 3 * MUL_TOOM33_THRESHOLD - 5;
    let mut long_xs = Vec::new();
    let mut long_ys = Vec::new();
    for i in 0..limit {
        long_xs.push(i as Limb + 1);
        long_ys.push((limit - i) as Limb);
    }
    let long_out_limbs = vec![10; 2 * limit];
    // _limbs_mul_to_out_toom_33 in _limbs_mul_to_out_toom_33_recursive
    test(
        long_xs,
        long_ys,
        long_out_limbs,
        vec![
            295, 884, 1766, 2940, 4405, 6160, 8204, 10536, 13155, 16060, 19250, 22724, 26481,
            30520, 34840, 39440, 44319, 49476, 54910, 60620, 66605, 72864, 79396, 86200, 93275,
            100620, 108234, 116116, 124265, 132680, 141360, 150304, 159511, 168980, 178710, 188700,
            198949, 209456, 220220, 231240, 242515, 254044, 265826, 277860, 290145, 302680, 315464,
            328496, 341775, 355300, 369070, 383084, 397341, 411840, 426580, 441560, 456779, 472236,
            487930, 503860, 520025, 536424, 553056, 569920, 587015, 604340, 621894, 639676, 657685,
            675920, 694380, 713064, 731971, 751100, 770450, 790020, 809809, 829816, 850040, 870480,
            891135, 912004, 933086, 954380, 975885, 997600, 1019524, 1041656, 1063995, 1086540,
            1109290, 1132244, 1155401, 1178760, 1202320, 1226080, 1250039, 1274196, 1298550,
            1323100, 1347845, 1372784, 1397916, 1423240, 1448755, 1474460, 1500354, 1526436,
            1552705, 1579160, 1605800, 1632624, 1659631, 1686820, 1714190, 1741740, 1769469,
            1797376, 1825460, 1853720, 1882155, 1910764, 1939546, 1968500, 1997625, 2026920,
            2056384, 2086016, 2115815, 2145780, 2175910, 2206204, 2236661, 2267280, 2298060,
            2329000, 2360099, 2391356, 2422770, 2454340, 2486065, 2517944, 2549976, 2582160,
            2614495, 2646980, 2679614, 2712396, 2745325, 2778400, 2811620, 2844984, 2878491,
            2912140, 2945930, 2979860, 3013929, 3048136, 3082480, 3116960, 3151575, 3186324,
            3221206, 3256220, 3291365, 3326640, 3362044, 3397576, 3433235, 3469020, 3504930,
            3540964, 3577121, 3613400, 3649800, 3686320, 3722959, 3759716, 3796590, 3833580,
            3870685, 3907904, 3945236, 3982680, 4020235, 4057900, 4095674, 4133556, 4171545,
            4209640, 4247840, 4286144, 4324551, 4363060, 4401670, 4440380, 4479189, 4518096,
            4557100, 4596200, 4635395, 4674684, 4714066, 4753540, 4793105, 4832760, 4872504,
            4912336, 4952255, 4992260, 5032350, 5072524, 5112781, 5153120, 5193540, 5234040,
            5274619, 5315276, 5356010, 5396820, 5437705, 5478664, 5519696, 5560800, 5601975,
            5643220, 5684534, 5725916, 5767365, 5808880, 5850460, 5892104, 5933811, 5975580,
            6017410, 6059300, 6101249, 6143256, 6185320, 6227440, 6269615, 6311844, 6354126,
            6396460, 6438845, 6481280, 6523764, 6566296, 6608875, 6651500, 6694170, 6736884,
            6779641, 6822440, 6865280, 6908160, 6951079, 6994036, 7037030, 7080060, 7123125,
            7166224, 7209356, 7252520, 7295715, 7338940, 7382194, 7425476, 7468785, 7512120,
            7555480, 7598864, 7642271, 7685700, 7729150, 7772620, 7816109, 7859616, 7903140,
            7946680, 7990235, 8033804, 8077386, 8120980, 8164585, 8208200, 8251824, 8295456,
            8339095, 8382740, 8426390, 8470044, 8513701, 8557360, 8601020, 8557360, 8513701,
            8470044, 8426390, 8382740, 8339095, 8295456, 8251824, 8208200, 8164585, 8120980,
            8077386, 8033804, 7990235, 7946680, 7903140, 7859616, 7816109, 7772620, 7729150,
            7685700, 7642271, 7598864, 7555480, 7512120, 7468785, 7425476, 7382194, 7338940,
            7295715, 7252520, 7209356, 7166224, 7123125, 7080060, 7037030, 6994036, 6951079,
            6908160, 6865280, 6822440, 6779641, 6736884, 6694170, 6651500, 6608875, 6566296,
            6523764, 6481280, 6438845, 6396460, 6354126, 6311844, 6269615, 6227440, 6185320,
            6143256, 6101249, 6059300, 6017410, 5975580, 5933811, 5892104, 5850460, 5808880,
            5767365, 5725916, 5684534, 5643220, 5601975, 5560800, 5519696, 5478664, 5437705,
            5396820, 5356010, 5315276, 5274619, 5234040, 5193540, 5153120, 5112781, 5072524,
            5032350, 4992260, 4952255, 4912336, 4872504, 4832760, 4793105, 4753540, 4714066,
            4674684, 4635395, 4596200, 4557100, 4518096, 4479189, 4440380, 4401670, 4363060,
            4324551, 4286144, 4247840, 4209640, 4171545, 4133556, 4095674, 4057900, 4020235,
            3982680, 3945236, 3907904, 3870685, 3833580, 3796590, 3759716, 3722959, 3686320,
            3649800, 3613400, 3577121, 3540964, 3504930, 3469020, 3433235, 3397576, 3362044,
            3326640, 3291365, 3256220, 3221206, 3186324, 3151575, 3116960, 3082480, 3048136,
            3013929, 2979860, 2945930, 2912140, 2878491, 2844984, 2811620, 2778400, 2745325,
            2712396, 2679614, 2646980, 2614495, 2582160, 2549976, 2517944, 2486065, 2454340,
            2422770, 2391356, 2360099, 2329000, 2298060, 2267280, 2236661, 2206204, 2175910,
            2145780, 2115815, 2086016, 2056384, 2026920, 1997625, 1968500, 1939546, 1910764,
            1882155, 1853720, 1825460, 1797376, 1769469, 1741740, 1714190, 1686820, 1659631,
            1632624, 1605800, 1579160, 1552705, 1526436, 1500354, 1474460, 1448755, 1423240,
            1397916, 1372784, 1347845, 1323100, 1298550, 1274196, 1250039, 1226080, 1202320,
            1178760, 1155401, 1132244, 1109290, 1086540, 1063995, 1041656, 1019524, 997600, 975885,
            954380, 933086, 912004, 891135, 870480, 850040, 829816, 809809, 790020, 770450, 751100,
            731971, 713064, 694380, 675920, 657685, 639676, 621894, 604340, 587015, 569920, 553056,
            536424, 520025, 503860, 487930, 472236, 456779, 441560, 426580, 411840, 397341, 383084,
            369070, 355300, 341775, 328496, 315464, 302680, 290145, 277860, 265826, 254044, 242515,
            231240, 220220, 209456, 198949, 188700, 178710, 168980, 159511, 150304, 141360, 132680,
            124265, 116116, 108234, 100620, 93275, 86200, 79396, 72864, 66605, 60620, 54910, 49476,
            44319, 39440, 34840, 30520, 26481, 22724, 19250, 16060, 13155, 10536, 8204, 6160, 4405,
            2940, 1766, 884, 295, 0,
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "index 1 out of range for slice of length 0")]
fn limbs_mul_to_out_toom_33_fail_1() {
    let mut scratch = vec![0; _limbs_mul_to_out_toom_33_scratch_size(1)];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_to_out_toom_33(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: xs_len >= ys_len")]
fn limbs_mul_to_out_toom_33_fail_2() {
    let mut scratch = vec![0; _limbs_mul_to_out_toom_33_scratch_size(5)];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_to_out_toom_33(
        &mut out,
        &[6, 7, 8, 9, 10],
        &[1, 2, 3, 4, 5, 6],
        &mut scratch,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: 0 < t && t <= n")]
fn limbs_mul_to_out_toom_33_fail_3() {
    let mut scratch = vec![0; _limbs_mul_to_out_toom_33_scratch_size(5)];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_to_out_toom_33(&mut out, &[6, 7, 8, 9, 10], &[1, 2, 3, 4], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "index 2 out of range for slice of length 0")]
fn limbs_mul_to_out_toom_33_fail_4() {
    let mut scratch = vec![0; _limbs_mul_to_out_toom_33_scratch_size(5)];
    let mut out = vec![10, 10, 10, 10, 10];
    _limbs_mul_to_out_toom_33(&mut out, &[6, 7, 8, 9, 10], &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: out_limbs.len() >= xs_len + ys_len")]
fn limbs_mul_to_out_toom_33_fail_5() {
    let mut scratch = vec![0; _limbs_mul_to_out_toom_33_scratch_size(6)];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_to_out_toom_33(
        &mut out,
        &[6, 7, 8, 9, 10, 11],
        &[1, 2, 3, 4, 5],
        &mut scratch,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_to_out_toom_42() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        _limbs_mul_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; _limbs_mul_to_out_toom_42_scratch_size(xs.len(), ys.len())];
        _limbs_mul_to_out_toom_42(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    // v_neg_1_neg
    // t == n
    // limbs_cmp_same_length(ys_0, ys_1) == Ordering::Less
    // s <= t
    // as1[n] not 1, 2, or 3
    test(
        vec![2, 3, 4, 5],
        vec![3, 4],
        vec![10, 10, 10, 10, 10, 10, 10],
        vec![6, 17, 24, 31, 20, 0, 10],
    );
    // !v_neg_1_neg
    // s != n
    // t != n
    // !(limbs_test_zero(&ys_0[t..]) && limbs_cmp_same_length(&ys_0[..t], ys_1) == Ordering::Less)
    test(
        vec![2, 3, 4, 5, 6, 7, 8],
        vec![3, 4, 5],
        vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10],
        vec![6, 17, 34, 46, 58, 70, 82, 67, 40, 0],
    );
    // s > t
    test(
        vec![2, 3, 4, 5, 6, 7, 8, 9],
        vec![3, 4, 5],
        vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10],
        vec![6, 17, 34, 46, 58, 70, 82, 94, 76, 45, 0],
    );
    // limbs_cmp_same_length(ys_0, ys_1) != Ordering::Less
    test(
        vec![0, 0, 0, 1],
        vec![1, 1],
        vec![10, 10, 10, 10, 10, 10],
        vec![0, 0, 0, 1, 1, 0],
    );
    // as1[n] == 1
    test(
        vec![
            2363703565, 2011430902, 405935879, 3293866119, 79230945, 4067912411, 54522599,
            3863530924, 2648195217, 3696638907, 2693775185, 2466180916, 2288038816, 3085875921,
            2622914893, 3412444602, 1714899352, 1458044565, 4160795266,
        ],
        vec![
            2010684769, 395852000, 1475286147, 263729287, 1827966398, 926833006, 3647866695,
            2299638628,
        ],
        vec![10; 27],
        vec![
            2935529197, 2628679470, 2989406385, 4135607148, 3098618197, 1986483787, 2969118597,
            4064944337, 1353361316, 3300804798, 3539475248, 1813351909, 4189109323, 1508204245,
            3032195050, 2111172804, 2647234523, 763063403, 499753337, 484003129, 951290762,
            31889895, 4291170933, 743974460, 931456782, 3403938046, 2227799389,
        ],
    );
    // bs1[n] != 0
    test(
        vec![
            1023706198, 1055957821, 62637438, 3129002448, 1343635842, 1979891039, 2332614953,
            820715064, 126240740, 3763174513, 874511155, 1433571832, 1799667271, 828081508,
            1790140791, 3456862168, 182082249,
        ],
        vec![
            272565221, 2271318511, 3915555663, 752672586, 2086228575, 93709012, 4089106295,
            1296382745, 4014782836, 4084383484,
        ],
        vec![10; 27],
        vec![
            2478924526, 600853546, 3764116188, 869876026, 49911338, 2430145334, 1531060628,
            4131353567, 2147110402, 1698823317, 3610138028, 2221603642, 2262453949, 2700908655,
            2085097953, 1179421079, 2314185794, 3274969801, 956808943, 183640877, 769743340,
            2499732116, 168215214, 1611459466, 1659741921, 3303732250, 173154690,
        ],
    );
    // as1[n] == 2
    test(
        vec![
            1048575, 0, 0, 4294965248, 33554431, 0, 0, 0, 4294966784, 4294967295, 4294967295,
            4294967295, 0, 2147483648, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        ],
        vec![
            4294967295, 4294967295, 4294967295, 2047, 0, 0, 4294705152, 4294967295,
        ],
        vec![10; 27],
        vec![
            4293918721, 4294967295, 4294967295, 2147483647, 4261412864, 4294967295, 4291035135,
            4294967231, 1049102, 536870912, 0, 4293914624, 33554430, 2147483648, 134217728, 2048,
            4294966784, 4294966271, 4294705151, 4294967294, 131072, 2147483648, 2047, 0, 0,
            4294705152, 4294967295,
        ],
    );
    // asm1[n] != 0
    test(
        vec![3338024033, 1570788701, 4067509056, 680440343],
        vec![599772085, 925834366],
        vec![10; 6],
        vec![
            1056633749, 686831275, 2758938475, 3727232403, 1859912609, 146677497,
        ],
    );
    // as1[n] == 3
    test(
        vec![4030415682, 3643742328, 2586387240, 3719633661],
        vec![708497006, 797041707],
        vec![10; 6],
        vec![
            4203348572, 3202027474, 4170951291, 2012723103, 3609216593, 690273745,
        ],
    );
    // limbs_test_zero(&ys_0[t..]) && limbs_cmp_same_length(&ys_0[..t], ys_1) == Ordering::Less
    test(
        vec![
            0, 4294967295, 4294967295, 1048575, 4294967280, 4294967295, 63, 0, 2147483648,
            4294967295, 4294967295,
        ],
        vec![65535, 0, 0, 4294967264],
        vec![10; 15],
        vec![
            0, 4294901761, 4294967295, 4293918719, 4293918783, 4294967294, 4265607103, 1049087,
            2147483632, 4294932480, 63, 65535, 2147483664, 4294967295, 4294967263,
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "index 1 out of range for slice of length 0")]
fn limbs_mul_to_out_toom_42_fail_1() {
    let mut scratch = vec![0; _limbs_mul_to_out_toom_42_scratch_size(1, 1)];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_to_out_toom_42(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "index 3 out of range for slice of length 2")]
fn limbs_mul_to_out_toom_42_fail_2() {
    let mut scratch = vec![0; _limbs_mul_to_out_toom_42_scratch_size(5, 6)];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_to_out_toom_42(
        &mut out,
        &[6, 7, 8, 9, 10],
        &[1, 2, 3, 4, 5, 6],
        &mut scratch,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: 0 < s && s <= n")]
fn limbs_mul_to_out_toom_42_fail_3() {
    let mut scratch = vec![0; _limbs_mul_to_out_toom_42_scratch_size(3, 2)];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_to_out_toom_42(&mut out, &[6, 7, 8], &[1, 2], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "index 2 out of range for slice of length 1")]
fn limbs_mul_to_out_toom_42_fail_4() {
    let mut scratch = vec![0; _limbs_mul_to_out_toom_42_scratch_size(5, 0)];
    let mut out = vec![10, 10, 10, 10, 10];
    _limbs_mul_to_out_toom_42(&mut out, &[6, 7, 8, 9, 10], &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: out_limbs.len() >= xs_len + ys_len")]
fn limbs_mul_to_out_toom_42_fail_5() {
    let mut scratch = vec![0; _limbs_mul_to_out_toom_42_scratch_size(4, 2)];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_to_out_toom_42(&mut out, &[6, 7, 8, 9], &[1, 2], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_to_out_toom_43() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        _limbs_mul_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; _limbs_mul_to_out_toom_43_scratch_size(xs.len(), ys.len())];
        _limbs_mul_to_out_toom_43(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    // n_high < n in _limbs_mul_toom_evaluate_deg_3_poly_in_2_and_neg_2
    // !v_neg_2_neg in _limbs_mul_toom_evaluate_deg_3_poly_in_2_and_neg_2
    // limbs_cmp_same_length(small_scratch, bsm1) != Ordering::Less
    // s <= t
    // !v_neg_2_neg in _limbs_mul_toom_interpolate_6_points
    // !v_neg_1_neg in _limbs_mul_toom_interpolate_6_points
    // n_high > n in _limbs_mul_toom_interpolate_6_points
    // special_carry_1 <= special_carry_2
    test(
        vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        vec![3, 4, 5, 6, 7, 8, 9, 10, 11],
        vec![10; 20],
        vec![
            6, 17, 34, 58, 90, 131, 182, 244, 318, 381, 444, 468, 476, 467, 440, 394, 328, 241,
            132, 0,
        ],
    );
    // n_high >= n in _limbs_mul_toom_evaluate_deg_3_poly_in_2_and_neg_2
    // v_neg_2_neg in _limbs_mul_toom_evaluate_deg_3_poly_in_2_and_neg_2
    // t != n
    // limbs_cmp_same_length(small_scratch, bsm1) == Ordering::Less
    // *bsm1_last == 0 && limbs_cmp_same_length(bsm1_init, ys_1) == Ordering::Less
    // s > t
    test(
        vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
        vec![3, 4, 5, 6, 7, 8, 9, 10],
        vec![10; 20],
        vec![
            6, 17, 34, 58, 90, 131, 182, 244, 296, 348, 400, 452, 462, 455, 430, 386, 322, 237,
            130, 0,
        ],
    );
    // v_neg_2_neg in _limbs_mul_toom_interpolate_6_points
    // v_neg_1_neg in _limbs_mul_toom_interpolate_6_points
    // n_high <= n in _limbs_mul_toom_interpolate_6_points
    test(
        vec![
            2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ],
        vec![3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
        vec![10; 30],
        vec![
            6, 17, 34, 58, 90, 131, 182, 244, 318, 405, 506, 594, 682, 770, 858, 946, 1034, 1122,
            1210, 1235, 1236, 1212, 1162, 1085, 980, 846, 682, 487, 260, 0,
        ],
    );
    // special_carry_1 > special_carry_2
    test(
        vec![
            3785023459, 4249117725, 1551102690, 4239134101, 2264608302, 1455009194, 3261002629,
            2233313730, 3807192178, 2550029068, 1259253479, 2657422450,
        ],
        vec![
            2921127090, 3493254221, 1579329255, 2624469567, 1678656523, 1653055771, 493445097,
            1702866165, 1046762910,
        ],
        vec![10; 21],
        vec![
            3169501142, 3910307595, 310092603, 1408815552, 1786334527, 2452212521, 670758829,
            4142968613, 1110881016, 3529286248, 2119180760, 3066268191, 1902231557, 1262478906,
            4083142666, 784312035, 3990199726, 3180402195, 1845375516, 421486236, 647662966,
        ],
    );
    test(
        vec![
            0, 0, 0, 4286578688, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295, 7, 0, 0, 0, 0, 4294967280, 4294967295, 4294967295, 4294967295,
        ],
        vec![
            2147483647, 4294963200, 2097151, 0, 0, 0, 2147483520, 0, 4294967280, 4294967295,
            4294967295, 4290789375, 4294967295, 4294967295, 4294967295, 4294967295,
        ],
        vec![10; 35],
        vec![
            0, 0, 0, 8388608, 4290772992, 7, 4294963200, 4294967295, 4294967295, 1073741823,
            4290772984, 134184963, 16777216, 0, 0, 8176, 64504, 4261412868, 4294967167, 2139095038,
            4294963200, 4263643135, 4294967287, 255, 0, 2147483520, 66846728, 4294967280,
            4294967295, 4294967295, 4290789375, 4294967279, 4294967295, 4294967295, 4294967295,
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "slice index starts at 3 but ends at 1")]
fn limbs_mul_to_out_toom_43_fail_1() {
    let mut scratch = vec![0; _limbs_mul_to_out_toom_43_scratch_size(1, 1)];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_to_out_toom_43(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "slice index starts at 12 but ends at 11")]
fn limbs_mul_to_out_toom_43_fail_2() {
    let mut scratch = vec![0; _limbs_mul_to_out_toom_43_scratch_size(11, 12)];
    let mut out = vec![10; 23];
    _limbs_mul_to_out_toom_43(
        &mut out,
        &[3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
        &[2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
        &mut scratch,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "slice index starts at 12 but ends at 9")]
fn limbs_mul_to_out_toom_43_fail_3() {
    let mut scratch = vec![0; _limbs_mul_to_out_toom_43_scratch_size(9, 10)];
    let mut out = vec![10; 19];
    _limbs_mul_to_out_toom_43(
        &mut out,
        &[3, 4, 5, 6, 7, 8, 9, 10, 11],
        &[2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        &mut scratch,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "index 3 out of range for slice of length 0")]
fn limbs_mul_to_out_toom_43_fail_4() {
    let mut scratch = vec![0; _limbs_mul_to_out_toom_43_scratch_size(12, 0)];
    let mut out = vec![10; 12];
    _limbs_mul_to_out_toom_43(
        &mut out,
        &[3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
        &[],
        &mut scratch,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "slice index starts at 12 but ends at 10")]
fn limbs_mul_to_out_toom_43_fail_5() {
    let mut scratch = vec![0; _limbs_mul_to_out_toom_43_scratch_size(4, 2)];
    let mut out = vec![10, 10, 10, 10, 10];
    _limbs_mul_to_out_toom_43(
        &mut out,
        &[3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        &[2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        &mut scratch,
    );
}

#[cfg(feature = "64_bit_limbs")]
#[test]
fn test_limbs_mul_to_out_toom_43() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        _limbs_mul_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; _limbs_mul_to_out_toom_43_scratch_size(xs.len(), ys.len())];
        _limbs_mul_to_out_toom_43(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    test(
        vec![
            18446744073701163071,
            18446744073709551615,
            18446744073709551615,
            68719476735,
            0,
            0,
            0,
            0,
            18446743936270598144,
            18446744073709551615,
            18446744073709551615,
            18446744073709551615,
            262143,
            0,
            0,
            0,
            18446462598732840960,
            32767,
            0,
            0,
            18446744073709518848,
            18446744073709551615,
            18446744073709551615,
            18446744073709551615,
            18446744073709551615,
        ],
        vec![
            18437736874454810624,
            1048575,
            0,
            18446744039349813248,
            18446744073709551615,
            18446744073709551615,
            18446744073709551615,
            140737488355327,
            0,
            18446744073709551600,
            18446744073709551615,
            18446744073709551615,
            134217727,
            18446744056529682432,
            18446744073709551615,
            18446744073709551615,
        ],
        vec![10; 41],
        vec![
            17879290520660869120,
            18446735277682593791,
            18446744073709551615,
            288228211488194559,
            72057594004373504,
            0,
            0,
            8866461766385536,
            18446744073709551552,
            18302628885835021327,
            18446744073709551615,
            524287,
            18445617082746798336,
            144114380622004095,
            0,
            9214364837600034816,
            18446744073700114495,
            2336462208959,
            34359738336,
            68719476736,
            18445618173803233282,
            18446744039345618958,
            127,
            1125899906842624,
            4611721063213039616,
            18437736874454810624,
            524287,
            13835058055282163712,
            18446744039350075391,
            4398047033343,
            18446181123756392448,
            18446744073709551615,
            18446598938174685183,
            562949953454079,
            18446744073709551600,
            18446744073709551615,
            18446744073709518847,
            134217727,
            18446744056529682432,
            18446744073709551615,
            18446744073709551615,
        ],
    );
}

#[test]
fn test_mul() {
    let test = |u, v, out| {
        let mut n = Natural::from_str(u).unwrap();
        n *= Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n *= &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n._mul_assign_basecase_mem_opt(Natural::from_str(v).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap() * Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() * Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap() * &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() * &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigUint::from_str(u).unwrap() * BigUint::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() * rug::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "0");
    test("123", "0", "0");
    test("1", "123", "123");
    test("123", "1", "123");
    test("123", "456", "56088");
    test("0", "1000000000000", "0");
    test("1000000000000", "0", "0");
    test("1", "1000000000000", "1000000000000");
    test("1000000000000", "1", "1000000000000");
    test("1000000000000", "123", "123000000000000");
    test("123", "1000000000000", "123000000000000");
    test("123456789000", "987654321000", "121932631112635269000000");
    test("4294967295", "2", "8589934590");
    test("4294967295", "4294967295", "18446744065119617025");
    test(
        "147502279655636565600250358452694051893980186696958535174009956523855720107322638159749368\
        0808217479494744305876890972595771484769733857514529616096199394092858302265998260483416016\
        5763904522044264005938281072568140883513713255548643044250086110483617215935636533809248102\
        6926590789142079805638445494760177551776636747830014495012489743990407355232286842071418922\
        9921358409573480901624487977319782755422730834468673438076805532952821406024399006814390166\
        6949827530796971086011267864607814906313334525518102221919643040440267323688341889035864376\
        1377246644579088153222669672271414315240318439843720039808993886410874969340996645010795670\
        2133518716987668865936529827437388042190084309005369564717390726257594902619365180097509576\
        6240189037770619308206906414128686856349950952623970023039440323701643457411485666776354448\
        186307133288106956593939073729500658176632828099789",
        "577397114388109712462006371470162814529304445639807296878809567953200969820156259914159240\
        9106139481288193067515284601342023565222679498917484131095648263181800618990427694244342686\
        4412105186059052689237237088193855584354278755933606296018800151986520872701706693002473648\
        4330061421236425747083307907706860804054565348593527605104495080560663025897787060638154303\
        7631781316565710346299551930891169154491973589315700505458672804104869879731391323700304",
        "851673906388325341550957943071111911557800036845129556099360938813259608650267079456739978\
        1156959952275409185911771336067392377245918291754269000751186715279414560474882570499082990\
        4913122978897463970860833616251189242098804876664368441608727895141238953179204529256780277\
        5978105200286025161944212712977056343127682601975191673217459602567633602198262568921008081\
        9448556670912575287371251190800855926311768876808375177446530243635212748346921654224589861\
        0625170426812525829689862407515510419445335472631905610235915226032848323874067128872385291\
        3730739275467227364692195226129501338887049710586931141309357190341064532366013123280106098\
        6468151628797945455179649866890394481799639832540978091736379482964522229064478167730317490\
        8194108506704480750395054067032502530392147690725919399930683143510771646869931527123340650\
        0547649792331568913460415939722111305270588701531404490040034302102101083691706550376288655\
        2667382899390792494118931379237432071316543313379792218794371176529684614085109418328963817\
        0601432767270419229719490809539776535671938041618536196941370647945336401901450921413823163\
        4059991707077834107830876756821880651429748186401020760113859498185638133726165286481741014\
        9079906337286599226335508424466369316294442004040440528589582239717042654541745348050157252\
        3448224036804997350851153108395928780441635856",
    );
}

fn limbs_mul_basecase_helper(out_limbs: &Vec<Limb>, xs: &Vec<Limb>, ys: &Vec<Limb>) -> Vec<Limb> {
    let mut out_limbs = out_limbs.to_vec();
    let old_out_limbs = out_limbs.clone();
    _limbs_mul_to_out_basecase(&mut out_limbs, xs, ys);
    let n = Natural::from_limbs_asc(xs) * Natural::from_limbs_asc(ys);
    let len = xs.len() + ys.len();
    let mut limbs = n.into_limbs_asc();
    limbs.resize(len, 0);
    assert_eq!(limbs, &out_limbs[..len]);
    assert_eq!(&out_limbs[len..], &old_out_limbs[len..]);
    out_limbs
}

#[test]
fn limbs_mul_to_out_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_10,
        |&(ref out_limbs, ref xs, ref ys)| {
            let expected_out_limbs = limbs_mul_basecase_helper(out_limbs, xs, ys);
            let mut out_limbs = out_limbs.to_vec();
            let highest_result_limb = mpn_mul(&mut out_limbs, xs, ys);
            assert_eq!(highest_result_limb, out_limbs[xs.len() + ys.len() - 1]);
            assert_eq!(out_limbs, expected_out_limbs);
        },
    );
}

#[test]
fn limbs_mul_to_out_toom_22_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_11,
        |&(ref out_limbs, ref xs, ref ys)| {
            let expected_out_limbs = limbs_mul_basecase_helper(out_limbs, xs, ys);
            let mut out_limbs = out_limbs.to_vec();
            let mut scratch = vec![0; _limbs_mul_to_out_toom_22_scratch_size(xs.len())];
            _limbs_mul_to_out_toom_22(&mut out_limbs, xs, ys, &mut scratch);
            assert_eq!(out_limbs, expected_out_limbs);
        },
    );
}

#[test]
fn limbs_mul_to_out_toom_32_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_12,
        |&(ref out_limbs, ref xs, ref ys)| {
            let expected_out_limbs = limbs_mul_basecase_helper(out_limbs, xs, ys);
            let mut out_limbs = out_limbs.to_vec();
            let mut scratch = vec![0; _limbs_mul_to_out_toom_32_scratch_size(xs.len(), ys.len())];
            _limbs_mul_to_out_toom_32(&mut out_limbs, xs, ys, &mut scratch);
            assert_eq!(out_limbs, expected_out_limbs);
        },
    );
}

#[test]
fn limbs_mul_to_out_toom_33_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_13,
        |&(ref out_limbs, ref xs, ref ys)| {
            let expected_out_limbs = limbs_mul_basecase_helper(out_limbs, xs, ys);
            let mut out_limbs = out_limbs.to_vec();
            let mut scratch = vec![0; _limbs_mul_to_out_toom_33_scratch_size(xs.len())];
            _limbs_mul_to_out_toom_33(&mut out_limbs, xs, ys, &mut scratch);
            assert_eq!(out_limbs, expected_out_limbs);
        },
    );
}

#[test]
fn limbs_mul_to_out_toom_42_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_14,
        |&(ref out_limbs, ref xs, ref ys)| {
            let expected_out_limbs = limbs_mul_basecase_helper(out_limbs, xs, ys);
            let mut out_limbs = out_limbs.to_vec();
            let mut scratch = vec![0; _limbs_mul_to_out_toom_42_scratch_size(xs.len(), ys.len())];
            _limbs_mul_to_out_toom_42(&mut out_limbs, xs, ys, &mut scratch);
            assert_eq!(out_limbs, expected_out_limbs);
        },
    );
}

#[test]
fn limbs_mul_to_out_toom_43_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_15,
        |&(ref out_limbs, ref xs, ref ys)| {
            let expected_out_limbs = limbs_mul_basecase_helper(out_limbs, xs, ys);
            let mut out_limbs = out_limbs.to_vec();
            let mut scratch = vec![0; _limbs_mul_to_out_toom_43_scratch_size(xs.len(), ys.len())];
            _limbs_mul_to_out_toom_43(&mut out_limbs, xs, ys, &mut scratch);
            assert_eq!(out_limbs, expected_out_limbs);
        },
    );
}

#[test]
fn mul_properties() {
    test_properties_custom_scale(2_048, pairs_of_naturals, |&(ref x, ref y)| {
        let product_val_val = x.clone() * y.clone();
        let product_val_ref = x.clone() * y;
        let product_ref_val = x * y.clone();
        let product = x * y;
        assert!(product_val_val.is_valid());
        assert!(product_val_ref.is_valid());
        assert!(product_ref_val.is_valid());
        assert!(product.is_valid());
        assert_eq!(product_val_val, product);
        assert_eq!(product_val_ref, product);
        assert_eq!(product_ref_val, product);

        let mut mut_x = x.clone();
        mut_x *= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, product);

        let mut mut_x = x.clone();
        mut_x *= y;
        assert_eq!(mut_x, product);
        assert!(mut_x.is_valid());

        let mut mut_x = x.clone();
        mut_x._mul_assign_basecase_mem_opt(y.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, product);

        let mut mut_x = natural_to_rug_integer(x);
        mut_x *= natural_to_rug_integer(y);
        assert_eq!(rug_integer_to_natural(&mut_x), product);

        assert_eq!(
            biguint_to_natural(&(natural_to_biguint(x) * natural_to_biguint(y))),
            product
        );
        assert_eq!(
            rug_integer_to_natural(&(natural_to_rug_integer(x) * natural_to_rug_integer(y))),
            product
        );
        assert_eq!(y * x, product);
        //TODO assert_eq!((product / x).unwrap(), *y);
        //TODO assert_eq!((product / y).unwrap(), *x);

        if *x != 0 && *y != 0 {
            assert!(product >= *x);
            assert!(product >= *y);
        }
    });

    test_properties_custom_scale(
        2_048,
        pairs_of_natural_and_unsigned,
        |&(ref x, y): &(Natural, Limb)| {
            let product = x * Natural::from(y);
            assert_eq!(x * y, product);
            assert_eq!(y * x, product);
        },
    );

    test_properties_custom_scale(
        2_048,
        pairs_of_natural_and_unsigned::<Limb>,
        |&(ref x, y)| {
            let product = x * Natural::from(y);
            assert_eq!(x * y, product);
            assert_eq!(y * x, product);
        },
    );

    test_properties_custom_scale(2_048, pairs_of_unsigneds::<Limb>, |&(x, y)| {
        assert_eq!(
            Natural::from(DoubleLimb::from(x) * DoubleLimb::from(y)),
            Natural::from(x) * Natural::from(y)
        );
    });

    #[allow(unknown_lints, erasing_op)]
    test_properties_custom_scale(2_048, naturals, |x| {
        assert_eq!(x * Natural::ZERO, 0);
        assert_eq!(Natural::ZERO * 0, 0);
        assert_eq!(x * Natural::ONE, *x);
        assert_eq!(Natural::ONE * x, *x);
        //TODO assert_eq!(x * x, x.pow(2));
    });

    test_properties_custom_scale(2_048, triples_of_naturals, |&(ref x, ref y, ref z)| {
        assert_eq!((x * y) * z, x * (y * z));
        assert_eq!(x * (y + z), x * y + x * z);
        assert_eq!((x + y) * z, x * z + y * z);
    });
}
