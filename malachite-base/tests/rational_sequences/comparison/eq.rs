// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::rational_sequences::RationalSequence;
use malachite_base::test_util::generators::{
    unsigned_rational_sequence_gen, unsigned_rational_sequence_pair_gen,
    unsigned_rational_sequence_triple_gen,
};

#[test]
fn test_eq() {
    let xs = &[
        RationalSequence::from_vec(vec![]),
        RationalSequence::from_vec(vec![1, 2, 3]),
        RationalSequence::from_vecs(vec![], vec![1, 2, 3]),
        RationalSequence::from_vecs(vec![1, 2, 3], vec![4, 5, 6]),
    ];
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in xs.iter().enumerate() {
            assert_eq!(i == j, x.clone() == y.clone());
        }
    }
}

#[test]
fn eq_properties() {
    unsigned_rational_sequence_pair_gen::<u8>().test_properties(|(xs, ys)| {
        assert_eq!(xs == ys, ys == xs);
    });

    unsigned_rational_sequence_gen::<u8>().test_properties(|xs| {
        assert_eq!(xs, xs);
    });

    unsigned_rational_sequence_triple_gen::<u8>().test_properties(|(xs, ys, zs)| {
        if xs == ys && ys == zs {
            assert_eq!(xs, zs);
        }
    });
}
