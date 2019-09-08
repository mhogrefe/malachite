use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{CheckedSubMul, SubMul, SubMulAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::arithmetic::sub_mul_limb::{
    limbs_sub_mul_limb_greater, limbs_sub_mul_limb_greater_in_place_left,
    limbs_sub_mul_limb_greater_in_place_right, limbs_sub_mul_limb_same_length_in_place_left,
    limbs_sub_mul_limb_same_length_in_place_right,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use common::test_properties;
use malachite_test::inputs::base::{
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_7,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_limb_var_1, pairs_of_naturals_var_1,
    triples_of_natural_natural_and_limb_var_1,
};

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_mul_limb_greater() {
    let test = |xs_before: &[Limb], ys_before: &[Limb], limb: Limb, result: &[Limb], borrow| {
        let o_result = limbs_sub_mul_limb_greater(xs_before, ys_before, limb);
        if borrow == 0 {
            assert_eq!(o_result.unwrap(), result);
        } else {
            assert!(o_result.is_none());
        }
        let mut xs = xs_before.to_vec();
        assert_eq!(
            limbs_sub_mul_limb_greater_in_place_left(&mut xs, ys_before, limb),
            borrow
        );
        assert_eq!(xs, result);
        let mut ys = ys_before.to_vec();
        assert_eq!(
            limbs_sub_mul_limb_greater_in_place_right(xs_before, &mut ys, limb),
            borrow
        );
        assert_eq!(ys, result);
    };
    test(&[], &[], 4, &[], 0);
    test(&[123, 456], &[], 4, &[123, 456], 0);
    test(&[123, 456], &[123], 0, &[123, 456], 0);
    test(&[123, 456], &[123], 4, &[4294966927, 455], 0);
    test(&[123, 456], &[123], 0xffff_ffff, &[246, 333], 0);
    test(&[123, 456], &[0, 123], 0xffff_ffff, &[123, 579], 123);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_limb_greater_fail() {
    limbs_sub_mul_limb_greater(&[10], &[10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_limb_greater_in_place_left_fail() {
    limbs_sub_mul_limb_greater_in_place_left(&mut [10], &[10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_limb_greater_in_place_right_fail() {
    limbs_sub_mul_limb_greater_in_place_right(&[10], &mut vec![10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_mul_limb_same_length() {
    let test = |xs_before: &[Limb], ys_before: &[Limb], limb: Limb, result: &[Limb], borrow| {
        let mut xs = xs_before.to_vec();
        assert_eq!(
            limbs_sub_mul_limb_same_length_in_place_left(&mut xs, ys_before, limb),
            borrow
        );
        assert_eq!(xs, result);
        let mut ys = ys_before.to_vec();
        assert_eq!(
            limbs_sub_mul_limb_same_length_in_place_right(xs_before, &mut ys, limb),
            borrow
        );
        assert_eq!(ys, result);
    };
    test(&[], &[], 4, &[], 0);
    test(&[123, 456], &[0, 0], 4, &[123, 456], 0);
    test(&[123, 456], &[123, 0], 0, &[123, 456], 0);
    test(&[123, 456], &[123, 0], 4, &[4294966927, 455], 0);
    test(&[123, 456], &[123, 0], 0xffff_ffff, &[246, 333], 0);
    test(&[123, 456], &[0, 123], 0xffff_ffff, &[123, 579], 123);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_limb_same_length_in_place_left_fail() {
    limbs_sub_mul_limb_same_length_in_place_left(&mut [10, 10], &[10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_limb_same_length_in_place_right_fail() {
    limbs_sub_mul_limb_same_length_in_place_right(&[10, 10], &mut [10], 10);
}

#[test]
fn test_sub_mul_limb() {
    let test = |u, v, c: Limb, out: &str| {
        let mut n = Natural::from_str(u).unwrap();
        n.sub_mul_assign(Natural::from_str(v).unwrap(), c);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n.sub_mul_assign(&Natural::from_str(v).unwrap(), c);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u)
            .unwrap()
            .sub_mul(Natural::from_str(v).unwrap(), c);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u)
            .unwrap()
            .sub_mul(&Natural::from_str(v).unwrap(), c);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).sub_mul(Natural::from_str(v).unwrap(), c);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).sub_mul(&Natural::from_str(v).unwrap(), c);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0", 0, "0");
    test("0", "0", 123, "0");
    test("123", "0", 5, "123");
    test("123", "5", 1, "118");
    test("15", "3", 4, "3");
    test("1000000000000", "0", 123, "1000000000000");
    test("1000000000000", "1", 123, "999999999877");
    test("1000000000000", "123", 1, "999999999877");
    test("1000000000000", "123", 100, "999999987700");
    test("1000000000000", "100", 123, "999999987700");
    test("1000000000000", "65536", 0x1_0000, "995705032704");
    test("1000000000000", "1000000000000", 0, "1000000000000");
    test("1000000000000", "1000000000000", 1, "0");
    test("4294967296", "1", 1, "4294967295");
    test(
        "1000000000000000000000",
        "1000000000000",
        1_000_000_000,
        "0",
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_fail_1() {
    let mut x = Natural::from_str("123").unwrap();
    x.sub_mul_assign(Natural::from_str("5").unwrap(), 100 as Limb);
}

#[test]
#[should_panic]
fn sub_mul_assign_fail_2() {
    let mut x = Natural::from_str("1000000000000").unwrap();
    x.sub_mul_assign(Natural::from_str("1000000000000").unwrap(), 100 as Limb);
}

#[test]
#[should_panic]
fn sub_mul_assign_ref_fail_1() {
    let mut x = Natural::from_str("123").unwrap();
    x.sub_mul_assign(&Natural::from_str("5").unwrap(), 100 as Limb);
}

#[test]
#[should_panic]
fn sub_mul_assign_ref_fail_2() {
    let mut x = Natural::from_str("1000000000000").unwrap();
    x.sub_mul_assign(&Natural::from_str("1000000000000").unwrap(), 100 as Limb);
}

#[test]
#[should_panic]
fn sub_mul_fail_1() {
    Natural::from_str("123")
        .unwrap()
        .sub_mul(Natural::from_str("5").unwrap(), 100 as Limb);
}

#[test]
#[should_panic]
fn sub_mul_fail_2() {
    Natural::from_str("1000000000000")
        .unwrap()
        .sub_mul(Natural::from_str("1000000000000").unwrap(), 100 as Limb);
}

#[test]
#[should_panic]
fn sub_mul_val_ref_fail_1() {
    Natural::from_str("123")
        .unwrap()
        .sub_mul(&Natural::from_str("5").unwrap(), 100 as Limb);
}

#[test]
#[should_panic]
fn sub_mul_val_ref_fail_2() {
    Natural::from_str("1000000000000")
        .unwrap()
        .sub_mul(&Natural::from_str("1000000000000").unwrap(), 100 as Limb);
}

#[test]
#[should_panic]
fn sub_mul_ref_val_fail_1() {
    (&Natural::from_str("123").unwrap()).sub_mul(Natural::from_str("5").unwrap(), 100 as Limb);
}

#[test]
#[should_panic]
fn sub_mul_ref_val_fail_2() {
    (&Natural::from_str("1000000000000").unwrap())
        .sub_mul(Natural::from_str("1000000000000").unwrap(), 100 as Limb);
}

#[test]
#[should_panic]
fn sub_mul_ref_ref_fail_1() {
    (&Natural::from_str("123").unwrap()).sub_mul(&Natural::from_str("5").unwrap(), 100 as Limb);
}

#[test]
#[should_panic]
fn sub_mul_ref_ref_fail_2() {
    (&Natural::from_str("1000000000000").unwrap())
        .sub_mul(&Natural::from_str("1000000000000").unwrap(), 100 as Limb);
}

#[test]
fn limbs_sub_mul_limb_greater_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1,
        |&(ref a, ref b, c)| {
            assert_eq!(
                limbs_sub_mul_limb_greater(a, b, c).map(Natural::from_owned_limbs_asc),
                Natural::from_limbs_asc(a).checked_sub_mul(Natural::from_limbs_asc(b), c)
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
            Natural::from_limbs_asc(a_old).sub_mul(Natural::from_limbs_asc(b), c)
        );
    } else {
        let mut extended_a = a_old.to_vec();
        extended_a.push(0);
        extended_a.push(1);
        let mut expected_limbs = Natural::from_owned_limbs_asc(extended_a)
            .sub_mul(Natural::from_limbs_asc(b), c)
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
                Natural::from_limbs_asc($a).sub_mul(Natural::from_limbs_asc(b_old), $c)
            );
        } else {
            let mut extended_a = $a.to_vec();
            extended_a.push(0);
            extended_a.push(1);
            let mut expected_limbs = Natural::from_owned_limbs_asc(extended_a)
                .sub_mul(Natural::from_limbs_asc(b_old), $c)
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
fn sub_mul_limb_properties() {
    test_properties(
        triples_of_natural_natural_and_limb_var_1,
        |&(ref a, ref b, c): &(Natural, Natural, Limb)| {
            let mut mut_a = a.clone();
            mut_a.sub_mul_assign(b, c);
            assert!(mut_a.is_valid());
            let result = mut_a;

            let mut mut_a = a.clone();
            mut_a.sub_mul_assign(b.clone(), c);
            assert!(mut_a.is_valid());
            assert_eq!(mut_a, result);

            let result_alt = a.sub_mul(b, c);
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = a.sub_mul(b, c.clone());
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = a.sub_mul(b.clone(), c);
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = a.sub_mul(b.clone(), c.clone());
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            assert_eq!(a - b * c, result);
            assert_eq!(a.sub_mul(b, &Natural::from(c)), result);
            assert_eq!(a.checked_sub_mul(b, c), Some(result))
        },
    );

    test_properties(naturals, |n| {
        assert_eq!(n.sub_mul(n, 1 as Limb), Natural::ZERO);
    });

    test_properties(
        pairs_of_natural_and_limb_var_1,
        |&(ref n, c): &(Natural, Limb)| {
            assert_eq!(n.sub_mul(Natural::ZERO, c), *n);
            assert_eq!(n.sub_mul(Natural::ONE, c), n - c);
            assert_eq!((n * c).sub_mul(n, c), Natural::ZERO);
        },
    );

    test_properties(pairs_of_naturals_var_1, |&(ref a, ref b)| {
        assert_eq!(a.sub_mul(b, 0 as Limb), *a);
        assert_eq!(a.sub_mul(b, 1 as Limb), a - b);
    });
}
