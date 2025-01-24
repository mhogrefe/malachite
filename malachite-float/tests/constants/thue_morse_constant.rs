// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::ThueMorseConstant;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::to_hex_string;
use malachite_float::test_util::constants::thue_morse_constant::*;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_thue_morse_constant_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::thue_morse_constant_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = thue_morse_constant_prec_round_naive(prec, Nearest);
    assert_eq!(x, x_alt);
    assert_eq!(o, o_alt);
}

#[test]
pub fn test_thue_morse_constant_prec() {
    test_thue_morse_constant_prec_helper(1, "0.5", "0x0.8#1", Greater);
    test_thue_morse_constant_prec_helper(2, "0.4", "0x0.6#2", Less);
    test_thue_morse_constant_prec_helper(3, "0.44", "0x0.7#3", Greater);
    test_thue_morse_constant_prec_helper(4, "0.41", "0x0.68#4", Less);
    test_thue_morse_constant_prec_helper(5, "0.41", "0x0.68#5", Less);
    test_thue_morse_constant_prec_helper(6, "0.414", "0x0.6a#6", Greater);
    test_thue_morse_constant_prec_helper(7, "0.414", "0x0.6a#7", Greater);
    test_thue_morse_constant_prec_helper(8, "0.412", "0x0.698#8", Less);
    test_thue_morse_constant_prec_helper(9, "0.412", "0x0.698#9", Less);
    test_thue_morse_constant_prec_helper(10, "0.4126", "0x0.69a#10", Greater);
    test_thue_morse_constant_prec_helper(
        100,
        "0.4124540336401075977833613682584",
        "0x0.69969669966969969669699668#100",
        Less,
    );
    test_thue_morse_constant_prec_helper(
        1000,
        "0.412454033640107597783361368258455283089478374455769557573379415348793592365782588963804\
        540486212133396256366446538137487421617125193606520130996812525914093898811685058241405148\
        794923736837717681632072372543961648455116929274915967121308797839015745939416763942926546\
        99576354558364476401478446511510844",
        "0x0.6996966996696996966969966996966996696996699696696996966996696996966969966996966969969\
        669966969966996966996696996966969966996966996696996699696696996966996696996699696699669699\
        6966969966996966969969669966969969669699669969669966969966996966969969669968#1000",
        Greater,
    );
    test_thue_morse_constant_prec_helper(
        10000,
        "0.412454033640107597783361368258455283089478374455769557573379415348793592365782588963804\
        540486212133396256366446538137487421617125193606520130996812525914093898811685058241405148\
        794923736837717681632072372543961648455116929274915967121308797839015745939416763942926546\
        995763545583644764014784465115108432988880527862797583778966429806153625600892794767509570\
        613156170981916634767556262265886591706560103745429025080980592282684200382286637025552138\
        072551019786013560720441271576469980054213089589582092326366528534417259882166080658990995\
        454185078647844640598822656110795985657599962216385510031350572070562361426971255128326932\
        200511472227565707730468866881043116266791929041303471097804134296620336523301148618999636\
        607737503684384287533124783775524067880628087063676089104069872257951121771322589484343969\
        687095887155500338806472758969301236596955913500219809107121878986270983191365987288252181\
        126158494661750104956413082261728325258668242137155812289887905859814150448158087621806081\
        262788059473203194858059514889641770915367577963421317521615653898873066742051436896862845\
        559388487896091534788479920439498978693239273118810516671742331317074845718885491752000298\
        494549886494536473002915644233647450149747441404205378363056251376268024177778124482615212\
        986239829412055649329758982540912039441942593759895057769683431420175890362746422845117500\
        028864842247279148167611365866356168475145086817445555637346709003759617604542400457827717\
        878660619366846815483941374378957060770110880180541318234323659237369552401312033505621806\
        489384014183232923162046501177883641270956312786606124433156307628120044554491098561964906\
        941642998546962635980094725095337754362574897392106593547505034744530716373746139055913844\
        730509118321844885047459136295028893709649115890738042446256488167921608092525231651808551\
        275515631923236859178416003062357823567287388396791613593690713019818632422297617615418611\
        504665052818584325511618352200787725967763734519804573074925771880268729695759436794061431\
        099083941098658903463402394219222397121260948820568428682987584463231060168451271023138827\
        618185738550975032215601211268731359671002623792857579075515885840571314634970190214029027\
        767535211043320712596064063934151016649744328535127842648397490736292971851028934173894800\
        896326950620264219873205858953858503888863733655930190565936770347310031381812711297640523\
        854923089574146136921130192885320184597064184712813990427490161972158454924359809839110934\
        222905109312597374946863138945391555541320741472882457682612774489682755147712902066198766\
        365305692386197821993411562093365185277357462610197541359244519518547852573339123003451159\
        324066183679763125504488718106276971066142222729525177728792136200206791458838091045048890\
        889263477842031146946636054226135511708407225571080034132437244605493617364337010772961383\
        884632019459005730120007550129264229222419321684701101892904472852138411474750231020219380\
        490568164815194679503954588116034274794607988054271917142826665676154809041554888257037645\
        08626387325662901457682774568054766955037516",
        "0x0.6996966996696996966969966996966996696996699696696996966996696996966969966996966969969\
        669966969966996966996696996966969966996966996696996699696696996966996696996699696699669699\
        696696996699696696996966996696996966969966996966996696996699696696996966996696996966969966\
        996966969969669966969966996966996696996966969966996966969969669966969969669699669969669966\
        969966996966969969669966969966996966996696996966969966996966996696996699696696996966996696\
        996966969966996966969969669966969966996966996696996966969966996966996696996699696696996966\
        996696996699696699669699696696996699696696996966996696996966969966996966996696996699696696\
        996966996696996699696699669699696696996699696699669699669969669699696699669699696696996699\
        696696996966996696996699696699669699696696996699696696996966996696996966969966996966996696\
        996699696696996966996696996966969966996966969969669966969966996966996696996966969966996966\
        996696996699696696996966996696996699696699669699696696996699696696996966996696996966969966\
        996966996696996699696696996966996696996966969966996966969969669966969966996966996696996966\
        969966996966969969669966969969669699669969669966969966996966969969669966969966996966996696\
        996966969966996966996696996699696696996966996696996966969966996966969969669966969966996966\
        996696996966969966996966969969669966969969669699669969669966969966996966969969669966969969\
        669699669969669699696699669699669969669966969969669699669969669966969966996966969969669966\
        969966996966996696996966969966996966969969669966969969669699669969669966969966996966969969\
        669966969966996966996696996966969966996966996696996699696696996966996696996966969966996966\
        969969669966969966996966996696996966969966996966996696996699696696996966996696996699696699\
        669699696696996699696696996966996696996966969966996966996696996699696696996966996696996966\
        969966996966969969669966969966996966996696996966969966996966969969669966969969669699669969\
        669966969966996966969969669966969966996966996696996966969966996966996696996699696696996966\
        996696996966969966996966969969669966969966996966996696996966969966996966996696996699696696\
        996966996696996699696699669699696696996699696696996966996696996966969966996966996696996699\
        696696996966996696996699696699669699696696996699696699669699669969669699696699669699696696\
        996699696696996966996696996699696699669699696696996699696696996966996696996966969966996966\
        996696996699696696996966996696996966969966996966969969669966969966996966996696996966969966\
        9969669966969966996966969969669966969966996966996696996966969966996966969968#10000",
        Less,
    );

    let tmc_f32 = Float::thue_morse_constant_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(tmc_f32.to_string(), "0.41245404");
    assert_eq!(to_hex_string(&tmc_f32), "0x0.6996968#24");
    assert_eq!(tmc_f32, f32::THUE_MORSE_CONSTANT);

    let tmc_f64 = Float::thue_morse_constant_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(tmc_f64.to_string(), "0.41245403364010758");
    assert_eq!(to_hex_string(&tmc_f64), "0x0.69969669966968#53");
    assert_eq!(tmc_f64, f64::THUE_MORSE_CONSTANT);
}

