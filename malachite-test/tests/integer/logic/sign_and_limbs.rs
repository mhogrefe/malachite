use common::LARGE_LIMIT;
use malachite_nz::integer::Integer;
use malachite_test::common::GenerationMode;
use malachite_test::integer::logic::sign_and_limbs::select_inputs;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_sign_and_limbs_le() {
    let test = |n, out| {
        assert_eq!(Integer::from_str(n).unwrap().sign_and_limbs_le(), out);
    };
    test("0", (Ordering::Equal, Vec::new()));
    test("123", (Ordering::Greater, vec![123]));
    test("-123", (Ordering::Less, vec![123]));
    test(
        "1000000000000",
        (Ordering::Greater, vec![3_567_587_328, 232]),
    );
    test("-1000000000000", (Ordering::Less, vec![3_567_587_328, 232]));
    test(
        "1701411834921604967429270619762735448065",
        (Ordering::Greater, vec![1, 2, 3, 4, 5]),
    );
    test(
        "-1701411834921604967429270619762735448065",
        (Ordering::Less, vec![1, 2, 3, 4, 5]),
    );
    test("4294967295", (Ordering::Greater, vec![u32::max_value()]));
    test("-4294967295", (Ordering::Less, vec![u32::max_value()]));
    test("4294967296", (Ordering::Greater, vec![0, 1]));
    test("-4294967296", (Ordering::Less, vec![0, 1]));
    test(
        "18446744073709551615",
        (Ordering::Greater, vec![u32::max_value(), u32::max_value()]),
    );
    test(
        "-18446744073709551615",
        (Ordering::Less, vec![u32::max_value(), u32::max_value()]),
    );
    test("18446744073709551616", (Ordering::Greater, vec![0, 0, 1]));
    test("-18446744073709551616", (Ordering::Less, vec![0, 0, 1]));
}

#[test]
fn test_sign_and_limbs_be() {
    let test = |n, out| {
        assert_eq!(Integer::from_str(n).unwrap().sign_and_limbs_be(), out);
    };
    test("0", (Ordering::Equal, Vec::new()));
    test("123", (Ordering::Greater, vec![123]));
    test("-123", (Ordering::Less, vec![123]));
    test(
        "1000000000000",
        (Ordering::Greater, vec![232, 3_567_587_328]),
    );
    test("-1000000000000", (Ordering::Less, vec![232, 3_567_587_328]));
    test(
        "1701411834921604967429270619762735448065",
        (Ordering::Greater, vec![5, 4, 3, 2, 1]),
    );
    test(
        "-1701411834921604967429270619762735448065",
        (Ordering::Less, vec![5, 4, 3, 2, 1]),
    );
    test("4294967295", (Ordering::Greater, vec![u32::max_value()]));
    test("-4294967295", (Ordering::Less, vec![u32::max_value()]));
    test("4294967296", (Ordering::Greater, vec![1, 0]));
    test("-4294967296", (Ordering::Less, vec![1, 0]));
    test(
        "18446744073709551615",
        (Ordering::Greater, vec![u32::max_value(), u32::max_value()]),
    );
    test(
        "-18446744073709551615",
        (Ordering::Less, vec![u32::max_value(), u32::max_value()]),
    );
    test("18446744073709551616", (Ordering::Greater, vec![1, 0, 0]));
    test("-18446744073709551616", (Ordering::Less, vec![1, 0, 0]));
}

#[test]
fn sign_and_limbs_le_properties() {
    // (sign, limbs) := x.sign_and_limbs_le(); from_sign_and_limbs_le(sign, limbs) == x
    // (sign, limbs) := x.sign_and_limbs_le(); x.sign_and_limbs_be() == (sign, limbs.rev())
    // (sign, limbs) := x.sign_and_limbs_le();
    //     (sign == Ordering::Equals) == limbs.is_empty() == (x == 0)
    // (sign, limbs) := x.sign_and_limbs_le(); if x != 0, limbs.last() != 0
    // (sign, limbs) := x.sign_and_limbs_le(); (-x).sign_and_limbs_le() == (sign.reverse(), limbs)
    let one_integer = |x: Integer| {
        let (sign, limbs) = x.sign_and_limbs_le();
        assert_eq!(Integer::from_sign_and_limbs_le(sign, &limbs), x);
        assert_eq!(
            x.sign_and_limbs_be(),
            (sign, limbs.iter().cloned().rev().collect::<Vec<u32>>(),)
        );
        assert_eq!(sign == Ordering::Equal, limbs.is_empty());
        assert_eq!(sign == Ordering::Equal, x == 0);
        if x != 0 {
            assert_ne!(*limbs.last().unwrap(), 0);
        }
        assert_eq!((-x).sign_and_limbs_le(), (sign.reverse(), limbs));
    };

    for n in exhaustive_integers().take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in random_integers(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_integer(n);
    }
}

#[test]
fn sign_and_limbs_be_properties() {
    // (sign, limbs) := x.sign_and_limbs_be(); from_sign_and_limbs_be(sign, limbs) == x
    // (sign, limbs) := x.sign_and_limbs_be(); x.sign_and_limbs_le() == (sign, limbs.rev())
    // (sign, limbs) := x.sign_and_limbs_be();
    //     (sign == Ordering::Equals) == limbs.is_empty() == (x == 0)
    // (sign, limbs) := x.sign_and_limbs_be(); if x != 0, limbs[0] != 0
    // (sign, limbs) := x.sign_and_limbs_be(); (-x).sign_and_limbs_be() == (sign.reverse(), limbs)
    let one_integer = |x: Integer| {
        let (sign, limbs) = x.sign_and_limbs_be();
        assert_eq!(Integer::from_sign_and_limbs_be(sign, &limbs), x);
        assert_eq!(
            x.sign_and_limbs_le(),
            (sign, limbs.iter().cloned().rev().collect::<Vec<u32>>(),)
        );
        assert_eq!(sign == Ordering::Equal, limbs.is_empty());
        assert_eq!(sign == Ordering::Equal, x == 0);
        if x != 0 {
            assert_ne!(limbs[0], 0);
        }
        assert_eq!((-x).sign_and_limbs_be(), (sign.reverse(), limbs));
    };

    for n in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }
}
