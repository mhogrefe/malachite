use malachite_base::num::arithmetic::traits::{CheckedSubMul, SubMul, SubMulAssign, UnsignedAbs};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_nz::integer::arithmetic::sub_mul::{
    limbs_overflowing_sub_mul, limbs_overflowing_sub_mul_in_place_left,
    limbs_overflowing_sub_mul_limb, limbs_overflowing_sub_mul_limb_in_place_either,
    limbs_overflowing_sub_mul_limb_in_place_left, limbs_overflowing_sub_mul_limb_in_place_right,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::SignedLimb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    triples_of_signeds, triples_of_signeds_var_3,
    triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3,
    triples_of_unsigned_vec_var_29,
};
use malachite_test::inputs::integer::{integers, pairs_of_integers, triples_of_integers};

#[test]
fn limbs_overflowing_sub_mul_limb_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3,
        |&(ref a, ref b, c)| {
            let (result_limbs, sign) = limbs_overflowing_sub_mul_limb(a, b, c);
            let expected_result = Integer::from(Natural::from_limbs_asc(a))
                .sub_mul(Integer::from(Natural::from_limbs_asc(b)), Integer::from(c));
            assert_eq!(sign, expected_result >= 0);
            assert_eq!(
                Natural::from_owned_limbs_asc(result_limbs),
                expected_result.unsigned_abs()
            );
        },
    );
}

#[test]
fn limbs_overflowing_sub_mul_limb_in_place_left_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3,
        |&(ref a, ref b, c)| {
            let a_old = a;
            let mut a = a_old.to_vec();
            let sign = limbs_overflowing_sub_mul_limb_in_place_left(&mut a, b, c);
            let expected_result = Integer::from(Natural::from_limbs_asc(a_old))
                .sub_mul(Integer::from(Natural::from_limbs_asc(b)), Integer::from(c));
            assert_eq!(sign, expected_result >= 0);
            assert_eq!(
                Natural::from_owned_limbs_asc(a),
                expected_result.unsigned_abs()
            );
        },
    );
}

#[test]
fn limbs_overflowing_sub_mul_limb_in_place_right_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3,
        |&(ref a, ref b, c)| {
            let b_old = b;
            let mut b = b_old.to_vec();
            let sign = limbs_overflowing_sub_mul_limb_in_place_right(a, &mut b, c);
            let expected_result = Integer::from(Natural::from_limbs_asc(a)).sub_mul(
                Integer::from(Natural::from_limbs_asc(b_old)),
                Integer::from(c),
            );
            assert_eq!(sign, expected_result >= 0);
            assert_eq!(
                Natural::from_owned_limbs_asc(b),
                expected_result.unsigned_abs()
            );
        },
    );
}

#[test]
fn limbs_overflowing_sub_mul_limb_in_place_either_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3,
        |&(ref a, ref b, c)| {
            let a_old = a;
            let b_old = b;
            let mut a = a_old.to_vec();
            let mut b = b_old.to_vec();
            let (right, sign) = limbs_overflowing_sub_mul_limb_in_place_either(&mut a, &mut b, c);
            let expected_result = Integer::from(Natural::from_limbs_asc(a_old)).sub_mul(
                Integer::from(Natural::from_limbs_asc(b_old)),
                Integer::from(c),
            );
            let result = Natural::from_owned_limbs_asc(if right {
                assert_eq!(a_old, &a);
                b
            } else {
                assert_eq!(b_old, &b);
                a
            });
            assert_eq!(sign, expected_result >= 0);
            assert_eq!(result, expected_result.unsigned_abs());
        },
    );
}

