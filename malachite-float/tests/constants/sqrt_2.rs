// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::Sqrt2;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{
    rug_round_try_from_rounding_mode, test_constant, to_hex_string,
};
use malachite_float::test_util::constants::sqrt_2::rug_sqrt_2_prec_round;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_sqrt_2_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::sqrt_2_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (rug_x, rug_o) =
        rug_sqrt_2_prec_round(prec, rug_round_try_from_rounding_mode(Nearest).unwrap());
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_x)),
        ComparableFloatRef(&x)
    );
    assert_eq!(rug_o, o);
}

#[test]
pub fn test_sqrt_2_prec() {
    test_sqrt_2_prec_helper(1, "1.0", "0x1.0#1", Less);
    test_sqrt_2_prec_helper(2, "1.5", "0x1.8#2", Greater);
    test_sqrt_2_prec_helper(3, "1.5", "0x1.8#3", Greater);
    test_sqrt_2_prec_helper(4, "1.4", "0x1.6#4", Less);
    test_sqrt_2_prec_helper(5, "1.44", "0x1.7#5", Greater);
    test_sqrt_2_prec_helper(6, "1.41", "0x1.68#6", Less);
    test_sqrt_2_prec_helper(7, "1.42", "0x1.6c#7", Greater);
    test_sqrt_2_prec_helper(8, "1.414", "0x1.6a#8", Less);
    test_sqrt_2_prec_helper(9, "1.414", "0x1.6a#9", Less);
    test_sqrt_2_prec_helper(10, "1.414", "0x1.6a0#10", Less);
    test_sqrt_2_prec_helper(
        100,
        "1.414213562373095048801688724209",
        "0x1.6a09e667f3bcc908b2fb1366e#100",
        Less,
    );
    test_sqrt_2_prec_helper(
        1000,
        "1.414213562373095048801688724209698078569671875376948073176679737990732478462107038850387\
        534327641572735013846230912297024924836055850737212644121497099935831413222665927505592755\
        799950501152782060571470109559971605970274534596862014728517418640889198609552329230484308\
        7143214508397626036279952514079896",
        "0x1.6a09e667f3bcc908b2fb1366ea957d3e3adec17512775099da2f590b0667322a95f90608757145875163f\
        cdfb907b6721ee950bc8738f694f0090e6c7bf44ed1a4405d0e855e3e9ca60b38c0237866f7956379222d108b1\
        48c1578e45ef89c678dab5147176fd3b99654c68663e7909bea5e241f06dcb05dd549411320#1000",
        Less,
    );
    test_sqrt_2_prec_helper(
        10000,
        "1.414213562373095048801688724209698078569671875376948073176679737990732478462107038850387\
        534327641572735013846230912297024924836055850737212644121497099935831413222665927505592755\
        799950501152782060571470109559971605970274534596862014728517418640889198609552329230484308\
        714321450839762603627995251407989687253396546331808829640620615258352395054745750287759961\
        729835575220337531857011354374603408498847160386899970699004815030544027790316454247823068\
        492936918621580578463111596668713013015618568987237235288509264861249497715421833420428568\
        606014682472077143585487415565706967765372022648544701585880162075847492265722600208558446\
        652145839889394437092659180031138824646815708263010059485870400318648034219489727829064104\
        507263688131373985525611732204024509122770022694112757362728049573810896750401836986836845\
        072579936472906076299694138047565482372899718032680247442062926912485905218100445984215059\
        112024944134172853147810580360337107730918286931471017111168391658172688941975871658215212\
        822951848847208969463386289156288276595263514054226765323969461751129160240871551013515045\
        538128756005263146801712740265396947024030051749531886292563138518816347800156936917688185\
        237868405228783762938921430065586956868596459515550164472450983689603688732311438941557665\
        104088391429233811320605243362948531704991577175622854974143899918802176243096520656421182\
        731672625753959471725593463723863226148274262220867115583959992652117625269891754098815934\
        864008345708518147223181420407042650905653233339843645786579679651926729239987536661721598\
        257886026336361782749599421940377775368142621773879919455139723127406689832998989538672882\
        285637869774966251996658352577619893932284534473569479496295216889148549253890475582883452\
        609652409654288939453864662574492755638196441031697983306185201937938494005715633372054806\
        854057586799967012137223947582142630658513221740883238294728761739364746783743196000159218\
        880734785761725221186749042497736692920731109636972160893370866115673458533483329525467585\
        164471075784860246360083444911481858765555428645512331421992631133251797060843655970435285\
        641008791850076036100915946567067688360557174007675690509613671940132493560524018599910506\
        210816359772643138060546701029356997104242510578174953105725593498445112692278034491350663\
        756874776028316282960553242242695753452902883876844642917328277088831808702533985233812274\
        999081237189254072647536785030482159180188616710897286922920119759988070381854333253646021\
        108229927929307287178079988809917674177410898306080032631181642798823117154363869661702999\
        934161614878686018045505553986913115186010386375325004558186044804075024119518430567453368\
        361367459737442398855328517930896037389891517319587413442881784212502191695187559344438739\
        618931454999990610758704909026088351763622474975785885836803745793115733980209998662218694\
        992259591327642361941059210032802614987456659968887406795616739185957288864247346358588686\
        449682238600698335264279905628316561391394255764906206518602164726303336297507569787060660\
        6856498160092718709292153132368281356988938",
        "0x1.6a09e667f3bcc908b2fb1366ea957d3e3adec17512775099da2f590b0667322a95f90608757145875163f\
        cdfb907b6721ee950bc8738f694f0090e6c7bf44ed1a4405d0e855e3e9ca60b38c0237866f7956379222d108b1\
        48c1578e45ef89c678dab5147176fd3b99654c68663e7909bea5e241f06dcb05dd5494113208194950272956db\
        1fa1dfbe9a74059d7927c1884c9b579aa516ca3719e6836df046d8e0209b803fc646a5e6654bd3ef7b43d7fed4\
        37c7f9444260fbd40c483ef55038583f97bbd45efb8663107145d5febe765a49e94ec7f597105fbfc2e1fa763e\
        f01f3599c82f2fe500b848cf0bd252ae046bf9f1ef7947d46769af8c14bcc67c7c290be76929b0578c10b584fb\
        487c924f5b71f82dcd2903609dee8912983d4eaad0eea321f7489f46a7e9030be20fb7694efb58c9984cdd70a1\
        da9045c3d133a068423d6e38303d901ba9da3476684796c5cd5972dc0ff3540c3412942d6406101ef6fc6de911\
        4a2b4f248c689c600bb40a8b56b041fd5de6e0dd0c66d4831fe7fff5757e4710980cdbd5c268485da5e91b3e2f\
        205b72725b971d60a1f888f08a0a6e100ccedc2ce5bd98aee71e42e268d37a6072f220234613ffc22453439ea9\
        7a999b6c9e3ce71f94d6092ace120ab8e550e0d5511688631778cf60350d02fe85f29ec8be5c72b807af5771b8\
        25b30a0e78376a91c08c6a7f0f8f323b36281d225689c0b5a82047db989f63a8a64e8519bc0d0c1e22280484d9\
        4f4f9bb3d4b31d489d75231b5c633c480c96be549bf5d96678b4d2c4dda867bd8e48029fea8c8173567c2ba3d3\
        ce9dfe0bd1b4dd771178057b695b7eaf1b05c22c8d5feebd077fec96db8f778fc1c2bbbce1b49ebf5af4460882\
        958add01ca7f1b6bc0b7ec1bc6e0a6edbc67f85b274e0861b3a137571b16549873d211c6aae69d801e579445bc\
        60a3e0a4fd8968eb794bdc702d6945da94b04a440cabc94387c3d26fd0f3be8af6305a53a177288aca13fb406c\
        982915d83ba0d3558d81dd1159e9643eaa27eb7757a2052975b6f4a889e3d092bb1c685480dc2e99947b372ded\
        a05e2192f95b1b926512b404c33181d64a359a89b6f8864f3d575319359aa386257a93a57a5977547fa0f606cc\
        32eea84ef7103d2d2d57b6c153a5f37c2c77fe2928d321b29470eae4158b3cedc64ade1e431a138bb7be7305b4\
        6f7fe0bc754ff3ec042bdc85b325996522a87b8643e3d17870f8c25b5978060d55b82fd4ed3f3e15892b5b5236\
        182cb2831f44ca27ad1a66fd56fdc29a7c7a93e4279aa12d460a0d49495b6be4dc73ddf96ee80a9dfc22d8d385\
        ea2ecc801d740c1990c7bf28458480527bb8ba8109d2770c3ffde121a7be434f83d0aabfbde531f74bafdbc2da\
        fbece02e65bb77b8fbb8cdb4ad8e4b9ef01e2c90f8069d23ecfb0c8e2e27d647daa43762a64b9c7d745344fdb5\
        16fc56fa1eae6a95874fb501f69b17b66b81dfba11ed52415d7ee3e53f9585946bb2081617805996805bb4f9da\
        796f633c9d13f6c172c3afe3401a32e99dd6a247e279600a3f5c09991a6554a35000ffe090abfbc2d8a4a88145\
        9ed127a54ccecb63cf6373a711c9cf91566c397b28278a7f0bb8bbc4516ddcab0f6e9557448f93eeb80f69f34b\
        525c760f3380ef9edb8aceefa8db3c7ce26e42aa5984d4db95612ec528a245fcf43db3757f0a0a1fc24fe218f4\
        b630720904044b232f38c4032f1c3ab8971dd4f7966dd97614bae8d797a0848b1d5cfdb137e#10000",
        Greater,
    );

    let sqrt_2_f32 = Float::sqrt_2_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(sqrt_2_f32.to_string(), "1.4142135");
    assert_eq!(to_hex_string(&sqrt_2_f32), "0x1.6a09e6#24");
    assert_eq!(sqrt_2_f32, f32::SQRT_2);

    let sqrt_2_f64 = Float::sqrt_2_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(sqrt_2_f64.to_string(), "1.4142135623730951");
    assert_eq!(to_hex_string(&sqrt_2_f64), "0x1.6a09e667f3bcd#53");
    assert_eq!(sqrt_2_f64, f64::SQRT_2);
}

