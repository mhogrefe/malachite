#[cfg(feature = "32_bit_limbs")]
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_to_limbs_asc() {
    let test = |n, out| {
        let n = Natural::from_str(n).unwrap();
        assert_eq!(n.limbs().collect::<Vec<Limb>>(), out);
        assert_eq!(n.to_limbs_asc(), out);
        assert_eq!(n.into_limbs_asc(), out);
    };
    test("0", vec![]);
    test("123", vec![123]);
    test("1000000000000", vec![3_567_587_328, 232]);
    test(
        "1701411834921604967429270619762735448065",
        vec![1, 2, 3, 4, 5],
    );
    test("4294967295", vec![u32::MAX]);
    test("4294967296", vec![0, 1]);
    test("18446744073709551615", vec![u32::MAX, u32::MAX]);
    test("18446744073709551616", vec![0, 0, 1]);

    let n = Natural::from_str("1701411834921604967429270619762735448065").unwrap();
    let mut limbs = n.limbs();
    assert_eq!(Some(1), limbs.next());
    assert_eq!(Some(5), limbs.next_back());
    assert_eq!(Some(4), limbs.next_back());
    assert_eq!(Some(2), limbs.next());
    assert_eq!(Some(3), limbs.next());
    assert_eq!(None, limbs.next());
    assert_eq!(None, limbs.next_back());

    assert_eq!(limbs[0], 1);
    assert_eq!(limbs[1], 2);
    assert_eq!(limbs[2], 3);
    assert_eq!(limbs[3], 4);
    assert_eq!(limbs[4], 5);
    assert_eq!(limbs[5], 0);

    let mut limbs = n.limbs();
    assert_eq!(Some(1), limbs.next());
    assert_eq!(Some(2), limbs.next());
    assert_eq!(Some(3), limbs.next());
    assert_eq!(Some(5), limbs.next_back());
    assert_eq!(Some(4), limbs.next_back());
    assert_eq!(None, limbs.next());
    assert_eq!(None, limbs.next_back());
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_to_limbs_desc() {
    let test = |n, out| {
        let n = Natural::from_str(n).unwrap();
        assert_eq!(n.limbs().rev().collect::<Vec<Limb>>(), out);
        assert_eq!(n.to_limbs_desc(), out);
        assert_eq!(n.into_limbs_desc(), out);
    };
    test("0", vec![]);
    test("123", vec![123]);
    test("1000000000000", vec![232, 3_567_587_328]);
    test(
        "1701411834921604967429270619762735448065",
        vec![5, 4, 3, 2, 1],
    );
    test("4294967295", vec![Limb::MAX]);
    test("4294967296", vec![1, 0]);
    test("18446744073709551615", vec![Limb::MAX, Limb::MAX]);
    test("18446744073709551616", vec![1, 0, 0]);
}
