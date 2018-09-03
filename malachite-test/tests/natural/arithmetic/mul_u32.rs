use common::test_properties;
use malachite_base::num::{DivMod, One, Zero};
use malachite_nz::natural::arithmetic::mul_u32::{
    limbs_mul_limb, limbs_mul_limb_to_out, limbs_slice_mul_limb_in_place,
    limbs_vec_mul_limb_in_place,
};
use malachite_nz::natural::Natural;
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::base::{
    pairs_of_unsigned_vec_and_unsigned, triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1,
    unsigneds,
};
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_unsigned};
use malachite_test::natural::arithmetic::mul_u32::num_mul_u32;
use num::BigUint;
use rug::{self, Assign};
use std::str::FromStr;

#[test]
fn test_limbs_mul_limb() {
    let test = |limbs: &[u32], limb: u32, out: &[u32]| {
        assert_eq!(limbs_mul_limb(limbs, limb), out);
    };
    test(&[], 0, &[]);
    test(&[], 5, &[]);
    test(&[6, 7], 2, &[12, 14]);
    test(&[100, 101, 102], 10, &[1000, 1010, 1020]);
    test(&[123, 456], 789, &[97_047, 359_784]);
    test(&[0xffff_ffff, 5], 2, &[4_294_967_294, 11]);
    test(&[0xffff_ffff], 2, &[4_294_967_294, 1]);
    test(&[0xffff_ffff], 0xffff_ffff, &[1, 4_294_967_294]);
}

#[test]
fn test_limbs_mul_limb_to_out() {
    let test = |limbs_out_before: &[u32],
                limbs_in: &[u32],
                limb: u32,
                carry: u32,
                limbs_out_after: &[u32]| {
        let mut limbs_out = limbs_out_before.to_vec();
        assert_eq!(limbs_mul_limb_to_out(&mut limbs_out, limbs_in, limb), carry);
        assert_eq!(limbs_out, limbs_out_after);
    };
    test(&[10, 10, 10, 10], &[], 0, 0, &[10, 10, 10, 10]);
    test(&[10, 10, 10, 10], &[], 5, 0, &[10, 10, 10, 10]);
    test(&[10, 10, 10, 10], &[6, 7], 2, 0, &[12, 14, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[100, 101, 102],
        10,
        0,
        &[1000, 1010, 1020, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[123, 456],
        789,
        0,
        &[97_047, 359_784, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[0xffff_ffff, 5],
        2,
        0,
        &[4_294_967_294, 11, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[0xffff_ffff],
        2,
        1,
        &[4_294_967_294, 10, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[0xffff_ffff],
        0xffff_ffff,
        4_294_967_294,
        &[1, 10, 10, 10],
    );
}

#[test]
#[should_panic(expected = "assertion failed: out_limbs.len() >= len")]
fn limbs_mul_limb_to_out_fail() {
    limbs_mul_limb_to_out(&mut [10], &[10, 10], 10);
}

#[test]
fn test_limbs_slice_mul_limb_in_place() {
    let test = |limbs: &[u32], limb: u32, carry: u32, out: &[u32]| {
        let mut limbs = limbs.to_vec();
        assert_eq!(limbs_slice_mul_limb_in_place(&mut limbs, limb), carry);
        assert_eq!(limbs, out);
    };
    test(&[], 0, 0, &[]);
    test(&[], 5, 0, &[]);
    test(&[6, 7], 2, 0, &[12, 14]);
    test(&[100, 101, 102], 10, 0, &[1000, 1010, 1020]);
    test(&[123, 456], 789, 0, &[97_047, 359_784]);
    test(&[0xffff_ffff, 5], 2, 0, &[4_294_967_294, 11]);
    test(&[0xffff_ffff], 2, 1, &[4_294_967_294]);
    test(&[0xffff_ffff], 0xffff_ffff, 4_294_967_294, &[1]);
}

#[test]
fn test_limbs_vec_mul_limb_in_place() {
    let test = |limbs: &[u32], limb: u32, out: &[u32]| {
        let mut limbs = limbs.to_vec();
        limbs_vec_mul_limb_in_place(&mut limbs, limb);
        assert_eq!(limbs, out);
    };
    test(&[], 0, &[]);
    test(&[], 5, &[]);
    test(&[6, 7], 2, &[12, 14]);
    test(&[100, 101, 102], 10, &[1000, 1010, 1020]);
    test(&[123, 456], 789, &[97_047, 359_784]);
    test(&[0xffff_ffff, 5], 2, &[4_294_967_294, 11]);
    test(&[0xffff_ffff], 2, &[4_294_967_294, 1]);
    test(&[0xffff_ffff], 0xffff_ffff, &[1, 4_294_967_294]);
}

#[test]
fn test_mul_u32() {
    let test = |u, v: u32, out| {
        let mut n = Natural::from_str(u).unwrap();
        n *= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from_str(u).unwrap();
        n *= v;
        assert_eq!(n.to_string(), out);

        let n = Natural::from_str(u).unwrap() * v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = num_mul_u32(BigUint::from_str(u).unwrap(), v);
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() * v;
        assert_eq!(n.to_string(), out);

        let n = &Natural::from_str(u).unwrap() * v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v * Natural::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v * rug::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);

        let n = v * &Natural::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from(0);
        n.assign(v * &rug::Integer::from_str(u).unwrap());
        assert_eq!(n.to_string(), out);
    };
    test("0", 0, "0");
    test("0", 123, "0");
    test("123", 0, "0");
    test("1", 123, "123");
    test("123", 1, "123");
    test("123", 456, "56088");
    test("1000000000000", 0, "0");
    test("1000000000000", 1, "1000000000000");
    test("1000000000000", 123, "123000000000000");
    test("4294967295", 2, "8589934590");
    test("18446744073709551615", 2, "36893488147419103230");
}

#[test]
fn limbs_mul_limb_properties() {
    test_properties(pairs_of_unsigned_vec_and_unsigned, |&(ref limbs, limb)| {
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_mul_limb(limbs, limb)),
            Natural::from_limbs_asc(limbs) * limb
        );
    });
}

#[test]
fn limbs_mul_limb_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1,
        |&(ref out_limbs, ref in_limbs, limb)| {
            let mut out_limbs = out_limbs.to_vec();
            let old_out_limbs = out_limbs.clone();
            let carry = limbs_mul_limb_to_out(&mut out_limbs, in_limbs, limb);
            let n = Natural::from_limbs_asc(in_limbs) * limb;
            let len = in_limbs.len();
            let mut limbs = n.into_limbs_asc();
            assert_eq!(carry != 0, limbs.len() == len + 1);
            limbs.resize(len, 0);
            assert_eq!(limbs, &out_limbs[..len]);
            assert_eq!(&out_limbs[len..], &old_out_limbs[len..]);
        },
    );
}

