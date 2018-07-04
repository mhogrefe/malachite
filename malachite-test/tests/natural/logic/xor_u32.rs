use common::test_properties;
use malachite_base::num::Zero;
use malachite_nz::natural::logic::xor_u32::{
    limbs_xor_limb, limbs_xor_limb_in_place, limbs_xor_limb_to_out,
};
use malachite_nz::natural::Natural;
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::base::{
    pairs_of_nonempty_unsigned_vec_and_unsigned,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_2, unsigneds,
};
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_unsigned};
use malachite_test::natural::logic::xor_u32::{
    natural_xor_u32_alt_1, natural_xor_u32_alt_2, num_or_u32,
};
use num::BigUint;
use rug::{self, Assign};
use std::str::FromStr;

#[test]
fn test_limbs_xor_limb() {
    let test = |limbs: &[u32], limb: u32, out: &[u32]| {
        assert_eq!(limbs_xor_limb(limbs, limb), out);
    };
    test(&[6, 7], 2, &[4, 7]);
    test(&[100, 101, 102], 10, &[110, 101, 102]);
    test(&[123, 456], 789, &[878, 456]);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
fn limbs_xor_limb_fail() {
    limbs_xor_limb(&[], 10);
}

#[test]
fn test_limbs_xor_limb_to_out() {
    let test = |limbs_out_before: &[u32], limbs_in: &[u32], limb: u32, limbs_out_after: &[u32]| {
        let mut limbs_out = limbs_out_before.to_vec();
        limbs_xor_limb_to_out(&mut limbs_out, limbs_in, limb);
        assert_eq!(limbs_out, limbs_out_after);
    };
    test(&[10, 10, 10, 10], &[6, 7], 2, &[4, 7, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[100, 101, 102],
        10,
        &[110, 101, 102, 10],
    );
    test(&[10, 10, 10, 10], &[123, 456], 789, &[878, 456, 10, 10]);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
fn limbs_xor_limb_to_out_fail_1() {
    limbs_xor_limb_to_out(&mut [], &[], 10);
}

#[test]
#[should_panic(expected = "index 2 out of range for slice of length 1")]
fn limbs_xor_limb_to_out_fail_2() {
    limbs_xor_limb_to_out(&mut [10], &[10, 10], 10);
}

#[test]
fn test_limbs_xor_limb_in_place() {
    let test = |limbs: &[u32], limb: u32, out: &[u32]| {
        let mut limbs = limbs.to_vec();
        limbs_xor_limb_in_place(&mut limbs, limb);
        assert_eq!(limbs, out);
    };
    test(&[6, 7], 2, &[4, 7]);
    test(&[100, 101, 102], 10, &[110, 101, 102]);
    test(&[123, 456], 789, &[878, 456]);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
fn limbs_xor_limb_in_place_fail() {
    limbs_xor_limb_in_place(&mut [], 10);
}

#[test]
fn test_or_i32() {
    let test = |u, v: u32, out| {
        let mut n = Natural::from_str(u).unwrap();
        n ^= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from_str(u).unwrap();
        n ^= v;
        assert_eq!(n.to_string(), out);

        let n = Natural::from_str(u).unwrap() ^ v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() ^ v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = num_or_u32(BigUint::from_str(u).unwrap(), v);
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() ^ v;
        assert_eq!(n.to_string(), out);

        let n = v ^ Natural::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v ^ &Natural::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        assert_eq!(
            natural_xor_u32_alt_1(&Natural::from_str(u).unwrap(), v).to_string(),
            out
        );
        assert_eq!(
            natural_xor_u32_alt_2(&Natural::from_str(u).unwrap(), v).to_string(),
            out
        );

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
}

#[test]
fn limbs_xor_limb_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_unsigned,
        |&(ref limbs, limb)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_xor_limb(limbs, limb)),
                Natural::from_limbs_asc(limbs) ^ limb
            );
        },
    );
}

#[test]
fn limbs_xor_limb_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_2,
        |&(ref out_limbs, ref in_limbs, limb)| {
            let mut out_limbs = out_limbs.to_vec();
            let old_out_limbs = out_limbs.clone();
            limbs_xor_limb_to_out(&mut out_limbs, in_limbs, limb);
            let len = in_limbs.len();
            assert_eq!(
                Natural::from_limbs_asc(&out_limbs[..len]),
                Natural::from_limbs_asc(in_limbs) ^ limb
            );
            assert_eq!(&out_limbs[len..], &old_out_limbs[len..]);
        },
    );
}

#[test]
fn limbs_xor_limb_in_place_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_unsigned,
        |&(ref limbs, limb)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            limbs_xor_limb_in_place(&mut limbs, limb);
            assert_eq!(
                Natural::from_limbs_asc(&limbs),
                Natural::from_limbs_asc(&old_limbs) ^ limb
            );
        },
    );
}

#[test]
fn xor_u32_properties() {
    test_properties(
        pairs_of_natural_and_unsigned,
        |&(ref n, u): &(Natural, u32)| {
            let mut mut_n = n.clone();
            mut_n ^= u;
            assert!(mut_n.is_valid());
            let result = mut_n;

            let mut rug_n = natural_to_rug_integer(n);
            rug_n ^= u;
            assert_eq!(rug_integer_to_natural(&rug_n), result);

            let result_alt = n ^ u;
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = n.clone() ^ u;
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = u ^ n;
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = u ^ n.clone();
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            assert_eq!(natural_xor_u32_alt_1(&n, u), result);
            assert_eq!(natural_xor_u32_alt_2(&n, u), result);

            //TODO assert_eq!(n ^ Natural::from(u), result);
            //TODO assert_eq!(Natural::from(u) ^ n, result);

            assert_eq!(&result ^ u, *n);

            assert_eq!(
                biguint_to_natural(&num_or_u32(natural_to_biguint(n), u)),
                result
            );
            assert_eq!(
                rug_integer_to_natural(&(natural_to_rug_integer(n) ^ u)),
                result
            );
        },
    );

    test_properties(naturals, |n| {
        assert_eq!(n ^ 0u32, *n);
        assert_eq!(0u32 ^ n, *n);
    });

    test_properties(unsigneds, |&u: &u32| {
        assert_eq!(&Natural::ZERO ^ u, u);
        assert_eq!(u ^ &Natural::ZERO, u);
    });
}