fn test_sqrt_2_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::sqrt_2_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_x, rug_o) = rug_sqrt_2_prec_round(prec, rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_x)),
            ComparableFloatRef(&x)
        );
        assert_eq!(rug_o, o);
    }
}

#[test]
pub fn test_sqrt_2_prec_round() {
    test_sqrt_2_prec_round_helper(1, Floor, "1.0", "0x1.0#1", Less);
    test_sqrt_2_prec_round_helper(1, Ceiling, "2.0", "0x2.0#1", Greater);
    test_sqrt_2_prec_round_helper(1, Down, "1.0", "0x1.0#1", Less);
    test_sqrt_2_prec_round_helper(1, Up, "2.0", "0x2.0#1", Greater);
    test_sqrt_2_prec_round_helper(1, Nearest, "1.0", "0x1.0#1", Less);

    test_sqrt_2_prec_round_helper(2, Floor, "1.0", "0x1.0#2", Less);
    test_sqrt_2_prec_round_helper(2, Ceiling, "1.5", "0x1.8#2", Greater);
    test_sqrt_2_prec_round_helper(2, Down, "1.0", "0x1.0#2", Less);
    test_sqrt_2_prec_round_helper(2, Up, "1.5", "0x1.8#2", Greater);
    test_sqrt_2_prec_round_helper(2, Nearest, "1.5", "0x1.8#2", Greater);

    test_sqrt_2_prec_round_helper(3, Floor, "1.2", "0x1.4#3", Less);
    test_sqrt_2_prec_round_helper(3, Ceiling, "1.5", "0x1.8#3", Greater);
    test_sqrt_2_prec_round_helper(3, Down, "1.2", "0x1.4#3", Less);
    test_sqrt_2_prec_round_helper(3, Up, "1.5", "0x1.8#3", Greater);
    test_sqrt_2_prec_round_helper(3, Nearest, "1.5", "0x1.8#3", Greater);

    test_sqrt_2_prec_round_helper(4, Floor, "1.4", "0x1.6#4", Less);
    test_sqrt_2_prec_round_helper(4, Ceiling, "1.5", "0x1.8#4", Greater);
    test_sqrt_2_prec_round_helper(4, Down, "1.4", "0x1.6#4", Less);
    test_sqrt_2_prec_round_helper(4, Up, "1.5", "0x1.8#4", Greater);
    test_sqrt_2_prec_round_helper(4, Nearest, "1.4", "0x1.6#4", Less);

    test_sqrt_2_prec_round_helper(5, Floor, "1.38", "0x1.6#5", Less);
    test_sqrt_2_prec_round_helper(5, Ceiling, "1.44", "0x1.7#5", Greater);
    test_sqrt_2_prec_round_helper(5, Down, "1.38", "0x1.6#5", Less);
    test_sqrt_2_prec_round_helper(5, Up, "1.44", "0x1.7#5", Greater);
    test_sqrt_2_prec_round_helper(5, Nearest, "1.44", "0x1.7#5", Greater);

    test_sqrt_2_prec_round_helper(6, Floor, "1.41", "0x1.68#6", Less);
    test_sqrt_2_prec_round_helper(6, Ceiling, "1.44", "0x1.70#6", Greater);
    test_sqrt_2_prec_round_helper(6, Down, "1.41", "0x1.68#6", Less);
    test_sqrt_2_prec_round_helper(6, Up, "1.44", "0x1.70#6", Greater);
    test_sqrt_2_prec_round_helper(6, Nearest, "1.41", "0x1.68#6", Less);

    test_sqrt_2_prec_round_helper(7, Floor, "1.41", "0x1.68#7", Less);
    test_sqrt_2_prec_round_helper(7, Ceiling, "1.42", "0x1.6c#7", Greater);
    test_sqrt_2_prec_round_helper(7, Down, "1.41", "0x1.68#7", Less);
    test_sqrt_2_prec_round_helper(7, Up, "1.42", "0x1.6c#7", Greater);
    test_sqrt_2_prec_round_helper(7, Nearest, "1.42", "0x1.6c#7", Greater);

    test_sqrt_2_prec_round_helper(8, Floor, "1.414", "0x1.6a#8", Less);
    test_sqrt_2_prec_round_helper(8, Ceiling, "1.42", "0x1.6c#8", Greater);
    test_sqrt_2_prec_round_helper(8, Down, "1.414", "0x1.6a#8", Less);
    test_sqrt_2_prec_round_helper(8, Up, "1.42", "0x1.6c#8", Greater);
    test_sqrt_2_prec_round_helper(8, Nearest, "1.414", "0x1.6a#8", Less);

    test_sqrt_2_prec_round_helper(9, Floor, "1.414", "0x1.6a#9", Less);
    test_sqrt_2_prec_round_helper(9, Ceiling, "1.418", "0x1.6b#9", Greater);
    test_sqrt_2_prec_round_helper(9, Down, "1.414", "0x1.6a#9", Less);
    test_sqrt_2_prec_round_helper(9, Up, "1.418", "0x1.6b#9", Greater);
    test_sqrt_2_prec_round_helper(9, Nearest, "1.414", "0x1.6a#9", Less);

    test_sqrt_2_prec_round_helper(10, Floor, "1.414", "0x1.6a0#10", Less);
    test_sqrt_2_prec_round_helper(10, Ceiling, "1.416", "0x1.6a8#10", Greater);
    test_sqrt_2_prec_round_helper(10, Down, "1.414", "0x1.6a0#10", Less);
    test_sqrt_2_prec_round_helper(10, Up, "1.416", "0x1.6a8#10", Greater);
    test_sqrt_2_prec_round_helper(10, Nearest, "1.414", "0x1.6a0#10", Less);

    test_sqrt_2_prec_round_helper(
        100,
        Floor,
        "1.414213562373095048801688724209",
        "0x1.6a09e667f3bcc908b2fb1366e#100",
        Less,
    );
    test_sqrt_2_prec_round_helper(
        100,
        Ceiling,
        "1.414213562373095048801688724211",
        "0x1.6a09e667f3bcc908b2fb13670#100",
        Greater,
    );
    test_sqrt_2_prec_round_helper(
        100,
        Down,
        "1.414213562373095048801688724209",
        "0x1.6a09e667f3bcc908b2fb1366e#100",
        Less,
    );
    test_sqrt_2_prec_round_helper(
        100,
        Up,
        "1.414213562373095048801688724211",
        "0x1.6a09e667f3bcc908b2fb13670#100",
        Greater,
    );
    test_sqrt_2_prec_round_helper(
        100,
        Nearest,
        "1.414213562373095048801688724209",
        "0x1.6a09e667f3bcc908b2fb1366e#100",
        Less,
    );
}

