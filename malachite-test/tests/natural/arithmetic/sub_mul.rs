use malachite_base::num::arithmetic::traits::{CheckedSub, CheckedSubMul, SubMul, SubMulAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::arithmetic::sub_mul::{
    limbs_sub_mul, limbs_sub_mul_in_place_left, limbs_sub_mul_limb_greater,
    limbs_sub_mul_limb_greater_in_place_left, limbs_sub_mul_limb_greater_in_place_right,
    limbs_sub_mul_limb_same_length_in_place_left, limbs_sub_mul_limb_same_length_in_place_right,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_7, triples_of_unsigned_vec_var_28,
    triples_of_unsigneds_var_4,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_naturals, pairs_of_naturals_var_1, triples_of_naturals_var_1,
};

#[test]
fn limbs_sub_mul_limb_greater_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1,
        |&(ref a, ref b, c)| {
            assert_eq!(
                limbs_sub_mul_limb_greater(a, b, c).map(Natural::from_owned_limbs_asc),
                Natural::from_limbs_asc(a)
                    .checked_sub_mul(Natural::from_limbs_asc(b), Natural::from(c))
            );
        },
    );
}

fn limbs_sub_mul_limb_in_place_left_helper(
    f: &mut dyn FnMut(&mut [Limb], &[Limb], Limb) -> Limb,
    a: &Vec<Limb>,
    b: &Vec<Limb>,
    c: Limb,
) {
    let a_old = a;
    let mut a = a.to_vec();
    let borrow = f(&mut a, b, c);
    if borrow == 0 {
        assert_eq!(
            Natural::from_owned_limbs_asc(a),
            Natural::from_limbs_asc(a_old).sub_mul(Natural::from_limbs_asc(b), Natural::from(c))
        );
    } else {
        let mut extended_a = a_old.to_vec();
        extended_a.push(0);
        extended_a.push(1);
        let mut expected_limbs = Natural::from_owned_limbs_asc(extended_a)
            .sub_mul(Natural::from_limbs_asc(b), Natural::from(c))
            .into_limbs_asc();
        assert_eq!(expected_limbs.pop().unwrap(), borrow.wrapping_neg());
        assert_eq!(a, expected_limbs);
    }
}

#[test]
fn limbs_sub_mul_limb_same_length_in_place_left_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_7,
        |&(ref a, ref b, c)| {
            limbs_sub_mul_limb_in_place_left_helper(
                &mut limbs_sub_mul_limb_same_length_in_place_left,
                a,
                b,
                c,
            )
        },
    );
}

#[test]
fn limbs_sub_mul_limb_greater_in_place_left_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1,
        |&(ref a, ref b, c)| {
            limbs_sub_mul_limb_in_place_left_helper(
                &mut limbs_sub_mul_limb_greater_in_place_left,
                a,
                b,
                c,
            )
        },
    );
}

macro_rules! limbs_sub_mul_limb_in_place_right_helper {
    ($f: ident, $a: ident, $b: ident, $c: ident) => {{
        let b_old = $b;
        let mut b = $b.to_vec();
        let borrow = $f($a, &mut b, $c);
        if borrow == 0 {
            assert_eq!(
                Natural::from_owned_limbs_asc(b),
                Natural::from_limbs_asc($a)
                    .sub_mul(Natural::from_limbs_asc(b_old), Natural::from($c))
            );
        } else {
            let mut extended_a = $a.to_vec();
            extended_a.push(0);
            extended_a.push(1);
            let mut expected_limbs = Natural::from_owned_limbs_asc(extended_a)
                .sub_mul(Natural::from_limbs_asc(b_old), Natural::from($c))
                .into_limbs_asc();
            assert_eq!(expected_limbs.pop().unwrap(), borrow.wrapping_neg());
            assert_eq!(b, expected_limbs);
        }
    }};
}

#[test]
fn limbs_sub_mul_limb_same_length_in_place_right_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_7,
        |&(ref a, ref b, c)| {
            limbs_sub_mul_limb_in_place_right_helper!(
                limbs_sub_mul_limb_same_length_in_place_right,
                a,
                b,
                c
            )
        },
    );
}

#[test]
fn limbs_sub_mul_limb_greater_in_place_right_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_7,
        |&(ref a, ref b, c)| {
            limbs_sub_mul_limb_in_place_right_helper!(
                limbs_sub_mul_limb_greater_in_place_right,
                a,
                b,
                c
            )
        },
    );
}

#[test]
fn limbs_sub_mul_properties() {
    test_properties(triples_of_unsigned_vec_var_28, |&(ref a, ref b, ref c)| {
        let expected = limbs_sub_mul(a, b, c).map(Natural::from_owned_limbs_asc);
        assert_eq!(
            expected,
            Natural::from_limbs_asc(a)
                .checked_sub_mul(Natural::from_limbs_asc(b), Natural::from_limbs_asc(c))
        );
    });
}

#[test]
fn limbs_sub_mul_in_place_left_properties() {
    test_properties(triples_of_unsigned_vec_var_28, |&(ref a, ref b, ref c)| {
        let a_old = a;
        let mut a = a.to_vec();
        let expected = if limbs_sub_mul_in_place_left(&mut a, b, c) {
            None
        } else {
            Some(Natural::from_owned_limbs_asc(a))
        };
        assert_eq!(
            expected,
            Natural::from_limbs_asc(a_old)
                .checked_sub_mul(Natural::from_limbs_asc(b), Natural::from_limbs_asc(c))
        );
    });
}

#[test]
fn sub_mul_properties() {
    test_properties(triples_of_naturals_var_1, |&(ref a, ref b, ref c)| {
        let mut mut_a = a.clone();
        mut_a.sub_mul_assign(b, c);
        assert!(mut_a.is_valid());
        let result = mut_a;

        let mut mut_a = a.clone();
        mut_a.sub_mul_assign(b, c.clone());
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let mut mut_a = a.clone();
        mut_a.sub_mul_assign(b.clone(), c);
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let mut mut_a = a.clone();
        mut_a.sub_mul_assign(b.clone(), c.clone());
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let result_alt = a.sub_mul(b, c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().sub_mul(b, c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().sub_mul(b, c.clone());
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().sub_mul(b.clone(), c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result_alt = a.clone().sub_mul(b.clone(), c.clone());
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!(a - b * c, result);
        assert_eq!(a.checked_sub(b * c), Some(result));
    });

    test_properties(naturals, |n| {
        assert_eq!(n.sub_mul(n, &Natural::ONE), 0);
    });

    test_properties(pairs_of_naturals, |&(ref a, ref b)| {
        assert_eq!(a.sub_mul(&Natural::ZERO, b), *a);
        assert_eq!(a.sub_mul(b, &Natural::ZERO), *a);
        assert_eq!((a * b).sub_mul(a, b), 0);
    });

    test_properties(pairs_of_naturals_var_1, |&(ref a, ref b)| {
        assert_eq!(a.sub_mul(&Natural::ONE, b), a - b);
        assert_eq!(a.sub_mul(b, &Natural::ONE), a - b);
    });

    test_properties(triples_of_unsigneds_var_4::<Limb>, |&(x, y, z)| {
        assert_eq!(
            Limb::from(x).sub_mul(Limb::from(y), Limb::from(z)),
            Natural::from(x).sub_mul(Natural::from(y), Natural::from(z))
        );
    });
}
