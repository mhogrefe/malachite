// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::__std_iter::empty;
use malachite_base::iterators::iterator_cache::IteratorCache;
use malachite_base::nevers::Never;
use malachite_base::num::exhaustive::exhaustive_signeds;

#[test]
fn test_iterator_cache() {
    let mut xs = IteratorCache::new(empty::<Never>());
    assert_eq!(xs.known_len(), None);
    assert_eq!(xs.get(1), None);
    assert_eq!(xs.known_len(), Some(0));
    assert_eq!(xs.get(0), None);

    let mut xs = IteratorCache::new([1, 2, 3].iter().copied());
    assert_eq!(xs.known_len(), None);
    assert_eq!(xs.get(1), Some(&2));
    assert_eq!(xs.assert_get(1), &2);
    assert_eq!(xs.known_len(), None);
    assert_eq!(xs.get(0), Some(&1));
    assert_eq!(xs.assert_get(0), &1);
    assert_eq!(xs.get(3), None);
    assert_eq!(xs.known_len(), Some(3));
    assert_eq!(xs.get(2), Some(&3));
    assert_eq!(xs.assert_get(2), &3);

    let mut xs = IteratorCache::new(exhaustive_signeds::<i64>());
    assert_eq!(xs.get(1), Some(&1));
    assert_eq!(xs.assert_get(1), &1);
    assert_eq!(xs.known_len(), None);
    assert_eq!(xs.get(0), Some(&0));
    assert_eq!(xs.assert_get(0), &0);
    assert_eq!(xs.get(3), Some(&2));
    assert_eq!(xs.assert_get(3), &2);
    assert_eq!(xs.get(100), Some(&-50));
    assert_eq!(xs.assert_get(100), &-50);
    assert_eq!(xs.known_len(), None);
}

#[test]
#[should_panic]
fn iterator_cache_fail() {
    IteratorCache::new(empty::<Never>()).assert_get(0);
}
