use std::str::FromStr;

use malachite_base::num::basic::traits::{NegativeOne, Zero};
use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_nz::integer::logic::xor_limb::{
    limbs_neg_xor_limb, limbs_neg_xor_limb_to_out, limbs_slice_neg_xor_limb_in_place,
    limbs_vec_neg_xor_limb_in_place,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
#[cfg(feature = "32_bit_limbs")]
use rug::{self, Assign};

use common::test_properties;
#[cfg(feature = "32_bit_limbs")]
use malachite_test::common::{integer_to_rug_integer, rug_integer_to_integer};
use malachite_test::inputs::base::{
    pairs_of_limb_vec_and_limb_var_1, triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_3,
    unsigneds,
};
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_unsigned};
use malachite_test::integer::logic::xor_limb::{integer_xor_limb_alt_1, integer_xor_limb_alt_2};

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_neg_xor_limb_and_limbs_vec_neg_xor_limb_in_place() {
    let test = |limbs: &[Limb], limb: Limb, out: &[Limb]| {
        assert_eq!(limbs_neg_xor_limb(limbs, limb), out);

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

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_neg_xor_limb_to_out() {
    let test = |out_before: &[Limb], limbs_in: &[Limb], limb: Limb, carry, out_after: &[Limb]| {
        let mut out = out_before.to_vec();
        assert_eq!(limbs_neg_xor_limb_to_out(&mut out, limbs_in, limb), carry);
        assert_eq!(out, out_after);
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

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_neg_xor_limb_to_out_fail() {
    limbs_neg_xor_limb_to_out(&mut [10], &[10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_neg_xor_limb_in_place() {
    let test = |limbs: &[Limb], limb: Limb, carry, out: &[Limb]| {
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
fn test_xor_signed_limb() {
    let test = |u, v: Limb, out| {
        let mut n = Integer::from_str(u).unwrap();
        n ^= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        #[cfg(feature = "32_bit_limbs")]
        {
            let mut n = rug::Integer::from_str(u).unwrap();
            n ^= v;
            assert_eq!(n.to_string(), out);
        }

        let n = Integer::from_str(u).unwrap() ^ v;
        assert_eq!(n.to_string(), out);

        let n = &Integer::from_str(u).unwrap() ^ v;
        assert_eq!(n.to_string(), out);

        #[cfg(feature = "32_bit_limbs")]
        {
            let n = rug::Integer::from_str(u).unwrap() ^ v;
            assert_eq!(n.to_string(), out);
        }

        let n = v ^ Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);

        let n = v ^ &Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);

        assert_eq!(
            integer_xor_limb_alt_1(&Integer::from_str(u).unwrap(), v).to_string(),
            out
        );
        assert_eq!(
            integer_xor_limb_alt_2(&Integer::from_str(u).unwrap(), v).to_string(),
            out
        );

        #[cfg(feature = "32_bit_limbs")]
        {
            let n = v ^ rug::Integer::from_str(u).unwrap();
            assert_eq!(n.to_string(), out);

            let mut n = rug::Integer::from(0);
            n.assign(v ^ &rug::Integer::from_str(u).unwrap());
            assert_eq!(n.to_string(), out);
        }
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
    test_properties(pairs_of_limb_vec_and_limb_var_1, |&(ref limbs, limb)| {
        assert_eq!(
            -Natural::from_owned_limbs_asc(limbs_neg_xor_limb(limbs, limb)),
            -Natural::from_limbs_asc(limbs) ^ limb
        );
    });
}

#[test]
fn limbs_neg_xor_limb_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_3,
        |&(ref out, ref in_limbs, limb)| {
            let mut out = out.to_vec();
            let old_out = out.clone();
            let len = in_limbs.len();
            if limbs_neg_xor_limb_to_out(&mut out, in_limbs, limb) {
                let mut result_limbs = Natural::checked_from(
                    -(Integer::from(Natural::from_limbs_asc(in_limbs)) ^ limb),
                )
                .unwrap()
                .to_limbs_asc();
                result_limbs.resize(len, 0);
                assert_eq!(result_limbs, &out[..len]);
            } else {
                assert_eq!(
                    -Natural::from_limbs_asc(&out[..len]),
                    -Natural::from_limbs_asc(in_limbs) ^ limb,
                );
            }
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_slice_neg_xor_limb_in_place_properties() {
    test_properties(pairs_of_limb_vec_and_limb_var_1, |&(ref limbs, limb)| {
        let mut limbs = limbs.to_vec();
        let old_limbs = limbs.clone();
        if limbs_slice_neg_xor_limb_in_place(&mut limbs, limb) {
            let mut result_limbs = Natural::checked_from(
                -(Integer::from(Natural::from_owned_limbs_asc(old_limbs)) ^ limb),
            )
            .unwrap()
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
    test_properties(pairs_of_limb_vec_and_limb_var_1, |&(ref limbs, limb)| {
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
fn xor_limb_properties() {
    test_properties(
        pairs_of_integer_and_unsigned,
        |&(ref n, u): &(Integer, Limb)| {
            let mut mut_n = n.clone();
            mut_n ^= u;
            assert!(mut_n.is_valid());
            let result = mut_n;

            #[cfg(feature = "32_bit_limbs")]
            {
                let mut rug_n = integer_to_rug_integer(n);
                rug_n ^= u;
                assert_eq!(rug_integer_to_integer(&rug_n), result);
            }

            assert_eq!(n ^ u, result);
            assert_eq!(u ^ n, result);
            assert_eq!(integer_xor_limb_alt_1(&n, u), result);
            assert_eq!(integer_xor_limb_alt_2(&n, u), result);

            assert_eq!(n ^ Integer::from(u), result);
            assert_eq!(Integer::from(u) ^ n, result);

            assert_eq!(&result ^ u, *n);

            #[cfg(feature = "32_bit_limbs")]
            assert_eq!(
                rug_integer_to_integer(&(integer_to_rug_integer(n) ^ u)),
                result
            );
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n ^ 0 as Limb, *n);
        assert_eq!(0 as Limb ^ n, *n);
    });

    test_properties(unsigneds, |&u: &Limb| {
        assert_eq!(&Integer::ZERO ^ u, u);
        assert_eq!(u ^ &Integer::ZERO, u);
        assert_eq!(&Integer::NEGATIVE_ONE ^ u, !Integer::from(u));
        assert_eq!(u ^ &Integer::NEGATIVE_ONE, !Integer::from(u));
    });
}
