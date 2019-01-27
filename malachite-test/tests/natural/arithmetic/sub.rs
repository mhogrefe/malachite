use common::test_properties;
use malachite_base::misc::{CheckedFrom, Max};
use malachite_base::num::{One, PrimitiveInteger, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::sub::{
    _limbs_sub_same_length_with_borrow_in_in_place_left,
    _limbs_sub_same_length_with_borrow_in_to_out, limbs_sub, limbs_sub_in_place_left,
    limbs_sub_in_place_right, limbs_sub_same_length_in_place_left,
    limbs_sub_same_length_in_place_right, limbs_sub_same_length_to_out, limbs_sub_to_out,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::base::pairs_of_unsigneds_var_1;
use malachite_test::inputs::base::{
    pairs_of_unsigned_vec_var_1, pairs_of_unsigned_vec_var_3,
    quadruples_of_three_unsigned_vecs_and_bool_var_1, triples_of_two_unsigned_vecs_and_bool_var_1,
    triples_of_unsigned_vec_var_3, triples_of_unsigned_vec_var_9,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_limb_and_natural_var_1, pairs_of_natural_and_limb_var_1,
    pairs_of_naturals_var_1,
};
use num::BigUint;
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub() {
    let test = |xs, ys, out| {
        assert_eq!(limbs_sub(xs, ys), out);
    };
    test(&[], &[], (vec![], false));
    test(&[2], &[], (vec![2], false));
    test(&[3], &[2], (vec![1], false));
    test(&[2], &[3], (vec![4_294_967_295], true));
    test(&[1, 2, 3], &[1, 1, 1], (vec![0, 1, 2], false));
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        (vec![0, 4_294_967_295, 4_294_967_293], true),
    );
    test(
        &[1, 2, 3],
        &[6, 7],
        (vec![4_294_967_291, 4_294_967_290, 2], false),
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        (vec![4_294_967_294, 4_294_967_295, 1], false),
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        (vec![2, 0, 4_294_967_294], true),
    );
    test(
        &[0, 0, 0],
        &[1],
        (vec![4_294_967_295, 4_294_967_295, 4_294_967_295], true),
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_fail() {
    limbs_sub(&[6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_same_length_to_out() {
    let test = |xs, ys, out_before: &[Limb], borrow, out_after| {
        let mut out = out_before.to_vec();
        assert_eq!(limbs_sub_same_length_to_out(&mut out, xs, ys), borrow);
        assert_eq!(out, out_after);
    };
    test(&[], &[], &[0, 0], false, vec![0, 0]);
    test(&[3], &[2], &[0, 0], false, vec![1, 0]);
    test(&[2], &[3], &[0, 0], true, vec![4_294_967_295, 0]);
    test(
        &[1, 2, 3],
        &[1, 1, 1],
        &[0, 1, 2, 5],
        false,
        vec![0, 1, 2, 5],
    );
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        &[5, 5, 5, 5],
        true,
        vec![0, 4_294_967_295, 4_294_967_293, 5],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        false,
        vec![4_294_967_294, 4_294_967_295, 1, 10],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        &[10, 10, 10, 10],
        true,
        vec![2, 0, 4_294_967_294, 10],
    );
    test(
        &[0, 0, 0],
        &[1, 0, 0],
        &[10, 10, 10, 10],
        true,
        vec![4_294_967_295, 4_294_967_295, 4_294_967_295, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_same_length_to_out_fail_1() {
    let mut out = vec![10, 10];
    limbs_sub_same_length_to_out(&mut out, &[6, 7, 8], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_same_length_to_out_fail_2() {
    let mut out = vec![10, 10, 10];
    limbs_sub_same_length_to_out(&mut out, &[6, 7, 8], &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_to_out() {
    let test = |xs, ys, out_before: &[Limb], borrow, out_after| {
        let mut out = out_before.to_vec();
        assert_eq!(limbs_sub_to_out(&mut out, xs, ys), borrow);
        assert_eq!(out, out_after);
    };
    test(&[], &[], &[0, 0], false, vec![0, 0]);
    test(&[2], &[], &[0, 0], false, vec![2, 0]);
    test(&[3], &[2], &[0, 0], false, vec![1, 0]);
    test(&[2], &[3], &[0, 0], true, vec![4_294_967_295, 0]);
    test(
        &[1, 2, 3],
        &[1, 1, 1],
        &[0, 1, 2, 5],
        false,
        vec![0, 1, 2, 5],
    );
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        &[5, 5, 5, 5],
        true,
        vec![0, 4_294_967_295, 4_294_967_293, 5],
    );
    test(
        &[1, 2, 3],
        &[6, 7],
        &[0, 0, 0],
        false,
        vec![4_294_967_291, 4_294_967_290, 2],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        false,
        vec![4_294_967_294, 4_294_967_295, 1, 10],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        &[10, 10, 10, 10],
        true,
        vec![2, 0, 4_294_967_294, 10],
    );
    test(
        &[0, 0, 0],
        &[1],
        &[10, 10, 10, 10],
        true,
        vec![4_294_967_295, 4_294_967_295, 4_294_967_295, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_to_out_fail_1() {
    let mut out = vec![10, 10];
    limbs_sub_to_out(&mut out, &[6, 7, 8], &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_to_out_fail_2() {
    let mut out = vec![10, 10, 10];
    limbs_sub_to_out(&mut out, &[6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_same_length_in_place_left() {
    let test = |xs_before: &[Limb], ys, borrow, xs_after| {
        let mut xs = xs_before.to_vec();
        assert_eq!(limbs_sub_same_length_in_place_left(&mut xs, ys), borrow);
        assert_eq!(xs, xs_after);
    };
    test(&[], &[], false, vec![]);
    test(&[3], &[2], false, vec![1]);
    test(&[2], &[3], true, vec![4_294_967_295]);
    test(&[1, 2, 3], &[1, 1, 1], false, vec![0, 1, 2]);
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        true,
        vec![0, 4_294_967_295, 4_294_967_293],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![4_294_967_294, 4_294_967_295, 1],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        true,
        vec![2, 0, 4_294_967_294],
    );
    test(
        &[0, 0, 0],
        &[1, 0, 0],
        true,
        vec![4_294_967_295, 4_294_967_295, 4_294_967_295],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_same_length_in_place_left_fail() {
    limbs_sub_same_length_in_place_left(&mut [6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_in_place_left() {
    let test = |xs_before: &[Limb], ys, borrow, xs_after| {
        let mut xs = xs_before.to_vec();
        assert_eq!(limbs_sub_in_place_left(&mut xs, ys), borrow);
        assert_eq!(xs, xs_after);
    };
    test(&[], &[], false, vec![]);
    test(&[2], &[], false, vec![2]);
    test(&[3], &[2], false, vec![1]);
    test(&[2], &[3], true, vec![4_294_967_295]);
    test(&[1, 2, 3], &[1, 1, 1], false, vec![0, 1, 2]);
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        true,
        vec![0, 4_294_967_295, 4_294_967_293],
    );
    test(
        &[1, 2, 3],
        &[6, 7],
        false,
        vec![4_294_967_291, 4_294_967_290, 2],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![4_294_967_294, 4_294_967_295, 1],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        true,
        vec![2, 0, 4_294_967_294],
    );
    test(
        &[0, 0, 0],
        &[1],
        true,
        vec![4_294_967_295, 4_294_967_295, 4_294_967_295],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_in_place_left_fail() {
    limbs_sub_in_place_left(&mut [6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_same_length_in_place_right() {
    let test = |xs, ys_before: &[Limb], borrow, ys_after| {
        let mut ys = ys_before.to_vec();
        assert_eq!(limbs_sub_same_length_in_place_right(xs, &mut ys), borrow);
        assert_eq!(ys, ys_after);
    };
    test(&[], &[], false, vec![]);
    test(&[3], &[2], false, vec![1]);
    test(&[2], &[3], true, vec![4_294_967_295]);
    test(&[1, 2, 3], &[1, 1, 1], false, vec![0, 1, 2]);
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        true,
        vec![0, 4_294_967_295, 4_294_967_293],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![4_294_967_294, 4_294_967_295, 1],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        true,
        vec![2, 0, 4_294_967_294],
    );
    test(
        &[0, 0, 0],
        &[1, 0, 0],
        true,
        vec![4_294_967_295, 4_294_967_295, 4_294_967_295],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_same_length_in_place_right_fail_1() {
    limbs_sub_same_length_in_place_right(&[6, 7], &mut vec![1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_same_length_in_place_right_fail_2() {
    limbs_sub_same_length_in_place_right(&[1, 2], &mut vec![6, 7, 8]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_in_place_right() {
    let test = |xs, ys_before: &[Limb], borrow, ys_after| {
        let mut ys = ys_before.to_vec();
        assert_eq!(limbs_sub_in_place_right(xs, &mut ys), borrow);
        assert_eq!(ys, ys_after);
    };
    test(&[], &[], false, vec![]);
    test(&[2], &[], false, vec![2]);
    test(&[3], &[2], false, vec![1]);
    test(&[2], &[3], true, vec![4_294_967_295]);
    test(&[1, 2, 3], &[1, 1, 1], false, vec![0, 1, 2]);
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        true,
        vec![0, 4_294_967_295, 4_294_967_293],
    );
    test(
        &[1, 2, 3],
        &[6, 7],
        false,
        vec![4_294_967_291, 4_294_967_290, 2],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![4_294_967_294, 4_294_967_295, 1],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        true,
        vec![2, 0, 4_294_967_294],
    );
    test(
        &[0, 0, 0],
        &[1],
        true,
        vec![4_294_967_295, 4_294_967_295, 4_294_967_295],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_in_place_right_fail() {
    limbs_sub_in_place_right(&[6, 7], &mut vec![1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_same_length_with_borrow_in_to_out() {
    let test = |xs, ys, borrow_in, out_before: &[Limb], borrow, out_after| {
        let mut out = out_before.to_vec();
        assert_eq!(
            _limbs_sub_same_length_with_borrow_in_to_out(&mut out, xs, ys, borrow_in),
            borrow
        );
        assert_eq!(out, out_after);
    };
    test(&[], &[], false, &[0, 0], false, vec![0, 0]);
    test(&[], &[], true, &[0, 0], true, vec![0, 0]);
    test(&[3], &[2], false, &[0, 0], false, vec![1, 0]);
    test(&[3], &[2], true, &[0, 0], false, vec![0, 0]);
    test(&[2], &[3], false, &[0, 0], true, vec![4_294_967_295, 0]);
    test(&[3], &[3], true, &[0, 0], true, vec![4_294_967_295, 0]);
    test(&[2], &[3], true, &[0, 0], true, vec![4_294_967_294, 0]);
    test(
        &[1, 2, 3],
        &[1, 1, 1],
        false,
        &[0, 1, 2, 5],
        false,
        vec![0, 1, 2, 5],
    );
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        false,
        &[5, 5, 5, 5],
        true,
        vec![0, 4_294_967_295, 4_294_967_293, 5],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        &[10, 10, 10, 10],
        false,
        vec![4_294_967_294, 4_294_967_295, 1, 10],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        false,
        &[10, 10, 10, 10],
        true,
        vec![2, 0, 4_294_967_294, 10],
    );
    test(
        &[0, 0, 0],
        &[1, 0, 0],
        false,
        &[10, 10, 10, 10],
        true,
        vec![4_294_967_295, 4_294_967_295, 4_294_967_295, 10],
    );
    test(
        &[0, 0, 0],
        &[1, 0, 0],
        true,
        &[10, 10, 10, 10],
        true,
        vec![4_294_967_294, 4_294_967_295, 4_294_967_295, 10],
    );
    test(
        &[0, 0, 0],
        &[0, 0, 0],
        true,
        &[10, 10, 10, 10],
        true,
        vec![4_294_967_295, 4_294_967_295, 4_294_967_295, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_same_length_with_borrow_in_to_out_fail_1() {
    let mut out = vec![10, 10];
    _limbs_sub_same_length_with_borrow_in_to_out(&mut out, &[6, 7, 8], &[1, 2, 3], false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_same_length_with_borrow_in_to_out_fail_2() {
    let mut out = vec![10, 10, 10];
    _limbs_sub_same_length_with_borrow_in_to_out(&mut out, &[6, 7, 8], &[1, 2], false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_same_length_with_borrow_in_in_place_left() {
    let test = |xs_before: &[Limb], ys, borrow_in, borrow, xs_after| {
        let mut xs = xs_before.to_vec();
        assert_eq!(
            _limbs_sub_same_length_with_borrow_in_in_place_left(&mut xs, ys, borrow_in),
            borrow
        );
        assert_eq!(xs, xs_after);
    };
    test(&[], &[], false, false, vec![]);
    test(&[], &[], true, true, vec![]);
    test(&[3], &[2], false, false, vec![1]);
    test(&[3], &[2], true, false, vec![0]);
    test(&[2], &[3], false, true, vec![4_294_967_295]);
    test(&[2], &[2], true, true, vec![4_294_967_295]);
    test(&[2], &[3], true, true, vec![4_294_967_294]);
    test(&[1, 2, 3], &[1, 1, 1], false, false, vec![0, 1, 2]);
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        false,
        true,
        vec![0, 4_294_967_295, 4_294_967_293],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        false,
        vec![4_294_967_294, 4_294_967_295, 1],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        false,
        true,
        vec![2, 0, 4_294_967_294],
    );
    test(
        &[0, 0, 0],
        &[1, 0, 0],
        false,
        true,
        vec![4_294_967_295, 4_294_967_295, 4_294_967_295],
    );
    test(
        &[0, 0, 0],
        &[1, 0, 0],
        true,
        true,
        vec![4_294_967_294, 4_294_967_295, 4_294_967_295],
    );
    test(
        &[0, 0, 0],
        &[0, 0, 0],
        true,
        true,
        vec![4_294_967_295, 4_294_967_295, 4_294_967_295],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_same_length_with_borrow_in_in_place_left_fail() {
    _limbs_sub_same_length_with_borrow_in_in_place_left(&mut [6, 7], &[1, 2, 3], false);
}

#[test]
fn test_sub_natural() {
    let test = |u, v, out| {
        let mut n = Natural::from_str(u).unwrap();
        n -= Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n -= &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap() - Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap() - &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() - Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() - &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigUint::from_str(u).unwrap() - BigUint::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() - rug::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("123", "0", "123");
    test("456", "123", "333");
    test("1000000000000", "123", "999999999877");
    test("12345678987654321", "314159265358979", "12031519722295342");
    test("4294967296", "1", "4294967295");
    test("4294967295", "4294967295", "0");
    test("4294967296", "4294967295", "1");
    test("4294967296", "4294967296", "0");
    test("18446744073709551616", "1", "18446744073709551615");
    test("18446744073709551615", "18446744073709551615", "0");
    test("18446744073709551616", "18446744073709551615", "1");
    test("70734740290631708", "282942734368", "70734457347897340");
}

#[test]
#[should_panic]
fn sub_assign_fail() {
    let mut x = Natural::from_str("123").unwrap();
    x -= Natural::from_str("456").unwrap();
}

#[test]
#[should_panic]
fn sub_assign_ref_fail() {
    let mut x = Natural::from_str("123").unwrap();
    x -= &Natural::from_str("456").unwrap();
}

#[test]
#[should_panic]
#[allow(unused_must_use)]
fn sub_fail_1() {
    Natural::from(123u32) - Natural::from(456u32);
}

#[test]
#[should_panic]
#[allow(unused_must_use)]
fn sub_fail_2() {
    Natural::from(123u32) - &Natural::from(456u32);
}

#[test]
#[should_panic]
#[allow(unused_must_use)]
fn sub_fail_3() {
    &Natural::from(123u32) - Natural::from(456u32);
}

#[test]
#[should_panic(
    expected = "Cannot subtract a Natural from a smaller Natural. self: 123, other: 456"
)]
#[allow(unused_must_use)]
fn sub_fail_4() {
    &Natural::from(123u32) - &Natural::from(456u32);
}

#[test]
fn limbs_sub_properties() {
    test_properties(pairs_of_unsigned_vec_var_3, |&(ref xs, ref ys)| {
        let (limbs, borrow) = limbs_sub(xs, ys);
        let len = limbs.len() as u32;
        let n = Natural::from_owned_limbs_asc(limbs);
        if borrow {
            assert_eq!(
                n,
                Integer::from(Natural::from_limbs_asc(xs))
                    - Integer::from(Natural::from_limbs_asc(ys))
                    + (Integer::ONE << (Limb::WIDTH * len))
            );
        } else {
            assert_eq!(n, Natural::from_limbs_asc(xs) - Natural::from_limbs_asc(ys));
        }
    });
}

fn limbs_sub_to_out_helper(
    f: &mut FnMut(&mut [Limb], &[Limb], &[Limb]) -> bool,
    out: &Vec<Limb>,
    xs: &Vec<Limb>,
    ys: &Vec<Limb>,
) {
    let mut out = out.to_vec();
    let old_out = out.clone();
    let len = xs.len();
    let limbs = if f(&mut out, xs, ys) {
        let n = Integer::from(Natural::from_limbs_asc(xs))
            - Integer::from(Natural::from_limbs_asc(ys))
            + (Integer::ONE << (Limb::WIDTH * (len as u32)));
        let mut limbs = Natural::checked_from(n).unwrap().into_limbs_asc();
        limbs.resize(len, Limb::MAX);
        limbs
    } else {
        let n = Natural::from_limbs_asc(xs) - Natural::from_limbs_asc(ys);
        let mut limbs = n.into_limbs_asc();
        limbs.resize(len, 0);
        limbs
    };
    assert_eq!(limbs, &out[..len]);
    assert_eq!(&out[len..], &old_out[len..]);
}

#[test]
fn limbs_sub_same_length_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_var_3,
        |&(ref out, ref xs, ref ys)| {
            limbs_sub_to_out_helper(&mut limbs_sub_same_length_to_out, out, xs, ys);
        },
    );
}

#[test]
fn limbs_sub_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_var_9,
        |&(ref out, ref xs, ref ys)| {
            limbs_sub_to_out_helper(&mut limbs_sub_to_out, out, xs, ys);
        },
    );
}

fn limbs_sub_in_place_left_helper(
    f: &mut FnMut(&mut [Limb], &[Limb]) -> bool,
    xs: &Vec<Limb>,
    ys: &Vec<Limb>,
) {
    let mut xs = xs.to_vec();
    let xs_old = xs.clone();
    let len = xs.len() as u32;
    let borrow = f(&mut xs, ys);
    let n = Natural::from_owned_limbs_asc(xs);
    if borrow {
        assert_eq!(
            n,
            Integer::from(Natural::from_owned_limbs_asc(xs_old))
                - Integer::from(Natural::from_limbs_asc(ys))
                + (Integer::ONE << (Limb::WIDTH * len))
        );
    } else {
        assert_eq!(
            n,
            Natural::from_owned_limbs_asc(xs_old) - Natural::from_limbs_asc(ys)
        );
    }
}

#[test]
fn limbs_sub_same_length_in_place_left_properties() {
    test_properties(pairs_of_unsigned_vec_var_1, |&(ref xs, ref ys)| {
        limbs_sub_in_place_left_helper(&mut limbs_sub_same_length_in_place_left, xs, ys);
    });
}

#[test]
fn limbs_sub_in_place_left_properties() {
    test_properties(pairs_of_unsigned_vec_var_3, |&(ref xs, ref ys)| {
        limbs_sub_in_place_left_helper(&mut limbs_sub_in_place_left, xs, ys);
    });
}

macro_rules! limbs_sub_in_place_right_helper {
    ($f:ident, $xs:ident, $ys:ident) => {
        |&(ref $xs, ref $ys)| {
            let mut ys = $ys.to_vec();
            let ys_old = $ys.clone();
            let len = $xs.len() as u32;
            let borrow = $f($xs, &mut ys);
            let n = Natural::from_limbs_asc(&ys);
            if borrow {
                assert_eq!(
                    n,
                    Integer::from(Natural::from_limbs_asc($xs))
                        - Integer::from(Natural::from_owned_limbs_asc(ys_old))
                        + (Integer::ONE << (Limb::WIDTH * len))
                );
            } else {
                assert_eq!(
                    n,
                    Natural::from_limbs_asc($xs) - Natural::from_owned_limbs_asc(ys_old)
                );
            }
        }
    };
}

#[test]
fn limbs_sub_same_length_in_place_right_properties() {
    test_properties(
        pairs_of_unsigned_vec_var_1,
        limbs_sub_in_place_right_helper!(limbs_sub_same_length_in_place_right, xs, ys),
    );
}

#[test]
fn limbs_sub_in_place_right_properties() {
    test_properties(
        pairs_of_unsigned_vec_var_3,
        limbs_sub_in_place_right_helper!(limbs_sub_in_place_right, xs, ys),
    );
}

#[test]
fn limbs_sub_same_length_with_borrow_in_to_out_properties() {
    test_properties(
        quadruples_of_three_unsigned_vecs_and_bool_var_1,
        |&(ref out, ref xs, ref ys, borrow_in)| {
            let mut out = out.to_vec();
            let old_out = out.clone();
            let len = xs.len();
            let limbs = if _limbs_sub_same_length_with_borrow_in_to_out(&mut out, xs, ys, borrow_in)
            {
                let mut n = Integer::from(Natural::from_limbs_asc(xs))
                    - Integer::from(Natural::from_limbs_asc(ys))
                    + (Integer::ONE << (Limb::WIDTH * (len as u32)));
                if borrow_in {
                    n -= 1 as Limb;
                }
                let mut limbs = Natural::checked_from(n).unwrap().into_limbs_asc();
                limbs.resize(len, Limb::MAX);
                limbs
            } else {
                let mut n = Natural::from_limbs_asc(xs) - Natural::from_limbs_asc(ys);
                if borrow_in {
                    n -= 1 as Limb;
                }
                let mut limbs = n.into_limbs_asc();
                limbs.resize(len, 0);
                limbs
            };
            assert_eq!(limbs, &out[..len]);
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_sub_same_length_with_borrow_in_in_place_left_properties() {
    test_properties(
        triples_of_two_unsigned_vecs_and_bool_var_1,
        |&(ref xs, ref ys, borrow_in)| {
            let mut xs = xs.to_vec();
            let xs_old = xs.clone();
            let len = xs.len() as u32;
            let borrow =
                _limbs_sub_same_length_with_borrow_in_in_place_left(&mut xs, ys, borrow_in);
            let n = Natural::from_owned_limbs_asc(xs);
            let mut expected_result = if borrow {
                Natural::checked_from(
                    Integer::from(Natural::from_owned_limbs_asc(xs_old))
                        - Integer::from(Natural::from_limbs_asc(ys))
                        + (Integer::ONE << (Limb::WIDTH * len)),
                )
                .unwrap()
            } else {
                Natural::from_owned_limbs_asc(xs_old) - Natural::from_limbs_asc(ys)
            };
            if borrow_in {
                expected_result -= 1;
            }
            assert_eq!(n, expected_result);
        },
    );
}

#[test]
fn sub_properties() {
    test_properties(pairs_of_naturals_var_1, |&(ref x, ref y)| {
        let mut mut_x = x.clone();
        mut_x -= y.clone();
        assert!(mut_x.is_valid());
        let difference = mut_x;

        let mut mut_x = x.clone();
        mut_x -= y;
        assert!(mut_x.is_valid());
        let difference_alt = mut_x;
        assert_eq!(difference_alt, difference);

        let mut rug_x = natural_to_rug_integer(x);
        rug_x -= natural_to_rug_integer(y);
        assert_eq!(rug_integer_to_natural(&rug_x), difference);

        let difference_alt = x.clone() - y.clone();
        assert!(difference_alt.is_valid());
        assert_eq!(difference_alt, difference);

        let difference_alt = x.clone() - y;
        assert_eq!(difference_alt, difference);
        assert!(difference_alt.is_valid());

        let difference_alt = x - y.clone();
        assert_eq!(difference_alt, difference);
        assert!(difference_alt.is_valid());

        let difference_alt = x - y;
        assert_eq!(difference_alt, difference);
        assert!(difference_alt.is_valid());

        assert_eq!(
            biguint_to_natural(&(natural_to_biguint(x) - natural_to_biguint(y))),
            difference
        );
        assert_eq!(
            rug_integer_to_natural(&(natural_to_rug_integer(x) - natural_to_rug_integer(y))),
            difference
        );

        assert!(difference <= *x);
        assert_eq!(difference + y, *x);
    });

    test_properties(pairs_of_natural_and_limb_var_1, |&(ref x, y)| {
        assert_eq!(x - y, x - Natural::from(y));
    });

    test_properties(pairs_of_limb_and_natural_var_1, |&(x, ref y)| {
        assert_eq!(x - y, Natural::from(x) - y);
    });

    test_properties(pairs_of_unsigneds_var_1::<Limb>, |&(x, y)| {
        assert_eq!(Natural::from(x - y), Natural::from(x) - Natural::from(y));
    });

    #[allow(unknown_lints, identity_op, eq_op)]
    test_properties(naturals, |x| {
        assert_eq!(x - Natural::ZERO, *x);
        assert_eq!(x - x, Natural::ZERO);
    });
}
