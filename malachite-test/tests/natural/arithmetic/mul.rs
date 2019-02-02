use common::test_properties_custom_scale;
use malachite_base::num::{One, Zero};
use malachite_nz::natural::arithmetic::mul::toom::{
    _limbs_mul_greater_to_out_toom_22, _limbs_mul_greater_to_out_toom_22_scratch_size,
    _limbs_mul_greater_to_out_toom_32, _limbs_mul_greater_to_out_toom_32_scratch_size,
    _limbs_mul_greater_to_out_toom_33, _limbs_mul_greater_to_out_toom_33_scratch_size,
    _limbs_mul_greater_to_out_toom_42, _limbs_mul_greater_to_out_toom_42_scratch_size,
    _limbs_mul_greater_to_out_toom_43, _limbs_mul_greater_to_out_toom_43_scratch_size,
    _limbs_mul_greater_to_out_toom_44, _limbs_mul_greater_to_out_toom_44_scratch_size,
    _limbs_mul_greater_to_out_toom_52, _limbs_mul_greater_to_out_toom_52_scratch_size,
    _limbs_mul_greater_to_out_toom_53, _limbs_mul_greater_to_out_toom_53_scratch_size,
    _limbs_mul_greater_to_out_toom_54, _limbs_mul_greater_to_out_toom_54_scratch_size,
    _limbs_mul_greater_to_out_toom_62, _limbs_mul_greater_to_out_toom_62_scratch_size,
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
    triples_of_unsigned_vec_var_15, triples_of_unsigned_vec_var_16, triples_of_unsigned_vec_var_17,
    triples_of_unsigned_vec_var_18, triples_of_unsigned_vec_var_19, triples_of_unsigned_vec_var_20,
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
#[should_panic]
fn limbs_mul_greater_to_out_fail_1() {
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_greater_to_out_basecase(&mut out, &[6, 7, 8], &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_fail_2() {
    let mut out = vec![10, 10, 10, 10, 10];
    _limbs_mul_greater_to_out_basecase(&mut out, &[6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
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
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_22_fail_1() {
    let mut scratch = vec![];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_22(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_22_fail_2() {
    let mut scratch = vec![];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_22(&mut out, &[6, 7, 8], &[1, 2, 3, 4], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_22_fail_3() {
    let mut scratch = vec![];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_22(&mut out, &[6, 7, 8, 9], &[1, 2], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_22_fail_4() {
    let mut scratch = vec![];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_22(&mut out, &[6, 7, 8], &[1, 2], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_22_fail_5() {
    let mut scratch = vec![];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_22(&mut out, &[6, 7, 8], &[1, 2, 3], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_22_fail_6() {
    let mut scratch = vec![];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_22(&mut out, &[6, 7, 8], &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
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
#[should_panic]
fn limbs_mul_greater_to_out_toom_32_fail_1() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_32_scratch_size(1, 1)];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_32(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_32_fail_2() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_32_scratch_size(3, 4)];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_32(&mut out, &[6, 7, 8], &[1, 2, 3, 4], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_32_fail_3() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_32_scratch_size(5, 4)];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_32(&mut out, &[6, 7, 8, 9, 10], &[1, 2, 3, 4], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_32_fail_4() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_32_scratch_size(6, 3)];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_32(&mut out, &[6, 7, 8, 9, 10, 11], &[1, 2, 3], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_32_fail_5() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_32_scratch_size(3, 0)];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_32(&mut out, &[6, 7, 8], &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
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
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_33_fail_1() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_33_scratch_size(1)];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_33(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
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
#[should_panic]
fn limbs_mul_greater_to_out_toom_33_fail_3() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_33_scratch_size(5)];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_33(&mut out, &[6, 7, 8, 9, 10], &[1, 2, 3, 4], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_33_fail_4() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_33_scratch_size(5)];
    let mut out = vec![10, 10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_33(&mut out, &[6, 7, 8, 9, 10], &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
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
#[should_panic]
fn limbs_mul_greater_to_out_toom_42_fail_1() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_42_scratch_size(1, 1)];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_42(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
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
#[should_panic]
fn limbs_mul_greater_to_out_toom_42_fail_3() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_42_scratch_size(3, 2)];
    let mut out = vec![10, 10, 10, 10, 10, 10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_42(&mut out, &[6, 7, 8], &[1, 2], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_42_fail_4() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_42_scratch_size(5, 0)];
    let mut out = vec![10, 10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_42(&mut out, &[6, 7, 8, 9, 10], &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
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
#[should_panic]
fn limbs_mul_greater_to_out_toom_43_fail_1() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_43_scratch_size(1, 1)];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_43(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
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
#[should_panic]
fn limbs_mul_greater_to_out_toom_43_fail_3() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_43_scratch_size(11, 8)];
    let mut out = vec![10; 19];
    _limbs_mul_greater_to_out_toom_43(
        &mut out,
        &[3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
        &[2, 3, 4, 5, 6, 7, 8, 9],
        &mut scratch,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
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
#[should_panic]
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
    test(
        vec![
            986333060, 254638637, 1577120658, 1458096412, 474582958, 4115735719, 4031007047,
        ],
        vec![
            2096725444, 3871299248, 1414038108, 2617834141, 1553210626, 2669030715, 3093885541,
        ],
        vec![10; 14],
        vec![
            2067797264, 3922708625, 2600678884, 825822853, 2499590824, 1035492325, 1957325707,
            1890833276, 3433274404, 1510974136, 2269171082, 854613327, 1796482159, 2903741417,
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_44_fail_1() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_44_scratch_size(1)];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_44(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_44_fail_2() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_44_scratch_size(4)];
    let mut out = vec![10; 9];
    _limbs_mul_greater_to_out_toom_44(&mut out, &[3, 4, 5, 6], &[2, 3, 4, 5, 6], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_44_fail_3() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_44_scratch_size(3)];
    let mut out = vec![10; 6];
    _limbs_mul_greater_to_out_toom_44(&mut out, &[3, 4, 5], &[2, 3, 4], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_44_fail_4() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_44_scratch_size(11)];
    let mut out = vec![10; 11];
    _limbs_mul_greater_to_out_toom_44(
        &mut out,
        &[3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
        &[],
        &mut scratch,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_44_fail_5() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_44_scratch_size(4)];
    let mut out = vec![10; 7];
    _limbs_mul_greater_to_out_toom_44(&mut out, &[3, 4, 5, 6], &[2, 3, 4, 5], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_52() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch =
            vec![0; _limbs_mul_greater_to_out_toom_52_scratch_size(xs.len(), ys.len())];
        _limbs_mul_greater_to_out_toom_52(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    // degree.even() in _limbs_mul_toom_evaluate_poly_in_2_and_neg_2
    // !v_neg_2_neg in _limbs_mul_toom_evaluate_poly_in_2_and_neg_2
    // t != n
    // !(limbs_test_zero(&ys_0[t..]) && limbs_cmp_same_length(&ys_0[..t], ys_1) == Ordering::Less)
    // !v_neg_1_neg
    // !(limbs_test_zero(&bsm1[t..]) && limbs_cmp_same_length(&bsm1[..t], ys_1) == Ordering::Less)
    // k & 1 == 0 in _limbs_mul_toom_evaluate_poly_in_1_and_neg_1
    // neg == 0 in _limbs_mul_toom_evaluate_poly_in_1_and_neg_1
    test(
        vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        vec![3, 4, 5, 6, 7],
        vec![10; 20],
        vec![
            6, 17, 34, 58, 90, 115, 140, 165, 190, 215, 240, 265, 290, 315, 340, 314, 268, 201,
            112, 0,
        ],
    );
    test(
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 1, 0],
        vec![10; 20],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
    );
    // n_high != n in _limbs_mul_toom_evaluate_poly_in_2_and_neg_2
    // t == n
    // limbs_cmp_same_length(ys_0, ys_1) == Ordering::Less
    // v_neg_1_neg
    test(
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0, 1],
        vec![10; 20],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    );
    // limbs_test_zero(&ys_0[t..]) && limbs_cmp_same_length(&ys_0[..t], ys_1) == Ordering::Less
    test(
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 1],
        vec![10; 20],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    );
    // limbs_cmp_same_length(ys_0, ys_1) != Ordering::Less
    // limbs_cmp_same_length(bsm1, ys_1) == Ordering::Less
    test(
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 1, 0, 0, 1],
        vec![10; 20],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0],
    );
    // v_neg_2_neg in _limbs_mul_toom_evaluate_poly_in_2_and_neg_2
    // neg != 0 in _limbs_mul_toom_evaluate_poly_in_1_and_neg_1
    // _limbs_mul_toom_evaluate_poly_in_1_and_neg_1(as1, asm1, 4, xs, n, s, &mut v_neg_1[..m])
    test(
        vec![
            281500646, 1572406350, 108746052, 4056047843, 89307364, 1006007374, 2902260577,
            1250995384, 1556873818, 3846421711, 280743259, 1728158805, 467926284, 2330565417,
        ],
        vec![
            2509320863, 2201587434, 926371577, 1243694325, 1112023631, 2791032478,
        ],
        vec![10; 20],
        vec![
            1191903194, 1277561191, 2672986331, 45667421, 2742410814, 2602170945, 2815699572,
            2317624023, 952805243, 577394769, 1002744907, 4175910221, 2433548489, 2550394831,
            3650814344, 1121996596, 3441179979, 3561879910, 1574546788, 1514489709,
        ],
    );
    // limbs_cmp_same_length(bsm1, ys_1) != Ordering::Less
    test(
        vec![
            2331447040, 1003213663, 1873981685, 3371337621, 3796896013, 4144448610, 2569252563,
            2859304641, 1027973602, 3158196152, 4058699545, 2002924383, 3295505824, 695758308,
        ],
        vec![
            725028139, 2984864771, 2939417227, 3047223286, 3526157986, 1078000342,
        ],
        vec![10; 20],
        vec![
            474121472, 1561322164, 715684992, 3182777436, 384530074, 3827205870, 2267366778,
            1586160630, 3779201468, 900553139, 2867049131, 2027414411, 2054056558, 2671776484,
            3374007062, 3091178442, 1888125000, 2974781424, 307612679, 174629431,
        ],
    );
    // limbs_test_zero(&bsm1[t..]) && limbs_cmp_same_length(&bsm1[..t], ys_1) == Ordering::Less
    test(
        vec![
            32767, 0, 0, 0, 0, 0, 4294836224, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295, 4278206463, 4294967295, 4294967295, 31, 0, 0, 4294443008, 4294967295,
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        ],
        vec![
            0, 0, 4294967295, 4294967295, 4294967295, 0, 0, 4294967232, 4294967295, 4227858559,
            4294967295,
        ],
        vec![10; 37],
        vec![
            0, 0, 4294934529, 4294967295, 4294967295, 32766, 0, 4292870208, 131071, 71303040,
            4294966784, 4294868990, 4294967295, 8388607, 16760832, 4278190080, 2047, 4278075360,
            4294967295, 1072693247, 524320, 2149580800, 259839, 4277682176, 2147487743, 33554431,
            32, 4227858432, 8190, 4294443008, 4294967295, 0, 0, 4294967232, 4294967295, 4227858559,
            4294967295,
        ],
    );
}

#[cfg(feature = "64_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_52() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch =
            vec![0; _limbs_mul_greater_to_out_toom_52_scratch_size(xs.len(), ys.len())];
        _limbs_mul_greater_to_out_toom_52(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    test(
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 1, 0],
        vec![10; 20],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_52_fail_1() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_52_scratch_size(1, 1)];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_52(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_52_fail_2() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_52_scratch_size(15, 16)];
    let mut out = vec![10; 9];
    _limbs_mul_greater_to_out_toom_52(
        &mut out,
        &[3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
        &[3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18],
        &mut scratch,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_52_fail_3() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_52_scratch_size(14, 5)];
    let mut out = vec![10; 6];
    _limbs_mul_greater_to_out_toom_52(
        &mut out,
        &[3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        &[3, 4, 5, 6, 7],
        &mut scratch,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_52_fail_4() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_52_scratch_size(15, 4)];
    let mut out = vec![10; 7];
    _limbs_mul_greater_to_out_toom_52(
        &mut out,
        &[3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
        &[3, 4, 5, 6],
        &mut scratch,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_52_fail_5() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_52_scratch_size(11, 0)];
    let mut out = vec![10; 12];
    _limbs_mul_greater_to_out_toom_52(
        &mut out,
        &[3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
        &[],
        &mut scratch,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_53() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch =
            vec![0; _limbs_mul_greater_to_out_toom_53_scratch_size(xs.len(), ys.len())];
        _limbs_mul_greater_to_out_toom_53(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    // !(*bs1_last == 0 && limbs_cmp_same_length(bs1_init, ys_1) == Ordering::Less)
    // limbs_cmp_same_length(bs2, &out[..n + 1]) != Ordering::Less
    // *asm1_last != 1 && *asm1_last != 2
    // *as1_last == 0
    test(
        vec![2, 3, 4, 5, 6],
        vec![3, 4, 5],
        vec![10; 8],
        vec![6, 17, 34, 46, 58, 49, 30, 0],
    );
    // *bs1_last == 0 && limbs_cmp_same_length(bs1_init, ys_1) == Ordering::Less
    // *as1_last == 2
    // *bs1_last == 1
    test(
        vec![611094747, 2426195984, 3948451542, 3575143460, 2163084716],
        vec![1043494367, 2432724646, 1148376235],
        vec![10; 8],
        vec![
            2911651269, 2135822080, 566305911, 1285474929, 3971527858, 1120629777, 2330897846,
            578359487,
        ],
    );
    // *as1_last == 1
    test(
        vec![83336617, 52963853, 1461131367, 615175494, 2376138249],
        vec![1085015601, 823246134, 3222784883],
        vec![10; 8],
        vec![
            4003668825, 3129188105, 1975732797, 2019047981, 943873016, 1483316813, 305883771,
            1782966412,
        ],
    );
    // limbs_cmp_same_length(bs2, &out[..n + 1]) == Ordering::Less
    // *as1_last > 2
    test(
        vec![
            3853679659, 4236745288, 2469732913, 4265854402, 4207372271, 1754370134, 137881047,
            1325109821, 2212043812, 3074170203,
        ],
        vec![
            1666773246, 4177391250, 4175984066, 2859904653, 3320165100, 314964734,
        ],
        vec![10; 16],
        vec![
            2336719530, 919351696, 4142757378, 49781824, 1315087108, 2534950116, 2674417418,
            1178559126, 171926136, 3132896187, 2074730624, 3561766617, 1155879861, 3985534229,
            380101898, 225439482,
        ],
    );
    // *asm1_last == 1
    test(
        vec![4171807709, 1363035595, 2692148345, 3728232161, 2672522097],
        vec![178202067, 736149219, 623937260],
        vec![10; 8],
        vec![
            2793195559, 2168235304, 1582195836, 18437203, 671570200, 635034059, 2378259056,
            388241865,
        ],
    );
    // *bs1_last == 2
    test(
        vec![361692441, 3665267779, 1770324312, 1221560416, 2810295690],
        vec![1887715703, 4035171395, 2815003797],
        vec![10; 8],
        vec![
            3298754463, 2516900264, 30373680, 909364693, 729609199, 3973437903, 3392713387,
            1841921601,
        ],
    );
    // *bsm1_last != 0
    test(
        vec![1542637461, 595638956, 1883922642, 2681579369, 2641006916],
        vec![3723002977, 116606811, 2193352864],
        vec![10; 8],
        vec![
            2246996853, 3232877055, 2347711939, 2476049074, 4132376421, 3855440382, 4040315714,
            1348708775,
        ],
    );
    // *asm1_last == 2
    test(
        vec![4043423637, 312331403, 3088235367, 41162462, 2934893364],
        vec![2702987034, 4184574368, 2455116868],
        vec![10; 8],
        vec![
            2912448546, 2297161059, 137328692, 115875329, 1975003140, 2441893159, 4034859213,
            1677662647,
        ],
    );
    test(
        vec![4194296, 3221225472, 4294967295, 1, 4294934528],
        vec![0, 4294959104, 4294967295],
        vec![10; 8],
        vec![
            0, 65536, 4294967288, 4196343, 3221209088, 268435455, 4294959106, 4294934527,
        ],
    );
}

#[cfg(feature = "64_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_53() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch =
            vec![0; _limbs_mul_greater_to_out_toom_53_scratch_size(xs.len(), ys.len())];
        _limbs_mul_greater_to_out_toom_53(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    test(
        vec![
            18446744073709551615,
            18446744073709551615,
            3,
            18446744073709551614,
            18446744073709551615,
        ],
        vec![
            18446744073709551615,
            18446744073709551615,
            18446744073709551615,
        ],
        vec![10; 8],
        vec![
            1,
            0,
            18446744073709551612,
            0,
            0,
            3,
            18446744073709551614,
            18446744073709551615,
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_53_fail_1() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_53_scratch_size(1, 1)];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_53(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_53_fail_2() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_53_scratch_size(5, 6)];
    let mut out = vec![10; 11];
    _limbs_mul_greater_to_out_toom_53(
        &mut out,
        &[3, 4, 5, 6, 7],
        &[3, 4, 5, 6, 7, 8],
        &mut scratch,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_53_fail_3() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_53_scratch_size(5, 4)];
    let mut out = vec![10; 9];
    _limbs_mul_greater_to_out_toom_53(&mut out, &[3, 4, 5, 6, 7], &[3, 4, 5, 6], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_53_fail_4() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_53_scratch_size(4, 3)];
    let mut out = vec![10; 6];
    _limbs_mul_greater_to_out_toom_53(&mut out, &[3, 4, 5, 6], &[3, 4, 5], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_53_fail_5() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_53_scratch_size(5, 2)];
    let mut out = vec![10; 7];
    _limbs_mul_greater_to_out_toom_53(&mut out, &[3, 4, 5, 6, 7], &[3, 4], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_53_fail_6() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_53_scratch_size(5, 0)];
    let mut out = vec![10; 12];
    _limbs_mul_greater_to_out_toom_53(&mut out, &[3, 4, 5, 6, 7], &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_54() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch =
            vec![0; _limbs_mul_greater_to_out_toom_54_scratch_size(xs.len(), ys.len())];
        _limbs_mul_greater_to_out_toom_54(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    // degree.even() in _limbs_mul_toom_evaluate_poly_in_2_pow_and_neg_2_pow
    // !v_neg_2_pow_neg in _limbs_mul_toom_evaluate_poly_in_2_pow_and_neg_2_pow
    // degree.odd() in _limbs_mul_toom_evaluate_poly_in_2_pow_and_neg_2_pow
    // !y_sign in _limbs_toom_couple_handling
    // y_shift != 0 in _limbs_toom_couple_handling
    // x_shift != 0 in _limbs_toom_couple_handling
    // s > t
    // carry_1 && !carry_2, first time, in _limbs_mul_toom_interpolate_8_points
    // carry_1 && !carry_2, second time, in _limbs_mul_toom_interpolate_8_points
    // s_plus_t != n in _limbs_mul_toom_interpolate_8_points
    test(
        vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        vec![3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
        vec![10; 26],
        vec![
            6, 17, 34, 58, 90, 131, 182, 244, 318, 405, 506, 594, 682, 770, 858, 895, 912, 908,
            882, 833, 760, 662, 538, 387, 208, 0,
        ],
    );
    // v_neg_2_pow_neg in _limbs_mul_toom_evaluate_poly_in_2_pow_and_neg_2_pow
    // y_sign in _limbs_toom_couple_handling
    // s <= t
    test(
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![10; 26],
        vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
        ],
    );
    // !carry_1 && carry_2, first time, in _limbs_mul_toom_interpolate_8_points
    test(
        vec![
            281500646, 1572406350, 108746052, 4056047843, 89307364, 1006007374, 2902260577,
            1250995384, 1556873818, 3846421711, 280743259, 1728158805, 467926284, 2330565417,
        ],
        vec![
            1365578038, 3224231142, 4103857906, 475734936, 3828952167, 3071966456, 1450111251,
            1166414077, 2218130537, 3324650407, 1559641024, 2423373264,
        ],
        vec![10; 26],
        vec![
            3471157380, 2179990259, 735116018, 3928626279, 2606792426, 4065628313, 3326106964,
            1358767242, 58836620, 2388814047, 1881937395, 448453590, 699295041, 2539838591,
            1014080982, 2627397171, 1231543630, 2956184941, 1108982880, 2083442227, 1445361702,
            3773463966, 3902311612, 4169089467, 614631841, 1314987876,
        ],
    );
    // s_plus_t == n in _limbs_mul_toom_interpolate_8_points
    test(
        vec![
            1372428912, 2999825770, 3824933735, 1252466299, 1644332514, 577056155, 267504800,
            2188417248, 1146838357, 1601878440, 2555350187, 2326995902, 341200833, 3311243465,
            3983323172, 1591023018, 498264278, 879686658, 2445286712, 3168806215, 3363960673,
            1002293448,
        ],
        vec![
            4155394173, 3251572031, 3012777338, 1405107169, 4263655764, 202386116, 2762119705,
            1046271690, 3730474184, 1761497041, 3530189728, 452831577, 2351117985, 3074633806,
            2337874996, 2372535352, 1907593160, 2034262144,
        ],
        vec![10; 40],
        vec![
            3438536880, 4020840252, 3753658662, 2750457729, 3984463794, 1677702279, 3627178035,
            1938289829, 2347934241, 1059164524, 3077109858, 1455283397, 4245424824, 2265496611,
            2507273589, 4106853892, 187386657, 3541881161, 3520589236, 977961476, 205850208,
            3040196950, 1303835716, 3039701923, 525989195, 1042461957, 4189151284, 3358396344,
            275215531, 2907721257, 3086020483, 2914223316, 652103889, 2430562590, 4256409533,
            774831877, 3808631269, 3720895601, 1034939105, 474724830,
        ],
    );
    // !carry_1 && carry_2, second time, in _limbs_mul_toom_interpolate_8_points
    test(
        vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4294967280, 4294967295, 4294967295, 4294967295,
        ],
        vec![
            2047, 0, 0, 4294966784, 4294967295, 127, 0, 4286578688, 4294967295, 262143, 4227858432,
            4294967295,
        ],
        vec![10; 26],
        vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4294934544, 4294967295, 4294967295, 8191, 2047,
            4294965248, 4294967295, 134217215, 0, 4290773120, 1073741823, 4286578688, 4294967279,
            262143, 4227858432, 4294967295,
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_54_fail_1() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_54_scratch_size(1, 1)];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_54(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_54_fail_2() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_54_scratch_size(15, 16)];
    let mut out = vec![10; 31];
    _limbs_mul_greater_to_out_toom_54(
        &mut out,
        &[3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        &[3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18],
        &mut scratch,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_54_fail_3() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_54_scratch_size(15, 10)];
    let mut out = vec![10; 25];
    _limbs_mul_greater_to_out_toom_54(
        &mut out,
        &[3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        &[3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        &mut scratch,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_54_fail_4() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_54_scratch_size(14, 11)];
    let mut out = vec![10; 25];
    _limbs_mul_greater_to_out_toom_54(
        &mut out,
        &[3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        &[3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
        &mut scratch,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_54_fail_5() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_54_scratch_size(15, 11)];
    let mut out = vec![10; 25];
    _limbs_mul_greater_to_out_toom_54(
        &mut out,
        &[3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
        &[3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13],
        &mut scratch,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_54_fail_6() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_54_scratch_size(15, 0)];
    let mut out = vec![10; 15];
    _limbs_mul_greater_to_out_toom_54(
        &mut out,
        &[3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
        &[],
        &mut scratch,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_62() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch =
            vec![0; _limbs_mul_greater_to_out_toom_62_scratch_size(xs.len(), ys.len())];
        _limbs_mul_greater_to_out_toom_62(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    // k & 1 != 0 in _limbs_mul_toom_evaluate_poly_in_1_and_neg_1
    // degree_u >= 5 in _limbs_mul_toom_evaluate_poly_in_2_and_neg_2
    // degree.odd() in _limbs_mul_toom_evaluate_poly_in_2_and_neg_2
    // t == n
    // limbs_cmp_same_length(ys_0, ys_1) == Ordering::Less
    // v_neg_1_neg_b
    // *as1_last == 0
    test(
        vec![2, 3, 4, 5, 6, 7],
        vec![3, 4],
        vec![10; 8],
        vec![6, 17, 24, 31, 38, 45, 28, 0],
    );
    // limbs_cmp_same_length(ys_0, ys_1) != Ordering::Less
    // !v_neg_1_neg_b
    // t >= n
    // limbs_cmp_same_length(&bsm1, ys_1) == Ordering::Less
    test(
        vec![0, 0, 0, 0, 0, 1],
        vec![1, 1],
        vec![10; 8],
        vec![0, 0, 0, 0, 0, 1, 1, 0],
    );
    // limbs_cmp_same_length(&bsm1, ys_1) != Ordering::Less
    test(
        vec![0, 0, 0, 0, 0, 1],
        vec![2, 1],
        vec![10; 8],
        vec![0, 0, 0, 0, 0, 2, 1, 0],
    );
    // t != n
    // !(limbs_test_zero(&ys_0[t..]) && limbs_cmp_same_length(&ys_0[..t], ys_1) == Ordering::Less)
    // t < n
    // !(limbs_test_zero(&bsm1[t..]) && limbs_cmp_same_length(&bsm1[..t], ys_1) == Ordering::Less)
    // *as1_last == 2
    test(
        vec![
            2291585918, 1380546066, 1861205162, 1395600128, 1509813785, 1715266614, 3251195596,
            3140058077, 1998653517, 3140019184, 2534426976,
        ],
        vec![2477133296, 625749873, 3687467688],
        vec![10; 14],
        vec![
            869772320, 253774892, 3270059412, 1629301906, 333315526, 1485838973, 1182872659,
            3973435471, 3570040059, 138616924, 3845622124, 4243476600, 2488800838, 2175946157,
        ],
    );
    // *as1_last > 2
    test(
        vec![
            706760835, 4153647095, 3843998199, 2077172825, 1158686949, 3157624247,
        ],
        vec![708497006, 797041707],
        vec![10; 8],
        vec![
            3596223050, 1899342498, 3768933007, 59388593, 2997914214, 150267535, 1848145862,
            585978436,
        ],
    );
    // *as1_last == 1
    test(
        vec![
            1817453168, 96871997, 3927306877, 3090061646, 3474317652, 437148773, 439538568,
            324686794, 772632617, 1424328970, 580538580,
        ],
        vec![4158498322, 3126677346, 655989538],
        vec![10; 14],
        vec![
            4142861280, 2093741387, 1223409636, 3430701278, 154561385, 1065478559, 1434432315,
            1709306376, 2647162930, 2288715437, 510829208, 3519993529, 1581992297, 88668250,
        ],
    );
    // *bs1_last != 0
    test(
        vec![
            478149678, 4026802122, 1384639138, 368837837, 183900171, 785221208,
        ],
        vec![1458158767, 4167624669],
        vec![10; 8],
        vec![
            1921854322, 141249793, 673006993, 2183916852, 295623924, 3471440317, 3387755993,
            761939975,
        ],
    );
    // *asm1_last == 1
    test(
        vec![
            760464004, 3698115579, 1282981837, 2124133062, 1943175022, 3815903204,
        ],
        vec![2302225798, 423133196],
        vec![10; 8],
        vec![
            1718420760, 4288660832, 1043184986, 2518603664, 1668853787, 1047988481, 4101944437,
            375936580,
        ],
    );
    // *asm1_last == 2
    test(
        vec![
            486320673, 3488920730, 3556919186, 380261964, 1609664786, 3382076763, 3478178414,
            1464325754, 2543330707, 3900552438, 1432199477,
        ],
        vec![1190326122, 1081384689, 2610845505, 3919894794],
        vec![10; 15],
        vec![
            3164946602, 4284198222, 380177155, 837655879, 3034889727, 3503063664, 3315274214,
            3998279880, 2501466635, 3524441, 312561544, 2480833439, 3092764257, 1045878247,
            1307127829,
        ],
    );
    // limbs_test_zero(&ys_0[t..]) && limbs_cmp_same_length(&ys_0[..t], ys_1) == Ordering::Less
    test(
        vec![
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295, 4294967295, 4294967295, 31, 0, 0, 0, 0, 0, 4294967232, 4294967295,
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295, 4294967295, 4294967295, 4294967295,
        ],
        vec![
            4294967295, 63, 0, 0, 0, 0, 4227858432, 4294967295, 4294967295, 4294967295, 4294967295,
        ],
        vec![10; 47],
        vec![
            1, 4294967232, 4294967295, 4294967295, 4294967295, 4294967295, 67108863, 0, 0, 0, 0,
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967263,
            2047, 0, 0, 0, 0, 2147483712, 4294963199, 4294967295, 4294967295, 4294967295, 31, 0, 1,
            0, 0, 0, 4294967232, 4294967295, 4294967294, 63, 0, 0, 0, 0, 4227858432, 4294967295,
            4294967295, 4294967295, 4294967295,
        ],
    );
    // limbs_test_zero(&bsm1[t..]) && limbs_cmp_same_length(&bsm1[..t], ys_1) == Ordering::Less
    test(
        vec![
            1073741823, 0, 0, 0, 0, 0, 0, 0, 4290772992, 4294967295, 4294967295, 4294967295,
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        ],
        vec![4293918720, 4294967295, 0, 268435448, 4294443008],
        vec![10; 22],
        vec![
            1048576, 4294705152, 1073741822, 4026531848, 67633149, 1073610751, 0, 0, 0, 1024,
            4290772992, 33554431, 4294705152, 4290773503, 4294967295, 4294967295, 4294967295,
            4293918719, 4294967295, 0, 268435448, 4294443008,
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_62_fail_1() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_62_scratch_size(1, 1)];
    let mut out = vec![10, 10, 10, 10];
    _limbs_mul_greater_to_out_toom_62(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_62_fail_2() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_62_scratch_size(6, 7)];
    let mut out = vec![10; 13];
    _limbs_mul_greater_to_out_toom_62(
        &mut out,
        &[3, 4, 5, 6, 7, 8],
        &[3, 4, 5, 6, 7, 8, 9],
        &mut scratch,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_62_fail_3() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_62_scratch_size(6, 1)];
    let mut out = vec![10; 7];
    _limbs_mul_greater_to_out_toom_62(&mut out, &[3, 4, 5, 6, 7, 8], &[3], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_62_fail_4() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_62_scratch_size(5, 2)];
    let mut out = vec![10; 7];
    _limbs_mul_greater_to_out_toom_62(&mut out, &[3, 4, 5, 6, 7], &[3, 4], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_62_fail_5() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_62_scratch_size(6, 2)];
    let mut out = vec![10; 7];
    _limbs_mul_greater_to_out_toom_62(&mut out, &[3, 4, 5, 6, 7, 8], &[3, 4], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_62_fail_6() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_62_scratch_size(6, 0)];
    let mut out = vec![10; 6];
    _limbs_mul_greater_to_out_toom_62(&mut out, &[3, 4, 5, 6, 7, 8], &[], &mut scratch);
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

fn limbs_mul_basecase_helper(out: &Vec<Limb>, xs: &Vec<Limb>, ys: &Vec<Limb>) -> Vec<Limb> {
    let mut out = out.to_vec();
    let old_out = out.clone();
    _limbs_mul_greater_to_out_basecase(&mut out, xs, ys);
    let n = Natural::from_limbs_asc(xs) * Natural::from_limbs_asc(ys);
    let len = xs.len() + ys.len();
    let mut limbs = n.into_limbs_asc();
    limbs.resize(len, 0);
    assert_eq!(limbs, &out[..len]);
    assert_eq!(&out[len..], &old_out[len..]);
    out
}

#[test]
fn limbs_mul_greater_to_out_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_10,
        |&(ref out, ref xs, ref ys)| {
            let expected_out = limbs_mul_basecase_helper(out, xs, ys);
            let mut out = out.to_vec();
            let highest_result_limb = limbs_mul_greater_to_out(&mut out, xs, ys);
            assert_eq!(highest_result_limb, out[xs.len() + ys.len() - 1]);
            assert_eq!(out, expected_out);
        },
    );
}

#[test]
fn limbs_mul_greater_to_out_toom_22_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_11,
        |&(ref out, ref xs, ref ys)| {
            let expected_out = limbs_mul_basecase_helper(out, xs, ys);
            let mut out = out.to_vec();
            let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_22_scratch_size(xs.len())];
            _limbs_mul_greater_to_out_toom_22(&mut out, xs, ys, &mut scratch);
            assert_eq!(out, expected_out);
        },
    );
}

#[test]
fn limbs_mul_greater_to_out_toom_32_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_12,
        |&(ref out, ref xs, ref ys)| {
            let expected_out = limbs_mul_basecase_helper(out, xs, ys);
            let mut out = out.to_vec();
            let mut scratch =
                vec![0; _limbs_mul_greater_to_out_toom_32_scratch_size(xs.len(), ys.len())];
            _limbs_mul_greater_to_out_toom_32(&mut out, xs, ys, &mut scratch);
            assert_eq!(out, expected_out);
        },
    );
}

#[test]
fn limbs_mul_greater_to_out_toom_33_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_13,
        |&(ref out, ref xs, ref ys)| {
            let expected_out = limbs_mul_basecase_helper(out, xs, ys);
            let mut out = out.to_vec();
            let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_33_scratch_size(xs.len())];
            _limbs_mul_greater_to_out_toom_33(&mut out, xs, ys, &mut scratch);
            assert_eq!(out, expected_out);
        },
    );
}

#[test]
fn limbs_mul_greater_to_out_toom_42_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_14,
        |&(ref out, ref xs, ref ys)| {
            let expected_out = limbs_mul_basecase_helper(out, xs, ys);
            let mut out = out.to_vec();
            let mut scratch =
                vec![0; _limbs_mul_greater_to_out_toom_42_scratch_size(xs.len(), ys.len())];
            _limbs_mul_greater_to_out_toom_42(&mut out, xs, ys, &mut scratch);
            assert_eq!(out, expected_out);
        },
    );
}

#[test]
fn limbs_mul_greater_to_out_toom_43_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_15,
        |&(ref out, ref xs, ref ys)| {
            let expected_out = limbs_mul_basecase_helper(out, xs, ys);
            let mut out = out.to_vec();
            let mut scratch =
                vec![0; _limbs_mul_greater_to_out_toom_43_scratch_size(xs.len(), ys.len())];
            _limbs_mul_greater_to_out_toom_43(&mut out, xs, ys, &mut scratch);
            assert_eq!(out, expected_out);
        },
    );
}

#[test]
fn limbs_mul_greater_to_out_toom_44_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_16,
        |&(ref out, ref xs, ref ys)| {
            let expected_out = limbs_mul_basecase_helper(out, xs, ys);
            let mut out = out.to_vec();
            let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_44_scratch_size(xs.len())];
            _limbs_mul_greater_to_out_toom_44(&mut out, xs, ys, &mut scratch);
            assert_eq!(out, expected_out);
        },
    );
}

#[test]
fn limbs_mul_greater_to_out_toom_52_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_17,
        |&(ref out, ref xs, ref ys)| {
            let expected_out = limbs_mul_basecase_helper(out, xs, ys);
            let mut out = out.to_vec();
            let mut scratch =
                vec![0; _limbs_mul_greater_to_out_toom_52_scratch_size(xs.len(), ys.len())];
            _limbs_mul_greater_to_out_toom_52(&mut out, xs, ys, &mut scratch);
            assert_eq!(out, expected_out);
        },
    );
}

#[test]
fn limbs_mul_greater_to_out_toom_53_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_18,
        |&(ref out, ref xs, ref ys)| {
            let expected_out = limbs_mul_basecase_helper(out, xs, ys);
            let mut out = out.to_vec();
            let mut scratch =
                vec![0; _limbs_mul_greater_to_out_toom_53_scratch_size(xs.len(), ys.len())];
            _limbs_mul_greater_to_out_toom_53(&mut out, xs, ys, &mut scratch);
            assert_eq!(out, expected_out);
        },
    );
}

#[test]
fn limbs_mul_greater_to_out_toom_54_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_19,
        |&(ref out, ref xs, ref ys)| {
            let expected_out = limbs_mul_basecase_helper(out, xs, ys);
            let mut out = out.to_vec();
            let mut scratch =
                vec![0; _limbs_mul_greater_to_out_toom_54_scratch_size(xs.len(), ys.len())];
            _limbs_mul_greater_to_out_toom_54(&mut out, xs, ys, &mut scratch);
            assert_eq!(out, expected_out);
        },
    );
}

#[test]
fn limbs_mul_greater_to_out_toom_62_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_20,
        |&(ref out, ref xs, ref ys)| {
            let expected_out = limbs_mul_basecase_helper(out, xs, ys);
            let mut out = out.to_vec();
            let mut scratch =
                vec![0; _limbs_mul_greater_to_out_toom_62_scratch_size(xs.len(), ys.len())];
            _limbs_mul_greater_to_out_toom_62(&mut out, xs, ys, &mut scratch);
            assert_eq!(out, expected_out);
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
