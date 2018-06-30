use common::test_properties;
use malachite_base::misc::CheckedFrom;
use malachite_base::num::{NegativeOne, Zero};
use malachite_nz::integer::logic::and_i32::{
    limbs_neg_and_limb_neg, limbs_neg_and_limb_neg_to_out, limbs_pos_and_limb_neg,
    limbs_pos_and_limb_neg_in_place, limbs_pos_and_limb_neg_to_out,
    limbs_slice_neg_and_limb_neg_in_place, limbs_vec_neg_and_limb_neg_in_place,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::common::{integer_to_rug_integer, rug_integer_to_integer};
use malachite_test::inputs::base::{
    pairs_of_nonempty_unsigned_vec_and_unsigned, pairs_of_u32_vec_and_u32_var_1, signeds,
    triples_of_u32_vec_u32_vec_and_u32_var_2,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1,
};
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_signed};
use malachite_test::integer::logic::and_i32::{integer_and_i32_alt_1, integer_and_i32_alt_2};
use rug::{self, Assign};
use std::str::FromStr;
use std::u32;

#[test]
fn test_limbs_pos_and_limb_neg() {
    let test = |limbs: &[u32], limb: u32, out: &[u32]| {
        assert_eq!(limbs_pos_and_limb_neg(limbs, limb), out);
    };
    test(&[6, 7], 2, &[2, 7]);
    test(&[100, 101, 102], 10, &[0, 101, 102]);
    test(&[123, 456], 789, &[17, 456]);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
fn limbs_pos_and_limb_neg_fail() {
    limbs_pos_and_limb_neg(&[], 10);
}

#[test]
fn test_limbs_pos_and_limb_neg_to_out() {
    let test = |limbs_out_before: &[u32], limbs_in: &[u32], limb: u32, limbs_out_after: &[u32]| {
        let mut limbs_out = limbs_out_before.to_vec();
        limbs_pos_and_limb_neg_to_out(&mut limbs_out, limbs_in, limb);
        assert_eq!(limbs_out, limbs_out_after);
    };
    test(&[10, 10, 10, 10], &[6, 7], 2, &[2, 7, 10, 10]);
    test(&[10, 10, 10, 10], &[100, 101, 102], 10, &[0, 101, 102, 10]);
    test(&[10, 10, 10, 10], &[123, 456], 789, &[17, 456, 10, 10]);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
fn limbs_pos_and_limb_neg_to_out_fail_1() {
    limbs_pos_and_limb_neg_to_out(&mut [], &[], 10);
}

#[test]
#[should_panic(expected = "assertion failed: out_limbs.len() >= len")]
fn limbs_pos_and_limb_neg_to_out_fail_2() {
    limbs_pos_and_limb_neg_to_out(&mut [10], &[10, 10], 10);
}

#[test]
fn test_limbs_pos_and_limb_neg_in_place() {
    let test = |limbs: &[u32], limb: u32, out: &[u32]| {
        let mut limbs = limbs.to_vec();
        limbs_pos_and_limb_neg_in_place(&mut limbs, limb);
        assert_eq!(limbs, out);
    };
    test(&[6, 7], 2, &[2, 7]);
    test(&[100, 101, 102], 10, &[0, 101, 102]);
    test(&[123, 456], 789, &[17, 456]);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
fn limbs_pos_and_limb_neg_in_place_fail() {
    limbs_pos_and_limb_neg_in_place(&mut [], 10);
}

#[test]
fn test_limbs_neg_and_limb_neg() {
    let test = |limbs: &[u32], limb: u32, out_limbs: &[u32]| {
        assert_eq!(limbs_neg_and_limb_neg(limbs, limb), out_limbs);
    };
    test(&[0, 2], 3, &[0, 2]);
    test(&[1, 1], 3, &[4_294_967_293, 1]);
    test(&[0xffff_fffe, 1], 1, &[0, 2]);
    test(&[0xffff_fffe, 0xffff_ffff], 1, &[0, 0, 1]);
}

#[test]
#[should_panic(expected = "assertion failed: mid <= len")]
fn limbs_neg_and_limb_neg_fail() {
    limbs_neg_and_limb_neg(&[], 10);
}

#[test]
fn test_limbs_neg_and_limb_neg_to_out() {
    let test =
        |out_limbs_before: &[u32], in_limbs: &[u32], limb: u32, carry, out_limbs_after: &[u32]| {
            let mut out_limbs = out_limbs_before.to_vec();
            assert_eq!(
                limbs_neg_and_limb_neg_to_out(&mut out_limbs, in_limbs, limb),
                carry
            );
            assert_eq!(out_limbs, out_limbs_after);
        };
    test(&[0, 0], &[0, 2], 3, false, &[0, 2]);
    test(&[1, 2, 100], &[0, 2, 100], 3, false, &[0, 2, 100]);
    test(&[0, 0], &[1, 1], 3, false, &[4_294_967_293, 1]);
    test(&[0, 0], &[0xffff_fffe, 1], 1, false, &[0, 2]);
    test(&[0, 0], &[0xffff_fffe, 0xffff_ffff], 1, true, &[0, 0]);
    test(
        &[1, 2, 100],
        &[0xffff_fffe, 0xffff_ffff],
        1,
        true,
        &[0, 0, 100],
    );
}

#[test]
#[should_panic(expected = "assertion failed: out_limbs.len() >= in_limbs.len()")]
fn limbs_neg_and_limb_neg_to_out_fail_1() {
    limbs_neg_and_limb_neg_to_out(&mut [1, 2, 3], &[1, 2, 3, 4], 10);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
fn limbs_neg_and_limb_neg_to_out_fail_2() {
    limbs_neg_and_limb_neg_to_out(&mut [1, 2, 3], &[], 10);
}

#[test]
fn test_limbs_slice_neg_and_limb_neg_in_place() {
    let test = |limbs_before: &[u32], limb: u32, carry, limbs_after: &[u32]| {
        let mut limbs = limbs_before.to_vec();
        assert_eq!(
            limbs_slice_neg_and_limb_neg_in_place(&mut limbs, limb),
            carry
        );
        assert_eq!(limbs, limbs_after);
    };
    test(&[0, 2], 3, false, &[0, 2]);
    test(&[1, 1], 3, false, &[4_294_967_293, 1]);
    test(&[0xffff_fffe, 1], 1, false, &[0, 2]);
    test(&[0xffff_fffe, 0xffff_ffff], 1, true, &[0, 0]);
}

#[test]
#[should_panic(expected = "assertion failed: mid <= len")]
fn limbs_slice_neg_and_limb_neg_in_place_fail() {
    limbs_slice_neg_and_limb_neg_in_place(&mut [], 10);
}

#[test]
fn test_limbs_vec_neg_and_limb_neg_in_place() {
    let test = |limbs_before: &[u32], limb: u32, limbs_after: &[u32]| {
        let mut limbs = limbs_before.to_vec();
        limbs_vec_neg_and_limb_neg_in_place(&mut limbs, limb);
        assert_eq!(limbs, limbs_after);
    };
    test(&[0, 2], 3, &[0, 2]);
    test(&[1, 1], 3, &[4_294_967_293, 1]);
    test(&[0xffff_fffe, 1], 1, &[0, 2]);
    test(&[0xffff_fffe, 0xffff_ffff], 1, &[0, 0, 1]);
}

#[test]
#[should_panic(expected = "assertion failed: mid <= len")]
fn limbs_vec_neg_and_limb_neg_in_place_fail() {
    let mut limbs = vec![];
    limbs_vec_neg_and_limb_neg_in_place(&mut limbs, 10);
}

#[test]
fn test_and_i32() {
    let test = |u, v: i32, out| {
        let mut n = Integer::from_str(u).unwrap();
        n &= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from_str(u).unwrap();
        n &= v;
        assert_eq!(n.to_string(), out);

        let n = Integer::from_str(u).unwrap() & v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() & v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = rug::Integer::from_str(u).unwrap() & v;
        assert_eq!(n.to_string(), out);

        let n = v & Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v & &Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        assert_eq!(
            integer_and_i32_alt_1(&Integer::from_str(u).unwrap(), v).to_string(),
            out
        );
        assert_eq!(
            integer_and_i32_alt_2(&Integer::from_str(u).unwrap(), v).to_string(),
            out
        );

        let n = v & rug::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);

        let mut n = rug::Integer::from(0);
        n.assign(v & &rug::Integer::from_str(u).unwrap());
        assert_eq!(n.to_string(), out);
    };

    test("0", 0, "0");
    test("0", 123, "0");
    test("123", 0, "0");
    test("123", 456, "72");
    test("1000000000000", 123, "0");
    test("1000000000001", 123, "1");
    test("12345678987654321", 987_654_321, "579887281");
    test("-123", 0, "0");
    test("-123", 456, "384");
    test("-1000000000000", 123, "0");
    test("-1000000000001", 123, "123");
    test("-12345678987654321", 987_654_321, "407767041");

    test("0", -123, "0");
    test("123", -456, "56");
    test("1000000000000", -123, "1000000000000");
    test("1000000000001", -123, "1000000000001");
    test("12345678987654321", -987_654_321, "12345678407767041");
    test("-123", -456, "-512");
    test("-1000000000000", -123, "-1000000000000");
    test("-1000000000001", -123, "-1000000000123");
    test("-12345678987654321", -987_654_321, "-12345679395421361");
    test(
        "16877400614591900061756902599",
        -1_958_485_034,
        "16877400614591900060882124998",
    );

    test("-3486", -12, "-3488");
    test("-3582", -12, "-3584");
    test("-55835164686", -65_532, "-55835230208");
    test("-60129476622", -65_532, "-60129542144");
    test("-4294901774", -65_532, "-4294967296");
    test(
        "-45671926166590716193855479615826927335145209855",
        -7_684,
        "-45671926166590716193855479615826927335145209856",
    );
}

#[test]
fn limbs_pos_and_limb_neg_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_unsigned,
        |&(ref limbs, limb)| {
            let limbs_out = limbs_pos_and_limb_neg(limbs, limb);
            let n = Integer::from(Natural::from_limbs_asc(limbs))
                & Integer::from_owned_twos_complement_limbs_asc(vec![limb, u32::MAX]);
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_out),
                Natural::checked_from(n).unwrap()
            );
        },
    );
}