#[test]
fn limbs_overflowing_sub_mul_properties() {
    test_properties(triples_of_unsigned_vec_var_29, |&(ref a, ref b, ref c)| {
        let (result_limbs, sign) = limbs_overflowing_sub_mul(a, b, c);
        let expected_result = Integer::from(Natural::from_limbs_asc(a)).sub_mul(
            Integer::from(Natural::from_limbs_asc(b)),
            Integer::from(Natural::from_limbs_asc(c)),
        );
        if expected_result != 0 {
            assert_eq!(sign, expected_result >= 0);
        }
        assert_eq!(
            Natural::from_owned_limbs_asc(result_limbs),
            expected_result.unsigned_abs()
        );
    });
}

#[test]
fn limbs_overflowing_sub_mul_in_place_left_properties() {
    test_properties(triples_of_unsigned_vec_var_29, |&(ref a, ref b, ref c)| {
        let a_old = a;
        let mut a = a_old.to_vec();
        let sign = limbs_overflowing_sub_mul_in_place_left(&mut a, b, c);
        let expected_result = Integer::from(Natural::from_limbs_asc(a_old)).sub_mul(
            Integer::from(Natural::from_limbs_asc(b)),
            Integer::from(Natural::from_limbs_asc(c)),
        );
        if expected_result != 0 {
            assert_eq!(sign, expected_result >= 0);
        }
        assert_eq!(
            Natural::from_owned_limbs_asc(a),
            expected_result.unsigned_abs()
        );
    });
}

#[test]
fn sub_mul_properties() {
    test_properties(triples_of_integers, |&(ref a, ref b, ref c)| {
        let mut mut_a = a.clone();
        mut_a.sub_mul_assign(b.clone(), c.clone());
        assert!(mut_a.is_valid());
        let result = mut_a;

        let mut mut_a = a.clone();
        mut_a.sub_mul_assign(b.clone(), c);
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let mut mut_a = a.clone();
        mut_a.sub_mul_assign(b, c.clone());
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let mut mut_a = a.clone();
        mut_a.sub_mul_assign(b, c);
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let result_alt = a.clone().sub_mul(b.clone(), c.clone());
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result = a.clone().sub_mul(b.clone(), c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result = a.clone().sub_mul(b, c.clone());
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result = a.clone().sub_mul(b, c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result = a.sub_mul(b, c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!(a - b * c, result);
        assert_eq!(a.sub_mul(c, b), result);
        assert_eq!(a.sub_mul(&(-b), &(-c)), result);
        assert_eq!((-a).sub_mul(&(-b), c), -&result);
        assert_eq!((-a).sub_mul(b, -c), -result);
    });

    test_properties(integers, |a| {
        assert_eq!(a.sub_mul(a, &Integer::ONE), 0);
        assert_eq!(a.sub_mul(&(-a), &Integer::NEGATIVE_ONE), 0);
    });

    test_properties(pairs_of_integers, |&(ref a, ref b)| {
        assert_eq!(a.sub_mul(&Integer::ZERO, b), *a);
        assert_eq!(a.sub_mul(&Integer::ONE, b), a - b);
        assert_eq!(Integer::ZERO.sub_mul(a, b), -a * b);
        assert_eq!(a.sub_mul(b, &Integer::ZERO), *a);
        assert_eq!(a.sub_mul(b, &Integer::ONE), a - b);
        assert_eq!((a * b).sub_mul(a, b), 0);
        assert_eq!((a * b).sub_mul(-a, -b), 0);
    });

    test_properties(triples_of_signeds_var_3::<SignedLimb>, |&(x, y, z)| {
        assert_eq!(
            SignedLimb::from(x).sub_mul(SignedLimb::from(y), SignedLimb::from(z)),
            Integer::from(x).sub_mul(Integer::from(y), Integer::from(z))
        );
    });

    test_properties(triples_of_signeds::<SignedLimb>, |&(x, y, z)| {
        let result = Integer::from(x).sub_mul(Integer::from(y), Integer::from(z));
        assert_eq!(
            SignedLimb::from(x)
                .checked_sub_mul(SignedLimb::from(y), SignedLimb::from(z))
                .is_some(),
            SignedLimb::convertible_from(result)
        );
    });
}
