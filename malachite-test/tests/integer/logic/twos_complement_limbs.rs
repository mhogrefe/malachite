use common::test_properties;
use malachite_base::num::{BitAccess, PrimitiveInteger};
use malachite_nz::integer::Integer;
use malachite_test::inputs::integer::integers;
use std::cmp::Ordering;
use std::str::FromStr;
use std::u32;

#[test]
fn test_twos_complement_limbs_asc() {
    let test = |n, out| {
        assert_eq!(
            Integer::from_str(n).unwrap().twos_complement_limbs_asc(),
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
fn test_twos_complement_limbs_desc() {
    let test = |n, out| {
        assert_eq!(
            Integer::from_str(n).unwrap().twos_complement_limbs_desc(),
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

const LAST_INDEX: u64 = u32::WIDTH as u64 - 1;

#[test]
fn twos_complement_limbs_asc_properties() {
    test_properties(integers, |x| {
        let limbs = x.twos_complement_limbs_asc();
        assert_eq!(Integer::from_twos_complement_limbs_asc(&limbs), *x);
        assert_eq!(
            x.twos_complement_limbs_desc(),
            limbs.iter().cloned().rev().collect::<Vec<u32>>()
        );
        match x.sign() {
            Ordering::Equal => assert!(limbs.is_empty()),
            Ordering::Greater => {
                let last = *limbs.last().unwrap();
                assert!(!last.get_bit(LAST_INDEX));
                if last == 0 {
                    assert!(limbs[limbs.len() - 2].get_bit(LAST_INDEX));
                }
            }
            Ordering::Less => {
                let last = *limbs.last().unwrap();
                assert!(last.get_bit(LAST_INDEX));
                if last == !0 && limbs.len() > 1 {
                    assert!(!limbs[limbs.len() - 2].get_bit(LAST_INDEX));
                }
            }
        }
    });
}

#[test]
fn limbs_desc_properties() {
    test_properties(integers, |x| {
        let limbs = x.twos_complement_limbs_desc();
        assert_eq!(Integer::from_twos_complement_limbs_desc(&limbs), *x);
        assert_eq!(
            x.twos_complement_limbs_asc(),
            limbs.iter().cloned().rev().collect::<Vec<u32>>()
        );
        match x.sign() {
            Ordering::Equal => assert!(limbs.is_empty()),
            Ordering::Greater => {
                let first = limbs[0];
                assert!(!first.get_bit(LAST_INDEX));
                if first == 0 {
                    assert!(limbs[1].get_bit(LAST_INDEX));
                }
            }
            Ordering::Less => {
                let first = limbs[0];
                assert!(first.get_bit(LAST_INDEX));
                if first == !0 && limbs.len() > 1 {
                    assert!(!limbs[1].get_bit(LAST_INDEX));
                }
            }
        }
    });
}
