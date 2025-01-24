// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::factorization::prime_sieve::{
    limbs_prime_sieve_size, limbs_prime_sieve_u32, limbs_prime_sieve_u64,
};
use malachite_base::test_util::generators::unsigned_gen_var_26;
use malachite_base::test_util::num::factorization::prime_sieve::{
    limbs_prime_sieve_naive_1, limbs_prime_sieve_naive_2,
};

#[test]
fn test_limbs_prime_sieve_u32() {
    let test = |n, out, out_sieve: &[u32]| {
        let mut sieve = vec![0; limbs_prime_sieve_size::<u32>(n)];
        assert_eq!(limbs_prime_sieve_u32(&mut sieve, n), out);
        assert_eq!(sieve, out_sieve);
        let mut sieve_alt = vec![0; limbs_prime_sieve_size::<u32>(n)];
        assert_eq!(limbs_prime_sieve_naive_1::<u32>(&mut sieve_alt, n), out);
        assert_eq!(sieve, sieve_alt);
        let mut sieve_alt = vec![0; limbs_prime_sieve_size::<u32>(n)];
        assert_eq!(limbs_prime_sieve_naive_2::<u32>(&mut sieve_alt, n), out);
        assert_eq!(sieve, sieve_alt);
    };
    let test_large = |n, out| {
        let mut sieve = vec![0; limbs_prime_sieve_size::<u32>(n)];
        assert_eq!(limbs_prime_sieve_u32(&mut sieve, n), out);
        let mut sieve_alt = vec![0; limbs_prime_sieve_size::<u32>(n)];
        assert_eq!(limbs_prime_sieve_naive_1::<u32>(&mut sieve_alt, n), out);
        assert_eq!(sieve, sieve_alt);
        let mut sieve_alt = vec![0; limbs_prime_sieve_size::<u32>(n)];
        assert_eq!(limbs_prime_sieve_naive_2::<u32>(&mut sieve_alt, n), out);
        assert_eq!(sieve, sieve_alt);
    };
    // - size <= BLOCK_SIZE << 1
    // - limbs == 0 in first_block_primesieve
    // - (bits + 1) & Limb::WIDTH_MASK != 0 in first_block_primesieve
    // - (bits + 1) & Limb::WIDTH_MASK != 0
    test(5, 1, &[4294967294]);
    // - (bits + 1) & Limb::WIDTH_MASK == 0 in first_block_primesieve
    // - (bits + 1) & Limb::WIDTH_MASK == 0
    test(97, 23, &[1762821248]);
    // - limbs != 0 in first_block_primesieve
    // - offset == 0 first time in fill_bitpattern
    test(101, 24, &[1762821248, 4294967294]);
    // - n_to_bit(SEED_LIMIT + 1) >= Limb::WIDTH in first_block_primesieve
    // - bit_array[index] & mask == 0 in first_block_primesieve
    // - lindex <= bits first time in first_block_primesieve
    // - lindex > bits second time in first_block_primesieve
    // - lindex > bits first time in first_block_primesieve
    test(121, 28, &[1762821248, 4294967264]);
    // - lindex <= bits second time in first_block_primesieve
    test(187, 40, &[1762821248, 4069837280]);
    // - limbs != 0 first time in fill_bitpattern
    // - limbs == 0 second time in fill_bitpattern
    test(197, 43, &[1762821248, 848611808, 4294967294]);
    // - limbs != 0 second time in fill_bitpattern
    test(293, 60, &[1762821248, 848611808, 3299549660, 4294967294]);
    // - bit_array[index] & mask != 0 in first_block_primesieve
    test(
        529,
        97,
        &[1762821248, 848611808, 3299549660, 2510511646, 3093902182, 4294954649],
    );
    test(
        10000,
        1227,
        &[
            1762821248, 848611808, 3299549660, 2510511646, 3093902182, 1255657113, 1921893675,
            1704310490, 2276511454, 3933052807, 3442636201, 1062642164, 1957128923, 4248324347,
            2716726959, 3686403537, 3525810597, 3469209982, 3144777046, 3941341117, 1482358003,
            990820275, 2682219599, 3848526070, 2757661436, 4267419563, 1005886333, 361623151,
            3991325978, 3193600964, 3397105325, 3613891391, 535771113, 3287706519, 969495549,
            1870576883, 3526745072, 3584421084, 3585498683, 3975838511, 3365889969, 3532586489,
            1037283151, 3414129786, 4285215436, 4005484237, 1590667644, 3585963000, 3148695799,
            570277455, 4005035495, 1580573621, 2816195785, 3656121683, 788406134, 4288601775,
            3209020842, 1475950840, 3242065846, 4101944926, 1238805919, 2074062642, 2532965119,
            3010383198, 4133027549, 1790162093, 3623277869, 1878747087, 3720235807, 3033363191,
            4214476775, 2614931297, 3853071358, 3216472538, 3950886702, 2080282321, 2138895219,
            667676511, 2805099227, 1743386524, 4235696025, 1592700903, 3706043128, 3619639167,
            2080028206, 4197678553, 2138431973, 2627728235, 2372861911, 1911355103, 1566205629,
            3013582698, 1609955564, 4047489974, 4125088590, 3923174885, 3200773946, 3589010553,
            3953720370, 2080348909, 1828150423, 2537461567, 2647369563, 4126591959, 4294967295,
        ],
    );
    // - size > BLOCK_SIZE << 1
    // - offset != 0 first time in fill_bitpattern
    // - offset != 0 second time in fill_bitpattern
    // - offset <= Limb::WIDTH in fill_bitpattern
    // - offset != Limb::WIDTH in fill_bitpattern
    // - offset > 70 - 2 * Limb::WIDTH in fill_bitpattern
    // - sieve[index] & mask == 0 in block_resieve
    // - lindex <= bits + off in block_resieve
    // - lindex < off first time in block_resieve
    // - lindex < off second time in block_resieve
    // - sieve[index] & mask != 0 in block_resieve
    // - lindex >= off first time in block_resieve
    // - lindex >= off second time in block_resieve
    // - lindex > bits + off in block_resieve
    // - off >= size
    test_large(400000, 33858);
    // - Limb::WIDTH < offset < 2 * Limb::WIDTH in fill_bitpattern
    // - offset > 70 - Limb::WIDTH in fill_bitpattern
    test_large(400037, 33861);
    // - offset <= 70 - 2 * Limb::WIDTH in fill_bitpattern
    test_large(400325, 33885);
    // - offset <= 70 - Limb::WIDTH in fill_bitpattern
    // - offset != 70 - Limb::WIDTH in fill_bitpattern
    test_large(400421, 33891);
    // - offset >= 2 * Limb::WIDTH in fill_bitpattern
    test_large(400517, 33896);
    // - offset == 70 - Limb::WIDTH in fill_bitpattern
    test_large(401477, 33963);
    // - offset == 0 second time in fill_bitpattern
    test_large(401573, 33969);
    // - offset == Limb::WIDTH in fill_bitpattern
    test_large(401669, 33975);
}

