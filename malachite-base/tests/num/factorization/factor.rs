// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::itertools::Itertools;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::exhaustive::exhaustive_positive_primitive_ints;
use malachite_base::num::factorization::traits::{Factor, IsPrime};
use malachite_base::test_util::generators::unsigned_gen_var_1;
use malachite_base::test_util::num::factorization::factor::factor_naive;
use std::panic::catch_unwind;

fn factor_helper<T: Factor + PrimitiveUnsigned>()
where
    <T as Factor>::FACTORS: IntoIterator<Item = (T, u8)>,
{
    let test = |n: u64, out: &[(u64, u8)]| {
        if let Ok(n) = T::try_from(n) {
            let factors = n
                .factor()
                .into_iter()
                .map(|(f, e)| {
                    let f: u64 = f.wrapping_into();
                    (f, e)
                })
                .collect_vec();
            assert_eq!(factors, out);

            let factors_alt = factor_naive(n)
                .into_iter()
                .map(|(f, e)| {
                    let f: u64 = f.wrapping_into();
                    (f, e)
                })
                .collect_vec();
            assert_eq!(factors_alt, out);
        }
    };
    // - in n_factor_trial_range_u32
    // - p * p > n in n_factor_trial_range_u32
    // - cofactor == 1 in u32::factor
    // - in n_factor_trial_range_u64
    // - p * p > n in n_factor_trial_range_u64
    // - cofactor == 1 in u64::factor
    test(1, &[]);
    // - cofactor != 1 in u32::factor
    // - cofactor.is_prime() in u32::factor
    // - cofactor != 1 in u64::factor
    // - cofactor.is_prime() in u64::factor
    test(2, &[(2, 1)]);
    test(3, &[(3, 1)]);
    // - p * p <= n in n_factor_trial_range_u32
    // - in n_remove2_precomp_u32
    // - p == 2 in n_remove2_precomp_u32
    // - exp != 0 in n_remove2_precomp_u32
    // - exp != 0 in n_factor_trial_range_u32
    // - p * p <= n in n_factor_trial_range_u64
    // - in n_remove2_precomp_u64
    // - p == 2 in n_remove2_precomp_u64
    // - exp != 0 in n_remove2_precomp_u64
    // - exp != 0 in n_factor_trial_range_u64
    test(4, &[(2, 2)]);
    // - exp == 0 in n_remove2_precomp_u32
    // - exp == 0 in n_factor_trial_range_u32
    // - exp == 0 in n_remove2_precomp_u64
    // - exp == 0 in n_factor_trial_range_u64
    test(5, &[(5, 1)]);
    test(6, &[(2, 1), (3, 1)]);
    test(7, &[(7, 1)]);
    test(8, &[(2, 3)]);
    // - p != 2 in n_remove2_precomp_u32
    // - in n_divrem2_precomp_u32
    // - a >= n in n_divrem2_precomp_u32
    // - !n.get_highest_bit() in n_divrem2_precomp_u32
    // - n != 1 in n_divrem2_precomp_u32
    // - 0 <= r < n first time in n_divrem2_precomp_u32
    // - r == 0 in n_remove2_precomp_u32
    // - p != 2 in n_remove2_precomp_u64
    // - in n_divrem2_precomp_u64
    // - a >= n in n_divrem2_precomp_u64
    // - !n.get_highest_bit() in n_divrem2_precomp_u64
    // - n != 1 in n_divrem2_precomp_u64
    // - 0 <= r < n first time in n_divrem2_precomp_u64
    // - r == 0 in n_remove2_precomp_u64
    test(9, &[(3, 2)]);
    test(10, &[(2, 1), (5, 1)]);

    // - r != 0 in n_remove2_precomp_u32
    // - r != 0 in n_remove2_precomp_u64
    test(u64::power_of_2(4) - 1, &[(3, 1), (5, 1)]);
    test(u64::power_of_2(5) - 1, &[(31, 1)]);
    test(u64::power_of_2(6) - 1, &[(3, 2), (7, 1)]);
    test(u64::power_of_2(7) - 1, &[(127, 1)]);
    test(u64::power_of_2(8) - 1, &[(3, 1), (5, 1), (17, 1)]);
    test(u64::power_of_2(9) - 1, &[(7, 1), (73, 1)]);
    test(u64::power_of_2(10) - 1, &[(3, 1), (11, 1), (31, 1)]);
    test(u64::power_of_2(11) - 1, &[(23, 1), (89, 1)]);
    test(u64::power_of_2(12) - 1, &[(3, 2), (5, 1), (7, 1), (13, 1)]);
    test(u64::power_of_2(13) - 1, &[(8191, 1)]);
    test(u64::power_of_2(14) - 1, &[(3, 1), (43, 1), (127, 1)]);
    test(u64::power_of_2(15) - 1, &[(7, 1), (31, 1), (151, 1)]);
    test(
        u64::power_of_2(16) - 1,
        &[(3, 1), (5, 1), (17, 1), (257, 1)],
    );
    test(u64::power_of_2(17) - 1, &[(131071, 1)]);
    test(u64::power_of_2(18) - 1, &[(3, 3), (7, 1), (19, 1), (73, 1)]);
    test(u64::power_of_2(19) - 1, &[(524287, 1)]);
    test(
        u64::power_of_2(20) - 1,
        &[(3, 1), (5, 2), (11, 1), (31, 1), (41, 1)],
    );
    test(u64::power_of_2(21) - 1, &[(7, 2), (127, 1), (337, 1)]);
    test(
        u64::power_of_2(22) - 1,
        &[(3, 1), (23, 1), (89, 1), (683, 1)],
    );
    test(u64::power_of_2(23) - 1, &[(47, 1), (178481, 1)]);
    test(
        u64::power_of_2(24) - 1,
        &[(3, 2), (5, 1), (7, 1), (13, 1), (17, 1), (241, 1)],
    );
    test(u64::power_of_2(25) - 1, &[(31, 1), (601, 1), (1801, 1)]);
    // - r >= n first time in n_divrem2_precomp_u32
    // - r >= n second time in n_divrem2_precomp_u32
    // - r >= n first time in n_divrem2_precomp_u64
    // - r >= n second time in n_divrem2_precomp_u64
    test(u64::power_of_2(26) - 1, &[(3, 1), (2731, 1), (8191, 1)]);
    test(u64::power_of_2(27) - 1, &[(7, 1), (73, 1), (262657, 1)]);
    test(
        u64::power_of_2(28) - 1,
        &[(3, 1), (5, 1), (29, 1), (43, 1), (113, 1), (127, 1)],
    );
    test(u64::power_of_2(29) - 1, &[(233, 1), (1103, 1), (2089, 1)]);
    test(
        u64::power_of_2(30) - 1,
        &[(3, 2), (7, 1), (11, 1), (31, 1), (151, 1), (331, 1)],
    );
    test(u64::power_of_2(31) - 1, &[(2147483647, 1)]);
    test(
        u64::power_of_2(32) - 1,
        &[(3, 1), (5, 1), (17, 1), (257, 1), (65537, 1)],
    );
    test(
        u64::power_of_2(33) - 1,
        &[(7, 1), (23, 1), (89, 1), (599479, 1)],
    );
    // - !cofactor.is_prime() in u64::factor
    // - factor >= cutoff in u64::factor
    // - in n_factor_power235_u64
    // - t != 0 first time in n_factor_power235_u64
    // - t != 0 second time in n_factor_power235_u64
    // - t == 0 third time in n_factor_power235_u64
    // - cofactor == 0 in u64::factor
    // - factor >= cutoff && !factor.is_prime() in u64::factor
    // - in n_factor_one_line_u64
    // - in n_is_square_u64
    // - IS_SQUARE_MOD64[(x % 64) as usize] != 0 in n_is_square_u64
    // - IS_SQUARE_MOD63[(x % 63) as usize] == 0 in n_is_square_u64
    // - !n_is_square_u64(mmod) in n_factor_one_line_u64
    // - IS_SQUARE_MOD63[(x % 63) as usize] != 0 in n_is_square_u64
    // - IS_SQUARE_MOD65[(x % 65) as usize] == 0 in n_is_square_u64
    // - IS_SQUARE_MOD64[(x % 64) as usize] == 0 in n_is_square_u64
    // - IS_SQUARE_MOD65[(x % 65) as usize] != 0 in n_is_square_u64
    // - n_is_square_u64(mmod) in n_factor_one_line_u64
    // - found factor in u64::factor
    // - factor < cutoff in u64::factor
    test(u64::power_of_2(34) - 1, &[(3, 1), (43691, 1), (131071, 1)]);
    test(
        u64::power_of_2(35) - 1,
        &[(31, 1), (71, 1), (127, 1), (122921, 1)],
    );
    test(
        u64::power_of_2(36) - 1,
        &[(3, 3), (5, 1), (7, 1), (13, 1), (19, 1), (37, 1), (73, 1), (109, 1)],
    );
    test(u64::power_of_2(37) - 1, &[(223, 1), (616318177, 1)]);
    test(u64::power_of_2(38) - 1, &[(3, 1), (174763, 1), (524287, 1)]);
    test(
        u64::power_of_2(39) - 1,
        &[(7, 1), (79, 1), (8191, 1), (121369, 1)],
    );
    test(
        u64::power_of_2(40) - 1,
        &[(3, 1), (5, 2), (11, 1), (17, 1), (31, 1), (41, 1), (61681, 1)],
    );
    test(u64::power_of_2(41) - 1, &[(13367, 1), (164511353, 1)]);
    test(
        u64::power_of_2(42) - 1,
        &[(3, 2), (7, 2), (43, 1), (127, 1), (337, 1), (5419, 1)],
    );
    test(
        u64::power_of_2(43) - 1,
        &[(431, 1), (9719, 1), (2099863, 1)],
    );
    test(
        u64::power_of_2(44) - 1,
        &[(3, 1), (5, 1), (23, 1), (89, 1), (397, 1), (683, 1), (2113, 1)],
    );
    test(
        u64::power_of_2(45) - 1,
        &[(7, 1), (31, 1), (73, 1), (151, 1), (631, 1), (23311, 1)],
    );
    // - t == 0 first time in n_factor_power235_u64
    test(
        u64::power_of_2(46) - 1,
        &[(3, 1), (47, 1), (178481, 1), (2796203, 1)],
    );
    test(
        u64::power_of_2(47) - 1,
        &[(2351, 1), (4513, 1), (13264529, 1)],
    );
    test(
        u64::power_of_2(48) - 1,
        &[(3, 2), (5, 1), (7, 1), (13, 1), (17, 1), (97, 1), (241, 1), (257, 1), (673, 1)],
    );
    test(u64::power_of_2(49) - 1, &[(127, 1), (4432676798593, 1)]);
    test(
        u64::power_of_2(50) - 1,
        &[(3, 1), (11, 1), (31, 1), (251, 1), (601, 1), (1801, 1), (4051, 1)],
    );
    test(
        u64::power_of_2(51) - 1,
        &[(7, 1), (103, 1), (2143, 1), (11119, 1), (131071, 1)],
    );
    test(
        u64::power_of_2(52) - 1,
        &[(3, 1), (5, 1), (53, 1), (157, 1), (1613, 1), (2731, 1), (8191, 1)],
    );
    // - in n_factor_pp1_wrapper_u64
    // - bits >= 31 in n_factor_pp1_wrapper_u64
    // - in n_factor_squfof_u64
    // - in ll_factor_squfof_u64
    // - n_hi == 0 in ll_factor_squfof_u64
    // - q != 0 && num != 0 in ll_factor_squfof_u64
    // - q > l in ll_factor_squfof_u64
    // - i.even() in ll_factor_squfof_u64
    // - !n_is_square_u64(q) in ll_factor_squfof_u64
    // - i.odd() in ll_factor_squfof_u64
    // - q <= l in ll_factor_squfof_u64
    // - q.odd() && q > l2 in ll_factor_squfof_u64
    // - q.odd() && q <= l2 in ll_factor_squfof_u64
    // - q.odd() && q <= l2 && qupto < 50 in ll_factor_squfof_u64
    // - q.even() first time in ll_factor_squfof_u64
    // - q.even() && qupto < 50 in ll_factor_squfof_u64
    // - n_is_square_u64(q) in ll_factor_squfof_u64
    // - qupto != 0 in ll_factor_squfof_u64
    // - r != qarr[j] in ll_factor_squfof_u64
    // - done in ll_factor_squfof_u64
    // - !finished_loop in ll_factor_squfof_u64 first time
    // - sqrt_hi == 0 in ll_factor_squfof_u64
    // - p != pnext in ll_factor_squfof_u64
    // - p == pnext in ll_factor_squfof_u64
    // - !finished_loop in ll_factor_squfof_u64 second time
    // - q.even() second time in ll_factor_squfof_u64
    // - factor != 0 first time in n_factor_squfof_u64
    // - !finished_loop in n_factor_squfof_u64
    test(
        u64::power_of_2(53) - 1,
        &[(6361, 1), (69431, 1), (20394401, 1)],
    );
    // - t == 0 second time in n_factor_power235_u64
    test(
        u64::power_of_2(54) - 1,
        &[(3, 4), (7, 1), (19, 1), (73, 1), (87211, 1), (262657, 1)],
    );
    // - -n <= r < 0 in n_divrem2_precomp_u64
    test(
        u64::power_of_2(55) - 1,
        &[(23, 1), (31, 1), (89, 1), (881, 1), (3191, 1), (201961, 1)],
    );
    // - 0 <= r < n second time in n_divrem2_precomp_u64
    test(
        u64::power_of_2(56) - 1,
        &[(3, 1), (5, 1), (17, 1), (29, 1), (43, 1), (113, 1), (127, 1), (15790321, 1)],
    );
    // - r < -n in n_divrem2_precomp_u64
    // - r < 0 in n_divrem2_precomp_u64
    // - t != 0 third time in n_factor_power235_u64
    // - t.even() in n_factor_power235_u64
    // - t & 2 == 0 in n_factor_power235_u64
    // - t & 4 == 0 in n_factor_power235_u64
    // - in n_factor_pp1_u64
    // - n.odd() in n_factor_pp1_u64
    // - pr < sqrt in n_factor_pp1_u64 first time
    // - in n_pp1_pow_ui_u64
    // - exp & bit == 0 in n_pp1_pow_ui_u64
    // - exp & bit != 0 in n_pp1_pow_ui_u64
    // - pr >= sqrt in n_factor_pp1_u64 first time
    // - in n_pp1_factor_u64
    // - norm != 0 in n_pp1_factor_u64
    // - x != 0 in n_pp1_factor_u64
    // - factor != 0 first time in n_factor_pp1_u64
    // - factor != 1 first time in n_factor_pp1_u64
    // - factor != 0 in n_factor_pp1_wrapper_u64
    test(
        u64::power_of_2(57) - 1,
        &[(7, 1), (32377, 1), (524287, 1), (1212847, 1)],
    );
    test(
        u64::power_of_2(58) - 1,
        &[(3, 1), (59, 1), (233, 1), (1103, 1), (2089, 1), (3033169, 1)],
    );
    // - factor < cutoff || factor.is_prime() in u64::factor
    test(u64::power_of_2(59) - 1, &[(179951, 1), (3203431780337, 1)]);
    test(
        u64::power_of_2(60) - 1,
        &[
            (3, 2),
            (5, 2),
            (7, 1),
            (11, 1),
            (13, 1),
            (31, 1),
            (41, 1),
            (61, 1),
            (151, 1),
            (331, 1),
            (1321, 1),
        ],
    );
    test(u64::power_of_2(61) - 1, &[(2305843009213693951, 1)]);
    // - t.odd() in n_factor_power235_u64
    // - n != y.square() in n_factor_power235_u64
    // - t & 4 != 0 in n_factor_power235_u64
    // - n != y.pow(5) in n_factor_power235_u64
    // - t & 2 != 0 in n_factor_power235_u64
    // - n != y.pow(3) in n_factor_power235_u64
    test(
        u64::power_of_2(62) - 1,
        &[(3, 1), (715827883, 1), (2147483647, 1)],
    );
    test(
        u64::power_of_2(63) - 1,
        &[(7, 2), (73, 1), (127, 1), (337, 1), (92737, 1), (649657, 1)],
    );
    test(
        u64::MAX,
        &[(3, 1), (5, 1), (17, 1), (257, 1), (641, 1), (65537, 1), (6700417, 1)],
    );

    test(u64::power_of_2(4) + 1, &[(17, 1)]);
    test(u64::power_of_2(5) + 1, &[(3, 1), (11, 1)]);
    test(u64::power_of_2(6) + 1, &[(5, 1), (13, 1)]);
    test(u64::power_of_2(7) + 1, &[(3, 1), (43, 1)]);
    test(u64::power_of_2(8) + 1, &[(257, 1)]);
    test(u64::power_of_2(9) + 1, &[(3, 3), (19, 1)]);
    test(u64::power_of_2(10) + 1, &[(5, 2), (41, 1)]);
    test(u64::power_of_2(11) + 1, &[(3, 1), (683, 1)]);
    test(u64::power_of_2(12) + 1, &[(17, 1), (241, 1)]);
    test(u64::power_of_2(13) + 1, &[(3, 1), (2731, 1)]);
    test(u64::power_of_2(14) + 1, &[(5, 1), (29, 1), (113, 1)]);
    test(u64::power_of_2(15) + 1, &[(3, 2), (11, 1), (331, 1)]);
    test(u64::power_of_2(16) + 1, &[(65537, 1)]);
    test(u64::power_of_2(17) + 1, &[(3, 1), (43691, 1)]);
    test(
        u64::power_of_2(18) + 1,
        &[(5, 1), (13, 1), (37, 1), (109, 1)],
    );
    test(u64::power_of_2(19) + 1, &[(3, 1), (174763, 1)]);
    test(u64::power_of_2(20) + 1, &[(17, 1), (61681, 1)]);
    test(u64::power_of_2(21) + 1, &[(3, 2), (43, 1), (5419, 1)]);
    test(u64::power_of_2(22) + 1, &[(5, 1), (397, 1), (2113, 1)]);
    test(u64::power_of_2(23) + 1, &[(3, 1), (2796203, 1)]);
    test(u64::power_of_2(24) + 1, &[(97, 1), (257, 1), (673, 1)]);
    test(
        u64::power_of_2(25) + 1,
        &[(3, 1), (11, 1), (251, 1), (4051, 1)],
    );
    test(
        u64::power_of_2(26) + 1,
        &[(5, 1), (53, 1), (157, 1), (1613, 1)],
    );
    test(u64::power_of_2(27) + 1, &[(3, 4), (19, 1), (87211, 1)]);
    test(u64::power_of_2(28) + 1, &[(17, 1), (15790321, 1)]);
    test(u64::power_of_2(29) + 1, &[(3, 1), (59, 1), (3033169, 1)]);
    test(
        u64::power_of_2(30) + 1,
        &[(5, 2), (13, 1), (41, 1), (61, 1), (1321, 1)],
    );
    test(u64::power_of_2(31) + 1, &[(3, 1), (715827883, 1)]);
    test(u64::power_of_2(32) + 1, &[(641, 1), (6700417, 1)]);
    test(
        u64::power_of_2(33) + 1,
        &[(3, 2), (67, 1), (683, 1), (20857, 1)],
    );
    test(
        u64::power_of_2(34) + 1,
        &[(5, 1), (137, 1), (953, 1), (26317, 1)],
    );
    test(
        u64::power_of_2(35) + 1,
        &[(3, 1), (11, 1), (43, 1), (281, 1), (86171, 1)],
    );
    test(
        u64::power_of_2(36) + 1,
        &[(17, 1), (241, 1), (433, 1), (38737, 1)],
    );
    test(u64::power_of_2(37) + 1, &[(3, 1), (1777, 1), (25781083, 1)]);
    test(
        u64::power_of_2(38) + 1,
        &[(5, 1), (229, 1), (457, 1), (525313, 1)],
    );
    test(u64::power_of_2(39) + 1, &[(3, 2), (2731, 1), (22366891, 1)]);
    test(u64::power_of_2(40) + 1, &[(257, 1), (4278255361, 1)]);
    test(u64::power_of_2(41) + 1, &[(3, 1), (83, 1), (8831418697, 1)]);
    test(
        u64::power_of_2(42) + 1,
        &[(5, 1), (13, 1), (29, 1), (113, 1), (1429, 1), (14449, 1)],
    );
    test(u64::power_of_2(43) + 1, &[(3, 1), (2932031007403, 1)]);
    test(
        u64::power_of_2(44) + 1,
        &[(17, 1), (353, 1), (2931542417, 1)],
    );
    test(
        u64::power_of_2(45) + 1,
        &[(3, 3), (11, 1), (19, 1), (331, 1), (18837001, 1)],
    );
    test(
        u64::power_of_2(46) + 1,
        &[(5, 1), (277, 1), (1013, 1), (1657, 1), (30269, 1)],
    );
    test(
        u64::power_of_2(47) + 1,
        &[(3, 1), (283, 1), (165768537521, 1)],
    );
    // - r == qarr[j] in ll_factor_squfof_u64
    // - !done in ll_factor_squfof_u64
    // - r != 1 in ll_factor_squfof_u64
    test(
        u64::power_of_2(48) + 1,
        &[(193, 1), (65537, 1), (22253377, 1)],
    );
    test(
        u64::power_of_2(49) + 1,
        &[(3, 1), (43, 1), (4363953127297, 1)],
    );
    test(
        u64::power_of_2(50) + 1,
        &[(5, 3), (41, 1), (101, 1), (8101, 1), (268501, 1)],
    );
    test(
        u64::power_of_2(51) + 1,
        &[(3, 2), (307, 1), (2857, 1), (6529, 1), (43691, 1)],
    );
    test(
        u64::power_of_2(52) + 1,
        &[(17, 1), (858001, 1), (308761441, 1)],
    );
    test(
        u64::power_of_2(53) + 1,
        &[(3, 1), (107, 1), (28059810762433, 1)],
    );
    test(
        u64::power_of_2(54) + 1,
        &[(5, 1), (13, 1), (37, 1), (109, 1), (246241, 1), (279073, 1)],
    );
    test(
        u64::power_of_2(55) + 1,
        &[(3, 1), (11, 2), (683, 1), (2971, 1), (48912491, 1)],
    );
    test(
        u64::power_of_2(56) + 1,
        &[(257, 1), (5153, 1), (54410972897, 1)],
    );
    // - q.odd() in ll_factor_squfof_u64
    test(
        u64::power_of_2(57) + 1,
        &[(3, 2), (571, 1), (174763, 1), (160465489, 1)],
    );
    test(
        u64::power_of_2(58) + 1,
        &[(5, 1), (107367629, 1), (536903681, 1)],
    );
    test(
        u64::power_of_2(59) + 1,
        &[(3, 1), (2833, 1), (37171, 1), (1824726041, 1)],
    );
    test(
        u64::power_of_2(60) + 1,
        &[(17, 1), (241, 1), (61681, 1), (4562284561, 1)],
    );
    test(u64::power_of_2(61) + 1, &[(3, 1), (768614336404564651, 1)]);
    test(
        u64::power_of_2(62) + 1,
        &[(5, 1), (5581, 1), (8681, 1), (49477, 1), (384773, 1)],
    );
    test(
        u64::power_of_2(63) + 1,
        &[(3, 3), (19, 1), (43, 1), (5419, 1), (77158673929, 1)],
    );

    test(99, &[(3, 2), (11, 1)]);
    test(999, &[(3, 3), (37, 1)]);
    test(9999, &[(3, 2), (11, 1), (101, 1)]);
    test(99999, &[(3, 2), (41, 1), (271, 1)]);
    test(999999, &[(3, 3), (7, 1), (11, 1), (13, 1), (37, 1)]);
    test(9999999, &[(3, 2), (239, 1), (4649, 1)]);
    test(99999999, &[(3, 2), (11, 1), (73, 1), (101, 1), (137, 1)]);
    test(999999999, &[(3, 4), (37, 1), (333667, 1)]);
    test(9999999999, &[(3, 2), (11, 1), (41, 1), (271, 1), (9091, 1)]);
    test(99999999999, &[(3, 2), (21649, 1), (513239, 1)]);
    test(
        999999999999,
        &[(3, 3), (7, 1), (11, 1), (13, 1), (37, 1), (101, 1), (9901, 1)],
    );
    test(9999999999999, &[(3, 2), (53, 1), (79, 1), (265371653, 1)]);
    test(
        99999999999999,
        &[(3, 2), (11, 1), (239, 1), (4649, 1), (909091, 1)],
    );
    test(
        999999999999999,
        &[(3, 3), (31, 1), (37, 1), (41, 1), (271, 1), (2906161, 1)],
    );
    test(
        9999999999999999,
        &[(3, 2), (11, 1), (17, 1), (73, 1), (101, 1), (137, 1), (5882353, 1)],
    );
    // - x == 0 in n_pp1_factor_u64
    // - factor == 0 first time in n_factor_pp1_u64
    // - pr >= b1 in n_factor_pp1_u64
    // - factor == 0 in n_factor_pp1_wrapper_u64
    // - qupto == 0 in ll_factor_squfof_u64
    test(99999999999999999, &[(3, 2), (2071723, 1), (5363222357, 1)]);
    test(
        999999999999999999,
        &[(3, 4), (7, 1), (11, 1), (13, 1), (19, 1), (37, 1), (52579, 1), (333667, 1)],
    );
    test(9999999999999999999, &[(3, 2), (1111111111111111111, 1)]);

    test(11, &[(11, 1)]);
    test(101, &[(101, 1)]);
    test(1001, &[(7, 1), (11, 1), (13, 1)]);
    test(10001, &[(73, 1), (137, 1)]);
    test(100001, &[(11, 1), (9091, 1)]);
    test(1000001, &[(101, 1), (9901, 1)]);
    test(10000001, &[(11, 1), (909091, 1)]);
    test(100000001, &[(17, 1), (5882353, 1)]);
    test(1000000001, &[(7, 1), (11, 1), (13, 1), (19, 1), (52579, 1)]);
    test(10000000001, &[(101, 1), (3541, 1), (27961, 1)]);
    test(100000000001, &[(11, 2), (23, 1), (4093, 1), (8779, 1)]);
    test(1000000000001, &[(73, 1), (137, 1), (99990001, 1)]);
    test(10000000000001, &[(11, 1), (859, 1), (1058313049, 1)]);
    test(
        100000000000001,
        &[(29, 1), (101, 1), (281, 1), (121499449, 1)],
    );
    test(
        1000000000000001,
        &[(7, 1), (11, 1), (13, 1), (211, 1), (241, 1), (2161, 1), (9091, 1)],
    );
    test(
        10000000000000001,
        &[(353, 1), (449, 1), (641, 1), (1409, 1), (69857, 1)],
    );
    test(
        100000000000000001,
        &[(11, 1), (103, 1), (4013, 1), (21993833369, 1)],
    );
    test(
        1000000000000000001,
        &[(101, 1), (9901, 1), (999999000001, 1)],
    );
    test(10000000000000000001, &[(11, 1), (909090909090909091, 1)]);

    test(13 * 13, &[(13, 2)]);
    test(251 * 251, &[(251, 2)]);
    // - !cofactor.is_prime() in u32::factor
    // - factor >= cutoff in u32::factor
    // - in n_factor_power235_u32
    // - t != 0 first time in n_factor_power235_u32
    // - t != 0 second time in n_factor_power235_u32
    // - t != 0 third time in n_factor_power235_u32
    // - t.odd() in n_factor_power235_u32
    // - n == y.square() in n_factor_power235_u32
    // - cofactor != 0 in u32::factor
    // - factor < cutoff || factor.is_prime() in u32::factor
    // - n == y.square() in n_factor_power235_u64
    // - cofactor != 0 in u64::factor
    test(65521 * 65521, &[(65521, 2)]);
    test(4294967291 * 4294967291, &[(4294967291, 2)]);

    // - t == 0 second time in n_factor_power235_u32
    // - cofactor == 0 in u32::factor
    // - factor >= cutoff && !factor.is_prime() in u32::factor
    // - found factor in u32::factor
    // - factor < cutoff in u32::factor
    test(2624473402, &[(2, 1), (36107, 1), (36343, 1)]);
    test(
        9255318658858690055,
        &[(5, 1), (11, 1), (151, 1), (1114427291855351, 1)],
    );
    test(
        17556177092145474537,
        &[(3, 4), (37, 1), (1663, 1), (125399, 1), (28090333, 1)],
    );
    // - pr < b1 in n_factor_pp1_u64
    // - pr < sqrt in n_factor_pp1_u64 second time
    // - factor != 0 second time in n_factor_pp1_u64
    // - factor == 1 second time in n_factor_pp1_u64
    // - pr >= sqrt in n_factor_pp1_u64 second time
    // - factor != 1 second time in n_factor_pp1_u64
    test(
        9037524565425377627,
        &[(63793, 1), (3293177, 1), (43019107, 1)],
    );
    // - t == 0 third time in n_factor_power235_u32
    test(2989205357, &[(53783, 1), (55579, 1)]);
    // - t == 0 first time in n_factor_power235_u32
    test(3703249699, &[(33107, 1), (111857, 1)]);
    // - t.even() in n_factor_power235_u32
    // - t & 2 == 0 in n_factor_power235_u32
    // - t & 4 == 0 in n_factor_power235_u32
    test(3763227521, &[(38317, 1), (98213, 1)]);
    // - t & 2 != 0 in n_factor_power235_u32
    // - n != y.pow(3) in n_factor_power235_u32
    test(2574439811, &[(40961, 1), (62851, 1)]);
    // - n != y.square() in n_factor_power235_u32
    test(3819439849, &[(56659, 1), (67411, 1)]);
    // - factor == 1 first time in n_factor_pp1_u64
    test(
        13303824785927286386,
        &[(2, 1), (41681503, 1), (159589072231, 1)],
    );
    // - norm == 0 in n_pp1_factor_u64
    test(10520651756201345771, &[(3995749, 1), (2632961118479, 1)]);
    // - finished_loop in ll_factor_squfof_u64 first time
    // - factor == 0 first time in n_factor_squfof_u64
    // - factor == 0 second time in n_factor_squfof_u64
    // - factor != 0 second time in n_factor_squfof_u64
    // - rem != 0 in n_factor_squfof_u64
    // - factor != 1 && factor != n in n_factor_squfof_u64
    test(3090502153497041509, &[(1036310063, 1), (2982217643, 1)]);
    // - rem == 0 in n_factor_squfof_u64
    // - factor == 1 || factor == n in n_factor_squfof_u64
    test(
        9489796947312579142,
        &[(2, 1), (7, 2), (2968261, 1), (32623365239, 1)],
    );
    // - n_hi != 0 in ll_factor_squfof_u64
    // - in limbs_sqrt_rem_to_out_u64
    // - xs_hi != 0 in limbs_sqrt_rem_to_out_u64
    // - xs_hi != 0 && shift != 0 in limbs_sqrt_rem_to_out_u64
    test(
        11309902016914315274,
        &[(2, 1), (1662217, 1), (3402053407261, 1)],
    );
    // - t & 4 != 0 in n_factor_power235_u32
    // - n != y.pow(5) in n_factor_power235_u32
    test(764502619, &[(27509, 1), (27791, 1)]);
    // - bits < 31 in n_factor_pp1_wrapper_u64
    // - q == 0 || num == 0 in ll_factor_squfof_u64
    test(568345381095148801, &[(27457, 4)]);
    // - n == y.pow(3) in n_factor_power235_u64
    test(20699471212993, &[(27457, 3)]);
    // - r == 1 in ll_factor_squfof_u64
    test(
        17600775324697324735,
        &[(5, 1), (6862897, 1), (512925527651, 1)],
    );
    // - factor == 0 second time in n_factor_pp1_u64
    test(7716102188277442009, &[(155049901, 1), (49765282909, 1)]);
    // - q.even() && qupto >= 50 in ll_factor_squfof_u64
    test(
        11711972945807393444,
        &[(2, 2), (17, 1), (16481, 1), (2530543, 1), (4129751, 1)],
    );
    // - q.odd() && q <= l2 && qupto >= 50 in ll_factor_squfof_u64
    test(
        5117728824117767206,
        &[(2, 1), (255121, 1), (1219831, 1), (8222453, 1)],
    );

    // primorials
    test(210, &[(2, 1), (3, 1), (5, 1), (7, 1)]);
    test(30030, &[(2, 1), (3, 1), (5, 1), (7, 1), (11, 1), (13, 1)]);
    test(
        223092870,
        &[(2, 1), (3, 1), (5, 1), (7, 1), (11, 1), (13, 1), (17, 1), (19, 1), (23, 1)],
    );
    test(
        614889782588491410,
        &[
            (2, 1),
            (3, 1),
            (5, 1),
            (7, 1),
            (11, 1),
            (13, 1),
            (17, 1),
            (19, 1),
            (23, 1),
            (29, 1),
            (31, 1),
            (37, 1),
            (41, 1),
            (43, 1),
            (47, 1),
        ],
    );
}

