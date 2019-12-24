use std::str::FromStr;

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
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_naturals, pairs_of_naturals_var_1, triples_of_naturals_var_1,
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

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_mul_and_limbs_sub_mul_in_place_left() {
    let test = |xs_before: &[Limb], ys: &[Limb], zs: &[Limb], result: Option<Vec<Limb>>| {
        assert_eq!(limbs_sub_mul(xs_before, ys, zs), result);
        let mut xs = xs_before.to_vec();
        let result_alt = if limbs_sub_mul_in_place_left(&mut xs, ys, zs) {
            None
        } else {
            Some(xs)
        };
        assert_eq!(result, result_alt);
    };
    test(&[123, 456, 789], &[123, 789], &[321, 654], None);
    test(
        &[123, 456, 789, 1],
        &[123, 789],
        &[321, 654],
        Some(vec![4294927936, 4294634040, 4294452078, 0]),
    );
    test(
        &[123, 456, 789, 987, 654],
        &[123, 789],
        &[321, 654],
        Some(vec![4294927936, 4294634040, 4294452078, 986, 654]),
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_fail_1() {
    limbs_sub_mul(&[10, 10, 10], &[10], &[10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_fail_2() {
    limbs_sub_mul(&[10, 10, 10], &[10, 10], &[10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_fail_3() {
    limbs_sub_mul(&[10, 10], &[10, 10], &[10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_in_place_left_fail_1() {
    let xs = &mut [10, 10, 10];
    limbs_sub_mul_in_place_left(xs, &[10], &[10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_in_place_left_fail_2() {
    let xs = &mut [10, 10, 10];
    limbs_sub_mul_in_place_left(xs, &[10, 10], &[10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_mul_in_place_left_fail_3() {
    let xs = &mut [10, 10];
    limbs_sub_mul_in_place_left(xs, &[10, 10], &[10, 10]);
}

#[test]
fn test_sub_mul() {
    let test = |u, v, w, out: &str| {
        let mut n = Natural::from_str(u).unwrap();
        n.sub_mul_assign(Natural::from_str(v).unwrap(), Natural::from_str(w).unwrap());
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let mut n = Natural::from_str(u).unwrap();
        n.sub_mul_assign(
            Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
        );
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let mut n = Natural::from_str(u).unwrap();
        n.sub_mul_assign(
            &Natural::from_str(v).unwrap(),
            Natural::from_str(w).unwrap(),
        );
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let mut n = Natural::from_str(u).unwrap();
        n.sub_mul_assign(
            &Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
        );
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = Natural::from_str(u)
            .unwrap()
            .sub_mul(Natural::from_str(v).unwrap(), Natural::from_str(w).unwrap());
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = Natural::from_str(u).unwrap().sub_mul(
            Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
        );
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = Natural::from_str(u).unwrap().sub_mul(
            &Natural::from_str(v).unwrap(),
            Natural::from_str(w).unwrap(),
        );
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = Natural::from_str(u).unwrap().sub_mul(
            &Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
        );
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let n = (&Natural::from_str(u).unwrap()).sub_mul(
            &Natural::from_str(v).unwrap(),
            &Natural::from_str(w).unwrap(),
        );
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0", "0");
    test("0", "0", "123", "0");
    test("123", "0", "5", "123");
    test("123", "5", "1", "118");
    test("15", "3", "4", "3");
    test("1000000000000", "0", "123", "1000000000000");
    test("1000000000000", "1", "123", "999999999877");
    test("1000000000000", "123", "1", "999999999877");
    test("1000000000000", "123", "100", "999999987700");
    test("1000000000000", "100", "123", "999999987700");
    test("1000000000000", "65536", "65536", "995705032704");
    test("1000000000000", "1000000000000", "0", "1000000000000");
    test("1000000000000", "1000000000000", "1", "0");
    test("4294967296", "1", "1", "4294967295");
    test(
        "1000000000000000000000000",
        "1000000000000",
        "1000000000000",
        "0",
    );
    test(
        "1000000000001000000000000",
        "1000000000000",
        "1000000000000",
        "1000000000000",
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_fail_1() {
    let mut x = Natural::from_str("123").unwrap();
    x.sub_mul_assign(
        Natural::from_str("5").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_fail_2() {
    let mut x = Natural::from_str("1000000000000").unwrap();
    x.sub_mul_assign(
        Natural::from_str("1000000000000").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_val_ref_fail_1() {
    let mut x = Natural::from_str("123").unwrap();
    x.sub_mul_assign(
        Natural::from_str("5").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_val_ref_fail_2() {
    let mut x = Natural::from_str("1000000000000").unwrap();
    x.sub_mul_assign(
        Natural::from_str("1000000000000").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_ref_val_fail_1() {
    let mut x = Natural::from_str("123").unwrap();
    x.sub_mul_assign(
        &Natural::from_str("5").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_ref_val_fail_2() {
    let mut x = Natural::from_str("1000000000000").unwrap();
    x.sub_mul_assign(
        &Natural::from_str("1000000000000").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_ref_ref_fail_1() {
    let mut x = Natural::from_str("123").unwrap();
    x.sub_mul_assign(
        &Natural::from_str("5").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_ref_ref_fail_2() {
    let mut x = Natural::from_str("1000000000000").unwrap();
    x.sub_mul_assign(
        &Natural::from_str("1000000000000").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_fail_1() {
    Natural::from_str("123").unwrap().sub_mul(
        Natural::from_str("5").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_fail_2() {
    Natural::from_str("1000000000000").unwrap().sub_mul(
        Natural::from_str("1000000000000").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_val_val_ref_fail_1() {
    Natural::from_str("123").unwrap().sub_mul(
        Natural::from_str("5").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_val_val_ref_fail_2() {
    Natural::from_str("1000000000000").unwrap().sub_mul(
        Natural::from_str("1000000000000").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_val_ref_val_fail_1() {
    Natural::from_str("123").unwrap().sub_mul(
        &Natural::from_str("5").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_val_ref_val_fail_2() {
    Natural::from_str("1000000000000").unwrap().sub_mul(
        &Natural::from_str("1000000000000").unwrap(),
        Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_val_ref_ref_fail_1() {
    Natural::from_str("123").unwrap().sub_mul(
        &Natural::from_str("5").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_val_ref_ref_fail_2() {
    Natural::from_str("1000000000000").unwrap().sub_mul(
        &Natural::from_str("1000000000000").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_ref_ref_ref_fail_1() {
    (&Natural::from_str("123").unwrap()).sub_mul(
        &Natural::from_str("5").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

#[test]
#[should_panic]
fn sub_mul_ref_ref_ref_fail_2() {
    (&Natural::from_str("1000000000000").unwrap()).sub_mul(
        &Natural::from_str("1000000000000").unwrap(),
        &Natural::from_str("100").unwrap(),
    );
}

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
        assert_eq!(n.sub_mul(n, &Natural::ONE), Natural::ZERO);
    });

    test_properties(pairs_of_naturals, |&(ref a, ref b)| {
        assert_eq!(a.sub_mul(&Natural::ZERO, b), *a);
        assert_eq!(a.sub_mul(b, &Natural::ZERO), *a);
        assert_eq!((a * b).sub_mul(a, b), Natural::ZERO);
    });

    test_properties(pairs_of_naturals_var_1, |&(ref a, ref b)| {
        assert_eq!(a.sub_mul(&Natural::ONE, b), a - b);
        assert_eq!(a.sub_mul(b, &Natural::ONE), a - b);
    });
}
