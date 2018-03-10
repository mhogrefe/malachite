use common::test_properties;
use malachite_nz::natural::Natural;
use malachite_test::inputs::natural::naturals;
use std::str::FromStr;
use std::u32;

#[test]
fn test_to_limbs_asc() {
    let test = |n, out| {
        let n = Natural::from_str(n).unwrap();
        assert_eq!(n.limbs().collect::<Vec<u32>>(), out);
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

    let mut limbs = n.limbs();
    assert_eq!(Some(1), limbs.next());
    assert_eq!(Some(2), limbs.next());
    assert_eq!(Some(3), limbs.next());
    assert_eq!(Some(5), limbs.next_back());
    assert_eq!(Some(4), limbs.next_back());
    assert_eq!(None, limbs.next());
    assert_eq!(None, limbs.next_back());
}

#[test]
fn test_to_limbs_desc() {
    let test = |n, out| {
        let n = Natural::from_str(n).unwrap();
        assert_eq!(n.limbs().rev().collect::<Vec<u32>>(), out);
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
    test("4294967295", vec![u32::MAX]);
    test("4294967296", vec![1, 0]);
    test("18446744073709551615", vec![u32::MAX, u32::MAX]);
    test("18446744073709551616", vec![1, 0, 0]);
}

#[test]
fn to_limbs_asc_properties() {
    test_properties(naturals, |x| {
        let limbs = x.to_limbs_asc();
        assert_eq!(x.clone().into_limbs_asc(), limbs);
        assert_eq!(x.limbs().collect::<Vec<u32>>(), limbs);
        assert_eq!(Natural::from_limbs_asc(&limbs), *x);
        assert_eq!(
            x.to_limbs_desc(),
            limbs.iter().cloned().rev().collect::<Vec<u32>>()
        );
        if *x != 0 {
            assert_ne!(*limbs.last().unwrap(), 0);
        }
    });
}

#[test]
fn to_limbs_desc_properties() {
    test_properties(naturals, |x| {
        let limbs = x.to_limbs_desc();
        assert_eq!(x.clone().into_limbs_desc(), limbs);
        assert_eq!(x.limbs().rev().collect::<Vec<u32>>(), limbs);
        assert_eq!(Natural::from_limbs_desc(&limbs), *x);
        assert_eq!(
            x.to_limbs_asc(),
            limbs.iter().cloned().rev().collect::<Vec<u32>>()
        );
        if *x != 0 {
            assert_ne!(limbs[0], 0);
        }
    });
}
