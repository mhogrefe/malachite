// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::OneOverPi;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{test_constant, to_hex_string};
use malachite_float::test_util::constants::one_over_pi::one_over_pi_prec_round_simple;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_one_over_pi_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::one_over_pi_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = one_over_pi_prec_round_simple(prec, Nearest);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);
}

#[test]
pub fn test_one_over_pi_prec() {
    test_one_over_pi_prec_helper(1, "0.2", "0x0.4#1", Less);
    test_one_over_pi_prec_helper(2, "0.4", "0x0.6#2", Greater);
    test_one_over_pi_prec_helper(3, "0.3", "0x0.5#3", Less);
    test_one_over_pi_prec_helper(4, "0.31", "0x0.50#4", Less);
    test_one_over_pi_prec_helper(5, "0.31", "0x0.50#5", Less);
    test_one_over_pi_prec_helper(6, "0.32", "0x0.52#6", Greater);
    test_one_over_pi_prec_helper(7, "0.316", "0x0.51#7", Less);
    test_one_over_pi_prec_helper(8, "0.318", "0x0.518#8", Greater);
    test_one_over_pi_prec_helper(9, "0.318", "0x0.518#9", Greater);
    test_one_over_pi_prec_helper(10, "0.3184", "0x0.518#10", Greater);
    test_one_over_pi_prec_helper(
        100,
        "0.3183098861837906715377675267449",
        "0x0.517cc1b727220a94fe13abe8f8#100",
        Less,
    );
    test_one_over_pi_prec_helper(
        1000,
        "0.318309886183790671537767526745028724068919291480912897495334688117793595268453070180227\
        605532506171912145685453515916073785823692229157305755934821463399678458479933874818155146\
        155492793850615377434785792434795323386724780483447258023664760228445399511431880923780173\
        80534791224097882187387568817105745",
        "0x0.517cc1b727220a94fe13abe8fa9a6ee06db14acc9e21c820ff28b1d5ef5de2b0db92371d2126e97003249\
        77504e8c90e7f0ef58e5894d39f74411afa975da24274ce38135a2fbf209cc8eb1cc1a99cfa4e422fc5defc941\
        d8ffc4bffef02cc07f79788c5ad05368fb69b3f6793e584dba7a31fb34f2ff516ba93dd63f60#1000",
        Greater,
    );
    test_one_over_pi_prec_helper(
        10000,
        "0.318309886183790671537767526745028724068919291480912897495334688117793595268453070180227\
        605532506171912145685453515916073785823692229157305755934821463399678458479933874818155146\
        155492793850615377434785792434795323386724780483447258023664760228445399511431880923780173\
        805347912240978821873875688171057446199892886800497344695478919221796646193566149812333972\
        925609398897304375763149573133928482077991748278697219967736198399924885751170342357716862\
        235037534321093095073976019478920729518667536118604988993270610654313551006440649555632794\
        332045893496239196331681212033606071996267823974997665573308870559510140032481355128777699\
        142621760244398752295362755529475781266136092915956963522624854628139921550049000595519714\
        178113805593570263050420032635492041849623212481122912406292968178496918382870423150815112\
        401743053213604434318281514949165445195492570799750310658781627963544818716509594146657438\
        081399951815315415698694078717965617434685128073379023325091411886655262537300052245435942\
        306422519900877335890075251121672634233905195162564498832466686290212247073757126227273384\
        334284139493920258501156672106239217189019679113437419909493020863247631035161678885959941\
        999010508775132258891766613692101570583030282080978597701277632155239398614682077999157383\
        781196187475544123750864454378602732510522477560775077762213628135308681656557053866853599\
        112141580772120705477992490251991498552594047188191168602329659282371155424811508898914043\
        579539584818980654589540433299207130636307088007681379749435383177526381933013928809553941\
        375367313556209559590900706791516603763677375875532249629906119931160438167197502070254258\
        086463160997439373755518931326924420684088817109957007585477388587073238755658574718756869\
        406460474291675847114237272683858920366364583928330017566158662706995581994917298580534901\
        219787378189176610067406107610946246431618863953520645662628379619499644876670348713979695\
        002079001367760079573447199216048005478021749909709575847136522279897806537994854166992229\
        841657807553569486071009136912167342958616913446654070970785112404173678648199124423506636\
        788041941587141549930997617372132721937323934074949084205662438503692449669982322299133112\
        075939352279862565992155216555980201566072004676545975817080477523114890861852023820108675\
        996778093098424965903214145706010454420472035046626346359518622100656310218747827279290611\
        585214360167235909753449291960947954584896218401874251573836665791772567980871737333279513\
        468902819007274654383486227761327661451846055194690121096425556074130675566064341975469933\
        136981600653977013483582929367016563233706628672321461990299706239639467516888419683311908\
        304501286788625728880767767123017595432900341294135037549121183217433715715878452469512663\
        422265997311883193781439701377488460115038395411380076436785124406774847072513616708313034\
        594217623443591873665129770374999497417106233196612202784289089203229769055405022822140497\
        043479490207733472807745720199349786347123624142348095779873777311384615697004611411428812\
        72772640473704021482011497184562231443936007",
        "0x0.517cc1b727220a94fe13abe8fa9a6ee06db14acc9e21c820ff28b1d5ef5de2b0db92371d2126e97003249\
        77504e8c90e7f0ef58e5894d39f74411afa975da24274ce38135a2fbf209cc8eb1cc1a99cfa4e422fc5defc941\
        d8ffc4bffef02cc07f79788c5ad05368fb69b3f6793e584dba7a31fb34f2ff516ba93dd63f5f2f8bd9e839cfbc\
        529497535fdafd88fc6ae842b0198237e3db5d5f867de104d7a1b0ed4f1c8b0af730d8432ccc2af8a50342046f\
        fec4026b9939883030aab6539d464b0713de04635a3e20ce1b3e6ee74049541ace23b45cb0e536ed7a268ab8c8\
        29f52ff83829fbf19f419616f27cc193edde19e9377b58f2f7c4f9d0f9ae5793f8ec3f890c83e3e12357d376ab\
        b9698219d8ae30a5ace8ce1e16256a0a6962e8006233ec316b8f1cd634d803119be695a4bd3da6aaa9bfb1f6b8\
        c0851fe3b26954eb255ebb87c3e31abd83d738a8bab24e06ceb1d9c4253e591923bc56b11aa2d5c8f800d8578e\
        fe70cff98cfb50f3330abcca3fdd66c3fbf5bb29144f419305ff366e277849b366a1faeebef0b6f1dac494def1\
        4116974431426ac711965630b718465bef028600bd38ef9adf00c1a099731094180a441adc77abfd856f9748f2\
        1a52469b3886c6ed5212fd76730b55214055a4ce9f953033fbbae41e151c41e30bc39c52d4657deebb7b1d316e\
        5dffa77c0c6b3e09322e52a9b6ce569541446b0e13be4890a13024da309622ce22262e448d926f98b8056a1ea7\
        2a494886afefe5f00664a0f7767387a9f09c078f661f3d9947c63ca02c99f38e0d9849779a285ce09443d9055c\
        fda9761492397993db6aa864853b90ff3b5cb6598a50b3cf13ca0c4effa4bca744273714b98ccb5f6c41b2faf8\
        77eddda4d24365233a13938992ec6dc0acf84f2de1298c69cba7b8e0298008606b40425ac77164855238173ba1\
        26b5ed33efbb92437778b4fd34a477b48da28a9e8f9056799cc103f25fab431d92f9f6e81aea03fc4c294a92ae\
        0321b886c369924193aa62dea38a7372a22e08485b4fa956ab30a4e8393a8022eed9dda62bb750bfcc3beb5a4d\
        d138e94b4cb5666632a0a56b5714844ecc42839165f52024a03bbb8187993fe005438f524e1331ef03241eecbc\
        b9fd1feca21c64306ef2098ce9cc946386ef3db8b9def84159b8ad0402e49c02d4908886c7407d7c03625ffed8\
        7c81c3b0c2c8ad2b15de5b0dcc4e3dea00802796913baa4fb5b75dd916dd50a05179344bb41b2199d848d4a075\
        51d28e1518ed776d789132e26e136ce3d16cbab60419f81fb7804c62015cc98b683da1c8a90062de1ec62497aa\
        5d6e352e527669bd39b54f34a4955b4216eef318cf7c63b2945b41bedfe55d0d7188aefd0d7006d7d863326b25\
        b82f6983294dfab2b9d7fa3dcfcb579df3aefc994184055fb46330ae58203117d0ef26cd2599ec78dab84e69b7\
        4a127525f09da91998d55785432a7d2e0e9079f85e6bc2dbb7c918245bdb90bc4a9d3637137378075f7ac254dd\
        bed625d335567e7bb0e816896f8d8e0ccc63bd6e1ed2443502efbfa406317f8564d766ede2e1fb6ef680fe3c85\
        b6d951d12d1cd578049a9d6822bdb5a1694bf4025d383ed07553b50acbd95090b16dbee7ef2fd7f6dc4fedf44b\
        63b727e548338401f0ab742ffc3fe839f1419b3b0c30c15755ea6d7f3d9b736c79cb3caaddf98a46bc20b6f982\
        196e39ab092e73864dc65987eb65fd10052723602d06ead23b790e90931422e5ca4b0b8702b0#10000",
        Less,
    );

    let one_over_pi_f32 = Float::one_over_pi_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(one_over_pi_f32.to_string(), "0.31830987");
    assert_eq!(to_hex_string(&one_over_pi_f32), "0x0.517cc18#24");
    assert_eq!(one_over_pi_f32, f32::ONE_OVER_PI);

    let one_over_pi_f64 = Float::one_over_pi_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(one_over_pi_f64.to_string(), "0.31830988618379069");
    assert_eq!(to_hex_string(&one_over_pi_f64), "0x0.517cc1b727220c#53");
    assert_eq!(one_over_pi_f64, f64::ONE_OVER_PI);
}