#[test]
fn limbs_pos_and_limb_neg_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1,
        |&(ref out_limbs, ref in_limbs, limb)| {
            let mut out_limbs = out_limbs.to_vec();
            let old_out_limbs = out_limbs.clone();
            limbs_pos_and_limb_neg_to_out(&mut out_limbs, in_limbs, limb);
            let n = Integer::from(Natural::from_limbs_asc(in_limbs))
                & Integer::from_owned_twos_complement_limbs_asc(vec![limb, u32::MAX]);
            let len = in_limbs.len();
            let mut limbs = Natural::checked_from(n).unwrap().into_limbs_asc();
            limbs.resize(len, 0);
            assert_eq!(limbs, &out_limbs[0..len]);
            assert_eq!(&out_limbs[len..], &old_out_limbs[len..]);
        },
    );
}

#[test]
fn limbs_pos_and_limb_neg_in_place_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_unsigned,
        |&(ref limbs, limb)| {
            let mut limbs = limbs.to_vec();
            limbs_pos_and_limb_neg_in_place(&mut limbs, limb);
            let n = Integer::from(Natural::from_limbs_asc(&limbs))
                & Integer::from_owned_twos_complement_limbs_asc(vec![limb, u32::MAX]);
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs),
                Natural::checked_from(n).unwrap()
            );
        },
    );
}

