// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::Sqrt5;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{
    rug_round_try_from_rounding_mode, test_constant, to_hex_string,
};
use malachite_float::test_util::constants::sqrt_5::rug_sqrt_5_prec_round;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_sqrt_5_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::sqrt_5_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (rug_x, rug_o) =
        rug_sqrt_5_prec_round(prec, rug_round_try_from_rounding_mode(Nearest).unwrap());
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_x)),
        ComparableFloatRef(&x)
    );
    assert_eq!(rug_o, o);
}

#[test]
pub fn test_sqrt_5_prec() {
    test_sqrt_5_prec_helper(1, "2.0", "0x2.0#1", Less);
    test_sqrt_5_prec_helper(2, "2.0", "0x2.0#2", Less);
    test_sqrt_5_prec_helper(3, "2.0", "0x2.0#3", Less);
    test_sqrt_5_prec_helper(4, "2.2", "0x2.4#4", Greater);
    test_sqrt_5_prec_helper(5, "2.2", "0x2.4#5", Greater);
    test_sqrt_5_prec_helper(6, "2.25", "0x2.4#6", Greater);
    test_sqrt_5_prec_helper(7, "2.25", "0x2.40#7", Greater);
    test_sqrt_5_prec_helper(8, "2.23", "0x2.3c#8", Less);
    test_sqrt_5_prec_helper(9, "2.234", "0x2.3c#9", Less);
    test_sqrt_5_prec_helper(10, "2.234", "0x2.3c#10", Less);
    test_sqrt_5_prec_helper(
        100,
        "2.236067977499789696409173668732",
        "0x2.3c6ef372fe94f82be73980c0c#100",
        Greater,
    );
    test_sqrt_5_prec_helper(
        1000,
        "2.236067977499789696409173668731276235440618359611525724270897245410520925637804899414414\
        408378782274969508176150773783504253267724447073863586360121533452708866778173191879165811\
        276645322639856580535761350417533785003423392414064442086432539097252592627228876299517402\
        4406816117759089094984923713907297",
        "0x2.3c6ef372fe94f82be73980c0b9db906821044ed7e744e4a3f0d8d423a1831d2a4ecfe162a7a4f6fe068e0\
        8b6b7e304fe0310de125080600583ac97481e66bc6de0d5af5d2e2f0efd0b073addff7afb8cc9a64ba38a6e2d0\
        595ba1999fbff77c2c4dc6771a096866377ee78f21b29ef3a8e389567da7b054bfd8a0ee0bc#1000",
        Less,
    );
    test_sqrt_5_prec_helper(
        10000,
        "2.236067977499789696409173668731276235440618359611525724270897245410520925637804899414414\
        408378782274969508176150773783504253267724447073863586360121533452708866778173191879165811\
        276645322639856580535761350417533785003423392414064442086432539097252592627228876299517402\
        440681611775908909498492371390729728898482088641542689894099131693577019748678884425089754\
        132956183176921499977424801530434115035957668332512498815178139408000562420855243542235556\
        106306342820234093331982933959746352271201341749614202635904737885504389687061135660045757\
        139956595566956917564578221952500060539231234005009286764875529722056766253666074485853505\
        262330678494633422242317637277026632407680104443315825733505893098136226343198686471946989\
        970180818952426445962034522141192232912598196325811104170495807048120403455994943506855551\
        855572512388641655010262436312571024449618789424682903404474716115455723201737676590460918\
        529575603577984398054155380779064393639723028756062999482213852177348592453515121046345555\
        040707227872421534778752911212121184331789335191038008011118179004590618846249647104244248\
        308880129406811314695953279447898998931691577460792461807500679877124204847380502773608291\
        559913962448914943560683462529064408327944642680888989746046308353537875042061374757606883\
        401879088192559117973574464190248537871146194090191913688035110397638436041281058110378698\
        951852014697045642021763892890884446377826385893792440046028875405398460156061705223615090\
        385775410042193684987254271850375215557693316723004778269866662446210678464272486385274578\
        213410067985645305271124180595972849455195451310172309750871496529436282902540012047780324\
        155464489988706177998190033606562243886409639287753517266295971438227956307956149523015444\
        235016538917278640913041979397111356282139367457681174922067562108887818873671671627622623\
        379877111539509682982890683018259081401003895509723261508452834587893607346396117236678366\
        571982607921440289119008995584241522495712918323216741189975720139403788197728015288723418\
        668345418382867300274315320229607628612524761028642346963020111802691220236015810127628430\
        541861717618575140690101561629091763981267225965596282349067854624161857945584442659612858\
        937564854974803490110813557514166474621951830235525956886569495816353036195574536832235265\
        007722422582873668753404700742232661451739766517420672644476219618024220397983536829835024\
        662680305467687674469001869572099585891983164402516209196461851057442482740872298204109437\
        109922361752853153022121091762951208863569597169079462572603250897522297040434128808223321\
        533901195515665140790221756461654212957878042231382078553676907726666431316593195462068720\
        646450914872744082488128177653475168679073591862464426874641991499778939913129472014591999\
        678257620639485262503594282864024622559103789556345382831782355983912962511600369101312659\
        057197182001817243605955127578519983299892856386044587104693349518653903308428042182726036\
        389445415780244174574723414697299996312510945622746959743313905497801628876810654967562756\
        4933834888459269829416314014705091414179546",
        "0x2.3c6ef372fe94f82be73980c0b9db906821044ed7e744e4a3f0d8d423a1831d2a4ecfe162a7a4f6fe068e0\
        8b6b7e304fe0310de125080600583ac97481e66bc6de0d5af5d2e2f0efd0b073addff7afb8cc9a64ba38a6e2d0\
        595ba1999fbff77c2c4dc6771a096866377ee78f21b29ef3a8e389567da7b054bfd8a0ee0bc95cdcbce753723e\
        7549b650f5c89e665d2474e79722c91c851d2eb46f03d603693b0ce9f42a10833c1d54807166a5b375a61e890b\
        6e351dec88a541ba81b91971f345a98a29e36453b954445584d1d2ccdc950cced228bebeb10153a159a773d18d\
        05e9f02064157d728069ce1e42c1180c35638395de3d7b9df78e4269d9e0ddb057f6fb1d63483203acbeb91956\
        55997865d563f7d0509e5689745d0698b127473d4f0ca887e91386ff0e8559b12b5fb0f68cfa45481a131e61ba\
        595fbd675767426a0f68d67aeaff80800320dc2ecfa818746f2544dddde556b7acd0b722b6ac52801f54d09d97\
        4ea5bbb96ba30aedaef6ca5581b3332e6d0cc08251e19a84e86b2dbd65a858f13ac9724cb0c216b72b8e335bd7\
        19c50e64c689c63ab34b32c64416349884ee3a8a9d8f1e247277f6958a8311cb2227af86a883e2b546942280e0\
        6be00df875ae1f81ac6638da8f3e9097e7397f7b2bccc9c737d7b8b613b3ab71f120bb300b903328afa8895212\
        62d98b1a716744b8e297d1a8452c9d2e4980676ce9159785c0b955ee6f92ac45cc907f40e0f3335f890532a2e0\
        38ddabe4f9c462ee12caa83ebe50f2fe5fd97b309a686b9ca8f9118708cdc50ca46ad07a888fc9664eaf2060b1\
        94235207e775470c92657014fa05ac36a183fe503abba528fb48c49778f8f7292c4e2f5da50872b885e4a782fe\
        925e80376d82f52b8652d6849b60b5f51cf5a4ac35943e9fadc1d6aff063c8044ff759a8cfff16ff086e15dee2\
        30aec69a19e8312005bbd0f0eaed8c63e17342e782d701cfb28382514a81923a6743b5e1e044530dc78ecc3654\
        f66e9b80db072ccb3c1fbd36f683b837761e85311a9a4a4b20d7bec94dd4b62b3e55bef91079d1890115368dd7\
        2c1ee83240210c57a3860ca018cf2b41453be9469a8563483153f7957aa3c310f541f1d7be88a5addf71a5291b\
        10f04adc58d867b919513ad1c0d5d57aad7942fbd446b1c8876ace403b7c4bc0d4e8a34e6d6299fc75e870a443\
        0ca49b5ad3e0a96c39b7e37534a9e8dfef4543ca121c144db7ceecdd7e808eb45e2cd56d62f2361dc1691b0c78\
        11f0132cf58d142b14d69ddfc9f5d1d63a659f4d48ae68d1826e8024c556f1274619602a48b6edf42d9f13b726\
        c8f76e9c4b035d1047a2980687df6f2166aec842c851add30adba0e9dcde823dc36bba54ce199c2457f0d17d03\
        0a4bc11eb7167b2804a570a76e49150754ac2c21e2f46bf907ca85529f96e2d298453df31ea0e8d41187f9b0eb\
        949729a72177483ccafe431256ddc1dee0a06959988df89c35efd3521ff309f8417b2b2d47b4b7d95034158b33\
        28f83ebdd7e3162d92d425f59d26a032d18b382def9b25d14b1a8e466bb02d7770bc395447da54c39819b7766e\
        0171767f2cd3f86b2068f78c7e98bb973655182e95975b7bdefbac3549bfeb3f675b946966ffaa9f9b7bd6d048\
        2d513db8caea955b547ababe65372553151e1191ad6c32fbd936c650cce7a42474f15b6dc2c57270102d05c2a8\
        ad0add49317dc758234070080f19084d63dd7535fd294c056159463b98ea76071ae8ae7912c#10000",
        Greater,
    );

    let sqrt_5_f32 = Float::sqrt_5_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(sqrt_5_f32.to_string(), "2.236068");
    assert_eq!(to_hex_string(&sqrt_5_f32), "0x2.3c6ef4#24");
    assert_eq!(sqrt_5_f32, f32::SQRT_5);

    let sqrt_5_f64 = Float::sqrt_5_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(sqrt_5_f64.to_string(), "2.2360679774997898");
    assert_eq!(to_hex_string(&sqrt_5_f64), "0x2.3c6ef372fe950#53");
    assert_eq!(sqrt_5_f64, f64::SQRT_5);
}

