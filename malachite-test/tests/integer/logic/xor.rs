use common::test_properties;
use malachite_base::misc::CheckedFrom;
use malachite_base::num::{NegativeOne, Zero};
use malachite_nz::integer::logic::xor::{
    limbs_xor_neg_neg, limbs_xor_neg_neg_in_place_either, limbs_xor_neg_neg_in_place_left,
    limbs_xor_neg_neg_to_out, limbs_xor_pos_neg, limbs_xor_pos_neg_in_place_either,
    limbs_xor_pos_neg_in_place_left, limbs_xor_pos_neg_in_place_right, limbs_xor_pos_neg_to_out,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::common::{integer_to_rug_integer, rug_integer_to_integer};
use malachite_test::inputs::base::{pairs_of_u32_vec_var_1, triples_of_u32_vec_var_7};
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_signed, pairs_of_integers, triples_of_integers,
};
use malachite_test::integer::logic::xor::{integer_xor_alt_1, integer_xor_alt_2};
use rug;
use std::cmp::max;
use std::str::FromStr;

#[test]
fn test_limbs_xor_pos_neg_limbs_xor_pos_neg_in_place_left_and_limbs_xor_pos_neg_in_place_right() {
    let test = |xs_before, ys_before, out| {
        assert_eq!(limbs_xor_pos_neg(xs_before, ys_before), out);

        let mut xs = xs_before.to_vec();
        limbs_xor_pos_neg_in_place_left(&mut xs, ys_before);
        assert_eq!(xs, out);

        let mut ys = ys_before.to_vec();
        limbs_xor_pos_neg_in_place_right(xs_before, &mut ys);
        assert_eq!(ys, out);
    };
    test(&[2], &[3], vec![1]);
    test(&[1, 1, 1], &[1, 2, 3], vec![2, 3, 2]);
    test(&[6, 7], &[1, 2, 3], vec![7, 5, 3]);
    test(&[1, 2, 3], &[6, 7], vec![5, 5, 3]);
    test(&[100, 101, 102], &[102, 101, 100], vec![2, 0, 2]);
    test(&[0, 0, 1], &[3], vec![3, 0, 1]);
    test(&[3], &[0, 0, 1], vec![4_294_967_293, 4_294_967_295, 0]);
    test(&[0, 3, 3], &[0, 0, 3], vec![0, 4_294_967_293, 1]);
    test(&[0, 0, 3], &[0, 3, 3], vec![0, 3, 0]);
    test(&[0, 3], &[0, 0, 3], vec![0, 4_294_967_293, 2]);
    test(&[0, 0, 3], &[0, 3], vec![0, 3, 3]);
}

#[test]
#[should_panic(expected = "assertion failed: x_i < xs_len")]
fn limbs_xor_pos_neg_fail_1() {
    limbs_xor_pos_neg(&[0, 0, 0], &[3]);
}

#[test]
#[should_panic(expected = "assertion failed: y_i < ys_len")]
fn limbs_xor_pos_neg_fail_2() {
    limbs_xor_pos_neg(&[3], &[0, 0, 0]);
}

#[test]
#[should_panic(expected = "assertion failed: x_i < xs_len")]
fn limbs_xor_pos_neg_in_place_left_fail_1() {
    limbs_xor_pos_neg_in_place_left(&mut vec![0, 0, 0], &[3]);
}

#[test]
#[should_panic(expected = "assertion failed: y_i < ys_len")]
fn limbs_xor_pos_neg_in_place_left_fail_2() {
    limbs_xor_pos_neg_in_place_left(&mut vec![3], &[0, 0, 0]);
}

#[test]
#[should_panic(expected = "assertion failed: x_i < xs_len")]
fn limbs_xor_pos_neg_in_place_right_fail_1() {
    limbs_xor_pos_neg_in_place_right(&[0, 0, 0], &mut vec![3]);
}

#[test]
#[should_panic(expected = "assertion failed: y_i < ys_len")]
fn limbs_xor_pos_neg_in_place_right_fail_2() {
    limbs_xor_pos_neg_in_place_right(&[3], &mut vec![0, 0, 0]);
}

