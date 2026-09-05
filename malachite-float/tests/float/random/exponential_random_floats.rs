// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::stats::moments::MomentStats;
use malachite_float::float::random::{
    exponential_random_floats, exponential_random_floats_from_u64s,
};
use malachite_float::test_util::common::rug_round_try_from_rounding_mode;
use malachite_float::test_util::float::random::{
    MalachiteRandGen, random_floats_helper_helper, random_floats_helper_helper_no_common_values,
};
use malachite_float::{ComparableFloat, Float};

#[test]
fn test_exponential_random_floats_vs_rug() {
    for rm in [Floor, Ceiling, Down, Up, Nearest] {
        let rug_rm = rug_round_try_from_rounding_mode(rm).unwrap();
        for prec in [1u64, 2, 10, 31, 32, 33, 63, 64, 65, 100] {
            let mut ours = exponential_random_floats(EXAMPLE_SEED, prec, rm);
            let mut bit_source = MalachiteRandGen::new(random_primitive_ints(EXAMPLE_SEED));
            let mut state = rug::rand::RandState::new_custom(&mut bit_source);
            for i in 0..50u32 {
                let x = ours.next().unwrap();
                let (theirs, o) = rug::Float::with_val_round(
                    u32::exact_from(prec),
                    rug::Float::random_exp(&mut state),
                    rug_rm,
                );
                assert_ne!(o, core::cmp::Ordering::Equal);
                assert_eq!(
                    ComparableFloat(Float::from(&theirs)),
                    ComparableFloat(x),
                    "rm {rm} prec {prec} output {i}"
                );
            }
        }
    }
}

fn exponential_random_floats_helper(
    prec: u64,
    rm: RoundingMode,
    expected_values: &[&str],
    expected_values_hex: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_common_values_hex: &[(&str, usize)],
    expected_median: (&str, Option<&str>),
    expected_median_hex: (&str, Option<&str>),
    expected_moment_stats: MomentStats,
) {
    random_floats_helper_helper(
        exponential_random_floats(EXAMPLE_SEED, prec, rm),
        expected_values,
        expected_values_hex,
        expected_common_values,
        expected_common_values_hex,
        expected_median,
        expected_median_hex,
        expected_moment_stats,
    );
}

fn exponential_random_floats_helper_no_common_values(
    prec: u64,
    rm: RoundingMode,
    expected_values: &[&str],
    expected_values_hex: &[&str],
    expected_median: (&str, Option<&str>),
    expected_median_hex: (&str, Option<&str>),
    expected_moment_stats: MomentStats,
) {
    random_floats_helper_helper_no_common_values(
        exponential_random_floats(EXAMPLE_SEED, prec, rm),
        expected_values,
        expected_values_hex,
        expected_median,
        expected_median_hex,
        expected_moment_stats,
    );
}