#[test]
#[should_panic]
fn sqrt_5_prec_fail_1() {
    Float::sqrt_5_prec(0);
}

fn test_sqrt_5_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::sqrt_5_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_x, rug_o) = rug_sqrt_5_prec_round(prec, rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_x)),
            ComparableFloatRef(&x)
        );
        assert_eq!(rug_o, o);
    }
}

#[test]
pub fn test_sqrt_5_prec_round() {
    test_sqrt_5_prec_round_helper(1, Floor, "2.0", "0x2.0#1", Less);
    test_sqrt_5_prec_round_helper(1, Ceiling, "4.0", "0x4.0#1", Greater);
    test_sqrt_5_prec_round_helper(1, Down, "2.0", "0x2.0#1", Less);
    test_sqrt_5_prec_round_helper(1, Up, "4.0", "0x4.0#1", Greater);
    test_sqrt_5_prec_round_helper(1, Nearest, "2.0", "0x2.0#1", Less);

    test_sqrt_5_prec_round_helper(2, Floor, "2.0", "0x2.0#2", Less);
    test_sqrt_5_prec_round_helper(2, Ceiling, "3.0", "0x3.0#2", Greater);
    test_sqrt_5_prec_round_helper(2, Down, "2.0", "0x2.0#2", Less);
    test_sqrt_5_prec_round_helper(2, Up, "3.0", "0x3.0#2", Greater);
    test_sqrt_5_prec_round_helper(2, Nearest, "2.0", "0x2.0#2", Less);

    test_sqrt_5_prec_round_helper(3, Floor, "2.0", "0x2.0#3", Less);
    test_sqrt_5_prec_round_helper(3, Ceiling, "2.5", "0x2.8#3", Greater);
    test_sqrt_5_prec_round_helper(3, Down, "2.0", "0x2.0#3", Less);
    test_sqrt_5_prec_round_helper(3, Up, "2.5", "0x2.8#3", Greater);
    test_sqrt_5_prec_round_helper(3, Nearest, "2.0", "0x2.0#3", Less);

    test_sqrt_5_prec_round_helper(4, Floor, "2.0", "0x2.0#4", Less);
    test_sqrt_5_prec_round_helper(4, Ceiling, "2.2", "0x2.4#4", Greater);
    test_sqrt_5_prec_round_helper(4, Down, "2.0", "0x2.0#4", Less);
    test_sqrt_5_prec_round_helper(4, Up, "2.2", "0x2.4#4", Greater);
    test_sqrt_5_prec_round_helper(4, Nearest, "2.2", "0x2.4#4", Greater);

    test_sqrt_5_prec_round_helper(5, Floor, "2.1", "0x2.2#5", Less);
    test_sqrt_5_prec_round_helper(5, Ceiling, "2.2", "0x2.4#5", Greater);
    test_sqrt_5_prec_round_helper(5, Down, "2.1", "0x2.2#5", Less);
    test_sqrt_5_prec_round_helper(5, Up, "2.2", "0x2.4#5", Greater);
    test_sqrt_5_prec_round_helper(5, Nearest, "2.2", "0x2.4#5", Greater);

    test_sqrt_5_prec_round_helper(6, Floor, "2.19", "0x2.3#6", Less);
    test_sqrt_5_prec_round_helper(6, Ceiling, "2.25", "0x2.4#6", Greater);
    test_sqrt_5_prec_round_helper(6, Down, "2.19", "0x2.3#6", Less);
    test_sqrt_5_prec_round_helper(6, Up, "2.25", "0x2.4#6", Greater);
    test_sqrt_5_prec_round_helper(6, Nearest, "2.25", "0x2.4#6", Greater);

    test_sqrt_5_prec_round_helper(7, Floor, "2.22", "0x2.38#7", Less);
    test_sqrt_5_prec_round_helper(7, Ceiling, "2.25", "0x2.40#7", Greater);
    test_sqrt_5_prec_round_helper(7, Down, "2.22", "0x2.38#7", Less);
    test_sqrt_5_prec_round_helper(7, Up, "2.25", "0x2.40#7", Greater);
    test_sqrt_5_prec_round_helper(7, Nearest, "2.25", "0x2.40#7", Greater);

    test_sqrt_5_prec_round_helper(8, Floor, "2.23", "0x2.3c#8", Less);
    test_sqrt_5_prec_round_helper(8, Ceiling, "2.25", "0x2.40#8", Greater);
    test_sqrt_5_prec_round_helper(8, Down, "2.23", "0x2.3c#8", Less);
    test_sqrt_5_prec_round_helper(8, Up, "2.25", "0x2.40#8", Greater);
    test_sqrt_5_prec_round_helper(8, Nearest, "2.23", "0x2.3c#8", Less);

    test_sqrt_5_prec_round_helper(9, Floor, "2.234", "0x2.3c#9", Less);
    test_sqrt_5_prec_round_helper(9, Ceiling, "2.24", "0x2.3e#9", Greater);
    test_sqrt_5_prec_round_helper(9, Down, "2.234", "0x2.3c#9", Less);
    test_sqrt_5_prec_round_helper(9, Up, "2.24", "0x2.3e#9", Greater);
    test_sqrt_5_prec_round_helper(9, Nearest, "2.234", "0x2.3c#9", Less);

    test_sqrt_5_prec_round_helper(10, Floor, "2.234", "0x2.3c#10", Less);
    test_sqrt_5_prec_round_helper(10, Ceiling, "2.238", "0x2.3d#10", Greater);
    test_sqrt_5_prec_round_helper(10, Down, "2.234", "0x2.3c#10", Less);
    test_sqrt_5_prec_round_helper(10, Up, "2.238", "0x2.3d#10", Greater);
    test_sqrt_5_prec_round_helper(10, Nearest, "2.234", "0x2.3c#10", Less);

    test_sqrt_5_prec_round_helper(
        100,
        Floor,
        "2.236067977499789696409173668728",
        "0x2.3c6ef372fe94f82be73980c08#100",
        Less,
    );
    test_sqrt_5_prec_round_helper(
        100,
        Ceiling,
        "2.236067977499789696409173668732",
        "0x2.3c6ef372fe94f82be73980c0c#100",
        Greater,
    );
    test_sqrt_5_prec_round_helper(
        100,
        Down,
        "2.236067977499789696409173668728",
        "0x2.3c6ef372fe94f82be73980c08#100",
        Less,
    );
    test_sqrt_5_prec_round_helper(
        100,
        Up,
        "2.236067977499789696409173668732",
        "0x2.3c6ef372fe94f82be73980c0c#100",
        Greater,
    );
    test_sqrt_5_prec_round_helper(
        100,
        Nearest,
        "2.236067977499789696409173668732",
        "0x2.3c6ef372fe94f82be73980c0c#100",
        Greater,
    );
}

