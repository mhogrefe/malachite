use common::test_properties;
use malachite_base::misc::CheckedFrom;
use malachite_base::num::{NegativeOne, Zero};
use malachite_nz::integer::logic::xor_u32::{
    limbs_neg_xor_limb, limbs_neg_xor_limb_to_out, limbs_slice_neg_xor_limb_in_place,
    limbs_vec_neg_xor_limb_in_place,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::common::{integer_to_rug_integer, rug_integer_to_integer};
use malachite_test::inputs::base::{
    pairs_of_u32_vec_and_u32_var_1, triples_of_u32_vec_u32_vec_and_u32_var_2, unsigneds,
};
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_unsigned};
use malachite_test::integer::logic::xor_u32::{integer_xor_u32_alt_1, integer_xor_u32_alt_2};
use rug::{self, Assign};
use std::str::FromStr;

#[test]
fn test_limbs_neg_xor_limb() {
    let test = |limbs: &[u32], limb: u32, out: &[u32]| {
        assert_eq!(limbs_neg_xor_limb(limbs, limb), out);
    };
    test(&[6, 7], 0, &[6, 7]);
    test(&[6, 7], 2, &[8, 7]);
    test(&[100, 101, 102], 10, &[106, 101, 102]);
    test(&[123, 456], 789, &[880, 456]);
    test(&[0xffff_fffe, 0xffff_ffff, 0xffff_ffff], 2, &[0, 0, 0, 1]);
    test(
        &[0, 0, 0, 1],
        2,
        &[0xffff_fffe, 0xffff_ffff, 0xffff_ffff, 0],
    );
}