#[test]
fn test_exponential_random_floats() {
    // precision 1, rounding mode Floor
    let values = &[
        "0.50", "1.0", "0.50", "2.0", "2.0", "2.0", "0.062", "1.0", "0.50", "0.031", "0.50",
        "0.12", "0.50", "2.0", "1.0", "0.12", "0.25", "1.0", "0.25", "0.50",
    ];
    let values_hex = &[
        "0x0.8#1", "0x1.0#1", "0x0.8#1", "0x2.0#1", "0x2.0#1", "0x2.0#1", "0x0.1#1", "0x1.0#1",
        "0x0.8#1", "0x0.08#1", "0x0.8#1", "0x0.2#1", "0x0.8#1", "0x2.0#1", "0x1.0#1", "0x0.2#1",
        "0x0.4#1", "0x1.0#1", "0x0.4#1", "0x0.8#1",
    ];
    let common_values = &[
        ("0.50", 238550),
        ("1.0", 232495),
        ("0.25", 172430),
        ("2.0", 117311),
        ("0.12", 103442),
        ("0.062", 57034),
        ("0.031", 29878),
        ("4.0", 17790),
        ("0.016", 15051),
        ("0.0078", 7806),
    ];
    let common_values_hex = &[
        ("0x0.8#1", 238550),
        ("0x1.0#1", 232495),
        ("0x0.4#1", 172430),
        ("0x2.0#1", 117311),
        ("0x0.2#1", 103442),
        ("0x0.1#1", 57034),
        ("0x0.08#1", 29878),
        ("0x4.0#1", 17790),
        ("0x0.04#1", 15051),
        ("0x0.02#1", 7806),
    ];
    let sample_median = ("0.50", None);
    let sample_median_hex = ("0x0.8#1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.7211407371186986),
        standard_deviation: NiceFloat(0.748672452206287),
        skewness: NiceFloat(2.215557876466322),
        excess_kurtosis: NiceFloat(7.7549464711956535),
    };
    exponential_random_floats_helper(
        1,
        Floor,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // precision 1, rounding mode Nearest
    let values = &[
        "0.50", "1.0", "1.0", "2.0", "4.0", "2.0", "0.12", "2.0", "0.50", "0.062", "0.50", "0.25",
        "1.0", "2.0", "2.0", "0.12", "0.25", "1.0", "0.50", "0.50",
    ];
    let values_hex = &[
        "0x0.8#1", "0x1.0#1", "0x1.0#1", "0x2.0#1", "0x4.0#1", "0x2.0#1", "0x0.2#1", "0x2.0#1",
        "0x0.8#1", "0x0.1#1", "0x0.8#1", "0x0.4#1", "0x1.0#1", "0x2.0#1", "0x2.0#1", "0x0.2#1",
        "0x0.4#1", "0x1.0#1", "0x0.8#1", "0x0.8#1",
    ];
    let common_values = &[
        ("1.0", 249286),
        ("0.50", 215007),
        ("2.0", 173381),
        ("0.25", 141538),
        ("0.12", 81474),
        ("4.0", 47278),
        ("0.062", 43880),
        ("0.031", 22652),
        ("0.016", 11274),
        ("0.0078", 5974),
    ];
    let common_values_hex = &[
        ("0x1.0#1", 249286),
        ("0x0.8#1", 215007),
        ("0x2.0#1", 173381),
        ("0x0.4#1", 141538),
        ("0x0.2#1", 81474),
        ("0x4.0#1", 47278),
        ("0x0.1#1", 43880),
        ("0x0.08#1", 22652),
        ("0x0.04#1", 11274),
        ("0x0.02#1", 5974),
    ];
    let sample_median = ("0.50", None);
    let sample_median_hex = ("0x0.8#1", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.9609848013134196),
        standard_deviation: NiceFloat(0.9963587740717318),
        skewness: NiceFloat(2.1951346169487147),
        excess_kurtosis: NiceFloat(7.52624334385923),
    };
    exponential_random_floats_helper(
        1,
        Nearest,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // precision 2, rounding mode Nearest
    let values = &[
        "0.75", "1.5", "1.0", "3.0", "4.0", "3.0", "0.094", "1.5", "0.75", "0.047", "0.50", "0.25",
        "1.0", "2.0", "1.5", "0.19", "0.38", "1.0", "0.50", "0.50",
    ];
    let values_hex = &[
        "0x0.c#2", "0x1.8#2", "0x1.0#2", "0x3.0#2", "0x4.0#2", "0x3.0#2", "0x0.18#2", "0x1.8#2",
        "0x0.c#2", "0x0.0c#2", "0x0.8#2", "0x0.4#2", "0x1.0#2", "0x2.0#2", "0x1.8#2", "0x0.3#2",
        "0x0.6#2", "0x1.0#2", "0x0.8#2", "0x0.8#2",
    ];
    let common_values = &[
        ("1.0", 130070),
        ("0.75", 118421),
        ("1.5", 112870),
        ("0.50", 110820),
        ("2.0", 91752),
        ("0.38", 85614),
        ("0.25", 71926),
        ("3.0", 51911),
        ("0.19", 51785),
        ("0.12", 41062),
    ];
    let common_values_hex = &[
        ("0x1.0#2", 130070),
        ("0x0.c#2", 118421),
        ("0x1.8#2", 112870),
        ("0x0.8#2", 110820),
        ("0x2.0#2", 91752),
        ("0x0.6#2", 85614),
        ("0x0.4#2", 71926),
        ("0x3.0#2", 51911),
        ("0x0.3#2", 51785),
        ("0x0.2#2", 41062),
    ];
    let sample_median = ("0.75", None);
    let sample_median_hex = ("0x0.c#2", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.9886291388144471),
        standard_deviation: NiceFloat(0.997978026355109),
        skewness: NiceFloat(2.053282056403858),
        excess_kurtosis: NiceFloat(6.423815454268535),
    };
    exponential_random_floats_helper(
        2,
        Nearest,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // precision 10, rounding mode Floor
    let values = &[
        "0.63086", "1.4648", "0.96484", "2.6797", "3.6680", "2.5703", "0.096924", "1.6602",
        "0.69629", "0.052429", "0.58398", "0.23486", "0.88965", "2.1992", "1.7461", "0.16748",
        "0.35645", "1.0996", "0.44189", "0.51172",
    ];
    let values_hex = &[
        "0x0.a18#10",
        "0x1.770#10",
        "0x0.f70#10",
        "0x2.ae#10",
        "0x3.ab#10",
        "0x2.92#10",
        "0x0.18d0#10",
        "0x1.a90#10",
        "0x0.b24#10",
        "0x0.0d6c#10",
        "0x0.958#10",
        "0x0.3c2#10",
        "0x0.e3c#10",
        "0x2.33#10",
        "0x1.bf0#10",
        "0x0.2ae#10",
        "0x0.5b4#10",
        "0x1.198#10",
        "0x0.712#10",
        "0x0.830#10",
    ];
    let common_values = &[
        ("1.0020", 759),
        ("1.0352", 747),
        ("1.0000", 746),
        ("1.0488", 733),
        ("1.0059", 727),
        ("1.0117", 727),
        ("1.0645", 727),
        ("1.0566", 720),
        ("1.0586", 717),
        ("1.0449", 713),
    ];
    let common_values_hex = &[
        ("0x1.008#10", 759),
        ("0x1.090#10", 747),
        ("0x1.000#10", 746),
        ("0x1.0c8#10", 733),
        ("0x1.018#10", 727),
        ("0x1.030#10", 727),
        ("0x1.108#10", 727),
        ("0x1.0e8#10", 720),
        ("0x1.0f0#10", 717),
        ("0x1.0b8#10", 713),
    ];
    let sample_median = ("0.69238", None);
    let sample_median_hex = ("0x0.b14#10", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.9986844250059675),
        standard_deviation: NiceFloat(0.99700888919536),
        skewness: NiceFloat(1.9864405885969671),
        excess_kurtosis: NiceFloat(5.894336034032721),
    };
    exponential_random_floats_helper(
        10,
        Floor,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // precision 10, rounding mode Nearest
    let values = &[
        "0.63184", "1.4648", "0.96582", "2.6836", "3.6719", "2.5703", "0.097046", "1.6602",
        "0.69629", "0.052429", "0.58398", "0.23486", "0.88965", "2.1992", "1.7480", "0.16748",
        "0.35693", "1.0996", "0.44238", "0.51172",
    ];
    let values_hex = &[
        "0x0.a1c#10",
        "0x1.770#10",
        "0x0.f74#10",
        "0x2.af#10",
        "0x3.ac#10",
        "0x2.92#10",
        "0x0.18d8#10",
        "0x1.a90#10",
        "0x0.b24#10",
        "0x0.0d6c#10",
        "0x0.958#10",
        "0x0.3c2#10",
        "0x0.e3c#10",
        "0x2.33#10",
        "0x1.bf8#10",
        "0x0.2ae#10",
        "0x0.5b6#10",
        "0x1.198#10",
        "0x0.714#10",
        "0x0.830#10",
    ];
    let common_values = &[
        ("1.0020", 760),
        ("1.0117", 755),
        ("1.0586", 753),
        ("1.0371", 748),
        ("1.0078", 732),
        ("1.0039", 728),
        ("1.0508", 726),
        ("1.0645", 726),
        ("1.0352", 712),
        ("1.0430", 712),
    ];
    let common_values_hex = &[
        ("0x1.008#10", 760),
        ("0x1.030#10", 755),
        ("0x1.0f0#10", 753),
        ("0x1.098#10", 748),
        ("0x1.020#10", 732),
        ("0x1.010#10", 728),
        ("0x1.0d0#10", 726),
        ("0x1.108#10", 726),
        ("0x1.090#10", 712),
        ("0x1.0b0#10", 712),
    ];
    let sample_median = ("0.69238", None);
    let sample_median_hex = ("0x0.b14#10", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.9993872355424093),
        standard_deviation: NiceFloat(0.9977120835579164),
        skewness: NiceFloat(1.9864637185603102),
        excess_kurtosis: NiceFloat(5.8945032830301365),
    };
    exponential_random_floats_helper(
        10,
        Nearest,
        values,
        values_hex,
        common_values,
        common_values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // precision 64, rounding mode Nearest
    let values = &[
        "0.631482296806871037074",
        "0.131571387756087777632",
        "1.98590310849954340580",
        "0.675643940615067947883",
        "3.67111271157822466956",
        "0.342106373949484636677",
        "0.187214410357104076309",
        "0.316121094354481826211",
        "0.497961358819469742157",
        "0.660501373861054021021",
        "2.23492862837140684779",
        "0.648300332203583707660",
        "0.237828851982281709826",
        "1.20037205435198222935",
        "0.791203659636058465022",
        "2.09975457554870250602",
        "3.03478264143584471621",
        "0.288750947205866028702",
        "1.04521745830144711855",
        "0.540623823778251325403",
    ];
    let values_hex = &[
        "0x0.a1a8d2e4c9d995bd#64",
        "0x0.21aea99780717f488#64",
        "0x1.fc64256807c055ec#64",
        "0x0.acf70054aeaa189c#64",
        "0x3.abce0aec2889fc74#64",
        "0x0.57944887ddbe291d8#64",
        "0x0.2fed4899d2dd10b88#64",
        "0x0.50ed4fe1d406dc260#64",
        "0x0.7f7a6546cd24dc860#64",
        "0x0.a9169e37b24078e5#64",
        "0x2.3c244857bfd419d4#64",
        "0x0.a5f702b4cce245a9#64",
        "0x0.3ce25a054f22cc1e0#64",
        "0x1.334b953c795e7f08#64",
        "0x0.ca8c52b29cc55a66#64",
        "0x2.1989840f9ba86990#64",
        "0x3.08e783e36f797d68#64",
        "0x0.49eb9502f02f00290#64",
        "0x1.0b935f109ceae2f2#64",
        "0x0.8a6652aa90e915d3#64",
    ];
    let sample_median = ("0.692232748222089592594", Some("0.692234723848414434824"));
    let sample_median_hex = ("0x0.b1362a56d5861111#64", Some("0x0.b1364b7c15a3d731#64"));
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.9981925489787625),
        standard_deviation: NiceFloat(0.9970049225487789),
        skewness: NiceFloat(1.9903499314327484),
        excess_kurtosis: NiceFloat(5.889098282882083),
    };
    exponential_random_floats_helper_no_common_values(
        64,
        Nearest,
        values,
        values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );

    // precision 100, rounding mode Ceiling
    let values = &[
        "0.63148229668888537030756507686382",
        "2.9859031086550421982462915573877",
        "4.6711127116210075034368335121708",
        "1.5718637948921485046451869645803",
        "0.49796135869246594100755857844781",
        "0.23241150721355720154585963363561",
        "2.2349286283478840096444810176261",
        "0.80032763816457309502613885689962",
        "1.7594498585616466933157479046931",
        "0.79120365953248377364236654596493",
        "0.35689475311962274010892002205739",
        "0.099754575516176001548047776221663",
        "0.018725186475156734382209698490348",
        "0.14701527005297409379828412145714",
        "3.0452174583417586429359441765884",
        "2.0885074805379435910404413904731",
        "0.33988311067313447942689120386520",
        "1.2481596205147649344382957968109",
        "0.15508458256476259503191818963135",
        "0.21142503898889550858085896329554",
    ];
    let values_hex = &[
        "0x0.a1a8d2e4481f9275c9d995bd8#100",
        "0x2.fc642568b2b95ac807c055ebc#100",
        "0x4.abce0aec579448875c5b00688#100",
        "0x1.9265aa68d406dc2618d718e76#100",
        "0x0.7f7a65464180782dcd24dc8618#100",
        "0x0.3b7f520eb24078e562078e7a94#100",
        "0x2.3c244857a5f702b4e3c4016fc#100",
        "0x0.cce245a8007388b43ce25a05e#100",
        "0x1.c26b4e51795e7f0774a95c1b4#100",
        "0x0.ca8c52b22ae3ab559cc55a65c#100",
        "0x0.5b5d745cc3452231eb0e71a9a8#100",
        "0x0.1989840f77e503e8713317a19c#100",
        "0x0.04cb2c7f85b56eba08cf26b75c0#100",
        "0x0.25a2caf0e3dfaf7f3d2282dd24#100",
        "0x3.0b935f10c93d92679ceae2f18#100",
        "0x2.16a86d1e5ca082ef7a35dfaa8#100",
        "0x0.5702945ccdc9d7e521119fb7f0#100",
        "0x1.3f87638e4c76b0db199dd883a#100",
        "0x0.27b39f8a3abe5be6bb8edfbf0c#100",
        "0x0.361ff38c0348e669cec6613f28#100",
    ];
    let sample_median = (
        "0.69305984161680842905604458735896",
        Some("0.69306336997661593125713890921209"),
    );
    let sample_median_hex = (
        "0x0.b16c5ea9ea4616ac784e1d9b8#100",
        Some("0x0.b16c99dc1ae8bb68ac1cec444#100"),
    );
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(0.9996387767260869),
        standard_deviation: NiceFloat(0.9980925216266997),
        skewness: NiceFloat(1.9883841485773066),
        excess_kurtosis: NiceFloat(5.880132373057661),
    };
    exponential_random_floats_helper_no_common_values(
        100,
        Ceiling,
        values,
        values_hex,
        sample_median,
        sample_median_hex,
        sample_moment_stats,
    );
}

