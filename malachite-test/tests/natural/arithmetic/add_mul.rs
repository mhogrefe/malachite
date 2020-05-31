use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign, CheckedAddMul};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_nz::natural::arithmetic::add_mul::{
    limbs_add_mul, limbs_add_mul_in_place_left, limbs_add_mul_limb,
    limbs_slice_add_mul_limb_same_length_in_place_left,
    limbs_slice_add_mul_limb_same_length_in_place_right, limbs_vec_add_mul_limb_in_place_either,
    limbs_vec_add_mul_limb_in_place_left, limbs_vec_add_mul_limb_in_place_right,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_7, triples_of_unsigned_vec_var_27,
    triples_of_unsigneds, triples_of_unsigneds_var_3,
};
use malachite_test::inputs::natural::{pairs_of_naturals, triples_of_naturals};

#[test]
fn limbs_add_mul_limb_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3,
        |&(ref a, ref b, c)| {
            assert_eq!(
                limbs_add_mul_limb(a, b, c),
                Natural::from_limbs_asc(a)
                    .add_mul(Natural::from_limbs_asc(b), Natural::from(c))
                    .into_limbs_asc()
            );
        },
    );
}

#[test]
fn limbs_slice_add_mul_limb_same_length_in_place_left_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_7,
        |&(ref a, ref b, c)| {
            let a_old = a;
            let mut a = a_old.to_vec();
            let carry = limbs_slice_add_mul_limb_same_length_in_place_left(&mut a, b, c);
            let len = b.len();
            let mut result = a[..len].to_vec();
            result.push(carry);
            assert_eq!(
                Natural::from_owned_limbs_asc(result),
                Natural::from_limbs_asc(&a_old[..len])
                    .add_mul(Natural::from_limbs_asc(b), Natural::from(c))
            );
            assert_eq!(&a[len..], &a_old[len..]);
        },
    );
}

#[test]
fn limbs_slice_add_mul_limb_same_length_in_place_right_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_7,
        |&(ref a, ref b, c)| {
            let b_old = b;
            let mut b = b.to_vec();
            let carry = limbs_slice_add_mul_limb_same_length_in_place_right(a, &mut b, c);
            b.push(carry);
            assert_eq!(
                Natural::from_owned_limbs_asc(b),
                Natural::from_limbs_asc(a)
                    .add_mul(Natural::from_limbs_asc(b_old), Natural::from(c))
            );
        },
    );
}

#[test]
fn limbs_vec_add_mul_limb_in_place_left_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3,
        |&(ref a, ref b, c)| {
            let a_old = a;
            let mut a = a_old.to_vec();
            limbs_vec_add_mul_limb_in_place_left(&mut a, b, c);
            assert_eq!(
                a,
                Natural::from_limbs_asc(a_old)
                    .add_mul(Natural::from_limbs_asc(b), Natural::from(c))
                    .into_limbs_asc()
            );
        },
    );
}

#[test]
fn limbs_vec_add_mul_limb_in_place_right_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3,
        |&(ref a, ref b, c)| {
            let b_old = b;
            let mut b = b_old.to_vec();
            limbs_vec_add_mul_limb_in_place_right(a, &mut b, c);
            assert_eq!(
                b,
                Natural::from_limbs_asc(a)
                    .add_mul(Natural::from_limbs_asc(b_old), Natural::from(c))
                    .into_limbs_asc()
            );
        },
    );
}

#[test]
fn limbs_vec_add_mul_limb_in_place_either_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3,
        |&(ref a, ref b, c)| {
            let a_old = a;
            let b_old = b;
            let mut a = a_old.to_vec();
            let mut b = b_old.to_vec();
            let result = if limbs_vec_add_mul_limb_in_place_either(&mut a, &mut b, c) {
                assert_eq!(a_old, &a);
                b
            } else {
                assert_eq!(b_old, &b);
                a
            };
            assert_eq!(
                result,
                Natural::from_limbs_asc(a_old)
                    .add_mul(Natural::from_limbs_asc(b_old), Natural::from(c))
                    .into_limbs_asc()
            );
        },
    );
}

#[test]
fn limbs_add_mul_properties() {
    test_properties(triples_of_unsigned_vec_var_27, |&(ref a, ref b, ref c)| {
        assert_eq!(
            limbs_add_mul(a, b, c),
            Natural::from_limbs_asc(a)
                .add_mul(Natural::from_limbs_asc(b), Natural::from_limbs_asc(c))
                .into_limbs_asc()
        );
    });
}

#[test]
fn limbs_add_mul_in_place_left_properties() {
    test_properties(triples_of_unsigned_vec_var_27, |&(ref a, ref b, ref c)| {
        let a_old = a;
        let mut a = a.to_vec();
        limbs_add_mul_in_place_left(&mut a, b, c);
        assert_eq!(
            a,
            Natural::from_limbs_asc(a_old)
                .add_mul(Natural::from_limbs_asc(b), Natural::from_limbs_asc(c))
                .into_limbs_asc()
        );
    });
}

#[test]
fn add_mul_properties() {
    test_properties(triples_of_naturals, |&(ref a, ref b, ref c)| {
        let mut mut_a = a.clone();
        mut_a.add_mul_assign(b.clone(), c.clone());
        assert!(mut_a.is_valid());
        let result = mut_a;

        let mut mut_a = a.clone();
        mut_a.add_mul_assign(b.clone(), c);
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let mut mut_a = a.clone();
        mut_a.add_mul_assign(b, c.clone());
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let mut mut_a = a.clone();
        mut_a.add_mul_assign(b, c);
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let result_alt = a.clone().add_mul(b.clone(), c.clone());
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().add_mul(b.clone(), c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().add_mul(b, c.clone());
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().add_mul(b, c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.add_mul(b, c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!(a + b * c, result);
        assert_eq!(a.add_mul(c, b), result);
    });

    test_properties(pairs_of_naturals, |&(ref a, ref b)| {
        assert_eq!(a.add_mul(&Natural::ZERO, b), *a);
        assert_eq!(a.add_mul(&Natural::ONE, b), a + b);
        assert_eq!(Natural::ZERO.add_mul(a, b), a * b);
        assert_eq!(a.add_mul(b, &Natural::ZERO), *a);
        assert_eq!(a.add_mul(b, &Natural::ONE), a + b);
    });

    test_properties(triples_of_unsigneds_var_3::<Limb>, |&(x, y, z)| {
        assert_eq!(
            Limb::from(x).add_mul(Limb::from(y), Limb::from(z)),
            Natural::from(x).add_mul(Natural::from(y), Natural::from(z))
        );
    });

    test_properties(triples_of_unsigneds::<Limb>, |&(x, y, z)| {
        let result = Natural::from(x).add_mul(Natural::from(y), Natural::from(z));
        assert_eq!(
            Limb::from(x)
                .checked_add_mul(Limb::from(y), Limb::from(z))
                .is_some(),
            Limb::convertible_from(result)
        );
    });
}
