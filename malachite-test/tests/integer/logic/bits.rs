use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitConvertible, BitIterable, SignificantBits};
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;

use malachite_test::common::{test_properties, test_properties_no_special};
use malachite_test::inputs::base::{signeds, small_unsigneds};
use malachite_test::inputs::integer::{
    pairs_of_integer_and_small_unsigned, pairs_of_integer_and_vec_of_bool_var_2,
};

#[test]
fn test_bits() {
    let n = Integer::from(-105);
    let mut bits = n.bits();
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next_back(), Some(false));
    assert_eq!(bits.next_back(), Some(false));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), Some(false));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), None);
    assert_eq!(bits.next_back(), None);

    assert_eq!(bits[0], true);
    assert_eq!(bits[1], true);
    assert_eq!(bits[2], true);
    assert_eq!(bits[3], false);
    assert_eq!(bits[4], true);
    assert_eq!(bits[5], false);
    assert_eq!(bits[6], false);
    assert_eq!(bits[7], true);
    assert_eq!(bits[8], true);

    let mut bits = n.bits();
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next_back(), Some(false));
    assert_eq!(bits.next_back(), Some(false));
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next_back(), Some(false));
    assert_eq!(bits.next(), None);
    assert_eq!(bits.next_back(), None);
}

#[test]
fn bits_properties() {
    test_properties(
        pairs_of_integer_and_vec_of_bool_var_2,
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

    test_properties(pairs_of_integer_and_small_unsigned, |&(ref n, u)| {
        if u < n.significant_bits() {
            assert_eq!(n.bits()[u], n.to_bits_asc()[usize::exact_from(u)]);
        } else {
            assert_eq!(n.bits()[u], *n < 0);
        }
    });

    test_properties_no_special(small_unsigneds, |&u| {
        assert_eq!(Integer::ZERO.bits()[u], false);
    });

    test_properties(signeds::<SignedLimb>, |&i| {
        assert!(i.bits().eq(Integer::from(i).bits()));
    });
}
