use common::{test_properties, test_properties_no_special};
#[cfg(feature = "32_bit_limbs")]
use malachite_base::misc::Max;
use malachite_base::num::Zero;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_test::inputs::base::small_usizes;
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_small_usize, pairs_of_natural_and_vec_of_bool_var_1,
};
#[cfg(feature = "32_bit_limbs")]
use std::str::FromStr;

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
    test("4294967295", vec![Limb::MAX]);
    test("4294967296", vec![0, 1]);
    test("18446744073709551615", vec![Limb::MAX, Limb::MAX]);
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

#[test]
fn to_limbs_asc_properties() {
    test_properties(naturals, |x| {
        let limbs = x.to_limbs_asc();
        assert_eq!(x.clone().into_limbs_asc(), limbs);
        assert_eq!(x.limbs().collect::<Vec<Limb>>(), limbs);
        assert_eq!(Natural::from_limbs_asc(&limbs), *x);
        if *x != 0 as Limb {
            assert_ne!(*limbs.last().unwrap(), 0);
        }
    });
}

#[test]
fn to_limbs_desc_properties() {
    test_properties(naturals, |x| {
        let limbs = x.to_limbs_desc();
        assert_eq!(x.clone().into_limbs_desc(), limbs);
        assert_eq!(x.limbs().rev().collect::<Vec<Limb>>(), limbs);
        assert_eq!(Natural::from_limbs_desc(&limbs), *x);
        if *x != 0 as Limb {
            assert_ne!(limbs[0], 0);
        }
    });
}

#[test]
fn limbs_properties() {
    test_properties(naturals, |n| {
        let limb_count = n.limb_count() as usize;
        assert_eq!(n.limbs().size_hint(), (limb_count, Some(limb_count)));
    });

    test_properties(
        pairs_of_natural_and_vec_of_bool_var_1,
        |&(ref n, ref bs)| {
            let mut limbs = n.limbs();
            let mut limb_vec = Vec::new();
            let mut i = 0;
            for &b in bs {
                if b {
                    limb_vec.insert(i, limbs.next().unwrap());
                    i += 1;
                } else {
                    limb_vec.insert(i, limbs.next_back().unwrap())
                }
            }
            assert!(limbs.next().is_none());
            assert!(limbs.next_back().is_none());
            assert_eq!(n.to_limbs_asc(), limb_vec);
        },
    );

    test_properties(pairs_of_natural_and_small_usize, |&(ref n, u)| {
        if u < n.limb_count() as usize {
            assert_eq!(n.limbs()[u], n.to_limbs_asc()[u]);
        } else {
            assert_eq!(n.limbs()[u], 0);
        }
    });

    test_properties_no_special(small_usizes, |&u| {
        assert_eq!(Natural::ZERO.limbs()[u], 0);
    });
}
