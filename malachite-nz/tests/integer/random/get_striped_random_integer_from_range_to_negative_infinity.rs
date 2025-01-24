// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::arithmetic::traits::Pow;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::random::striped::StripedBitSource;
use malachite_base::num::random::VariableRangeGenerator;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::strings::ToDebugString;
use malachite_nz::integer::random::get_striped_random_integer_from_range_to_negative_infinity;
use malachite_nz::integer::Integer;
use std::str::FromStr;

fn get_striped_random_integer_from_range_to_negative_infinity_helper(
    a: &str,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    out: &str,
) {
    let mut bit_source = StripedBitSource::new(
        EXAMPLE_SEED.fork("bs"),
        mean_stripe_numerator,
        mean_stripe_denominator,
    );
    let mut vrg = VariableRangeGenerator::new(EXAMPLE_SEED.fork("vr"));
    let xs = (0..10)
        .map(|_| {
            get_striped_random_integer_from_range_to_negative_infinity(
                &mut bit_source,
                &mut vrg,
                Integer::from_str(a).unwrap(),
                mean_bits_numerator,
                mean_bits_denominator,
            )
        })
        .collect_vec();
    assert_eq!(xs.to_debug_string(), out);
}

#[test]
fn test_get_striped_random_integer_from_range_to_negative_infinity() {
    get_striped_random_integer_from_range_to_negative_infinity_helper(
        "0",
        10,
        1,
        1,
        1,
        "[0, -1, -4, -1, -3, -1, -1, 0, 0, 0]",
    );
    get_striped_random_integer_from_range_to_negative_infinity_helper(
        "0",
        10,
        1,
        10,
        1,
        "[-7, -4126, -511, -255, -1, -248, -15, 0, -7, -8191]",
    );
    get_striped_random_integer_from_range_to_negative_infinity_helper(
        "0",
        10,
        1,
        100,
        1,
        "[-2920647185518839672075671782293383, -1048576, -4739083809502202445823, \
        -52881249693939267460543734043540867155861295152331915605467750832281057293232876651442515\
        08429219658089743953085078238920703, -170472354644468060339516718901910044671, \
        -1099511627776, -1864155699627791716808549365426730850182777525827580465975886163118124479\
        4885996525825572366927316848155197439, -137436864511, \
        -187077834941321271646236832607563322963774152898535, \
        -897560034344005936431766292731307813053258931074478508157776167326430855104]",
    );
    get_striped_random_integer_from_range_to_negative_infinity_helper(
        "1000",
        10,
        1,
        11,
        1,
        "[-8071, -4, 127, -1, 3, -2, -2147484671, -1, -32775, 515]",
    );
    get_striped_random_integer_from_range_to_negative_infinity_helper(
        "1000",
        10,
        1,
        100,
        1,
        "[-2151677831, 767, -2269323280383999, 1, -5008327044809161965583, \
        -466786749337581235991134723080519000284817084727848060138773225560246511976448, \
        -140672846790145, -29360131, -283605347598335, -2096896]",
    );
    get_striped_random_integer_from_range_to_negative_infinity_helper(
        "-1000",
        10,
        1,
        11,
        1,
        "[-1020, -2040, -4351, -1087, -4095, -1536, -2047, -1023, -1022, -1007]",
    );
    get_striped_random_integer_from_range_to_negative_infinity_helper(
        "-1000",
        10,
        1,
        100,
        1,
        "[-973535863568279311376735658835847, -268435456, -151115988660057280545023, \
        -78804162148071191216516082357891447914487966356096522578354327251352959924343880239689167\
        596389234574457387517706367, -61144487963324812669777136988278751233, -35184372088704, \
        -10895890978264955623462498507774930172227024735034037509048522869011439846881182062714689\
        50192652287, -34084862300097, -2922291218836765780501982453037225347447510761475, \
        -113055782260031715275206575103088333621424376204588378848515940842185162750]",
    );
}

#[test]
#[should_panic]
fn get_striped_random_integer_from_range_to_negative_infinity_fail_1() {
    let mut bit_source = StripedBitSource::new(EXAMPLE_SEED.fork("bs"), 10, 1);
    get_striped_random_integer_from_range_to_negative_infinity(
        &mut bit_source,
        &mut VariableRangeGenerator::new(EXAMPLE_SEED.fork("vr")),
        Integer::ZERO,
        1,
        0,
    );
}

#[test]
#[should_panic]
fn get_striped_random_integer_from_range_to_negative_infinity_fail_2() {
    let mut bit_source = StripedBitSource::new(EXAMPLE_SEED.fork("bs"), 10, 1);
    get_striped_random_integer_from_range_to_negative_infinity(
        &mut bit_source,
        &mut VariableRangeGenerator::new(EXAMPLE_SEED.fork("vr")),
        Integer::ZERO,
        2,
        0,
    );
}

#[test]
#[should_panic]
fn get_striped_random_integer_from_range_to_negative_infinity_fail_3() {
    let mut bit_source = StripedBitSource::new(EXAMPLE_SEED.fork("bs"), 10, 1);
    get_striped_random_integer_from_range_to_negative_infinity(
        &mut bit_source,
        &mut VariableRangeGenerator::new(EXAMPLE_SEED.fork("vr")),
        -Integer::from(10).pow(100),
        10,
        1,
    );
}
