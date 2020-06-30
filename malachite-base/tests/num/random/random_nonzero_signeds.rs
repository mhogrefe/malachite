use malachite_base_test_util::num::float::nice_float::NiceFloat;
use malachite_base_test_util::stats::moments::{
    deleted_disc_uniform_dist_assertions, CheckedToF64, MomentStats,
};

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::random::random_nonzero_signeds;
use malachite_base::random::EXAMPLE_SEED;

fn random_nonzero_signeds_helper<T: CheckedToF64 + PrimitiveSigned>(
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_pop_median: NiceFloat<f64>,
    expected_sample_median: (T, Option<T>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) {
    deleted_disc_uniform_dist_assertions(
        random_nonzero_signeds::<T>(EXAMPLE_SEED),
        &T::MIN,
        &T::MAX,
        &T::ZERO,
        expected_values,
        expected_common_values,
        expected_pop_median,
        expected_sample_median,
        expected_pop_moment_stats,
        expected_sample_moment_stats,
    );
}

#[allow(clippy::decimal_literal_representation)]
#[test]
fn test_random_nonzero_signeds() {
    // i8
    let values = &[
        113, -28, 87, -68, 93, -67, 117, -105, 7, 72, -23, 12, 114, 39, 104, -28, -14, -17, -21,
        -56,
    ];
    let common_values = &[
        (88, 4079),
        (121, 4067),
        (47, 4057),
        (-83, 4056),
        (123, 4053),
        (27, 4051),
        (-73, 4048),
        (74, 4044),
        (-7, 4036),
        (16, 4036),
    ];
    let pop_median = NiceFloat(-0.5);
    let sample_median = (-1, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-0.5019607843137255),
        stdev: NiceFloat(74.04502469734098),
        skewness: NiceFloat(0.00007944171209676364),
        kurtosis: NiceFloat(-1.207067080672989),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-0.49814199999998987),
        stdev: NiceFloat(74.0199926878361),
        skewness: NiceFloat(0.0005883357441058907),
        kurtosis: NiceFloat(-1.206566395164229),
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
        -4239, -11548, 8279, 8892, -14243, -27203, -28043, -22121, 9735, 32584, 27625, 4620,
        -20878, -20953, 9576, 31460, -1806, -5649, 21995, 23240,
    ];
    let common_values = &[
        (11780, 35),
        (13255, 34),
        (-14, 33),
        (8969, 33),
        (-1223, 32),
        (-30479, 32),
        (8247, 31),
        (-2848, 31),
        (-7646, 31),
        (24576, 31),
    ];
    let pop_median = NiceFloat(-0.5);
    let sample_median = (-3, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-0.5000076295109483),
        stdev: NiceFloat(18918.757957822254),
        skewness: NiceFloat(1.209832742724696e-9),
        kurtosis: NiceFloat(-1.200027466379059),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(7.788612999999801),
        stdev: NiceFloat(18924.172011313593),
        skewness: NiceFloat(0.0005778008681197786),
        kurtosis: NiceFloat(-1.200896987198633),
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
    let pop_median = NiceFloat(-0.5);
    let sample_median = (-3037682, Some(-3035621));
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-0.5000000002328306),
        stdev: NiceFloat(1239850262.3974571),
        skewness: NiceFloat(-1.408419937135895e-19),
        kurtosis: NiceFloat(-1.2000000004190956),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1708075.0735549931),
        stdev: NiceFloat(1239705769.0057693),
        skewness: NiceFloat(0.0016706713036923025),
        kurtosis: NiceFloat(-1.2008047456655335),
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
    let pop_median = NiceFloat(0.0);
    let sample_median = (-5305218289400184, Some(-5271053954352614));
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(0.25),
        stdev: NiceFloat(5.325116328314171e18),
        skewness: NiceFloat(2.0694870984063076e-58),
        kurtosis: NiceFloat(-1.1999999999999997),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-5606718482239710.0),
        stdev: NiceFloat(5.325063903618647e18),
        skewness: NiceFloat(0.0011248693866288532),
        kurtosis: NiceFloat(-1.200551786344892),
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
    let pop_median = NiceFloat(0.0);
    let sample_median = (
        -10072503186589325235371920356302834,
        Some(-9899574809150113239535729822182407),
    );
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(0.25),
        stdev: NiceFloat(9.82310580711434e37),
        skewness: NiceFloat(3.296883156664058e-116),
        kurtosis: NiceFloat(-1.1999999999999997),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(8.651168263124604e34),
        stdev: NiceFloat(9.82707680675722e37),
        skewness: NiceFloat(0.00010083962773749455),
        kurtosis: NiceFloat(-1.2019920806441844),
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
