use common::test_properties_custom_scale;
use malachite_base::num::{One, Zero};
use malachite_nz::natural::arithmetic::mul::toom::{
    _limbs_mul_greater_to_out_toom_22, _limbs_mul_greater_to_out_toom_22_scratch_size,
    _limbs_mul_greater_to_out_toom_32, _limbs_mul_greater_to_out_toom_32_scratch_size,
    _limbs_mul_greater_to_out_toom_33, _limbs_mul_greater_to_out_toom_33_scratch_size,
    _limbs_mul_greater_to_out_toom_42, _limbs_mul_greater_to_out_toom_42_scratch_size,
    _limbs_mul_greater_to_out_toom_43, _limbs_mul_greater_to_out_toom_43_scratch_size,
    _limbs_mul_greater_to_out_toom_44, _limbs_mul_greater_to_out_toom_44_scratch_size,
};
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::arithmetic::mul::toom::{
    MUL_TOOM22_THRESHOLD, MUL_TOOM33_THRESHOLD, MUL_TOOM44_THRESHOLD,
};
use malachite_nz::natural::arithmetic::mul::{
    _limbs_mul_greater_to_out_basecase, limbs_mul_greater_to_out,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::{DoubleLimb, Limb};
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::base::{
    pairs_of_unsigneds, triples_of_unsigned_vec_var_10, triples_of_unsigned_vec_var_11,
    triples_of_unsigned_vec_var_12, triples_of_unsigned_vec_var_13, triples_of_unsigned_vec_var_14,
    triples_of_unsigned_vec_var_15, triples_of_unsigned_vec_var_16,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_unsigned, pairs_of_naturals, triples_of_naturals,
};
use num::BigUint;
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out() {
    let test = |xs, ys, out_before: &[Limb], highest_result_limb, out_after| {
        let mut out = out_before.to_vec();
        _limbs_mul_greater_to_out_basecase(&mut out, xs, ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        assert_eq!(
            limbs_mul_greater_to_out(&mut out, xs, ys),
            highest_result_limb
        );
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
fn limbs_mul_greater_to_out_fail_1() {
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_greater_to_out_basecase(&mut out, &[6, 7, 8], &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: xs_len >= ys_len")]
fn limbs_mul_greater_to_out_fail_2() {
    let mut out = vec![10, 10, 10, 10, 10];
    _limbs_mul_greater_to_out_basecase(&mut out, &[6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: `(left != right)")]
fn limbs_mul_greater_to_out_fail_3() {
    let mut out = vec![10, 10, 10];
    _limbs_mul_greater_to_out_basecase(&mut out, &[6, 7], &[]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_22() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_22_scratch_size(xs.len())];
        let mut out = out_before.to_vec();
        _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        _limbs_mul_greater_to_out_toom_22(&mut out, &xs, &ys, &mut scratch);
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
    // limbs_mul_greater_to_out_basecase in limbs_mul_greater_to_out_toom_22_recursive
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
    // limbs_mul_greater_to_out_basecase in limbs_mul_same_length_to_out_toom_22_recursive
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
    // limbs_mul_greater_to_out_toom_22 in limbs_mul_same_length_to_out_toom_22_recursive
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
    // limbs_mul_greater_to_out_toom_22 in limbs_mul_greater_to_out_toom_22_recursive
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
    // limbs_mul_greater_to_out_toom_32 in limbs_mul_greater_to_out_toom_22_recursive
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
fn limbs_mul_greater_to_out_toom_22_fail_1() {
    let mut scratch = vec![];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_22(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: xs_len >= ys_len")]
fn limbs_mul_greater_to_out_toom_22_fail_2() {
    let mut scratch = vec![];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_22(&mut out, &[6, 7, 8], &[1, 2, 3, 4], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: 0 < t && t <= s")]
fn limbs_mul_greater_to_out_toom_22_fail_3() {
    let mut scratch = vec![];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_22(&mut out, &[6, 7, 8, 9], &[1, 2], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: 0 < t && t <= s")]
fn limbs_mul_greater_to_out_toom_22_fail_4() {
    let mut scratch = vec![];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_22(&mut out, &[6, 7, 8], &[1, 2], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: mid <= len")]
fn limbs_mul_greater_to_out_toom_22_fail_5() {
    let mut scratch = vec![];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_22(&mut out, &[6, 7, 8], &[1, 2, 3], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: ys_len >= n")]
fn limbs_mul_greater_to_out_toom_22_fail_6() {
    let mut scratch = vec![];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_22(&mut out, &[6, 7, 8], &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: mid <= len")]
fn limbs_mul_greater_to_out_toom_22_fail_7() {
    let mut scratch = vec![];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_22(&mut out, &[6, 7, 8], &[1, 2, 3], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_32() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch =
            vec![0; _limbs_mul_greater_to_out_toom_32_scratch_size(xs.len(), ys.len())];
        _limbs_mul_greater_to_out_toom_32(&mut out, &xs, &ys, &mut scratch);
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
fn limbs_mul_greater_to_out_toom_32_fail_1() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_32_scratch_size(1, 1)];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_32(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: xs_len >= ys_len")]
fn limbs_mul_greater_to_out_toom_32_fail_2() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_32_scratch_size(3, 4)];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_32(&mut out, &[6, 7, 8], &[1, 2, 3, 4], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: ys_len + 2 <= xs_len && xs_len + 6 <= 3 * ys_len")]
fn limbs_mul_greater_to_out_toom_32_fail_3() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_32_scratch_size(5, 4)];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_32(&mut out, &[6, 7, 8, 9, 10], &[1, 2, 3, 4], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: ys_len + 2 <= xs_len && xs_len + 6 <= 3 * ys_len")]
fn limbs_mul_greater_to_out_toom_32_fail_4() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_32_scratch_size(6, 3)];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_32(&mut out, &[6, 7, 8, 9, 10, 11], &[1, 2, 3], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: ys_len + 2 <= xs_len && xs_len + 6 <= 3 * ys_len")]
fn limbs_mul_greater_to_out_toom_32_fail_5() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_32_scratch_size(3, 0)];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_32(&mut out, &[6, 7, 8], &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: out_limbs.len() >= xs_len + ys_len")]
fn limbs_mul_greater_to_out_toom_32_fail_6() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_32_scratch_size(6, 4)];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_32(&mut out, &[6, 7, 8, 9, 10, 11], &[1, 2, 3, 4], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_33() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_33_scratch_size(xs.len())];
        _limbs_mul_greater_to_out_toom_33(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    // carry == 0 && limbs_cmp_same_length(&gp[..n], xs_1) == Ordering::Less
    // s != n
    // carry == 0 && limbs_cmp_same_length(&gp[..n], ys_1) == Ordering::Less
    // t != n
    // s <= t
    // !v_neg_1
    // two_r <= k + 1
    // _limbs_mul_greater_to_out_basecase in _limbs_mul_greater_to_out_toom_33_recursive
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
    // _limbs_mul_greater_to_out_toom_22 in _limbs_mul_greater_to_out_toom_33_recursive
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
    // _limbs_mul_greater_to_out_toom_33 in _limbs_mul_greater_to_out_toom_33_recursive
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
fn limbs_mul_greater_to_out_toom_33_fail_1() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_33_scratch_size(1)];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_33(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: xs_len >= ys_len")]
fn limbs_mul_greater_to_out_toom_33_fail_2() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_33_scratch_size(5)];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_33(
        &mut out,
        &[6, 7, 8, 9, 10],
        &[1, 2, 3, 4, 5, 6],
        &mut scratch,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: 0 < t && t <= n")]
fn limbs_mul_greater_to_out_toom_33_fail_3() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_33_scratch_size(5)];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_33(&mut out, &[6, 7, 8, 9, 10], &[1, 2, 3, 4], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "index 2 out of range for slice of length 0")]
fn limbs_mul_greater_to_out_toom_33_fail_4() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_33_scratch_size(5)];
    let mut out = vec![10, 10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_33(&mut out, &[6, 7, 8, 9, 10], &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: out_limbs.len() >= xs_len + ys_len")]
fn limbs_mul_greater_to_out_toom_33_fail_5() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_33_scratch_size(6)];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_33(
        &mut out,
        &[6, 7, 8, 9, 10, 11],
        &[1, 2, 3, 4, 5],
        &mut scratch,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_42() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch =
            vec![0; _limbs_mul_greater_to_out_toom_42_scratch_size(xs.len(), ys.len())];
        _limbs_mul_greater_to_out_toom_42(&mut out, &xs, &ys, &mut scratch);
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
fn limbs_mul_greater_to_out_toom_42_fail_1() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_42_scratch_size(1, 1)];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_42(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "index 3 out of range for slice of length 2")]
fn limbs_mul_greater_to_out_toom_42_fail_2() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_42_scratch_size(5, 6)];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_42(
        &mut out,
        &[6, 7, 8, 9, 10],
        &[1, 2, 3, 4, 5, 6],
        &mut scratch,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: 0 < s && s <= n")]
fn limbs_mul_greater_to_out_toom_42_fail_3() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_42_scratch_size(3, 2)];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_42(&mut out, &[6, 7, 8], &[1, 2], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "index 2 out of range for slice of length 1")]
fn limbs_mul_greater_to_out_toom_42_fail_4() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_42_scratch_size(5, 0)];
    let mut out = vec![10, 10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_42(&mut out, &[6, 7, 8, 9, 10], &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "assertion failed: out_limbs.len() >= xs_len + ys_len")]
fn limbs_mul_greater_to_out_toom_42_fail_5() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_42_scratch_size(4, 2)];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_42(&mut out, &[6, 7, 8, 9], &[1, 2], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_43() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch =
            vec![0; _limbs_mul_greater_to_out_toom_43_scratch_size(xs.len(), ys.len())];
        _limbs_mul_greater_to_out_toom_43(&mut out, &xs, &ys, &mut scratch);
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
fn limbs_mul_greater_to_out_toom_43_fail_1() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_43_scratch_size(1, 1)];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_43(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "slice index starts at 12 but ends at 11")]
fn limbs_mul_greater_to_out_toom_43_fail_2() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_43_scratch_size(11, 12)];
    let mut out = vec![10; 23];
    _limbs_mul_greater_to_out_toom_43(
        &mut out,
        &[3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
        &[2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
        &mut scratch,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "slice index starts at 12 but ends at 9")]
fn limbs_mul_greater_to_out_toom_43_fail_3() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_43_scratch_size(9, 10)];
    let mut out = vec![10; 19];
    _limbs_mul_greater_to_out_toom_43(
        &mut out,
        &[3, 4, 5, 6, 7, 8, 9, 10, 11],
        &[2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        &mut scratch,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "index 3 out of range for slice of length 0")]
fn limbs_mul_greater_to_out_toom_43_fail_4() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_43_scratch_size(12, 0)];
    let mut out = vec![10; 12];
    _limbs_mul_greater_to_out_toom_43(
        &mut out,
        &[3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
        &[],
        &mut scratch,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic(expected = "slice index starts at 12 but ends at 10")]
fn limbs_mul_greater_to_out_toom_43_fail_5() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_43_scratch_size(4, 2)];
    let mut out = vec![10, 10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_43(
        &mut out,
        &[3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        &[2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        &mut scratch,
    );
}

#[cfg(feature = "64_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_43() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch =
            vec![0; _limbs_mul_greater_to_out_toom_43_scratch_size(xs.len(), ys.len())];
        _limbs_mul_greater_to_out_toom_43(&mut out, &xs, &ys, &mut scratch);
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

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_44() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_44_scratch_size(xs.len())];
        _limbs_mul_greater_to_out_toom_44(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    // _limbs_mul_toom_evaluate_deg_3_poly_in_2_and_neg_2(bpx, bmx, ys, n, t, &mut tp[..n + 1])
    // _limbs_mul_toom_evaluate_deg_3_poly_in_1_and_neg_1(bpx, bmx, ys, n, t, &mut tp[..n + 1])
    // s <= t
    // !w1_neg
    // !w3_neg
    // w6n <= n + 1
    // _limbs_mul_greater_to_out_basecase in _limbs_mul_same_length_to_out_toom_44_recursive
    test(
        vec![2, 3, 4, 5],
        vec![3, 4, 5, 6],
        vec![10; 8],
        vec![6, 17, 34, 58, 58, 49, 30, 0],
    );
    // w3_neg
    test(
        vec![0, 0, 0, 1],
        vec![0, 0, 1, 1],
        vec![10; 8],
        vec![0, 0, 0, 0, 0, 1, 1, 0],
    );
    // w6n > n + 1
    test(
        vec![
            1528859315, 4288784328, 3677151116, 445199233, 3304488688, 3566979465, 3541025426,
            2491779846, 3112990742, 2583249486, 3403111749, 1930721237,
        ],
        vec![
            2700212626, 3890522506, 1407330442, 2072012244, 292784856, 2848511017, 2011019434,
            3729188240, 1314875514, 1752114201, 3480385261, 1532349465,
        ],
        vec![10; 24],
        vec![
            301610262, 3665600695, 2790869988, 562719619, 254881625, 3646308155, 2857045174,
            4219173388, 3417896791, 458617279, 3882403287, 617740409, 3296542840, 435168928,
            3570119313, 863483077, 2646855475, 2878510649, 4228994627, 2357119023, 2589237669,
            2274199643, 3000367783, 688838692,
        ],
    );
    // s > t
    test(
        vec![
            1588217107, 79108222, 2883552792, 2390312777, 1587172303, 2070384343, 2265280181,
            4013380367,
        ],
        vec![
            3177381025, 2776698917, 954518943, 3785176644, 3521195169, 550485155, 1499535299,
        ],
        vec![10; 15],
        vec![
            2639930611, 1074195093, 3974952249, 2825437951, 3084912647, 2589723741, 1008656003,
            3022162475, 2305314017, 1619919364, 894905935, 3957960884, 814161571, 756465381,
            1401222667,
        ],
    );
    // w1_neg
    test(
        vec![
            1047248630, 339306853, 1100911694, 3907715577, 4281628442, 1447091409, 3425204321,
            3871347591, 339462242, 1765234031, 3774533011, 980706746,
        ],
        vec![
            1454868694, 1975460471, 2212752551, 1982786615, 983847073, 3073742136, 438698610,
            1215648998, 2824467771, 3299124311, 2818671068,
        ],
        vec![10; 23],
        vec![
            2438877604, 4249888081, 2301349363, 1817920534, 2538709343, 1739256708, 179543633,
            2275519806, 1688820820, 759475921, 3927834077, 2138533648, 958932069, 2429920287,
            3858014276, 2853106604, 1837491388, 1616377262, 231659922, 680814190, 417532392,
            428918230, 643611358,
        ],
    );
    let limit = 4 * (MUL_TOOM22_THRESHOLD + 1) - 3;
    let mut long_xs = Vec::new();
    let mut long_ys = Vec::new();
    for i in 0..limit {
        long_xs.push(i as Limb + 1);
        long_ys.push((limit - i) as Limb);
    }
    let long_out_limbs = vec![10; 2 * limit];
    // limbs_mul_greater_to_out_toom_22 in limbs_mul_same_length_to_out_toom_44_recursive
    test(
        long_xs,
        long_ys,
        long_out_limbs,
        vec![
            121, 362, 722, 1200, 1795, 2506, 3332, 4272, 5325, 6490, 7766, 9152, 10647, 12250,
            13960, 15776, 17697, 19722, 21850, 24080, 26411, 28842, 31372, 34000, 36725, 39546,
            42462, 45472, 48575, 51770, 55056, 58432, 61897, 65450, 69090, 72816, 76627, 80522,
            84500, 88560, 92701, 96922, 101222, 105600, 110055, 114586, 119192, 123872, 128625,
            133450, 138346, 143312, 148347, 153450, 158620, 163856, 169157, 174522, 179950, 185440,
            190991, 196602, 202272, 208000, 213785, 219626, 225522, 231472, 237475, 243530, 249636,
            255792, 261997, 268250, 274550, 280896, 287287, 293722, 300200, 306720, 313281, 319882,
            326522, 333200, 339915, 346666, 353452, 360272, 367125, 374010, 380926, 387872, 394847,
            401850, 408880, 415936, 423017, 430122, 437250, 444400, 451571, 458762, 465972, 473200,
            480445, 487706, 494982, 502272, 509575, 516890, 524216, 531552, 538897, 546250, 553610,
            560976, 568347, 575722, 583100, 590480, 597861, 590480, 583100, 575722, 568347, 560976,
            553610, 546250, 538897, 531552, 524216, 516890, 509575, 502272, 494982, 487706, 480445,
            473200, 465972, 458762, 451571, 444400, 437250, 430122, 423017, 415936, 408880, 401850,
            394847, 387872, 380926, 374010, 367125, 360272, 353452, 346666, 339915, 333200, 326522,
            319882, 313281, 306720, 300200, 293722, 287287, 280896, 274550, 268250, 261997, 255792,
            249636, 243530, 237475, 231472, 225522, 219626, 213785, 208000, 202272, 196602, 190991,
            185440, 179950, 174522, 169157, 163856, 158620, 153450, 148347, 143312, 138346, 133450,
            128625, 123872, 119192, 114586, 110055, 105600, 101222, 96922, 92701, 88560, 84500,
            80522, 76627, 72816, 69090, 65450, 61897, 58432, 55056, 51770, 48575, 45472, 42462,
            39546, 36725, 34000, 31372, 28842, 26411, 24080, 21850, 19722, 17697, 15776, 13960,
            12250, 10647, 9152, 7766, 6490, 5325, 4272, 3332, 2506, 1795, 1200, 722, 362, 121, 0,
        ],
    );
    let limit = 4 * (MUL_TOOM33_THRESHOLD + 1) - 3;
    let mut long_xs = Vec::new();
    let mut long_ys = Vec::new();
    for i in 0..limit {
        long_xs.push(i as Limb + 1);
        long_ys.push((limit - i) as Limb);
    }
    let long_out_limbs = vec![10; 2 * limit];
    // limbs_mul_greater_to_out_toom_33 in limbs_mul_same_length_to_out_toom_44_recursive
    test(
        long_xs,
        long_ys,
        long_out_limbs,
        vec![
            401, 1202, 2402, 4000, 5995, 8386, 11172, 14352, 17925, 21890, 26246, 30992, 36127,
            41650, 47560, 53856, 60537, 67602, 75050, 82880, 91091, 99682, 108652, 118000, 127725,
            137826, 148302, 159152, 170375, 181970, 193936, 206272, 218977, 232050, 245490, 259296,
            273467, 288002, 302900, 318160, 333781, 349762, 366102, 382800, 399855, 417266, 435032,
            453152, 471625, 490450, 509626, 529152, 549027, 569250, 589820, 610736, 631997, 653602,
            675550, 697840, 720471, 743442, 766752, 790400, 814385, 838706, 863362, 888352, 913675,
            939330, 965316, 991632, 1018277, 1045250, 1072550, 1100176, 1128127, 1156402, 1185000,
            1213920, 1243161, 1272722, 1302602, 1332800, 1363315, 1394146, 1425292, 1456752,
            1488525, 1520610, 1553006, 1585712, 1618727, 1652050, 1685680, 1719616, 1753857,
            1788402, 1823250, 1858400, 1893851, 1929602, 1965652, 2002000, 2038645, 2075586,
            2112822, 2150352, 2188175, 2226290, 2264696, 2303392, 2342377, 2381650, 2421210,
            2461056, 2501187, 2541602, 2582300, 2623280, 2664541, 2706082, 2747902, 2790000,
            2832375, 2875026, 2917952, 2961152, 3004625, 3048370, 3092386, 3136672, 3181227,
            3226050, 3271140, 3316496, 3362117, 3408002, 3454150, 3500560, 3547231, 3594162,
            3641352, 3688800, 3736505, 3784466, 3832682, 3881152, 3929875, 3978850, 4028076,
            4077552, 4127277, 4177250, 4227470, 4277936, 4328647, 4379602, 4430800, 4482240,
            4533921, 4585842, 4638002, 4690400, 4743035, 4795906, 4849012, 4902352, 4955925,
            5009730, 5063766, 5118032, 5172527, 5227250, 5282200, 5337376, 5392777, 5448402,
            5504250, 5560320, 5616611, 5673122, 5729852, 5786800, 5843965, 5901346, 5958942,
            6016752, 6074775, 6133010, 6191456, 6250112, 6308977, 6368050, 6427330, 6486816,
            6546507, 6606402, 6666500, 6726800, 6787301, 6848002, 6908902, 6970000, 7031295,
            7092786, 7154472, 7216352, 7278425, 7340690, 7403146, 7465792, 7528627, 7591650,
            7654860, 7718256, 7781837, 7845602, 7909550, 7973680, 8037991, 8102482, 8167152,
            8232000, 8297025, 8362226, 8427602, 8493152, 8558875, 8624770, 8690836, 8757072,
            8823477, 8890050, 8956790, 9023696, 9090767, 9158002, 9225400, 9292960, 9360681,
            9428562, 9496602, 9564800, 9633155, 9701666, 9770332, 9839152, 9908125, 9977250,
            10046526, 10115952, 10185527, 10255250, 10325120, 10395136, 10465297, 10535602,
            10606050, 10676640, 10747371, 10818242, 10889252, 10960400, 11031685, 11103106,
            11174662, 11246352, 11318175, 11390130, 11462216, 11534432, 11606777, 11679250,
            11751850, 11824576, 11897427, 11970402, 12043500, 12116720, 12190061, 12263522,
            12337102, 12410800, 12484615, 12558546, 12632592, 12706752, 12781025, 12855410,
            12929906, 13004512, 13079227, 13154050, 13228980, 13304016, 13379157, 13454402,
            13529750, 13605200, 13680751, 13756402, 13832152, 13908000, 13983945, 14059986,
            14136122, 14212352, 14288675, 14365090, 14441596, 14518192, 14594877, 14671650,
            14748510, 14825456, 14902487, 14979602, 15056800, 15134080, 15211441, 15288882,
            15366402, 15444000, 15521675, 15599426, 15677252, 15755152, 15833125, 15911170,
            15989286, 16067472, 16145727, 16224050, 16302440, 16380896, 16459417, 16538002,
            16616650, 16695360, 16774131, 16852962, 16931852, 17010800, 17089805, 17168866,
            17247982, 17327152, 17406375, 17485650, 17564976, 17644352, 17723777, 17803250,
            17882770, 17962336, 18041947, 18121602, 18201300, 18281040, 18360821, 18440642,
            18520502, 18600400, 18680335, 18760306, 18840312, 18920352, 19000425, 19080530,
            19160666, 19240832, 19321027, 19401250, 19481500, 19561776, 19642077, 19722402,
            19802750, 19883120, 19963511, 20043922, 20124352, 20204800, 20285265, 20365746,
            20446242, 20526752, 20607275, 20687810, 20768356, 20848912, 20929477, 21010050,
            21090630, 21171216, 21251807, 21332402, 21413000, 21493600, 21574201, 21493600,
            21413000, 21332402, 21251807, 21171216, 21090630, 21010050, 20929477, 20848912,
            20768356, 20687810, 20607275, 20526752, 20446242, 20365746, 20285265, 20204800,
            20124352, 20043922, 19963511, 19883120, 19802750, 19722402, 19642077, 19561776,
            19481500, 19401250, 19321027, 19240832, 19160666, 19080530, 19000425, 18920352,
            18840312, 18760306, 18680335, 18600400, 18520502, 18440642, 18360821, 18281040,
            18201300, 18121602, 18041947, 17962336, 17882770, 17803250, 17723777, 17644352,
            17564976, 17485650, 17406375, 17327152, 17247982, 17168866, 17089805, 17010800,
            16931852, 16852962, 16774131, 16695360, 16616650, 16538002, 16459417, 16380896,
            16302440, 16224050, 16145727, 16067472, 15989286, 15911170, 15833125, 15755152,
            15677252, 15599426, 15521675, 15444000, 15366402, 15288882, 15211441, 15134080,
            15056800, 14979602, 14902487, 14825456, 14748510, 14671650, 14594877, 14518192,
            14441596, 14365090, 14288675, 14212352, 14136122, 14059986, 13983945, 13908000,
            13832152, 13756402, 13680751, 13605200, 13529750, 13454402, 13379157, 13304016,
            13228980, 13154050, 13079227, 13004512, 12929906, 12855410, 12781025, 12706752,
            12632592, 12558546, 12484615, 12410800, 12337102, 12263522, 12190061, 12116720,
            12043500, 11970402, 11897427, 11824576, 11751850, 11679250, 11606777, 11534432,
            11462216, 11390130, 11318175, 11246352, 11174662, 11103106, 11031685, 10960400,
            10889252, 10818242, 10747371, 10676640, 10606050, 10535602, 10465297, 10395136,
            10325120, 10255250, 10185527, 10115952, 10046526, 9977250, 9908125, 9839152, 9770332,
            9701666, 9633155, 9564800, 9496602, 9428562, 9360681, 9292960, 9225400, 9158002,
            9090767, 9023696, 8956790, 8890050, 8823477, 8757072, 8690836, 8624770, 8558875,
            8493152, 8427602, 8362226, 8297025, 8232000, 8167152, 8102482, 8037991, 7973680,
            7909550, 7845602, 7781837, 7718256, 7654860, 7591650, 7528627, 7465792, 7403146,
            7340690, 7278425, 7216352, 7154472, 7092786, 7031295, 6970000, 6908902, 6848002,
            6787301, 6726800, 6666500, 6606402, 6546507, 6486816, 6427330, 6368050, 6308977,
            6250112, 6191456, 6133010, 6074775, 6016752, 5958942, 5901346, 5843965, 5786800,
            5729852, 5673122, 5616611, 5560320, 5504250, 5448402, 5392777, 5337376, 5282200,
            5227250, 5172527, 5118032, 5063766, 5009730, 4955925, 4902352, 4849012, 4795906,
            4743035, 4690400, 4638002, 4585842, 4533921, 4482240, 4430800, 4379602, 4328647,
            4277936, 4227470, 4177250, 4127277, 4077552, 4028076, 3978850, 3929875, 3881152,
            3832682, 3784466, 3736505, 3688800, 3641352, 3594162, 3547231, 3500560, 3454150,
            3408002, 3362117, 3316496, 3271140, 3226050, 3181227, 3136672, 3092386, 3048370,
            3004625, 2961152, 2917952, 2875026, 2832375, 2790000, 2747902, 2706082, 2664541,
            2623280, 2582300, 2541602, 2501187, 2461056, 2421210, 2381650, 2342377, 2303392,
            2264696, 2226290, 2188175, 2150352, 2112822, 2075586, 2038645, 2002000, 1965652,
            1929602, 1893851, 1858400, 1823250, 1788402, 1753857, 1719616, 1685680, 1652050,
            1618727, 1585712, 1553006, 1520610, 1488525, 1456752, 1425292, 1394146, 1363315,
            1332800, 1302602, 1272722, 1243161, 1213920, 1185000, 1156402, 1128127, 1100176,
            1072550, 1045250, 1018277, 991632, 965316, 939330, 913675, 888352, 863362, 838706,
            814385, 790400, 766752, 743442, 720471, 697840, 675550, 653602, 631997, 610736, 589820,
            569250, 549027, 529152, 509626, 490450, 471625, 453152, 435032, 417266, 399855, 382800,
            366102, 349762, 333781, 318160, 302900, 288002, 273467, 259296, 245490, 232050, 218977,
            206272, 193936, 181970, 170375, 159152, 148302, 137826, 127725, 118000, 108652, 99682,
            91091, 82880, 75050, 67602, 60537, 53856, 47560, 41650, 36127, 30992, 26246, 21890,
            17925, 14352, 11172, 8386, 5995, 4000, 2402, 1202, 401, 0,
        ],
    );
    let limit = 4 * (MUL_TOOM44_THRESHOLD + 1) - 3;
    let mut long_xs = Vec::new();
    let mut long_ys = Vec::new();
    for i in 0..limit {
        long_xs.push(i as Limb + 1);
        long_ys.push((limit - i) as Limb);
    }
    let long_out_limbs = vec![10; 2 * limit];
    // limbs_mul_greater_to_out_toom_44 in limbs_mul_same_length_to_out_toom_44_recursive
    test(
        long_xs,
        long_ys,
        long_out_limbs,
        vec![
            1201, 3602, 7202, 12000, 17995, 25186, 33572, 43152, 53925, 65890, 79046, 93392,
            108927, 125650, 143560, 162656, 182937, 204402, 227050, 250880, 275891, 302082, 329452,
            358000, 387725, 418626, 450702, 483952, 518375, 553970, 590736, 628672, 667777, 708050,
            749490, 792096, 835867, 880802, 926900, 974160, 1022581, 1072162, 1122902, 1174800,
            1227855, 1282066, 1337432, 1393952, 1451625, 1510450, 1570426, 1631552, 1693827,
            1757250, 1821820, 1887536, 1954397, 2022402, 2091550, 2161840, 2233271, 2305842,
            2379552, 2454400, 2530385, 2607506, 2685762, 2765152, 2845675, 2927330, 3010116,
            3094032, 3179077, 3265250, 3352550, 3440976, 3530527, 3621202, 3713000, 3805920,
            3899961, 3995122, 4091402, 4188800, 4287315, 4386946, 4487692, 4589552, 4692525,
            4796610, 4901806, 5008112, 5115527, 5224050, 5333680, 5444416, 5556257, 5669202,
            5783250, 5898400, 6014651, 6132002, 6250452, 6370000, 6490645, 6612386, 6735222,
            6859152, 6984175, 7110290, 7237496, 7365792, 7495177, 7625650, 7757210, 7889856,
            8023587, 8158402, 8294300, 8431280, 8569341, 8708482, 8848702, 8990000, 9132375,
            9275826, 9420352, 9565952, 9712625, 9860370, 10009186, 10159072, 10310027, 10462050,
            10615140, 10769296, 10924517, 11080802, 11238150, 11396560, 11556031, 11716562,
            11878152, 12040800, 12204505, 12369266, 12535082, 12701952, 12869875, 13038850,
            13208876, 13379952, 13552077, 13725250, 13899470, 14074736, 14251047, 14428402,
            14606800, 14786240, 14966721, 15148242, 15330802, 15514400, 15699035, 15884706,
            16071412, 16259152, 16447925, 16637730, 16828566, 17020432, 17213327, 17407250,
            17602200, 17798176, 17995177, 18193202, 18392250, 18592320, 18793411, 18995522,
            19198652, 19402800, 19607965, 19814146, 20021342, 20229552, 20438775, 20649010,
            20860256, 21072512, 21285777, 21500050, 21715330, 21931616, 22148907, 22367202,
            22586500, 22806800, 23028101, 23250402, 23473702, 23698000, 23923295, 24149586,
            24376872, 24605152, 24834425, 25064690, 25295946, 25528192, 25761427, 25995650,
            26230860, 26467056, 26704237, 26942402, 27181550, 27421680, 27662791, 27904882,
            28147952, 28392000, 28637025, 28883026, 29130002, 29377952, 29626875, 29876770,
            30127636, 30379472, 30632277, 30886050, 31140790, 31396496, 31653167, 31910802,
            32169400, 32428960, 32689481, 32950962, 33213402, 33476800, 33741155, 34006466,
            34272732, 34539952, 34808125, 35077250, 35347326, 35618352, 35890327, 36163250,
            36437120, 36711936, 36987697, 37264402, 37542050, 37820640, 38100171, 38380642,
            38662052, 38944400, 39227685, 39511906, 39797062, 40083152, 40370175, 40658130,
            40947016, 41236832, 41527577, 41819250, 42111850, 42405376, 42699827, 42995202,
            43291500, 43588720, 43886861, 44185922, 44485902, 44786800, 45088615, 45391346,
            45694992, 45999552, 46305025, 46611410, 46918706, 47226912, 47536027, 47846050,
            48156980, 48468816, 48781557, 49095202, 49409750, 49725200, 50041551, 50358802,
            50676952, 50996000, 51315945, 51636786, 51958522, 52281152, 52604675, 52929090,
            53254396, 53580592, 53907677, 54235650, 54564510, 54894256, 55224887, 55556402,
            55888800, 56222080, 56556241, 56891282, 57227202, 57564000, 57901675, 58240226,
            58579652, 58919952, 59261125, 59603170, 59946086, 60289872, 60634527, 60980050,
            61326440, 61673696, 62021817, 62370802, 62720650, 63071360, 63422931, 63775362,
            64128652, 64482800, 64837805, 65193666, 65550382, 65907952, 66266375, 66625650,
            66985776, 67346752, 67708577, 68071250, 68434770, 68799136, 69164347, 69530402,
            69897300, 70265040, 70633621, 71003042, 71373302, 71744400, 72116335, 72489106,
            72862712, 73237152, 73612425, 73988530, 74365466, 74743232, 75121827, 75501250,
            75881500, 76262576, 76644477, 77027202, 77410750, 77795120, 78180311, 78566322,
            78953152, 79340800, 79729265, 80118546, 80508642, 80899552, 81291275, 81683810,
            82077156, 82471312, 82866277, 83262050, 83658630, 84056016, 84454207, 84853202,
            85253000, 85653600, 86055001, 86457202, 86860202, 87264000, 87668595, 88073986,
            88480172, 88887152, 89294925, 89703490, 90112846, 90522992, 90933927, 91345650,
            91758160, 92171456, 92585537, 93000402, 93416050, 93832480, 94249691, 94667682,
            95086452, 95506000, 95926325, 96347426, 96769302, 97191952, 97615375, 98039570,
            98464536, 98890272, 99316777, 99744050, 100172090, 100600896, 101030467, 101460802,
            101891900, 102323760, 102756381, 103189762, 103623902, 104058800, 104494455, 104930866,
            105368032, 105805952, 106244625, 106684050, 107124226, 107565152, 108006827, 108449250,
            108892420, 109336336, 109780997, 110226402, 110672550, 111119440, 111567071, 112015442,
            112464552, 112914400, 113364985, 113816306, 114268362, 114721152, 115174675, 115628930,
            116083916, 116539632, 116996077, 117453250, 117911150, 118369776, 118829127, 119289202,
            119750000, 120211520, 120673761, 121136722, 121600402, 122064800, 122529915, 122995746,
            123462292, 123929552, 124397525, 124866210, 125335606, 125805712, 126276527, 126748050,
            127220280, 127693216, 128166857, 128641202, 129116250, 129592000, 130068451, 130545602,
            131023452, 131502000, 131981245, 132461186, 132941822, 133423152, 133905175, 134387890,
            134871296, 135355392, 135840177, 136325650, 136811810, 137298656, 137786187, 138274402,
            138763300, 139252880, 139743141, 140234082, 140725702, 141218000, 141710975, 142204626,
            142698952, 143193952, 143689625, 144185970, 144682986, 145180672, 145679027, 146178050,
            146677740, 147178096, 147679117, 148180802, 148683150, 149186160, 149689831, 150194162,
            150699152, 151204800, 151711105, 152218066, 152725682, 153233952, 153742875, 154252450,
            154762676, 155273552, 155785077, 156297250, 156810070, 157323536, 157837647, 158352402,
            158867800, 159383840, 159900521, 160417842, 160935802, 161454400, 161973635, 162493506,
            163014012, 163535152, 164056925, 164579330, 165102366, 165626032, 166150327, 166675250,
            167200800, 167726976, 168253777, 168781202, 169309250, 169837920, 170367211, 170897122,
            171427652, 171958800, 172490565, 173022946, 173555942, 174089552, 174623775, 175158610,
            175694056, 176230112, 176766777, 177304050, 177841930, 178380416, 178919507, 179459202,
            179999500, 180540400, 181081901, 181624002, 182166702, 182710000, 183253895, 183798386,
            184343472, 184889152, 185435425, 185982290, 186529746, 187077792, 187626427, 188175650,
            188725460, 189275856, 189826837, 190378402, 190930550, 191483280, 192036591, 192590482,
            193144952, 193700000, 194255625, 194811826, 195368602, 195925952, 196483875, 197042370,
            197601436, 198161072, 198721277, 199282050, 199843390, 200405296, 200967767, 201530802,
            202094400, 202658560, 203223281, 203788562, 204354402, 204920800, 205487755, 206055266,
            206623332, 207191952, 207761125, 208330850, 208901126, 209471952, 210043327, 210615250,
            211187720, 211760736, 212334297, 212908402, 213483050, 214058240, 214633971, 215210242,
            215787052, 216364400, 216942285, 217520706, 218099662, 218679152, 219259175, 219839730,
            220420816, 221002432, 221584577, 222167250, 222750450, 223334176, 223918427, 224503202,
            225088500, 225674320, 226260661, 226847522, 227434902, 228022800, 228611215, 229200146,
            229789592, 230379552, 230970025, 231561010, 232152506, 232744512, 233337027, 233930050,
            234523580, 235117616, 235712157, 236307202, 236902750, 237498800, 238095351, 238692402,
            239289952, 239888000, 240486545, 241085586, 241685122, 242285152, 242885675, 243486690,
            244088196, 244690192, 245292677, 245895650, 246499110, 247103056, 247707487, 248312402,
            248917800, 249523680, 250130041, 250736882, 251344202, 251952000, 252560275, 253169026,
            253778252, 254387952, 254998125, 255608770, 256219886, 256831472, 257443527, 258056050,
            258669040, 259282496, 259896417, 260510802, 261125650, 261740960, 262356731, 262972962,
            263589652, 264206800, 264824405, 265442466, 266060982, 266679952, 267299375, 267919250,
            268539576, 269160352, 269781577, 270403250, 271025370, 271647936, 272270947, 272894402,
            273518300, 274142640, 274767421, 275392642, 276018302, 276644400, 277270935, 277897906,
            278525312, 279153152, 279781425, 280410130, 281039266, 281668832, 282298827, 282929250,
            283560100, 284191376, 284823077, 285455202, 286087750, 286720720, 287354111, 287987922,
            288622152, 289256800, 289891865, 290527346, 291163242, 291799552, 292436275, 293073410,
            293710956, 294348912, 294987277, 295626050, 296265230, 296904816, 297544807, 298185202,
            298826000, 299467200, 300108801, 300750802, 301393202, 302036000, 302679195, 303322786,
            303966772, 304611152, 305255925, 305901090, 306546646, 307192592, 307838927, 308485650,
            309132760, 309780256, 310428137, 311076402, 311725050, 312374080, 313023491, 313673282,
            314323452, 314974000, 315624925, 316276226, 316927902, 317579952, 318232375, 318885170,
            319538336, 320191872, 320845777, 321500050, 322154690, 322809696, 323465067, 324120802,
            324776900, 325433360, 326090181, 326747362, 327404902, 328062800, 328721055, 329379666,
            330038632, 330697952, 331357625, 332017650, 332678026, 333338752, 333999827, 334661250,
            335323020, 335985136, 336647597, 337310402, 337973550, 338637040, 339300871, 339965042,
            340629552, 341294400, 341959585, 342625106, 343290962, 343957152, 344623675, 345290530,
            345957716, 346625232, 347293077, 347961250, 348629750, 349298576, 349967727, 350637202,
            351307000, 351977120, 352647561, 353318322, 353989402, 354660800, 355332515, 356004546,
            356676892, 357349552, 358022525, 358695810, 359369406, 360043312, 360717527, 361392050,
            362066880, 362742016, 363417457, 364093202, 364769250, 365445600, 366122251, 366799202,
            367476452, 368154000, 368831845, 369509986, 370188422, 370867152, 371546175, 372225490,
            372905096, 373584992, 374265177, 374945650, 375626410, 376307456, 376988787, 377670402,
            378352300, 379034480, 379716941, 380399682, 381082702, 381766000, 382449575, 383133426,
            383817552, 384501952, 385186625, 385871570, 386556786, 387242272, 387928027, 388614050,
            389300340, 389986896, 390673717, 391360802, 392048150, 392735760, 393423631, 394111762,
            394800152, 395488800, 396177705, 396866866, 397556282, 398245952, 398935875, 399626050,
            400316476, 401007152, 401698077, 402389250, 403080670, 403772336, 404464247, 405156402,
            405848800, 406541440, 407234321, 407927442, 408620802, 409314400, 410008235, 410702306,
            411396612, 412091152, 412785925, 413480930, 414176166, 414871632, 415567327, 416263250,
            416959400, 417655776, 418352377, 419049202, 419746250, 420443520, 421141011, 421838722,
            422536652, 423234800, 423933165, 424631746, 425330542, 426029552, 426728775, 427428210,
            428127856, 428827712, 429527777, 430228050, 430928530, 431629216, 432330107, 433031202,
            433732500, 434434000, 435135701, 435837602, 436539702, 437242000, 437944495, 438647186,
            439350072, 440053152, 440756425, 441459890, 442163546, 442867392, 443571427, 444275650,
            444980060, 445684656, 446389437, 447094402, 447799550, 448504880, 449210391, 449916082,
            450621952, 451328000, 452034225, 452740626, 453447202, 454153952, 454860875, 455567970,
            456275236, 456982672, 457690277, 458398050, 459105990, 459814096, 460522367, 461230802,
            461939400, 462648160, 463357081, 464066162, 464775402, 465484800, 466194355, 466904066,
            467613932, 468323952, 469034125, 469744450, 470454926, 471165552, 471876327, 472587250,
            473298320, 474009536, 474720897, 475432402, 476144050, 476855840, 477567771, 478279842,
            478992052, 479704400, 480416885, 481129506, 481842262, 482555152, 483268175, 483981330,
            484694616, 485408032, 486121577, 486835250, 487549050, 488262976, 488977027, 489691202,
            490405500, 491119920, 491834461, 492549122, 493263902, 493978800, 494693815, 495408946,
            496124192, 496839552, 497555025, 498270610, 498986306, 499702112, 500418027, 501134050,
            501850180, 502566416, 503282757, 503999202, 504715750, 505432400, 506149151, 506866002,
            507582952, 508300000, 509017145, 509734386, 510451722, 511169152, 511886675, 512604290,
            513321996, 514039792, 514757677, 515475650, 516193710, 516911856, 517630087, 518348402,
            519066800, 519785280, 520503841, 521222482, 521941202, 522660000, 523378875, 524097826,
            524816852, 525535952, 526255125, 526974370, 527693686, 528413072, 529132527, 529852050,
            530571640, 531291296, 532011017, 532730802, 533450650, 534170560, 534890531, 535610562,
            536330652, 537050800, 537771005, 538491266, 539211582, 539931952, 540652375, 541372850,
            542093376, 542813952, 543534577, 544255250, 544975970, 545696736, 546417547, 547138402,
            547859300, 548580240, 549301221, 550022242, 550743302, 551464400, 552185535, 552906706,
            553627912, 554349152, 555070425, 555791730, 556513066, 557234432, 557955827, 558677250,
            559398700, 560120176, 560841677, 561563202, 562284750, 563006320, 563727911, 564449522,
            565171152, 565892800, 566614465, 567336146, 568057842, 568779552, 569501275, 570223010,
            570944756, 571666512, 572388277, 573110050, 573831830, 574553616, 575275407, 575997202,
            576719000, 577440800, 578162601, 577440800, 576719000, 575997202, 575275407, 574553616,
            573831830, 573110050, 572388277, 571666512, 570944756, 570223010, 569501275, 568779552,
            568057842, 567336146, 566614465, 565892800, 565171152, 564449522, 563727911, 563006320,
            562284750, 561563202, 560841677, 560120176, 559398700, 558677250, 557955827, 557234432,
            556513066, 555791730, 555070425, 554349152, 553627912, 552906706, 552185535, 551464400,
            550743302, 550022242, 549301221, 548580240, 547859300, 547138402, 546417547, 545696736,
            544975970, 544255250, 543534577, 542813952, 542093376, 541372850, 540652375, 539931952,
            539211582, 538491266, 537771005, 537050800, 536330652, 535610562, 534890531, 534170560,
            533450650, 532730802, 532011017, 531291296, 530571640, 529852050, 529132527, 528413072,
            527693686, 526974370, 526255125, 525535952, 524816852, 524097826, 523378875, 522660000,
            521941202, 521222482, 520503841, 519785280, 519066800, 518348402, 517630087, 516911856,
            516193710, 515475650, 514757677, 514039792, 513321996, 512604290, 511886675, 511169152,
            510451722, 509734386, 509017145, 508300000, 507582952, 506866002, 506149151, 505432400,
            504715750, 503999202, 503282757, 502566416, 501850180, 501134050, 500418027, 499702112,
            498986306, 498270610, 497555025, 496839552, 496124192, 495408946, 494693815, 493978800,
            493263902, 492549122, 491834461, 491119920, 490405500, 489691202, 488977027, 488262976,
            487549050, 486835250, 486121577, 485408032, 484694616, 483981330, 483268175, 482555152,
            481842262, 481129506, 480416885, 479704400, 478992052, 478279842, 477567771, 476855840,
            476144050, 475432402, 474720897, 474009536, 473298320, 472587250, 471876327, 471165552,
            470454926, 469744450, 469034125, 468323952, 467613932, 466904066, 466194355, 465484800,
            464775402, 464066162, 463357081, 462648160, 461939400, 461230802, 460522367, 459814096,
            459105990, 458398050, 457690277, 456982672, 456275236, 455567970, 454860875, 454153952,
            453447202, 452740626, 452034225, 451328000, 450621952, 449916082, 449210391, 448504880,
            447799550, 447094402, 446389437, 445684656, 444980060, 444275650, 443571427, 442867392,
            442163546, 441459890, 440756425, 440053152, 439350072, 438647186, 437944495, 437242000,
            436539702, 435837602, 435135701, 434434000, 433732500, 433031202, 432330107, 431629216,
            430928530, 430228050, 429527777, 428827712, 428127856, 427428210, 426728775, 426029552,
            425330542, 424631746, 423933165, 423234800, 422536652, 421838722, 421141011, 420443520,
            419746250, 419049202, 418352377, 417655776, 416959400, 416263250, 415567327, 414871632,
            414176166, 413480930, 412785925, 412091152, 411396612, 410702306, 410008235, 409314400,
            408620802, 407927442, 407234321, 406541440, 405848800, 405156402, 404464247, 403772336,
            403080670, 402389250, 401698077, 401007152, 400316476, 399626050, 398935875, 398245952,
            397556282, 396866866, 396177705, 395488800, 394800152, 394111762, 393423631, 392735760,
            392048150, 391360802, 390673717, 389986896, 389300340, 388614050, 387928027, 387242272,
            386556786, 385871570, 385186625, 384501952, 383817552, 383133426, 382449575, 381766000,
            381082702, 380399682, 379716941, 379034480, 378352300, 377670402, 376988787, 376307456,
            375626410, 374945650, 374265177, 373584992, 372905096, 372225490, 371546175, 370867152,
            370188422, 369509986, 368831845, 368154000, 367476452, 366799202, 366122251, 365445600,
            364769250, 364093202, 363417457, 362742016, 362066880, 361392050, 360717527, 360043312,
            359369406, 358695810, 358022525, 357349552, 356676892, 356004546, 355332515, 354660800,
            353989402, 353318322, 352647561, 351977120, 351307000, 350637202, 349967727, 349298576,
            348629750, 347961250, 347293077, 346625232, 345957716, 345290530, 344623675, 343957152,
            343290962, 342625106, 341959585, 341294400, 340629552, 339965042, 339300871, 338637040,
            337973550, 337310402, 336647597, 335985136, 335323020, 334661250, 333999827, 333338752,
            332678026, 332017650, 331357625, 330697952, 330038632, 329379666, 328721055, 328062800,
            327404902, 326747362, 326090181, 325433360, 324776900, 324120802, 323465067, 322809696,
            322154690, 321500050, 320845777, 320191872, 319538336, 318885170, 318232375, 317579952,
            316927902, 316276226, 315624925, 314974000, 314323452, 313673282, 313023491, 312374080,
            311725050, 311076402, 310428137, 309780256, 309132760, 308485650, 307838927, 307192592,
            306546646, 305901090, 305255925, 304611152, 303966772, 303322786, 302679195, 302036000,
            301393202, 300750802, 300108801, 299467200, 298826000, 298185202, 297544807, 296904816,
            296265230, 295626050, 294987277, 294348912, 293710956, 293073410, 292436275, 291799552,
            291163242, 290527346, 289891865, 289256800, 288622152, 287987922, 287354111, 286720720,
            286087750, 285455202, 284823077, 284191376, 283560100, 282929250, 282298827, 281668832,
            281039266, 280410130, 279781425, 279153152, 278525312, 277897906, 277270935, 276644400,
            276018302, 275392642, 274767421, 274142640, 273518300, 272894402, 272270947, 271647936,
            271025370, 270403250, 269781577, 269160352, 268539576, 267919250, 267299375, 266679952,
            266060982, 265442466, 264824405, 264206800, 263589652, 262972962, 262356731, 261740960,
            261125650, 260510802, 259896417, 259282496, 258669040, 258056050, 257443527, 256831472,
            256219886, 255608770, 254998125, 254387952, 253778252, 253169026, 252560275, 251952000,
            251344202, 250736882, 250130041, 249523680, 248917800, 248312402, 247707487, 247103056,
            246499110, 245895650, 245292677, 244690192, 244088196, 243486690, 242885675, 242285152,
            241685122, 241085586, 240486545, 239888000, 239289952, 238692402, 238095351, 237498800,
            236902750, 236307202, 235712157, 235117616, 234523580, 233930050, 233337027, 232744512,
            232152506, 231561010, 230970025, 230379552, 229789592, 229200146, 228611215, 228022800,
            227434902, 226847522, 226260661, 225674320, 225088500, 224503202, 223918427, 223334176,
            222750450, 222167250, 221584577, 221002432, 220420816, 219839730, 219259175, 218679152,
            218099662, 217520706, 216942285, 216364400, 215787052, 215210242, 214633971, 214058240,
            213483050, 212908402, 212334297, 211760736, 211187720, 210615250, 210043327, 209471952,
            208901126, 208330850, 207761125, 207191952, 206623332, 206055266, 205487755, 204920800,
            204354402, 203788562, 203223281, 202658560, 202094400, 201530802, 200967767, 200405296,
            199843390, 199282050, 198721277, 198161072, 197601436, 197042370, 196483875, 195925952,
            195368602, 194811826, 194255625, 193700000, 193144952, 192590482, 192036591, 191483280,
            190930550, 190378402, 189826837, 189275856, 188725460, 188175650, 187626427, 187077792,
            186529746, 185982290, 185435425, 184889152, 184343472, 183798386, 183253895, 182710000,
            182166702, 181624002, 181081901, 180540400, 179999500, 179459202, 178919507, 178380416,
            177841930, 177304050, 176766777, 176230112, 175694056, 175158610, 174623775, 174089552,
            173555942, 173022946, 172490565, 171958800, 171427652, 170897122, 170367211, 169837920,
            169309250, 168781202, 168253777, 167726976, 167200800, 166675250, 166150327, 165626032,
            165102366, 164579330, 164056925, 163535152, 163014012, 162493506, 161973635, 161454400,
            160935802, 160417842, 159900521, 159383840, 158867800, 158352402, 157837647, 157323536,
            156810070, 156297250, 155785077, 155273552, 154762676, 154252450, 153742875, 153233952,
            152725682, 152218066, 151711105, 151204800, 150699152, 150194162, 149689831, 149186160,
            148683150, 148180802, 147679117, 147178096, 146677740, 146178050, 145679027, 145180672,
            144682986, 144185970, 143689625, 143193952, 142698952, 142204626, 141710975, 141218000,
            140725702, 140234082, 139743141, 139252880, 138763300, 138274402, 137786187, 137298656,
            136811810, 136325650, 135840177, 135355392, 134871296, 134387890, 133905175, 133423152,
            132941822, 132461186, 131981245, 131502000, 131023452, 130545602, 130068451, 129592000,
            129116250, 128641202, 128166857, 127693216, 127220280, 126748050, 126276527, 125805712,
            125335606, 124866210, 124397525, 123929552, 123462292, 122995746, 122529915, 122064800,
            121600402, 121136722, 120673761, 120211520, 119750000, 119289202, 118829127, 118369776,
            117911150, 117453250, 116996077, 116539632, 116083916, 115628930, 115174675, 114721152,
            114268362, 113816306, 113364985, 112914400, 112464552, 112015442, 111567071, 111119440,
            110672550, 110226402, 109780997, 109336336, 108892420, 108449250, 108006827, 107565152,
            107124226, 106684050, 106244625, 105805952, 105368032, 104930866, 104494455, 104058800,
            103623902, 103189762, 102756381, 102323760, 101891900, 101460802, 101030467, 100600896,
            100172090, 99744050, 99316777, 98890272, 98464536, 98039570, 97615375, 97191952,
            96769302, 96347426, 95926325, 95506000, 95086452, 94667682, 94249691, 93832480,
            93416050, 93000402, 92585537, 92171456, 91758160, 91345650, 90933927, 90522992,
            90112846, 89703490, 89294925, 88887152, 88480172, 88073986, 87668595, 87264000,
            86860202, 86457202, 86055001, 85653600, 85253000, 84853202, 84454207, 84056016,
            83658630, 83262050, 82866277, 82471312, 82077156, 81683810, 81291275, 80899552,
            80508642, 80118546, 79729265, 79340800, 78953152, 78566322, 78180311, 77795120,
            77410750, 77027202, 76644477, 76262576, 75881500, 75501250, 75121827, 74743232,
            74365466, 73988530, 73612425, 73237152, 72862712, 72489106, 72116335, 71744400,
            71373302, 71003042, 70633621, 70265040, 69897300, 69530402, 69164347, 68799136,
            68434770, 68071250, 67708577, 67346752, 66985776, 66625650, 66266375, 65907952,
            65550382, 65193666, 64837805, 64482800, 64128652, 63775362, 63422931, 63071360,
            62720650, 62370802, 62021817, 61673696, 61326440, 60980050, 60634527, 60289872,
            59946086, 59603170, 59261125, 58919952, 58579652, 58240226, 57901675, 57564000,
            57227202, 56891282, 56556241, 56222080, 55888800, 55556402, 55224887, 54894256,
            54564510, 54235650, 53907677, 53580592, 53254396, 52929090, 52604675, 52281152,
            51958522, 51636786, 51315945, 50996000, 50676952, 50358802, 50041551, 49725200,
            49409750, 49095202, 48781557, 48468816, 48156980, 47846050, 47536027, 47226912,
            46918706, 46611410, 46305025, 45999552, 45694992, 45391346, 45088615, 44786800,
            44485902, 44185922, 43886861, 43588720, 43291500, 42995202, 42699827, 42405376,
            42111850, 41819250, 41527577, 41236832, 40947016, 40658130, 40370175, 40083152,
            39797062, 39511906, 39227685, 38944400, 38662052, 38380642, 38100171, 37820640,
            37542050, 37264402, 36987697, 36711936, 36437120, 36163250, 35890327, 35618352,
            35347326, 35077250, 34808125, 34539952, 34272732, 34006466, 33741155, 33476800,
            33213402, 32950962, 32689481, 32428960, 32169400, 31910802, 31653167, 31396496,
            31140790, 30886050, 30632277, 30379472, 30127636, 29876770, 29626875, 29377952,
            29130002, 28883026, 28637025, 28392000, 28147952, 27904882, 27662791, 27421680,
            27181550, 26942402, 26704237, 26467056, 26230860, 25995650, 25761427, 25528192,
            25295946, 25064690, 24834425, 24605152, 24376872, 24149586, 23923295, 23698000,
            23473702, 23250402, 23028101, 22806800, 22586500, 22367202, 22148907, 21931616,
            21715330, 21500050, 21285777, 21072512, 20860256, 20649010, 20438775, 20229552,
            20021342, 19814146, 19607965, 19402800, 19198652, 18995522, 18793411, 18592320,
            18392250, 18193202, 17995177, 17798176, 17602200, 17407250, 17213327, 17020432,
            16828566, 16637730, 16447925, 16259152, 16071412, 15884706, 15699035, 15514400,
            15330802, 15148242, 14966721, 14786240, 14606800, 14428402, 14251047, 14074736,
            13899470, 13725250, 13552077, 13379952, 13208876, 13038850, 12869875, 12701952,
            12535082, 12369266, 12204505, 12040800, 11878152, 11716562, 11556031, 11396560,
            11238150, 11080802, 10924517, 10769296, 10615140, 10462050, 10310027, 10159072,
            10009186, 9860370, 9712625, 9565952, 9420352, 9275826, 9132375, 8990000, 8848702,
            8708482, 8569341, 8431280, 8294300, 8158402, 8023587, 7889856, 7757210, 7625650,
            7495177, 7365792, 7237496, 7110290, 6984175, 6859152, 6735222, 6612386, 6490645,
            6370000, 6250452, 6132002, 6014651, 5898400, 5783250, 5669202, 5556257, 5444416,
            5333680, 5224050, 5115527, 5008112, 4901806, 4796610, 4692525, 4589552, 4487692,
            4386946, 4287315, 4188800, 4091402, 3995122, 3899961, 3805920, 3713000, 3621202,
            3530527, 3440976, 3352550, 3265250, 3179077, 3094032, 3010116, 2927330, 2845675,
            2765152, 2685762, 2607506, 2530385, 2454400, 2379552, 2305842, 2233271, 2161840,
            2091550, 2022402, 1954397, 1887536, 1821820, 1757250, 1693827, 1631552, 1570426,
            1510450, 1451625, 1393952, 1337432, 1282066, 1227855, 1174800, 1122902, 1072162,
            1022581, 974160, 926900, 880802, 835867, 792096, 749490, 708050, 667777, 628672,
            590736, 553970, 518375, 483952, 450702, 418626, 387725, 358000, 329452, 302082, 275891,
            250880, 227050, 204402, 182937, 162656, 143560, 125650, 108927, 93392, 79046, 65890,
            53925, 43152, 33572, 25186, 17995, 12000, 7202, 3602, 1201, 0,
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
    _limbs_mul_greater_to_out_basecase(&mut out_limbs, xs, ys);
    let n = Natural::from_limbs_asc(xs) * Natural::from_limbs_asc(ys);
    let len = xs.len() + ys.len();
    let mut limbs = n.into_limbs_asc();
    limbs.resize(len, 0);
    assert_eq!(limbs, &out_limbs[..len]);
    assert_eq!(&out_limbs[len..], &old_out_limbs[len..]);
    out_limbs
}

#[test]
fn limbs_mul_greater_to_out_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_10,
        |&(ref out_limbs, ref xs, ref ys)| {
            let expected_out_limbs = limbs_mul_basecase_helper(out_limbs, xs, ys);
            let mut out_limbs = out_limbs.to_vec();
            let highest_result_limb = limbs_mul_greater_to_out(&mut out_limbs, xs, ys);
            assert_eq!(highest_result_limb, out_limbs[xs.len() + ys.len() - 1]);
            assert_eq!(out_limbs, expected_out_limbs);
        },
    );
}

#[test]
fn limbs_mul_greater_to_out_toom_22_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_11,
        |&(ref out_limbs, ref xs, ref ys)| {
            let expected_out_limbs = limbs_mul_basecase_helper(out_limbs, xs, ys);
            let mut out_limbs = out_limbs.to_vec();
            let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_22_scratch_size(xs.len())];
            _limbs_mul_greater_to_out_toom_22(&mut out_limbs, xs, ys, &mut scratch);
            assert_eq!(out_limbs, expected_out_limbs);
        },
    );
}

#[test]
fn limbs_mul_greater_to_out_toom_32_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_12,
        |&(ref out_limbs, ref xs, ref ys)| {
            let expected_out_limbs = limbs_mul_basecase_helper(out_limbs, xs, ys);
            let mut out_limbs = out_limbs.to_vec();
            let mut scratch =
                vec![0; _limbs_mul_greater_to_out_toom_32_scratch_size(xs.len(), ys.len())];
            _limbs_mul_greater_to_out_toom_32(&mut out_limbs, xs, ys, &mut scratch);
            assert_eq!(out_limbs, expected_out_limbs);
        },
    );
}

#[test]
fn limbs_mul_greater_to_out_toom_33_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_13,
        |&(ref out_limbs, ref xs, ref ys)| {
            let expected_out_limbs = limbs_mul_basecase_helper(out_limbs, xs, ys);
            let mut out_limbs = out_limbs.to_vec();
            let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_33_scratch_size(xs.len())];
            _limbs_mul_greater_to_out_toom_33(&mut out_limbs, xs, ys, &mut scratch);
            assert_eq!(out_limbs, expected_out_limbs);
        },
    );
}

#[test]
fn limbs_mul_greater_to_out_toom_42_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_14,
        |&(ref out_limbs, ref xs, ref ys)| {
            let expected_out_limbs = limbs_mul_basecase_helper(out_limbs, xs, ys);
            let mut out_limbs = out_limbs.to_vec();
            let mut scratch =
                vec![0; _limbs_mul_greater_to_out_toom_42_scratch_size(xs.len(), ys.len())];
            _limbs_mul_greater_to_out_toom_42(&mut out_limbs, xs, ys, &mut scratch);
            assert_eq!(out_limbs, expected_out_limbs);
        },
    );
}

#[test]
fn limbs_mul_greater_to_out_toom_43_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_15,
        |&(ref out_limbs, ref xs, ref ys)| {
            let expected_out_limbs = limbs_mul_basecase_helper(out_limbs, xs, ys);
            let mut out_limbs = out_limbs.to_vec();
            let mut scratch =
                vec![0; _limbs_mul_greater_to_out_toom_43_scratch_size(xs.len(), ys.len())];
            _limbs_mul_greater_to_out_toom_43(&mut out_limbs, xs, ys, &mut scratch);
            assert_eq!(out_limbs, expected_out_limbs);
        },
    );
}

#[test]
fn limbs_mul_greater_to_out_toom_44_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_16,
        |&(ref out_limbs, ref xs, ref ys)| {
            let expected_out_limbs = limbs_mul_basecase_helper(out_limbs, xs, ys);
            let mut out_limbs = out_limbs.to_vec();
            let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_44_scratch_size(xs.len())];
            _limbs_mul_greater_to_out_toom_44(&mut out_limbs, xs, ys, &mut scratch);
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
