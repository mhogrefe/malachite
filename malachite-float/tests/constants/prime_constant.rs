// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::PrimeConstant;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::to_hex_string;
use malachite_float::test_util::constants::prime_constant::prime_constant_prec_round_naive;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_prime_constant_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::prime_constant_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = prime_constant_prec_round_naive(prec, Nearest);
    assert_eq!(x, x_alt);
    assert_eq!(o, o_alt);
}

#[test]
pub fn test_prime_constant_prec() {
    test_prime_constant_prec_helper(1, "0.5", "0x0.8#1", Greater);
    test_prime_constant_prec_helper(2, "0.4", "0x0.6#2", Less);
    test_prime_constant_prec_helper(3, "0.44", "0x0.7#3", Greater);
    test_prime_constant_prec_helper(4, "0.41", "0x0.68#4", Less);
    test_prime_constant_prec_helper(5, "0.42", "0x0.6c#5", Greater);
    test_prime_constant_prec_helper(6, "0.414", "0x0.6a#6", Less);
    test_prime_constant_prec_helper(7, "0.414", "0x0.6a#7", Less);
    test_prime_constant_prec_helper(8, "0.414", "0x0.6a0#8", Less);
    test_prime_constant_prec_helper(9, "0.415", "0x0.6a4#9", Greater);
    test_prime_constant_prec_helper(10, "0.4146", "0x0.6a2#10", Less);
    test_prime_constant_prec_helper(
        100,
        "0.4146825098511116602481096221542",
        "0x0.6a28a20a08a208282282208088#100",
        Less,
    );
    test_prime_constant_prec_helper(
        1000,
        "0.414682509851111660248109622154307708365774238137916977868245414488640960619357334196290\
        048428475777939616159352082985957835749978453022009904120814650033958993701974119186285615\
        579237191637251488161071073428432480218016979856815134246794749243902778820311537783180662\
        98787003544718107300315282293658657",
        "0x0.6a28a20a08a20828228220808a28800220a00a08220828028a00200228828020820a08a00800228800208\
        028820208220808808028028220808a20020220220800a00008200820a08020828208a00200a20828008820200\
        808020208220208808800200800a00a28020008a20008a200002202008088208002208202080#1000",
        Less,
    );
    test_prime_constant_prec_helper(
        10000,
        "0.414682509851111660248109622154307708365774238137916977868245414488640960619357334196290\
        048428475777939616159352082985957835749978453022009904120814650033958993701974119186285615\
        579237191637251488161071073428432480218016979856815134246794749243902778820311537783180662\
        987870035447181073003152822936586571701195884345760580684835877525498698320011540191930758\
        099002319261468413919152574118369129168574279239129804341592424543430422903399908325112045\
        668292742611194732667211553722399175647987527658878910703815406904018048152558727250330913\
        032734465178315959501124916374515488312368223411699732976990859213965904304781712473399850\
        022855936162301792842610279636663239004692389139831027778036326093312778898964673169309518\
        310306063152926347489776338241054180226337674492907279938068989754912015273280509376794011\
        427472704319064395781618400649434291302477306540023894812147846692492936635719588061120657\
        051390501956020214967100891969377486235872143778145145772057385386205343716378192432613513\
        172804602223001006685864520195545148953810709046574934296166432665763774833905637940402631\
        576801946504697746229836623924610157737793410864303438788766145163542908821015516523230106\
        797290246886870760828326453371632716614992987127399459741555239537031562083871442168565303\
        189912375257157779026541835903342269582971374263275545243196592115378128694441070207946879\
        764496037081508369373832295654819400087481268863376301792472431388415929294658211923608901\
        402710138982797422281950231204740825160556996531760010125162932087538066645067994257503747\
        988951392973626720366328982170505846234832544143958403519197518756622565009818379834626553\
        617954708446358169530464486348037793106750286422766029143787907491013162528697477255180525\
        301688970187379440498919433856401585513390951795192326204312092377569597171283867789460769\
        396842845072133229690180683640002570495073917768933999215761139873608859874089589096776461\
        995923745390535138086006656259166023212941879359754829079532092146864715376576674216923592\
        356284657954731472882379615899071977783469111502062539802490350514979367013733212811256787\
        651076562739406500368100788692192060657019146542871834536027154909579465624259941136769107\
        561003973746479133595976239940200240349439391177366640902400434482715348474352654705232334\
        865757263289528822462280034190931946143020430007789481367871305250457767936834157491630698\
        107741376398506062111121345698764997552559136414698454192672485636202709453131509266585281\
        706706580881014762163488645479634465001692797445197175229384471918672226624328100132469141\
        109007889697011845498155181004138530396370327432831414575546500251524843825045622130807383\
        973351694821712447724540951485939349474765305310454105446321469155906341381357010374677903\
        488746578301462245594571327626904723478958729803396726212447661231362593184418074810567100\
        150303566417543158729492869454556673124380354806665894633517169712578312518114309438722159\
        398158466744490582994391095649028975452941714271570359608073520730264326067239780059666358\
        3765668592857275961081596601034186349684386",
        "0x0.6a28a20a08a20828228220808a28800220a00a08220828028a00200228828020820a08a00800228800208\
        028820208220808808028028220808a20020220220800a00008200820a08020828208a00200a20828008820200\
        808020208220208808800200800a00a28020008a20008a20000220200808820800220820208008828028200a00\
        a08000228820808208000028020200820808008820a08008020000a20a08a20028200000000820808000200800\
        228820228200200a28820020020200208820220220008828828200800008228000008a00800a00808208200008\
        228008020020200020008228a00800820800028000a00000820208a20208028020008020800a28020028000a08\
        a00808000200000220808000a20200028808208800a00000828220800208a08820020820008820020208800000\
        200008020220a08000020028000808800800200a20208228820228200a00808800200800a08a00020008820208\
        800020200820808020020008820a00000028020220008808008200220800008028000800000228008220020808\
        808000028a00200000028208220a00220028028000a00020800008000808a28000020200208028828200a00200\
        a00800208820208200020028800208808020208200000828020008820000a00800228020008808000220a20a00\
        220000200000800a20828200000a08000820028200200028820008a00008000008020a00a00a00828028020808\
        000000200a00808200800020800800a00008220820000a00800208a20000028008200020208200008000a20a08\
        200820020020000a08800020200000228828008800200000008020800800208020028820208028800208220a00\
        a00020020200008820028208200000020800220200808008000228800000800828000820800200800220000000\
        820820208a00208a08000020a20000a20028220000008208808020000000200028228a00800208808020800008\
        a08000000220200820808200220a08828028000820000220020008000000820000220000808220800208200a00\
        800808028020200200800000a20208808000200820a08020028020820808200828208200808000008200000a00\
        008808020000000808000228020a08800020200200008000828008200a00a08020008800000a00028028020000\
        228808000820a00200020220800000028028220820208800820200220008008020000020800828000020200008\
        020800228200808220800000020008028008028000a00008000008000208000800020a00008828020200820808\
        828028008820a00220008000220800008808220220008220008008a00000008800000820800800800208a20000\
        820020208800a00200000008000a08000020008200800828808020820000020008220200200008028028820000\
        208800228800208008000008020808028000000220a00220008200000000800808200a20000200808000a00800\
        228008200020808808820208200208820808000220000220028020a00800a20000208000000208808020020a00\
        000028228000800a08020200000200208000208800208800808200220800820028000800008a00800020220008\
        a08000020800208200828028a00000a20820020800200800820220000000020008008220a00208008000a00808\
        000028008220800028008220020208008820020a00200220008200020a008080002080000000#10000",
        Less,
    );

    let pc_f32 = Float::prime_constant_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(pc_f32.to_string(), "0.41468251");
    assert_eq!(to_hex_string(&pc_f32), "0x0.6a28a20#24");
    assert_eq!(pc_f32, f32::PRIME_CONSTANT);

    let pc_f64 = Float::prime_constant_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(pc_f64.to_string(), "0.41468250985111166");
    assert_eq!(to_hex_string(&pc_f64), "0x0.6a28a20a08a208#53");
    assert_eq!(pc_f64, f64::PRIME_CONSTANT);
}

