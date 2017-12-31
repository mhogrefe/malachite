use common::LARGE_LIMIT;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::{gmp_natural_to_native, GenerationMode};
use malachite_test::natural::logic::limbs::select_inputs;
use std::str::FromStr;

#[test]
fn test_to_limbs_le() {
    let test = |n, out| {
        assert_eq!(native::Natural::from_str(n).unwrap().to_limbs_le(), out);
        assert_eq!(gmp::Natural::from_str(n).unwrap().to_limbs_le(), out);
    };
    test("0", vec![]);
    test("123", vec![123]);
    test("1000000000000", vec![3567587328, 232]);
    test(
        "1701411834921604967429270619762735448065",
        vec![1, 2, 3, 4, 5],
    );
    test("4294967295", vec![4294967295]);
    test("4294967296", vec![0, 1]);
    test("18446744073709551615", vec![4294967295, 4294967295]);
    test("18446744073709551616", vec![0, 0, 1]);
}

#[test]
fn test_to_limbs_be() {
    let test = |n, out| {
        assert_eq!(native::Natural::from_str(n).unwrap().to_limbs_be(), out);
        assert_eq!(gmp::Natural::from_str(n).unwrap().to_limbs_be(), out);
    };
    test("0", vec![]);
    test("123", vec![123]);
    test("1000000000000", vec![232, 3567587328]);
    test(
        "1701411834921604967429270619762735448065",
        vec![5, 4, 3, 2, 1],
    );
    test("4294967295", vec![4294967295]);
    test("4294967296", vec![1, 0]);
    test("18446744073709551615", vec![4294967295, 4294967295]);
    test("18446744073709551616", vec![1, 0, 0]);
}

#[test]
fn to_limbs_le_properties() {
    // x.to_limbs_le() is equivalent for malachite-gmp and malachite-native.
    // from_limbs_le(x.to_limbs_le()) == x
    // x.to_limbs_le().rev() == x.to_limbs_be()
    // if x != 0, x.to_limbs_le().last() != 0
    let one_natural = |gmp_x: gmp::Natural| {
        let x = gmp_natural_to_native(&gmp_x);
        let limbs = x.to_limbs_le();
        assert_eq!(gmp_x.to_limbs_le(), limbs);
        assert_eq!(native::Natural::from_limbs_le(&limbs), x);
        assert_eq!(
            x.to_limbs_be(),
            limbs.iter().cloned().rev().collect::<Vec<u32>>()
        );
        if x != 0 {
            assert_ne!(*limbs.last().unwrap(), 0);
        }
    };

    for n in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_natural(n);
    }
}

#[test]
fn limbs_be_properties() {
    // x.to_limbs_be() is equivalent for malachite-gmp and malachite-native.
    // from_limbs_be(x.to_limbs_be()) == x
    // x.to_limbs_be().rev() == x.to_limbs_le()
    // if x != 0, x.to_limbs_be().last() != 0
    let one_natural = |gmp_x: gmp::Natural| {
        let x = gmp_natural_to_native(&gmp_x);
        let limbs = x.to_limbs_be();
        assert_eq!(gmp_x.to_limbs_be(), limbs);
        assert_eq!(native::Natural::from_limbs_be(&limbs), x);
        assert_eq!(
            x.to_limbs_le(),
            limbs.iter().cloned().rev().collect::<Vec<u32>>()
        );
        if x != 0 {
            assert_ne!(limbs[0], 0);
        }
    };

    for n in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_natural(n);
    }
}