#[test]
fn test_limbs_xor_pos_neg_to_out() {
    let test = |xs, ys, out_before: &[u32], out_after| {
        let mut out = out_before.to_vec();
        limbs_xor_pos_neg_to_out(&mut out, xs, ys);
        assert_eq!(out, out_after);
    };
    test(&[2], &[3], &[10, 10, 10, 10], vec![1, 10, 10, 10]);
    test(&[1, 1, 1], &[1, 2, 3], &[10, 10, 10, 10], vec![2, 3, 2, 10]);
    test(&[6, 7], &[1, 2, 3], &[10, 10, 10, 10], vec![7, 5, 3, 10]);
    test(&[1, 2, 3], &[6, 7], &[10, 10, 10, 10], vec![5, 5, 3, 10]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        vec![2, 0, 2, 10],
    );
    test(&[0, 0, 1], &[3], &[10, 10, 10, 10], vec![3, 0, 1, 10]);
    test(
        &[3],
        &[0, 0, 1],
        &[10, 10, 10, 10],
        vec![4_294_967_293, 4_294_967_295, 0, 10],
    );
    test(
        &[0, 3, 3],
        &[0, 0, 3],
        &[10, 10, 10, 10],
        vec![0, 4_294_967_293, 1, 10],
    );
    test(&[0, 0, 3], &[0, 3, 3], &[10, 10, 10, 10], vec![0, 3, 0, 10]);
    test(
        &[0, 3],
        &[0, 0, 3],
        &[10, 10, 10, 10],
        vec![0, 4_294_967_293, 2, 10],
    );
    test(&[0, 0, 3], &[0, 3], &[10, 10, 10, 10], vec![0, 3, 3, 10]);
}

#[test]
#[should_panic(expected = "assertion failed: x_i < xs_len")]
fn limbs_xor_pos_neg_to_out_fail_1() {
    let mut out = vec![10, 10, 10, 10];
    limbs_xor_pos_neg_to_out(&mut out, &[0, 0, 0], &[3]);
}

#[test]
#[should_panic(expected = "assertion failed: y_i < ys_len")]
fn limbs_xor_pos_neg_to_out_fail_2() {
    let mut out = vec![10, 10, 10, 10];
    limbs_xor_pos_neg_to_out(&mut out, &[3], &[0, 0, 0]);
}

#[test]
#[should_panic(expected = "assertion failed: out_limbs.len() >= xs_len")]
fn limbs_xor_pos_neg_to_out_fail_3() {
    let mut out = vec![10];
    limbs_xor_pos_neg_to_out(&mut out, &[6, 7], &[1, 2]);
}

#[test]
fn test_limbs_xor_pos_neg_in_place_either() {
    let test = |xs_before: &[u32], ys_before: &[u32], b, xs_after, ys_after| {
        let mut xs = xs_before.to_vec();
        let mut ys = ys_before.to_vec();
        assert_eq!(limbs_xor_pos_neg_in_place_either(&mut xs, &mut ys), b);
        assert_eq!(xs, xs_after);
        assert_eq!(ys, ys_after);
    };
    test(&[2], &[3], false, vec![1], vec![3]);
    test(&[1, 1, 1], &[1, 2, 3], false, vec![2, 3, 2], vec![1, 2, 3]);
    test(&[1, 2, 3], &[6, 7], false, vec![5, 5, 3], vec![6, 7]);
    test(&[6, 7], &[1, 2, 3], true, vec![6, 7], vec![7, 5, 3]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![2, 0, 2],
        vec![102, 101, 100],
    );
    test(&[0, 0, 1], &[3], false, vec![3, 0, 1], vec![3]);
    test(
        &[3],
        &[0, 0, 1],
        true,
        vec![3],
        vec![4_294_967_293, 4_294_967_295, 0],
    );
    test(
        &[0, 3, 3],
        &[0, 0, 3],
        false,
        vec![0, 4_294_967_293, 1],
        vec![0, 0, 3],
    );
    test(&[0, 0, 3], &[0, 3, 3], false, vec![0, 3, 0], vec![0, 3, 3]);
    test(
        &[0, 3],
        &[0, 0, 3],
        true,
        vec![0, 3],
        vec![0, 4_294_967_293, 2],
    );
    test(&[0, 0, 3], &[0, 3], false, vec![0, 3, 3], vec![0, 3]);
}

#[test]
#[should_panic(expected = "assertion failed: x_i < xs_len")]
fn limbs_xor_pos_neg_in_place_either_fail_1() {
    limbs_xor_pos_neg_in_place_either(&mut vec![0, 0, 0], &mut vec![3]);
}

