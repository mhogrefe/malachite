#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_from_limbs_asc() {
    let test = |xs: &[Limb], out| {
        let x = Natural::from_limbs_asc(xs);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = Natural::from_owned_limbs_asc(xs.to_vec());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test(&[], "0");
    test(&[0], "0");
    test(&[0, 0, 0], "0");
    test(&[123], "123");
    test(&[123, 0], "123");
    test(&[123, 0, 0, 0], "123");
    test(&[3567587328, 232], "1000000000000");
    test(&[3567587328, 232, 0], "1000000000000");
    test(&[1, 2, 3, 4, 5], "1701411834921604967429270619762735448065");
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_from_limbs_desc() {
    let test = |xs: Vec<Limb>, out| {
        let x = Natural::from_limbs_desc(&xs);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = Natural::from_owned_limbs_desc(xs.to_vec());
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test(vec![], "0");
    test(vec![0], "0");
    test(vec![0, 0, 0], "0");
    test(vec![123], "123");
    test(vec![0, 123], "123");
    test(vec![0, 0, 0, 123], "123");
    test(vec![232, 3567587328], "1000000000000");
    test(vec![0, 232, 3567587328], "1000000000000");
    test(
        vec![5, 4, 3, 2, 1],
        "1701411834921604967429270619762735448065",
    );
}