#[test]
fn limbs_slice_mul_limb_in_place_properties() {
    test_properties(pairs_of_unsigned_vec_and_unsigned, |&(ref limbs, limb)| {
        let mut limbs = limbs.to_vec();
        let old_limbs = limbs.clone();
        let carry = limbs_slice_mul_limb_in_place(&mut limbs, limb);
        let n = Natural::from_limbs_asc(&old_limbs) * limb;
        let mut expected_limbs = n.into_limbs_asc();
        assert_eq!(carry != 0, expected_limbs.len() == limbs.len() + 1);
        expected_limbs.resize(limbs.len(), 0);
        assert_eq!(limbs, expected_limbs);
    });
}

#[test]
fn limbs_vec_mul_limb_in_place_properties() {
    test_properties(pairs_of_unsigned_vec_and_unsigned, |&(ref limbs, limb)| {
        let mut limbs = limbs.to_vec();
        let old_limbs = limbs.clone();
        limbs_vec_mul_limb_in_place(&mut limbs, limb);
        let n = Natural::from_limbs_asc(&old_limbs) * limb;
        assert_eq!(Natural::from_owned_limbs_asc(limbs), n);
    });
}

#[test]
fn mul_u32_properties() {
    test_properties(
        pairs_of_natural_and_unsigned,
        |&(ref n, u): &(Natural, u32)| {
            let mut mut_n = n.clone();
            mut_n *= u;
            assert!(mut_n.is_valid());
            let product = mut_n;

            let mut rug_n = natural_to_rug_integer(n);
            rug_n *= u;
            assert_eq!(rug_integer_to_natural(&rug_n), product);

            let product_alt = n * u;
            assert!(product_alt.is_valid());
            assert_eq!(product_alt, product);

            let product_alt = n.clone() * u;
            assert!(product_alt.is_valid());
            assert_eq!(product_alt, product);

            let product_alt = u * n;
            assert!(product_alt.is_valid());
            assert_eq!(product_alt, product);

            let product_alt = u * n.clone();
            assert!(product_alt.is_valid());
            assert_eq!(product_alt, product);

            assert_eq!(n * Natural::from(u), product);
            assert_eq!(Natural::from(u) * n, product);

            assert_eq!(
                biguint_to_natural(&num_mul_u32(natural_to_biguint(n), u)),
                product
            );
            assert_eq!(
                rug_integer_to_natural(&(natural_to_rug_integer(n) * u)),
                product
            );

            if *n != 0 && u != 0 {
                assert!(product >= *n);
                assert!(product >= u);
            }
            if u != 0 {
                let (q, r) = product.div_mod(u);
                assert_eq!(q, *n);
                assert_eq!(r, 0);
            }
            //TODO assert_eq!(product / n, u);
        },
    );

    #[allow(unknown_lints, erasing_op, identity_op)]
    test_properties(naturals, |n| {
        assert_eq!(n * 0u32, 0);
        assert_eq!(0u32 * n, 0);
        assert_eq!(n * 1u32, *n);
        assert_eq!(1u32 * n, *n);
        assert_eq!(n * 2u32, n << 1);
        assert_eq!(2u32 * n, n << 1);
    });

    test_properties(unsigneds, |&u: &u32| {
        assert_eq!(Natural::ZERO * u, 0);
        assert_eq!(u * Natural::ZERO, 0);
        assert_eq!(Natural::ONE * u, u);
        assert_eq!(u * Natural::ONE, u);
    });
}
