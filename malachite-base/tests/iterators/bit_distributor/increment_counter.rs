// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::iterators::bit_distributor::{BitDistributor, BitDistributorOutputType};

#[test]
fn test_increment_counter() {
    let mut bd = BitDistributor::new(&[BitDistributorOutputType::normal(1)]);
    let mut outputs = Vec::new();
    for _ in 0..20 {
        outputs.push(bd.get_output(0));
        bd.increment_counter();
    }
    assert_eq!(
        outputs,
        &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]
    );
}
