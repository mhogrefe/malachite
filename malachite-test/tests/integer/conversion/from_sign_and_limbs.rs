use std::cmp::Ordering;

use malachite_nz::integer::Integer;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::pairs_of_ordering_and_vec_of_unsigned_var_1;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_from_sign_and_limbs_asc() {
    let test = |sign: Ordering, limbs: &[Limb], out| {
        let x = Integer::from_sign_and_limbs_asc(sign, limbs);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = Integer::from_sign_and_owned_limbs_asc(sign, limbs.to_vec());
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
    test(Ordering::Greater, &[3_567_587_328, 232], "1000000000000");
    test(Ordering::Less, &[3_567_587_328, 232], "-1000000000000");
    test(Ordering::Greater, &[3_567_587_328, 232, 0], "1000000000000");
    test(Ordering::Less, &[3_567_587_328, 232, 0], "-1000000000000");
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

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn from_sign_and_limbs_asc_fail_1() {
    Integer::from_sign_and_limbs_asc(Ordering::Equal, &[1]);
}

#[test]
#[should_panic]
fn from_sign_and_limbs_asc_fail_2() {
    Integer::from_sign_and_limbs_asc(Ordering::Greater, &[]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn from_sign_and_limbs_asc_fail_3() {
    Integer::from_sign_and_limbs_asc(Ordering::Greater, &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn from_sign_and_owned_limbs_asc_fail_1() {
    Integer::from_sign_and_owned_limbs_asc(Ordering::Equal, vec![1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn from_sign_and_owned_limbs_asc_fail_2() {
    Integer::from_sign_and_owned_limbs_asc(Ordering::Greater, vec![]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn from_sign_and_owned_limbs_asc_fail_3() {
    Integer::from_sign_and_owned_limbs_asc(Ordering::Greater, vec![0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_from_sign_and_limbs_desc() {
    let test = |sign: Ordering, limbs: &[Limb], out| {
        let x = Integer::from_sign_and_limbs_desc(sign, limbs);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = Integer::from_sign_and_owned_limbs_desc(sign, limbs.to_vec());
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
    test(Ordering::Greater, &[232, 3_567_587_328], "1000000000000");
    test(Ordering::Less, &[232, 3_567_587_328], "-1000000000000");
    test(Ordering::Greater, &[0, 232, 3_567_587_328], "1000000000000");
    test(Ordering::Less, &[0, 232, 3_567_587_328], "-1000000000000");
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

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn from_sign_and_limbs_desc_fail_1() {
    Integer::from_sign_and_limbs_desc(Ordering::Equal, &[1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn from_sign_and_limbs_desc_fail_2() {
    Integer::from_sign_and_limbs_desc(Ordering::Greater, &[]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn from_sign_and_limbs_desc_fail_3() {
    Integer::from_sign_and_limbs_desc(Ordering::Greater, &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn from_sign_and_owned_limbs_desc_fail_1() {
    Integer::from_sign_and_owned_limbs_desc(Ordering::Equal, vec![1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn from_sign_and_owned_limbs_desc_fail_2() {
    Integer::from_sign_and_owned_limbs_desc(Ordering::Greater, vec![]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn from_sign_and_owned_limbs_desc_fail_3() {
    Integer::from_sign_and_owned_limbs_desc(Ordering::Greater, vec![0, 0, 0]);
}

#[test]
fn from_sign_and_limbs_asc_properties() {
    test_properties(
        pairs_of_ordering_and_vec_of_unsigned_var_1,
        |&(sign, ref limbs): &(Ordering, Vec<Limb>)| {
            let x = Integer::from_sign_and_limbs_asc(sign, limbs);
            assert!(x.is_valid());
            assert_eq!(
                Integer::from_sign_and_owned_limbs_asc(sign, limbs.clone()),
                x
            );
            let mut trimmed_limbs: Vec<Limb> = limbs
                .iter()
                .cloned()
                .rev()
                .skip_while(|&limb| limb == 0)
                .collect();
            trimmed_limbs.reverse();
            let (sign_be, limbs_desc) = x.to_sign_and_limbs_asc();
            assert_eq!(sign_be, sign);
            assert_eq!(limbs_desc, trimmed_limbs);
            assert_eq!(
                Integer::from_sign_and_limbs_desc(
                    sign,
                    &limbs.iter().cloned().rev().collect::<Vec<Limb>>(),
                ),
                x
            );
        },
    );
}

#[test]
fn from_sign_and_limbs_desc_properties() {
    test_properties(
        pairs_of_ordering_and_vec_of_unsigned_var_1,
        |&(sign, ref limbs): &(Ordering, Vec<Limb>)| {
            let x = Integer::from_sign_and_limbs_desc(sign, limbs);
            assert!(x.is_valid());
            assert_eq!(
                Integer::from_sign_and_owned_limbs_desc(sign, limbs.clone()),
                x
            );
            let (sign_le, limbs_asc) = x.to_sign_and_limbs_desc();
            assert_eq!(sign_le, sign);
            assert_eq!(
                limbs_asc,
                limbs
                    .iter()
                    .cloned()
                    .skip_while(|&limb| limb == 0)
                    .collect::<Vec<Limb>>()
            );
            assert_eq!(
                Integer::from_sign_and_limbs_asc(
                    sign,
                    &limbs.iter().cloned().rev().collect::<Vec<Limb>>(),
                ),
                x
            );
        },
    );
}