#[test]
#[should_panic]
fn one_over_pi_prec_fail_1() {
    Float::one_over_pi_prec(0);
}

fn test_one_over_pi_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::one_over_pi_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = one_over_pi_prec_round_simple(prec, rm);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);
}

#[test]
pub fn test_one_over_pi_prec_round() {
    test_one_over_pi_prec_round_helper(1, Floor, "0.2", "0x0.4#1", Less);
    test_one_over_pi_prec_round_helper(1, Ceiling, "0.5", "0x0.8#1", Greater);
    test_one_over_pi_prec_round_helper(1, Down, "0.2", "0x0.4#1", Less);
    test_one_over_pi_prec_round_helper(1, Up, "0.5", "0x0.8#1", Greater);
    test_one_over_pi_prec_round_helper(1, Nearest, "0.2", "0x0.4#1", Less);

    test_one_over_pi_prec_round_helper(2, Floor, "0.2", "0x0.4#2", Less);
    test_one_over_pi_prec_round_helper(2, Ceiling, "0.4", "0x0.6#2", Greater);
    test_one_over_pi_prec_round_helper(2, Down, "0.2", "0x0.4#2", Less);
    test_one_over_pi_prec_round_helper(2, Up, "0.4", "0x0.6#2", Greater);
    test_one_over_pi_prec_round_helper(2, Nearest, "0.4", "0x0.6#2", Greater);

    test_one_over_pi_prec_round_helper(3, Floor, "0.3", "0x0.5#3", Less);
    test_one_over_pi_prec_round_helper(3, Ceiling, "0.38", "0x0.6#3", Greater);
    test_one_over_pi_prec_round_helper(3, Down, "0.3", "0x0.5#3", Less);
    test_one_over_pi_prec_round_helper(3, Up, "0.38", "0x0.6#3", Greater);
    test_one_over_pi_prec_round_helper(3, Nearest, "0.3", "0x0.5#3", Less);

    test_one_over_pi_prec_round_helper(4, Floor, "0.31", "0x0.50#4", Less);
    test_one_over_pi_prec_round_helper(4, Ceiling, "0.34", "0x0.58#4", Greater);
    test_one_over_pi_prec_round_helper(4, Down, "0.31", "0x0.50#4", Less);
    test_one_over_pi_prec_round_helper(4, Up, "0.34", "0x0.58#4", Greater);
    test_one_over_pi_prec_round_helper(4, Nearest, "0.31", "0x0.50#4", Less);

    test_one_over_pi_prec_round_helper(5, Floor, "0.31", "0x0.50#5", Less);
    test_one_over_pi_prec_round_helper(5, Ceiling, "0.33", "0x0.54#5", Greater);
    test_one_over_pi_prec_round_helper(5, Down, "0.31", "0x0.50#5", Less);
    test_one_over_pi_prec_round_helper(5, Up, "0.33", "0x0.54#5", Greater);
    test_one_over_pi_prec_round_helper(5, Nearest, "0.31", "0x0.50#5", Less);

    test_one_over_pi_prec_round_helper(6, Floor, "0.31", "0x0.50#6", Less);
    test_one_over_pi_prec_round_helper(6, Ceiling, "0.32", "0x0.52#6", Greater);
    test_one_over_pi_prec_round_helper(6, Down, "0.31", "0x0.50#6", Less);
    test_one_over_pi_prec_round_helper(6, Up, "0.32", "0x0.52#6", Greater);
    test_one_over_pi_prec_round_helper(6, Nearest, "0.32", "0x0.52#6", Greater);

    test_one_over_pi_prec_round_helper(7, Floor, "0.316", "0x0.51#7", Less);
    test_one_over_pi_prec_round_helper(7, Ceiling, "0.32", "0x0.52#7", Greater);
    test_one_over_pi_prec_round_helper(7, Down, "0.316", "0x0.51#7", Less);
    test_one_over_pi_prec_round_helper(7, Up, "0.32", "0x0.52#7", Greater);
    test_one_over_pi_prec_round_helper(7, Nearest, "0.316", "0x0.51#7", Less);

    test_one_over_pi_prec_round_helper(8, Floor, "0.316", "0x0.510#8", Less);
    test_one_over_pi_prec_round_helper(8, Ceiling, "0.318", "0x0.518#8", Greater);
    test_one_over_pi_prec_round_helper(8, Down, "0.316", "0x0.510#8", Less);
    test_one_over_pi_prec_round_helper(8, Up, "0.318", "0x0.518#8", Greater);
    test_one_over_pi_prec_round_helper(8, Nearest, "0.318", "0x0.518#8", Greater);

    test_one_over_pi_prec_round_helper(9, Floor, "0.317", "0x0.514#9", Less);
    test_one_over_pi_prec_round_helper(9, Ceiling, "0.318", "0x0.518#9", Greater);
    test_one_over_pi_prec_round_helper(9, Down, "0.317", "0x0.514#9", Less);
    test_one_over_pi_prec_round_helper(9, Up, "0.318", "0x0.518#9", Greater);
    test_one_over_pi_prec_round_helper(9, Nearest, "0.318", "0x0.518#9", Greater);

    test_one_over_pi_prec_round_helper(10, Floor, "0.3179", "0x0.516#10", Less);
    test_one_over_pi_prec_round_helper(10, Ceiling, "0.3184", "0x0.518#10", Greater);
    test_one_over_pi_prec_round_helper(10, Down, "0.3179", "0x0.516#10", Less);
    test_one_over_pi_prec_round_helper(10, Up, "0.3184", "0x0.518#10", Greater);
    test_one_over_pi_prec_round_helper(10, Nearest, "0.3184", "0x0.518#10", Greater);

    test_one_over_pi_prec_round_helper(
        100,
        Floor,
        "0.3183098861837906715377675267449",
        "0x0.517cc1b727220a94fe13abe8f8#100",
        Less,
    );
    test_one_over_pi_prec_round_helper(
        100,
        Ceiling,
        "0.3183098861837906715377675267453",
        "0x0.517cc1b727220a94fe13abe900#100",
        Greater,
    );
    test_one_over_pi_prec_round_helper(
        100,
        Down,
        "0.3183098861837906715377675267449",
        "0x0.517cc1b727220a94fe13abe8f8#100",
        Less,
    );
    test_one_over_pi_prec_round_helper(
        100,
        Up,
        "0.3183098861837906715377675267453",
        "0x0.517cc1b727220a94fe13abe900#100",
        Greater,
    );
    test_one_over_pi_prec_round_helper(
        100,
        Nearest,
        "0.3183098861837906715377675267449",
        "0x0.517cc1b727220a94fe13abe8f8#100",
        Less,
    );
}

