#[cfg(feature = "32_bit_limbs")]
use malachite_nz::integer::Integer;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_from_twos_complement_limbs_asc() {
    let test = |xs: &[Limb], out| {
        let x = Integer::from_twos_complement_limbs_asc(xs);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
        let x = Integer::from_owned_twos_complement_limbs_asc(xs.to_vec());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test(&[], "0");
    test(&[0], "0");
    test(&[0, 0, 0], "0");
    test(&[123], "123");
    test(&[123, 0], "123");
    test(&[4294967173], "-123");
    test(&[4294967173, u32::MAX], "-123");
    test(&[3567587328, 232], "1000000000000");
    test(&[727379968, 4294967063], "-1000000000000");
    test(&[1, 2, 3, 4, 5], "1701411834921604967429270619762735448065");
    test(
        &[
            u32::MAX,
            u32::MAX - 2,
            u32::MAX - 3,
            u32::MAX - 4,
            u32::MAX - 5,
        ],
        "-1701411834921604967429270619762735448065",
    );
    test(&[u32::MAX, 0], "4294967295");
    test(&[1, u32::MAX], "-4294967295");
    test(&[0, 1], "4294967296");
    test(&[0, u32::MAX], "-4294967296");
    test(&[u32::MAX, u32::MAX, 0], "18446744073709551615");
    test(&[1, 0, u32::MAX], "-18446744073709551615");
    test(&[0, 0, 1], "18446744073709551616");
    test(&[0, 0, u32::MAX], "-18446744073709551616");
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_from_twos_complement_limbs_desc() {
    let test = |xs: &[Limb], out| {
        let x = Integer::from_twos_complement_limbs_desc(xs);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
        let x = Integer::from_owned_twos_complement_limbs_desc(xs.to_vec());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test(&[], "0");
    test(&[0], "0");
    test(&[0, 0, 0], "0");
    test(&[123], "123");
    test(&[0, 123], "123");
    test(&[4294967173], "-123");
    test(&[u32::MAX, 4294967173], "-123");
    test(&[232, 3567587328], "1000000000000");
    test(&[4294967063, 727379968], "-1000000000000");
    test(&[5, 4, 3, 2, 1], "1701411834921604967429270619762735448065");
    test(
        &[
            u32::MAX - 5,
            u32::MAX - 4,
            u32::MAX - 3,
            u32::MAX - 2,
            u32::MAX,
        ],
        "-1701411834921604967429270619762735448065",
    );
    test(&[0, u32::MAX], "4294967295");
    test(&[u32::MAX, 1], "-4294967295");
    test(&[1, 0], "4294967296");
    test(&[u32::MAX, 0], "-4294967296");
    test(&[0, u32::MAX, u32::MAX], "18446744073709551615");
    test(&[u32::MAX, 0, 1], "-18446744073709551615");
    test(&[1, 0, 0], "18446744073709551616");
    test(&[u32::MAX, 0, 0], "-18446744073709551616");
}