#[test]
#[should_panic]
fn sqrt_5_prec_round_fail_1() {
    Float::sqrt_5_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn sqrt_5_prec_round_fail_2() {
    Float::sqrt_5_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn sqrt_5_prec_round_fail_3() {
    Float::sqrt_5_prec_round(1000, Exact);
}

#[test]
fn sqrt_5_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (sqrt_5, o) = Float::sqrt_5_prec(prec);
        assert!(sqrt_5.is_valid());
        assert_eq!(sqrt_5.get_prec(), Some(prec));
        assert_eq!(sqrt_5.get_exponent(), Some(2));
        assert_ne!(o, Equal);
        if o == Less {
            let (sqrt_5_alt, o_alt) = Float::sqrt_5_prec_round(prec, Ceiling);
            let mut next_upper = sqrt_5.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(sqrt_5_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !sqrt_5.is_power_of_2() {
            let (sqrt_5_alt, o_alt) = Float::sqrt_5_prec_round(prec, Floor);
            let mut next_lower = sqrt_5.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(sqrt_5_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
        let (sqrt_5_alt, o_alt) = Float::sqrt_5_prec_round(prec, Nearest);
        assert_eq!(ComparableFloatRef(&sqrt_5_alt), ComparableFloatRef(&sqrt_5));
        assert_eq!(o_alt, o);

        let (rug_sqrt_5, rug_o) =
            rug_sqrt_5_prec_round(prec, rug_round_try_from_rounding_mode(Nearest).unwrap());
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_sqrt_5)),
            ComparableFloatRef(&sqrt_5)
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn sqrt_5_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (sqrt_5, o) = Float::sqrt_5_prec_round(prec, rm);
        assert!(sqrt_5.is_valid());
        assert_eq!(sqrt_5.get_prec(), Some(prec));
        let expected_exponent = match (prec, rm) {
            (1, Ceiling | Up) => 3,
            _ => 2,
        };
        assert_eq!(sqrt_5.get_exponent(), Some(expected_exponent));
        assert_ne!(o, Equal);
        if o == Less {
            let (sqrt_5_alt, o_alt) = Float::sqrt_5_prec_round(prec, Ceiling);
            let mut next_upper = sqrt_5.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(sqrt_5_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !sqrt_5.is_power_of_2() {
            let (sqrt_5_alt, o_alt) = Float::sqrt_5_prec_round(prec, Floor);
            let mut next_lower = sqrt_5.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(sqrt_5_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_sqrt_5, rug_o) = rug_sqrt_5_prec_round(prec, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_sqrt_5)),
                ComparableFloatRef(&sqrt_5)
            );
            assert_eq!(rug_o, o);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::sqrt_5_prec_round(prec, Exact));
    });

    test_constant(Float::sqrt_5_prec_round, 10000);
}
