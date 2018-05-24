use common::test_properties;
use malachite_base::num::Zero;
use malachite_nz::natural::logic::or_u32::{
    limbs_or_limb, limbs_or_limb_in_place, limbs_or_limb_to_out,
};
use malachite_nz::natural::Natural;
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::base::{
    pairs_of_nonempty_unsigned_vec_and_unsigned,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1, unsigneds,
};
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_unsigned};
use malachite_test::natural::logic::or_u32::{
    natural_or_u32_alt_1, natural_or_u32_alt_2, num_or_u32,
};
use num::BigUint;
use rug::{self, Assign};
use std::str::FromStr;

#[test]
fn test_limbs_or_limb() {
    let test = |limbs: &[u32], limb: u32, out: &[u32]| {
        assert_eq!(limbs_or_limb(limbs, limb), out);
    };
    test(&[6, 7], 2, &[6, 7]);
    test(&[100, 101, 102], 10, &[110, 101, 102]);
    test(&[123, 456], 789, &[895, 456]);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
fn limbs_or_limb_fail() {
    limbs_or_limb(&[], 10);
}

#[test]
fn test_limbs_or_limb_to_out() {
    let test = |limbs_out_before: &[u32], limbs_in: &[u32], limb: u32, limbs_out_after: &[u32]| {
        let mut limbs_out = limbs_out_before.to_vec();
        limbs_or_limb_to_out(&mut limbs_out, limbs_in, limb);
        assert_eq!(limbs_out, limbs_out_after);
    };
    test(&[10, 10, 10, 10], &[6, 7], 2, &[6, 7, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[100, 101, 102],
        10,
        &[110, 101, 102, 10],
    );
    test(&[10, 10, 10, 10], &[123, 456], 789, &[895, 456, 10, 10]);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
fn limbs_or_limb_to_out_fail_1() {
    limbs_or_limb_to_out(&mut [], &[], 10);
}

#[test]
#[should_panic(expected = "index 2 out of range for slice of length 1")]
fn limbs_or_limb_to_out_fail_2() {
    limbs_or_limb_to_out(&mut [10], &[10, 10], 10);
}

#[test]
fn test_limbs_or_limb_in_place() {
    let test = |limbs: &[u32], limb: u32, out: &[u32]| {
        let mut limbs = limbs.to_vec();
        limbs_or_limb_in_place(&mut limbs, limb);
        assert_eq!(limbs, out);
    };
    test(&[6, 7], 2, &[6, 7]);
    test(&[100, 101, 102], 10, &[110, 101, 102]);
    test(&[123, 456], 789, &[895, 456]);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
fn limbs_or_limb_in_place_fail() {
    limbs_or_limb_in_place(&mut [], 10);
}

#[test]
fn test_or_i32() {
    let test = |u, v: u32, out| {
        let mut n = Natural::from_str(u).unwrap();
        n |= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from_str(u).unwrap();
        n |= v;
        assert_eq!(n.to_string(), out);

        let n = Natural::from_str(u).unwrap() | v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() | v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = num_or_u32(BigUint::from_str(u).unwrap(), v);
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() | v;
        assert_eq!(n.to_string(), out);

        let n = v | Natural::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v | &Natural::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v | rug::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);

        let mut n = rug::Integer::from(0);
        n.assign(v | &rug::Integer::from_str(u).unwrap());
        assert_eq!(n.to_string(), out);
    };

    test("0", 0, "0");
    test("0", 123, "123");
    test("123", 0, "123");
    test("123", 456, "507");
    test("999999999999", 123, "999999999999");
    test("1000000000000", 123, "1000000000123");
    test("1000000000001", 123, "1000000000123");
    test("12345678987654321", 456, "12345678987654649");
    test("12345678987654321", 987_654_321, "12345679395421361");
}

#[test]
fn limbs_or_limb_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_unsigned,
        |&(ref limbs, limb)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_or_limb(limbs, limb)),
                Natural::from_limbs_asc(limbs) | limb
            );
        },
    );
}

#[test]
fn limbs_or_limb_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1,
        |&(ref out_limbs, ref in_limbs, limb)| {
            let mut out_limbs = out_limbs.to_vec();
            let old_out_limbs = out_limbs.clone();
            limbs_or_limb_to_out(&mut out_limbs, in_limbs, limb);
            let len = in_limbs.len();
            assert_eq!(
                Natural::from_limbs_asc(&out_limbs[0..len]),
                Natural::from_limbs_asc(in_limbs) | limb
            );
            assert_eq!(&out_limbs[len..], &old_out_limbs[len..]);
        },
    );
}

#[test]
fn limbs_or_limb_in_place_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_unsigned,
        |&(ref limbs, limb)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            limbs_or_limb_in_place(&mut limbs, limb);
            assert_eq!(
                Natural::from_limbs_asc(&limbs),
                Natural::from_limbs_asc(&old_limbs) | limb
            );
        },
    );
}

#[test]
fn or_u32_properties() {
    test_properties(
        pairs_of_natural_and_unsigned,
        |&(ref n, u): &(Natural, u32)| {
            let mut mut_n = n.clone();
            mut_n |= u;
            assert!(mut_n.is_valid());
            let result = mut_n;

            let mut rug_n = natural_to_rug_integer(n);
            rug_n |= u;
            assert_eq!(rug_integer_to_natural(&rug_n), result);

            assert_eq!(n | u, result);
            assert_eq!(u | n, result);
            assert_eq!(natural_or_u32_alt_1(&n, u), result);
            assert_eq!(natural_or_u32_alt_2(&n, u), result);

            assert_eq!(n | Natural::from(u), result);
            assert_eq!(Natural::from(u) | n, result);

            assert_eq!(&result | u, result);

            assert_eq!(
                biguint_to_natural(&num_or_u32(natural_to_biguint(n), u)),
                result
            );
            assert_eq!(
                rug_integer_to_natural(&(natural_to_rug_integer(n) | u)),
                result
            );

            assert!(result >= *n);
            assert!(result >= u);

            let ones = result.count_ones();
            assert!(ones >= u64::from(n.count_ones()));
            assert!(ones >= u64::from(u.count_ones()));
        },
    );

    test_properties(naturals, |n| {
        assert_eq!(n | 0u32, *n);
        assert_eq!(0u32 | n, *n);
    });

    test_properties(unsigneds, |&u: &u32| {
        assert_eq!(&Natural::ZERO | u, u);
        assert_eq!(u | &Natural::ZERO, u);
    });
}
