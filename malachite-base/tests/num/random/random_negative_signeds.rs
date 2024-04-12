// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::random_negative_signeds;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::{
    uniform_primitive_int_assertions, CheckedToF64, MomentStats,
};

fn random_negative_signeds_helper<T: CheckedToF64 + PrimitiveSigned>(
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_pop_median: (T, Option<T>),
    expected_sample_median: (T, Option<T>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) {
    uniform_primitive_int_assertions(
        random_negative_signeds::<T>(EXAMPLE_SEED),
        T::MIN,
        T::NEGATIVE_ONE,
        expected_values,
        expected_common_values,
        expected_pop_median,
        expected_sample_median,
        expected_pop_moment_stats,
        expected_sample_moment_stats,
    );
}

#[test]
fn test_random_negative_signeds() {
    // i8
    let values = &[
        -15, -34, -105, -30, -58, -36, -76, -44, -95, -81, -127, -15, -74, -118, -81, -111, -39,
        -36, -9, -62,
    ];
    let common_values = &[
        (-126, 8077),
        (-7, 8039),
        (-80, 8015),
        (-15, 7966),
        (-120, 7937),
        (-51, 7933),
        (-78, 7928),
        (-37, 7927),
        (-46, 7925),
        (-26, 7924),
    ];
    let pop_median = (-65, Some(-64));
    let sample_median = (-65, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-64.5),
        standard_deviation: NiceFloat(36.94928957368463),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2001464933162425),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-64.5219110000005),
        standard_deviation: NiceFloat(36.96113842989553),
        skewness: NiceFloat(-0.00045403607545706974),
        excess_kurtosis: NiceFloat(-1.1998683031732797),
    };
    random_negative_signeds_helper::<i8>(
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i16
    let values = &[
        -4239, -10101, -13423, -29370, -31366, -4668, -20716, -21359, -8722, -21616, -2610, -13139,
        -9059, -3506, -11769, -11061, -24146, -13298, -16060, -1468,
    ];
    let common_values = &[
        (-19856, 55),
        (-13769, 54),
        (-22387, 54),
        (-24506, 53),
        (-3350, 52),
        (-5788, 52),
        (-13915, 52),
        (-15908, 52),
        (-4224, 51),
        (-6649, 51),
    ];
    let pop_median = (-16385, Some(-16384));
    let sample_median = (-16374, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-16384.5),
        standard_deviation: NiceFloat(9459.306805997996),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2000000022351742),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-16380.035980999151),
        standard_deviation: NiceFloat(9454.356832489315),
        skewness: NiceFloat(-0.0012474540243074617),
        excess_kurtosis: NiceFloat(-1.1990519661334669),
    };
    random_negative_signeds_helper::<i16>(
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // i32
    let values = &[
        -330961039,
        -1018059320,
        -843546274,
        -221702686,
        -1185118754,
        -1154304082,
        -2015060622,
        -682308700,
        -786036959,
        -486633184,
        -1179671039,
        -1601150023,
        -1159254214,
        -1782251801,
        -1990578648,
        -1116570062,
        -118340245,
        -740352665,
        -676399881,
        -700432894,
    ];
    let common_values = &[
        (-2599452, 2),
        (-30488905, 2),
        (-41127657, 2),
        (-47856776, 2),
        (-48474862, 2),
        (-49257777, 2),
        (-60585238, 2),
        (-63413113, 2),
        (-90112461, 2),
        (-92925255, 2),
    ];
    let pop_median = (-1073741825, Some(-1073741824));
    let sample_median = (-1074856053, Some(-1074853857));
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-1073741824.5),
        standard_deviation: NiceFloat(619925131.1265597),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1074906083.2686987),
        standard_deviation: NiceFloat(620056431.3063879),
        skewness: NiceFloat(0.0014601286400033587),
        excess_kurtosis: NiceFloat(-1.199960979327699),
    };
    random_negative_signeds_helper::<i32>(
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
        -2543894455013064529,
        -6384319808022306441,
        -8253606379248643154,
        -8712226084792147854,
        -828027299486139088,
        -6328813128147821380,
        -5350994895914748836,
        -4762011734783757755,
        -957700949488707718,
        -18200542995983670,
        -8783235928878162462,
        -4155015398842244784,
        -4289107952336657329,
        -7405001020242809510,
        -6794264555784823258,
        -2519748630514291420,
        -7648817145995647999,
        -9106100154098864328,
        -4919412237833411880,
    ];
    let common_values = &[
        (-32633775716835, 1),
        (-33530332767202, 1),
        (-40083601698629, 1),
        (-54667548368563, 1),
        (-55711023537196, 1),
        (-57995675195609, 1),
        (-60421027989814, 1),
        (-63506819698059, 1),
        (-68934218411234, 1),
        (-96764893067540, 1),
    ];
    let pop_median = (-4611686018427387905, Some(-4611686018427387904));
    let sample_median = (-4611728095588534025, Some(-4611727108812917254));
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-4.611686018427388e18),
        standard_deviation: NiceFloat(2.6625581641570857e18),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-4.6115362340100536e18),
        standard_deviation: NiceFloat(2.662035771623978e18),
        skewness: NiceFloat(-0.0011632136334032948),
        excess_kurtosis: NiceFloat(-1.199076351845568),
    };
    random_negative_signeds_helper::<i64>(
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
        -80598337005901372441143627278986800965,
        -65712244757634169433712468828377278436,
        -112507516749334860586903414387765316713,
        -112207228655756896752822217493077586140,
        -127479026999391874118015113759794656618,
        -49799561443270533731448712589160289403,
        -146040043895595313659878263535431185574,
        -66404670470687813165172155219808425171,
        -137331410817277317523667344713411139753,
        -61772253759520947103813984461161990585,
        -94669961513147556924937535271867136925,
        -147286153684267262486075794004723474035,
        -28901193698918481257103771344393158057,
        -163543466457468893471435449188126961953,
        -52013697625607057926696259332712003741,
        -135818881406374717851025070785289538883,
        -155095021859918622072363625069353664600,
        -105715127425582911952591509436340685943,
        -17867004190809636239342386617104828138,
    ];
    let common_values = &[
        (-5142203300700144436674870612888, 1),
        (-182426925066696407144329295811271, 1),
        (-609085828667135587396424148113815, 1),
        (-709584458395955034053916159170888, 1),
        (-793734383127263040096517829674535, 1),
        (-1015040108753864759296367578696398, 1),
        (-1102384056888326002883014030016669, 1),
        (-1302359298776052550565920555095243, 1),
        (-1597976831171503518058570309070626, 1),
        (-1756747886524482972001931222767918, 1),
    ];
    let pop_median = (
        -85070591730234615865843651857942052865,
        Some(-85070591730234615865843651857942052864),
    );
    let sample_median = (
        -85155824490580823057448751976597073035,
        Some(-85155787748119061708188744195415064211),
    );
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-8.507059173023462e37),
        standard_deviation: NiceFloat(4.91155290355717e37),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-8.513078374898942e37),
        standard_deviation: NiceFloat(4.9100826128137265e37),
        skewness: NiceFloat(0.001668047082869008),
        excess_kurtosis: NiceFloat(-1.1994016145834168),
    };
    random_negative_signeds_helper::<i128>(
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );
}