#[test]
fn limbs_neg_and_limb_neg_properties() {
    test_properties(pairs_of_u32_vec_and_u32_var_1, |&(ref limbs, limb)| {
        let limbs_out = limbs_neg_and_limb_neg(limbs, limb);
        let n = -Natural::from_limbs_asc(limbs)
            & Integer::from_owned_twos_complement_limbs_asc(vec![limb, u32::MAX]);
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_out),
            Natural::checked_from(-n).unwrap()
        );
    });
}

#[test]
fn limbs_neg_and_limb_neg_to_out_properties() {
    test_properties(
        triples_of_u32_vec_u32_vec_and_u32_var_2,
        |&(ref out_limbs, ref in_limbs, limb)| {
            let mut out_limbs = out_limbs.to_vec();
            let old_out_limbs = out_limbs.clone();
            let carry = limbs_neg_and_limb_neg_to_out(&mut out_limbs, in_limbs, limb);
            let n = -Natural::from_limbs_asc(in_limbs)
                & Integer::from_owned_twos_complement_limbs_asc(vec![limb, u32::MAX]);
            let len = in_limbs.len();
            let mut limbs = Natural::checked_from(-n).unwrap().into_limbs_asc();
            assert_eq!(carry, limbs.len() == len + 1);
            limbs.resize(len, 0);
            assert_eq!(limbs, &out_limbs[0..len]);
            assert_eq!(&out_limbs[len..], &old_out_limbs[len..]);
        },
    );
}