#[test]
#[should_panic]
fn one_over_pi_prec_round_fail_1() {
    Float::one_over_pi_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn one_over_pi_prec_round_fail_2() {
    Float::one_over_pi_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn one_over_pi_prec_round_fail_3() {
    Float::one_over_pi_prec_round(1000, Exact);
}

#[test]
fn one_over_pi_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (one_over_pi, o) = Float::one_over_pi_prec(prec);
        assert!(one_over_pi.is_valid());
        assert_eq!(one_over_pi.get_prec(), Some(prec));
        assert_eq!(one_over_pi.get_exponent(), Some(-1));
        assert_ne!(o, Equal);
        if o == Less {
            let (one_over_pi_alt, o_alt) = Float::one_over_pi_prec_round(prec, Ceiling);
            let mut next_upper = one_over_pi.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(
                    ComparableFloat(one_over_pi_alt),
                    ComparableFloat(next_upper)
                );
                assert_eq!(o_alt, Greater);
            }
        } else if !one_over_pi.is_power_of_2() {
            let (one_over_pi_alt, o_alt) = Float::one_over_pi_prec_round(prec, Floor);
            let mut next_lower = one_over_pi.clone();
            next_lower.decrement();
            assert_eq!(
                ComparableFloat(one_over_pi_alt),
                ComparableFloat(next_lower)
            );
            assert_eq!(o_alt, Less);
        }
        let (one_over_pi_alt, o_alt) = Float::one_over_pi_prec_round(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&one_over_pi_alt),
            ComparableFloatRef(&one_over_pi)
        );
        assert_eq!(o_alt, o);

        let (one_over_pi_alt, o_alt) = one_over_pi_prec_round_simple(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&one_over_pi_alt),
            ComparableFloatRef(&one_over_pi)
        );
        assert_eq!(o_alt, o);
    });
}