#[test]
fn exponential_random_floats_properties() {
    for rm in [Floor, Ceiling, Down, Up, Nearest] {
        for prec in [1, 3, 10, 63, 64, 65, 100] {
            for x in exponential_random_floats(EXAMPLE_SEED, prec, rm).take(1000) {
                assert!(x.is_valid());
                // - the result is positive; the exact deviate is positive, and rounding to zero
                //   would require underflow, which is unreachable by sampling
                assert!(x > 0u32);
                // - every output has precision `prec`
                assert_eq!(x.get_prec(), Some(prec));
            }
        }
    }
}

#[test]
#[should_panic]
fn exponential_random_floats_fail_1() {
    exponential_random_floats(EXAMPLE_SEED, 0, Nearest);
}

#[test]
#[should_panic]
fn exponential_random_floats_fail_2() {
    exponential_random_floats(EXAMPLE_SEED, 10, Exact);
}

// Packs a u32 stream into u64s, low half first.
struct PackU32s<J: Iterator<Item = u32>> {
    xs: J,
}

impl<J: Iterator<Item = u32>> Iterator for PackU32s<J> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let lo = u64::from(self.xs.next()?);
        let hi = u64::from(self.xs.next().unwrap_or(0));
        Some(lo | (hi << 32))
    }
}