#[test]
fn test_limbs_neg_xor_limb_to_out() {
    let test =
        |limbs_out_before: &[u32], limbs_in: &[u32], limb: u32, carry, limbs_out_after: &[u32]| {
            let mut limbs_out = limbs_out_before.to_vec();
            assert_eq!(
                limbs_neg_xor_limb_to_out(&mut limbs_out, limbs_in, limb),
                carry
            );
            assert_eq!(limbs_out, limbs_out_after);
        };
    test(&[10, 10, 10, 10], &[6, 7], 0, false, &[6, 7, 10, 10]);
    test(&[10, 10, 10, 10], &[6, 7], 2, false, &[8, 7, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[100, 101, 102],
        10,
        false,
        &[106, 101, 102, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[123, 456],
        789,
        false,
        &[880, 456, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[0xffff_fffe, 0xffff_ffff, 0xffff_ffff],
        2,
        true,
        &[0, 0, 0, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[0, 0, 0, 1],
        2,
        false,
        &[0xffff_fffe, 0xffff_ffff, 0xffff_ffff, 0],
    );
}

#[test]
#[should_panic(expected = "assertion failed: out_limbs.len() >= len")]
fn limbs_neg_xor_limb_to_out_fail() {
    limbs_neg_xor_limb_to_out(&mut [10], &[10, 10], 10);
}

#[test]
fn test_limbs_slice_neg_xor_limb_in_place() {
    let test = |limbs: &[u32], limb: u32, carry, out: &[u32]| {
        let mut limbs = limbs.to_vec();
        assert_eq!(limbs_slice_neg_xor_limb_in_place(&mut limbs, limb), carry);
        assert_eq!(limbs, out);
    };
    test(&[6, 7], 0, false, &[6, 7]);
    test(&[6, 7], 2, false, &[8, 7]);
    test(&[100, 101, 102], 10, false, &[106, 101, 102]);
    test(&[123, 456], 789, false, &[880, 456]);
    test(
        &[0xffff_fffe, 0xffff_ffff, 0xffff_ffff],
        2,
        true,
        &[0, 0, 0],
    );
    test(
        &[0, 0, 0, 1],
        2,
        false,
        &[0xffff_fffe, 0xffff_ffff, 0xffff_ffff, 0],
    );
}

#[test]
fn test_limbs_vec_neg_xor_limb_in_place() {
    let test = |limbs: &[u32], limb: u32, out: &[u32]| {
        let mut limbs = limbs.to_vec();
        limbs_vec_neg_xor_limb_in_place(&mut limbs, limb);
        assert_eq!(limbs, out);
    };
    test(&[6, 7], 0, &[6, 7]);
    test(&[6, 7], 2, &[8, 7]);
    test(&[100, 101, 102], 10, &[106, 101, 102]);
    test(&[123, 456], 789, &[880, 456]);
    test(&[0xffff_fffe, 0xffff_ffff, 0xffff_ffff], 2, &[0, 0, 0, 1]);
    test(
        &[0, 0, 0, 1],
        2,
        &[0xffff_fffe, 0xffff_ffff, 0xffff_ffff, 0],
    );
}

#[test]
fn test_xor_i32() {
    let test = |u, v: u32, out| {
        let mut n = Integer::from_str(u).unwrap();
        n ^= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from_str(u).unwrap();
        n ^= v;
        assert_eq!(n.to_string(), out);

        let n = Integer::from_str(u).unwrap() ^ v;
        assert_eq!(n.to_string(), out);

        let n = &Integer::from_str(u).unwrap() ^ v;
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() ^ v;
        assert_eq!(n.to_string(), out);

        let n = v ^ Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);

        let n = v ^ &Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);

        let n = v ^ rug::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);

        let mut n = rug::Integer::from(0);
        n.assign(v ^ &rug::Integer::from_str(u).unwrap());
        assert_eq!(n.to_string(), out);
    };

    test("0", 0, "0");
    test("0", 123, "123");
    test("123", 0, "123");
    test("123", 456, "435");
    test("999999999999", 123, "999999999876");
    test("1000000000000", 123, "1000000000123");
    test("1000000000001", 123, "1000000000122");
    test("12345678987654321", 0, "12345678987654321");
    test("12345678987654321", 456, "12345678987654521");
    test("12345678987654321", 987_654_321, "12345678815534080");

    test("-123", 0, "-123");
    test("-123", 456, "-435");
    test("-999999999999", 123, "-999999999878");
    test("-1000000000000", 123, "-999999999877");
    test("-1000000000001", 123, "-1000000000124");
    test("-12345678987654321", 0, "-12345678987654321");
    test("-12345678987654321", 456, "-12345678987654521");
    test("-12345678987654321", 987_654_321, "-12345678815534082");

    test("-4294967293", 3, "-4294967296");
    test("-4294967296", 3, "-4294967293");
    test("-18446744073709551613", 3, "-18446744073709551616");
    test("-18446744073709551616", 3, "-18446744073709551613");
}

#[test]
fn limbs_neg_xor_limb_properties() {
    test_properties(pairs_of_u32_vec_and_u32_var_1, |&(ref limbs, limb)| {
        assert_eq!(
            -Natural::from_owned_limbs_asc(limbs_neg_xor_limb(limbs, limb)),
            -Natural::from_limbs_asc(limbs) ^ limb
        );
    });
}

#[test]
fn limbs_neg_xor_limb_to_out_properties() {
    test_properties(
        triples_of_u32_vec_u32_vec_and_u32_var_2,
        |&(ref out_limbs, ref in_limbs, limb)| {
            let mut out_limbs = out_limbs.to_vec();
            let old_out_limbs = out_limbs.clone();
            let len = in_limbs.len();
            if limbs_neg_xor_limb_to_out(&mut out_limbs, in_limbs, limb) {
                let mut result_limbs = Natural::checked_from(
                    -(Integer::from(Natural::from_limbs_asc(in_limbs)) ^ limb),
                ).unwrap()
                    .to_limbs_asc();
                result_limbs.resize(len, 0);
                assert_eq!(result_limbs, &out_limbs[0..len]);
            } else {
                assert_eq!(
                    -Natural::from_limbs_asc(&out_limbs[0..len]),
                    -Natural::from_limbs_asc(in_limbs) ^ limb,
                );
            }
            assert_eq!(&out_limbs[len..], &old_out_limbs[len..]);
        },
    );
}

#[test]
fn limbs_slice_neg_xor_limb_in_place_properties() {
    test_properties(pairs_of_u32_vec_and_u32_var_1, |&(ref limbs, limb)| {
        let mut limbs = limbs.to_vec();
        let old_limbs = limbs.clone();
        if limbs_slice_neg_xor_limb_in_place(&mut limbs, limb) {
            let mut result_limbs = Natural::checked_from(
                -(Integer::from(Natural::from_owned_limbs_asc(old_limbs)) ^ limb),
            ).unwrap()
                .to_limbs_asc();
            result_limbs.resize(limbs.len(), 0);
            assert_eq!(result_limbs, limbs);
        } else {
            assert_eq!(
                -Natural::from_limbs_asc(&limbs),
                -Natural::from_owned_limbs_asc(old_limbs) ^ limb
            );
        }
    });
}

#[test]
fn limbs_vec_neg_xor_limb_in_place_properties() {
    test_properties(pairs_of_u32_vec_and_u32_var_1, |&(ref limbs, limb)| {
        let mut limbs = limbs.to_vec();
        let old_limbs = limbs.clone();
        limbs_vec_neg_xor_limb_in_place(&mut limbs, limb);
        assert_eq!(
            -Natural::from_limbs_asc(&limbs),
            -Natural::from_owned_limbs_asc(old_limbs) ^ limb
        );
    });
}

#[test]
fn xor_u32_properties() {
    test_properties(
        pairs_of_integer_and_unsigned,
        |&(ref n, u): &(Integer, u32)| {
            let mut mut_n = n.clone();
            mut_n ^= u;
            assert!(mut_n.is_valid());
            let result = mut_n;

            let mut rug_n = integer_to_rug_integer(n);
            rug_n ^= u;
            assert_eq!(rug_integer_to_integer(&rug_n), result);

            assert_eq!(n ^ u, result);
            assert_eq!(u ^ n, result);
            assert_eq!(integer_xor_u32_alt_1(&n, u), result);
            assert_eq!(integer_xor_u32_alt_2(&n, u), result);

            //TODO assert_eq!(n ^ Integer::from(u), result);
            //TODO assert_eq!(Integer::from(u) ^ n, result);

            assert_eq!(&result ^ u, *n);

            assert_eq!(
                rug_integer_to_integer(&(integer_to_rug_integer(n) ^ u)),
                result
            );
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n ^ 0u32, *n);
        assert_eq!(0u32 ^ n, *n);
    });

    test_properties(unsigneds, |&u: &u32| {
        assert_eq!(&Integer::ZERO ^ u, u);
        assert_eq!(u ^ &Integer::ZERO, u);
        assert_eq!(&Integer::NEGATIVE_ONE ^ u, !Integer::from(u));
        assert_eq!(u ^ &Integer::NEGATIVE_ONE, !Integer::from(u));
    });
}
