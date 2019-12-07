use std::cmp::min;
use std::str::FromStr;

use malachite_base::comparison::Max;
use malachite_base::num::basic::traits::{NegativeOne, Zero};
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_nz::integer::logic::or::{
    limbs_neg_or_limb, limbs_neg_or_limb_in_place, limbs_neg_or_limb_to_out, limbs_neg_or_neg_limb,
    limbs_or_neg_neg, limbs_or_neg_neg_in_place_either, limbs_or_neg_neg_to_out,
    limbs_pos_or_neg_limb, limbs_slice_or_neg_neg_in_place_left,
    limbs_vec_or_neg_neg_in_place_left,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};
use rug;

use common::test_properties;
use malachite_test::common::{integer_to_rug_integer, rug_integer_to_integer};
use malachite_test::inputs::base::{
    pairs_of_limb_vec_and_limb_var_1, pairs_of_limb_vec_and_positive_limb_var_1, pairs_of_signeds,
    pairs_of_unsigned_vec_var_6, triples_of_limb_vec_var_8,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_3,
};
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_natural, pairs_of_integers, triples_of_integers,
};
use malachite_test::inputs::natural::pairs_of_naturals;
use malachite_test::integer::logic::or::{integer_or_alt_1, integer_or_alt_2};

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_neg_or_limb_and_limbs_neg_or_limb_in_place() {
    let test = |limbs: &[Limb], limb: Limb, out: &[Limb]| {
        assert_eq!(limbs_neg_or_limb(limbs, limb), out);

        let mut limbs = limbs.to_vec();
        limbs_neg_or_limb_in_place(&mut limbs, limb);
        assert_eq!(limbs, out);
    };
    test(&[6, 7], 0, &[6, 7]);
    test(&[6, 7], 2, &[6, 7]);
    test(&[100, 101, 102], 10, &[98, 101, 102]);
    test(&[123, 456], 789, &[107, 456]);
    test(&[0, 0, 456], 789, &[0xffff_fceb, 0xffff_ffff, 455]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_neg_or_limb_in_place_fail() {
    limbs_neg_or_limb_in_place(&mut [0, 0, 0], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_neg_or_limb_fail() {
    limbs_neg_or_limb(&[0, 0, 0], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_neg_or_limb_to_out() {
    let test = |out_before: &[Limb], limbs_in: &[Limb], limb: Limb, out_after: &[Limb]| {
        let mut out = out_before.to_vec();
        limbs_neg_or_limb_to_out(&mut out, limbs_in, limb);
        assert_eq!(out, out_after);
    };
    test(&[10, 10, 10, 10], &[6, 7], 0, &[6, 7, 10, 10]);
    test(&[10, 10, 10, 10], &[6, 7], 2, &[6, 7, 10, 10]);
    test(&[10, 10, 10, 10], &[100, 101, 102], 10, &[98, 101, 102, 10]);
    test(&[10, 10, 10, 10], &[123, 456], 789, &[107, 456, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[0, 0, 456],
        789,
        &[0xffff_fceb, 0xffff_ffff, 455, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_neg_or_limb_to_out_fail_1() {
    limbs_neg_or_limb_to_out(&mut [10, 10], &[0, 0], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_neg_or_limb_to_out_fail_2() {
    limbs_neg_or_limb_to_out(&mut [10], &[10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_pos_or_neg_limb() {
    let test = |limbs: &[Limb], u: Limb, out: Limb| {
        assert_eq!(limbs_pos_or_neg_limb(limbs, u), out);
    };
    test(&[6, 7], 3, 0xffff_fff9);
    test(&[100, 101, 102], 10, 0xffff_ff92);
    test(&[0, 0, 1], 100, 0xffff_ff9c);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_or_neg_limb_fail() {
    limbs_pos_or_neg_limb(&[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_neg_or_neg_limb() {
    let test = |limbs: &[Limb], u: Limb, out: Limb| {
        assert_eq!(limbs_neg_or_neg_limb(limbs, u), out);
    };
    test(&[6, 7], 3, 5);
    test(&[100, 101, 102], 10, 98);
    test(&[0, 0, 1], 100, 0xffff_ff9c);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_neg_or_neg_limb_fail() {
    limbs_neg_or_neg_limb(&[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_or_neg_neg_and_limbs_vec_or_neg_neg_in_place_left() {
    let test = |xs_before, ys, out| {
        assert_eq!(limbs_or_neg_neg(xs_before, ys), out);

        let mut xs = xs_before.to_vec();
        limbs_vec_or_neg_neg_in_place_left(&mut xs, ys);
        assert_eq!(xs, out);
    };
    test(&[2], &[3], vec![1]);
    test(&[1, 1, 1], &[1, 2, 3], vec![1, 0, 1]);
    test(&[6, 7], &[1, 2, 3], vec![1, 2]);
    test(&[1, 2, 3], &[6, 7], vec![1, 2]);
    test(&[100, 101, 102], &[102, 101, 100], vec![98, 101, 100]);
    test(&[0, 0, 1], &[3], vec![3]);
    test(&[3], &[0, 0, 1], vec![3]);
    test(&[0, 3, 3], &[0, 0, 3], vec![0, 3, 2]);
    test(&[0, 0, 3], &[0, 3, 3], vec![0, 3, 2]);
    test(&[0, 3], &[0, 0, 3], vec![0, 3]);
    test(&[0, 0, 3], &[0, 3], vec![0, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_or_neg_neg_fail_1() {
    limbs_or_neg_neg(&[0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_or_neg_neg_fail_2() {
    limbs_or_neg_neg(&[3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_or_neg_neg_to_out() {
    let test = |xs, ys, out_before: &[Limb], out_after| {
        let mut out = out_before.to_vec();
        limbs_or_neg_neg_to_out(&mut out, xs, ys);
        assert_eq!(out, out_after);
    };
    test(&[2], &[3], &[10, 10, 10, 10], vec![1, 10, 10, 10]);
    test(&[1, 1, 1], &[1, 2, 3], &[10, 10, 10, 10], vec![1, 0, 1, 10]);
    test(&[6, 7], &[1, 2, 3], &[10, 10, 10, 10], vec![1, 2, 10, 10]);
    test(&[1, 2, 3], &[6, 7], &[10, 10, 10, 10], vec![1, 2, 10, 10]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        vec![98, 101, 100, 10],
    );
    test(&[0, 0, 1], &[3], &[10, 10, 10, 10], vec![3, 10, 10, 10]);
    test(&[3], &[0, 0, 1], &[10, 10, 10, 10], vec![3, 10, 10, 10]);
    test(&[0, 3, 3], &[0, 0, 3], &[10, 10, 10, 10], vec![0, 3, 2, 10]);
    test(&[0, 0, 3], &[0, 3, 3], &[10, 10, 10, 10], vec![0, 3, 2, 10]);
    test(&[0, 3], &[0, 0, 3], &[10, 10, 10, 10], vec![0, 3, 10, 10]);
    test(&[0, 0, 3], &[0, 3], &[10, 10, 10, 10], vec![0, 3, 10, 10]);

    test(&[1, 2, 3], &[6, 7], &[10, 10], vec![1, 2]);
    test(&[6, 7], &[1, 2, 3], &[10, 10], vec![1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_or_neg_neg_to_out_fail_1() {
    let mut out = vec![10, 10, 10, 10];
    limbs_or_neg_neg_to_out(&mut out, &[0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_or_neg_neg_to_out_fail_2() {
    let mut out = vec![10, 10, 10, 10];
    limbs_or_neg_neg_to_out(&mut out, &[3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_or_neg_neg_to_out_fail_3() {
    let mut out = vec![10];
    limbs_or_neg_neg_to_out(&mut out, &[6, 7, 8], &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_or_neg_neg_in_place_left() {
    let test = |xs_before: &[Limb], ys, xs_after| {
        let mut xs = xs_before.to_vec();
        limbs_slice_or_neg_neg_in_place_left(&mut xs, ys);
        assert_eq!(xs, xs_after);
    };
    test(&[2], &[3], vec![1]);
    test(&[1, 1, 1], &[1, 2, 3], vec![1, 0, 1]);
    test(&[6, 7], &[1, 2, 3], vec![1, 2]);
    test(&[1, 2, 3], &[6, 7], vec![1, 2, 0]);
    test(&[100, 101, 102], &[102, 101, 100], vec![98, 101, 100]);
    test(&[0, 0, 1], &[3], vec![3, 0, 0]);
    test(&[3], &[0, 0, 1], vec![3]);
    test(&[0, 3, 3], &[0, 0, 3], vec![0, 3, 2]);
    test(&[0, 0, 3], &[0, 3, 3], vec![0, 3, 2]);
    test(&[0, 3], &[0, 0, 3], vec![0, 3]);
    test(&[0, 0, 3], &[0, 3], vec![0, 3, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_or_neg_neg_in_place_left_fail_1() {
    limbs_slice_or_neg_neg_in_place_left(&mut [0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_or_neg_neg_in_place_left_fail_2() {
    limbs_slice_or_neg_neg_in_place_left(&mut [3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_vec_or_neg_neg_in_place_left_fail_1() {
    limbs_vec_or_neg_neg_in_place_left(&mut vec![0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_vec_or_neg_neg_in_place_left_fail_2() {
    limbs_vec_or_neg_neg_in_place_left(&mut vec![3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_or_neg_neg_in_place_either() {
    let test = |xs_before: &[Limb], ys_before: &[Limb], b, xs_after, ys_after| {
        let mut xs = xs_before.to_vec();
        let mut ys = ys_before.to_vec();
        assert_eq!(limbs_or_neg_neg_in_place_either(&mut xs, &mut ys), b);
        assert_eq!(xs, xs_after);
        assert_eq!(ys, ys_after);
    };
    test(&[2], &[3], false, vec![1], vec![3]);
    test(&[1, 1, 1], &[1, 2, 3], false, vec![1, 0, 1], vec![1, 2, 3]);
    test(&[1, 2, 3], &[6, 7], true, vec![1, 2, 3], vec![1, 2]);
    test(&[6, 7], &[1, 2, 3], false, vec![1, 2], vec![1, 2, 3]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![98, 101, 100],
        vec![102, 101, 100],
    );
    test(&[0, 0, 1], &[3], true, vec![0, 0, 1], vec![3]);
    test(&[3], &[0, 0, 1], false, vec![3], vec![0, 0, 1]);
    test(&[0, 3, 3], &[0, 0, 3], false, vec![0, 3, 2], vec![0, 0, 3]);
    test(&[0, 0, 3], &[0, 3, 3], false, vec![0, 3, 2], vec![0, 3, 3]);
    test(&[0, 3], &[0, 0, 3], false, vec![0, 3], vec![0, 0, 3]);
    test(&[0, 0, 3], &[0, 3], true, vec![0, 0, 3], vec![0, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_or_neg_neg_in_place_either_fail_1() {
    limbs_or_neg_neg_in_place_either(&mut vec![0, 0, 0], &mut vec![3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_or_neg_neg_in_place_either_fail_2() {
    limbs_or_neg_neg_in_place_either(&mut vec![3], &mut vec![0, 0, 0]);
}

#[test]
fn test_or() {
    let test = |u, v, out| {
        let mut n = Integer::from_str(u).unwrap();
        n |= Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Integer::from_str(u).unwrap();
        n |= &Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap() | Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() | Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap() | &Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() | &Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        assert_eq!(
            integer_or_alt_1(
                &Integer::from_str(u).unwrap(),
                &Integer::from_str(v).unwrap(),
            )
            .to_string(),
            out
        );
        assert_eq!(
            integer_or_alt_2(
                &Integer::from_str(u).unwrap(),
                &Integer::from_str(v).unwrap(),
            )
            .to_string(),
            out
        );

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
    test("0", "-123", "-123");
    test("123", "-456", "-389");
    test("1000000000000", "-123", "-123");
    test("123", "-1000000000000", "-999999999877");
    test("1000000000000", "-999999999999", "-4095");
    test("12345678987654321", "-314159265358979", "-1827599417347");
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
    test("-123", "-456", "-67");
    test("-1000000000000", "-123", "-123");
    test("-123", "-1000000000000", "-123");
    test("-1000000000000", "-999999999999", "-999999999999");
    test("-12345678987654321", "-314159265358979", "-312331665941633");

    test(
        "17561442137713604341197",
        "-533163900219836",
        "-75045493870643",
    );
    test(
        "-18446744013580009457",
        "-18446673704965373937",
        "-18446673644835831793",
    );
    test(
        "-18446673704965373937",
        "-18446744013580009457",
        "-18446673644835831793",
    );
    test(
        "-324518553658389833295008601473024",
        "317057721155483154675232931839",
        "-324201495937234350140333368541185",
    );
    test(
        "317057721155483154675232931839",
        "-324518553658389833295008601473024",
        "-324201495937234350140333368541185",
    );
    test(
        "-324201495937234350140333368541185",
        "-324518553658389833295008601473024",
        "-324201495937234350140333368541185",
    );
    test(
        "-324518553658389833295008601473024",
        "-324201495937234350140333368541185",
        "-324201495937234350140333368541185",
    );
    test(
        "576458553284361984",
        "-10889035741470030830237691627457877114880",
        "-10889035741470030830237115168904592752896",
    );
    test(
        "-26298808336",
        "170141183460469156173823577801560686592",
        "-26298808336",
    );
    test(
        "-4363947867655",
        "-158453907176889445928738488320",
        "-4363947867655",
    );
}

#[test]
fn limbs_neg_or_limb_properties() {
    test_properties(pairs_of_limb_vec_and_limb_var_1, |&(ref limbs, limb)| {
        assert_eq!(
            -Natural::from_owned_limbs_asc(limbs_neg_or_limb(limbs, limb)),
            -Natural::from_limbs_asc(limbs) | Integer::from(limb)
        );
    });
}

#[test]
fn limbs_neg_or_limb_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_3,
        |&(ref out, ref in_limbs, limb)| {
            let mut out = out.to_vec();
            let old_out = out.clone();
            limbs_neg_or_limb_to_out(&mut out, in_limbs, limb);
            let len = in_limbs.len();
            assert_eq!(
                -Natural::from_limbs_asc(&out[..len]),
                -Natural::from_limbs_asc(in_limbs) | Integer::from(limb),
            );
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_neg_or_limb_in_place_properties() {
    test_properties(pairs_of_limb_vec_and_limb_var_1, |&(ref limbs, limb)| {
        let mut limbs = limbs.to_vec();
        let old_limbs = limbs.clone();
        limbs_neg_or_limb_in_place(&mut limbs, limb);
        assert_eq!(
            -Natural::from_limbs_asc(&limbs),
            -Natural::from_limbs_asc(&old_limbs) | Integer::from(limb)
        );
    });
}

#[test]
fn limbs_pos_or_neg_limb_properties() {
    test_properties(
        pairs_of_limb_vec_and_positive_limb_var_1,
        |&(ref limbs, u)| {
            assert_eq!(
                limbs_pos_or_neg_limb(limbs, u),
                -(Integer::from(Natural::from_limbs_asc(limbs))
                    | Integer::from_owned_twos_complement_limbs_asc(vec![u, Limb::MAX]))
            );
        },
    );
}

#[test]
fn limbs_neg_or_neg_limb_properties() {
    test_properties(
        pairs_of_limb_vec_and_positive_limb_var_1,
        |&(ref limbs, u)| {
            assert_eq!(
                limbs_neg_or_neg_limb(limbs, u),
                -(-Natural::from_limbs_asc(limbs)
                    | Integer::from_owned_twos_complement_limbs_asc(vec![u, Limb::MAX]))
            );
        },
    );
}

#[test]
fn limbs_or_neg_neg_properties() {
    test_properties(pairs_of_unsigned_vec_var_6, |&(ref xs, ref ys)| {
        assert_eq!(
            -Natural::from_owned_limbs_asc(limbs_or_neg_neg(xs, ys)),
            -Natural::from_limbs_asc(xs) | -Natural::from_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_or_neg_neg_to_out_properties() {
    test_properties(triples_of_limb_vec_var_8, |&(ref xs, ref ys, ref zs)| {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_or_neg_neg_to_out(&mut xs, ys, zs);
        let len = min(ys.len(), zs.len());
        let result =
            Natural::checked_from(-(-Natural::from_limbs_asc(ys) | -Natural::from_limbs_asc(zs)))
                .unwrap();
        let mut expected_limbs = result.to_limbs_asc();
        expected_limbs.resize(len, 0);
        assert_eq!(&xs[..len], expected_limbs.as_slice());
        assert_eq!(&xs[len..], &xs_old[len..]);
    });
}

macro_rules! limbs_slice_or_neg_neg_in_place_left_helper {
    ($f:ident, $xs:ident, $ys:ident) => {
        |&(ref $xs, ref $ys)| {
            let mut xs = $xs.to_vec();
            let xs_old = $xs.clone();
            $f(&mut xs, $ys);
            assert_eq!(
                -Natural::from_owned_limbs_asc(xs),
                -Natural::from_owned_limbs_asc(xs_old) | -Natural::from_limbs_asc($ys)
            );
        }
    };
}

#[test]
fn limbs_slice_or_neg_neg_in_place_left_properties() {
    test_properties(
        pairs_of_unsigned_vec_var_6,
        limbs_slice_or_neg_neg_in_place_left_helper!(limbs_slice_or_neg_neg_in_place_left, xs, ys),
    );
}

#[test]
fn limbs_vec_or_neg_neg_in_place_left_properties() {
    test_properties(
        pairs_of_unsigned_vec_var_6,
        limbs_slice_or_neg_neg_in_place_left_helper!(limbs_vec_or_neg_neg_in_place_left, xs, ys),
    );
}

#[test]
fn limbs_or_neg_neg_in_place_either_properties() {
    test_properties(pairs_of_unsigned_vec_var_6, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let mut ys = ys.to_vec();
        let ys_old = ys.clone();
        let right = limbs_or_neg_neg_in_place_either(&mut xs, &mut ys);
        let expected = -Natural::from_limbs_asc(&xs_old) | -Natural::from_limbs_asc(&ys_old);
        if right {
            assert_eq!(xs, xs_old);
            assert_eq!(-Natural::from_owned_limbs_asc(ys), expected);
        } else {
            assert_eq!(-Natural::from_owned_limbs_asc(xs), expected);
            assert_eq!(ys, ys_old);
        }
    });
}

#[test]
fn or_properties() {
    test_properties(pairs_of_integers, |&(ref x, ref y)| {
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
        mut_x |= integer_to_rug_integer(y);
        assert_eq!(rug_integer_to_integer(&mut_x), result);

        assert_eq!(
            rug_integer_to_integer(&(integer_to_rug_integer(x) | integer_to_rug_integer(y))),
            result
        );

        assert_eq!(integer_or_alt_1(&x, y), result);
        assert_eq!(integer_or_alt_2(&x, y), result);

        assert_eq!(y | x, result);
        assert_eq!(&result | x, result);
        assert_eq!(&result | y, result);
        assert_eq!(!(!x & !y), result);
    });

    test_properties(integers, |x| {
        assert_eq!(x | Integer::ZERO, *x);
        assert_eq!(Integer::ZERO | x, *x);
        assert_eq!(x | Integer::NEGATIVE_ONE, -1 as SignedLimb);
        assert_eq!(Integer::NEGATIVE_ONE | x, -1 as SignedLimb);
        assert_eq!(x | x, *x);
        assert_eq!(x | !x, -1 as SignedLimb);
    });

    test_properties(triples_of_integers, |&(ref x, ref y, ref z)| {
        assert_eq!((x | y) | z, x | (y | z));
        assert_eq!(x & (y | z), (x & y) | (x & z));
        assert_eq!((x & y) | z, (x | z) & (y | z));
        assert_eq!(x | (y & z), (x | y) & (x | z));
        assert_eq!((x | y) & z, (x & z) | (y & z));
    });

    test_properties(pairs_of_signeds::<SignedLimb>, |&(i, j)| {
        assert_eq!(Integer::from(i) | Integer::from(j), i | j);
    });

    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        assert_eq!(Integer::from(x) | Integer::from(y), x | y);
    });

    test_properties(pairs_of_integer_and_natural, |&(ref x, ref y)| {
        assert_eq!(x | y, x | Integer::from(y));
    });
}