#[test]
#[should_panic(expected = "assertion failed: y_i < ys_len")]
fn limbs_xor_pos_neg_in_place_either_fail_2() {
    limbs_xor_pos_neg_in_place_either(&mut vec![3], &mut vec![0, 0, 0]);
}

#[test]
fn test_limbs_xor_neg_neg_and_limbs_xor_neg_neg_in_place_left() {
    let test = |xs_before, ys, out| {
        assert_eq!(limbs_xor_neg_neg(xs_before, ys), out);

        let mut xs = xs_before.to_vec();
        limbs_xor_neg_neg_in_place_left(&mut xs, ys);
        assert_eq!(xs, out);
    };
    test(&[2], &[3], vec![3]);
    test(&[1, 1, 1], &[1, 2, 3], vec![0, 3, 2]);
    test(&[6, 7], &[1, 2, 3], vec![5, 5, 3]);
    test(&[1, 2, 3], &[6, 7], vec![5, 5, 3]);
    test(&[100, 101, 102], &[102, 101, 100], vec![6, 0, 2]);
    test(&[0, 0, 1], &[3], vec![4_294_967_293, 4_294_967_295, 0]);
    test(&[3], &[0, 0, 1], vec![4_294_967_293, 4_294_967_295, 0]);
    test(&[0, 3, 3], &[0, 0, 3], vec![0, 4_294_967_293, 1]);
    test(&[0, 0, 3], &[0, 3, 3], vec![0, 4_294_967_293, 1]);
    test(&[0, 3], &[0, 0, 3], vec![0, 4_294_967_293, 2]);
    test(&[0, 0, 3], &[0, 3], vec![0, 4_294_967_293, 2]);
}

#[test]
#[should_panic(expected = "assertion failed: x_i < xs_len")]
fn limbs_xor_neg_neg_in_place_left_fail_1() {
    limbs_xor_neg_neg_in_place_left(&mut vec![0, 0, 0], &[3]);
}

#[test]
#[should_panic(expected = "assertion failed: y_i < ys_len")]
fn limbs_xor_neg_neg_in_place_left_fail_2() {
    limbs_xor_neg_neg_in_place_left(&mut vec![3], &[0, 0, 0]);
}

#[test]
fn test_limbs_xor_neg_neg_to_out() {
    let test = |xs, ys, out_before: &[u32], out_after| {
        let mut out = out_before.to_vec();
        limbs_xor_neg_neg_to_out(&mut out, xs, ys);
        assert_eq!(out, out_after);
    };
    test(&[2], &[3], &[10, 10, 10, 10], vec![3, 10, 10, 10]);
    test(&[1, 1, 1], &[1, 2, 3], &[10, 10, 10, 10], vec![0, 3, 2, 10]);
    test(&[6, 7], &[1, 2, 3], &[10, 10, 10, 10], vec![5, 5, 3, 10]);
    test(&[1, 2, 3], &[6, 7], &[10, 10, 10, 10], vec![5, 5, 3, 10]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        vec![6, 0, 2, 10],
    );
    test(
        &[0, 0, 1],
        &[3],
        &[10, 10, 10, 10],
        vec![4_294_967_293, 4_294_967_295, 0, 10],
    );
    test(
        &[3],
        &[0, 0, 1],
        &[10, 10, 10, 10],
        vec![4_294_967_293, 4_294_967_295, 0, 10],
    );
    test(
        &[0, 3, 3],
        &[0, 0, 3],
        &[10, 10, 10, 10],
        vec![0, 4_294_967_293, 1, 10],
    );
    test(
        &[0, 0, 3],
        &[0, 3, 3],
        &[10, 10, 10, 10],
        vec![0, 4_294_967_293, 1, 10],
    );
    test(
        &[0, 3],
        &[0, 0, 3],
        &[10, 10, 10, 10],
        vec![0, 4_294_967_293, 2, 10],
    );
    test(
        &[0, 0, 3],
        &[0, 3],
        &[10, 10, 10, 10],
        vec![0, 4_294_967_293, 2, 10],
    );
}

#[test]
#[should_panic(expected = "assertion failed: x_i < xs_len")]
fn limbs_xor_neg_neg_to_out_fail_1() {
    let mut out = vec![10, 10, 10, 10];
    limbs_xor_neg_neg_to_out(&mut out, &[0, 0, 0], &[3]);
}

