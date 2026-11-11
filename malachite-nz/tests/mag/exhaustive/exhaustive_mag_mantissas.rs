// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_nz::mag::exhaustive::exhaustive_mag_mantissas;

#[test]
fn test_exhaustive_mag_mantissas() {
    assert_eq!(
        exhaustive_mag_mantissas()
            .take(20)
            .map(|x| x.to_string())
            .collect_vec(),
        &[
            "536870912",
            "805306368",
            "671088640",
            "939524096",
            "603979776",
            "738197504",
            "872415232",
            "1006632960",
            "570425344",
            "637534208",
            "704643072",
            "771751936",
            "838860800",
            "905969664",
            "973078528",
            "1040187392",
            "553648128",
            "587202560",
            "620756992",
            "654311424"
        ]
    );
}

#[test]
fn exhaustive_mag_mantissas_properties() {
    // - the first 2^k mantissas are exactly those whose odd parts have at most k + 1 bits
    for k in 0..12 {
        let xs = exhaustive_mag_mantissas().take(1 << k).collect_vec();
        assert!(xs.iter().all(|&x| x >> 29 == 1));
        assert!(xs.iter().all(|&x| (x >> x.trailing_zeros()) < 1 << (k + 1)));
        assert!(xs.iter().tuple_windows().all(|(x, y)| x != y));
    }
}