fn test_prime_constant_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::prime_constant_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = prime_constant_prec_round_naive(prec, rm);
    assert_eq!(x, x_alt);
    assert_eq!(o, o_alt);
}

#[test]
pub fn test_prime_constant_prec_round() {
    test_prime_constant_prec_round_helper(1, Floor, "0.2", "0x0.4#1", Less);
    test_prime_constant_prec_round_helper(1, Ceiling, "0.5", "0x0.8#1", Greater);
    test_prime_constant_prec_round_helper(1, Down, "0.2", "0x0.4#1", Less);
    test_prime_constant_prec_round_helper(1, Up, "0.5", "0x0.8#1", Greater);
    test_prime_constant_prec_round_helper(1, Nearest, "0.5", "0x0.8#1", Greater);

    test_prime_constant_prec_round_helper(2, Floor, "0.4", "0x0.6#2", Less);
    test_prime_constant_prec_round_helper(2, Ceiling, "0.5", "0x0.8#2", Greater);
    test_prime_constant_prec_round_helper(2, Down, "0.4", "0x0.6#2", Less);
    test_prime_constant_prec_round_helper(2, Up, "0.5", "0x0.8#2", Greater);
    test_prime_constant_prec_round_helper(2, Nearest, "0.4", "0x0.6#2", Less);

    test_prime_constant_prec_round_helper(3, Floor, "0.38", "0x0.6#3", Less);
    test_prime_constant_prec_round_helper(3, Ceiling, "0.44", "0x0.7#3", Greater);
    test_prime_constant_prec_round_helper(3, Down, "0.38", "0x0.6#3", Less);
    test_prime_constant_prec_round_helper(3, Up, "0.44", "0x0.7#3", Greater);
    test_prime_constant_prec_round_helper(3, Nearest, "0.44", "0x0.7#3", Greater);

    test_prime_constant_prec_round_helper(4, Floor, "0.41", "0x0.68#4", Less);
    test_prime_constant_prec_round_helper(4, Ceiling, "0.44", "0x0.70#4", Greater);
    test_prime_constant_prec_round_helper(4, Down, "0.41", "0x0.68#4", Less);
    test_prime_constant_prec_round_helper(4, Up, "0.44", "0x0.70#4", Greater);
    test_prime_constant_prec_round_helper(4, Nearest, "0.41", "0x0.68#4", Less);

    test_prime_constant_prec_round_helper(5, Floor, "0.41", "0x0.68#5", Less);
    test_prime_constant_prec_round_helper(5, Ceiling, "0.42", "0x0.6c#5", Greater);
    test_prime_constant_prec_round_helper(5, Down, "0.41", "0x0.68#5", Less);
    test_prime_constant_prec_round_helper(5, Up, "0.42", "0x0.6c#5", Greater);
    test_prime_constant_prec_round_helper(5, Nearest, "0.42", "0x0.6c#5", Greater);

    test_prime_constant_prec_round_helper(6, Floor, "0.414", "0x0.6a#6", Less);
    test_prime_constant_prec_round_helper(6, Ceiling, "0.42", "0x0.6c#6", Greater);
    test_prime_constant_prec_round_helper(6, Down, "0.414", "0x0.6a#6", Less);
    test_prime_constant_prec_round_helper(6, Up, "0.42", "0x0.6c#6", Greater);
    test_prime_constant_prec_round_helper(6, Nearest, "0.414", "0x0.6a#6", Less);

    test_prime_constant_prec_round_helper(7, Floor, "0.414", "0x0.6a#7", Less);
    test_prime_constant_prec_round_helper(7, Ceiling, "0.418", "0x0.6b#7", Greater);
    test_prime_constant_prec_round_helper(7, Down, "0.414", "0x0.6a#7", Less);
    test_prime_constant_prec_round_helper(7, Up, "0.418", "0x0.6b#7", Greater);
    test_prime_constant_prec_round_helper(7, Nearest, "0.414", "0x0.6a#7", Less);

    test_prime_constant_prec_round_helper(8, Floor, "0.414", "0x0.6a0#8", Less);
    test_prime_constant_prec_round_helper(8, Ceiling, "0.416", "0x0.6a8#8", Greater);
    test_prime_constant_prec_round_helper(8, Down, "0.414", "0x0.6a0#8", Less);
    test_prime_constant_prec_round_helper(8, Up, "0.416", "0x0.6a8#8", Greater);
    test_prime_constant_prec_round_helper(8, Nearest, "0.414", "0x0.6a0#8", Less);

    test_prime_constant_prec_round_helper(9, Floor, "0.414", "0x0.6a0#9", Less);
    test_prime_constant_prec_round_helper(9, Ceiling, "0.415", "0x0.6a4#9", Greater);
    test_prime_constant_prec_round_helper(9, Down, "0.414", "0x0.6a0#9", Less);
    test_prime_constant_prec_round_helper(9, Up, "0.415", "0x0.6a4#9", Greater);
    test_prime_constant_prec_round_helper(9, Nearest, "0.415", "0x0.6a4#9", Greater);

    test_prime_constant_prec_round_helper(10, Floor, "0.4146", "0x0.6a2#10", Less);
    test_prime_constant_prec_round_helper(10, Ceiling, "0.415", "0x0.6a4#10", Greater);
    test_prime_constant_prec_round_helper(10, Down, "0.4146", "0x0.6a2#10", Less);
    test_prime_constant_prec_round_helper(10, Up, "0.415", "0x0.6a4#10", Greater);
    test_prime_constant_prec_round_helper(10, Nearest, "0.4146", "0x0.6a2#10", Less);

    test_prime_constant_prec_round_helper(
        100,
        Floor,
        "0.4146825098511116602481096221542",
        "0x0.6a28a20a08a208282282208088#100",
        Less,
    );
    test_prime_constant_prec_round_helper(
        100,
        Ceiling,
        "0.4146825098511116602481096221546",
        "0x0.6a28a20a08a208282282208090#100",
        Greater,
    );
    test_prime_constant_prec_round_helper(
        100,
        Down,
        "0.4146825098511116602481096221542",
        "0x0.6a28a20a08a208282282208088#100",
        Less,
    );
    test_prime_constant_prec_round_helper(
        100,
        Up,
        "0.4146825098511116602481096221546",
        "0x0.6a28a20a08a208282282208090#100",
        Greater,
    );
    test_prime_constant_prec_round_helper(
        100,
        Nearest,
        "0.4146825098511116602481096221542",
        "0x0.6a28a20a08a208282282208088#100",
        Less,
    );
}