#[test]
#[should_panic(expected = "assertion failed: y_i < ys_len")]
fn limbs_xor_neg_neg_to_out_fail_2() {
    let mut out = vec![10, 10, 10, 10];
    limbs_xor_neg_neg_to_out(&mut out, &[3], &[0, 0, 0]);
}

#[test]
#[should_panic(expected = "assertion failed: out_limbs.len() >= xs_len")]
fn limbs_xor_neg_neg_to_out_fail_3() {
    let mut out = vec![10];
    limbs_xor_neg_neg_to_out(&mut out, &[6, 7], &[1, 2]);
}

#[test]
fn test_limbs_xor_neg_neg_in_place_either() {
    let test = |xs_before: &[u32], ys_before: &[u32], b, xs_after, ys_after| {
        let mut xs = xs_before.to_vec();
        let mut ys = ys_before.to_vec();
        assert_eq!(limbs_xor_neg_neg_in_place_either(&mut xs, &mut ys), b);
        assert_eq!(xs, xs_after);
        assert_eq!(ys, ys_after);
    };
    test(&[2], &[3], false, vec![3], vec![3]);
    test(&[1, 1, 1], &[1, 2, 3], false, vec![0, 3, 2], vec![1, 2, 3]);
    test(&[1, 2, 3], &[6, 7], false, vec![5, 5, 3], vec![6, 7]);
    test(&[6, 7], &[1, 2, 3], true, vec![6, 7], vec![5, 5, 3]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![6, 0, 2],
        vec![102, 101, 100],
    );
    test(
        &[0, 0, 1],
        &[3],
        false,
        vec![4_294_967_293, 4_294_967_295, 0],
        vec![3],
    );
    test(
        &[3],
        &[0, 0, 1],
        true,
        vec![3],
        vec![4_294_967_293, 4_294_967_295, 0],
    );
    test(
        &[0, 3, 3],
        &[0, 0, 3],
        false,
        vec![0, 4_294_967_293, 1],
        vec![0, 0, 3],
    );
    test(
        &[0, 0, 3],
        &[0, 3, 3],
        false,
        vec![0, 4_294_967_293, 1],
        vec![0, 3, 3],
    );
    test(
        &[0, 3],
        &[0, 0, 3],
        true,
        vec![0, 3],
        vec![0, 4_294_967_293, 2],
    );
    test(
        &[0, 0, 3],
        &[0, 3],
        false,
        vec![0, 4_294_967_293, 2],
        vec![0, 3],
    );
}

#[test]
#[should_panic(expected = "assertion failed: x_i < xs_len")]
fn limbs_xor_neg_neg_in_place_either_fail_1() {
    limbs_xor_neg_neg_in_place_either(&mut vec![0, 0, 0], &mut vec![3]);
}

#[test]
#[should_panic(expected = "assertion failed: y_i < ys_len")]
fn limbs_xor_neg_neg_in_place_either_fail_2() {
    limbs_xor_neg_neg_in_place_either(&mut vec![3], &mut vec![0, 0, 0]);
}

