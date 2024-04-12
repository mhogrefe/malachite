// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::random_highest_bit_set_unsigneds;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::moments::{
    uniform_primitive_int_assertions, CheckedToF64, MomentStats,
};

fn random_highest_bit_set_unsigneds_helper<T: CheckedToF64 + PrimitiveUnsigned>(
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_pop_median: (T, Option<T>),
    expected_sample_median: (T, Option<T>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) {
    uniform_primitive_int_assertions(
        random_highest_bit_set_unsigneds(EXAMPLE_SEED),
        T::power_of_2(T::WIDTH - 1),
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
fn test_random_highest_bit_set_unsigneds() {
    // u8
    let values = &[
        241, 222, 151, 226, 198, 220, 180, 212, 161, 175, 129, 241, 182, 138, 175, 145, 217, 220,
        247, 194,
    ];
    let common_values = &[
        (130, 8077),
        (249, 8039),
        (176, 8015),
        (241, 7966),
        (136, 7937),
        (205, 7933),
        (178, 7928),
        (219, 7927),
        (210, 7925),
        (230, 7924),
    ];
    let pop_median = (191, Some(192));
    let sample_median = (191, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(191.5),
        standard_deviation: NiceFloat(36.94928957368463),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2001464933162425),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(191.47808899999873),
        standard_deviation: NiceFloat(36.961138429895456),
        skewness: NiceFloat(-0.00045403607545687984),
        excess_kurtosis: NiceFloat(-1.1998683031732567),
    };
    random_highest_bit_set_unsigneds_helper::<u8>(
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u16
    let values = &[
        61297, 55435, 52113, 36166, 34170, 60868, 44820, 44177, 56814, 43920, 62926, 52397, 56477,
        62030, 53767, 54475, 41390, 52238, 49476, 64068,
    ];
    let common_values = &[
        (45680, 55),
        (43149, 54),
        (51767, 54),
        (41030, 53),
        (49628, 52),
        (51621, 52),
        (59748, 52),
        (62186, 52),
        (34198, 51),
        (36279, 51),
    ];
    let pop_median = (49151, Some(49152));
    let sample_median = (49162, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(49151.5),
        standard_deviation: NiceFloat(9459.306805997996),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2000000022351742),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(49155.96401899928),
        standard_deviation: NiceFloat(9454.356832489304),
        skewness: NiceFloat(-0.0012474540243074795),
        excess_kurtosis: NiceFloat(-1.1990519661334587),
    };
    random_highest_bit_set_unsigneds_helper::<u16>(
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u32
    let values = &[
        3964006257, 3276907976, 3451421022, 4073264610, 3109848542, 3140663214, 2279906674,
        3612658596, 3508930337, 3808334112, 3115296257, 2693817273, 3135713082, 2512715495,
        2304388648, 3178397234, 4176627051, 3554614631, 3618567415, 3594534402,
    ];
    let common_values = &[
        (2148512502, 2),
        (2152753653, 2),
        (2199919756, 2),
        (2200926180, 2),
        (2201533159, 2),
        (2203162873, 2),
        (2204334486, 2),
        (2243577040, 2),
        (2248172201, 2),
        (2250885871, 2),
    ];
    let pop_median = (3221225471, Some(3221225472));
    let sample_median = (3220111243, Some(3220113439));
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(3221225471.5),
        standard_deviation: NiceFloat(619925131.1265597),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(3220061212.7312818),
        standard_deviation: NiceFloat(620056431.3063867),
        skewness: NiceFloat(0.0014601286400032912),
        excess_kurtosis: NiceFloat(-1.1999609793276904),
    };
    random_highest_bit_set_unsigneds_helper::<u32>(
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
        15902849618696487087,
        12062424265687245175,
        10193137694460908462,
        9734517988917403762,
        17618716774223412528,
        12117930945561730236,
        13095749177794802780,
        13684732338925793861,
        17489043124220843898,
        18428543530713567946,
        9663508144831389154,
        14291728674867306832,
        14157636121372894287,
        11041743053466742106,
        11652479517924728358,
        15926995443195260196,
        10797926927713903617,
        9340643919610687288,
        13527331835876139736,
    ];
    let common_values = &[
        (9223375360960685195, 1),
        (9223375840468297219, 1),
        (9223377246635142855, 1),
        (9223378573599571464, 1),
        (9223392818083847604, 1),
        (9223397883812499659, 1),
        (9223410046446842059, 1),
        (9223414297499376833, 1),
        (9223420250639904388, 1),
        (9223423782640535515, 1),
    ];
    let pop_median = (13835058055282163711, Some(13835058055282163712));
    let sample_median = (13835015978121017591, Some(13835016964896634362));
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(1.3835058055282164e19),
        standard_deviation: NiceFloat(2.6625581641570857e18),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.3835207839699462e19),
        standard_deviation: NiceFloat(2.6620357716239826e18),
        skewness: NiceFloat(-0.001163213633403464),
        excess_kurtosis: NiceFloat(-1.1990763518455725),
    };
    random_highest_bit_set_unsigneds_helper::<u64>(
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
        259684029915037091022230980152781410491,
        274570122163304294029662138603390933020,
        227774850171603602876471193044002894743,
        228075138265181566710552389938690625316,
        212803339921546589345359493671973554838,
        290482805477667929731925894842607922053,
        194242323025343149803496343896337025882,
        273877696450250650298202452211959786285,
        202950956103661145939707262718357071703,
        278510113161417516359560622970606220871,
        245612405407790906538437072159901074531,
        192996213236671200977298813427044737421,
        311381173222019982206270836087375053399,
        176738900463469569991939158243641249503,
        288268669295331405536678348099056207715,
        204463485514563745612349536646478672573,
        185187345061019841391010982362414546856,
        234567239495355551510783097995427525513,
        322415362730128827224032220814663383318,
    ];
    let common_values = &[
        (170141233166538297897770214998043520489, 1),
        (170141477709687108748255943535626236844, 1),
        (170141963031764576849655827097535573872, 1),
        (170142211761517152257367037557307192382, 1),
        (170142224955858751899306474848524651318, 1),
        (170142344567707776674180030986486606478, 1),
        (170142519806846123597203111583658441461, 1),
        (170142677117909400976523007761944910786, 1),
        (170142818348199748232299322264932053448, 1),
        (170143268636071676474898792639923408007, 1),
    ];
    let pop_median = (
        255211775190703847597530955573826158591,
        Some(255211775190703847597530955573826158592),
    );
    let sample_median = (
        255126542430357640405925855455171138421,
        Some(255126579172819401755185863236353147245),
    );
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(2.5521177519070385e38),
        standard_deviation: NiceFloat(4.91155290355717e37),
        skewness: NiceFloat(0.0),
        excess_kurtosis: NiceFloat(-1.2),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.5515158317195176e38),
        standard_deviation: NiceFloat(4.910082612813729e37),
        skewness: NiceFloat(0.0016680470828693357),
        excess_kurtosis: NiceFloat(-1.199401614583425),
    };
    random_highest_bit_set_unsigneds_helper::<u128>(
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );
}