fn rigged<'a>(
    head: &'a [u32],
    zero_chunks: u64,
    tail: &'a [u32],
) -> impl Iterator<Item = u64> + Send + Sync + 'a {
    PackU32s {
        xs: head
            .iter()
            .copied()
            .chain(core::iter::repeat_n(0u32, usize::exact_from(zero_chunks)))
            .chain(tail.iter().copied())
            .chain(core::iter::repeat_n(0u32, 8)),
    }
}

fn run_rigged_case(
    head: &[u32],
    zero_chunks: u64,
    tail: &[u32],
    prec: u64,
    rm: RoundingMode,
) -> Float {
    let ours = exponential_random_floats_from_u64s(rigged(head, zero_chunks, tail), prec, rm)
        .next()
        .unwrap();
    let mut bit_source = MalachiteRandGen::new(rigged(head, zero_chunks, tail));
    let mut state = rug::rand::RandState::new_custom(&mut bit_source);
    let (theirs, o) = rug::Float::with_val_round(
        u32::exact_from(prec),
        rug::Float::random_exp(&mut state),
        rug_round_try_from_rounding_mode(rm).unwrap(),
    );
    assert_ne!(o, core::cmp::Ordering::Equal);
    assert_eq!(
        ComparableFloat(Float::from(&theirs)),
        ComparableFloat(ours.clone()),
        "prec {prec} rm {rm}"
    );
    ours
}

