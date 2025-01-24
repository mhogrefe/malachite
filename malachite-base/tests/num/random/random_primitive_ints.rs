// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::{
    uniform_primitive_int_assertions, CheckedToF64, MomentStats,
};

fn random_primitive_ints_helper<T: CheckedToF64 + PrimitiveInt>(
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_pop_median: (T, Option<T>),
    expected_sample_median: (T, Option<T>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) {
    uniform_primitive_int_assertions(
        random_primitive_ints(EXAMPLE_SEED),
        T::MIN,
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
fn test_random_primitive_ints() {
    // u8
    let values = &[
        113, 239, 69, 108, 228, 210, 168, 161, 87, 32, 110, 83, 188, 34, 89, 238, 93, 200, 149, 115,
    ];
    let common_values = &[
        (214, 4097),
        (86, 4078),
        (166, 4049),
        (22, 4048),
        (126, 4047),
        (55, 4040),
        (93, 4037),
        (191, 4036),
        (36, 4035),
        (42, 4032),
    ];
    let pop_median = (127, Some(128));
    let sample_median = (127, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(127.5),
        standard_deviation: NiceFloat(73.90027063549903),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.200036621652552),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(127.4588370000015),
        standard_deviation: NiceFloat(73.908735397844),
        skewness: NiceFloat(0.0004407839380447086),
        excess_kurtosis: NiceFloat(-1.200418003526934),
    };
    random_primitive_ints_helper::<u8>(
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
    let pop_median = (32767, Some(32768));
    let sample_median = (32764, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(32767.5),
        standard_deviation: NiceFloat(18918.61361860324),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2000000005587934),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(32768.5946480003),
        standard_deviation: NiceFloat(18919.97151989925),
        skewness: NiceFloat(0.00005872108073206368),
        excess_kurtosis: NiceFloat(-1.200244178722062),
    };
    random_primitive_ints_helper::<u16>(
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
    let pop_median = (2147483647, Some(2147483648));
    let sample_median = (2150296456, Some(2150302375));
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(2147483647.5),
        standard_deviation: NiceFloat(1239850262.2531195),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2148696150.6876254),
        standard_deviation: NiceFloat(1239453907.667566),
        skewness: NiceFloat(-0.0016842295222180032),
        excess_kurtosis: NiceFloat(-1.2007050556559806),
    };
    random_primitive_ints_helper::<u32>(
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
    let pop_median = (9223372036854775807, Some(9223372036854775808));
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
    random_primitive_ints_helper::<u64>(
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
    let pop_median = (
        170141183460469231731687303715884105727,
        Some(170141183460469231731687303715884105728),
    );
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
    random_primitive_ints_helper::<u128>(
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i8
    let values = &[
        113, -17, 69, 108, -28, -46, -88, -95, 87, 32, 110, 83, -68, 34, 89, -18, 93, -56, -107,
        115,
    ];
    let common_values = &[
        (-42, 4097),
        (86, 4078),
        (-90, 4049),
        (22, 4048),
        (126, 4047),
        (55, 4040),
        (93, 4037),
        (-65, 4036),
        (36, 4035),
        (42, 4032),
    ];
    let pop_median = (-1, Some(0));
    let sample_median = (0, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-0.5),
        standard_deviation: NiceFloat(73.90027063549903),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.200036621652552),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.5135149999999996),
        standard_deviation: NiceFloat(73.89764871907164),
        skewness: NiceFloat(-0.00024093275514460485),
        excess_kurtosis: NiceFloat(-1.199484141789355),
    };
    random_primitive_ints_helper::<i8>(
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i16
    let values = &[
        -4239, 27717, -11548, -24152, 8279, 21358, 8892, -4519, -14243, 29589, -27203, -13863,
        -28043, 18463, -22121, 8622, 9735, -28591, 32584, -32655,
    ];
    let common_values = &[
        (27447, 34),
        (-898, 33),
        (5606, 33),
        (5836, 33),
        (-15023, 33),
        (-197, 32),
        (3582, 32),
        (-7314, 32),
        (19279, 32),
        (20588, 32),
    ];
    let pop_median = (-1, Some(0));
    let sample_median = (4, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-0.5),
        standard_deviation: NiceFloat(18918.61361860324),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2000000005587934),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(5.11663199999982),
        standard_deviation: NiceFloat(18918.420140333936),
        skewness: NiceFloat(-0.00020582179614538415),
        excess_kurtosis: NiceFloat(-1.19980156293678),
    };
    random_primitive_ints_helper::<i16>(
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i32
    let values = &[
        1816522609,
        -1582771484,
        1399726167,
        -296148292,
        1939195997,
        -908487235,
        1210028661,
        565094807,
        -1873730041,
        -2140045496,
        1999530985,
        -207351284,
        -147083662,
        -1197429209,
        -60545688,
        1164671716,
        -1900807950,
        -1120015889,
        130045419,
        -1296475448,
    ];
    let common_values = &[
        (20095656, 2),
        (29107328, 2),
        (83328146, 2),
        (96543416, 2),
        (-59532811, 2),
        (-72250103, 2),
        (-88423413, 2),
        (109257003, 2),
        (132308363, 2),
        (140940582, 2),
    ];
    let pop_median = (-1, Some(0));
    let sample_median = (-3037682, Some(-3035621));
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-0.5),
        standard_deviation: NiceFloat(1239850262.2531195),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1708075.0735549931),
        standard_deviation: NiceFloat(1239705769.0057693),
        skewness: NiceFloat(0.0016706713036923025),
        excess_kurtosis: NiceFloat(-1.2008047456655335),
    };
    random_primitive_ints_helper::<i32>(
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i64
    let values = &[
        -6797951759004864655,
        -1271947227506532265,
        -3901922961219270563,
        2427063716414460533,
        -9191425414850861561,
        -890566981564077079,
        -5142919287782265230,
        5002226935030621544,
        -4810431611861206798,
        -5568319649096903189,
        -4872912570782646188,
        1513424385005459611,
        2484972586252155822,
        -5374443828107932323,
        4344958725064805398,
        3252798961345668310,
        -7926092317508205845,
        -6066899635121005951,
        6654913321726770291,
        -7940875872878966649,
    ];
    let common_values = &[
        (26914038281329, 1),
        (32553719576594, 1),
        (53892651831494, 1),
        (66354421349686, 1),
        (86226284907602, 1),
        (89837182726049, 1),
        (95691351770484, 1),
        (-45554336062456, 1),
        (-45700426911569, 1),
        (-50232881235535, 1),
    ];
    let pop_median = (-1, Some(0));
    let sample_median = (-5305218289400184, Some(-5271053954352614));
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(5.325116328314171e18),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-5606718482239710.0),
        standard_deviation: NiceFloat(5.325063903618647e18),
        skewness: NiceFloat(0.0011248693866288532),
        excess_kurtosis: NiceFloat(-1.200551786344892),
    };
    random_primitive_ints_helper::<i64>(
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i128
    let values = &[
        -23463284981077418827267202649482203279,
        44771423227283929645271838218448652381,
        -16428061189408542358428117207094319609,
        92274800069126412258941638023956901490,
        -102717367487498748896125496933502158606,
        27917752305106984883503397141734686804,
        -99140989835634876684435608594951128146,
        60003549963171791765607325839025294358,
        -111914544889959057594096246794876321045,
        -146483304948093240780207189413481284493,
        -153586157979719815495295783806580152035,
        33018320828004690757952445968579104952,
        24887066387352849554815992782110776358,
        79085537771456044427857440036467563654,
        19637669411666889498466854442215856999,
        -102694834600182680427466985752932389987,
        -85298529075242965442847250193117638905,
        -67944983823469089095474618642592431761,
        105189689748742230503365861545668092951,
        -81854971460639281226116917410207070376,
    ];
    let common_values = &[
        (68570815139656170990830410045915, 1),
        (381682482227926990846204728028719, 1),
        (565207126752383841908924745713103, 1),
        (717866653939818807939025508430762, 1),
        (775173738585689418081884794376186, 1),
        (818497230601034032775791540657915, 1),
        (-307666299724089175945459600408325, 1),
        (-413570452196184856884474016102340, 1),
        (-789195894019805665974324122519229, 1),
        (-843024079296967638987633859098218, 1),
    ];
    let pop_median = (-1, Some(0));
    let sample_median = (
        -10072503186589325235371920356302834,
        Some(-9899574809150113239535729822182407),
    );
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(9.82310580711434e37),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(8.651168263124604e34),
        standard_deviation: NiceFloat(9.82707680675722e37),
        skewness: NiceFloat(0.00010083962773749455),
        excess_kurtosis: NiceFloat(-1.2019920806441844),
    };
    random_primitive_ints_helper::<i128>(
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );
}
