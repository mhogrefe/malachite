use common::test_properties;
use malachite_nz::integer::Integer;
use malachite_test::inputs::integer::integers;
use std::cmp::Ordering;
use std::u32;
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
    test("4294967295", (Ordering::Greater, vec![u32::MAX]));
    test("-4294967295", (Ordering::Less, vec![u32::MAX]));
    test("4294967296", (Ordering::Greater, vec![0, 1]));
    test("-4294967296", (Ordering::Less, vec![0, 1]));
    test(
        "18446744073709551615",
        (Ordering::Greater, vec![u32::MAX, u32::MAX]),
    );
    test(
        "-18446744073709551615",
        (Ordering::Less, vec![u32::MAX, u32::MAX]),
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
    test("4294967295", (Ordering::Greater, vec![u32::MAX]));
    test("-4294967295", (Ordering::Less, vec![u32::MAX]));
    test("4294967296", (Ordering::Greater, vec![1, 0]));
    test("-4294967296", (Ordering::Less, vec![1, 0]));
    test(
        "18446744073709551615",
        (Ordering::Greater, vec![u32::MAX, u32::MAX]),
    );
    test(
        "-18446744073709551615",
        (Ordering::Less, vec![u32::MAX, u32::MAX]),
    );
    test("18446744073709551616", (Ordering::Greater, vec![1, 0, 0]));
    test("-18446744073709551616", (Ordering::Less, vec![1, 0, 0]));
}

#[test]
fn sign_and_limbs_le_properties() {
    test_properties(integers, |x| {
        let (sign, limbs) = x.sign_and_limbs_le();
        assert_eq!(Integer::from_sign_and_limbs_le(sign, &limbs), *x);
        assert_eq!(
            x.sign_and_limbs_be(),
            (sign, limbs.iter().cloned().rev().collect::<Vec<u32>>(),)
        );
        assert_eq!(sign == Ordering::Equal, limbs.is_empty());
        assert_eq!(sign == Ordering::Equal, *x == 0);
        if *x != 0 {
            assert_ne!(*limbs.last().unwrap(), 0);
        }
        assert_eq!((-x).sign_and_limbs_le(), (sign.reverse(), limbs));
    });
}

#[test]
fn sign_and_limbs_be_properties() {
    test_properties(integers, |x| {
        let (sign, limbs) = x.sign_and_limbs_be();
        assert_eq!(Integer::from_sign_and_limbs_be(sign, &limbs), *x);
        assert_eq!(
            x.sign_and_limbs_le(),
            (sign, limbs.iter().cloned().rev().collect::<Vec<u32>>(),)
        );
        assert_eq!(sign == Ordering::Equal, limbs.is_empty());
        assert_eq!(sign == Ordering::Equal, *x == 0);
        if *x != 0 {
            assert_ne!(limbs[0], 0);
        }
        assert_eq!((-x).sign_and_limbs_be(), (sign.reverse(), limbs));
    });
}
