use common::test_properties;
use malachite_base::num::Zero;
use malachite_nz::natural::logic::or::{
    limbs_or, limbs_or_in_place_either, limbs_or_in_place_left, limbs_or_same_length,
    limbs_or_same_length_in_place_left, limbs_or_same_length_to_out, limbs_or_to_out,
};
use malachite_nz::natural::Natural;
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::base::{
    pairs_of_unsigned_vec, pairs_of_unsigned_vec_var_1, triples_of_unsigned_vec_var_3,
    triples_of_unsigned_vec_var_4,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_unsigned, pairs_of_naturals, triples_of_naturals,
};
use malachite_test::natural::logic::or::{natural_or_alt_1, natural_or_alt_2};
use num::BigUint;
use rug;
use std::cmp::max;
use std::str::FromStr;

#[test]
fn test_limbs_or_same_length_and_limbs_or_same_length_in_place_left() {
    let test = |xs_before, ys, out| {
        assert_eq!(limbs_or_same_length(xs_before, ys), out);

        let mut xs = xs_before.to_vec();
        limbs_or_same_length_in_place_left(&mut xs, ys);
        assert_eq!(xs, out);
    };
    test(&[], &[], vec![]);
    test(&[2], &[3], vec![3]);
    test(&[1, 1, 1], &[1, 2, 3], vec![1, 3, 3]);
    test(&[6, 7], &[1, 2], vec![7, 7]);
    test(&[100, 101, 102], &[102, 101, 100], vec![102, 101, 102]);
}

#[test]
#[should_panic(expected = "assertion failed: `(left == right)`")]
fn limbs_or_same_length_fail_1() {
    limbs_or_same_length(&[6, 7], &[1, 2, 3]);
}

#[test]
#[should_panic(expected = "assertion failed: `(left == right)`")]
fn limbs_or_same_length_in_place_left_fail() {
    let mut out = vec![6, 7];
    limbs_or_same_length_in_place_left(&mut out, &[1, 2, 3]);
}

#[test]
fn test_limbs_or() {
    let test = |xs, ys, out| {
        assert_eq!(limbs_or(xs, ys), out);
    };
    test(&[], &[], vec![]);
    test(&[2], &[3], vec![3]);
    test(&[1, 1, 1], &[1, 2, 3], vec![1, 3, 3]);
    test(&[6, 7], &[1, 2, 3], vec![7, 7, 3]);
    test(&[1, 2, 3], &[6, 7], vec![7, 7, 3]);
    test(&[100, 101, 102], &[102, 101, 100], vec![102, 101, 102]);
}

#[test]
fn test_limbs_or_same_length_to_out() {
    let test = |xs, ys, out_before: &[u32], out_after| {
        let mut out = out_before.to_vec();
        limbs_or_same_length_to_out(&mut out, xs, ys);
        assert_eq!(out, out_after);
    };
    test(&[], &[], &[0, 0], vec![0, 0]);
    test(&[1, 1, 1], &[1, 2, 3], &[5, 5, 5, 5], vec![1, 3, 3, 5]);
    test(&[6, 7], &[1, 2], &[0, 0], vec![7, 7]);
    test(&[6, 7], &[1, 2], &[10, 10, 10, 10], vec![7, 7, 10, 10]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        vec![102, 101, 102, 10],
    );
}

#[test]
#[should_panic(expected = "assertion failed: `(left == right)`")]
fn limbs_or_same_length_to_out_fail_1() {
    let mut out = vec![10, 10, 10, 10];
    limbs_or_same_length_to_out(&mut out, &[6, 7], &[1, 2, 3]);
}

#[test]
#[should_panic(expected = "assertion failed: out_limbs.len() >= len")]
fn limbs_or_same_length_to_out_fail_2() {
    let mut out = vec![10];
    limbs_or_same_length_to_out(&mut out, &[6, 7], &[1, 2]);
}

#[test]
fn test_limbs_or_to_out() {
    let test = |xs, ys, out_before: &[u32], out_after| {
        let mut out = out_before.to_vec();
        limbs_or_to_out(&mut out, xs, ys);
        assert_eq!(out, out_after);
    };
    test(&[], &[], &[0, 0], vec![0, 0]);
    test(&[1, 1, 1], &[1, 2, 3], &[5, 5, 5, 5], vec![1, 3, 3, 5]);
    test(&[6, 7], &[1, 2, 3], &[10, 10, 10, 10], vec![7, 7, 3, 10]);
    test(&[1, 2, 3], &[6, 7], &[10, 10, 10, 10], vec![7, 7, 3, 10]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        vec![102, 101, 102, 10],
    );
}

