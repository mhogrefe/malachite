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
    _limbs_mul_greater_to_out_toom_63, _limbs_mul_greater_to_out_toom_63_scratch_size,
    _limbs_mul_greater_to_out_toom_6h, _limbs_mul_greater_to_out_toom_6h_scratch_size,
    _limbs_mul_greater_to_out_toom_8h, _limbs_mul_greater_to_out_toom_8h_scratch_size,
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
    triples_of_unsigned_vec_var_21, triples_of_unsigned_vec_var_22, triples_of_unsigned_vec_var_23,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_unsigned, pairs_of_naturals, triples_of_naturals,
};
use num::BigUint;
use rug;
use std::str::FromStr;

fn series(start: Limb, len: usize) -> Vec<Limb> {
    (start..start + len as Limb).collect()
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out() {
    let test =
        |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, highest_result_limb, out_after| {
            let mut out = out_before.clone();
            _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
            assert_eq!(out, out_after);
            let mut out = out_before.clone();
            assert_eq!(
                limbs_mul_greater_to_out(&mut out, &xs, &ys),
                highest_result_limb
            );
            assert_eq!(out, out_after);
        };
    test(vec![2], vec![3], vec![10; 3], 0, vec![6, 0, 10]);
    test(
        vec![1; 3],
        series(1, 3),
        vec![5; 8],
        0,
        vec![1, 3, 6, 5, 3, 0, 5, 5],
    );
    test(
        series(1, 3),
        vec![6, 7],
        vec![0; 5],
        0,
        vec![6, 19, 32, 21, 0],
    );
    test(
        vec![100, 101, 102],
        vec![102, 101, 100],
        vec![10; 7],
        0,
        vec![10_200, 20_402, 30_605, 20_402, 10_200, 0, 10],
    );
    test(
        vec![0xffff_ffff],
        vec![1],
        vec![10; 3],
        0,
        vec![0xffff_ffff, 0, 10],
    );
    test(
        vec![0xffff_ffff],
        vec![0xffff_ffff],
        vec![10; 4],
        0xffff_fffe,
        vec![1, 0xffff_fffe, 10, 10],
    );
    test(
        vec![0xffff_ffff; 3],
        vec![0xffff_ffff; 3],
        vec![10; 6],
        0xffff_ffff,
        vec![1, 0, 0, 0xffff_fffe, 0xffff_ffff, 0xffff_ffff],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_fail_1() {
    let mut out = vec![10; 4];
    let xs = series(6, 3);
    _limbs_mul_greater_to_out_basecase(&mut out, &xs, &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_fail_2() {
    let mut out = vec![10; 5];
    let ys = series(1, 3);
    _limbs_mul_greater_to_out_basecase(&mut out, &[6, 7], &ys);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_fail_3() {
    let mut out = vec![10; 3];
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
        series(2, 3),
        series(3, 3),
        vec![10; 6],
        vec![6, 17, 34, 31, 20, 0],
    );
    // xs0[s] == 0 && limbs_cmp_same_length(&xs0[..s], xs1) == Ordering::Less
    // v_neg_1_neg
    //test(&[2, 0, 4], &[3, 4, 5], &[10, 10, 10, 10, 10, 10], vec![6, 8, 22, 16, 20, 0]);
    test(
        vec![1; 3],
        series(1, 3),
        vec![5; 8],
        vec![1, 3, 6, 5, 3, 0, 5, 5],
    );
    // s == n
    // limbs_cmp_same_length(ys0, ys1) != Ordering::Less
    // t == n
    // limbs_cmp_same_length(ys0, ys1) == Ordering::Less
    test(
        vec![1; 4],
        series(1, 4),
        vec![5; 8],
        vec![1, 3, 6, 10, 9, 7, 4, 0],
    );
    // limbs_cmp_same_length(&a0[..n], &a1[..n]) == Ordering::Less
    // limbs_cmp_same_length(&b0[..n], &b1[..n]) != Ordering::Less
    test(
        series(1, 4),
        vec![1; 4],
        vec![5; 8],
        vec![1, 3, 6, 10, 9, 7, 4, 0],
    );
    // limbs_test_zero(&ys0[t..]) && limbs_cmp_same_length(&ys0[..t], ys1) == Ordering::Less
    test(
        series(1, 5),
        vec![1, 0, 0, 4],
        vec![5; 9],
        vec![1, 2, 3, 8, 13, 12, 16, 20, 0],
    );
    // s > t
    test(
        vec![1; 4],
        series(1, 3),
        vec![5; 8],
        vec![1, 3, 6, 6, 5, 3, 0, 5],
    );
    test(
        vec![100, 101, 102],
        vec![102, 101, 100],
        vec![10; 7],
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
    let mut out = vec![10; 4];
    _limbs_mul_greater_to_out_toom_22(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_22_fail_2() {
    let mut scratch = vec![];
    let mut out = vec![10; 7];
    let xs = series(6, 3);
    let ys = series(1, 4);
    _limbs_mul_greater_to_out_toom_22(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_22_fail_3() {
    let mut scratch = vec![];
    let mut out = vec![10; 7];
    let xs = series(6, 4);
    _limbs_mul_greater_to_out_toom_22(&mut out, &xs, &[1, 2], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_22_fail_4() {
    let mut scratch = vec![];
    let mut out = vec![10; 7];
    let xs = series(6, 3);
    _limbs_mul_greater_to_out_toom_22(&mut out, &xs, &[1, 2], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_22_fail_5() {
    let mut scratch = vec![];
    let mut out = vec![10; 4];
    let xs = series(6, 3);
    let ys = series(1, 3);
    _limbs_mul_greater_to_out_toom_22(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_22_fail_6() {
    let mut scratch = vec![];
    let mut out = vec![10; 4];
    let xs = series(6, 3);
    _limbs_mul_greater_to_out_toom_22(&mut out, &xs, &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_22_fail_7() {
    let mut scratch = vec![];
    let mut out = vec![10; 4];
    let xs = series(6, 3);
    let ys = series(1, 4);
    _limbs_mul_greater_to_out_toom_22(&mut out, &xs, &ys, &mut scratch);
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
        series(2, 6),
        series(3, 4),
        vec![10; 10],
        vec![6, 17, 34, 58, 76, 94, 88, 71, 42, 0],
    );
    // limbs_cmp_same_length(ys0, ys1) != Ordering::Less
    // ap1_hi == 2
    // bp1_hi
    // !v_neg_1_neg
    test(
        vec![0xffff_ffff; 6],
        vec![0xffff_ffff; 4],
        vec![10; 10],
        vec![
            1, 0, 0, 0, 4294967295, 4294967295, 4294967294, 4294967295, 4294967295, 4294967295,
        ],
    );
    // ap1_hi == 0 && limbs_cmp_same_length(ap1, xs1) == Ordering::Less
    test(
        vec![0, 0, 1, 1, 0, 1],
        vec![0, 0, 0, 1],
        vec![10; 10],
        vec![0, 0, 0, 0, 0, 1, 1, 0, 1, 0],
    );
    // t != n
    // limbs_test_zero(&ys0[t..]) && limbs_cmp_same_length(&ys0[..t], ys1) == Ordering::Less
    // s + t <= n
    test(
        vec![0, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 1],
        vec![10; 12],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    );
    // !(limbs_test_zero(&ys0[t..]) && limbs_cmp_same_length(&ys0[..t], ys1) == Ordering::Less)
    test(
        vec![0, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 1, 0, 1],
        vec![10; 12],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0],
    );
    // s > t
    test(
        series(1, 9),
        vec![9, 8, 7, 6, 5],
        vec![10; 14],
        vec![9, 26, 50, 80, 115, 150, 185, 220, 255, 200, 146, 94, 45, 0],
    );
    // ap1_hi == 1
    test(
        vec![
            2543705880, 1859419010, 3343322808, 1165039137, 1872701663, 1957510151, 1589243046,
        ],
        vec![1919189400, 1295801997, 354566481, 1212146910, 1886225431],
        vec![10; 14],
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
    let mut out = vec![10; 4];
    _limbs_mul_greater_to_out_toom_32(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_32_fail_2() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_32_scratch_size(3, 4)];
    let mut out = vec![10; 7];
    let xs = series(6, 3);
    let ys = series(1, 4);
    _limbs_mul_greater_to_out_toom_32(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_32_fail_3() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_32_scratch_size(5, 4)];
    let mut out = vec![10; 9];
    let xs = series(6, 5);
    let ys = series(1, 4);
    _limbs_mul_greater_to_out_toom_32(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_32_fail_4() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_32_scratch_size(6, 3)];
    let mut out = vec![10; 9];
    let xs = series(6, 6);
    let ys = series(1, 3);
    _limbs_mul_greater_to_out_toom_32(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_32_fail_5() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_32_scratch_size(3, 0)];
    let mut out = vec![10; 4];
    let xs = series(6, 3);
    _limbs_mul_greater_to_out_toom_32(&mut out, &xs, &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_32_fail_6() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_32_scratch_size(6, 4)];
    let mut out = vec![10; 9];
    let xs = series(6, 6);
    let ys = series(1, 4);
    _limbs_mul_greater_to_out_toom_32(&mut out, &xs, &ys, &mut scratch);
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
        series(2, 5),
        series(3, 5),
        vec![10; 10],
        vec![6, 17, 34, 58, 90, 94, 88, 71, 42, 0],
    );
    // s > t
    test(
        series(2, 6),
        series(3, 5),
        vec![10; 11],
        vec![6, 17, 34, 58, 90, 115, 116, 106, 84, 49, 0],
    );
    // v_neg_1
    // two_r > k + 1
    test(
        series(2, 9),
        series(3, 8),
        vec![10; 17],
        vec![
            6, 17, 34, 58, 90, 131, 182, 244, 296, 315, 320, 310, 284, 241, 180, 100, 0,
        ],
    );
    test(
        series(3, 5),
        series(2, 5),
        vec![10; 10],
        vec![6, 17, 34, 58, 90, 94, 88, 71, 42, 0],
    );
    // !(carry == 0 && limbs_cmp_same_length(&gp[..n], xs_1) == Ordering::Less)
    // !(carry == 0 && limbs_cmp_same_length(&gp[..n], ys_1) == Ordering::Less)
    test(
        vec![0xffff_ffff; 5],
        vec![0xffff_ffff; 5],
        vec![10; 10],
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
    let mut out = vec![10; 4];
    _limbs_mul_greater_to_out_toom_33(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_33_fail_2() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_33_scratch_size(5)];
    let mut out = vec![10; 11];
    let xs = series(6, 5);
    let ys = series(1, 6);
    _limbs_mul_greater_to_out_toom_33(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_33_fail_3() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_33_scratch_size(5)];
    let mut out = vec![10; 9];
    let xs = series(6, 5);
    let ys = series(1, 4);
    _limbs_mul_greater_to_out_toom_33(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_33_fail_4() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_33_scratch_size(5)];
    let mut out = vec![10; 5];
    let xs = series(6, 5);
    _limbs_mul_greater_to_out_toom_33(&mut out, &xs, &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_33_fail_5() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_33_scratch_size(6)];
    let mut out = vec![10; 9];
    let xs = series(6, 6);
    let ys = series(1, 5);
    _limbs_mul_greater_to_out_toom_33(&mut out, &xs, &ys, &mut scratch);
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
        series(2, 4),
        vec![3, 4],
        vec![10; 7],
        vec![6, 17, 24, 31, 20, 0, 10],
    );
    // !v_neg_1_neg
    // s != n
    // t != n
    // !(limbs_test_zero(&ys_0[t..]) && limbs_cmp_same_length(&ys_0[..t], ys_1) == Ordering::Less)
    test(
        series(2, 7),
        series(3, 3),
        vec![10; 10],
        vec![6, 17, 34, 46, 58, 70, 82, 67, 40, 0],
    );
    // s > t
    test(
        series(2, 8),
        series(3, 3),
        vec![10; 11],
        vec![6, 17, 34, 46, 58, 70, 82, 94, 76, 45, 0],
    );
    // limbs_cmp_same_length(ys_0, ys_1) != Ordering::Less
    test(
        vec![0, 0, 0, 1],
        vec![1, 1],
        vec![10; 6],
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
    let mut out = vec![10; 4];
    _limbs_mul_greater_to_out_toom_42(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_42_fail_2() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_42_scratch_size(5, 6)];
    let mut out = vec![10; 11];
    let xs = series(6, 5);
    let ys = series(1, 6);
    _limbs_mul_greater_to_out_toom_42(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_42_fail_3() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_42_scratch_size(3, 2)];
    let mut out = vec![10; 9];
    let xs = series(6, 3);
    _limbs_mul_greater_to_out_toom_42(&mut out, &xs, &[1, 2], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_42_fail_4() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_42_scratch_size(5, 0)];
    let mut out = vec![10; 5];
    let xs = series(6, 5);
    _limbs_mul_greater_to_out_toom_42(&mut out, &xs, &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_42_fail_5() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_42_scratch_size(4, 2)];
    let mut out = vec![10; 4];
    let xs = series(6, 4);
    _limbs_mul_greater_to_out_toom_42(&mut out, &xs, &[1, 2], &mut scratch);
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
        series(2, 11),
        series(3, 9),
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
        series(2, 12),
        series(3, 8),
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
        series(2, 19),
        series(3, 11),
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
#[should_panic]
fn limbs_mul_greater_to_out_toom_43_fail_1() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_43_scratch_size(1, 1)];
    let mut out = vec![10; 4];
    _limbs_mul_greater_to_out_toom_43(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_43_fail_2() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_43_scratch_size(11, 12)];
    let mut out = vec![10; 23];
    let xs = series(3, 11);
    let ys = series(2, 12);
    _limbs_mul_greater_to_out_toom_43(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_43_fail_3() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_43_scratch_size(11, 8)];
    let mut out = vec![10; 19];
    let xs = series(3, 11);
    let ys = series(2, 8);
    _limbs_mul_greater_to_out_toom_43(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_43_fail_4() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_43_scratch_size(12, 0)];
    let mut out = vec![10; 12];
    let xs = series(3, 11);
    _limbs_mul_greater_to_out_toom_43(&mut out, &xs, &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_43_fail_5() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_43_scratch_size(4, 2)];
    let mut out = vec![10; 5];
    let xs = series(3, 10);
    let ys = series(2, 10);
    _limbs_mul_greater_to_out_toom_43(&mut out, &xs, &ys, &mut scratch);
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
        series(2, 4),
        series(3, 4),
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

#[cfg(feature = "64_bit_limbs")]
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
    test(
        series(2, 4),
        series(3, 4),
        vec![10; 8],
        vec![6, 17, 34, 58, 58, 49, 30, 0],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_44_fail_1() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_44_scratch_size(1)];
    let mut out = vec![10; 10];
    _limbs_mul_greater_to_out_toom_44(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_44_fail_2() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_44_scratch_size(4)];
    let mut out = vec![10; 9];
    let xs = series(3, 4);
    let ys = series(2, 5);
    _limbs_mul_greater_to_out_toom_44(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_44_fail_3() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_44_scratch_size(3)];
    let mut out = vec![10; 6];
    let xs = series(3, 3);
    let ys = series(2, 3);
    _limbs_mul_greater_to_out_toom_44(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_44_fail_4() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_44_scratch_size(11)];
    let mut out = vec![10; 11];
    let xs = series(3, 11);
    _limbs_mul_greater_to_out_toom_44(&mut out, &xs, &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_44_fail_5() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_44_scratch_size(4)];
    let mut out = vec![10; 7];
    let xs = series(3, 4);
    let ys = series(2, 4);
    _limbs_mul_greater_to_out_toom_44(&mut out, &xs, &ys, &mut scratch);
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
    // degree.even() in _limbs_mul_toom_evaluate_poly_in_1_and_neg_1
    // !v_neg_1_neg in _limbs_mul_toom_evaluate_poly_in_1_and_neg_1
    test(
        series(2, 15),
        series(3, 5),
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
    // v_neg_1_neg in _limbs_mul_toom_evaluate_poly_in_1_and_neg_1
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
    let mut out = vec![10; 4];
    _limbs_mul_greater_to_out_toom_52(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_52_fail_2() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_52_scratch_size(15, 16)];
    let mut out = vec![10; 9];
    let xs = series(3, 15);
    let ys = series(3, 16);
    _limbs_mul_greater_to_out_toom_52(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_52_fail_3() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_52_scratch_size(14, 5)];
    let mut out = vec![10; 6];
    let xs = series(3, 14);
    let ys = series(3, 5);
    _limbs_mul_greater_to_out_toom_52(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_52_fail_4() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_52_scratch_size(15, 4)];
    let mut out = vec![10; 7];
    let xs = series(3, 15);
    let ys = series(3, 4);
    _limbs_mul_greater_to_out_toom_52(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_52_fail_5() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_52_scratch_size(11, 0)];
    let mut out = vec![10; 12];
    let xs = series(3, 11);
    _limbs_mul_greater_to_out_toom_52(&mut out, &xs, &[], &mut scratch);
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
        series(2, 5),
        series(3, 3),
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
    let mut out = vec![10; 4];
    _limbs_mul_greater_to_out_toom_53(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_53_fail_2() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_53_scratch_size(5, 6)];
    let mut out = vec![10; 11];
    let xs = series(3, 5);
    let ys = series(3, 6);
    _limbs_mul_greater_to_out_toom_53(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_53_fail_3() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_53_scratch_size(5, 4)];
    let mut out = vec![10; 9];
    let xs = series(3, 5);
    let ys = series(3, 4);
    _limbs_mul_greater_to_out_toom_53(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_53_fail_4() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_53_scratch_size(4, 3)];
    let mut out = vec![10; 6];
    let xs = series(3, 4);
    let ys = series(3, 3);
    _limbs_mul_greater_to_out_toom_53(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_53_fail_5() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_53_scratch_size(5, 2)];
    let mut out = vec![10; 7];
    let xs = series(3, 5);
    _limbs_mul_greater_to_out_toom_53(&mut out, &xs, &[3, 4], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_53_fail_6() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_53_scratch_size(5, 0)];
    let mut out = vec![10; 12];
    let xs = series(3, 5);
    _limbs_mul_greater_to_out_toom_53(&mut out, &xs, &[], &mut scratch);
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
        series(2, 15),
        series(3, 11),
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
    let mut out = vec![10; 4];
    _limbs_mul_greater_to_out_toom_54(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_54_fail_2() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_54_scratch_size(15, 16)];
    let mut out = vec![10; 31];
    let xs = series(3, 14);
    let ys = series(3, 17);
    _limbs_mul_greater_to_out_toom_54(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_54_fail_3() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_54_scratch_size(15, 10)];
    let mut out = vec![10; 25];
    let xs = series(3, 14);
    let ys = series(3, 10);
    _limbs_mul_greater_to_out_toom_54(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_54_fail_4() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_54_scratch_size(14, 11)];
    let mut out = vec![10; 25];
    let xs = series(3, 14);
    let ys = series(3, 11);
    _limbs_mul_greater_to_out_toom_54(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_54_fail_5() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_54_scratch_size(15, 11)];
    let mut out = vec![10; 25];
    let xs = series(3, 15);
    let ys = series(3, 11);
    _limbs_mul_greater_to_out_toom_54(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_54_fail_6() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_54_scratch_size(15, 0)];
    let mut out = vec![10; 15];
    let xs = series(3, 15);
    _limbs_mul_greater_to_out_toom_54(&mut out, &xs, &[], &mut scratch);
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
    // degree.odd() in _limbs_mul_toom_evaluate_poly_in_1_and_neg_1
    // degree > 4 in _limbs_mul_toom_evaluate_poly_in_1_and_neg_1
    // degree_u >= 5 in _limbs_mul_toom_evaluate_poly_in_2_and_neg_2
    // degree.odd() in _limbs_mul_toom_evaluate_poly_in_2_and_neg_2
    // t == n
    // limbs_cmp_same_length(ys_0, ys_1) == Ordering::Less
    // v_neg_1_neg_b
    // *as1_last == 0
    test(
        series(2, 6),
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
    let mut out = vec![10; 4];
    _limbs_mul_greater_to_out_toom_62(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_62_fail_2() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_62_scratch_size(6, 7)];
    let mut out = vec![10; 13];
    let xs = series(3, 6);
    let ys = series(3, 7);
    _limbs_mul_greater_to_out_toom_62(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_62_fail_3() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_62_scratch_size(6, 1)];
    let mut out = vec![10; 7];
    let xs = series(3, 6);
    _limbs_mul_greater_to_out_toom_62(&mut out, &xs, &[3], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_62_fail_4() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_62_scratch_size(5, 2)];
    let mut out = vec![10; 7];
    let xs = series(3, 5);
    _limbs_mul_greater_to_out_toom_62(&mut out, &xs, &[3, 4], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_62_fail_5() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_62_scratch_size(6, 2)];
    let mut out = vec![10; 7];
    let xs = series(3, 6);
    _limbs_mul_greater_to_out_toom_62(&mut out, &xs, &[3, 4], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_62_fail_6() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_62_scratch_size(6, 0)];
    let mut out = vec![10; 6];
    let xs = series(3, 6);
    _limbs_mul_greater_to_out_toom_62(&mut out, &xs, &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_63() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch =
            vec![0; _limbs_mul_greater_to_out_toom_63_scratch_size(xs.len(), ys.len())];
        _limbs_mul_greater_to_out_toom_63(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    // n == t
    // !(!carry && limbs_cmp_same_length(scratch2_lo, ys_1) == Ordering::Less)
    // s <= t
    test(
        series(2, 17),
        series(3, 9),
        vec![10; 26],
        vec![
            6, 17, 34, 58, 90, 131, 182, 244, 318, 381, 444, 507, 570, 633, 696, 759, 822, 828,
            812, 773, 710, 622, 508, 367, 198, 0,
        ],
    );
    // n != t
    test(
        vec![
            3047748962, 2284186344, 3132866461, 2331447040, 1003213663, 1873981685, 3371337621,
            3796896013, 4144448610, 2569252563, 2859304641, 1027973602, 3158196152, 4058699545,
            2002924383, 3295505824, 695758308, 544681384, 3452307839, 1190734708, 4232023153,
            451772934, 673919865, 2022672425, 3493426012, 1142609332, 477542383, 1304798841,
            461115870, 3268103575, 2243523508,
        ],
        vec![
            3987208830, 1336657961, 2605546090, 1112778759, 2243645577, 3695113963, 637209276,
            527642657, 1586863943, 2178788843, 4128924923, 574016400, 118333022, 3019059425,
            3734056582, 3974475640, 958936732,
        ],
        vec![10; 48],
        vec![
            901282364, 4131825926, 550521101, 4239081984, 354957348, 2987335611, 2947836402,
            1594339509, 1900787939, 3942224706, 1915750189, 2686147736, 455238733, 595779993,
            992449470, 225135268, 4216025815, 112446550, 2736130746, 1015352940, 1166343395,
            3559470539, 2787138552, 3128535813, 2203140859, 3479459112, 599923700, 684443693,
            1557326194, 1699057519, 2198150417, 2196463130, 1973109458, 3642841764, 426750624,
            1438683694, 42406461, 1444651840, 2152704621, 722727455, 3882030279, 205951250,
            838845869, 2997862064, 779154540, 1753953589, 1791445120, 500911172,
        ],
    );
    test(
        vec![
            2547108010, 2828666778, 3252702258, 3885923576, 2331974758, 730724707, 1528859315,
            4288784328, 3677151116, 445199233, 3304488688, 3566979465, 3541025426, 2491779846,
            3112990742, 2583249486, 3403111749, 1930721237, 3428792463, 2896462048, 2985885576,
            1819460734, 21206096, 3560441846, 987100555, 2844904275, 84854892, 1268249628,
            3963306788, 3338670067, 2504599089, 65588657, 321493327, 4249673617, 4150876068,
            721566898,
        ],
        vec![
            2339094549, 568841948, 757218994, 54206328, 2888117240, 1758638903, 3215886938,
            2041086168, 259363425, 3740850804, 3272104239, 3101597497, 4170226346, 1487680512,
            2997309052, 1761169487, 680164259, 104354801, 3642294827, 2001649447,
        ],
        vec![10; 56],
        vec![
            4156749298, 1238334534, 3541686335, 400023669, 3354392679, 146448234, 338562445,
            2541647274, 3476105410, 3869729511, 2592129633, 1524174755, 2864342013, 3189404137,
            2408966423, 1748955694, 848863232, 2061232865, 2863992687, 1780371599, 1814973544,
            4129152748, 1067034680, 3960771432, 1978132071, 249147649, 4113633238, 3331366833,
            103867284, 4274561406, 24372440, 1874890180, 2262704206, 4185039557, 1493676561,
            3605651563, 184712156, 1714079946, 3695806969, 3114374817, 2698021971, 2617815992,
            3374318018, 2710182754, 2217042831, 3166354273, 3526471084, 2282901181, 17853137,
            2805842653, 2980411632, 2879849003, 22987084, 2408312078, 212023482, 336282883,
        ],
    );
    // !carry && limbs_cmp_same_length(scratch2_lo, ys_1) == Ordering::Less
    // s > t
    test(
        vec![
            275320572, 2678313698, 1997150503, 1718206458, 3389415001, 1347098060, 423205500,
            1228674579, 1683636524, 1761485682, 3886555164, 1343770739, 3728441996, 3386212640,
            4218849286, 3154177905, 383775865, 685210915, 2915358388, 356527607, 1399377005,
            2203631586, 3950305635, 4107289625,
        ],
        vec![
            343872945, 2028904125, 1525417887, 867188532, 3911999830, 2139706847, 3256484706,
            961423019, 1530068826, 3577946967,
        ],
        vec![10; 34],
        vec![
            367134780, 454511356, 740068730, 2466817027, 444007987, 2116910983, 3588258390,
            4148666142, 241899205, 3037479671, 967522541, 1695514557, 3417684811, 1755587152,
            57889847, 1893598444, 894827452, 1259092281, 343759711, 417669929, 4250137916,
            2931151486, 4137704826, 1616987343, 118402896, 3476900958, 3144858924, 799089809,
            2899882887, 413231425, 2515242049, 142267098, 1727945779, 3421601015,
        ],
    );
}

#[cfg(feature = "64_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_63() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch =
            vec![0; _limbs_mul_greater_to_out_toom_63_scratch_size(xs.len(), ys.len())];
        _limbs_mul_greater_to_out_toom_63(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    test(
        vec![
            6746486103788831552,
            2922469023463657485,
            7190781201699911122,
            6369274278675525514,
            11602031538822447399,
            18146097755068799938,
            10715195159596301824,
            1582667531232164822,
            17310503547119278200,
            11108448614311336701,
            16131384432757080248,
            10724146198461527790,
            17486718158725257827,
            6011711000953739951,
            12044019786490872751,
            12126819472937875768,
            11736689834584491812,
            2624631955548590096,
        ],
        vec![
            8718882040837103283,
            12513261442998616191,
            3363599670593686195,
            2576001491054566526,
            8476413363242630098,
            11800520882738943180,
            15256756628116724015,
            15102633230716809194,
            4752404995807312312,
        ],
        vec![10; 27],
        vec![
            11055708298853713344,
            11718134630995530406,
            1540454672309197922,
            2461234873920328802,
            12156343925049526190,
            7669775936281025739,
            5569544286309952271,
            1251802631971472159,
            7852335389754101252,
            16331287242162052217,
            16922468211499817236,
            1090055930057904858,
            4774304109866833132,
            2115064630415334045,
            3041714142401192073,
            5249251501654981623,
            6324653539847586925,
            7895228639492924348,
            13455067205957702368,
            1142009976612635724,
            13095096323291438869,
            4348574203955863428,
            8491467291307697179,
            3535832683825156722,
            3832291870552829557,
            16965222076837711040,
            676179707804463061,
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_63_fail_1() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_63_scratch_size(1, 1)];
    let mut out = vec![10; 4];
    _limbs_mul_greater_to_out_toom_63(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_63_fail_2() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_63_scratch_size(17, 18)];
    let mut out = vec![10; 13];
    let xs = series(3, 17);
    let ys = series(3, 18);
    _limbs_mul_greater_to_out_toom_63(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_63_fail_3() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_63_scratch_size(17, 8)];
    let mut out = vec![10; 25];
    let xs = series(3, 17);
    let ys = series(3, 8);
    _limbs_mul_greater_to_out_toom_63(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_63_fail_4() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_63_scratch_size(16, 9)];
    let mut out = vec![10; 25];
    let xs = series(3, 17);
    let ys = series(3, 9);
    _limbs_mul_greater_to_out_toom_63(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_63_fail_5() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_63_scratch_size(17, 9)];
    let mut out = vec![10; 25];
    let xs = series(3, 17);
    let ys = series(3, 9);
    _limbs_mul_greater_to_out_toom_63(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_63_fail_6() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_63_scratch_size(17, 0)];
    let mut out = vec![10; 6];
    let xs = series(3, 17);
    _limbs_mul_greater_to_out_toom_63(&mut out, &xs, &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_6h() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>| {
        let mut out = out_before.to_vec();
        _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        let out_after = out;
        let mut out = out_before.to_vec();
        let mut scratch =
            vec![0; _limbs_mul_greater_to_out_toom_6h_scratch_size(xs.len(), ys.len())];
        _limbs_mul_greater_to_out_toom_6h(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    // xs_len * LIMIT_DENOMINATOR < LIMIT_NUMERATOR * ys_len
    // degree.odd() in _limbs_mul_toom_evaluate_poly_in_2_pow_neg_and_neg_2_pow_neg
    // degree > 3 in _limbs_mul_toom_evaluate_poly_in_2_pow_neg_and_neg_2_pow_neg
    // !neg in _limbs_mul_toom_evaluate_poly_in_2_pow_neg_and_neg_2_pow_neg
    // q != 3
    // !half in _limbs_mul_toom_interpolate_12_points
    test(series(2, 42), series(3, 42), vec![10; 84]);
    test(vec![0; 43], vec![0; 42], vec![10; 85]);
    let xs = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ];
    let ys = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ];
    let out_len = xs.len() + ys.len();
    // v_2_pow_neg_neg in _limbs_mul_toom_evaluate_poly_in_2_pow_neg_and_neg_2_pow_neg
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        2176876728, 2496909862, 111654638, 4071443844, 1244732003, 1399710541, 3492272815,
        2804216879, 294683567, 2823495183, 1539340600, 2732661048, 2371405604, 611094747,
        2426195984, 3948451542, 3575143460, 2163084716, 2877537071, 1849282685, 1662381818,
        2022577840, 552741512, 1863034519, 2109621858, 3426780715, 233006082, 2766239663,
        1257764921, 1179443268, 3311729910, 4228711990, 3676801557, 83336617, 52963853, 1461131367,
        615175494, 2376138249, 1373985035, 3055102427, 1823691121, 175073115, 3051957217,
    ];
    let ys = vec![
        344785207, 1075768263, 3315797254, 2656376324, 160336834, 3872758991, 671370872,
        1253701757, 217686653, 4064957864, 1185854346, 2308111201, 847669579, 195002426,
        1955159211, 2003106801, 1041767923, 3605273739, 3153084777, 2806535311, 1401436525,
        1148858479, 958627821, 1267879008, 4138398998, 1028065582, 3914213477, 3370118288,
        4054975453, 1815994585, 2486521917, 995353494, 16609723, 4010498224, 1214270934, 797624362,
        4000265982, 1287753121, 874311717, 2200865401, 21122981, 1507911002,
    ];
    let out_len = xs.len() + ys.len();
    // r4_last.leading_zeros() < 3 in _limbs_mul_toom_interpolate_12_points
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        2327202328, 3179332026, 2188958336, 2717879675, 130062885, 140536268, 2499125438,
        3163111280, 4259661702, 2176278885, 422519228, 2482586299, 2904549185, 656169575,
        2052350629, 1346745024, 2132509288, 3672720658, 1036389958, 1864007789, 4247227128,
        3920036168, 1436562554, 4261984498, 3509215437, 583752676, 3145348403, 2267709494,
        2846186667, 95392897, 3743233716, 2210401890, 333864866, 4114644153, 3030283850,
        2885600773, 209380485, 753945396, 719327396, 1293498320, 881901364, 2799735404, 3880748109,
        2227099476, 2045911493, 279042015, 1825819541, 1783146691, 2256898093, 2186071881,
    ];
    let ys = vec![
        4062960470, 3852836537, 2696572187, 2332897564, 3819654112, 1805852435, 2339319161,
        3891614436, 3143079880, 3244604349, 2122448594, 1926396564, 3938383812, 51745369,
        2731805677, 4257919711, 2550692774, 4079294279, 223709465, 1648526554, 689775843,
        3524108772, 1404538310, 806199241, 4278266886, 2467028886, 3773289773, 3246095241,
        2201055218, 2036154035, 3144210007, 423367788, 3883829868, 2190252193, 2069131777,
        3027047320, 1576225469, 3459606326, 2343356582, 2658410138, 1927376994, 3129832669,
        3772482523,
    ];
    let out_len = xs.len() + ys.len();
    // xs_len * LIMIT_DENOMINATOR >= LIMIT_NUMERATOR * ys_len
    // xs_len * 5 * LIMIT_NUMERATOR < LIMIT_DENOMINATOR * 7 * ys_len
    // half
    // degree.even() in _limbs_mul_toom_evaluate_poly_in_2_pow_neg_and_neg_2_pow_neg
    // degree > 5 in _limbs_mul_toom_evaluate_poly_in_1_and_neg_1
    // s <= t
    // half in _limbs_mul_toom_interpolate_12_points
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        1940830933, 3780770129, 1587254032, 832573251, 1504418072, 4247592896, 317874907,
        949850421, 2252881736, 3574316069, 3062236166, 1396410954, 3249498785, 3495392204,
        540855070, 1908700137, 1469179505, 4199276220, 953657385, 3056452157, 2141569526,
        2342475731, 3746376146, 3271677606, 2770490239, 2212992129, 1758619376, 1446549455,
        409094501, 767129031, 3284625381, 1887741449, 1134874072, 2988924415, 1641550007,
        856704035, 80648349, 1467185629, 2753807208, 1609415681, 4087676277, 3276525355,
        1530490532, 3475014952, 1971819359, 2190766950, 2667577576, 2404497182, 4128259693,
        2449514447, 4199089872, 2205116036, 4089987616, 457231895, 2931469481, 3147651033,
        2352907189,
    ];
    let ys = vec![
        3461606200, 1584050797, 14355481, 3385840230, 1703326352, 1625259628, 3642322228,
        911402341, 2158835226, 939248485, 3607511108, 2863853568, 1611642161, 1312857772,
        1839433327, 567060478, 3139863681, 3642698184, 3744632443, 712538472, 2692932947,
        576185818, 156934113, 518107105, 2803035863, 2284220097, 3447382922, 2400125006,
        3565062840, 160044186, 3644393084, 4196433258, 3391883838, 1115703759, 2380388002,
        962895870, 4001772616, 2311278419, 2620271020, 3047789793, 3229254302, 3182628087,
        2718480927, 2872538422,
    ];
    let out_len = xs.len() + ys.len();
    // t < 1
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        3796896013, 4144448610, 2569252563, 2859304641, 1027973602, 3158196152, 4058699545,
        2002924383, 3295505824, 695758308, 544681384, 3452307839, 1190734708, 4232023153,
        451772934, 673919865, 2022672425, 3493426012, 1142609332, 477542383, 1304798841, 461115870,
        3268103575, 2243523508, 606810814, 4235312469, 1885993181, 114475077, 757688489,
        1965769398, 260629125, 2265559181, 2568323569, 4202738507, 422918034, 1258453131,
        3552221985, 1666914845, 4063631552, 1893061685, 1362616670, 3828572660, 3003680479,
        119501228, 2101943449, 1119123129, 2512417484,
    ];
    let ys = vec![
        610160726, 3751120540, 2655318738, 2490069121, 732352936, 1985503906, 765573690,
        2709177647, 3058016350, 1432725430, 2213840145, 1911049343, 3116245242, 519557432,
        1828983405, 3092431113, 3844759473, 547304293, 1609305183, 1824076406, 2409386071,
        2970173039, 4255413180, 894750419, 90356879, 2880999631, 2157180976, 2261258057, 715581698,
        332174009, 27958638, 2464799420, 3232925197, 1952944696, 915312443, 1464711675, 4079172443,
        2445511584, 2092009263, 3412361485, 2354390078, 3106038172, 3481973486,
    ];
    let out_len = xs.len() + ys.len();
    // s < 1
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        2187046946, 3600373521, 4275090943, 2120016813, 4177241875, 3185774231, 2397692077,
        1015362399, 2178889151, 3433916223, 1688082118, 1971242178, 236388706, 3802829765,
        521309115, 2299816689, 3207614143, 1053195464, 3584561145, 1178690670, 2940812254,
        3321982035, 2754825123, 3073598062, 202404806, 547895545, 1188944547, 1056841779,
        529463805, 204665384, 850370055, 2063320161, 3724100092, 1180272690, 1398467003,
        2814052449, 1311768018, 659771105, 3226477227, 4230080238, 2134344405, 1461172705,
        2728018383, 1816821358, 3231137250, 2012377728, 2206916761, 3121807673,
    ];
    let ys = vec![
        1717557648, 1819215517, 3449795284, 844168976, 1574237607, 758725457, 762624299, 533122182,
        1201164787, 1968174784, 896982568, 3419630169, 2247559545, 3983311870, 3975342941,
        1112833399, 2721518545, 2493587613, 3444837338, 3313000598, 751186769, 2970698395,
        915811688, 1206259449, 1340427760, 3844346545, 3762393860, 543253569, 1197933603,
        3734607133, 4037352821, 2263945478, 2831362781, 3363558852, 476952769, 1916745391,
        208671986, 2395250976, 1549715018, 2746690542, 1219103496, 256305249,
    ];
    let out_len = xs.len() + ys.len();
    // s_plus_t > n in _limbs_mul_toom_interpolate_12_points
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        1976230069, 2821313230, 4002048052, 2248747478, 1208640865, 1469538686, 2438066233,
        1106183979, 1877645648, 2583513281, 904899723, 1001323826, 3134049747, 292171929,
        1479818350, 821125410, 2017124898, 3447449059, 2073983663, 1214861045, 3270809855,
        2826108666, 412311360, 3687943078, 157663911, 447468817, 1727023746, 1120132848, 462566659,
        21711861, 2204912119, 631663514, 2655508903, 2912870262, 1326931248, 1409724492,
        3912444286, 1986726296, 190162730, 675575771, 234714100, 3787240294, 3149710501,
        1950469069, 1222949463, 218525862, 929916299, 1757577031, 3896857869, 443052809,
        4256330379, 1106528307, 2502814887, 108409846,
    ];
    let ys = vec![
        3774873792, 2622161570, 566787739, 1447674683, 1128900692, 2570098345, 3920242059,
        2431899603, 1456341665, 269610676, 673205188, 3712878022, 3795578329, 996518376,
        3414916195, 4167667588, 4013410429, 724257700, 698186720, 1170923258, 3652768880,
        1373260172, 3271469225, 971070649, 1556038273, 2204702414, 673789949, 3790414001,
        1550521405, 2173912108, 70968354, 1856452807, 2648613270, 2751500372, 1057118618,
        3117394831, 4409774, 2422780755, 3367234488, 1080583495, 29356841, 3627216363,
    ];
    let out_len = xs.len() + ys.len();
    // s > t
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        2764481948, 3824853452, 3714446166, 1652416239, 2448871004, 3349954116, 2715554665,
        2953094534, 2191528165, 1105735060, 407641991, 1058849514, 2583237649, 3635224830,
        1509496009, 2360185935, 2419261549, 2433663350, 262632960, 3504095388, 2570319009,
        2415092334, 72373859, 3953007752, 3259518037, 3401184350, 574975346, 1921349734,
        1293058836, 2824387015, 670301824, 3449438821, 3149566748, 2370941125, 3445476733,
        1172535390, 684380840, 4007537582, 3019960994, 3833788436, 2407231528, 532343833,
        438092212, 830534904, 325324494, 1629611634, 3991887007, 1617691624, 3806774950,
        2737609900, 4123817599, 1139254855, 4270594452, 3772632696, 357643096, 978439292,
        3535266500, 1036728326, 408519941, 386395864, 986295007,
    ];
    let ys = vec![
        2893157767, 2933782072, 1630695663, 765017133, 148924741, 3933388144, 2827967305,
        1580462312, 4233997190, 2184167709, 1124313531, 1269787970, 2637050113, 1899399034,
        458443927, 676372848, 3341236235, 2358837775, 78253712, 1308766267, 1398616295, 442007911,
        3803960772, 2890078708, 2362278228, 452577827, 2295445770, 1281833658, 3733263779,
        3192119570, 1465309963, 4149236735, 2550067398, 3391554453, 3763654782, 280954439,
        4216404337, 2988297132, 1171366979, 752568358, 3832355781, 3002295862,
    ];
    let out_len = xs.len() + ys.len();
    // xs_len * 5 * LIMIT_DENOMINATOR < LIMIT_NUMERATOR * 7 * ys_len
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        1537074800, 903591185, 3505885895, 1600301704, 2247503777, 2456507858, 354178772,
        4264234279, 4276311343, 2137271746, 3095634214, 3503644667, 3271712752, 1235289576,
        3972513632, 4268165027, 3304957815, 2349877036, 1814187379, 1622480961, 1887152020,
        617829740, 2759792107, 2650325546, 3834300382, 1711067002, 16368281, 3248020475,
        1355293366, 2500355734, 3216660200, 2844209744, 919471841, 2536405197, 286948869,
        3207728956, 1786641001, 3909697676, 2990524533, 3373134471, 2770917041, 2941741335,
        2275165617, 610985518, 1663622513, 780492488, 696913656, 1787332447, 1693914179,
        2059746330, 4084862137, 1720114882, 2072770321, 2800094080, 164377327, 114079185,
        1630830573, 866212705, 86571916, 2701570437, 1022361296, 2774191689, 1485998454,
        1449541799,
    ];
    let ys = vec![
        10887125, 840662268, 2350057862, 3489480809, 2643647461, 2120151555, 433525765, 1719122308,
        3784715068, 3156307967, 4113669583, 607844816, 2149779595, 55766995, 3922134877,
        1464452041, 2877070520, 3517698059, 3219767758, 138329276, 1434547315, 1010269423,
        3836852303, 521525549, 1124005096, 128173038, 1627976147, 4217098680, 963901397,
        4003948876, 4078383999, 3163439869, 1376461045, 1260808800, 1583549957, 3016546386,
        601137572, 2476346948, 1057124592, 2232232546, 2939285402, 2703166574, 2566511508,
    ];
    let out_len = xs.len() + ys.len();
    // xs_len * LIMIT_NUMERATOR < LIMIT_DENOMINATOR * 2 * ys_len
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        2480817744, 2385986715, 908796583, 3725142486, 4259996640, 2324291843, 2514777689,
        776517112, 1179390166, 2884250121, 2107025487, 1847592315, 1214792717, 581761941,
        2035752941, 3257884740, 1011095107, 388625485, 621566511, 1878249130, 2298430809,
        3893830507, 2516166455, 1685998768, 3349147146, 4262358486, 164529678, 1000098113,
        1468664761, 1088142633, 2140348214, 672483433, 4236152545, 460911546, 1948312076,
        1030937440, 3633681142, 1170002101, 2159285228, 1104198886, 1581288546, 2266152509,
        1437951300, 3854459332, 88193405, 3804599756, 577997778, 3610194716, 2527782134,
        4194448103, 3390832927, 863423772, 2308481008, 1764994151, 2876150765, 474256942,
        3850214133, 2831691105, 4251752821, 80285354, 3225163007, 84390462, 1489215151, 1516077116,
        299402893, 1093360002, 706962212, 375054336, 678692965, 2794629958, 3684518009, 1067098399,
        3918266067, 770155119, 1400555696, 4260143847, 3420662760, 2234352998, 2627202272,
        2396298990, 2703934662, 2975030448, 1678542783, 3962857080, 2037990778, 2350341680,
        3690768614, 3327392397, 2374080995, 1568940040,
    ];
    let ys = vec![
        2432887163, 3617411153, 4065664491, 954897002, 1958352130, 2690853400, 3170435422,
        333223996, 1886503369, 2874118364, 2360990628, 3409169651, 14803166, 2428352279,
        2882529293, 215157778, 3595826381, 1351666697, 3213081864, 1796627015, 138520647,
        1446708749, 549025603, 1154696063, 951257454, 1061151557, 3578338019, 553024835,
        1032056788, 3332695385, 1916952270, 1402847201, 418140204, 1113800470, 3311963507,
        3579825680, 283695808, 1030062334, 2885288472, 2307021635, 1215165167, 361703549,
        3359666682, 2960119991, 3759575408,
    ];
    let out_len = xs.len() + ys.len();
    // xs_len * LIMIT_DENOMINATOR < LIMIT_NUMERATOR * 2 * ys_len
    // q == 3
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        2182584668, 370736031, 944597706, 368333101, 3076089385, 4269551750, 455799119, 1640998687,
        1332255273, 3039440200, 1094187469, 4158542740, 4241437189, 786279542, 3313987323,
        801901648, 2460914857, 2458651362, 1161118074, 3733983107, 1911753349, 4261306583,
        981590361, 1357088215, 210591422, 1159943023, 510963968, 2705428227, 3460159465,
        1967595187, 703584117, 3474024702, 3343010520, 1232104952, 823854220, 4012290690,
        3252492197, 3975386640, 1309751464, 232265040, 2026518879, 794539121, 1849747498,
        773993567, 2415934846, 842827728, 25297943, 3952540535, 2909076393, 4183158950, 2579267900,
        898983053, 2480815324, 1004385686, 3272214418, 2360496610, 3884948711, 3937994494,
        1355835525, 1862072763, 4077270583, 456721854, 1202741767, 1334238573, 3202598432,
        2518498766, 1873498914, 1155219866, 3257357513, 3381800028, 777225471, 1628571355,
        281982096, 1238331533, 728101793, 407378640, 1088081860, 2405377044, 2080950804,
        3105324348, 3065313268, 2776290680, 1200951260, 1789619269, 1088225065, 317598486,
        924903972, 3504476787, 1605816151, 388266283, 1613602905, 4051481387, 2773856406,
        3434866445, 2039264971, 1587433780, 1787644933, 2852323335,
    ];
    let ys = vec![
        3040086267, 3720432305, 3025753876, 3307555779, 2232302878, 1705545587, 3746861739,
        3551552480, 3791909589, 3559707401, 3597994914, 1201195479, 2759785652, 2538497144,
        2628719068, 1220743906, 2592330951, 357425155, 2683446134, 369894528, 2918070813,
        3201581079, 352827384, 2667389301, 406071886, 1478662115, 3424718337, 3498162517,
        1851891341, 2009161130, 4175528772, 2739823403, 2691610015, 530787751, 2995441702,
        238468207, 84087963, 2802633771, 2722772179, 1905704311, 791349630, 4036308669, 1333503772,
    ];
    let out_len = xs.len() + ys.len();
    // p == 9, q == 4
    test(xs, ys, vec![10; out_len]);
}

#[cfg(feature = "64_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_6h() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>| {
        let mut out = out_before.to_vec();
        _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        let out_after = out;
        let mut out = out_before.to_vec();
        let mut scratch =
            vec![0; _limbs_mul_greater_to_out_toom_6h_scratch_size(xs.len(), ys.len())];
        _limbs_mul_greater_to_out_toom_6h(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    test(series(2, 42), series(3, 42), vec![10; 84]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_6h_fail_1() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_6h_scratch_size(1, 1)];
    let mut out = vec![10; 4];
    _limbs_mul_greater_to_out_toom_6h(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_6h_fail_2() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_6h_scratch_size(41, 42)];
    let mut out = vec![10; 83];
    let xs = series(3, 41);
    let ys = series(3, 42);
    _limbs_mul_greater_to_out_toom_6h(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_6h_fail_3() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_6h_scratch_size(42, 41)];
    let mut out = vec![10; 83];
    let xs = series(3, 42);
    let ys = series(3, 41);
    _limbs_mul_greater_to_out_toom_6h(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_6h_fail_4() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_6h_scratch_size(41, 41)];
    let mut out = vec![10; 82];
    let xs = series(3, 41);
    let ys = series(3, 41);
    _limbs_mul_greater_to_out_toom_6h(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_6h_fail_5() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_6h_scratch_size(42, 42)];
    let mut out = vec![10; 83];
    let xs = series(3, 42);
    let ys = series(3, 42);
    _limbs_mul_greater_to_out_toom_6h(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_6h_fail_6() {
    let mut scratch = vec![0; _limbs_mul_greater_to_out_toom_6h_scratch_size(42, 0)];
    let mut out = vec![10; 42];
    let xs = series(3, 42);
    _limbs_mul_greater_to_out_toom_6h(&mut out, &xs, &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_8h() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>| {
        let mut out = out_before.to_vec();
        _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        let out_after = out;
        let mut out = out_before.to_vec();
        let mut scratch =
            vec![0; _limbs_mul_greater_to_out_toom_8h_scratch_size(xs.len(), ys.len())];
        _limbs_mul_greater_to_out_toom_8h(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    // an == bn || an * (TOOM_8H_LIMIT_DENOMINATOR >> 1) < TOOM_8H_LIMIT_NUMERATOR * (bn >> 1)
    // !(Limb::WIDTH > 12 * 3 && q == 3)
    // (r6[n3] & (Limb::MAX << (Limb::WIDTH - 3))) != 0 in _limbs_mul_toom_interpolate_16_points
    // !half in _limbs_mul_toom_interpolate_16_points
    test(series(2, 86), series(3, 86), vec![10; 172]);
    let xs = vec![
        3581553119, 2147449432, 208926434, 2037430803, 4143975728, 2356343321, 937192435,
        1637432038, 661638621, 1801480924, 3779152128, 4243491821, 1667774376, 1715755489,
        3661813139, 1605971891, 4030695606, 2961165054, 1368430397, 2222904896, 2817587025,
        1714442303, 3822714979, 300305701, 1874484285, 2601340412, 2275789197, 2695461089,
        2246464394, 1119579754, 1646098622, 3280004748, 33497272, 1940830933, 3780770129,
        1587254032, 832573251, 1504418072, 4247592896, 317874907, 949850421, 2252881736,
        3574316069, 3062236166, 1396410954, 3249498785, 3495392204, 540855070, 1908700137,
        1469179505, 4199276220, 953657385, 3056452157, 2141569526, 2342475731, 3746376146,
        3271677606, 2770490239, 2212992129, 1758619376, 1446549455, 409094501, 767129031,
        3284625381, 1887741449, 1134874072, 2988924415, 1641550007, 856704035, 80648349,
        1467185629, 2753807208, 1609415681, 4087676277, 3276525355, 1530490532, 3475014952,
        1971819359, 2190766950, 2667577576, 2404497182, 4128259693, 2449514447, 4199089872,
        2205116036, 4089987616, 457231895,
    ];
    let ys = vec![
        1495737173, 3863569894, 2781409865, 2031883388, 2335263853, 2715800358, 580338429,
        3465089273, 419683969, 372309798, 2092398197, 1587236508, 1706866472, 1926863329,
        2427550983, 3014840641, 2591183237, 311998012, 1838159904, 2382380991, 3168560843,
        2457672651, 1329938456, 1585986499, 32624746, 1886190156, 1819802220, 4189456784,
        2354442118, 1007664036, 3528608675, 3607011918, 3175583218, 2103466232, 4139172560,
        1977990249, 408055457, 1917901811, 4285926188, 2576630504, 3833124229, 664620480,
        3594197860, 38119241, 2843152292, 1589895470, 132829200, 911163756, 3350029197, 141124331,
        628197809, 3184483823, 2738720089, 3684675439, 2998575143, 2394913714, 2088681890,
        2743885961, 2257026807, 2812703572, 678096205, 2964972038, 1641032123, 3238217254,
        2452280240, 193873172, 277301379, 106064560, 2264572378, 3461606200, 1584050797, 14355481,
        3385840230, 1703326352, 1625259628, 3642322228, 911402341, 2158835226, 939248485,
        3607511108, 2863853568, 1611642161, 1312857772, 1839433327, 567060478, 3139863681,
    ];
    let out_len = xs.len() + ys.len();
    // (r5[n3] & (Limb::MAX << (Limb::WIDTH - 7))) != 0 in _limbs_mul_toom_interpolate_16_points
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        3998843185, 3237409891, 364765898, 887299373, 875693912, 790653310, 1949338310, 309040598,
        2753929769, 1560315881, 2158749638, 124625299, 1949071109, 4293842935, 3418183766,
        1387429696, 64843603, 1303399904, 455978049, 3724928213, 4182321093, 1342619213,
        1692503310, 2594578249, 2811338438, 1715625698, 751013184, 1529801113, 2582454920,
        4199343251, 3183268625, 2516721877, 1167772050, 2317983168, 1793272983, 311653702,
        3588179354, 661601476, 2154410870, 2334965650, 4135084105, 1682699224, 47903600,
        3273743199, 3845966395, 1357302998, 3756718174, 2451701689, 2321438159, 3211448326,
        2377823945, 50814995, 1672303030, 4158805623, 2661886690, 1846253587, 702414278,
        4059841129, 3727323213, 1424047747, 2939622087, 2231052374, 2013876172, 2053003398,
        1741887596, 3509712959, 5142212, 3825464748, 3375048072, 338658021, 2655991044, 2889153792,
        2332483687, 934832926, 3863652984, 1414099507, 2895368376, 1013122176, 2794762768,
        2981493251, 3152252275, 1564424419, 536147906, 242465174, 3000707896, 3526733161,
        943706939, 349997931, 1497577916, 3473622068, 1517005385, 2206423568, 1544165865,
        3199998353,
    ];
    let ys = vec![
        1562512360, 3239315566, 2225439589, 502536858, 1867965636, 618137922, 4149231651,
        476678563, 4203415530, 4178036608, 1956783646, 4023049148, 2645084690, 270122366,
        201340005, 4276855303, 1021151730, 916821881, 663141922, 2795604136, 3762385264, 348487994,
        2655354829, 343872945, 2028904125, 1525417887, 867188532, 3911999830, 2139706847,
        3256484706, 961423019, 1530068826, 3577946967, 2361035355, 337639742, 3774308229,
        2185652798, 3532716804, 4018761888, 1357817255, 2216301712, 2861241181, 3053055924,
        3777579308, 795689292, 3386662598, 4160296368, 2005833155, 1297354264, 2851045342,
        954306552, 1613754854, 2227385445, 528669733, 3315107199, 3402866739, 1117279433,
        232818134, 1490857876, 1962534623, 1227821174, 159891958, 1385848424, 4061426539,
        647828819, 2061390815, 4239314784, 1854131914, 3258304017, 524974854, 450125309, 684998491,
        2942294237, 4191667771, 2230185588, 1844054665, 193300986, 2652500966, 4050934267,
        1133780381, 3709046706, 909867408, 4209959016, 4275912160, 277155368, 1775051743,
        4190065677,
    ];
    let out_len = xs.len() + ys.len();
    // !(an == bn || an * (TOOM_8H_LIMIT_DENOMINATOR >> 1) < TOOM_8H_LIMIT_NUMERATOR * (bn >> 1))
    // an * 13 < 16 * bn
    // half
    // s <= t
    // half in _limbs_mul_toom_interpolate_16_points
    // spt > n in _limbs_mul_toom_interpolate_16_points
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        2456061149, 2562918666, 2903450513, 1994190773, 99234624, 3722083920, 4262323306,
        202219441, 4201857695, 3988878636, 1533308334, 401400520, 1069756554, 2457773969,
        2892388936, 3423117995, 1944069442, 492036629, 3426800580, 2282483359, 4006366620,
        1695364515, 2555180845, 1669287836, 349290429, 778467450, 2020203604, 2218159817,
        1450404019, 1278304750, 2412695340, 1592154884, 3868182043, 2240370481, 3859902860,
        1008825116, 412233394, 2475457637, 3664379433, 4204584226, 2750684469, 4113507475,
        2916584959, 285955744, 739598569, 18278051, 3768126932, 2181905109, 2612988076, 1827656088,
        1160380415, 4160443718, 1846086671, 3050604645, 2547108010, 2828666778, 3252702258,
        3885923576, 2331974758, 730724707, 1528859315, 4288784328, 3677151116, 445199233,
        3304488688, 3566979465, 3541025426, 2491779846, 3112990742, 2583249486, 3403111749,
        1930721237, 3428792463, 2896462048, 2985885576, 1819460734, 21206096, 3560441846,
        987100555, 2844904275, 84854892, 1268249628, 3963306788, 3338670067, 2504599089, 65588657,
        321493327, 4249673617, 4150876068, 721566898, 2186945060, 922948272, 1502464627,
        1426914435, 2906888275, 3454987739, 2609132626, 2073366782, 1058809001, 1226951003,
        2624503637,
    ];
    let ys = vec![
        3941840558, 1662743930, 1905993615, 2485835810, 3925643251, 3071436009, 851721712,
        1325046168, 3214018378, 1465803515, 2459667310, 2361559987, 2668552637, 2425633974,
        3200812339, 2594448814, 4170435967, 1112582678, 3198704424, 4028094030, 2482710119,
        2990475705, 708195759, 612294539, 2794828841, 2498141427, 3805184114, 3010938369,
        1479667740, 660767380, 1641177565, 1782849661, 1915222559, 1626388136, 1816788637,
        1338361170, 783877621, 4003339370, 1930607900, 1259399167, 3351643097, 1641708262,
        967800396, 1800752717, 2198926109, 1163817943, 2710351254, 451351637, 1285647338,
        865168955, 645286276, 2685132510, 1773153387, 4273868103, 2604563645, 4105767904,
        2556376985, 158907213, 3579937882, 3059825408, 1920542835, 528717490, 1430681949,
        616489338, 597761261, 3760865497, 963173252, 2915089223, 1441674715, 1717557648,
        1819215517, 3449795284, 844168976, 1574237607, 758725457, 762624299, 533122182, 1201164787,
        1968174784, 896982568, 3419630169, 2247559545, 3983311870, 3975342941, 1112833399,
        2721518545, 2493587613, 3444837338,
    ];
    let out_len = xs.len() + ys.len();
    // s > t
    // spt <= n in _limbs_mul_toom_interpolate_16_points
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        2166912886, 3021127478, 1088026295, 863338925, 1902617744, 2706401163, 3211745137,
        3537828549, 2310228205, 2585051285, 3210490216, 612524924, 269492174, 83675252, 3088638931,
        2020592214, 884676247, 2114372012, 2448236682, 3651962645, 4142890271, 3807368959,
        3038213130, 1740849525, 1839016815, 3718350068, 1798083657, 4018300117, 2557824626,
        1367910868, 3524299249, 2718734101, 2199735437, 2156117642, 3314330151, 91570504,
        1763771190, 730175380, 3035959105, 930897603, 4104577491, 1545111417, 2973200358,
        1531233892, 3216274102, 2879326700, 4043195388, 4012932329, 1225928231, 3148638781,
        3350412374, 571148440, 42117077, 2619230436, 570695610, 3533920410, 2337569860, 2616128436,
        1101128308, 986097032, 4127211776, 1459526104, 121723950, 1459838938, 1563443987,
        3106615121, 2637954840, 238917822, 3086105506, 2960421944, 2937286162, 3871313970,
        554575295, 450448609, 493464699, 3492897008, 3198787067, 2691863142, 874317820, 1804414164,
        572281701, 2867423932, 412542374, 239109523, 4270925097, 1858402222, 3784404338, 162014339,
        182208178, 171269941, 1556499146, 3122050585, 2070559038, 1293272336,
    ];
    let ys = vec![
        131674806, 603734923, 2440163395, 2896151903, 2142986136, 3702794463, 407655836,
        1281722924, 1990690788, 2883417209, 1106804242, 965105623, 3369860750, 2422075060,
        1042530548, 1864787458, 1722387953, 324177444, 3169639558, 1324636283, 1394919591,
        1382200609, 4014256585, 1943865290, 1318181231, 2753206532, 465681637, 3556126827,
        3726586809, 2859198026, 1880611700, 2743775719, 2312093882, 2611444395, 2043850780,
        1748249887, 1827465861, 1827026074, 3842470222, 886015214, 1202152837, 1760966154,
        1303682364, 2141063912, 2027419958, 3046273896, 276337299, 1629565318, 3973822671,
        3586055166, 515343743, 4150823547, 3812419028, 4047886683, 408756427, 30807697, 3839670586,
        3241113948, 1946580966, 211283947, 1648787704, 1254977229, 324210665, 409019127, 999906525,
        3589880779, 2652719468, 2740912614, 75319316, 3276454084, 3598090610, 225502084,
        1039377126, 3755265351, 299690912, 2582901309, 891564570, 1062813956, 318910996,
        2153235228, 2834278326, 130377847, 977327805, 3290994684, 2956083989, 826986477,
        1417957671, 2007397536, 3845476521,
    ];
    let out_len = xs.len() + ys.len();
    // s < 1
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        1012246656, 3781649075, 2144318856, 2608903399, 688555306, 1040133166, 3831584367,
        1593112617, 1823654254, 840638304, 3109717334, 188514461, 398797195, 75875990, 1486449995,
        4269057266, 3729965858, 1861862237, 3631015569, 3651675458, 103019792, 4115125912,
        854107191, 437995362, 1626634580, 1556708150, 2197935825, 142443256, 2516681044, 165384798,
        622726627, 2804275513, 3768014324, 1019999140, 1630141384, 1569491385, 2650112147,
        404117490, 959368136, 1567892691, 3740061638, 1492035182, 2806958299, 3558543973,
        2394278513, 193040368, 140963621, 2363913022, 521760299, 1509309827, 1222622424, 236238235,
        148145098, 1185145642, 4050835140, 3496710859, 2912031916, 2811044753, 293786270,
        1593967022, 3059741198, 957447590, 999733770, 3225819121, 389969264, 1617194653, 930042654,
        2073424372, 1334988223, 2244143480, 3036433790, 314486992, 3505856530, 2253001666,
        2732695676, 2150239253, 2058771616, 2553846568, 3156714961, 275374496, 2154639432,
        1705499511, 2661128488, 2996751598, 1991220721, 2971546013, 947096109, 1988630082,
        3629027637, 2894867708, 982953971, 1288656915, 3544920961, 2725968940, 2718109332,
        1685012966, 2463009759, 1861144639, 2364403606, 3459863283, 983775524, 3466796660,
        1976698215, 708098181, 3069387825, 3638611575, 2579187312, 632774203,
    ];
    let ys = vec![
        1809516468, 2803977220, 3078159083, 486681337, 1568336896, 4117841648, 422990983,
        2706208156, 3747890395, 2705136812, 2904348475, 1582408791, 723059442, 3021061511,
        4080366324, 344817763, 4291264074, 846996023, 4266039848, 1034099747, 3469554547,
        1098932136, 4197098884, 2840685725, 3598360260, 3858664271, 2988904929, 3788334949,
        2778508367, 2862059554, 3453038230, 315104137, 659918534, 3119028578, 178870393,
        1471088291, 908295683, 5373305, 1643272591, 1306263419, 808966614, 4084169993, 740212697,
        4046005160, 2962244838, 2183688745, 2126344144, 2041407930, 201066579, 4119015900,
        3263668172, 1482349211, 660638692, 596028971, 3002749394, 3127689329, 147925750,
        1069598238, 1868876453, 1293290441, 1391999979, 1064595909, 1912901608, 751720124,
        313663396, 2718231373, 1813378594, 1913592155, 2372166689, 312370283, 1294902637,
        1519106439, 2159217107, 3862662328, 3650935678, 3673744494, 1365354839, 4239084491,
        2676645359, 906655247, 2012326184, 363781147, 121405308, 3179196888, 1415338639, 788446024,
        2165764832,
    ];
    let out_len = xs.len() + ys.len();
    // Limb::WIDTH <= 9 * 3 ||
    //      an * (TOOM_8H_LIMIT_DENOMINATOR >> 1) < (TOOM_8H_LIMIT_NUMERATOR / 7 * 9) * (bn >> 1)
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        4119986492, 3769784140, 1016845783, 1462133596, 4070906664, 3720888633, 4162266585,
        357581522, 1461543577, 4176530320, 4211178471, 3101664977, 3852655570, 166352500,
        1437797012, 3499540684, 1659293446, 4040889056, 2872135683, 3443479700, 655062062,
        1438477128, 1251147166, 2862092792, 1899445621, 1706155530, 2740470033, 732343724,
        3637646944, 4084594941, 2604690616, 4034800391, 3052473123, 2211244267, 947388355,
        584537104, 4143732645, 753960748, 3490638800, 3716731483, 812984705, 1845462359, 65215620,
        4176252687, 2616921776, 2554085123, 4119079055, 4015290385, 697509015, 234073199,
        845662165, 1354305840, 981298174, 1565184955, 207005143, 3409837524, 1220287572, 729153595,
        4103593694, 3696910742, 3965466426, 2266950204, 3856396952, 1764904477, 2684424799,
        2851670593, 1238534904, 1193928568, 775873269, 1360693711, 2015831201, 4011315900,
        3412793575, 214657369, 4288738109, 2288646350, 4016569358, 3132961648, 4045851426,
        3660819126, 4044839853, 3089247133, 2180567261, 2646234732, 1387965746, 2657998851,
        713566741, 3356621670, 3732665499, 1904626236, 64110644, 1408823950, 3590020345,
        2474929782, 849015605, 44073994, 1392682200, 2899713947, 276297197, 2522590522, 3057126922,
        2424068009, 1656987557, 1344629217, 2147192728, 3358875432, 3127883048, 1416207351,
        2542101426, 711240683, 2104649063,
    ];
    let ys = vec![
        2166824272, 3241826034, 3119928903, 4235394337, 702909009, 952063230, 3767289278,
        3471432542, 1289423414, 4165356232, 1144080646, 1098693005, 2158644075, 3466960484,
        107907398, 1849951849, 1697379716, 3245621651, 789557144, 3055443426, 3784862213,
        3687293729, 3527108073, 2085509714, 2098672286, 4237955923, 1799505183, 4280924128,
        1714047371, 679046973, 2920210487, 2630108623, 3799940507, 2820960341, 2480102998,
        3063576036, 1124333889, 3649141414, 3766465016, 1301782752, 3365747207, 318110166,
        1798715740, 3939897237, 1972418626, 525713989, 4204639302, 1845175119, 3066964494,
        3197166778, 2045294098, 1778200774, 1122512884, 487879411, 3912690682, 2631572995,
        119236796, 3659697136, 875446358, 2784882013, 724223194, 2290104863, 3553626657,
        1049986268, 1149074120, 457683007, 342994481, 3969592954, 4124706173, 793289745, 50385201,
        428623925, 330776585, 154172871, 652756593, 1305471058, 3295431270, 1976260297, 1729803474,
        1132360814, 2965768226, 3482945302, 2017386623, 1093051437, 2874103717, 2882475975,
        3735654948, 1766940801, 3723445548, 3203977826, 1788553316,
    ];
    let out_len = xs.len() + ys.len();
    // t < 1
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        562392731, 220608607, 3016011233, 1988425644, 1753293069, 202000452, 2988281129,
        1833355482, 2406139229, 3819843447, 3864310556, 2964129037, 3243750205, 1300008578,
        213321522, 4162936161, 3499001762, 2548817881, 797422884, 3464557820, 3172918275,
        3342298017, 4095467160, 1278405537, 2731045246, 1797909329, 915931552, 1234105774,
        1721010619, 393116356, 3595672812, 246921897, 3156619416, 367413315, 835896205, 1133867872,
        732164137, 2864249493, 4191919416, 2012484604, 2046119300, 464214194, 1309621688,
        2133767576, 1817717936, 3210357881, 2703486295, 73128890, 3834854978, 1247202589,
        3867658887, 743571365, 623502109, 2414902368, 4157134303, 505113368, 3563229135,
        2326845431, 1870329856, 412186635, 643126122, 918171482, 3174437348, 992920198, 2549886607,
        2594507263, 870344606, 3354423872, 3768408002, 1124888954, 3015715321, 3554830011,
        153164314, 2571405898, 3088317836, 3826710038, 532463221, 2174408986, 4066384743,
        2858347925, 3362316763, 3912725306, 1672655485, 747559434, 2494848220, 3353179599,
        2958541661, 2754014801, 2253228000, 3548360599, 2532574632, 3609949183, 4224112455,
        2830762232, 1638592699, 748357099, 2027377618, 2154359009, 2042715188, 2328113060,
        2228778844, 3805284055, 3740811424, 437279916, 2305090412, 2502181871, 3285232891,
        3972490704, 3821166397, 3272678301, 2818983671, 4257635933, 1730183078, 4193248424,
        1863033893, 2751966968, 1985590742, 1553448103, 2731396486, 102894954, 1596356734,
        2399109494, 326183031, 3303826610,
    ];
    let ys = vec![
        1675796150, 1752707855, 2960577702, 4246206199, 1769535683, 1968809225, 2828046910,
        2881173858, 4049894594, 690462953, 288094502, 2301238042, 171278398, 2941234911,
        3855716963, 3569445656, 3999649666, 1033046275, 1441788099, 1121368236, 3979411258,
        1744237927, 2218358768, 3293576320, 3290293896, 2918243870, 1271587143, 1530970846,
        1057501000, 1208621673, 1776318661, 2630121830, 1577699073, 3947123592, 1916313897,
        3189157970, 1684300643, 5245214, 2973935012, 1013692937, 2575458340, 1202811269,
        2350985644, 938605227, 710807110, 3840777315, 2476378686, 1408221563, 3963538750,
        1495981337, 345677390, 2267206171, 597425252, 3652332994, 1484311898, 395641995, 508511757,
        1756437663, 1140313927, 4146891666, 1764315654, 3179667093, 2753886170, 2955381796,
        1486042517, 194560773, 4113616196, 3870970045, 687965138, 970031260, 4029682995, 652798493,
        3718790353, 2790548419, 1973920939, 1737499520, 3093968446, 4016940528, 1440510403,
        2896783742, 3442955437, 3111677005, 4265014223, 2141411993, 177598581, 1546615872,
        1296900550,
    ];
    let out_len = xs.len() + ys.len();
    // an * 10 < 33 * (bn >> 1)
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        2699110155, 1597521691, 470373633, 1547603733, 1114505968, 121868046, 1203637014,
        1508031395, 2678363006, 1428373366, 181016145, 2228522822, 3784155833, 1174663302,
        3119880811, 3351843127, 1893166310, 2733160757, 573074872, 1444139090, 3771161592,
        3202218806, 1184188558, 1337716051, 2651973158, 1523269291, 3416369561, 374511279,
        2679410392, 1510022750, 228616166, 4003251265, 4290642350, 3280834410, 1463007103,
        2311946289, 160203186, 1585276951, 3812024477, 3976220702, 3453132955, 903478724,
        1692984648, 32969770, 393253462, 2089515635, 2580037721, 1368262724, 3975524017,
        1095890302, 3362835893, 1467244702, 3126524190, 1558041706, 1473844963, 2931771668,
        769941843, 1383766743, 2048229827, 3587516656, 744923988, 3114188668, 2900631137,
        1550641047, 3971430916, 1024708451, 266103704, 1961354549, 2996989736, 96509114,
        3209890269, 558760343, 1942895993, 3030238742, 3901981217, 1553802266, 1100766439,
        3617908428, 2903765815, 160559154, 3223711195, 1505354960, 3400362702, 1532921847,
        2633984159, 2547091597, 3753857128, 1603256426, 1467979288, 834683287, 883770936,
        2091938738, 717946381, 1738927478, 4212395432, 3554713903, 2891799196, 2460462345,
        1068611661, 1983982847, 4254702408, 2862607717, 205351503, 899537845, 4178691861,
        2027719370, 1613590765, 1667586567, 658709687, 569869145, 2542265621, 4018309335,
        3115945617, 1860868443, 2042873761, 2857432666, 3454761191, 644158605, 952236065,
        1246066126, 1054146509, 820815201, 4116210106, 911797864, 980581305, 3662945636,
        2395465042, 2988547838, 1592529958, 4123985797, 1086072833, 1344358819, 2713461665,
        1166149285, 868088866, 120572741, 2719927699, 1609748024, 1381464015, 2371158669,
        2027765235, 2167125167,
    ];
    let ys = vec![
        1088368182, 3374520919, 2135624591, 387360487, 3348241848, 2559227752, 3399060139,
        2714380393, 371475119, 1878664574, 3306012397, 3678253780, 2537332523, 634258529,
        2378309044, 1907416933, 2176550942, 3624058493, 608851538, 77324946, 854257549, 2563267740,
        1842976277, 2560652658, 1177372492, 4164431297, 2857340159, 2813781292, 3608170666,
        289363804, 1276568988, 1858470908, 2027103570, 1210716416, 3885179582, 980951621,
        1332461771, 2439102632, 78855299, 1535655076, 820717475, 1372739985, 4277759699,
        1928781862, 2056547589, 2689637269, 3487926306, 1712399855, 2387894324, 1345157890,
        420194957, 2408734980, 1088476282, 1237271902, 1570597541, 1299046081, 2179334980,
        3757788366, 1320170918, 2220338411, 3413493273, 4047658929, 1004605073, 3758106669,
        3623304103, 2595195415, 3392723185, 227342906, 3297612463, 1577658966, 3646845515,
        1442494023, 1805636027, 1293916606, 1856823520, 2157779944, 1701394115, 1586957718,
        2203990942, 3794477956, 470446365, 3294563814, 2801795027, 2712013665, 1473818504,
        2726878536, 4276109446,
    ];
    let out_len = xs.len() + ys.len();
    // Limb::WIDTH <= 10 * 3 ||
    //      an * (TOOM_8H_LIMIT_DENOMINATOR / 5) < (TOOM_8H_LIMIT_NUMERATOR / 3) * bn
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        561532292, 1489901668, 253691236, 2318497628, 4251899866, 2953100123, 2461942387,
        3249119706, 369296206, 4217598289, 2953582842, 2377320700, 2568035293, 3298340549,
        2920237456, 546954422, 3577037488, 92033404, 145112422, 2502470868, 1400281201, 2303329463,
        633903343, 3944799609, 57410139, 3300617501, 2988597979, 3756577241, 1111328153,
        2315706065, 2359556880, 170569603, 1875977300, 2265470483, 1673672630, 2694260146,
        620660163, 4086502272, 2268845329, 2531408738, 745892765, 2985301421, 641961881, 620799476,
        1513471210, 2206613713, 895576219, 3432428917, 1326478424, 721293050, 4129832181,
        2328492091, 790053303, 1886834609, 2560250292, 14318242, 2263105643, 3768652300,
        3685567034, 1053183071, 4035043131, 1140590999, 1312717632, 820131789, 2381319255,
        515196511, 2436315339, 513976227, 688721295, 2969875582, 2843970288, 567346371, 2277297382,
        3266747935, 3125131739, 391700432, 2628083321, 779071641, 2971551059, 3314957816,
        871191953, 3336232721, 2709555815, 918246312, 923872244, 2827827195, 2966239254,
        1586350108, 1024706608, 3525365202, 594940169, 1872199600, 3239665333, 694926057,
        4271587637, 3916707341, 2190558956, 2300957253, 772629754, 238192213, 4247448230,
        3565892036, 3184365211, 2516885224, 3979985839, 1180780557, 783722885, 1061155274,
        3798456603, 3320505371, 589311966, 1623819314, 1001947009, 4232577387, 474033387,
        3930737007, 1729002759, 3148522805, 658463592, 1424102704, 2305467923, 552214960,
        1642169523, 2066768192, 3794357111, 3557589618, 4204044663, 1778418301, 1181058217,
        1612951946, 588858899, 3836952607, 2977777237, 9660119, 2962495164, 2992962211, 3923151463,
        3345257705, 2981383558, 2363319525, 3608470059, 874691575, 2586822309, 912499640,
        603852379, 1888867173, 2770352234, 4238262229, 3877831016, 2596074823, 3663087235,
        542677879, 228437282, 480155344, 709141324, 782255006, 2839979153, 1271748198, 1031245745,
        3053801112, 3462023195, 172164778, 3874269611, 3279470898, 4076666435, 3596981639,
        810288236,
    ];
    let ys = vec![
        2267307147, 2856749182, 90961593, 1052868712, 3437758783, 899762302, 2825414504,
        3100252964, 214994098, 4262558841, 2740902902, 1743352008, 1922058509, 2975766063,
        3399126202, 897115238, 401142729, 1715015464, 244955103, 3177992227, 405891649, 1768495060,
        3524094602, 4080016656, 1432684874, 3397000143, 434821341, 1754546815, 4094846559,
        4286153335, 2240106918, 2310322076, 1713831329, 1428414845, 2188185809, 2111765503,
        1131727372, 929039425, 465389857, 2677898170, 1160632541, 3376736943, 491317513,
        3242464822, 2045506450, 1242019843, 3965879224, 2484620055, 3447163057, 2809067396,
        2409780789, 548871240, 2024164190, 4133800101, 105887616, 4257692355, 1942633927,
        1532037864, 2395107706, 1815832330, 3470252735, 3388820081, 2275739186, 2499364631,
        2076801086, 3670985009, 395675635, 4219873512, 338672631, 3757753689, 730801911, 529959909,
        393050276, 2506914867, 349901023, 889932113, 2359995672, 2260685091, 3193258383, 993644814,
        660499678, 4213349264, 915065087, 44382277, 1138965336, 3728412916,
    ];
    let out_len = xs.len() + ys.len();
    // an * 6 < 13 * bn
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        2628750713, 2361938253, 4208887130, 2080756349, 672997060, 2130716095, 4212789307,
        1848408220, 3496438901, 84923093, 3765911616, 1894564551, 1611354899, 273564832,
        4150644671, 3064400972, 1543250045, 2858928926, 1070491873, 1579001797, 1184344436,
        2022081087, 579467674, 3617124184, 243126922, 3969739693, 3428743965, 4195070089,
        3234082950, 333482038, 2496442330, 894715026, 434494401, 2735937738, 194433417, 3547773069,
        1310458322, 1092526211, 460831665, 314882384, 352225614, 2524634920, 3907974253,
        3587596708, 90585625, 3922151265, 2706453821, 2479984430, 1899379393, 521798300,
        3544490367, 4025847744, 520557399, 1960228079, 2440638916, 3922652110, 2874111917,
        3780219346, 1155970954, 3101729918, 1154605497, 1641746684, 3885558155, 713658859,
        2298415211, 1104859444, 397648670, 938276629, 2245930839, 351999985, 3962599907, 162580649,
        4135267160, 3893533927, 708603373, 3649893874, 1549341047, 446919848, 3848748260,
        1193215655, 1667453481, 4263900238, 3083741929, 569862864, 111540402, 371222591, 836814821,
        2523967214, 3373518119, 288800478, 2983910658, 3822451776, 3717238299, 4103554210,
        497321656, 1267537380, 2210886058, 393666292, 2341926460, 2993069655, 3449632275,
        345728673, 1850135319, 1546568315, 349065480, 4148532822, 2743969263, 1135023914,
        856540508, 710683508, 621037301, 2245404525, 1375763902, 4230256152, 1103848377,
        4068950000, 2774111626, 4005998377, 1420452414, 142442998, 296389949, 1793483671,
        3236856344, 1470778143, 2199111141, 1485252921, 3021831668, 3409728715, 494048497,
        425352623, 547187992, 307378564, 1878128309, 3632431108, 3608263098, 3158948042, 268203532,
        1889965332, 2413564070, 494017444, 4018318246, 2256416411, 2325799856, 424840978,
        1475143253, 2578705133, 3454058108, 875893914, 3369487214, 2161583703, 2368049199,
        3710749831, 2234731371, 2548143256, 1212646047, 775618131, 821458424, 3027168315,
        841398247, 3991240853, 2094376383, 3587145176, 1943420573, 781156526, 2434343084,
        2126213029, 2402207510, 4019808646, 316909832, 2750686513, 2438176721, 308346316,
        242903105, 3531437189, 4095795963, 2087963376, 3007755141, 1683404210, 3086330285,
        1333246101, 1581088323, 1356633529, 3666603849, 540703941, 1410918479, 2987931996,
        2750320701, 3483743338, 2503688388, 3308034421, 3019960566, 2668657879, 2363438262,
        1470517413,
    ];
    let ys = vec![
        2312659839, 2350424241, 1787407270, 1271425122, 4187967770, 818645453, 3539315256,
        2178962268, 2575529612, 3589703821, 2051328589, 1350506812, 1181962471, 440960359,
        1364212437, 3414960630, 901255513, 1225743051, 2301315145, 1970642256, 2850715818,
        3128888797, 2317420929, 2155667782, 1962983120, 2710186451, 648444928, 2272821232,
        133989660, 3141011857, 1529770260, 802759102, 2173416392, 1305065341, 45650077, 1082105231,
        1602486318, 3755990436, 1936896216, 2400713018, 1591016508, 4068454220, 3596573883,
        2619324298, 33580971, 2286577695, 3083324417, 1169438566, 3225233768, 808739442,
        2766243970, 3455083573, 1549857550, 3592398125, 2248831497, 3521856807, 1967034,
        3078700295, 1346379862, 3820864333, 2903766704, 3884607466, 4174763992, 270916374,
        3218398044, 3434381035, 159751999, 2768080251, 2464394277, 566049661, 442155673,
        4112913396, 1456961327, 38309439, 1525792638, 2372197825, 1956558568, 4294769490,
        3096019721, 2031664251, 3017984223, 1381760341, 4260655051, 2253457354, 2984264086,
        1088854315,
    ];
    let out_len = xs.len() + ys.len();
    // Limb::WIDTH <= 11 * 3 || an * 4 < 9 * bn
    test(xs, ys, vec![10; out_len]);
}

#[cfg(feature = "64_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_8h() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>| {
        let mut out = out_before.to_vec();
        _limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        let out_after = out;
        let mut out = out_before.to_vec();
        let mut scratch =
            vec![0; _limbs_mul_greater_to_out_toom_8h_scratch_size(xs.len(), ys.len())];
        _limbs_mul_greater_to_out_toom_8h(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    test(series(2, 86), series(3, 86), vec![10; 172]);
    let xs = vec![
        4161517722334671206,
        271035878974614969,
        8768264966582261304,
        8206956804546718361,
        10016740860128464264,
        2943682457917422384,
        10577659840261915262,
        12098681961003341371,
        2525073961085508373,
        6868684266500244649,
        509821878609210517,
        4263249474085213536,
        2307565444887817803,
        12419028787241261317,
        1281995004584322618,
        13869993964458308221,
        4485392892470363180,
        3274613913599014818,
        13075432300049036016,
        14042578030952079199,
        13098932791585915075,
        10142506622182970580,
        7251838551095799764,
        17051632328075633635,
        14834683551906590335,
        18022997779550454484,
        13851155116066438974,
        3279920275984726839,
        12575373964173554443,
        15489604937489489906,
        12630529117895897413,
        9562379919499143009,
        1417878505992996127,
        2188363987094684136,
        4744951957683006311,
        12112952790370550632,
        313413052918057660,
        952838993607855174,
        5933080761807357068,
        5875775551766205334,
        10228588026136726855,
        13111641204516926240,
        10636665232562365918,
        11359964631071199362,
        5929704785320756798,
        7890881054270407934,
        4884891330151666074,
        11055829837821054078,
        13707765469312479203,
        8153558212434726394,
        17445193585880639275,
        6568289716541023323,
        8041757936108402209,
        11089742802624534358,
        9104866424438942973,
        3236275382520001400,
        9213626463300221545,
        5359296447813232573,
        2888775200925828643,
        1504166968227419931,
        14327007717613163305,
        11802896026004225094,
        12726419078417922871,
        13309155468447837337,
        8586421913645886721,
        53962250520164792,
        10299535356260218467,
        16946113957982976032,
        2902460381404773190,
        14757465720632393328,
        4285719983639600380,
        8437230965528545912,
        5716398831975234496,
        1373020012523386515,
        3326027605041066746,
        17656221602314109866,
        5927567778944922379,
        7395768072445629410,
        11551011221061348004,
        13862329630891761456,
        3443745263810155735,
        497965567194021216,
        13073929868627981515,
        9340721263069758697,
        16189911797862953019,
        17331477506134450185,
        18441976800868209749,
        3733349995001197864,
        6937510789920909911,
        10459182483341515090,
        16282716012969111817,
        3142838808933013004,
        176169927348158611,
        11447076894000834768,
    ];
    let ys = vec![
        3898028307372664956,
        17056541935478225194,
        14004255653437064260,
        5500365157672511509,
        15774417221201329293,
        3229812365626959565,
        1542674716041014040,
        7356251598468809943,
        18181760582149085284,
        6447899299954117957,
        15228766707939040914,
        15272444333081468110,
        8256864946368840840,
        15131537266446006793,
        15615697223616434527,
        18149135087211146951,
        6359898540214993921,
        11306735121000975748,
        10447887135010383963,
        12772438236294882417,
        17631737056955710770,
        8945404460793598129,
        8945720889114856152,
        3648711115155303988,
        4353348842999127960,
        2258094147328762698,
        17154005505580115535,
        13882701371593165208,
        1610163839528654069,
        15350954595089578211,
        2071555476679360064,
        7797386300145290156,
        12827100752536039252,
        9294676638100895403,
        13194197740670114341,
        9490868657650122292,
        13133123495028388830,
        12350221742051084451,
        12424378851382358824,
        9807292823459903392,
        10987641767148832341,
        10914994897211362878,
        828242546480310184,
        18006801931269403354,
        3042908768715701160,
        8117699035539485321,
        11944855102415629844,
        7384949013429384602,
        11066738683960763872,
        14686958392900209441,
        16412025437157422416,
        1334344044228684681,
        1631366399820348565,
        18062594111889109095,
        5175299421808157128,
        16616812968596909641,
        797326939277169478,
        14593183003025528412,
        3580961852669434633,
        2104948106588459323,
        14322976299272137248,
        3536903766355663369,
        6932211742640251008,
        17616766237027326857,
        1477865108082927148,
        7817082715310166375,
        16183969129154492111,
        18146981620947356859,
        11618268397687338183,
        15294321769160092821,
        2447614867702883346,
        15261926111061449320,
        4029723450982123355,
        7820711996327940306,
        6188156586792352365,
        15703528769184364862,
        6698415575574578533,
        7770946582061166480,
        3543987370105940918,
        8845414905041844753,
        13110356713999163167,
        12862812457872444435,
        10749027774576978236,
        17822296942008093229,
        13898152040175560707,
        1879212271519144526,
        5428215269251527991,
    ];
    let out_len = xs.len() + ys.len();
    test(xs, ys, vec![10; out_len]);
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
fn limbs_mul_greater_to_out_toom_63_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_21,
        |&(ref out, ref xs, ref ys)| {
            let expected_out = limbs_mul_basecase_helper(out, xs, ys);
            let mut out = out.to_vec();
            let mut scratch =
                vec![0; _limbs_mul_greater_to_out_toom_63_scratch_size(xs.len(), ys.len())];
            _limbs_mul_greater_to_out_toom_63(&mut out, xs, ys, &mut scratch);
            assert_eq!(out, expected_out);
        },
    );
}

#[test]
fn limbs_mul_greater_to_out_toom_6h_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_22,
        |&(ref out, ref xs, ref ys)| {
            let expected_out = limbs_mul_basecase_helper(out, xs, ys);
            let mut out = out.to_vec();
            let mut scratch =
                vec![0; _limbs_mul_greater_to_out_toom_6h_scratch_size(xs.len(), ys.len())];
            _limbs_mul_greater_to_out_toom_6h(&mut out, xs, ys, &mut scratch);
            assert_eq!(out, expected_out);
        },
    );
}

#[test]
fn limbs_mul_greater_to_out_toom_8h_properties() {
    test_properties_custom_scale(
        2_048,
        triples_of_unsigned_vec_var_23,
        |&(ref out, ref xs, ref ys)| {
            let expected_out = limbs_mul_basecase_helper(out, xs, ys);
            let mut out = out.to_vec();
            let mut scratch =
                vec![0; _limbs_mul_greater_to_out_toom_8h_scratch_size(xs.len(), ys.len())];
            _limbs_mul_greater_to_out_toom_8h(&mut out, xs, ys, &mut scratch);
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
