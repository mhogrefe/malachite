// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::unsigned_matrix::random::random_unsigned_matrices_with_dimensions;

fn random_unsigned_matrices_with_dimensions_helper(
    height: usize,
    width: usize,
    expected_values: &[&str],
) {
    assert_eq!(
        (random_unsigned_matrices_with_dimensions::<u8>(EXAMPLE_SEED, height, width))
            .take(20)
            .map(|m| m.to_string())
            .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_unsigned_matrices_with_dimensions() {
    // 0-by-3
    random_unsigned_matrices_with_dimensions_helper(
        0,
        3,
        &[
            "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}",
            "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}",
            "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}", "[]_{0,3}",
        ],
    );
    // 2-by-2
    random_unsigned_matrices_with_dimensions_helper(
        2,
        2,
        &[
            "[113, 239; 69, 108]",
            "[228, 210; 168, 161]",
            "[87, 32; 110, 83]",
            "[188, 34; 89, 238]",
            "[93, 200; 149, 115]",
            "[189, 149; 217, 201]",
            "[117, 146; 31, 72]",
            "[151, 169; 174, 33]",
            "[7, 38; 81, 144]",
            "[72, 127; 113, 128]",
            "[233, 107; 46, 119]",
            "[12, 18; 164, 243]",
            "[114, 174; 59, 247]",
            "[39, 174; 160, 184]",
            "[104, 37; 100, 252]",
            "[228, 122; 107, 69]",
            "[242, 248; 179, 142]",
            "[239, 233; 61, 189]",
            "[235, 85; 192, 7]",
            "[200, 90; 185, 178]",
        ],
    );
    // 2-by-3
    random_unsigned_matrices_with_dimensions_helper(
        2,
        3,
        &[
            "[113, 239, 69; 108, 228, 210]",
            "[168, 161, 87; 32, 110, 83]",
            "[188, 34, 89; 238, 93, 200]",
            "[149, 115, 189; 149, 217, 201]",
            "[117, 146, 31; 72, 151, 169]",
            "[174, 33, 7; 38, 81, 144]",
            "[72, 127, 113; 128, 233, 107]",
            "[46, 119, 12; 18, 164, 243]",
            "[114, 174, 59; 247, 39, 174]",
            "[160, 184, 104; 37, 100, 252]",
            "[228, 122, 107; 69, 242, 248]",
            "[179, 142, 239; 233, 61, 189]",
            "[235, 85, 192; 7, 200, 90]",
            "[185, 178, 84; 0, 247, 172]",
            "[213, 239, 95; 188, 155, 24]",
            "[170, 174, 117; 195, 0, 21]",
            "[174, 187, 36; 184, 102, 101]",
            "[124, 34, 93; 13, 86, 56]",
            "[208, 35, 106; 181, 22, 204]",
            "[56, 155, 240; 100, 76, 60]",
        ],
    );
    // 3-by-1
    random_unsigned_matrices_with_dimensions_helper(
        3,
        1,
        &[
            "[113; 239; 69]",
            "[108; 228; 210]",
            "[168; 161; 87]",
            "[32; 110; 83]",
            "[188; 34; 89]",
            "[238; 93; 200]",
            "[149; 115; 189]",
            "[149; 217; 201]",
            "[117; 146; 31]",
            "[72; 151; 169]",
            "[174; 33; 7]",
            "[38; 81; 144]",
            "[72; 127; 113]",
            "[128; 233; 107]",
            "[46; 119; 12]",
            "[18; 164; 243]",
            "[114; 174; 59]",
            "[247; 39; 174]",
            "[160; 184; 104]",
            "[37; 100; 252]",
        ],
    );
}

#[test]
#[should_panic]
fn random_unsigned_matrices_with_dimensions_fail_1() {
    // height * width overflows
    let _ = random_unsigned_matrices_with_dimensions::<u8>(EXAMPLE_SEED, usize::MAX, 2);
}