#[test]
#[should_panic(expected = "assertion failed: out_limbs.len() >= ys_len")]
fn limbs_or_to_out_fail() {
    let mut out = vec![10, 10];
    limbs_or_to_out(&mut out, &[6, 7], &[1, 2, 3]);
}

#[test]
fn test_limbs_or_in_place_left() {
    let test = |xs_before: &[u32], ys, xs_after| {
        let mut xs = xs_before.to_vec();
        limbs_or_in_place_left(&mut xs, ys);
        assert_eq!(xs, xs_after);
    };
    test(&[], &[], vec![]);
    test(&[6, 7], &[1, 2], vec![7, 7]);
    test(&[6, 7], &[1, 2, 3], vec![7, 7, 3]);
    test(&[1, 2, 3], &[6, 7], vec![7, 7, 3]);
    test(&[], &[1, 2, 3], vec![1, 2, 3]);
    test(&[1, 2, 3], &[], vec![1, 2, 3]);
    test(&[1, 1, 1], &[1, 2, 3], vec![1, 3, 3]);
    test(&[100, 101, 102], &[102, 101, 100], vec![102, 101, 102]);
}

#[test]
fn test_limbs_or_in_place_either() {
    let test = |xs_before: &[u32], ys_before: &[u32], right, xs_after, ys_after| {
        let mut xs = xs_before.to_vec();
        let mut ys = ys_before.to_vec();
        assert_eq!(limbs_or_in_place_either(&mut xs, &mut ys), right);
        assert_eq!(xs, xs_after);
        assert_eq!(ys, ys_after);
    };
    test(&[], &[], false, vec![], vec![]);
    test(&[6, 7], &[1, 2], false, vec![7, 7], vec![1, 2]);
    test(&[6, 7], &[1, 2, 3], true, vec![6, 7], vec![7, 7, 3]);
    test(&[1, 2, 3], &[6, 7], false, vec![7, 7, 3], vec![6, 7]);
    test(&[], &[1, 2, 3], true, vec![], vec![1, 2, 3]);
    test(&[1, 2, 3], &[], false, vec![1, 2, 3], vec![]);
    test(&[1, 1, 1], &[1, 2, 3], false, vec![1, 3, 3], vec![1, 2, 3]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![102, 101, 102],
        vec![102, 101, 100],
    );
}

#[test]
fn test_or() {
    let test = |u, v, out| {
        let mut n = Natural::from_str(u).unwrap();
        n |= Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n |= &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap() | Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() | Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap() | &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() | &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        assert_eq!(
            natural_or_alt_1(
                &Natural::from_str(u).unwrap(),
                &Natural::from_str(v).unwrap()
            )
            .to_string(),
            out
        );
        assert_eq!(
            natural_or_alt_2(
                &Natural::from_str(u).unwrap(),
                &Natural::from_str(v).unwrap()
            )
            .to_string(),
            out
        );

        let n = BigUint::from_str(u).unwrap() | BigUint::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() | rug::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "123");
    test("123", "0", "123");
    test("123", "456", "507");
    test("1000000000000", "123", "1000000000123");
    test("123", "1000000000000", "1000000000123");
    test("1000000000000", "999999999999", "1000000004095");
    test("12345678987654321", "314159265358979", "12347506587071667");
}

fn limbs_or_helper(f: &mut FnMut(&[u32], &[u32]) -> Vec<u32>, xs: &Vec<u32>, ys: &Vec<u32>) {
    assert_eq!(
        Natural::from_owned_limbs_asc(f(xs, ys)),
        Natural::from_limbs_asc(xs) | Natural::from_limbs_asc(ys)
    );
}

#[test]
fn limbs_or_same_length_properties() {
    test_properties(pairs_of_unsigned_vec_var_1, |&(ref xs, ref ys)| {
        limbs_or_helper(&mut limbs_or_same_length, xs, ys);
    });
}

#[test]
fn limbs_or_properties() {
    test_properties(pairs_of_unsigned_vec, |&(ref xs, ref ys)| {
        limbs_or_helper(&mut limbs_or, xs, ys);
    });
}

#[test]
fn limbs_or_same_length_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_var_3,
        |&(ref xs, ref ys, ref zs)| {
            let mut xs = xs.to_vec();
            let xs_old = xs.clone();
            limbs_or_same_length_to_out(&mut xs, ys, zs);
            let len = ys.len();
            assert_eq!(
                Natural::from_limbs_asc(&xs[..len]),
                Natural::from_limbs_asc(ys) | Natural::from_limbs_asc(zs)
            );
            assert_eq!(&xs[len..], &xs_old[len..]);
        },
    );
}

