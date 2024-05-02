// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::iterators::bit_distributor::{BitDistributor, BitDistributorOutputType};

#[test]
fn test_bit_distributor() {
    BitDistributor::new(&[BitDistributorOutputType::normal(1)]);
}

#[test]
#[should_panic]
fn bit_distributor_fail_1() {
    BitDistributor::new(&[]);
}

#[test]
#[should_panic]
fn bit_distributor_fail_2() {
    BitDistributor::new(&[BitDistributorOutputType::tiny(); 2]);
}