#[test]
fn test_limbs_prime_sieve_u64() {
    let test = |n, out, out_sieve: &[u64]| {
        let mut sieve = vec![0; limbs_prime_sieve_size::<u64>(n)];
        assert_eq!(limbs_prime_sieve_u64(&mut sieve, n), out);
        assert_eq!(sieve, out_sieve);
        let mut sieve_alt = vec![0; limbs_prime_sieve_size::<u64>(n)];
        assert_eq!(limbs_prime_sieve_naive_1(&mut sieve_alt, n), out);
        assert_eq!(sieve, sieve_alt);
        let mut sieve_alt = vec![0; limbs_prime_sieve_size::<u64>(n)];
        assert_eq!(limbs_prime_sieve_naive_2(&mut sieve_alt, n), out);
        assert_eq!(sieve, sieve_alt);
    };
    let test_large = |n, out| {
        let mut sieve = vec![0; limbs_prime_sieve_size::<u64>(n)];
        assert_eq!(limbs_prime_sieve_u64(&mut sieve, n), out);
        let mut sieve_alt = vec![0; limbs_prime_sieve_size::<u64>(n)];
        assert_eq!(limbs_prime_sieve_naive_1(&mut sieve_alt, n), out);
        assert_eq!(sieve, sieve_alt);
        let mut sieve_alt = vec![0; limbs_prime_sieve_size::<u64>(n)];
        assert_eq!(limbs_prime_sieve_naive_2(&mut sieve_alt, n), out);
        assert_eq!(sieve, sieve_alt);
    };
    // - offset == 0 first time in fill_bitpattern
    // - limbs == 0 first time in fill_bitpattern
    // - limbs != 0 first time in fill_bitpattern
    // - limbs == 0 second time in fill_bitpattern
    // - limbs != 0 second time in fill_bitpattern
    test(197, 43, &[3644759964122252416, 18446744073709551614]);
    test(
        10000,
        1227,
        &[
            3644759964122252416,
            10782565419096678876,
            5393006238418678630,
            7319957818701628715,
            16892333181782511326,
            4564013345173304745,
            18246414135122684635,
            15832982633990452911,
            14900143419172559269,
            16927931203039886678,
            4255540678821084403,
            16529293611135626319,
            18328427464153273084,
            1553159608027356029,
            13716411700845399322,
            15521545339038054061,
            14120591978486773737,
            8034066538108113917,
            15394971334399613936,
            17076096382507834939,
            15172343443912353713,
            14663575776206761807,
            17203423806843728588,
            15401593811256715644,
            2449323022019807479,
            6788512015120334311,
            15702923061497674953,
            18419404369980956534,
            6339160591512749482,
            17617719310405205942,
            8908031218484161951,
            12929497386370857727,
            7688687648106938077,
            8069157299743544621,
            13028195705955437343,
            11231044406116339687,
            13814644363045188606,
            8934744539092860718,
            2867648781191279475,
            7487788107672218331,
            6840598294930364313,
            15546231849291725560,
            18028892106335630894,
            11286006834239234533,
            8209207660800573399,
            12943239133267650237,
            17383837070827845868,
            16849907831688649550,
            15414682953334648634,
            8935030532378000434,
            10898314446950063255,
            17723577510488942427,
            18446744073709551615,
        ],
    );
    // - offset != 0 first time in fill_bitpattern
    // - m21 != 0 in fill_bitpattern
    // - m21 < Limb::WIDTH in fill_bitpattern
    // - m21 <= 110 - Limb::WIDTH in fill_bitpattern
    // - offset != 0 second time in fill_bitpattern
    // - offset >= 2 * Limb::WIDTH in fill_bitpattern
    test_large(800000, 63949);
    // - m21 >= Limb::WIDTH in fill_bitpattern
    // - offset <= Limb::WIDTH in fill_bitpattern
    // - offset != Limb::WIDTH in fill_bitpattern
    // - offset <= 182 - 2 * Limb::WIDTH in fill_bitpattern
    test_large(800069, 63953);
    // - m21 > 110 - Limb::WIDTH in fill_bitpattern
    // - Limb::WIDTH < offset < 2 * Limb::WIDTH in fill_bitpattern
    // - offset <= 182 - Limb::WIDTH in fill_bitpattern
    // - offset != 182 - Limb::WIDTH in fill_bitpattern
    test_large(800261, 63971);
    // - offset > 182 - 2 * Limb::WIDTH in fill_bitpattern
    test_large(801797, 64088);
    // - offset > 182 - Limb::WIDTH in fill_bitpattern
    test_large(801989, 64100);
    // - m21 == 0 in fill_bitpattern
    test_large(805061, 64323);
    // - off < size
    // - offset == Limb::WIDTH in fill_bitpattern
    test_large(1800005, 135070);
    // - offset == 182 - Limb::WIDTH in fill_bitpattern
    test_large(1808261, 135646);
    // - offset == 0 second time in fill_bitpattern
    test_large(1808453, 135656);
}