fn test_thue_morse_constant_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::thue_morse_constant_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = thue_morse_constant_prec_round_naive(prec, rm);
    assert_eq!(x, x_alt);
    assert_eq!(o, o_alt);
}

#[test]
pub fn test_thue_morse_constant_prec_round() {
    test_thue_morse_constant_prec_round_helper(1, Floor, "0.2", "0x0.4#1", Less);
    test_thue_morse_constant_prec_round_helper(1, Ceiling, "0.5", "0x0.8#1", Greater);
    test_thue_morse_constant_prec_round_helper(1, Down, "0.2", "0x0.4#1", Less);
    test_thue_morse_constant_prec_round_helper(1, Up, "0.5", "0x0.8#1", Greater);
    test_thue_morse_constant_prec_round_helper(1, Nearest, "0.5", "0x0.8#1", Greater);

    test_thue_morse_constant_prec_round_helper(2, Floor, "0.4", "0x0.6#2", Less);
    test_thue_morse_constant_prec_round_helper(2, Ceiling, "0.5", "0x0.8#2", Greater);
    test_thue_morse_constant_prec_round_helper(2, Down, "0.4", "0x0.6#2", Less);
    test_thue_morse_constant_prec_round_helper(2, Up, "0.5", "0x0.8#2", Greater);
    test_thue_morse_constant_prec_round_helper(2, Nearest, "0.4", "0x0.6#2", Less);

    test_thue_morse_constant_prec_round_helper(3, Floor, "0.38", "0x0.6#3", Less);
    test_thue_morse_constant_prec_round_helper(3, Ceiling, "0.44", "0x0.7#3", Greater);
    test_thue_morse_constant_prec_round_helper(3, Down, "0.38", "0x0.6#3", Less);
    test_thue_morse_constant_prec_round_helper(3, Up, "0.44", "0x0.7#3", Greater);
    test_thue_morse_constant_prec_round_helper(3, Nearest, "0.44", "0x0.7#3", Greater);

    test_thue_morse_constant_prec_round_helper(4, Floor, "0.41", "0x0.68#4", Less);
    test_thue_morse_constant_prec_round_helper(4, Ceiling, "0.44", "0x0.70#4", Greater);
    test_thue_morse_constant_prec_round_helper(4, Down, "0.41", "0x0.68#4", Less);
    test_thue_morse_constant_prec_round_helper(4, Up, "0.44", "0x0.70#4", Greater);
    test_thue_morse_constant_prec_round_helper(4, Nearest, "0.41", "0x0.68#4", Less);

    test_thue_morse_constant_prec_round_helper(5, Floor, "0.41", "0x0.68#5", Less);
    test_thue_morse_constant_prec_round_helper(5, Ceiling, "0.42", "0x0.6c#5", Greater);
    test_thue_morse_constant_prec_round_helper(5, Down, "0.41", "0x0.68#5", Less);
    test_thue_morse_constant_prec_round_helper(5, Up, "0.42", "0x0.6c#5", Greater);
    test_thue_morse_constant_prec_round_helper(5, Nearest, "0.41", "0x0.68#5", Less);

    test_thue_morse_constant_prec_round_helper(6, Floor, "0.406", "0x0.68#6", Less);
    test_thue_morse_constant_prec_round_helper(6, Ceiling, "0.414", "0x0.6a#6", Greater);
    test_thue_morse_constant_prec_round_helper(6, Down, "0.406", "0x0.68#6", Less);
    test_thue_morse_constant_prec_round_helper(6, Up, "0.414", "0x0.6a#6", Greater);
    test_thue_morse_constant_prec_round_helper(6, Nearest, "0.414", "0x0.6a#6", Greater);

    test_thue_morse_constant_prec_round_helper(7, Floor, "0.41", "0x0.69#7", Less);
    test_thue_morse_constant_prec_round_helper(7, Ceiling, "0.414", "0x0.6a#7", Greater);
    test_thue_morse_constant_prec_round_helper(7, Down, "0.41", "0x0.69#7", Less);
    test_thue_morse_constant_prec_round_helper(7, Up, "0.414", "0x0.6a#7", Greater);
    test_thue_morse_constant_prec_round_helper(7, Nearest, "0.414", "0x0.6a#7", Greater);

    test_thue_morse_constant_prec_round_helper(8, Floor, "0.412", "0x0.698#8", Less);
    test_thue_morse_constant_prec_round_helper(8, Ceiling, "0.414", "0x0.6a0#8", Greater);
    test_thue_morse_constant_prec_round_helper(8, Down, "0.412", "0x0.698#8", Less);
    test_thue_morse_constant_prec_round_helper(8, Up, "0.414", "0x0.6a0#8", Greater);
    test_thue_morse_constant_prec_round_helper(8, Nearest, "0.412", "0x0.698#8", Less);

    test_thue_morse_constant_prec_round_helper(9, Floor, "0.412", "0x0.698#9", Less);
    test_thue_morse_constant_prec_round_helper(9, Ceiling, "0.413", "0x0.69c#9", Greater);
    test_thue_morse_constant_prec_round_helper(9, Down, "0.412", "0x0.698#9", Less);
    test_thue_morse_constant_prec_round_helper(9, Up, "0.413", "0x0.69c#9", Greater);
    test_thue_morse_constant_prec_round_helper(9, Nearest, "0.412", "0x0.698#9", Less);

    test_thue_morse_constant_prec_round_helper(10, Floor, "0.4121", "0x0.698#10", Less);
    test_thue_morse_constant_prec_round_helper(10, Ceiling, "0.4126", "0x0.69a#10", Greater);
    test_thue_morse_constant_prec_round_helper(10, Down, "0.4121", "0x0.698#10", Less);
    test_thue_morse_constant_prec_round_helper(10, Up, "0.4126", "0x0.69a#10", Greater);
    test_thue_morse_constant_prec_round_helper(10, Nearest, "0.4126", "0x0.69a#10", Greater);

    test_thue_morse_constant_prec_round_helper(
        100,
        Floor,
        "0.4124540336401075977833613682584",
        "0x0.69969669966969969669699668#100",
        Less,
    );
    test_thue_morse_constant_prec_round_helper(
        100,
        Ceiling,
        "0.4124540336401075977833613682588",
        "0x0.69969669966969969669699670#100",
        Greater,
    );
    test_thue_morse_constant_prec_round_helper(
        100,
        Down,
        "0.4124540336401075977833613682584",
        "0x0.69969669966969969669699668#100",
        Less,
    );
    test_thue_morse_constant_prec_round_helper(
        100,
        Up,
        "0.4124540336401075977833613682588",
        "0x0.69969669966969969669699670#100",
        Greater,
    );
    test_thue_morse_constant_prec_round_helper(
        100,
        Nearest,
        "0.4124540336401075977833613682584",
        "0x0.69969669966969969669699668#100",
        Less,
    );
}