#[test]
fn test_factor() {
    factor_helper::<u8>();
    factor_helper::<u16>();
    factor_helper::<u32>();
    factor_helper::<u64>();
    factor_helper::<usize>();
}

fn factor_fail_helper<T: Factor + PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.factor());
}

#[test]
pub fn factor_fail() {
    factor_fail_helper::<u8>();
    factor_fail_helper::<u16>();
    factor_fail_helper::<u32>();
    factor_fail_helper::<u64>();
    factor_fail_helper::<usize>();
}

fn factor_properties_helper_helper<T: Factor + IsPrime + PrimitiveUnsigned>(n: T, test_naive: bool)
where
    <T as Factor>::FACTORS: IntoIterator<Item = (T, u8)>,
{
    let factors = n.factor().into_iter().collect_vec();
    if test_naive {
        assert_eq!(factor_naive(n), factors);
    }
    for &(p, e) in &factors {
        assert!(p.is_prime());
        assert_ne!(e, 0);
    }
    assert_eq!(
        T::product(factors.into_iter().map(|(p, e)| p.pow(u64::from(e)))),
        n
    );
}

fn factor_properties_helper_1<T: Factor + IsPrime + PrimitiveUnsigned>()
where
    <T as Factor>::FACTORS: IntoIterator<Item = (T, u8)>,
{
    if T::WIDTH < u32::WIDTH {
        for n in exhaustive_positive_primitive_ints::<T>() {
            factor_properties_helper_helper(n, true);
        }
    } else {
        for n in exhaustive_positive_primitive_ints::<T>().take(10_000_000) {
            factor_properties_helper_helper(n, true);
        }
        unsigned_gen_var_1::<T>().test_properties(|n| {
            factor_properties_helper_helper(n, false);
        });
    }
}

#[test]
fn factor_properties() {
    factor_properties_helper_1::<u8>();
    factor_properties_helper_1::<u16>();
    factor_properties_helper_1::<u32>();
    factor_properties_helper_1::<u64>();
    factor_properties_helper_1::<usize>();
}