#[test]
#[should_panic]
fn sqrt_2_prec_round_fail_1() {
    Float::sqrt_2_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn sqrt_2_prec_round_fail_2() {
    Float::sqrt_2_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn sqrt_2_prec_round_fail_3() {
    Float::sqrt_2_prec_round(1000, Exact);
}

#[test]
fn sqrt_2_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (sqrt_2, o) = Float::sqrt_2_prec(prec);
        assert!(sqrt_2.is_valid());
        assert_eq!(sqrt_2.get_prec(), Some(prec));
        assert_eq!(sqrt_2.get_exponent(), Some(1));
        assert_ne!(o, Equal);
        if o == Less {
            let (sqrt_2_alt, o_alt) = Float::sqrt_2_prec_round(prec, Ceiling);
            let mut next_upper = sqrt_2.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(sqrt_2_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !sqrt_2.is_power_of_2() {
            let (sqrt_2_alt, o_alt) = Float::sqrt_2_prec_round(prec, Floor);
            let mut next_lower = sqrt_2.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(sqrt_2_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
        let (sqrt_2_alt, o_alt) = Float::sqrt_2_prec_round(prec, Nearest);
        assert_eq!(ComparableFloatRef(&sqrt_2_alt), ComparableFloatRef(&sqrt_2));
        assert_eq!(o_alt, o);

        let (rug_sqrt_2, rug_o) =
            rug_sqrt_2_prec_round(prec, rug_round_try_from_rounding_mode(Nearest).unwrap());
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_sqrt_2)),
            ComparableFloatRef(&sqrt_2)
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn sqrt_2_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (sqrt_2, o) = Float::sqrt_2_prec_round(prec, rm);
        assert!(sqrt_2.is_valid());
        assert_eq!(sqrt_2.get_prec(), Some(prec));
        assert_eq!(
            sqrt_2.get_exponent(),
            Some(if prec == 1 && (rm == Ceiling || rm == Up) {
                2
            } else {
                1
            })
        );
        assert_ne!(o, Equal);
        if o == Less {
            let (sqrt_2_alt, o_alt) = Float::sqrt_2_prec_round(prec, Ceiling);
            let mut next_upper = sqrt_2.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(sqrt_2_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !sqrt_2.is_power_of_2() {
            let (sqrt_2_alt, o_alt) = Float::sqrt_2_prec_round(prec, Floor);
            let mut next_lower = sqrt_2.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(sqrt_2_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_sqrt_2, rug_o) = rug_sqrt_2_prec_round(prec, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_sqrt_2)),
                ComparableFloatRef(&sqrt_2)
            );
            assert_eq!(rug_o, o);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::sqrt_2_prec_round(prec, Exact));
    });

    test_constant(Float::sqrt_2_prec_round, 10000);
}
