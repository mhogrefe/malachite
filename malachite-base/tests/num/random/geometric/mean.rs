// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::float::NiceFloat;
use malachite_base::test_util::num::random::geometric::{
    adjusted_mean_to_unadjusted_mean, unadjusted_mean_to_adjusted_mean,
};

#[test]
pub fn test_unadjusted_mean_to_adjusted_mean() {
    let test = |unadjusted_mean, limit, adjusted_mean| {
        assert_eq!(
            NiceFloat(unadjusted_mean_to_adjusted_mean(unadjusted_mean, limit)),
            NiceFloat(adjusted_mean)
        );
    };
    test(1.0e-45, 255.0, 0.0);
    test(0.001, 255.0, 0.000999999999999855);
    test(1.0, 255.0, 1.0);
    test(10.0, 255.0, 9.999999993517946);
    test(50.0, 255.0, 48.38067332601502);
    test(100.0, 255.0, 78.25417704300385);
    test(255.0, 255.0, 106.4745261876895);
    test(1000000.0, 255.0, 127.49458799160993);

    test(1.0000000000000002, 255.0, 1.0);
    test(10.000000006482054, 255.0, 10.000000000000002);
    test(51.95952531604177, 255.0, 50.0);
    test(192.50163549359422, 255.0, 100.00000000000001);
    test(10921.900023504326, 255.0, 127.0000000000199);
    test(546163.2639224805, 255.0, 127.49000008135106);
}

#[test]
pub fn test_adjusted_mean_to_unadjusted_mean() {
    let test = |adjusted_mean, limit, unadjusted_mean| {
        assert_eq!(
            NiceFloat(adjusted_mean_to_unadjusted_mean(adjusted_mean, limit)),
            NiceFloat(unadjusted_mean)
        );
    };
    test(1.0, 255.0, 1.0000000000000002);
    test(10.0, 255.0, 10.000000006482054);
    test(50.0, 255.0, 51.95952531604177);
    test(100.0, 255.0, 192.50163549359422);
    test(127.0, 255.0, 10921.900023504326);
    test(127.49, 255.0, 546163.2639224805);

    test(0.000999999999999855, 255.0, 0.0009999999999998803);
    test(1.0, 255.0, 1.0000000000000002);
    test(9.999999993517946, 255.0, 10.0);
    test(48.38067332601502, 255.0, 50.0);
    test(78.25417704300385, 255.0, 100.0);
    test(106.4745261876895, 255.0, 255.0);
    test(127.49458799160993, 255.0, 1006078.1813340919);
}
