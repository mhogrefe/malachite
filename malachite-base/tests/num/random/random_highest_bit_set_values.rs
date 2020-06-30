use malachite_base_test_util::num::float::nice_float::NiceFloat;
use malachite_base_test_util::stats::moments::{
    disc_uniform_dist_assertions, CheckedToF64, MomentStats,
};

use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::random::random_highest_bit_set_values;
use malachite_base::random::{standard_random_values, EXAMPLE_SEED};

fn random_highest_bit_set_values_helper<T: CheckedToF64 + PrimitiveInteger>(
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_pop_median: NiceFloat<f64>,
    expected_sample_median: (T, Option<T>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) {
    disc_uniform_dist_assertions(
        random_highest_bit_set_values(standard_random_values(EXAMPLE_SEED)),
        &T::power_of_two(T::WIDTH - 1),
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
fn test_random_highest_bit_set_values() {
    // u8
    let values = &[
        241, 228, 215, 188, 221, 189, 245, 151, 135, 200, 233, 140, 242, 167, 232, 228, 242, 239,
        235, 200,
    ];
    let common_values = &[
        (249, 8065),
        (183, 8045),
        (216, 8031),
        (208, 8005),
        (155, 8004),
        (173, 7997),
        (202, 7997),
        (191, 7966),
        (130, 7958),
        (196, 7954),
    ];
    let pop_median = NiceFloat(191.5);
    let sample_median = (191, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(191.5),
        stdev: NiceFloat(36.94928957368463),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-1.2001464933162425),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(191.4814529999942),
        stdev: NiceFloat(36.93806598117275),
        skewness: NiceFloat(-0.00012946335812918567),
        kurtosis: NiceFloat(-1.1988830398738464),
    };
    random_highest_bit_set_values_helper::<u8>(
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u16
    let values = &[
        61297, 53988, 41047, 41660, 51293, 38333, 37493, 43415, 42503, 65352, 60393, 37388, 44658,
        44583, 42344, 64228, 63730, 59887, 54763, 56008,
    ];
    let common_values = &[
        (44548, 58),
        (57344, 56),
        (64313, 55),
        (35057, 53),
        (37721, 53),
        (40612, 53),
        (41928, 53),
        (44259, 53),
        (37142, 51),
        (46023, 51),
    ];
    let pop_median = NiceFloat(49151.5);
    let sample_median = (49157, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(49151.5),
        stdev: NiceFloat(9459.306805997996),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-1.2000000022351742),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(49160.60158099989),
        stdev: NiceFloat(9458.368628370561),
        skewness: NiceFloat(-0.0011390662756405401),
        kurtosis: NiceFloat(-1.200388191787568),
    };
    random_highest_bit_set_values_helper::<u16>(
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );

    // u32
    let values = &[
        3964006257, 2712195812, 3547209815, 3998819004, 4086679645, 3386480061, 3357512309,
        2712578455, 2421237255, 2154921800, 4147014633, 4087616012, 4147883634, 3097538087,
        4234421608, 3312155364, 2394159346, 3174951407, 2277529067, 2998491848,
    ];
    let common_values = &[
        (2159315734, 2),
        (2167579304, 2),
        (2176590976, 2),
        (2203840659, 2),
        (2206577785, 2),
        (2210582739, 2),
        (2228173416, 2),
        (2230811794, 2),
        (2238589955, 2),
        (2244027064, 2),
    ];
    let pop_median = NiceFloat(3221225471.5);
    let sample_median = (3220592110, Some(3220593796));
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(3221225471.5),
        stdev: NiceFloat(619925131.1265597),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-1.2),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(3220977685.807094),
        stdev: NiceFloat(619384909.7173147),
        skewness: NiceFloat(-0.00005599726986469473),
        kurtosis: NiceFloat(-1.1995883580002127),
    };
    random_highest_bit_set_values_helper::<u32>(
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
        11650435753269236341,
        9255318658858690055,
        17556177092145474537,
        13303824785927286386,
        14225598971885397352,
        13636312461848344818,
        12878424424612648427,
        13573831502926905428,
        10736796421860235419,
        11708344623106931630,
        13072300245601619293,
        13568330761919581206,
        12476170998200444118,
        10520651756201345771,
        12379844438588545665,
        15878285358581546099,
        10505868200830584967,
    ];
    let common_values = &[
        (9223379004893714086, 1),
        (9223391994788466704, 1),
        (9223398950893057137, 1),
        (9223404590574352402, 1),
        (9223413478971873075, 1),
        (9223425929506607302, 1),
        (9223433074090451726, 1),
        (9223438391276125494, 1),
        (9223449051868292713, 1),
        (9223458263139683410, 1),
    ];
    let pop_median = NiceFloat(1.3835058055282164e19);
    let sample_median = (13830693790857358602, Some(13830694635751939941));
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(1.3835058055282164e19),
        stdev: NiceFloat(2.6625581641570857e18),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-1.2),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(1.3832144561434841e19),
        stdev: NiceFloat(2.660846313009212e18),
        skewness: NiceFloat(0.0013170610963875562),
        kurtosis: NiceFloat(-1.1997156166162686),
    };
    random_highest_bit_set_values_helper::<u64>(
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
        214912606687753161376959141934332758109,
        323854305731529921104946490224673891847,
        262415983529595643990628941739841007218,
        237564999433439714567249110498266052850,
        198058935765576216615190700857618792532,
        241141377085303586778938998836817083310,
        230144733423641023497294629554909400086,
        228367822030979405869278360636891890411,
        193799061972845222683167418018286926963,
        186696208941218647968078823625188059421,
        203159504288473922489639749684463210680,
        195028249847822081286503296497994882086,
        249226721231925276159544743752351669382,
        189778852872136121230154158158099962727,
        237587532320755783035907621678835821469,
        254983837845695498020527357238650572551,
        272337383097469374367899988789175779695,
        275330873209211462235053165261552198679,
        258427395460299182237257690021561141080,
    ];
    let common_values = &[
        (170141252031284371387858294546294151643, 1),
        (170141551619364267771442242708540309119, 1),
        (170141565142951459658678149920612134447, 1),
        (170141748667595984115529212640629818831, 1),
        (170141901327123171550495242741392536490, 1),
        (170141958634207817421105385600678481914, 1),
        (170142001957699832765720079507424763643, 1),
        (170142258397986446359926857120026540206, 1),
        (170142309398734712090570431015245281671, 1),
        (170142407483498028493073772168096632983, 1),
    ];
    let pop_median = NiceFloat(2.5521177519070385e38);
    let sample_median = (
        255377939452212221484568068977532569574,
        Some(255378114957821544040944314548520494458),
    );
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(2.5521177519070385e38),
        stdev: NiceFloat(4.91155290355717e37),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-1.2),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(2.553032209676484e38),
        stdev: NiceFloat(4.907582941044101e37),
        skewness: NiceFloat(-0.0018251036670471157),
        kurtosis: NiceFloat(-1.1986105938157385),
    };
    random_highest_bit_set_values_helper::<u128>(
        values,
        common_values,
        pop_median,
        sample_median,
        pop_moment_stats,
        sample_moment_stats,
    );
}
