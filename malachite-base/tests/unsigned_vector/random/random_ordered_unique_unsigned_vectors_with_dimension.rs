// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::unsigned_vector::random::*;

#[track_caller]
fn random_ordered_unique_unsigned_vectors_with_dimension_helper<T: PrimitiveUnsigned>(
    dimension: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        random_ordered_unique_unsigned_vectors_with_dimension::<T>(EXAMPLE_SEED, dimension)
            .take(20)
            .inspect(|v| assert!(v.elements.is_sorted_by(|x, y| x < y)))
            .map(|v| v.to_string())
            .collect_vec(),
        expected_values
    );
}

#[test]
fn test_random_ordered_unique_unsigned_vectors_with_dimension() {
    // dimension 0: only the 0-dimensional vector
    random_ordered_unique_unsigned_vectors_with_dimension_helper::<u8>(
        0,
        &[
            "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()", "()",
            "()", "()", "()", "()", "()", "()",
        ],
    );
    // dimension 1, u8 elements
    random_ordered_unique_unsigned_vectors_with_dimension_helper::<u8>(
        1,
        &[
            "(113)", "(239)", "(69)", "(108)", "(228)", "(210)", "(168)", "(161)", "(87)", "(32)",
            "(110)", "(83)", "(188)", "(34)", "(89)", "(238)", "(93)", "(200)", "(149)", "(115)",
        ],
    );
    // dimension 3, u16 elements
    random_ordered_unique_unsigned_vectors_with_dimension_helper::<u16>(
        3,
        &[
            "(27717, 53988, 61297)",
            "(8279, 21358, 41384)",
            "(8892, 51293, 61017)",
            "(29589, 38333, 51673)",
            "(18463, 37493, 43415)",
            "(8622, 9735, 36945)",
            "(27625, 32584, 32881)",
            "(4620, 30510, 62372)",
            "(44583, 44658, 63291)",
            "(9576, 47264, 64612)",
            "(17771, 31460, 63730)",
            "(36531, 48445, 59887)",
            "(1984, 21995, 23240)",
            "(84, 44279, 45753)",
            "(6299, 48223, 61397)",
            "(5376, 44714, 50037)",
            "(25958, 47140, 48046)",
            "(3421, 8828, 14422)",
            "(9168, 46442, 52246)",
            "(15436, 25840, 39736)",
        ],
    );
    // dimension 3, u64 elements
    random_ordered_unique_unsigned_vectors_with_dimension_helper::<u64>(
        3,
        &[
            "(11648792314704686961, 14544821112490281053, 17174796846203019351)",
            "(2427063716414460533, 9255318658858690055, 17556177092145474537)",
            "(5002226935030621544, 13303824785927286386, 13636312461848344818)",
            "(1513424385005459611, 12878424424612648427, 13573831502926905428)",
            "(2484972586252155822, 4344958725064805398, 13072300245601619293)",
            "(3252798961345668310, 10520651756201345771, 12379844438588545665)",
            "(6654913321726770291, 6675835970186520861, 10505868200830584967)",
            "(1789926758677303265, 10120821766443845785, 10549025069376278712)",
            "(1349130572197946694, 4719904547594886278, 16803490371501218854)",
            "(1064560191934068965, 4287235593199853111, 7063771197644659559)",
            "(12879645934881671052, 14487826419108801437, 17339918376835714823)",
            "(11959030311444283759, 13822701547049732183, 14763439120164669521)",
            "(4387167968186967063, 5702344507433126068, 15155234994481563480)",
            "(8406351451358205244, 14009377179391347975, 18145642094924824327)",
            "(3090502153497041509, 6583546172974348565, 13784423396442657458)",
            "(2088544682656473649, 11216330939590870031, 16937600221963686059)",
            "(345418722595047515, 634768641442919875, 8639315757046634401)",
            "(2522870791098518256, 16420035727437497053, 17703201532397579962)",
            "(5326514823793670395, 8032589287581516771, 17307052068462657828)",
            "(1778110292340054401, 9007957063259873040, 11746737840597549467)",
        ],
    );
}

#[test]
fn random_ordered_unique_unsigned_vectors_with_dimension_all_values() {
    // dimension 256: every vector holds every u8
    let expected = (0..=u8::MAX).collect_vec();
    for v in random_ordered_unique_unsigned_vectors_with_dimension::<u8>(EXAMPLE_SEED, 256).take(3)
    {
        assert_eq!(v.elements, expected);
    }
}

#[test]
#[should_panic]
fn random_ordered_unique_unsigned_vectors_with_dimension_fail_1() {
    // dimension is greater than the number of u8s
    let _ = random_ordered_unique_unsigned_vectors_with_dimension::<u8>(EXAMPLE_SEED, 257);
}