#[test]
#[should_panic]
fn thue_morse_constant_prec_round_fail_1() {
    Float::thue_morse_constant_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn thue_morse_constant_prec_round_fail_2() {
    Float::thue_morse_constant_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn thue_morse_constant_prec_round_fail_3() {
    Float::thue_morse_constant_prec_round(1000, Exact);
}

#[test]
fn thue_morse_constant_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (tmc, o) = Float::thue_morse_constant_prec(prec);
        assert!(tmc.is_valid());
        assert_eq!(tmc.get_prec(), Some(prec));
        assert_ne!(o, Equal);
        if o == Less {
            let (tmc_alt, o_alt) = Float::thue_morse_constant_prec_round(prec, Ceiling);
            let mut next_upper = tmc.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(tmc_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !tmc.is_power_of_2() {
            let (tmc_alt, o_alt) = Float::thue_morse_constant_prec_round(prec, Floor);
            let mut next_lower = tmc.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(tmc_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
        let (tmc_alt, o_alt) = Float::thue_morse_constant_prec_round(prec, Nearest);
        assert_eq!(ComparableFloatRef(&tmc_alt), ComparableFloatRef(&tmc));
        assert_eq!(o_alt, o);

        let (tmc_alt, o_alt) = thue_morse_constant_prec_round_naive(prec, Nearest);
        assert_eq!(tmc, tmc_alt);
        assert_eq!(o, o_alt);
    });
}

#[test]
fn thue_morse_constant_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (tmc, o) = Float::thue_morse_constant_prec_round(prec, rm);
        assert!(tmc.is_valid());
        assert_eq!(tmc.get_prec(), Some(prec));
        assert_ne!(o, Equal);
        if o == Less {
            let (tmc_alt, o_alt) = Float::thue_morse_constant_prec_round(prec, Ceiling);
            let mut next_upper = tmc.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(tmc_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !tmc.is_power_of_2() {
            let (tmc_alt, o_alt) = Float::thue_morse_constant_prec_round(prec, Floor);
            let mut next_lower = tmc.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(tmc_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }

        let (tmc_alt, o_alt) = thue_morse_constant_prec_round_naive(prec, rm);
        assert_eq!(tmc, tmc_alt);
        assert_eq!(o, o_alt);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::thue_morse_constant_prec_round(prec, Exact));
    });
}
