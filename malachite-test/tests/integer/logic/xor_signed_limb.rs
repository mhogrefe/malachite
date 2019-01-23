use common::test_properties;
use malachite_base::misc::{CheckedFrom, Max};
use malachite_base::num::{NegativeOne, Zero};
use malachite_nz::integer::logic::xor_signed_limb::{
    limbs_neg_xor_limb_neg, limbs_neg_xor_limb_neg_in_place, limbs_neg_xor_limb_neg_to_out,
    limbs_pos_xor_limb_neg, limbs_pos_xor_limb_neg_to_out, limbs_slice_pos_xor_limb_neg_in_place,
    limbs_vec_pos_xor_limb_neg_in_place,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};
#[cfg(feature = "32_bit_limbs")]
use malachite_test::common::{integer_to_rug_integer, rug_integer_to_integer};
use malachite_test::inputs::base::{
    pairs_of_limb_vec_and_limb_var_1, pairs_of_nonempty_unsigned_vec_and_unsigned,
    pairs_of_signeds, signeds, triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_2,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_3,
};
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_signed};
use malachite_test::integer::logic::xor_signed_limb::{
    integer_xor_signed_limb_alt_1, integer_xor_signed_limb_alt_2,
};
#[cfg(feature = "32_bit_limbs")]
use rug::{self, Assign};
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_pos_xor_limb_neg_and_limbs_vec_pos_xor_limb_neg_in_place() {
    let test = |limbs: &[Limb], limb: Limb, out_limbs: &[Limb]| {
        assert_eq!(limbs_pos_xor_limb_neg(limbs, limb), out_limbs);

        let mut limbs = limbs.to_vec();
        limbs_vec_pos_xor_limb_neg_in_place(&mut limbs, limb);
        assert_eq!(limbs, out_limbs);
    };
    test(&[0, 2], 3, &[4_294_967_293, 2]);
    test(&[1, 2, 3], 4, &[4_294_967_291, 2, 3]);
    test(&[2, 0xffff_ffff], 2, &[0, 0, 1]);
    test(&[2, 0xffff_ffff, 0xffff_ffff], 2, &[0, 0, 0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_xor_limb_neg_fail() {
    limbs_pos_xor_limb_neg(&[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_vec_pos_xor_limb_neg_in_place_fail() {
    let mut limbs = vec![];
    limbs_vec_pos_xor_limb_neg_in_place(&mut limbs, 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_pos_xor_limb_neg_to_out() {
    let test = |out_limbs_before: &[Limb],
                in_limbs: &[Limb],
                limb: Limb,
                carry,
                out_limbs_after: &[Limb]| {
        let mut out_limbs = out_limbs_before.to_vec();
        assert_eq!(
            limbs_pos_xor_limb_neg_to_out(&mut out_limbs, in_limbs, limb),
            carry
        );
        assert_eq!(out_limbs, out_limbs_after);
    };
    test(&[0, 0], &[0, 2], 3, false, &[4_294_967_293, 2]);
    test(
        &[1, 2, 100],
        &[0, 2, 100],
        3,
        false,
        &[4_294_967_293, 2, 100],
    );
    test(&[0, 0, 0], &[1, 2, 3], 4, false, &[4_294_967_291, 2, 3]);
    test(&[0, 0], &[2, 0xffff_ffff], 2, true, &[0, 0]);
    test(
        &[0, 0, 0],
        &[2, 0xffff_ffff, 0xffff_ffff],
        2,
        true,
        &[0, 0, 0],
    );
    test(
        &[1, 2, 3, 100],
        &[2, 0xffff_ffff, 0xffff_ffff],
        2,
        true,
        &[0, 0, 0, 100],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_xor_limb_neg_to_out_fail_1() {
    limbs_pos_xor_limb_neg_to_out(&mut [1, 2, 3], &[1, 2, 3, 4], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_xor_limb_neg_to_out_fail_2() {
    limbs_pos_xor_limb_neg_to_out(&mut [1, 2, 3], &[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_pos_xor_limb_neg_in_place() {
    let test = |limbs_before: &[Limb], limb: Limb, carry, limbs_after: &[Limb]| {
        let mut limbs = limbs_before.to_vec();
        assert_eq!(
            limbs_slice_pos_xor_limb_neg_in_place(&mut limbs, limb),
            carry
        );
        assert_eq!(limbs, limbs_after);
    };
    test(&[0, 2], 3, false, &[4_294_967_293, 2]);
    test(&[1, 2, 3], 4, false, &[4_294_967_291, 2, 3]);
    test(&[2, 0xffff_ffff], 2, true, &[0, 0]);
    test(&[2, 0xffff_ffff, 0xffff_ffff], 2, true, &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_pos_xor_limb_neg_in_place_fail() {
    limbs_slice_pos_xor_limb_neg_in_place(&mut [], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_neg_xor_limb_neg_and_limbs_neg_xor_limb_neg_in_place() {
    let test = |limbs: &[Limb], limb: Limb, out: &[Limb]| {
        assert_eq!(limbs_neg_xor_limb_neg(limbs, limb), out);

        let mut limbs = limbs.to_vec();
        limbs_neg_xor_limb_neg_in_place(&mut limbs, limb);
        assert_eq!(limbs, out);
    };
    test(&[0, 2], 3, &[3, 1]);
    test(&[6, 7], 2, &[4_294_967_288, 7]);
    test(&[1, 2, 3], 4, &[4_294_967_291, 2, 3]);
    test(&[100, 101, 102], 10, &[4_294_967_190, 101, 102]);
    test(&[123, 456], 789, &[4_294_966_416, 456]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_neg_xor_limb_neg_fail() {
    limbs_neg_xor_limb_neg(&[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_neg_xor_limb_neg_in_place_fail() {
    limbs_neg_xor_limb_neg_in_place(&mut [], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_neg_xor_limb_neg_to_out() {
    let test =
        |limbs_out_before: &[Limb], limbs_in: &[Limb], limb: Limb, limbs_out_after: &[Limb]| {
            let mut limbs_out = limbs_out_before.to_vec();
            limbs_neg_xor_limb_neg_to_out(&mut limbs_out, limbs_in, limb);
            assert_eq!(limbs_out, limbs_out_after);
        };
    test(&[10, 10, 10, 10], &[0, 2], 3, &[3, 1, 10, 10]);
    test(&[10, 10, 10, 10], &[6, 7], 2, &[4_294_967_288, 7, 10, 10]);
    test(&[10, 10, 10, 10], &[1, 2, 3], 4, &[4_294_967_291, 2, 3, 10]);
    test(
        &[10, 10, 10, 10],
        &[100, 101, 102],
        10,
        &[4_294_967_190, 101, 102, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[123, 456],
        789,
        &[4_294_966_416, 456, 10, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_neg_xor_limb_neg_to_out_fail_1() {
    limbs_neg_xor_limb_neg_to_out(&mut [], &[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_neg_xor_limb_neg_to_out_fail_2() {
    limbs_neg_xor_limb_neg_to_out(&mut [10], &[10, 10], 10);
}

#[test]
fn test_xor_signed_limb() {
    let test = |u, v: SignedLimb, out| {
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
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() ^ v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        #[cfg(feature = "32_bit_limbs")]
        {
            let n = rug::Integer::from_str(u).unwrap() ^ v;
            assert_eq!(n.to_string(), out);
        }

        let n = v ^ Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v ^ &Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        assert_eq!(
            integer_xor_signed_limb_alt_1(&Integer::from_str(u).unwrap(), v).to_string(),
            out
        );
        assert_eq!(
            integer_xor_signed_limb_alt_2(&Integer::from_str(u).unwrap(), v).to_string(),
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

    test("0", -123, "-123");
    test("123", -456, "-445");
    test("999999999999", -123, "-999999999878");
    test("1000000000000", -123, "-1000000000123");
    test("1000000000001", -123, "-1000000000124");
    test("12345678987654321", -456, "-12345678987654519");
    test("12345678987654321", -987_654_321, "-12345678815534082");

    test("-123", -456, "445");
    test("-999999999999", -123, "999999999876");
    test("-1000000000000", -123, "999999999877");
    test("-1000000000001", -123, "1000000000122");
    test("-12345678987654321", -456, "12345678987654519");
    test("-12345678987654321", -987_654_321, "12345678815534080");

    test("-4294967294", 2, "-4294967296");
    test("-4294967296", 2, "-4294967294");
    test(
        "79228162514264337593543950335",
        -1,
        "-79228162514264337593543950336",
    );
    test("-68169720922112", -470_806_536, "68169250115576");
}

#[test]
fn limbs_pos_xor_limb_neg_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_unsigned,
        |&(ref limbs, limb)| {
            let limbs_out = limbs_pos_xor_limb_neg(limbs, limb);
            let n = Integer::from(Natural::from_limbs_asc(limbs))
                ^ Integer::from_owned_twos_complement_limbs_asc(vec![limb, Limb::MAX]);
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_out),
                Natural::checked_from(-n).unwrap()
            );
        },
    );
}

#[test]
fn limbs_pos_xor_limb_neg_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_2,
        |&(ref out_limbs, ref in_limbs, limb)| {
            let mut out_limbs = out_limbs.to_vec();
            let old_out_limbs = out_limbs.clone();
            limbs_pos_xor_limb_neg_to_out(&mut out_limbs, in_limbs, limb);
            let n = Integer::from(Natural::from_limbs_asc(in_limbs))
                ^ Integer::from_owned_twos_complement_limbs_asc(vec![limb, Limb::MAX]);
            let len = in_limbs.len();
            let mut limbs = Natural::checked_from(-n).unwrap().into_limbs_asc();
            limbs.resize(len, 0);
            assert_eq!(limbs, &out_limbs[..len]);
            assert_eq!(&out_limbs[len..], &old_out_limbs[len..]);
        },
    );
}

#[test]
fn limbs_slice_pos_xor_limb_neg_in_place_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_unsigned,
        |&(ref limbs, limb)| {
            let mut mut_limbs = limbs.to_vec();
            let carry = limbs_slice_pos_xor_limb_neg_in_place(&mut mut_limbs, limb);
            let n = Integer::from(Natural::from_limbs_asc(&limbs))
                ^ Integer::from_owned_twos_complement_limbs_asc(vec![limb, Limb::MAX]);
            if carry {
                let result_limbs = Natural::checked_from(-n).unwrap().to_limbs_asc();
                assert_eq!(mut_limbs, &result_limbs[..limbs.len()]);
            } else {
                assert_eq!(
                    Natural::from_owned_limbs_asc(mut_limbs),
                    Natural::checked_from(-n).unwrap()
                );
            }
        },
    );
}

#[test]
fn limbs_vec_pos_xor_limb_neg_in_place_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_unsigned,
        |&(ref limbs, limb)| {
            let mut mut_limbs = limbs.to_vec();
            limbs_vec_pos_xor_limb_neg_in_place(&mut mut_limbs, limb);
            let n = Integer::from(Natural::from_limbs_asc(&limbs))
                ^ Integer::from_owned_twos_complement_limbs_asc(vec![limb, Limb::MAX]);
            assert_eq!(
                Natural::from_owned_limbs_asc(mut_limbs),
                Natural::checked_from(-n).unwrap()
            );
        },
    );
}

#[test]
fn limbs_neg_xor_limb_neg_properties() {
    test_properties(pairs_of_limb_vec_and_limb_var_1, |&(ref limbs, limb)| {
        let limbs_out = limbs_neg_xor_limb_neg(limbs, limb);
        let n = -Natural::from_limbs_asc(limbs)
            ^ Integer::from_owned_twos_complement_limbs_asc(vec![limb, Limb::MAX]);
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_out),
            Natural::checked_from(n).unwrap()
        );
    });
}

#[test]
fn limbs_neg_xor_limb_neg_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_3,
        |&(ref out_limbs, ref in_limbs, limb)| {
            let mut out_limbs = out_limbs.to_vec();
            let old_out_limbs = out_limbs.clone();
            limbs_neg_xor_limb_neg_to_out(&mut out_limbs, in_limbs, limb);
            let n = -Natural::from_limbs_asc(in_limbs)
                ^ Integer::from_owned_twos_complement_limbs_asc(vec![limb, Limb::MAX]);
            let len = in_limbs.len();
            let mut limbs = Natural::checked_from(n).unwrap().into_limbs_asc();
            limbs.resize(len, 0);
            assert_eq!(limbs, &out_limbs[..len]);
            assert_eq!(&out_limbs[len..], &old_out_limbs[len..]);
        },
    );
}

#[test]
fn limbs_neg_xor_limb_neg_in_place_properties() {
    test_properties(pairs_of_limb_vec_and_limb_var_1, |&(ref limbs, limb)| {
        let mut mut_limbs = limbs.to_vec();
        limbs_neg_xor_limb_neg_in_place(&mut mut_limbs, limb);
        let n = -Natural::from_limbs_asc(&limbs)
            ^ Integer::from_owned_twos_complement_limbs_asc(vec![limb, Limb::MAX]);
        let mut expected_limbs = Natural::checked_from(n).unwrap().into_limbs_asc();
        expected_limbs.resize(limbs.len(), 0);
        assert_eq!(mut_limbs, expected_limbs);
    });
}

#[test]
fn or_signed_limb_properties() {
    test_properties(
        pairs_of_integer_and_signed,
        |&(ref n, i): &(Integer, SignedLimb)| {
            let mut mut_n = n.clone();
            mut_n ^= i;
            assert!(mut_n.is_valid());
            let result = mut_n;

            #[cfg(feature = "32_bit_limbs")]
            {
                let mut rug_n = integer_to_rug_integer(n);
                rug_n ^= i;
                assert_eq!(rug_integer_to_integer(&rug_n), result);
            }

            let result_alt = n ^ i;
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = n.clone() ^ i;
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = i ^ n;
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = i ^ n.clone();
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            assert_eq!(integer_xor_signed_limb_alt_1(&n, i), result);
            assert_eq!(integer_xor_signed_limb_alt_2(&n, i), result);

            assert_eq!(n ^ Integer::from(i), result);
            assert_eq!(Integer::from(i) ^ n, result);

            assert_eq!(&result ^ i, *n);

            #[cfg(feature = "32_bit_limbs")]
            assert_eq!(
                rug_integer_to_integer(&(integer_to_rug_integer(n) ^ i)),
                result
            );
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n ^ 0 as Limb, *n);
        assert_eq!(0 as Limb ^ n, *n);
        assert_eq!(n ^ -1 as SignedLimb, !n);
        assert_eq!(-1 as SignedLimb ^ n, !n);
    });

    test_properties(signeds, |&i: &SignedLimb| {
        assert_eq!(&Integer::ZERO ^ i, i);
        assert_eq!(i ^ &Integer::ZERO, i);
        assert_eq!(&Integer::NEGATIVE_ONE ^ i, !i);
        assert_eq!(i ^ &Integer::NEGATIVE_ONE, !i);
    });

    test_properties(pairs_of_signeds::<SignedLimb>, |&(i, j)| {
        assert_eq!(Integer::from(i) ^ j, i ^ j);
        assert_eq!(i ^ Integer::from(j), i ^ j);
    });
}
