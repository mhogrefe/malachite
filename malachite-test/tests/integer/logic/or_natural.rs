use common::test_properties;
use malachite_base::conversion::CheckedFrom;
use malachite_base::num::traits::{NegativeOne, Zero};
use malachite_nz::integer::logic::or_natural::{
    limbs_or_pos_neg, limbs_or_pos_neg_in_place_right, limbs_or_pos_neg_to_out,
    limbs_slice_or_pos_neg_in_place_left, limbs_vec_or_pos_neg_in_place_left,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_test::common::{
    integer_to_rug_integer, natural_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::base::{pairs_of_limb_vec_var_1, triples_of_limb_vec_var_6};
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_natural, pairs_of_integer_and_unsigned,
};
use malachite_test::inputs::natural::{naturals, pairs_of_naturals};
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_or_pos_neg_limbs_vec_or_pos_neg_in_place_left_and_limbs_or_pos_neg_in_place_right() {
    let test = |xs_before, ys_before, out| {
        assert_eq!(limbs_or_pos_neg(xs_before, ys_before), out);

        let mut xs = xs_before.to_vec();
        limbs_vec_or_pos_neg_in_place_left(&mut xs, ys_before);
        assert_eq!(xs, out);

        let mut ys = ys_before.to_vec();
        limbs_or_pos_neg_in_place_right(xs_before, &mut ys);
        assert_eq!(ys, out);
    };
    test(&[2], &[3], vec![1]);
    test(&[1, 1, 1], &[1, 2, 3], vec![1, 2, 2]);
    test(&[6, 7], &[1, 2, 3], vec![1, 0, 3]);
    test(&[1, 2, 3], &[6, 7], vec![5, 5]);
    test(&[100, 101, 102], &[102, 101, 100], vec![2, 0, 0]);
    test(&[0, 0, 1], &[3], vec![3]);
    test(&[3], &[0, 0, 1], vec![4_294_967_293, 4_294_967_295, 0]);
    test(&[0, 3, 3], &[0, 0, 3], vec![0, 4_294_967_293, 0]);
    test(&[0, 0, 3], &[0, 3, 3], vec![0, 3, 0]);
    test(&[0, 3], &[0, 0, 3], vec![0, 4_294_967_293, 2]);
    test(&[0, 0, 3], &[0, 3], vec![0, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_or_pos_neg_fail_1() {
    limbs_or_pos_neg(&[0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_or_pos_neg_fail_2() {
    limbs_or_pos_neg(&[3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_or_pos_neg_to_out() {
    let test = |xs, ys, out_before: &[Limb], out_after| {
        let mut out = out_before.to_vec();
        limbs_or_pos_neg_to_out(&mut out, xs, ys);
        assert_eq!(out, out_after);
    };
    test(&[2], &[3], &[10, 10, 10, 10], vec![1, 10, 10, 10]);
    test(&[1, 1, 1], &[1, 2, 3], &[10, 10, 10, 10], vec![1, 2, 2, 10]);
    test(&[6, 7], &[1, 2, 3], &[10, 10, 10, 10], vec![1, 0, 3, 10]);
    test(&[1, 2, 3], &[6, 7], &[10, 10, 10, 10], vec![5, 5, 10, 10]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        vec![2, 0, 0, 10],
    );
    test(&[0, 0, 1], &[3], &[10, 10, 10, 10], vec![3, 10, 10, 10]);
    test(
        &[3],
        &[0, 0, 1],
        &[10, 10, 10, 10],
        vec![4_294_967_293, 4_294_967_295, 0, 10],
    );
    test(
        &[0, 3, 3],
        &[0, 0, 3],
        &[10, 10, 10, 10],
        vec![0, 4_294_967_293, 0, 10],
    );
    test(&[0, 0, 3], &[0, 3, 3], &[10, 10, 10, 10], vec![0, 3, 0, 10]);
    test(
        &[0, 3],
        &[0, 0, 3],
        &[10, 10, 10, 10],
        vec![0, 4_294_967_293, 2, 10],
    );
    test(&[0, 0, 3], &[0, 3], &[10, 10, 10, 10], vec![0, 3, 10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_or_pos_neg_to_out_fail_1() {
    let mut out = vec![10, 10, 10, 10];
    limbs_or_pos_neg_to_out(&mut out, &[0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_or_pos_neg_to_out_fail_2() {
    let mut out = vec![10, 10, 10, 10];
    limbs_or_pos_neg_to_out(&mut out, &[3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_or_pos_neg_to_out_fail_3() {
    let mut out = vec![10];
    limbs_or_pos_neg_to_out(&mut out, &[6, 7], &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_or_pos_neg_in_place_left() {
    let test = |xs_before: &[Limb], ys, b, xs_after| {
        let mut xs = xs_before.to_vec();
        assert_eq!(limbs_slice_or_pos_neg_in_place_left(&mut xs, ys), b);
        assert_eq!(xs, xs_after);
    };
    test(&[2], &[3], false, vec![1]);
    test(&[1, 1, 1], &[1, 2, 3], false, vec![1, 2, 2]);
    test(&[6, 7], &[1, 2, 3], true, vec![1, 0]);
    test(&[1, 2, 3], &[6, 7], false, vec![5, 5, 0]);
    test(&[100, 101, 102], &[102, 101, 100], false, vec![2, 0, 0]);
    test(&[0, 0, 1], &[3], false, vec![3, 0, 0]);
    test(&[3], &[0, 0, 1], true, vec![4_294_967_293]);
    test(&[0, 3, 3], &[0, 0, 3], false, vec![0, 4_294_967_293, 0]);
    test(&[0, 0, 3], &[0, 3, 3], false, vec![0, 3, 0]);
    test(&[0, 3], &[0, 0, 3], true, vec![0, 4_294_967_293]);
    test(&[0, 0, 3], &[0, 3], false, vec![0, 3, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_or_pos_neg_in_place_left_fail_1() {
    limbs_slice_or_pos_neg_in_place_left(&mut [0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_or_pos_neg_in_place_left_fail_2() {
    limbs_slice_or_pos_neg_in_place_left(&mut [3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_vec_or_pos_neg_in_place_left_fail_1() {
    limbs_vec_or_pos_neg_in_place_left(&mut vec![0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_vec_or_pos_neg_in_place_left_fail_2() {
    limbs_vec_or_pos_neg_in_place_left(&mut vec![3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_or_pos_neg_in_place_right_fail_1() {
    limbs_or_pos_neg_in_place_right(&[0, 0, 0], &mut [3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_or_pos_neg_in_place_right_fail_2() {
    limbs_or_pos_neg_in_place_right(&[3], &mut [0, 0, 0]);
}

#[test]
fn test_or_natural() {
    let test = |u, v, out| {
        let mut n = Integer::from_str(u).unwrap();
        n |= Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Integer::from_str(u).unwrap();
        n |= &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap() | Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() | Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap() | &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() | &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = rug::Integer::from_str(u).unwrap() | rug::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "123");
    test("123", "0", "123");
    test("123", "456", "507");
    test("1000000000000", "123", "1000000000123");
    test("123", "1000000000000", "1000000000123");
    test("1000000000000", "999999999999", "1000000004095");
    test("12345678987654321", "314159265358979", "12347506587071667");
    test("-123", "0", "-123");
    test("-123", "456", "-51");
    test("-1000000000000", "123", "-999999999877");
    test("-123", "1000000000000", "-123");
    test("-1000000000000", "999999999999", "-1");
    test(
        "-12345678987654321",
        "314159265358979",
        "-12033347321712689",
    );

    test(
        "-324518553658389833295008601473024",
        "317057721155483154675232931839",
        "-324201495937234350140333368541185",
    );
    test(
        "-26298808336",
        "170141183460469156173823577801560686592",
        "-26298808336",
    );
}

#[test]
fn limbs_or_pos_neg_properties() {
    test_properties(pairs_of_limb_vec_var_1, |&(ref xs, ref ys)| {
        assert_eq!(
            -Natural::from_owned_limbs_asc(limbs_or_pos_neg(xs, ys)),
            Integer::from(Natural::from_limbs_asc(xs)) | -Natural::from_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_or_pos_neg_to_out_properties() {
    test_properties(triples_of_limb_vec_var_6, |&(ref out, ref xs, ref ys)| {
        let mut out = out.to_vec();
        let out_old = out.clone();
        limbs_or_pos_neg_to_out(&mut out, xs, ys);
        let len = ys.len();
        assert_eq!(
            -Natural::from_limbs_asc(&out[..len]),
            Integer::from(Natural::from_limbs_asc(xs)) | -Natural::from_limbs_asc(ys)
        );
        assert_eq!(&out[len..], &out_old[len..]);
    });
}

#[test]
fn limbs_slice_or_pos_neg_in_place_left_properties() {
    test_properties(pairs_of_limb_vec_var_1, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        if limbs_slice_or_pos_neg_in_place_left(&mut xs, ys) {
            let mut result_limbs = Natural::checked_from(
                -(Integer::from(Natural::from_owned_limbs_asc(xs_old))
                    | -Natural::from_limbs_asc(ys)),
            )
            .unwrap()
            .to_limbs_asc();
            result_limbs.resize(xs.len(), 0);
            assert_eq!(result_limbs, xs);
        } else {
            assert_eq!(
                -Natural::from_owned_limbs_asc(xs),
                Integer::from(Natural::from_owned_limbs_asc(xs_old)) | -Natural::from_limbs_asc(ys)
            );
        }
    });
}

#[test]
fn limbs_vec_or_pos_neg_in_place_left_properties() {
    test_properties(pairs_of_limb_vec_var_1, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_vec_or_pos_neg_in_place_left(&mut xs, ys);
        assert_eq!(
            -Natural::from_owned_limbs_asc(xs),
            Integer::from(Natural::from_owned_limbs_asc(xs_old)) | -Natural::from_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_or_pos_neg_in_place_right_properties() {
    test_properties(pairs_of_limb_vec_var_1, |&(ref xs, ref ys)| {
        let mut ys = ys.to_vec();
        let ys_old = ys.clone();
        limbs_or_pos_neg_in_place_right(xs, &mut ys);
        assert_eq!(
            -Natural::from_owned_limbs_asc(ys),
            Integer::from(Natural::from_limbs_asc(xs)) | -Natural::from_owned_limbs_asc(ys_old)
        );
    });
}

#[test]
fn or_natural_properties() {
    test_properties(pairs_of_integer_and_natural, |&(ref x, ref y)| {
        let result_val_val = x.clone() | y.clone();
        let result_val_ref = x.clone() | y;
        let result_ref_val = x | y.clone();
        let result = x | y;
        assert!(result_val_val.is_valid());
        assert!(result_val_ref.is_valid());
        assert!(result_ref_val.is_valid());
        assert!(result.is_valid());
        assert_eq!(result_val_val, result);
        assert_eq!(result_val_ref, result);
        assert_eq!(result_ref_val, result);

        let mut mut_x = x.clone();
        mut_x |= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, result);
        let mut mut_x = x.clone();
        mut_x |= y;
        assert_eq!(mut_x, result);
        assert!(mut_x.is_valid());

        let mut mut_x = integer_to_rug_integer(x);
        mut_x |= natural_to_rug_integer(y);
        assert_eq!(rug_integer_to_integer(&mut_x), result);

        assert_eq!(
            rug_integer_to_integer(&(integer_to_rug_integer(x) | natural_to_rug_integer(y))),
            result
        );

        assert_eq!(y | x, result);
        assert_eq!(&result | x, result);
        assert_eq!(&result | y, result);
        assert_eq!(!(!x & !y), result);
    });

    test_properties(
        pairs_of_integer_and_unsigned,
        |&(ref x, y): &(Integer, Limb)| {
            let result = x | Natural::from(y);
            assert_eq!(x | y, result);
            assert_eq!(y | x, result);
        },
    );

    test_properties(integers, |x| {
        assert_eq!(x | Natural::ZERO, *x);
        assert_eq!(Natural::ZERO | x, *x);
    });

    test_properties(naturals, |x| {
        assert_eq!(x | Integer::ZERO, *x);
        assert_eq!(Integer::ZERO | x, *x);
        assert_eq!(x | Integer::NEGATIVE_ONE, -1 as SignedLimb);
        assert_eq!(Integer::NEGATIVE_ONE | x, -1 as SignedLimb);
        assert_eq!(x | Integer::from(x), *x);
        assert_eq!(Integer::from(x) | x, *x);
        assert_eq!(x | !x, -1 as SignedLimb);
    });

    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        assert_eq!(Integer::from(x) | y, x | y);
        assert_eq!(x | Integer::from(y), x | y);
    });
}
