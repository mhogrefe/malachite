// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_nz::gaussian_integer::random::striped_random_gaussian_integers;

fn striped_random_gaussian_integers_helper(
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
) {
    assert_eq!(
        striped_random_gaussian_integers(
            EXAMPLE_SEED,
            mean_stripe_numerator,
            mean_stripe_denominator,
            mean_bits_numerator,
            mean_bits_denominator
        )
        .take(20)
        .map(|x| x.to_string())
        .collect_vec(),
        expected_values
    );
}

#[test]
fn test_striped_random_gaussian_integers() {
    // mean stripe = 2, mean bits = 2
    striped_random_gaussian_integers_helper(
        2,
        1,
        2,
        1,
        &[
            "i", "28-28i", "-22-4i", "2+7i", "-3", "-1-i", "-40-9i", "-12-4i", "0", "6+i", "-8+i",
            "-11+i", "33", "-12+4i", "-10+5i", "-2i", "9-7i", "8-i", "6-2i", "6i",
        ],
    );
    // mean stripe = 4, mean bits = 32
    striped_random_gaussian_integers_helper(
        4,
        1,
        32,
        1,
        &[
            "75540121799304929871856+122880i",
            "-1907242564614+44i",
            "-1017-66605579808i",
            "-112",
            "4604+114673i",
            "-262271+155889i",
            "-107255954944-8192i",
            "1823-7548692088i",
            "-545+28i",
            "-32-56952i",
            "-1376808158541222276638207953221759-77i",
            "50815870+562958551744270i",
            "894-11i",
            "-401531263-2188963289328328896514040i",
            "599232216465293763040+3i",
            "5311927213510075327081549494572906186498+6i",
            "-141695392837632+17942796625250428i",
            "9902801505770281643509506+1067253808i",
            "2110-341966301724431i",
            "110053881867+8568579i",
        ],
    );
    // mean stripe = 16, mean bits = 32
    striped_random_gaussian_integers_helper(
        16,
        1,
        32,
        1,
        &[
            "75521006248971741167616+65536i",
            "-2199023255520+32i",
            "-527-68719468544i",
            "-112",
            "4152+131071i",
            "-262145+262143i",
            "-137405429760-8192i",
            "1219-4294967296i",
            "-1023+16i",
            "-32-32768i",
            "-2596148429230815474023008414203903-127i",
            "33554432+1125899906834432i",
            "992-15i",
            "-536870911-1662282549535388802615296i",
            "1033021899046364639232+3i",
            "5104567810812868457303400322181516689408+4i",
            "-140739635838960+9007199288279040i",
            "19267264464508174443487232+1073348604i",
            "2048-283673999966207i",
            "120259085191+8390655i",
        ],
    );
    // mean stripe = 32, mean bits = 64
    striped_random_gaussian_integers_helper(
        32,
        1,
        64,
        1,
        &[
            "178427569518544464724715670468776264076361728+8192i",
            "-262144+8176i",
            "-226655146685469074391039-268435456i",
            "-67108863+4294967296i",
            "45671926166590716193865150952632647489410830335-19807040628566083848630173696i",
            "25217283965692466698592647766367652888868773818546142944566019485979788718647436525711\
            32639068666062843684114535546880+43978334404607i",
            "-4194304-1728806579227565766676057273846916536097145074328900789155504620306432i",
            "-1-16777215i",
            "31742+43556142803623322374103370143943282917375i",
            "-129703669268270284799-4123168604160i",
            "-4294967296-84374823951189178076189489232871423i",
            "16511-i",
            "-6+68719476736i",
            "-536870911+15i",
            "172863442395995195159959919445409775190016-1349149829244564015420722528901497082397654\
            8884033294107651385294848i",
            "6+227039847824022386351441756229761005297949595686600568205927356012240566272i",
            "114688+2352895257719374327969589999959125437250538596608723989400123171247155613741875\
            20i",
            "402669567-74074643721040429056i",
            "-98303+16384i",
            "4611686018427371520+127i",
        ],
    );
}

#[test]
#[should_panic]
fn striped_random_gaussian_integers_fail_1() {
    let _ = striped_random_gaussian_integers(EXAMPLE_SEED, 1, 2, 32, 1);
}

#[test]
#[should_panic]
fn striped_random_gaussian_integers_fail_2() {
    let _ = striped_random_gaussian_integers(EXAMPLE_SEED, 2, 1, 1, 0);
}
