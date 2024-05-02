// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::random_natural_signeds;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::{
    uniform_primitive_int_assertions, CheckedToF64, MomentStats,
};

fn random_natural_signeds_helper<T: CheckedToF64 + PrimitiveSigned>(
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_pop_median: (T, Option<T>),
    expected_sample_median: (T, Option<T>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) {
    uniform_primitive_int_assertions(
        random_natural_signeds::<T>(EXAMPLE_SEED),
        T::ZERO,
        T::MAX,
        expected_values,
        expected_common_values,
        expected_pop_median,
        expected_sample_median,
        expected_pop_moment_stats,
        expected_sample_moment_stats,
    );
}

#[test]
fn test_random_natural_signeds() {
    // i8
    let values =
        &[113, 94, 23, 98, 70, 92, 52, 84, 33, 47, 1, 113, 54, 10, 47, 17, 89, 92, 119, 66];
    let common_values = &[
        (2, 8077),
        (121, 8039),
        (48, 8015),
        (113, 7966),
        (8, 7937),
        (77, 7933),
        (50, 7928),
        (91, 7927),
        (82, 7925),
        (102, 7924),
    ];
    let pop_median = (63, Some(64));
    let sample_median = (63, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(63.5),
        standard_deviation: NiceFloat(36.94928957368463),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2001464933162425),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(63.478088999999315),
        standard_deviation: NiceFloat(36.96113842989552),
        skewness: NiceFloat(-0.000454036075457304),
        excess_kurtosis: NiceFloat(-1.1998683031732713),
    };
    random_natural_signeds_helper::<i8>(
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i16
    let values = &[
        28529, 22667, 19345, 3398, 1402, 28100, 12052, 11409, 24046, 11152, 30158, 19629, 23709,
        29262, 20999, 21707, 8622, 19470, 16708, 31300,
    ];
    let common_values = &[
        (12912, 55),
        (10381, 54),
        (18999, 54),
        (8262, 53),
        (16860, 52),
        (18853, 52),
        (26980, 52),
        (29418, 52),
        (1430, 51),
        (3511, 51),
    ];
    let pop_median = (16383, Some(16384));
    let sample_median = (16394, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(16383.5),
        standard_deviation: NiceFloat(9459.306805997996),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2000000022351742),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(16387.96401900046),
        standard_deviation: NiceFloat(9454.356832489306),
        skewness: NiceFloat(-0.0012474540243073533),
        excess_kurtosis: NiceFloat(-1.199051966133461),
    };
    random_natural_signeds_helper::<i16>(
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i32
    let values = &[
        1816522609, 1129424328, 1303937374, 1925780962, 962364894, 993179566, 132423026,
        1465174948, 1361446689, 1660850464, 967812609, 546333625, 988229434, 365231847, 156905000,
        1030913586, 2029143403, 1407130983, 1471083767, 1447050754,
    ];
    let common_values = &[
        (1028854, 2),
        (5270005, 2),
        (52436108, 2),
        (53442532, 2),
        (54049511, 2),
        (55679225, 2),
        (56850838, 2),
        (96093392, 2),
        (100688553, 2),
        (103402223, 2),
    ];
    let pop_median = (1073741823, Some(1073741824));
    let sample_median = (1072627595, Some(1072629791));
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(1073741823.5),
        standard_deviation: NiceFloat(619925131.1265597),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1072577564.7313439),
        standard_deviation: NiceFloat(620056431.3063871),
        skewness: NiceFloat(0.0014601286400032305),
        excess_kurtosis: NiceFloat(-1.1999609793276922),
    };
    random_natural_signeds_helper::<i32>(
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i64
    let values = &[
        2425420277849911153,
        6679477581841711279,
        2839052228832469367,
        969765657606132654,
        511145952062627954,
        8395344737368636720,
        2894558908706954428,
        3872377140940026972,
        4461360302071018053,
        8265671087366068090,
        9205171493858792138,
        440136107976613346,
        5068356638012531024,
        4934264084518118479,
        1818371016611966298,
        2429107481069952550,
        6703623406340484388,
        1574554890859127809,
        117271882755911480,
        4303959799021363928,
    ];
    let common_values = &[
        (3324105909387, 1),
        (3803613521411, 1),
        (5209780367047, 1),
        (6536744795656, 1),
        (20781229071796, 1),
        (25846957723851, 1),
        (38009592066251, 1),
        (42260644601025, 1),
        (48213785128580, 1),
        (51745785759707, 1),
    ];
    let pop_median = (4611686018427387903, Some(4611686018427387904));
    let sample_median = (4611643941266241783, Some(4611644928041858554));
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(4.611686018427388e18),
        standard_deviation: NiceFloat(2.6625581641570857e18),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(4.611835802844851e18),
        standard_deviation: NiceFloat(2.6620357716239764e18),
        skewness: NiceFloat(-0.0011632136334031786),
        excess_kurtosis: NiceFloat(-1.1990763518455585),
    };
    random_natural_signeds_helper::<i64>(
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i128
    let values = &[
        146677898479391812904420101066401902449,
        89542846454567859290543676436897304763,
        104428938702835062297974834887506827292,
        57633666711134371144783889328118789015,
        57933954804712334978865086222806519588,
        42662156461077357613672189956089449110,
        120341622017198698000238591126723816325,
        24101139564873918071809040180452920154,
        103736512989781418566515148496075680557,
        32809772643191914208019959002472965975,
        108368929700948284627873319254722115143,
        75471221947321674806749768444016968803,
        22855029776201969245611509711160631693,
        141239989761550750474583532371490947671,
        6597717003000338260251854527757143775,
        118127485834862173804991044383172101987,
        34322302054094513880662232930594566845,
        15046161600550609659323678646530441128,
        64426056034886319779095794279543419785,
        152274179269659595492344917098779277590,
    ];
    let common_values = &[
        (49706069066166082911282159414761, 1),
        (294249217877016568639819742131116, 1),
        (779571295345117968523381651468144, 1),
        (1028301047920525679733841423086654, 1),
        (1041495389520167619171132640545590, 1),
        (1161107238544942492727270602500750, 1),
        (1336346376891865515807867774335733, 1),
        (1493657440169244835704046060805058, 1),
        (1634887730516500612018549047947720, 1),
        (2085175602444743211488924039302279, 1),
    ];
    let pop_median = (
        85070591730234615865843651857942052863,
        Some(85070591730234615865843651857942052864),
    );
    let sample_median = (
        84985358969888408674238551739287032693,
        Some(84985395712350170023498559520469041517),
    );
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(8.507059173023462e37),
        standard_deviation: NiceFloat(4.91155290355717e37),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(8.501039971148385e37),
        standard_deviation: NiceFloat(4.910082612813726e37),
        skewness: NiceFloat(0.0016680470828691063),
        excess_kurtosis: NiceFloat(-1.199401614583414),
    };
    random_natural_signeds_helper::<i128>(
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );
}
