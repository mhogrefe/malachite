// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitConvertible, BitIterable, SignificantBits};
use malachite_base::test_util::common::test_double_ended_iterator_size_hint;
use malachite_base::test_util::generators::{unsigned_gen, unsigned_gen_var_5};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_bool_vec_pair_gen_var_2, natural_gen, natural_unsigned_pair_gen_var_4,
};

#[test]
fn test_bits() {
    let n = Natural::from(105u32);
    let mut bits = n.bits();
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next_back(), Some(false));
    assert_eq!(bits.next(), Some(false));
    assert_eq!(bits.next(), Some(false));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), None);
    assert_eq!(bits.next_back(), None);

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
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next(), Some(true));
    assert_eq!(bits.next(), Some(false));
    assert_eq!(bits.next(), Some(false));
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next_back(), Some(false));
    assert_eq!(bits.next_back(), Some(true));
    assert_eq!(bits.next(), None);
    assert_eq!(bits.next_back(), None);
}

#[test]
fn bits_properties() {
    natural_gen().test_properties(|n| {
        test_double_ended_iterator_size_hint(n.bits(), usize::exact_from(n.significant_bits()));
    });

    natural_bool_vec_pair_gen_var_2().test_properties(|(n, bs)| {
        let mut bits = n.bits();
        let mut bit_vec = Vec::new();
        let mut i = 0;
        for &b in &bs {
            if b {
                bit_vec.insert(i, bits.next().unwrap());
                i += 1;
            } else {
                bit_vec.insert(i, bits.next_back().unwrap());
            }
        }
        assert!(bits.next().is_none());
        assert!(bits.next_back().is_none());
        assert_eq!(n.to_bits_asc(), bit_vec);
    });

    natural_unsigned_pair_gen_var_4().test_properties(|(n, u)| {
        if u < n.significant_bits() {
            assert_eq!(n.bits()[u], n.to_bits_asc()[usize::exact_from(u)]);
        } else {
            assert_eq!(n.bits()[u], false);
        }
    });

    unsigned_gen_var_5().test_properties(|u| {
        assert_eq!(Natural::ZERO.bits()[u], false);
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        assert!(u.bits().eq(Natural::from(u).bits()));
    });
}
