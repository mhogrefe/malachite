use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::natural::arithmetic::add_mul_limb::{
    limbs_add_mul_limb, limbs_slice_add_mul_limb_same_length_in_place_left,
    limbs_slice_add_mul_limb_same_length_in_place_right, limbs_vec_add_mul_limb_in_place_either,
    limbs_vec_add_mul_limb_in_place_left, limbs_vec_add_mul_limb_in_place_right,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use common::test_properties;
use malachite_test::inputs::base::{
    triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_7,
};
use malachite_test::inputs::natural::{
    pairs_of_natural_and_unsigned, pairs_of_naturals, triples_of_natural_natural_and_unsigned,
};

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_add_mul_limb() {
    let test = |xs_before: &[Limb], ys_before: &[Limb], limb: Limb, result: &[Limb]| {
        assert_eq!(limbs_add_mul_limb(xs_before, ys_before, limb), result);
        let mut xs = xs_before.to_vec();
        limbs_vec_add_mul_limb_in_place_left(&mut xs, ys_before, limb);
        assert_eq!(xs, result);
        let mut ys = ys_before.to_vec();
        limbs_vec_add_mul_limb_in_place_right(xs_before, &mut ys, limb);
        assert_eq!(ys, result);
    };
    test(&[123, 456], &[123], 4, &[615, 456]);
    test(&[123, 456], &[123], 0xffff_ffff, &[0, 579]);
    test(&[123], &[0, 123], 4, &[123, 492]);
    test(&[123, 456], &[0, 123], 0xffff_ffff, &[123, 333, 123]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_add_mul_limb_same_length_in_place_left() {
    let test = |xs_before: &[Limb], ys: &[Limb], limb: Limb, xs_after: &[Limb], carry: Limb| {
        let mut xs = xs_before.to_vec();
        assert_eq!(
            limbs_slice_add_mul_limb_same_length_in_place_left(&mut xs, ys, limb),
            carry
        );
        assert_eq!(xs, xs_after);
    };
    test(&[123], &[123], 4, &[615], 0);
    test(&[123], &[123], 0xffff_ffff, &[0], 123);
    test(&[123, 0], &[0, 123], 4, &[123, 492], 0);
    test(&[123, 456], &[0, 123], 0xffff_ffff, &[123, 333], 123);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_add_mul_limb_same_length_in_place_left_fail() {
    limbs_slice_add_mul_limb_same_length_in_place_left(&mut [10], &[10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_add_mul_limb_same_length_in_place_right() {
    let test = |xs: &[Limb], ys_before: &[Limb], limb: Limb, ys_after: &[Limb], carry: Limb| {
        let mut ys = ys_before.to_vec();
        assert_eq!(
            limbs_slice_add_mul_limb_same_length_in_place_right(xs, &mut ys, limb),
            carry
        );
        assert_eq!(ys, ys_after);
    };
    test(&[123, 456], &[123, 0], 4, &[615, 456], 0);
    test(&[123, 456], &[123, 0], 0xffff_ffff, &[0, 579], 0);
    test(&[123, 0], &[0, 123], 4, &[123, 492], 0);
    test(&[123, 456], &[0, 123], 0xffff_ffff, &[123, 333], 123);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_add_mul_limb_same_length_in_place_right_fail() {
    limbs_slice_add_mul_limb_same_length_in_place_right(&[10, 10], &mut [10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_vec_add_mul_limb_in_place_either() {
    let test = |xs_before: &[Limb],
                ys_before: &[Limb],
                limb: Limb,
                right: bool,
                xs_after: &[Limb],
                ys_after: &[Limb]| {
        let mut xs = xs_before.to_vec();
        let mut ys = ys_before.to_vec();
        assert_eq!(
            limbs_vec_add_mul_limb_in_place_either(&mut xs, &mut ys, limb),
            right
        );
        assert_eq!(xs, xs_after);
        assert_eq!(ys, ys_after);
    };
    test(&[123, 456], &[123], 4, false, &[615, 456], &[123]);
    test(
        &[123, 456],
        &[123, 0],
        0xffff_ffff,
        false,
        &[0, 579],
        &[123, 0],
    );
    test(&[123], &[0, 123], 4, true, &[123], &[123, 492]);
    test(
        &[123, 456],
        &[0, 123],
        0xffff_ffff,
        false,
        &[123, 333, 123],
        &[0, 123],
    );
}

#[test]
fn test_add_mul_limb() {
    let test = |u, v, c: Limb, out| {
        let mut a = Natural::from_str(u).unwrap();
        a.add_mul_assign(Natural::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = Natural::from_str(u).unwrap();
        a.add_mul_assign(&Natural::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Natural::from_str(u)
            .unwrap()
            .add_mul(Natural::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Natural::from_str(u)
            .unwrap()
            .add_mul(&Natural::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = (&Natural::from_str(u).unwrap()).add_mul(Natural::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = (&Natural::from_str(u).unwrap()).add_mul(&Natural::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());
    };
    test("0", "0", 0, "0");
    test("0", "0", 123, "0");
    test("123", "0", 5, "123");
    test("123", "5", 1, "128");
    test("123", "5", 100, "623");
    test("10", "3", 4, "22");
    test("1000000000000", "0", 123, "1000000000000");
    test("1000000000000", "1", 123, "1000000000123");
    test("1000000000000", "123", 1, "1000000000123");
    test("1000000000000", "123", 100, "1000000012300");
    test("1000000000000", "100", 123, "1000000012300");
    test("1000000000000", "65536", 0x1_0000, "1004294967296");
    test("1000000000000", "1000000000000", 0, "1000000000000");
    test("1000000000000", "1000000000000", 1, "2000000000000");
    test("1000000000000", "1000000000000", 100, "101000000000000");
    test("0", "1000000000000", 100, "100000000000000");
    test(
        "18446744073583722494",
        "2",
        4_033_876_984,
        "18446744081651476462",
    );
}

#[test]
fn limbs_add_mul_limb_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_positive_unsigned_var_3,
        |&(ref a, ref b, c)| {
            assert_eq!(
                limbs_add_mul_limb(a, b, c),
                Natural::from_limbs_asc(a)
                    .add_mul(Natural::from_limbs_asc(b), c)
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
                Natural::from_limbs_asc(&a_old[..len]).add_mul(Natural::from_limbs_asc(b), c)
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
                Natural::from_limbs_asc(a).add_mul(Natural::from_limbs_asc(b_old), c)
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
                    .add_mul(Natural::from_limbs_asc(b), c)
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
                    .add_mul(Natural::from_limbs_asc(b_old), c)
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
                    .add_mul(Natural::from_limbs_asc(b_old), c)
                    .into_limbs_asc()
            );
        },
    );
}

#[test]
fn add_mul_limb_properties() {
    test_properties(
        triples_of_natural_natural_and_unsigned,
        |&(ref a, ref b, c): &(Natural, Natural, Limb)| {
            let mut mut_a = a.clone();
            mut_a.add_mul_assign(b.clone(), c);
            assert!(mut_a.is_valid());
            let result = mut_a;

            let mut mut_a = a.clone();
            mut_a.add_mul_assign(b, c);
            assert!(mut_a.is_valid());
            assert_eq!(mut_a, result);

            let result = a.add_mul(b.clone(), c);
            assert!(mut_a.is_valid());
            assert_eq!(mut_a, result);

            let result_alt = a.add_mul(b, c);
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = a.clone().add_mul(b.clone(), c);
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = a.add_mul(b, c);
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            assert_eq!(a + b * Natural::from(c), result);
            assert_eq!(a.add_mul(b, &Natural::from(c)), result);
        },
    );

    test_properties(
        pairs_of_natural_and_unsigned,
        |&(ref n, c): &(Natural, Limb)| {
            assert_eq!(n.add_mul(&Natural::ZERO, c), *n);
            assert_eq!(n.add_mul(&Natural::ONE, c), n + c);
            assert_eq!(Natural::ZERO.add_mul(n, c), n * Natural::from(c));
        },
    );

    test_properties(pairs_of_naturals, |&(ref a, ref b)| {
        assert_eq!(a.add_mul(b, 0 as Limb), *a);
        assert_eq!(a.add_mul(b, 1 as Limb), a + b);
    });
}
