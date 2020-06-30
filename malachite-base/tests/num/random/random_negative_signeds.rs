use malachite_base_test_util::num::float::nice_float::NiceFloat;
use malachite_base_test_util::stats::moments::{
    disc_uniform_dist_assertions, CheckedToF64, MomentStats,
};

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::random::random_negative_signeds;
use malachite_base::random::EXAMPLE_SEED;

fn random_negative_signeds_helper<T: CheckedToF64 + PrimitiveSigned>(
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_pop_median: NiceFloat<f64>,
    expected_sample_median: (T, Option<T>),
    expected_pop_moment_stats: MomentStats,
    expected_sample_moment_stats: MomentStats,
) {
    disc_uniform_dist_assertions(
        random_negative_signeds::<T>(EXAMPLE_SEED),
        &T::MIN,
        &T::NEGATIVE_ONE,
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
fn test_random_negative_signeds() {
    // i8
    let values = &[
        -15, -28, -41, -68, -35, -67, -11, -105, -121, -56, -23, -116, -14, -89, -24, -28, -14,
        -17, -21, -56,
    ];
    let common_values = &[
        (-7, 8065),
        (-73, 8045),
        (-40, 8031),
        (-48, 8005),
        (-101, 8004),
        (-54, 7997),
        (-83, 7997),
        (-65, 7966),
        (-126, 7958),
        (-60, 7954),
    ];
    let pop_median = NiceFloat(-64.5);
    let sample_median = (-65, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-64.5),
        stdev: NiceFloat(36.94928957368463),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-1.2001464933162425),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-64.51854699999991),
        stdev: NiceFloat(36.93806598117264),
        skewness: NiceFloat(-0.00012946335812907643),
        kurtosis: NiceFloat(-1.1988830398738408),
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
        -4239, -11548, -24489, -23876, -14243, -27203, -28043, -22121, -23033, -184, -5143, -28148,
        -20878, -20953, -23192, -1308, -1806, -5649, -10773, -9528,
    ];
    let common_values = &[
        (-20988, 58),
        (-8192, 56),
        (-1223, 55),
        (-21277, 53),
        (-23608, 53),
        (-24924, 53),
        (-27815, 53),
        (-30479, 53),
        (-11773, 51),
        (-14255, 51),
    ];
    let pop_median = NiceFloat(-16384.5);
    let sample_median = (-16379, None);
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-16384.5),
        stdev: NiceFloat(9459.306805997996),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-1.2000000022351742),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-16375.398418999966),
        stdev: NiceFloat(9458.368628370577),
        skewness: NiceFloat(-0.0011390662756404482),
        kurtosis: NiceFloat(-1.2003881917875792),
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
        -1582771484,
        -747757481,
        -296148292,
        -208287651,
        -908487235,
        -937454987,
        -1582388841,
        -1873730041,
        -2140045496,
        -147952663,
        -207351284,
        -147083662,
        -1197429209,
        -60545688,
        -982811932,
        -1900807950,
        -1120015889,
        -2017438229,
        -1296475448,
    ];
    let common_values = &[
        (-21329002, 2),
        (-46150866, 2),
        (-54996302, 2),
        (-59532811, 2),
        (-66341078, 2),
        (-72250103, 2),
        (-81496956, 2),
        (-83331506, 2),
        (-88423413, 2),
        (-90103020, 2),
    ];
    let pop_median = NiceFloat(-1073741824.5);
    let sample_median = (-1074375186, Some(-1074373500));
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-1073741824.5),
        stdev: NiceFloat(619925131.1265597),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-1.2),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-1073989610.192923),
        stdev: NiceFloat(619384909.7173141),
        skewness: NiceFloat(-0.000055997269865255725),
        kurtosis: NiceFloat(-1.1995883580002011),
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
        -1271947227506532265,
        -3901922961219270563,
        -6796308320440315275,
        -9191425414850861561,
        -890566981564077079,
        -5142919287782265230,
        -4221145101824154264,
        -4810431611861206798,
        -5568319649096903189,
        -4872912570782646188,
        -7709947651849316197,
        -6738399450602619986,
        -5374443828107932323,
        -4878413311789970410,
        -5970573075509107498,
        -7926092317508205845,
        -6066899635121005951,
        -2568458715128005517,
        -7940875872878966649,
    ];
    let common_values = &[
        (-22303660901939, 1),
        (-30497061311649, 1),
        (-45554336062456, 1),
        (-45700426911569, 1),
        (-50232881235535, 1),
        (-98533258057512, 1),
        (-99023209454232, 1),
        (-106167936019845, 1),
        (-114150965713548, 1),
        (-138932091854646, 1),
    ];
    let pop_median = NiceFloat(-4.611686018427388e18);
    let sample_median = (-4616050282852193014, Some(-4616049437957611675));
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-4.611686018427388e18),
        stdev: NiceFloat(2.6625581641570857e18),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-1.2),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-4.614599512275074e18),
        stdev: NiceFloat(2.6608463130092104e18),
        skewness: NiceFloat(0.0013170610963879432),
        kurtosis: NiceFloat(-1.1997156166162757),
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
        -125369760233185302086415465497435453347,
        -16428061189408542358428117207094319609,
        -77866383391342819472745665691927204238,
        -102717367487498748896125496933502158606,
        -142223431155362246848183906574149418924,
        -99140989835634876684435608594951128146,
        -110137633497297439966079977876858811370,
        -111914544889959057594096246794876321045,
        -146483304948093240780207189413481284493,
        -153586157979719815495295783806580152035,
        -137122862632464540973734857747305000776,
        -145254117073116382176871310933773329370,
        -91055645689013187303829863679416542074,
        -150503514048802342233220449273668248729,
        -102694834600182680427466985752932389987,
        -85298529075242965442847250193117638905,
        -67944983823469089095474618642592431761,
        -64951493711727001228321442170216012777,
        -81854971460639281226116917410207070376,
    ];
    let common_values = &[
        (-307666299724089175945459600408325, 1),
        (-411429924564869001796009206817675, 1),
        (-413570452196184856884474016102340, 1),
        (-789195894019805665974324122519229, 1),
        (-843024079296967638987633859098218, 1),
        (-926633104236955359276901842016159, 1),
        (-1286943689405430016863515337971589, 1),
        (-1299189825673750198478910507258059, 1),
        (-1817617794135686054996535771368739, 1),
        (-1841568568885601990044560857724957, 1),
    ];
    let pop_median = NiceFloat(-8.507059173023462e37);
    let sample_median = (
        -84904427468726241978806538454235641882,
        Some(-84904251963116919422430292883247716998),
    );
    let pop_moment_stats = MomentStats {
        mean: NiceFloat(-8.507059173023462e37),
        stdev: NiceFloat(4.91155290355717e37),
        skewness: NiceFloat(0.0),
        kurtosis: NiceFloat(-1.2),
    };
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-8.497914595328467e37),
        stdev: NiceFloat(4.907582941044099e37),
        skewness: NiceFloat(-0.0018251036670471031),
        kurtosis: NiceFloat(-1.1986105938157332),
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
