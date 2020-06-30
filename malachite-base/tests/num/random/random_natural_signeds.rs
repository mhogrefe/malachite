use malachite_base_test_util::num::float::nice_float::NiceFloat;
use malachite_base_test_util::stats::moments::{
    disc_uniform_dist_assertions, CheckedToF64, MomentStats,
};

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::random::random_natural_signeds;
use malachite_base::random::EXAMPLE_SEED;

fn random_natural_signeds_helper<T: CheckedToF64 + PrimitiveSigned>(
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_pop_median: NiceFloat<f64>,
    expected_sample_median: (T, Option<T>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) {
    disc_uniform_dist_assertions(
        random_natural_signeds::<T>(EXAMPLE_SEED),
        &T::ZERO,
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
fn test_random_natural_signeds() {
    // i8
    let values = &[
        113, 100, 87, 60, 93, 61, 117, 23, 7, 72, 105, 12, 114, 39, 104, 100, 114, 111, 107, 72,
    ];
    let common_values = &[
        (121, 8065),
        (55, 8045),
        (88, 8031),
        (80, 8005),
        (27, 8004),
        (45, 7997),
        (74, 7997),
        (63, 7966),
        (2, 7958),
        (68, 7954),
    ];
    let pop_median = NiceFloat(63.5);
    let sample_median = (63, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(63.5),
        stdev: NiceFloat(36.94928957368463),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-1.2001464933162425),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(63.48145299999957),
        stdev: NiceFloat(36.93806598117268),
        skewness: NiceFloat(-0.0001294633581290759),
        kurtosis: NiceFloat(-1.1988830398738437),
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
        28529, 21220, 8279, 8892, 18525, 5565, 4725, 10647, 9735, 32584, 27625, 4620, 11890, 11815,
        9576, 31460, 30962, 27119, 21995, 23240,
    ];
    let common_values = &[
        (11780, 58),
        (24576, 56),
        (31545, 55),
        (2289, 53),
        (4953, 53),
        (7844, 53),
        (9160, 53),
        (11491, 53),
        (4374, 51),
        (13255, 51),
    ];
    let pop_median = NiceFloat(16383.5);
    let sample_median = (16389, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(16383.5),
        stdev: NiceFloat(9459.306805997996),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-1.2000000022351742),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(16392.601581000094),
        stdev: NiceFloat(9458.368628370572),
        skewness: NiceFloat(-0.0011390662756402986),
        kurtosis: NiceFloat(-1.200388191787575),
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
        1816522609, 564712164, 1399726167, 1851335356, 1939195997, 1238996413, 1210028661,
        565094807, 273753607, 7438152, 1999530985, 1940132364, 2000399986, 950054439, 2086937960,
        1164671716, 246675698, 1027467759, 130045419, 851008200,
    ];
    let common_values = &[
        (11832086, 2),
        (20095656, 2),
        (29107328, 2),
        (56357011, 2),
        (59094137, 2),
        (63099091, 2),
        (80689768, 2),
        (83328146, 2),
        (91106307, 2),
        (96543416, 2),
    ];
    let pop_median = NiceFloat(1073741823.5);
    let sample_median = (1073108462, Some(1073110148));
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(1073741823.5),
        stdev: NiceFloat(619925131.1265597),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-1.2),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1073494037.8071007),
        stdev: NiceFloat(619384909.7173146),
        skewness: NiceFloat(-0.00005599726986513312),
        kurtosis: NiceFloat(-1.1995883580002054),
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
        7951424809348243543,
        5321449075635505245,
        2427063716414460533,
        31946622003914247,
        8332805055290698729,
        4080452749072510578,
        5002226935030621544,
        4412940424993569010,
        3655052387757872619,
        4350459466072129620,
        1513424385005459611,
        2484972586252155822,
        3848928208746843485,
        4344958725064805398,
        3252798961345668310,
        1297279719346569963,
        3156472401733769857,
        6654913321726770291,
        1282496163975809159,
    ];
    let common_values = &[
        (6968038938278, 1),
        (19957933690896, 1),
        (26914038281329, 1),
        (32553719576594, 1),
        (41442117097267, 1),
        (53892651831494, 1),
        (61037235675918, 1),
        (66354421349686, 1),
        (77015013516905, 1),
        (86226284907602, 1),
    ];
    let pop_median = NiceFloat(4.611686018427388e18);
    let sample_median = (4607321754002582794, Some(4607322598897164133));
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(4.611686018427388e18),
        stdev: NiceFloat(2.6625581641570857e18),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-1.2),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(4.608772524579839e18),
        stdev: NiceFloat(2.660846313009219e18),
        skewness: NiceFloat(0.0013170610963878248),
        kurtosis: NiceFloat(-1.1997156166162775),
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
        44771423227283929645271838218448652381,
        153713122271060689373259186508789786119,
        92274800069126412258941638023956901490,
        67423815972970482835561806782381947122,
        27917752305106984883503397141734686804,
        71000193624834355047251695120932977582,
        60003549963171791765607325839025294358,
        58226638570510174137591056921007784683,
        23657878512375990951480114302402821235,
        16555025480749416236391519909303953693,
        33018320828004690757952445968579104952,
        24887066387352849554815992782110776358,
        79085537771456044427857440036467563654,
        19637669411666889498466854442215856999,
        67446348860286551304220317962951715741,
        84842654385226266288840053522766466823,
        102196199637000142636212685073291673967,
        105189689748742230503365861545668092951,
        88286211999829950505570386305677035352,
    ];
    let common_values = &[
        (68570815139656170990830410045915, 1),
        (368158895036039754938992656203391, 1),
        (381682482227926990846204728028719, 1),
        (565207126752383841908924745713103, 1),
        (717866653939818807939025508430762, 1),
        (775173738585689418081884794376186, 1),
        (818497230601034032775791540657915, 1),
        (1074937517214628239553404142434478, 1),
        (1125938265480358883127299361175943, 1),
        (1224023028796761386468452212527255, 1),
    ];
    let pop_median = NiceFloat(8.507059173023462e37);
    let sample_median = (
        85236755991742989752880765261648463846,
        Some(85236931497352312309257010832636388730),
    );
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(8.507059173023462e37),
        stdev: NiceFloat(4.91155290355717e37),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-1.2),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(8.516203750719125e37),
        stdev: NiceFloat(4.9075829410441e37),
        skewness: NiceFloat(-0.0018251036670469207),
        kurtosis: NiceFloat(-1.198610593815736),
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