#[test]
#[should_panic]
fn prime_constant_prec_round_fail_1() {
    Float::prime_constant_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn prime_constant_prec_round_fail_2() {
    Float::prime_constant_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn prime_constant_prec_round_fail_3() {
    Float::prime_constant_prec_round(1000, Exact);
}

#[test]
fn prime_constant_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (pc, o) = Float::prime_constant_prec(prec);
        assert!(pc.is_valid());
        assert_eq!(pc.get_prec(), Some(prec));
        assert_ne!(o, Equal);
        if o == Less {
            let (pc_alt, o_alt) = Float::prime_constant_prec_round(prec, Ceiling);
            let mut next_upper = pc.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(pc_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !pc.is_power_of_2() {
            let (pc_alt, o_alt) = Float::prime_constant_prec_round(prec, Floor);
            let mut next_lower = pc.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(pc_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
        let (pc_alt, o_alt) = Float::prime_constant_prec_round(prec, Nearest);
        assert_eq!(ComparableFloatRef(&pc_alt), ComparableFloatRef(&pc));
        assert_eq!(o_alt, o);

        let (pc_alt, o_alt) = prime_constant_prec_round_naive(prec, Nearest);
        assert_eq!(pc, pc_alt);
        assert_eq!(o, o_alt);
    });
}

#[test]
fn prime_constant_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (pc, o) = Float::prime_constant_prec_round(prec, rm);
        assert!(pc.is_valid());
        assert_eq!(pc.get_prec(), Some(prec));
        assert_ne!(o, Equal);
        if o == Less {
            let (pc_alt, o_alt) = Float::prime_constant_prec_round(prec, Ceiling);
            let mut next_upper = pc.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(pc_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !pc.is_power_of_2() {
            let (pc_alt, o_alt) = Float::prime_constant_prec_round(prec, Floor);
            let mut next_lower = pc.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(pc_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }

        let (pc_alt, o_alt) = prime_constant_prec_round_naive(prec, rm);
        assert_eq!(pc, pc_alt);
        assert_eq!(o, o_alt);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::prime_constant_prec_round(prec, Exact));
    });
}