#[test]
fn limbs_prime_sieve_properties() {
    unsigned_gen_var_26().test_properties(|n: u64| {
        let mut sieve = vec![0; limbs_prime_sieve_size::<u32>(n)];
        let out = limbs_prime_sieve_u32(&mut sieve, n);
        assert!(out < n);

        let mut sieve_alt = vec![0; limbs_prime_sieve_size::<u32>(n)];
        assert_eq!(limbs_prime_sieve_naive_1::<u32>(&mut sieve_alt, n), out);
        assert_eq!(sieve, sieve_alt);

        let mut sieve_alt = vec![0; limbs_prime_sieve_size::<u32>(n)];
        assert_eq!(limbs_prime_sieve_naive_2::<u32>(&mut sieve_alt, n), out);
        assert_eq!(sieve, sieve_alt);

        let mut sieve = vec![0; limbs_prime_sieve_size::<u64>(n)];
        let out = limbs_prime_sieve_u64(&mut sieve, n);
        assert!(out < n);

        let mut sieve_alt = vec![0; limbs_prime_sieve_size::<u64>(n)];
        assert_eq!(limbs_prime_sieve_naive_1::<u64>(&mut sieve_alt, n), out);
        assert_eq!(sieve, sieve_alt);

        let mut sieve_alt = vec![0; limbs_prime_sieve_size::<u64>(n)];
        assert_eq!(limbs_prime_sieve_naive_2::<u64>(&mut sieve_alt, n), out);
        assert_eq!(sieve, sieve_alt);
    });
}