#[test]
fn test_xor() {
    let test = |u, v, out| {
        let mut n = Integer::from_str(u).unwrap();
        n ^= Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Integer::from_str(u).unwrap();
        n ^= &Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap() ^ Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() ^ Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap() ^ &Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() ^ &Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        assert_eq!(
            integer_xor_alt_1(
                &Integer::from_str(u).unwrap(),
                &Integer::from_str(v).unwrap()
            )
            .to_string(),
            out
        );
        assert_eq!(
            integer_xor_alt_2(
                &Integer::from_str(u).unwrap(),
                &Integer::from_str(v).unwrap()
            )
            .to_string(),
            out
        );

        let n = rug::Integer::from_str(u).unwrap() ^ rug::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "123");
    test("123", "0", "123");
    test("123", "456", "435");
    test("1000000000000", "123", "1000000000123");
    test("123", "1000000000000", "1000000000123");
    test("1000000000000", "999999999999", "8191");
    test("12345678987654321", "314159265358979", "12035174921130034");
    test("0", "-123", "-123");
    test("123", "-456", "-445");
    test("1000000000000", "-123", "-1000000000123");
    test("123", "-1000000000000", "-999999999877");
    test("1000000000000", "-999999999999", "-8191");
    test(
        "12345678987654321",
        "-314159265358979",
        "-12035174921130036",
    );
    test("-123", "0", "-123");
    test("-123", "456", "-435");
    test("-1000000000000", "123", "-999999999877");
    test("-123", "1000000000000", "-1000000000123");
    test("-1000000000000", "999999999999", "-1");
    test(
        "-12345678987654321",
        "314159265358979",
        "-12035174921130036",
    );
    test("-123", "-456", "445");
    test("-1000000000000", "-123", "999999999877");
    test("-123", "-1000000000000", "999999999877");
    test("-1000000000000", "-999999999999", "1");
    test(
        "-12345678987654321",
        "-314159265358979",
        "12035174921130034",
    );

    test(
        "-58833344151816",
        "-163347918670491773353",
        "163347906017862822063",
    );
    test(
        "-163347918670491773353",
        "-58833344151816",
        "163347906017862822063",
    );
    test(
        "4722366342132156858368",
        "-35201550909443",
        "-4722366377333707767811",
    );
    test(
        "-35201550909443",
        "4722366342132156858368",
        "-4722366377333707767811",
    );
    test(
        "-26298808336",
        "170141183460469156173823577801560686592",
        "-170141183460469156173823577827859494928",
    );
    test(
        "170141183460469156173823577801560686592",
        "-26298808336",
        "-170141183460469156173823577827859494928",
    );
    test(
        "-19191697422411034898997120",
        "-748288838313422294120286634350663119087542623797248",
        "748288838313422294120286615158965696676507724800128",
    );
    test(
        "-748288838313422294120286634350663119087542623797248",
        "-19191697422411034898997120",
        "748288838313422294120286615158965696676507724800128",
    );
}

