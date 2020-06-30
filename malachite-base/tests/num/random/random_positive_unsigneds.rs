use malachite_base_test_util::num::float::nice_float::NiceFloat;
use malachite_base_test_util::stats::moments::{
    disc_uniform_dist_assertions, CheckedToF64, MomentStats,
};

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::random::random_positive_unsigneds;
use malachite_base::random::EXAMPLE_SEED;

fn random_positive_unsigneds_helper<T: CheckedToF64 + PrimitiveUnsigned>(
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_pop_median: NiceFloat<f64>,
    expected_sample_median: (T, Option<T>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) {
    disc_uniform_dist_assertions(
        random_positive_unsigneds::<T>(EXAMPLE_SEED),
        &T::ONE,
        &T::MAX,
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
fn test_random_positive_unsigneds() {
    // u8
    let values = &[
        113, 228, 87, 188, 93, 189, 117, 151, 7, 72, 233, 12, 114, 39, 104, 228, 242, 239, 235, 200,
    ];
    let common_values = &[
        (88, 4079),
        (121, 4067),
        (47, 4057),
        (173, 4056),
        (123, 4053),
        (27, 4051),
        (183, 4048),
        (74, 4044),
        (16, 4036),
        (55, 4036),
    ];
    let pop_median = NiceFloat(128.0);
    let sample_median = (128, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(128.0),
        stdev: NiceFloat(73.6115932898254),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-1.2000369094488188),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(127.95984199999869),
        stdev: NiceFloat(73.62518112712627),
        skewness: NiceFloat(-0.0005525716990304842),
        kurtosis: NiceFloat(-1.2009449554459193),
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
        61297, 53988, 8279, 8892, 51293, 38333, 37493, 43415, 9735, 32584, 27625, 4620, 44658,
        44583, 9576, 31460, 63730, 59887, 21995, 23240,
    ];
    let common_values = &[
        (11780, 35),
        (13255, 34),
        (8969, 33),
        (65522, 33),
        (35057, 32),
        (64313, 32),
        (8247, 31),
        (24576, 31),
        (50513, 31),
        (54829, 31),
    ];
    let pop_median = NiceFloat(32768.0);
    let sample_median = (32770, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(32768.0),
        stdev: NiceFloat(18918.32494346861),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-1.2000000005588105),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(32778.14790899955),
        stdev: NiceFloat(18911.999165341334),
        skewness: NiceFloat(-0.0008666222841378997),
        kurtosis: NiceFloat(-1.1993550250875007),
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
    let pop_median = NiceFloat(2147483648.0);
    let sample_median = (2150296456, Some(2150302375));
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(2147483648.0),
        stdev: NiceFloat(1239850261.9644444),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-1.2),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2148696150.6876254),
        stdev: NiceFloat(1239453907.667566),
        skewness: NiceFloat(-0.0016842295222180032),
        kurtosis: NiceFloat(-1.2007050556559806),
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
    let pop_median = NiceFloat(9.223372036854776e18);
    let sample_median = (9228795451400314170, Some(9228799993322832549));
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(9.223372036854776e18),
        stdev: NiceFloat(5.325116328314171e18),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-1.2),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(9.223151767642118e18),
        stdev: NiceFloat(5.323459825978695e18),
        skewness: NiceFloat(-0.0007960342667556958),
        kurtosis: NiceFloat(-1.2005689675667335),
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
    let pop_median = NiceFloat(1.7014118346046923e38);
    let sample_median = (
        170151864710150847082485192587168481404,
        Some(170151925651660504506169909296979843840),
    );
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(1.7014118346046923e38),
        stdev: NiceFloat(9.82310580711434e37),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-1.2),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.702375633317502e38),
        stdev: NiceFloat(9.815169783582294e37),
        skewness: NiceFloat(-0.0005571163134867303),
        kurtosis: NiceFloat(-1.1992896928276262),
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
