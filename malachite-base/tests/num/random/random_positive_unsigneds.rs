// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::random_positive_unsigneds;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::{
    uniform_primitive_int_assertions, CheckedToF64, MomentStats,
};

fn random_positive_unsigneds_helper<T: CheckedToF64 + PrimitiveUnsigned>(
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_pop_median: (T, Option<T>),
    expected_sample_median: (T, Option<T>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) {
    uniform_primitive_int_assertions(
        random_positive_unsigneds::<T>(EXAMPLE_SEED),
        T::ONE,
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
fn test_random_positive_unsigneds() {
    // u8
    let values = &[
        113, 239, 69, 108, 228, 210, 168, 161, 87, 32, 110, 83, 188, 34, 89, 238, 93, 200, 149, 115,
    ];
    let common_values = &[
        (214, 4112),
        (86, 4092),
        (166, 4063),
        (22, 4061),
        (126, 4061),
        (93, 4054),
        (55, 4053),
        (191, 4052),
        (36, 4049),
        (42, 4047),
    ];
    let pop_median = (128, None);
    let sample_median = (128, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(128.0),
        standard_deviation: NiceFloat(73.6115932898254),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2000369094488188),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(127.95426100000232),
        standard_deviation: NiceFloat(73.62056563848995),
        skewness: NiceFloat(0.0005293443118466251),
        excess_kurtosis: NiceFloat(-1.2003369218920343),
    };
    random_positive_unsigneds_helper::<u8>(
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u16
    let values = &[
        61297, 27717, 53988, 41384, 8279, 21358, 8892, 61017, 51293, 29589, 38333, 51673, 37493,
        18463, 43415, 8622, 9735, 36945, 32584, 32881,
    ];
    let common_values = &[
        (27447, 34),
        (5606, 33),
        (5836, 33),
        (50513, 33),
        (64638, 33),
        (3582, 32),
        (19279, 32),
        (20588, 32),
        (27377, 32),
        (40163, 32),
    ];
    let pop_median = (32768, None);
    let sample_median = (32764, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(32768.0),
        standard_deviation: NiceFloat(18918.32494346861),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2000000005588105),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(32769.087121000004),
        standard_deviation: NiceFloat(18919.668065423208),
        skewness: NiceFloat(0.00005282326597264531),
        excess_kurtosis: NiceFloat(-1.2002448370280603),
    };
    random_positive_unsigneds_helper::<u16>(
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u32
    let values = &[
        1816522609, 2712195812, 1399726167, 3998819004, 1939195997, 3386480061, 1210028661,
        565094807, 2421237255, 2154921800, 1999530985, 4087616012, 4147883634, 3097538087,
        4234421608, 1164671716, 2394159346, 3174951407, 130045419, 2998491848,
    ];
    let common_values = &[
        (20095656, 2),
        (29107328, 2),
        (83328146, 2),
        (96543416, 2),
        (109257003, 2),
        (132308363, 2),
        (140940582, 2),
        (168698132, 2),
        (182460287, 2),
        (184573980, 2),
    ];
    let pop_median = (2147483648, None);
    let sample_median = (2150296456, Some(2150302375));
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(2147483648.0),
        standard_deviation: NiceFloat(1239850261.9644444),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2148696150.6876254),
        standard_deviation: NiceFloat(1239453907.667566),
        skewness: NiceFloat(-0.0016842295222180032),
        excess_kurtosis: NiceFloat(-1.2007050556559806),
    };
    random_positive_unsigneds_helper::<u32>(
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u64
    let values = &[
        11648792314704686961,
        17174796846203019351,
        14544821112490281053,
        2427063716414460533,
        9255318658858690055,
        17556177092145474537,
        13303824785927286386,
        5002226935030621544,
        13636312461848344818,
        12878424424612648427,
        13573831502926905428,
        1513424385005459611,
        2484972586252155822,
        13072300245601619293,
        4344958725064805398,
        3252798961345668310,
        10520651756201345771,
        12379844438588545665,
        6654913321726770291,
        10505868200830584967,
    ];
    let common_values = &[
        (26914038281329, 1),
        (32553719576594, 1),
        (53892651831494, 1),
        (66354421349686, 1),
        (86226284907602, 1),
        (89837182726049, 1),
        (95691351770484, 1),
        (166741761063383, 1),
        (171574734234584, 1),
        (212518263578065, 1),
    ];
    let pop_median = (9223372036854775808, None);
    let sample_median = (9228795451400314170, Some(9228799993322832549));
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(9.223372036854776e18),
        standard_deviation: NiceFloat(5.325116328314171e18),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(9.223151767642118e18),
        standard_deviation: NiceFloat(5.323459825978695e18),
        skewness: NiceFloat(-0.0007960342667556958),
        excess_kurtosis: NiceFloat(-1.2005689675667335),
    };
    random_positive_unsigneds_helper::<u64>(
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u128
    let values = &[
        316819081939861044636107404782286008177,
        44771423227283929645271838218448652381,
        323854305731529921104946490224673891847,
        92274800069126412258941638023956901490,
        237564999433439714567249110498266052850,
        27917752305106984883503397141734686804,
        241141377085303586778938998836817083310,
        60003549963171791765607325839025294358,
        228367822030979405869278360636891890411,
        193799061972845222683167418018286926963,
        186696208941218647968078823625188059421,
        33018320828004690757952445968579104952,
        24887066387352849554815992782110776358,
        79085537771456044427857440036467563654,
        19637669411666889498466854442215856999,
        237587532320755783035907621678835821469,
        254983837845695498020527357238650572551,
        272337383097469374367899988789175779695,
        105189689748742230503365861545668092951,
        258427395460299182237257690021561141080,
    ];
    let common_values = &[
        (68570815139656170990830410045915, 1),
        (381682482227926990846204728028719, 1),
        (565207126752383841908924745713103, 1),
        (717866653939818807939025508430762, 1),
        (775173738585689418081884794376186, 1),
        (818497230601034032775791540657915, 1),
        (1224023028796761386468452212527255, 1),
        (1379103576141836593923341631562888, 1),
        (1765193876177447622538546939111747, 1),
        (2049979073093489039458791025727172, 1),
    ];
    let pop_median = (170141183460469231731687303715884105728, None);
    let sample_median = (
        170151864710150847082485192587168481404,
        Some(170151925651660504506169909296979843840),
    );
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(1.7014118346046923e38),
        standard_deviation: NiceFloat(9.82310580711434e37),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.702375633317502e38),
        standard_deviation: NiceFloat(9.815169783582294e37),
        skewness: NiceFloat(-0.0005571163134867303),
        excess_kurtosis: NiceFloat(-1.1992896928276262),
    };
    random_positive_unsigneds_helper::<u128>(
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );
}
