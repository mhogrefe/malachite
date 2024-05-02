// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::nevers::Never;
use malachite_base::tuples::singletons;
use std::fmt::Debug;

fn singletons_helper<T: Clone + Debug + Eq>(xs: &[T], out: &[(T,)]) {
    assert_eq!(singletons(xs.iter().cloned()).collect_vec().as_slice(), out);
}

#[test]
fn test_singletons() {
    singletons_helper::<Never>(&[], &[]);
    singletons_helper(&[5], &[(5,)]);
    singletons_helper(&[1, 2, 3], &[(1,), (2,), (3,)]);
    singletons_helper(&[(2,), (1,), (5,)], &[((2,),), ((1,),), ((5,),)]);
}
