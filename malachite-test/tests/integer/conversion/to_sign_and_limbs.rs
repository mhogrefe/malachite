use common::test_properties;
#[cfg(feature = "32_bit_limbs")]
use malachite_base::comparison::Max;
use malachite_nz::integer::Integer;
use malachite_nz::platform::Limb;
use malachite_test::inputs::integer::integers;
use std::cmp::Ordering;
#[cfg(feature = "32_bit_limbs")]
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_to_sign_and_limbs_asc() {
    let test = |n, out| {
        let n = Integer::from_str(n).unwrap();
        assert_eq!(n.to_sign_and_limbs_asc(), out);
        assert_eq!(n.into_sign_and_limbs_asc(), out);
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
    test("4294967295", (Ordering::Greater, vec![Limb::MAX]));
    test("-4294967295", (Ordering::Less, vec![Limb::MAX]));
    test("4294967296", (Ordering::Greater, vec![0, 1]));
    test("-4294967296", (Ordering::Less, vec![0, 1]));
    test(
        "18446744073709551615",
        (Ordering::Greater, vec![Limb::MAX, Limb::MAX]),
    );
    test(
        "-18446744073709551615",
        (Ordering::Less, vec![Limb::MAX, Limb::MAX]),
    );
    test("18446744073709551616", (Ordering::Greater, vec![0, 0, 1]));
    test("-18446744073709551616", (Ordering::Less, vec![0, 0, 1]));
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_to_sign_and_limbs_desc() {
    let test = |n, out| {
        let n = Integer::from_str(n).unwrap();
        assert_eq!(n.to_sign_and_limbs_desc(), out);
        assert_eq!(n.into_sign_and_limbs_desc(), out);
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
    test("4294967295", (Ordering::Greater, vec![Limb::MAX]));
    test("-4294967295", (Ordering::Less, vec![Limb::MAX]));
    test("4294967296", (Ordering::Greater, vec![1, 0]));
    test("-4294967296", (Ordering::Less, vec![1, 0]));
    test(
        "18446744073709551615",
        (Ordering::Greater, vec![Limb::MAX, Limb::MAX]),
    );
    test(
        "-18446744073709551615",
        (Ordering::Less, vec![Limb::MAX, Limb::MAX]),
    );
    test("18446744073709551616", (Ordering::Greater, vec![1, 0, 0]));
    test("-18446744073709551616", (Ordering::Less, vec![1, 0, 0]));
}

#[test]
fn to_sign_and_limbs_asc_properties() {
    test_properties(integers, |x| {
        let (sign, limbs) = x.to_sign_and_limbs_asc();
        assert_eq!(x.clone().into_sign_and_limbs_asc(), (sign, limbs.clone()));
        assert_eq!(Integer::from_sign_and_limbs_asc(sign, &limbs), *x);
        assert_eq!(
            x.to_sign_and_limbs_desc(),
            (sign, limbs.iter().cloned().rev().collect::<Vec<Limb>>())
        );
        assert_eq!(sign == Ordering::Equal, limbs.is_empty());
        assert_eq!(sign == Ordering::Equal, *x == 0 as Limb);
        if *x != 0 as Limb {
            assert_ne!(*limbs.last().unwrap(), 0 as Limb);
        }
        assert_eq!((-x).to_sign_and_limbs_asc(), (sign.reverse(), limbs));
    });
}

#[test]
fn to_sign_and_limbs_desc_properties() {
    test_properties(integers, |x| {
        let (sign, limbs) = x.to_sign_and_limbs_desc();
        assert_eq!(x.clone().into_sign_and_limbs_desc(), (sign, limbs.clone()));
        assert_eq!(Integer::from_sign_and_limbs_desc(sign, &limbs), *x);
        assert_eq!(
            x.to_sign_and_limbs_asc(),
            (sign, limbs.iter().cloned().rev().collect::<Vec<Limb>>())
        );
        assert_eq!(sign == Ordering::Equal, limbs.is_empty());
        assert_eq!(sign == Ordering::Equal, *x == 0 as Limb);
        if *x != 0 as Limb {
            assert_ne!(limbs[0], 0 as Limb);
        }
        assert_eq!((-x).to_sign_and_limbs_desc(), (sign.reverse(), limbs));
    });
}
