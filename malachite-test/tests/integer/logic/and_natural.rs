use common::test_properties;
use malachite_base::conversion::CheckedFrom;
use malachite_base::num::traits::Zero;
use malachite_nz::integer::logic::and_natural::{
    limbs_and_pos_neg, limbs_and_pos_neg_in_place_left, limbs_and_pos_neg_to_out,
    limbs_slice_and_pos_neg_in_place_right, limbs_vec_and_pos_neg_in_place_right,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_test::common::{
    integer_to_rug_integer, natural_to_rug_integer, rug_integer_to_integer,
};
use malachite_test::inputs::base::{pairs_of_limb_vec_var_1, triples_of_limb_vec_var_5};
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_natural, pairs_of_integer_and_unsigned,
};
use malachite_test::inputs::natural::{naturals, pairs_of_naturals};
use rug;
use std::cmp::min;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_and_pos_neg() {
    let test = |xs, ys, out| {
        assert_eq!(limbs_and_pos_neg(xs, ys), out);
    };
    test(&[2], &[3], vec![0]);
    test(&[1, 1, 1], &[1, 2, 3], vec![1, 1, 0]);
    test(&[6, 7], &[1, 2, 3], vec![6, 5]);
    test(&[1, 2, 3], &[6, 7], vec![0, 0, 3]);
    test(&[100, 101, 102], &[102, 101, 100], vec![0, 0, 2]);
    test(&[0, 0, 1], &[3], vec![0, 0, 1]);
    test(&[3], &[0, 0, 1], vec![]);
    test(&[0, 3, 3], &[0, 0, 3], vec![0, 0, 1]);
    test(&[0, 0, 3], &[0, 3, 3], vec![0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_and_pos_neg_fail_1() {
    limbs_and_pos_neg(&[0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_and_pos_neg_fail_2() {
    limbs_and_pos_neg(&[3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_and_pos_neg_to_out() {
    let test = |xs, ys, out_before: &[Limb], out_after| {
        let mut out = out_before.to_vec();
        limbs_and_pos_neg_to_out(&mut out, xs, ys);
        assert_eq!(out, out_after);
    };
    test(&[2], &[3], &[10, 10, 10, 10], vec![0, 10, 10, 10]);
    test(&[1, 1, 1], &[1, 2, 3], &[10, 10, 10, 10], vec![1, 1, 0, 10]);
    test(&[6, 7], &[1, 2, 3], &[10, 10, 10, 10], vec![6, 5, 10, 10]);
    test(&[1, 2, 3], &[6, 7], &[10, 10, 10, 10], vec![0, 0, 3, 10]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        vec![0, 0, 2, 10],
    );
    test(&[0, 0, 1], &[3], &[10, 10, 10, 10], vec![0, 0, 1, 10]);
    test(&[3], &[0, 0, 1], &[10, 10, 10, 10], vec![0, 10, 10, 10]);
    test(&[0, 3, 3], &[0, 0, 3], &[10, 10, 10, 10], vec![0, 0, 1, 10]);
    test(&[0, 0, 3], &[0, 3, 3], &[10, 10, 10, 10], vec![0, 0, 0, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_and_pos_neg_to_out_fail_1() {
    let mut out = vec![10, 10, 10, 10];
    limbs_and_pos_neg_to_out(&mut out, &[0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_and_pos_neg_to_out_fail_2() {
    let mut out = vec![10, 10, 10, 10];
    limbs_and_pos_neg_to_out(&mut out, &[3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_and_pos_neg_to_out_fail_3() {
    let mut out = vec![10];
    limbs_and_pos_neg_to_out(&mut out, &[6, 7], &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_pos_neg_in_place_left_and_limbs_vec_and_pos_neg_in_place_right() {
    let test = |xs: &[Limb], ys, out| {
        {
            let mut mut_xs = xs.to_vec();
            limbs_and_pos_neg_in_place_left(&mut mut_xs, ys);
            assert_eq!(mut_xs, out);
        }
        {
            let mut mut_ys = ys.to_vec();
            limbs_vec_and_pos_neg_in_place_right(xs, &mut mut_ys);
            assert_eq!(mut_ys, out);
        }
    };
    test(&[2], &[3], vec![0]);
    test(&[1, 1, 1], &[1, 2, 3], vec![1, 1, 0]);
    test(&[6, 7], &[1, 2, 3], vec![6, 5]);
    test(&[1, 2, 3], &[6, 7], vec![0, 0, 3]);
    test(&[100, 101, 102], &[102, 101, 100], vec![0, 0, 2]);
    test(&[0, 0, 1], &[3], vec![0, 0, 1]);
    test(&[3], &[0, 0, 1], vec![0]);
    test(&[0, 3, 3], &[0, 0, 3], vec![0, 0, 1]);
    test(&[0, 0, 3], &[0, 3, 3], vec![0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_and_pos_neg_in_place_left_fail_1() {
    limbs_and_pos_neg_in_place_left(&mut [0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_and_pos_neg_in_place_left_fail_2() {
    limbs_and_pos_neg_in_place_left(&mut [3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_vec_and_pos_neg_in_place_right_fail_1() {
    let mut ys = vec![3];
    limbs_vec_and_pos_neg_in_place_right(&[0, 0, 0], &mut ys);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_vec_and_pos_neg_in_place_right_fail_2() {
    let mut ys = vec![0, 0, 0];
    limbs_vec_and_pos_neg_in_place_right(&[3], &mut ys);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_and_pos_neg_in_place_right() {
    let test = |xs, ys_before: &[Limb], ys_after| {
        let mut ys = ys_before.to_vec();
        limbs_slice_and_pos_neg_in_place_right(xs, &mut ys);
        assert_eq!(ys, ys_after);
    };
    test(&[2], &[3], vec![0]);
    test(&[1, 1, 1], &[1, 2, 3], vec![1, 1, 0]);
    test(&[6, 7], &[1, 2, 3], vec![6, 5, 3]);
    test(&[1, 2, 3], &[6, 7], vec![0, 0]);
    test(&[100, 101, 102], &[102, 101, 100], vec![0, 0, 2]);
    test(&[0, 0, 1], &[3], vec![0]);
    test(&[3], &[0, 0, 1], vec![0, 0, 0]);
    test(&[0, 3, 3], &[0, 0, 3], vec![0, 0, 1]);
    test(&[0, 0, 3], &[0, 3, 3], vec![0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_and_pos_neg_in_place_right_fail_1() {
    limbs_slice_and_pos_neg_in_place_right(&[0, 0, 0], &mut [3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_and_pos_neg_in_place_right_fail_2() {
    limbs_slice_and_pos_neg_in_place_right(&[3], &mut [0, 0, 0]);
}

#[test]
fn test_and_natural() {
    let test = |u, v, out| {
        let mut n = Integer::from_str(u).unwrap();
        n &= Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Integer::from_str(u).unwrap();
        n &= &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap() & Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() & Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap() & &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() & &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(v).unwrap();
        n &= Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(v).unwrap();
        n &= &Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(v).unwrap() & Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(v).unwrap() & Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(v).unwrap() & &Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(v).unwrap() & &Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = rug::Integer::from_str(u).unwrap() & rug::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "0");
    test("123", "0", "0");
    test("123", "456", "72");
    test("1000000000000", "123", "0");
    test("123", "1000000000000", "0");
    test("1000000000000", "999999999999", "999999995904");
    test("12345678987654321", "314159265358979", "312331665941633");
    test("-123", "0", "0");
    test("-123", "456", "384");
    test("-1000000000000", "123", "0");
    test("-123", "1000000000000", "1000000000000");
    test("-1000000000000", "999999999999", "0");
    test("-12345678987654321", "314159265358979", "1827599417347");
    test(
        "-478178031043645514337313657924474082957368",
        "3332140978726732268209104861552",
        "2539024739207132029580719268160",
    );
}

#[test]
fn limbs_and_pos_neg_properties() {
    test_properties(pairs_of_limb_vec_var_1, |&(ref xs, ref ys)| {
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_and_pos_neg(xs, ys)),
            Integer::from(Natural::from_limbs_asc(xs)) & -Natural::from_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_and_pos_neg_to_out_properties() {
    test_properties(triples_of_limb_vec_var_5, |&(ref out, ref xs, ref ys)| {
        let mut out = out.to_vec();
        let out_old = out.clone();
        limbs_and_pos_neg_to_out(&mut out, xs, ys);
        let len = xs.len();
        assert_eq!(
            Natural::from_limbs_asc(&out[..len]),
            Integer::from(Natural::from_limbs_asc(xs)) & -Natural::from_limbs_asc(ys)
        );
        assert_eq!(&out[len..], &out_old[len..]);
    });
}

#[test]
fn limbs_and_pos_neg_in_place_left_properties() {
    test_properties(pairs_of_limb_vec_var_1, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_and_pos_neg_in_place_left(&mut xs, ys);
        assert_eq!(
            Natural::from_owned_limbs_asc(xs),
            Integer::from(Natural::from_owned_limbs_asc(xs_old)) & -Natural::from_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_slice_and_pos_neg_in_place_right_properties() {
    test_properties(pairs_of_limb_vec_var_1, |&(ref xs, ref ys)| {
        let mut ys = ys.to_vec();
        let ys_old = ys.clone();
        limbs_slice_and_pos_neg_in_place_right(xs, &mut ys);
        let result =
            Integer::from(Natural::from_limbs_asc(xs)) & -Natural::from_owned_limbs_asc(ys_old);
        let mut expected_limbs = Natural::checked_from(result).unwrap().into_limbs_asc();
        let len = min(xs.len(), ys.len());
        expected_limbs.resize(len, 0);
        assert_eq!(&ys[..len], expected_limbs.as_slice());
    });
}

#[test]
fn limbs_vec_and_pos_neg_in_place_right_properties() {
    test_properties(pairs_of_limb_vec_var_1, |&(ref xs, ref ys)| {
        let mut ys = ys.to_vec();
        let ys_old = ys.clone();
        limbs_vec_and_pos_neg_in_place_right(xs, &mut ys);
        let result =
            Integer::from(Natural::from_limbs_asc(xs)) & -Natural::from_owned_limbs_asc(ys_old);
        let mut expected_limbs = Natural::checked_from(result).unwrap().into_limbs_asc();
        expected_limbs.resize(xs.len(), 0);
        ys.resize(xs.len(), 0);
        assert_eq!(ys, expected_limbs);
    });
}

#[test]
fn and_natural_properties() {
    test_properties(pairs_of_integer_and_natural, |&(ref x, ref y)| {
        let result_val_val = x.clone() & y.clone();
        let result_val_ref = x.clone() & y;
        let result_ref_val = x & y.clone();
        let result = x & y;
        assert!(result_val_val.is_valid());
        assert!(result_val_ref.is_valid());
        assert!(result_ref_val.is_valid());
        assert!(result.is_valid());
        assert_eq!(result_val_val, result);
        assert_eq!(result_val_ref, result);
        assert_eq!(result_ref_val, result);

        let result_val_val = y.clone() & x.clone();
        let result_val_ref = y.clone() & x;
        let result_ref_val = y & x.clone();
        let result_alt = y & x;
        assert!(result_val_val.is_valid());
        assert!(result_val_ref.is_valid());
        assert!(result_ref_val.is_valid());
        assert!(result_alt.is_valid());
        assert_eq!(result_val_val, result);
        assert_eq!(result_val_ref, result);
        assert_eq!(result_ref_val, result);
        assert_eq!(result_alt, result);

        let mut mut_x = x.clone();
        mut_x &= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, result);
        let mut mut_x = x.clone();
        mut_x &= y;
        assert_eq!(mut_x, result);
        assert!(mut_x.is_valid());

        let mut mut_y = y.clone();
        mut_y &= x.clone();
        assert!(mut_y.is_valid());
        assert_eq!(mut_y, result);
        let mut mut_y = y.clone();
        mut_y &= x;
        assert_eq!(mut_y, result);
        assert!(mut_y.is_valid());

        let mut mut_x = integer_to_rug_integer(x);
        mut_x &= natural_to_rug_integer(y);
        assert_eq!(rug_integer_to_integer(&mut_x), result);

        assert_eq!(
            rug_integer_to_integer(&(integer_to_rug_integer(x) & natural_to_rug_integer(y))),
            result
        );

        assert_eq!(&result & x, result);
        assert_eq!(&result & y, result);
        assert_eq!(!(!x | !y), result);
    });

    test_properties(
        pairs_of_integer_and_unsigned,
        |&(ref x, y): &(Integer, Limb)| {
            let result = x & Natural::from(y);
            assert_eq!(x & y, result);
            assert_eq!(y & x, result);
        },
    );

    test_properties(integers, |x| {
        assert_eq!(x & Natural::ZERO, 0 as Limb);
        assert_eq!(Natural::ZERO & x, 0 as Limb);
        assert_eq!(x & x, *x);
    });

    test_properties(naturals, |x| {
        assert_eq!(x & Integer::ZERO, 0 as Limb);
        assert_eq!(Integer::ZERO & x, 0 as Limb);
        assert_eq!(x & Integer::from(x), *x);
        assert_eq!(Integer::from(x) & x, *x);
        assert_eq!(x & !x, Integer::ZERO);
    });

    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        assert_eq!(Integer::from(x) & y, x & y);
        assert_eq!(x & Integer::from(y), x & y);
    });
}
