use common::LARGE_LIMIT;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, GenerationMode};
use malachite_test::integer::logic::from_sign_and_limbs::select_inputs;
use std::cmp::Ordering;

#[test]
fn test_from_sign_and_limbs_le() {
    let test = |sign: Ordering, limbs: &[u32], out| {
        let x = native::Integer::from_sign_and_limbs_le(sign, limbs);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = gmp::Integer::from_sign_and_limbs_le(sign, limbs);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test(Ordering::Equal, &[], "0");
    test(Ordering::Equal, &[0], "0");
    test(Ordering::Equal, &[0, 0, 0], "0");
    test(Ordering::Greater, &[123], "123");
    test(Ordering::Less, &[123], "-123");
    test(Ordering::Greater, &[123, 0], "123");
    test(Ordering::Less, &[123, 0], "-123");
    test(Ordering::Greater, &[123, 0, 0, 0], "123");
    test(Ordering::Less, &[123, 0, 0, 0], "-123");
    test(Ordering::Greater, &[3567587328, 232], "1000000000000");
    test(Ordering::Less, &[3567587328, 232], "-1000000000000");
    test(Ordering::Greater, &[3567587328, 232, 0], "1000000000000");
    test(Ordering::Less, &[3567587328, 232, 0], "-1000000000000");
    test(
        Ordering::Greater,
        &[1, 2, 3, 4, 5],
        "1701411834921604967429270619762735448065",
    );
    test(
        Ordering::Less,
        &[1, 2, 3, 4, 5],
        "-1701411834921604967429270619762735448065",
    );
}

#[test]
#[should_panic(expected = "sign should be Equal iff limbs only contains zeros. sign: Equal, \
                           limbs: [1]")]
fn from_sign_and_limbs_le_fail_native_1() {
    native::Integer::from_sign_and_limbs_le(Ordering::Equal, &[1]);
}

#[test]
#[should_panic(expected = "sign should be Equal iff limbs only contains zeros. sign: Greater, \
                           limbs: []")]
fn from_sign_and_limbs_le_fail_native_2() {
    native::Integer::from_sign_and_limbs_le(Ordering::Greater, &[]);
}

#[test]
#[should_panic(expected = "sign should be Equal iff limbs only contains zeros. sign: Greater, \
                           limbs: [0, 0, 0]")]
fn from_sign_and_limbs_le_fail_native_3() {
    native::Integer::from_sign_and_limbs_le(Ordering::Greater, &[0, 0, 0]);
}

#[test]
#[should_panic(expected = "sign should be Equal iff limbs only contains zeros. sign: Equal, \
                           limbs: [1]")]
fn from_sign_and_limbs_le_fail_gmp_1() {
    gmp::Integer::from_sign_and_limbs_le(Ordering::Equal, &[1]);
}

#[test]
#[should_panic(expected = "sign should be Equal iff limbs only contains zeros. sign: Greater, \
                           limbs: []")]
fn from_sign_and_limbs_le_fail_gmp_2() {
    gmp::Integer::from_sign_and_limbs_le(Ordering::Greater, &[]);
}

#[test]
#[should_panic(expected = "sign should be Equal iff limbs only contains zeros. sign: Greater, \
                           limbs: [0, 0, 0]")]
fn from_sign_and_limbs_le_fail_gmp_3() {
    gmp::Integer::from_sign_and_limbs_le(Ordering::Greater, &[0, 0, 0]);
}

#[test]
fn test_from_sign_and_limbs_be() {
    let test = |sign: Ordering, limbs: &[u32], out| {
        let x = native::Integer::from_sign_and_limbs_be(sign, limbs);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = gmp::Integer::from_sign_and_limbs_be(sign, limbs);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test(Ordering::Equal, &[], "0");
    test(Ordering::Equal, &[0], "0");
    test(Ordering::Equal, &[0, 0, 0], "0");
    test(Ordering::Greater, &[123], "123");
    test(Ordering::Less, &[123], "-123");
    test(Ordering::Greater, &[0, 123], "123");
    test(Ordering::Less, &[0, 123], "-123");
    test(Ordering::Greater, &[0, 0, 0, 123], "123");
    test(Ordering::Less, &[0, 0, 0, 123], "-123");
    test(Ordering::Greater, &[232, 3567587328], "1000000000000");
    test(Ordering::Less, &[232, 3567587328], "-1000000000000");
    test(Ordering::Greater, &[0, 232, 3567587328], "1000000000000");
    test(Ordering::Less, &[0, 232, 3567587328], "-1000000000000");
    test(
        Ordering::Greater,
        &[5, 4, 3, 2, 1],
        "1701411834921604967429270619762735448065",
    );
    test(
        Ordering::Less,
        &[5, 4, 3, 2, 1],
        "-1701411834921604967429270619762735448065",
    );
}

#[test]
#[should_panic(expected = "sign should be Equal iff limbs only contains zeros. sign: Equal, \
                           limbs: [1]")]
fn from_sign_and_limbs_be_fail_native_1() {
    native::Integer::from_sign_and_limbs_be(Ordering::Equal, &[1]);
}

#[test]
#[should_panic(expected = "sign should be Equal iff limbs only contains zeros. sign: Greater, \
                           limbs: []")]
fn from_sign_and_limbs_be_fail_native_2() {
    native::Integer::from_sign_and_limbs_be(Ordering::Greater, &[]);
}

#[test]
#[should_panic(expected = "sign should be Equal iff limbs only contains zeros. sign: Greater, \
                           limbs: [0, 0, 0]")]
fn from_sign_and_limbs_be_fail_native_3() {
    native::Integer::from_sign_and_limbs_be(Ordering::Greater, &[0, 0, 0]);
}

#[test]
#[should_panic(expected = "sign should be Equal iff limbs only contains zeros. sign: Equal, \
                           limbs: [1]")]
fn from_sign_and_limbs_be_fail_gmp_1() {
    gmp::Integer::from_sign_and_limbs_be(Ordering::Equal, &[1]);
}

#[test]
#[should_panic(expected = "sign should be Equal iff limbs only contains zeros. sign: Greater, \
                           limbs: []")]
fn from_sign_and_limbs_be_fail_gmp_2() {
    gmp::Integer::from_sign_and_limbs_be(Ordering::Greater, &[]);
}

#[test]
#[should_panic(expected = "sign should be Equal iff limbs only contains zeros. sign: Greater, \
                           limbs: [0, 0, 0]")]
fn from_sign_and_limbs_be_fail_gmp_3() {
    gmp::Integer::from_sign_and_limbs_be(Ordering::Greater, &[0, 0, 0]);
}

#[test]
fn from_sign_and_limbs_le_properties() {
    // Integer::from_sign_and_limbs_le(sign, limbs) is equivalent for malachite-gmp and
    // malachite-native.
    // x := Integer::from_sign_and_limbs_le(sign, limbs); x.sign() == sign and
    //      x.limbs_le() == limbs.rev().skip_while(|u| u == 0).rev()
    // Integer::from_sign_and-limbs_le(sign, limbs.reverse()) ==
    //      Integer::from_sign_and_limbs_be(sign, limbs)
    let ordering_and_u32_slice = |sign: Ordering, limbs: &[u32]| {
        let x = native::Integer::from_sign_and_limbs_le(sign, limbs);
        assert_eq!(
            gmp_integer_to_native(&gmp::Integer::from_sign_and_limbs_le(sign, limbs)),
            x
        );
        let mut trimmed_limbs: Vec<u32> = limbs
            .iter()
            .cloned()
            .rev()
            .skip_while(|&u| u == 0)
            .collect();
        trimmed_limbs.reverse();
        let (sign_2, limbs_2) = x.sign_and_limbs_le();
        assert_eq!(sign_2, sign);
        assert_eq!(limbs_2, trimmed_limbs);
        assert_eq!(
            native::Integer::from_sign_and_limbs_be(
                sign,
                &limbs.iter().cloned().rev().collect::<Vec<u32>>(),
            ),
            x
        );
    };

    for (sign, limbs) in select_inputs(GenerationMode::Exhaustive)
        .filter(|&(sign, ref limbs)| {
            limbs.iter().all(|&limb| limb == 0) == (sign == Ordering::Equal)
        })
        .take(LARGE_LIMIT)
    {
        ordering_and_u32_slice(sign, &limbs);
    }

    for (sign, limbs) in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        ordering_and_u32_slice(sign, &limbs);
    }
}

#[test]
fn from_sign_and_limbs_be_properties() {
    // Integer::from_sign_and_limbs_be(sign, limbs) is equivalent for malachite-gmp and
    // malachite-native.
    // x := Integer::from_sign_and_limbs_be(sign, limbs); x.sign() == sign and
    //      x.limbs_le() == limbs.skip_while(|u| u == 0)
    // Integer::from_sign_and-limbs_be(sign, limbs.reverse()) ==
    //      Integer::from_sign_and_limbs_le(sign, limbs)
    let ordering_and_u32_slice = |sign: Ordering, limbs: &[u32]| {
        let x = native::Integer::from_sign_and_limbs_be(sign, limbs);
        assert_eq!(
            gmp_integer_to_native(&gmp::Integer::from_sign_and_limbs_be(sign, limbs)),
            x
        );
        let (sign_2, limbs_2) = x.sign_and_limbs_be();
        assert_eq!(sign_2, sign);
        assert_eq!(
            limbs_2,
            limbs
                .iter()
                .cloned()
                .skip_while(|&u| u == 0)
                .collect::<Vec<u32>>()
        );
        assert_eq!(
            native::Integer::from_sign_and_limbs_le(
                sign,
                &limbs.iter().cloned().rev().collect::<Vec<u32>>(),
            ),
            x
        );
    };

    for (sign, limbs) in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        ordering_and_u32_slice(sign, &limbs);
    }

    for (sign, limbs) in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        ordering_and_u32_slice(sign, &limbs);
    }
}