#[test]
fn limbs_or_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_var_4,
        |&(ref xs, ref ys, ref zs)| {
            let mut xs = xs.to_vec();
            let xs_old = xs.clone();
            limbs_or_to_out(&mut xs, ys, zs);
            let len = max(ys.len(), zs.len());
            assert_eq!(
                Natural::from_limbs_asc(&xs[..len]),
                Natural::from_limbs_asc(ys) | Natural::from_limbs_asc(zs)
            );
            assert_eq!(&xs[len..], &xs_old[len..]);
        },
    );
}

#[test]
fn limbs_or_same_length_in_place_left_properties() {
    test_properties(pairs_of_unsigned_vec_var_1, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_or_same_length_in_place_left(&mut xs, ys);
        assert_eq!(
            Natural::from_owned_limbs_asc(xs),
            Natural::from_owned_limbs_asc(xs_old) | Natural::from_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_or_in_place_left_properties() {
    test_properties(pairs_of_unsigned_vec_var_1, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_or_in_place_left(&mut xs, ys);
        let n = Natural::from_limbs_asc(&xs_old) | Natural::from_limbs_asc(ys);
        assert_eq!(Natural::from_owned_limbs_asc(xs), n);
    });
}

#[test]
fn limbs_or_in_place_either_properties() {
    test_properties(pairs_of_unsigned_vec_var_1, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let mut ys = ys.to_vec();
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let right = limbs_or_in_place_either(&mut xs, &mut ys);
        let n = Natural::from_limbs_asc(&xs_old) | Natural::from_limbs_asc(&ys_old);
        if right {
            assert_eq!(xs, xs_old);
            assert_eq!(Natural::from_owned_limbs_asc(ys), n);
        } else {
            assert_eq!(Natural::from_owned_limbs_asc(xs), n);
            assert_eq!(ys, ys_old);
        }
    });
}

#[test]
fn or_properties() {
    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        let result_val_val = x.clone() | y.clone();
        let result_val_ref = x.clone() | y;
        let result_ref_val = x | y.clone();
        let result = x | y;
        assert!(result_val_val.is_valid());
        assert!(result_val_ref.is_valid());
        assert!(result_ref_val.is_valid());
        assert!(result.is_valid());
        assert_eq!(result_val_val, result);
        assert_eq!(result_val_ref, result);
        assert_eq!(result_ref_val, result);

        let mut mut_x = x.clone();
        mut_x |= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, result);
        let mut mut_x = x.clone();
        mut_x |= y;
        assert_eq!(mut_x, result);
        assert!(mut_x.is_valid());

        let mut mut_x = natural_to_rug_integer(x);
        mut_x |= natural_to_rug_integer(y);
        assert_eq!(rug_integer_to_natural(&mut_x), result);

        assert_eq!(
            biguint_to_natural(&(natural_to_biguint(x) | natural_to_biguint(y))),
            result
        );
        assert_eq!(
            rug_integer_to_natural(&(natural_to_rug_integer(x) | natural_to_rug_integer(y))),
            result
        );

        assert_eq!(natural_or_alt_1(&x, y), result);
        assert_eq!(natural_or_alt_2(&x, y), result);

        assert_eq!(y | x, result);
        assert_eq!(&result | x, result);
        assert_eq!(&result | y, result);

        assert!(result >= *x);
        assert!(result >= *y);

        let ones = result.count_ones();
        assert!(ones >= u64::from(x.count_ones()));
        assert!(ones >= u64::from(y.count_ones()));
    });

    test_properties(
        pairs_of_natural_and_unsigned,
        |&(ref x, y): &(Natural, u32)| {
            let result = x | Natural::from(y);
            assert_eq!(x | y, result);
            assert_eq!(y | x, result);
        },
    );

    test_properties(naturals, |x| {
        assert_eq!(x | Natural::ZERO, *x);
        assert_eq!(Natural::ZERO | x, *x);
        assert_eq!(x | x, *x);
    });

    test_properties(triples_of_naturals, |&(ref x, ref y, ref z)| {
        assert_eq!((x | y) | z, x | (y | z));
        assert_eq!(x & (y | z), (x & y) | (x & z));
        assert_eq!((x & y) | z, (x | z) & (y | z));
        assert_eq!(x | (y & z), (x | y) & (x | z));
        assert_eq!((x | y) & z, (x & z) | (y & z));
    });
}