#[test]
fn limbs_xor_pos_neg_properties() {
    test_properties(pairs_of_u32_vec_var_1, |&(ref xs, ref ys)| {
        assert_eq!(
            -Natural::from_owned_limbs_asc(limbs_xor_pos_neg(xs, ys)),
            Integer::from(Natural::from_limbs_asc(xs)) ^ -Natural::from_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_xor_pos_neg_to_out_properties() {
    test_properties(
        triples_of_u32_vec_var_7,
        |&(ref out_limbs, ref xs, ref ys)| {
            let mut out_limbs = out_limbs.to_vec();
            let out_limbs_old = out_limbs.clone();
            limbs_xor_pos_neg_to_out(&mut out_limbs, xs, ys);
            let len = max(xs.len(), ys.len());
            assert_eq!(
                -Natural::from_limbs_asc(&out_limbs[..len]),
                Integer::from(Natural::from_limbs_asc(xs)) ^ -Natural::from_limbs_asc(ys)
            );
            assert_eq!(&out_limbs[len..], &out_limbs_old[len..]);
        },
    );
}

#[test]
fn limbs_xor_pos_neg_in_place_left_properties() {
    test_properties(pairs_of_u32_vec_var_1, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_xor_pos_neg_in_place_left(&mut xs, ys);
        assert_eq!(
            -Natural::from_owned_limbs_asc(xs),
            Integer::from(Natural::from_owned_limbs_asc(xs_old)) ^ -Natural::from_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_xor_pos_neg_in_place_right_properties() {
    test_properties(pairs_of_u32_vec_var_1, |&(ref xs, ref ys)| {
        let mut ys = ys.to_vec();
        let ys_old = ys.clone();
        limbs_xor_pos_neg_in_place_right(xs, &mut ys);
        assert_eq!(
            -Natural::from_owned_limbs_asc(ys),
            Integer::from(Natural::from_limbs_asc(xs)) ^ -Natural::from_owned_limbs_asc(ys_old)
        );
    });
}

#[test]
fn limbs_xor_pos_neg_in_place_either_properties() {
    test_properties(pairs_of_u32_vec_var_1, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let mut ys = ys.to_vec();
        let ys_old = ys.clone();
        let right = limbs_xor_pos_neg_in_place_either(&mut xs, &mut ys);
        let expected =
            Integer::from(Natural::from_limbs_asc(&xs_old)) ^ -Natural::from_limbs_asc(&ys_old);
        if right {
            assert_eq!(xs, xs_old);
            assert_eq!(-Natural::from_owned_limbs_asc(ys), expected);
        } else {
            assert_eq!(-Natural::from_owned_limbs_asc(xs), expected);
            assert_eq!(ys, ys_old);
        }
    });
}

#[test]
fn limbs_xor_neg_neg_properties() {
    test_properties(pairs_of_u32_vec_var_1, |&(ref xs, ref ys)| {
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_xor_neg_neg(xs, ys)),
            -Natural::from_limbs_asc(xs) ^ -Natural::from_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_xor_neg_neg_to_out_properties() {
    test_properties(triples_of_u32_vec_var_7, |&(ref xs, ref ys, ref zs)| {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_xor_neg_neg_to_out(&mut xs, ys, zs);
        let len = max(ys.len(), zs.len());
        let result =
            Natural::checked_from(-Natural::from_limbs_asc(ys) ^ -Natural::from_limbs_asc(zs))
                .unwrap();
        let mut expected_limbs = result.to_limbs_asc();
        expected_limbs.resize(len, 0);
        assert_eq!(&xs[..len], expected_limbs.as_slice());
        assert_eq!(&xs[len..], &xs_old[len..]);
    });
}

#[test]
fn limbs_xor_neg_neg_in_place_left_properties() {
    test_properties(pairs_of_u32_vec_var_1, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_xor_neg_neg_in_place_left(&mut xs, ys);
        assert_eq!(
            Natural::from_owned_limbs_asc(xs),
            -Natural::from_owned_limbs_asc(xs_old) ^ -Natural::from_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_xor_neg_neg_in_place_either_properties() {
    test_properties(pairs_of_u32_vec_var_1, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let mut ys = ys.to_vec();
        let ys_old = ys.clone();
        let right = limbs_xor_neg_neg_in_place_either(&mut xs, &mut ys);
        let expected = -Natural::from_limbs_asc(&xs_old) ^ -Natural::from_limbs_asc(&ys_old);
        if right {
            assert_eq!(xs, xs_old);
            assert_eq!(Natural::from_owned_limbs_asc(ys), expected);
        } else {
            assert_eq!(Natural::from_owned_limbs_asc(xs), expected);
            assert_eq!(ys, ys_old);
        }
    });
}

#[test]
fn xor_properties() {
    test_properties(pairs_of_integers, |&(ref x, ref y)| {
        let result_val_val = x.clone() ^ y.clone();
        let result_val_ref = x.clone() ^ y;
        let result_ref_val = x ^ y.clone();
        let result = x ^ y;
        assert!(result_val_val.is_valid());
        assert!(result_val_ref.is_valid());
        assert!(result_ref_val.is_valid());
        assert!(result.is_valid());
        assert_eq!(result_val_val, result, "{} {}", x, y);
        assert_eq!(result_val_ref, result);
        assert_eq!(result_ref_val, result);

        let mut mut_x = x.clone();
        mut_x ^= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, result);
        let mut mut_x = x.clone();
        mut_x ^= y;
        assert_eq!(mut_x, result);
        assert!(mut_x.is_valid());

        let mut mut_x = integer_to_rug_integer(x);
        mut_x ^= integer_to_rug_integer(y);
        assert_eq!(rug_integer_to_integer(&mut_x), result);

        assert_eq!(
            rug_integer_to_integer(&(integer_to_rug_integer(x) ^ integer_to_rug_integer(y))),
            result
        );

        assert_eq!(integer_xor_alt_1(&x, y), result);
        assert_eq!(integer_xor_alt_2(&x, y), result);

        assert_eq!(y ^ x, result);
        assert_eq!(&result ^ x, *y);
        assert_eq!(&result ^ y, *x);
        assert_eq!(!x ^ !y, result);
        assert_eq!(!(x ^ !y), result);
        assert_eq!(!(!x ^ y), result);
    });

    test_properties(
        pairs_of_integer_and_signed,
        |&(ref x, y): &(Integer, i32)| {
            let result = x ^ Integer::from(y);
            assert_eq!(x ^ y, result);
            assert_eq!(y ^ x, result);
        },
    );

    test_properties(integers, |x| {
        assert_eq!(x ^ Integer::ZERO, *x);
        assert_eq!(Integer::ZERO ^ x, *x);
        assert_eq!(x ^ Integer::NEGATIVE_ONE, !x);
        assert_eq!(Integer::NEGATIVE_ONE ^ x, !x);
        assert_eq!(x ^ x, 0);
        assert_eq!(x ^ !x, -1);
        assert_eq!(!x ^ x, -1);
    });

    test_properties(triples_of_integers, |&(ref x, ref y, ref z)| {
        assert_eq!((x ^ y) ^ z, x ^ (y ^ z));
    });
}
