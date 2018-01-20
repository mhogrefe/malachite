use common::LARGE_LIMIT;
use malachite_nz::integer::Integer;
use malachite_test::common::GenerationMode;
use malachite_test::integer::logic::twos_complement_limbs::select_inputs;
use std::cmp::Ordering;
use std::str::FromStr;
use std::u32;

#[test]
fn test_twos_complement_limbs_le() {
    let test = |n, out| {
        assert_eq!(
            Integer::from_str(n).unwrap().twos_complement_limbs_le(),
            out
        );
    };
    test("0", vec![]);
    test("123", vec![123]);
    test("-123", vec![4_294_967_173]);
    test("1000000000000", vec![3_567_587_328, 232]);
    test("-1000000000000", vec![727_379_968, 4_294_967_063]);
    test(
        "1701411834921604967429270619762735448065",
        vec![1, 2, 3, 4, 5],
    );
    test(
        "-1701411834921604967429270619762735448065",
        vec![
            u32::MAX,
            u32::MAX - 2,
            u32::MAX - 3,
            u32::MAX - 4,
            u32::MAX - 5,
        ],
    );
    test("4294967295", vec![u32::MAX, 0]);
    test("-4294967295", vec![1, u32::MAX]);
    test("4294967296", vec![0, 1]);
    test("-4294967296", vec![0, u32::MAX]);
    test("18446744073709551615", vec![u32::MAX, u32::MAX, 0]);
    test("-18446744073709551615", vec![1, 0, u32::MAX]);
    test("18446744073709551616", vec![0, 0, 1]);
    test("-18446744073709551616", vec![0, 0, u32::MAX]);
}

#[test]
fn test_twos_complement_limbs_be() {
    let test = |n, out| {
        assert_eq!(
            Integer::from_str(n).unwrap().twos_complement_limbs_be(),
            out
        );
    };
    test("0", vec![]);
    test("123", vec![123]);
    test("-123", vec![4_294_967_173]);
    test("1000000000000", vec![232, 3_567_587_328]);
    test("-1000000000000", vec![4_294_967_063, 727_379_968]);
    test(
        "1701411834921604967429270619762735448065",
        vec![5, 4, 3, 2, 1],
    );
    test(
        "-1701411834921604967429270619762735448065",
        vec![
            u32::MAX - 5,
            u32::MAX - 4,
            u32::MAX - 3,
            u32::MAX - 2,
            u32::MAX,
        ],
    );
    test("4294967295", vec![0, u32::MAX]);
    test("-4294967295", vec![u32::MAX, 1]);
    test("4294967296", vec![1, 0]);
    test("-4294967296", vec![u32::MAX, 0]);
    test("18446744073709551615", vec![0, u32::MAX, u32::MAX]);
    test("-18446744073709551615", vec![u32::MAX, 0, 1]);
    test("18446744073709551616", vec![1, 0, 0]);
    test("-18446744073709551616", vec![u32::MAX, 0, 0]);
}

#[test]
fn twos_complement_limbs_le_properties() {
    // from_twos_complement_limbs_le(x.twos_complement_limbs_le()) == x
    // x.twos_complement_limbs_le().rev() == x.twos_complement_limbs_be()
    // if x != 0, limbs is empty.
    // if x > 0, limbs.last() == 0 => limbs[limbs.len() - 2].get_bit(31) == true
    // if x < -1, limbs.last() == !0 => limbs[limbs.len() - 2].get_bit(31) == false
    let one_integer = |x: Integer| {
        let limbs = x.twos_complement_limbs_le();
        assert_eq!(Integer::from_twos_complement_limbs_le(&limbs), x);
        assert_eq!(
            x.twos_complement_limbs_be(),
            limbs.iter().cloned().rev().collect::<Vec<u32>>()
        );
        match x.sign() {
            Ordering::Equal => assert!(limbs.is_empty()),
            Ordering::Greater => {
                let last = *limbs.last().unwrap();
                assert_eq!(last & 0x8000_0000, 0);
                if last == 0 {
                    assert_ne!(limbs[limbs.len() - 2] & 0x8000_0000, 0);
                }
            }
            Ordering::Less => {
                let last = *limbs.last().unwrap();
                assert_ne!(last & 0x8000_0000, 0);
                if last == !0 && limbs.len() > 1 {
                    assert_eq!(limbs[limbs.len() - 2] & 0x8000_0000, 0);
                }
            }
        }
    };

    for n in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }
}

#[test]
fn limbs_be_properties() {
    // from_twos_complement_limbs_be(x.twos_complement_limbs_be()) == x
    // x.twos_complement_limbs_be().rev() == x.twos_complement_limbs_le()
    // if x != 0, limbs is empty.
    // if x > 0, limbs[0] == 0 => limbs[1].get_bit(31) == true
    // if x < -1, limbs[0] == !0 => limbs[1].get_bit(31) == false
    let one_integer = |x: Integer| {
        let limbs = x.twos_complement_limbs_be();
        assert_eq!(Integer::from_twos_complement_limbs_be(&limbs), x);
        assert_eq!(
            x.twos_complement_limbs_le(),
            limbs.iter().cloned().rev().collect::<Vec<u32>>()
        );
        match x.sign() {
            Ordering::Equal => assert!(limbs.is_empty()),
            Ordering::Greater => {
                let first = limbs[0];
                assert_eq!(first & 0x8000_0000, 0);
                if first == 0 {
                    assert_ne!(limbs[1] & 0x8000_0000, 0);
                }
            }
            Ordering::Less => {
                let first = limbs[0];
                assert_ne!(first & 0x8000_0000, 0);
                if first == !0 && limbs.len() > 1 {
                    assert_eq!(limbs[1] & 0x8000_0000, 0);
                }
            }
        }
    };

    for n in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }
}
