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
use malachite_nz::integer::random::get_striped_random_integer_from_range_to_infinity;
use malachite_nz::integer::Integer;
use std::str::FromStr;

fn get_striped_random_integer_from_range_to_infinity_helper(
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
            get_striped_random_integer_from_range_to_infinity(
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
fn test_get_striped_random_integer_from_range_to_infinity() {
    get_striped_random_integer_from_range_to_infinity_helper(
        "0",
        10,
        1,
        1,
        1,
        "[0, 1, 4, 1, 3, 1, 1, 0, 0, 0]",
    );
    get_striped_random_integer_from_range_to_infinity_helper(
        "0",
        10,
        1,
        10,
        1,
        "[7, 4126, 511, 255, 1, 248, 15, 0, 7, 8191]",
    );
    get_striped_random_integer_from_range_to_infinity_helper(
        "0",
        10,
        1,
        100,
        1,
        "[2920647185518839672075671782293383, 1048576, 4739083809502202445823, \
        528812496939392674605437340435408671558612951523319156054677508322810572932328766514425150\
        8429219658089743953085078238920703, 170472354644468060339516718901910044671, \
        1099511627776, \
        186415569962779171680854936542673085018277752582758046597588616311812447948859965258255723\
        66927316848155197439, 137436864511, 187077834941321271646236832607563322963774152898535, \
        897560034344005936431766292731307813053258931074478508157776167326430855104]",
    );
    get_striped_random_integer_from_range_to_infinity_helper(
        "1000",
        10,
        1,
        11,
        1,
        "[1020, 2040, 4351, 1087, 4095, 1536, 2047, 1023, 1022, 1007]",
    );
    get_striped_random_integer_from_range_to_infinity_helper(
        "1000",
        10,
        1,
        100,
        1,
        "[973535863568279311376735658835847, 268435456, 151115988660057280545023, \
        788041621480711912165160823578914479144879663560965225783543272513529599243438802396891675\
        96389234574457387517706367, 61144487963324812669777136988278751233, 35184372088704, \
        108958909782649556234624985077749301722270247350340375090485228690114398468811820627146895\
        0192652287, 34084862300097, 2922291218836765780501982453037225347447510761475, \
        113055782260031715275206575103088333621424376204588378848515940842185162750]",
    );
    get_striped_random_integer_from_range_to_infinity_helper(
        "-1000",
        10,
        1,
        1,
        1,
        "[-3, -2, -1, 1, 17, -2, 0, 0, -1, 0]",
    );
    get_striped_random_integer_from_range_to_infinity_helper(
        "-1000",
        10,
        1,
        11,
        1,
        "[-3, 188, -1, 3, -3, -512, 1073725455, 1023, 0, 2047]",
    );
    get_striped_random_integer_from_range_to_infinity_helper(
        "-1000",
        10,
        1,
        100,
        1,
        "[130951, 19023203726501412800, 2055, 4629700416936738823, \
        206378257939585890650633610891897828407043084058691785319121408973257569307274994499096280\
        35369831199775792242011278409759, 295147869995014168352, 132373800004962371502079, \
        507510783, 1297559231579135076369342163583007, -128]",
    );
}

#[test]
#[should_panic]
fn get_striped_random_integer_from_range_to_infinity_fail_1() {
    let mut bit_source = StripedBitSource::new(EXAMPLE_SEED.fork("bs"), 10, 1);
    get_striped_random_integer_from_range_to_infinity(
        &mut bit_source,
        &mut VariableRangeGenerator::new(EXAMPLE_SEED.fork("vr")),
        Integer::ZERO,
        1,
        0,
    );
}

#[test]
#[should_panic]
fn get_striped_random_integer_from_range_to_infinity_fail_2() {
    let mut bit_source = StripedBitSource::new(EXAMPLE_SEED.fork("bs"), 10, 1);
    get_striped_random_integer_from_range_to_infinity(
        &mut bit_source,
        &mut VariableRangeGenerator::new(EXAMPLE_SEED.fork("vr")),
        Integer::ZERO,
        2,
        0,
    );
}

#[test]
#[should_panic]
fn get_striped_random_integer_from_range_to_infinity_fail_3() {
    let mut bit_source = StripedBitSource::new(EXAMPLE_SEED.fork("bs"), 10, 1);
    get_striped_random_integer_from_range_to_infinity(
        &mut bit_source,
        &mut VariableRangeGenerator::new(EXAMPLE_SEED.fork("vr")),
        Integer::from(10u32).pow(100),
        10,
        1,
    );
}