#[test]
fn one_over_pi_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (one_over_pi, o) = Float::one_over_pi_prec_round(prec, rm);
        assert!(one_over_pi.is_valid());
        assert_eq!(one_over_pi.get_prec(), Some(prec));
        assert_eq!(
            one_over_pi.get_exponent(),
            Some(if prec == 1 && (rm == Ceiling || rm == Up) {
                0
            } else {
                -1
            })
        );
        assert_ne!(o, Equal);
        if o == Less {
            let (one_over_pi_alt, o_alt) = Float::one_over_pi_prec_round(prec, Ceiling);
            let mut next_upper = one_over_pi.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(
                    ComparableFloat(one_over_pi_alt),
                    ComparableFloat(next_upper)
                );
                assert_eq!(o_alt, Greater);
            }
        } else if !one_over_pi.is_power_of_2() {
            let (one_over_pi_alt, o_alt) = Float::one_over_pi_prec_round(prec, Floor);
            let mut next_lower = one_over_pi.clone();
            next_lower.decrement();
            assert_eq!(
                ComparableFloat(one_over_pi_alt),
                ComparableFloat(next_lower)
            );
            assert_eq!(o_alt, Less);
        }

        let (one_over_pi_alt, o_alt) = one_over_pi_prec_round_simple(prec, rm);
        assert_eq!(
            ComparableFloatRef(&one_over_pi_alt),
            ComparableFloatRef(&one_over_pi)
        );
        assert_eq!(o_alt, o);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::one_over_pi_prec_round(prec, Exact));
    });

    test_constant(Float::one_over_pi_prec_round, 10000);
}