#[test]
fn limbs_slice_neg_and_limb_neg_in_place_properties() {
    test_properties(pairs_of_u32_vec_and_u32_var_1, |&(ref limbs, limb)| {
        let mut limbs = limbs.to_vec();
        limbs_slice_neg_and_limb_neg_in_place(&mut limbs, limb);
        let n = -Natural::from_limbs_asc(&limbs)
            & Integer::from_owned_twos_complement_limbs_asc(vec![limb, u32::MAX]);
        let mut expected_limbs = Natural::checked_from(-n).unwrap().into_limbs_asc();
        expected_limbs.resize(limbs.len(), 0);
        assert_eq!(limbs, expected_limbs);
    });
}

#[test]
fn limbs_vec_neg_and_limb_neg_in_place_properties() {
    test_properties(pairs_of_u32_vec_and_u32_var_1, |&(ref limbs, limb)| {
        let mut limbs = limbs.to_vec();
        limbs_vec_neg_and_limb_neg_in_place(&mut limbs, limb);
        let n = -Natural::from_limbs_asc(&limbs)
            & Integer::from_owned_twos_complement_limbs_asc(vec![limb, u32::MAX]);
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs),
            Natural::checked_from(-n).unwrap()
        );
    });
}

#[test]
fn and_i32_properties() {
    test_properties(
        pairs_of_integer_and_signed,
        |&(ref n, i): &(Integer, i32)| {
            let mut mut_n = n.clone();
            mut_n &= i;
            assert!(mut_n.is_valid());
            let result = mut_n;

            let mut rug_n = integer_to_rug_integer(n);
            rug_n &= i;
            assert_eq!(rug_integer_to_integer(&rug_n), result, "{} {}", n, i);

            let result_alt = n & i;
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = n.clone() & i;
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = i & n;
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = i & n.clone();
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            assert_eq!(integer_and_i32_alt_1(&n, i), result);
            assert_eq!(integer_and_i32_alt_2(&n, i), result);

            assert_eq!(&result & i, result);

            assert_eq!(n & Integer::from(i), result);
            assert_eq!(Integer::from(i) & n, result);

            assert_eq!(
                rug_integer_to_integer(&(integer_to_rug_integer(n) & i)),
                result
            );
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n & 0, 0);
        assert_eq!(0 & n, 0);
        assert_eq!(n & -1, *n);
        assert_eq!(-1 & n, *n);
    });

    test_properties(signeds, |&i: &i32| {
        assert_eq!(&Integer::ZERO & i, 0);
        assert_eq!(i & &Integer::ZERO, 0);
        assert_eq!(&Integer::NEGATIVE_ONE & i, i);
        assert_eq!(i & &Integer::NEGATIVE_ONE, i);
    });
}
