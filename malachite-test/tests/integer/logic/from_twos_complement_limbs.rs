use common::LARGE_LIMIT;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, GenerationMode};
use malachite_test::integer::logic::from_twos_complement_limbs::select_inputs;
use std::cmp::Ordering;

#[test]
fn test_from_from_twos_complement_limbs_le() {
    let test = |limbs: &[u32], out| {
        let x = native::Integer::from_twos_complement_limbs_le(limbs);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = gmp::Integer::from_twos_complement_limbs_le(limbs);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test(&[], "0");
    test(&[0], "0");
    test(&[0, 0, 0], "0");
    test(&[123], "123");
    test(&[123, 0], "123");
    test(&[4_294_967_173], "-123");
    test(&[4_294_967_173, u32::max_value()], "-123");
    test(&[3_567_587_328, 232], "1000000000000");
    test(&[727_379_968, 4_294_967_063], "-1000000000000");
    test(&[1, 2, 3, 4, 5], "1701411834921604967429270619762735448065");
    test(
        &[
            u32::max_value(),
            u32::max_value() - 2,
            u32::max_value() - 3,
            u32::max_value() - 4,
            u32::max_value() - 5,
        ],
        "-1701411834921604967429270619762735448065",
    );
    test(&[u32::max_value(), 0], "4294967295");
    test(&[1, u32::max_value()], "-4294967295");
    test(&[0, 1], "4294967296");
    test(&[0, u32::max_value()], "-4294967296");
    test(
        &[u32::max_value(), u32::max_value(), 0],
        "18446744073709551615",
    );
    test(&[1, 0, u32::max_value()], "-18446744073709551615");
    test(&[0, 0, 1], "18446744073709551616");
    test(&[0, 0, u32::max_value()], "-18446744073709551616");
}

#[test]
fn test_from_from_twos_complement_limbs_be() {
    let test = |limbs: &[u32], out| {
        let x = native::Integer::from_twos_complement_limbs_be(limbs);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = gmp::Integer::from_twos_complement_limbs_be(limbs);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test(&[], "0");
    test(&[0], "0");
    test(&[0, 0, 0], "0");
    test(&[123], "123");
    test(&[0, 123], "123");
    test(&[4_294_967_173], "-123");
    test(&[u32::max_value(), 4_294_967_173], "-123");
    test(&[232, 3_567_587_328], "1000000000000");
    test(&[4_294_967_063, 727_379_968], "-1000000000000");
    test(&[5, 4, 3, 2, 1], "1701411834921604967429270619762735448065");
    test(
        &[
            u32::max_value() - 5,
            u32::max_value() - 4,
            u32::max_value() - 3,
            u32::max_value() - 2,
            u32::max_value(),
        ],
        "-1701411834921604967429270619762735448065",
    );
    test(&[0, u32::max_value()], "4294967295");
    test(&[u32::max_value(), 1], "-4294967295");
    test(&[1, 0], "4294967296");
    test(&[u32::max_value(), 0], "-4294967296");
    test(
        &[0, u32::max_value(), u32::max_value()],
        "18446744073709551615",
    );
    test(&[u32::max_value(), 0, 1], "-18446744073709551615");
    test(&[1, 0, 0], "18446744073709551616");
    test(&[u32::max_value(), 0, 0], "-18446744073709551616");
}

fn trim_be_limbs(xs: &mut Vec<u32>) {
    if xs.is_empty() {
        return;
    }
    if xs[0] & 0x8000_0000 == 0 {
        match xs.iter().position(|&limb| limb != 0) {
            None => xs.clear(),
            Some(i) => {
                let i = if xs[i] & 0x8000_0000 != 0 { i - 1 } else { i };
                *xs = xs.iter().cloned().skip(i).collect();
            }
        }
    } else {
        match xs.iter().position(|&limb| limb != 0xffff_ffff) {
            None => {
                xs.clear();
                xs.push(0xffff_ffff);
            }
            Some(i) => {
                let i = if xs[i] & 0x8000_0000 == 0 { i - 1 } else { i };
                *xs = xs.iter().cloned().skip(i).collect();
            }
        }
    }
}

#[test]
fn from_twos_complement_limbs_le_properties() {
    // Integer::from_twos_complement_limbs_le(limbs) is equivalent for malachite-gmp and
    //      malachite-native.
    // (Integer::from_twos_complement_limbs_le(limbs) == x) ==
    //      (x.limbs_le() == limbs.rev().trim_be_limbs().rev())
    // Integer::from_twos_complement_limbs_le(limbs.reverse()) ==
    //      Integer::from_twos_complement_limbs_be(limbs)
    // if limbs is canonical, Integer::from_twos_complement_limbs_le(limbs).limbs_le() == x
    let u32_slice = |limbs: &[u32]| {
        let x = native::Integer::from_twos_complement_limbs_le(limbs);
        assert_eq!(
            gmp_integer_to_native(&gmp::Integer::from_twos_complement_limbs_le(limbs)),
            x
        );
        let mut trimmed_limbs: Vec<u32> = limbs.iter().cloned().rev().collect();
        trim_be_limbs(&mut trimmed_limbs);
        trimmed_limbs.reverse();
        assert_eq!(x.twos_complement_limbs_le(), trimmed_limbs);
        assert_eq!(
            native::Integer::from_twos_complement_limbs_be(&limbs
                .iter()
                .cloned()
                .rev()
                .collect::<Vec<u32>>(),),
            x
        );
        if match x.sign() {
            Ordering::Equal => limbs.is_empty(),
            Ordering::Greater => {
                let last = *limbs.last().unwrap();
                last & 0x8000_0000 == 0 && (last != 0 || limbs[limbs.len() - 2] & 0x8000_0000 != 0)
            }
            Ordering::Less => {
                let last = *limbs.last().unwrap();
                last & 0x8000_0000 != 0
                    && (last != !0 || limbs.len() <= 1 || limbs[limbs.len() - 2] & 0x8000_0000 == 0)
            }
        } {
            assert_eq!(&x.twos_complement_limbs_le()[..], limbs);
        }
    };

    for xs in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        u32_slice(&xs);
    }

    for xs in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        u32_slice(&xs);
    }
}