// Rigged streams driving paths that sampling cannot reach in practice.
#[test]
fn test_exponential_random_floats_rigged() {
    // - a tie in the leading chunks of a deviate comparison (probability 2^(-32) per comparison),
    //   forcing the bit-by-bit tie-breaking loop: the first two chunks drawn in the first
    //   comparison are equal, the tie-breaking bits differ in the comparison's favor, the second
    //   comparison then rejects, and the third accepts with the integer part 1
    for rm in [Floor, Ceiling, Down, Up, Nearest] {
        let x = run_rigged_case(
            &[0xdeadbeef, 0xdeadbeef, 0, 0x8000_0000, 0xffff_ffff, 0x1234_5678, 0x0234_5678],
            0,
            &[],
            10,
            rm,
        );
        assert!(x > 1u32);
    }
}

// The underflow inside the final value conversion, unreachable by sampling: the fraction of the
// accepted deviate must begin with more than 2^30 zero bits. The rigged walk is cheap on both
// sides, since a fraction that is still zero costs nothing to extend; MPFR, fed the identical
// stream, performs the same walk. Release-scale: each case draws tens of millions of chunks.
#[test]
fn test_exponential_random_floats_underflow_extreme() {
    // head: the first comparison sees p = nonzero, x with high chunk 0, so p < x is false and the
    // deviate is accepted immediately with integer part 0; its fraction then needs a leading-bit
    // search that walks through the zero chunks.
    let head = &[0xdeadbeef, 0];
    // 33600000 zero chunks put the leading bit beyond position 2^30
    let z = 33600000;
    let x = run_rigged_case(head, z, &[1], 10, Floor);
    assert_eq!(ComparableFloat(x), ComparableFloat(Float::ZERO));
    let x = run_rigged_case(head, z, &[1], 10, Nearest);
    assert_eq!(ComparableFloat(x), ComparableFloat(Float::ZERO));
    let x = run_rigged_case(head, z, &[1], 10, Ceiling);
    assert!(x > 0u32);
    assert_eq!(x.get_prec(), Some(10));
}
