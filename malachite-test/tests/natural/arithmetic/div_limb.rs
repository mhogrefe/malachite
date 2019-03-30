use common::test_properties;
use malachite_base::num::{DivRem, One, Zero};
use malachite_nz::natural::arithmetic::div_limb::{
    limbs_div_limb, limbs_div_limb_in_place, limbs_div_limb_to_out,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_test::common::{biguint_to_natural, natural_to_biguint};
#[cfg(feature = "32_bit_limbs")]
use malachite_test::common::{natural_to_rug_integer, rug_integer_to_natural};
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_positive_unsigned, pairs_of_unsigned_vec_and_positive_unsigned_var_1,
    positive_unsigneds, triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_1,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_positive_limb_var_1, pairs_of_natural_and_positive_unsigned,
    pairs_of_natural_and_unsigned_var_2, pairs_of_unsigned_and_positive_natural,
};
use num::BigUint;
#[cfg(feature = "32_bit_limbs")]
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_div_limb_and_limbs_div_limb_in_place() {
    let test = |limbs: &[Limb], limb: Limb, quotient: &[Limb]| {
        assert_eq!(limbs_div_limb(limbs, limb), quotient);

        let mut limbs = limbs.to_vec();
        limbs_div_limb_in_place(&mut limbs, limb);
        assert_eq!(limbs, quotient);
    };
    test(&[0, 0], 2, &[0, 0]);
    test(&[6, 7], 1, &[6, 7]);
    test(&[6, 7], 2, &[2_147_483_651, 3]);
    test(&[100, 101, 102], 10, &[1_288_490_198, 858_993_469, 10]);
    test(&[123, 456], 789, &[2_482_262_467, 0]);
    test(&[0xffff_ffff, 0xffff_ffff], 2, &[0xffff_ffff, 0x7fff_ffff]);
    test(&[0xffff_ffff, 0xffff_ffff], 3, &[0x5555_5555, 0x5555_5555]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_div_limb_fail_1() {
    limbs_div_limb(&[10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_div_limb_fail_2() {
    limbs_div_limb(&[10, 10], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_div_limb_in_place_fail_1() {
    limbs_div_limb_in_place(&mut [10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_div_limb_in_place_fail_2() {
    limbs_div_limb_in_place(&mut [10, 10], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_div_limb_to_out() {
    let test =
        |limbs_out_before: &[Limb], limbs_in: &[Limb], limb: Limb, limbs_out_after: &[Limb]| {
            let mut limbs_out = limbs_out_before.to_vec();
            limbs_div_limb_to_out(&mut limbs_out, limbs_in, limb);
            assert_eq!(limbs_out, limbs_out_after);
        };
    test(&[10, 10, 10, 10], &[0, 0], 2, &[0, 0, 10, 10]);
    test(&[10, 10, 10, 10], &[6, 7], 1, &[6, 7, 10, 10]);
    test(&[10, 10, 10, 10], &[6, 7], 2, &[2_147_483_651, 3, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[100, 101, 102],
        10,
        &[1_288_490_198, 858_993_469, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[123, 456],
        789,
        &[2_482_262_467, 0, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[0xffff_ffff, 0xffff_ffff],
        2,
        &[0xffff_ffff, 0x7fff_ffff, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[0xffff_ffff, 0xffff_ffff],
        3,
        &[0x5555_5555, 0x5555_5555, 10, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_div_limb_to_out_fail_1() {
    limbs_div_limb_to_out(&mut [10], &[10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_div_limb_to_out_fail_2() {
    limbs_div_limb_to_out(&mut [10, 10], &[10, 10], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_div_limb_to_out_fail_3() {
    limbs_div_limb_to_out(&mut [10], &[10, 10], 10);
}

#[test]
fn test_div_limb() {
    let test = |u, v: Limb, quotient| {
        let mut n = Natural::from_str(u).unwrap();
        n /= v;
        assert_eq!(n.to_string(), quotient);
        assert!(n.is_valid());

        let q = Natural::from_str(u).unwrap() / v;
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = &Natural::from_str(u).unwrap() / v;
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = Natural::from_str(u).unwrap().div_rem(v).0;
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = BigUint::from_str(u).unwrap() / v;
        assert_eq!(q.to_string(), quotient);

        #[cfg(feature = "32_bit_limbs")]
        {
            let q = rug::Integer::from_str(u).unwrap() / v;
            assert_eq!(q.to_string(), quotient);
        }
    };
    test("0", 1, "0");
    test("0", 123, "0");
    test("1", 1, "1");
    test("123", 1, "123");
    test("123", 123, "1");
    test("123", 456, "0");
    test("456", 123, "3");
    test("4294967295", 1, "4294967295");
    test("4294967295", 4_294_967_295, "1");
    test("1000000000000", 1, "1000000000000");
    test("1000000000000", 3, "333333333333");
    test("1000000000000", 123, "8130081300");
    test("1000000000000", 4_294_967_295, "232");
    test("1000000000000000000000000", 1, "1000000000000000000000000");
    test("1000000000000000000000000", 3, "333333333333333333333333");
    test("1000000000000000000000000", 123, "8130081300813008130081");
    test(
        "1000000000000000000000000",
        4_294_967_295,
        "232830643708079",
    );
}

#[test]
#[should_panic]
fn div_assign_limb_fail() {
    let mut n = Natural::from(10u32);
    n /= 0 as Limb;
}

#[test]
#[allow(unused_must_use)]
#[should_panic]
fn div_limb_fail() {
    Natural::from(10u32) / 0 as Limb;
}

#[test]
#[allow(unused_must_use)]
#[should_panic]
fn div_limb_ref_fail() {
    &Natural::from(10u32) / 0 as Limb;
}

#[test]
fn test_limb_div_natural() {
    let test = |u: Limb, v, quotient| {
        let mut mut_u = u;
        mut_u /= Natural::from_str(v).unwrap();
        assert_eq!(mut_u, quotient);

        let mut mut_u = u;
        mut_u /= &Natural::from_str(v).unwrap();
        assert_eq!(mut_u, quotient);

        assert_eq!(u / Natural::from_str(v).unwrap(), quotient);
        assert_eq!(u / &Natural::from_str(v).unwrap(), quotient);

        assert_eq!(u.div_rem(Natural::from_str(v).unwrap()).0, quotient);
    };
    test(0, "1", 0);
    test(0, "123", 0);
    test(1, "1", 1);
    test(123, "1", 123);
    test(123, "123", 1);
    test(123, "456", 0);
    test(456, "123", 3);
    test(4_294_967_295, "1", 4_294_967_295);
    test(4_294_967_295, "4294967295", 1);
    test(0, "1000000000000", 0);
    test(123, "1000000000000", 0);
}

#[test]
#[allow(unused_must_use)]
#[should_panic]
fn limb_div_natural_fail() {
    10 as Limb / Natural::ZERO;
}

#[test]
#[allow(unused_must_use)]
#[should_panic]
fn limb_div_natural_ref_fail() {
    10 as Limb / &Natural::ZERO;
}

#[test]
#[should_panic]
fn limb_div_assign_natural_fail() {
    let mut n: Limb = 10;
    n /= Natural::ZERO;
}

#[test]
#[should_panic]
fn limb_div_assign_natural_ref_fail() {
    let mut n: Limb = 10;
    n /= &Natural::ZERO;
}

#[test]
fn limbs_div_limb_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_positive_unsigned_var_1,
        |&(ref limbs, limb)| {
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs_div_limb(limbs, limb)),
                Natural::from_limbs_asc(limbs) / limb
            );
        },
    );
}

#[test]
fn limbs_div_limb_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_1,
        |&(ref out, ref in_limbs, limb)| {
            let mut out = out.to_vec();
            let old_out = out.clone();
            limbs_div_limb_to_out(&mut out, in_limbs, limb);
            let len = in_limbs.len();
            assert_eq!(
                Natural::from_limbs_asc(&out[..len]),
                Natural::from_limbs_asc(in_limbs) / limb
            );
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_div_limb_in_place_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_positive_unsigned_var_1,
        |&(ref limbs, limb)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            limbs_div_limb_in_place(&mut limbs, limb);
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs),
                Natural::from_limbs_asc(&old_limbs) / limb
            );
        },
    );
}

fn div_limb_properties_helper(n: &Natural, u: Limb) {
    let mut mut_n = n.clone();
    mut_n /= u;
    assert!(mut_n.is_valid());
    let quotient = mut_n;

    let quotient_alt = n / u;
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    let quotient_alt = n.clone() / u;
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);

    assert_eq!(n.div_rem(u).0, quotient);

    //TODO assert_eq!(n / Natural::from(u), quotient);

    assert_eq!(biguint_to_natural(&(natural_to_biguint(n) / u)), quotient);
    #[cfg(feature = "32_bit_limbs")]
    assert_eq!(
        rug_integer_to_natural(&(natural_to_rug_integer(n) / u)),
        quotient
    );
    assert!(n - quotient * u < u);
}

#[test]
fn div_limb_properties() {
    test_properties(
        pairs_of_natural_and_positive_unsigned,
        |&(ref n, u): &(Natural, Limb)| {
            div_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_natural_and_unsigned_var_2,
        |&(ref n, u): &(Natural, Limb)| {
            div_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_natural_and_positive_limb_var_1,
        |&(ref n, u): &(Natural, Limb)| {
            div_limb_properties_helper(n, u);
        },
    );

    test_properties(
        pairs_of_unsigned_and_positive_natural,
        |&(u, ref n): &(Limb, Natural)| {
            let mut mut_u = u;
            mut_u /= n;
            let quotient = mut_u;

            let mut mut_u = u;
            mut_u /= n.clone();
            assert_eq!(mut_u, quotient);

            let quotient_alt = u / n;
            assert_eq!(quotient_alt, quotient);

            let quotient_alt = u / n.clone();
            assert_eq!(quotient_alt, quotient);

            assert_eq!(u.div_rem(n).0, quotient);
            assert!(u - quotient * n < *n);
        },
    );

    test_properties(
        pairs_of_unsigned_and_positive_unsigned::<Limb>,
        |&(x, y)| {
            let quotient = x / y;
            assert_eq!(quotient, Natural::from(x) / y);
            assert_eq!(quotient, x / Natural::from(y));
        },
    );

    test_properties(naturals, |n| {
        assert_eq!(n / 1 as Limb, *n);
    });

    test_properties(positive_unsigneds, |&u: &Limb| {
        assert_eq!(Natural::ZERO / u, 0);
        if u > 1 {
            assert_eq!(1 / u, 0);
        }
        assert_eq!(u / Natural::ONE, u);
        assert_eq!(u / Natural::from(u), 1);
        assert_eq!(Natural::from(u) / u, 1);
    });
}