#[test]
fn from_twos_complement_limbs_be_properties() {
    // Integer::from_twos_complement_limbs_be(limbs) is equivalent for malachite-gmp and
    //      malachite-native.
    // (Integer::from_twos_complement_limbs_be(limbs) == x) ==
    //      (x.limbs_le() == limbs.trim_be_limbs())
    // Integer::from_twos_complement_limbs_be(limbs.reverse()) ==
    //      Integer::from_twos_complement_limbs_le(limbs)
    // if limbs is canonical, Integer::from_twos_complement_limbs_be(limbs).limbs_be() == x
    let u32_slice = |limbs: &[u32]| {
        let x = native::Integer::from_twos_complement_limbs_be(limbs);
        assert_eq!(
            gmp_integer_to_native(&gmp::Integer::from_twos_complement_limbs_be(limbs)),
            x
        );
        let mut trimmed_limbs: Vec<u32> = limbs.to_vec();
        trim_be_limbs(&mut trimmed_limbs);
        assert_eq!(x.twos_complement_limbs_be(), trimmed_limbs);
        assert_eq!(
            native::Integer::from_twos_complement_limbs_le(&limbs
                .iter()
                .cloned()
                .rev()
                .collect::<Vec<u32>>(),),
            x
        );
        if match x.sign() {
            Ordering::Equal => limbs.is_empty(),
            Ordering::Greater => {
                let first = limbs[0];
                first & 0x8000_0000 == 0 && (first != 0 || limbs[1] & 0x8000_0000 != 0)
            }
            Ordering::Less => {
                let first = limbs[0];
                first & 0x8000_0000 != 0
                    && (first != !0 || limbs.len() <= 1 || limbs[1] & 0x8000_0000 == 0)
            }
        } {
            assert_eq!(&x.twos_complement_limbs_be()[..], limbs);
        }
    };

    for xs in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        u32_slice(&xs);
    }

    for xs in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        u32_slice(&xs);
    }
}
