// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::random_nonzero_signeds;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::{
    deleted_uniform_primitive_int_assertions, CheckedToF64, MomentStats,
};

fn random_nonzero_signeds_helper<T: CheckedToF64 + PrimitiveSigned>(
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_pop_median: (T, Option<T>),
    expected_sample_median: (T, Option<T>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) {
    deleted_uniform_primitive_int_assertions(
        random_nonzero_signeds::<T>(EXAMPLE_SEED),
        T::MIN,
        T::MAX,
        T::ZERO,
        expected_values,
        expected_common_values,
        expected_pop_median,
        expected_sample_median,
        expected_pop_moment_stats,
        expected_sample_moment_stats,
    );
}

#[test]
fn test_random_nonzero_signeds() {
    // i8
    let values = &[
        113, -17, 69, 108, -28, -46, -88, -95, 87, 32, 110, 83, -68, 34, 89, -18, 93, -56, -107,
        115,
    ];
    let common_values = &[
        (-42, 4112),
        (86, 4092),
        (-90, 4063),
        (22, 4061),
        (126, 4061),
        (93, 4054),
        (55, 4053),
        (-65, 4052),
        (36, 4049),
        (42, 4047),
    ];
    let pop_median = (-1, None);
    let sample_median = (-1, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-0.5019607843137255),
        standard_deviation: NiceFloat(74.04502469734098),
        skewness: NiceFloat(0.00007944171209676364),
        excess_kurtosis: NiceFloat(-1.207067080672989),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.5131950000000044),
        standard_deviation: NiceFloat(74.04373630744782),
        skewness: NiceFloat(-0.00024235105395531954),
        excess_kurtosis: NiceFloat(-1.2065292546223),
    };
    random_nonzero_signeds_helper::<i8>(
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
    let pop_median = (-1, None);
    let sample_median = (4, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-0.5000076295109483),
        standard_deviation: NiceFloat(18918.757957822254),
        skewness: NiceFloat(1.2098327432880512e-9),
        excess_kurtosis: NiceFloat(-1.200027466379059),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(5.019281000000275),
        standard_deviation: NiceFloat(18918.558463263365),
        skewness: NiceFloat(-0.0001978759287576409),
        excess_kurtosis: NiceFloat(-1.1998315900739762),
    };
    random_nonzero_signeds_helper::<i16>(
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
    let pop_median = (-1, None);
    let sample_median = (-3037682, Some(-3035621));
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-0.5000000001164153),
        standard_deviation: NiceFloat(1239850262.3974571),
        skewness: NiceFloat(2.8168398703367104e-19),
        excess_kurtosis: NiceFloat(-1.2000000004190956),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1708075.0735549931),
        standard_deviation: NiceFloat(1239705769.0057693),
        skewness: NiceFloat(0.0016706713036923025),
        excess_kurtosis: NiceFloat(-1.2008047456655335),
    };
    random_nonzero_signeds_helper::<i32>(
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
    let pop_median = (-1, None);
    let sample_median = (-5305218289400184, Some(-5271053954352614));
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(5.325116328314171e18),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.1999999999999997),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-5606718482239710.0),
        standard_deviation: NiceFloat(5.325063903618647e18),
        skewness: NiceFloat(0.0011248693866288532),
        excess_kurtosis: NiceFloat(-1.200551786344892),
    };
    random_nonzero_signeds_helper::<i64>(
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
    let pop_median = (-1, None);
    let sample_median = (
        -10072503186589325235371920356302834,
        Some(-9899574809150113239535729822182407),
    );
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(0.0),
        standard_deviation: NiceFloat(9.82310580711434e37),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.1999999999999997),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(8.651168263124604e34),
        standard_deviation: NiceFloat(9.82707680675722e37),
        skewness: NiceFloat(0.00010083962773749455),
        excess_kurtosis: NiceFloat(-1.2019920806441844),
    };
    random_nonzero_signeds_helper::<i128>(
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );
}
