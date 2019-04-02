use common::{test_properties, test_properties_no_special};
use malachite_base::num::{SignificantBits, Zero};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_test::inputs::base::small_unsigneds;
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_small_unsigned, pairs_of_natural_and_vec_of_bool_var_2,
};
use std::str::FromStr;

#[test]
fn test_to_bits_asc() {
    let test = |n, out| {
        let n = Natural::from_str(n).unwrap();
        assert_eq!(n.bits().collect::<Vec<bool>>(), out);
        assert_eq!(n.to_bits_asc(), out);
    };
    test("0", vec![]);
    test("1", vec![true]);
    test("6", vec![false, true, true]);
    test("105", vec![true, false, false, true, false, true, true]);
    test(
        "1000000000000",
        vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            true, false, false, false, true, false, true, false, false, true, false, true, false,
            false, true, false, true, false, true, true, false, false, false, true, false, true,
            true, true,
        ],
    );
    test(
        "4294967295",
        vec![
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true,
        ],
    );
    test(
        "4294967296",
        vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, true,
        ],
    );
    test(
        "18446744073709551615",
        vec![
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true,
        ],
    );
    test(
        "18446744073709551616",
        vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, true,
        ],
    );

    let n = Natural::from(105u32);
    let mut bits = n.bits();
    assert_eq!(Some(true), bits.next());
    assert_eq!(Some(true), bits.next_back());
    assert_eq!(Some(true), bits.next_back());
    assert_eq!(Some(false), bits.next_back());
    assert_eq!(Some(false), bits.next());
    assert_eq!(Some(false), bits.next());
    assert_eq!(Some(true), bits.next());
    assert_eq!(None, bits.next());
    assert_eq!(None, bits.next_back());

    assert_eq!(bits[0], true);
    assert_eq!(bits[1], false);
    assert_eq!(bits[2], false);
    assert_eq!(bits[3], true);
    assert_eq!(bits[4], false);
    assert_eq!(bits[5], true);
    assert_eq!(bits[6], true);
    assert_eq!(bits[7], false);
    assert_eq!(bits[8], false);

    let mut bits = n.bits();
    assert_eq!(Some(true), bits.next_back());
    assert_eq!(Some(true), bits.next());
    assert_eq!(Some(false), bits.next());
    assert_eq!(Some(false), bits.next());
    assert_eq!(Some(true), bits.next_back());
    assert_eq!(Some(false), bits.next_back());
    assert_eq!(Some(true), bits.next_back());
    assert_eq!(None, bits.next());
    assert_eq!(None, bits.next_back());
}

#[test]
fn test_to_bits_desc() {
    let test = |n, out| {
        let n = Natural::from_str(n).unwrap();
        assert_eq!(n.bits().rev().collect::<Vec<bool>>(), out);
        assert_eq!(n.to_bits_desc(), out);
    };
    test("0", vec![]);
    test("1", vec![true]);
    test("6", vec![true, true, false]);
    test("105", vec![true, true, false, true, false, false, true]);
    test(
        "1000000000000",
        vec![
            true, true, true, false, true, false, false, false, true, true, false, true, false,
            true, false, false, true, false, true, false, false, true, false, true, false, false,
            false, true, false, false, false, false, false, false, false, false, false, false,
            false, false,
        ],
    );
    test(
        "4294967295",
        vec![
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true,
        ],
    );
    test(
        "4294967296",
        vec![
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false,
        ],
    );
    test(
        "18446744073709551615",
        vec![
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true,
        ],
    );
    test(
        "18446744073709551616",
        vec![
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false,
        ],
    );
}

#[test]
fn to_limbs_asc_properties() {
    test_properties(naturals, |x| {
        let bits = x.to_bits_asc();
        assert_eq!(x.bits().collect::<Vec<bool>>(), bits);
        assert_eq!(Natural::from_bits_asc(&bits), *x);
        if *x != 0 as Limb {
            assert_ne!(*bits.last().unwrap(), false);
        }
    });
}

#[test]
fn to_bits_desc_properties() {
    test_properties(naturals, |x| {
        let bits = x.to_bits_desc();
        assert_eq!(x.bits().rev().collect::<Vec<bool>>(), bits);
        assert_eq!(Natural::from_bits_desc(&bits), *x);
        if *x != 0 as Limb {
            assert_ne!(bits[0], false);
        }
    });
}

#[test]
fn bits_properties() {
    test_properties(naturals, |n| {
        let significant_bits = n.significant_bits() as usize;
        assert_eq!(
            n.bits().size_hint(),
            (significant_bits, Some(significant_bits))
        );
    });

    test_properties(
        pairs_of_natural_and_vec_of_bool_var_2,
        |&(ref n, ref bs)| {
            let mut bits = n.bits();
            let mut bit_vec = Vec::new();
            let mut i = 0;
            for &b in bs {
                if b {
                    bit_vec.insert(i, bits.next().unwrap());
                    i += 1;
                } else {
                    bit_vec.insert(i, bits.next_back().unwrap())
                }
            }
            assert!(bits.next().is_none());
            assert!(bits.next_back().is_none());
            assert_eq!(n.to_bits_asc(), bit_vec);
        },
    );

    test_properties(pairs_of_natural_and_small_unsigned, |&(ref n, u)| {
        if u < n.significant_bits() {
            assert_eq!(n.bits()[u], n.to_bits_asc()[u as usize]);
        } else {
            assert_eq!(n.bits()[u], false);
        }
    });

    test_properties_no_special(small_unsigneds, |&u| {
        assert_eq!(Natural::ZERO.bits()[u], false);
    });
}
