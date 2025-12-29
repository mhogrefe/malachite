// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::Sqrt3Over3;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{
    rug_round_try_from_rounding_mode, test_constant, to_hex_string,
};
use malachite_float::test_util::constants::sqrt_3_over_3::rug_sqrt_3_over_3_prec_round;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_sqrt_3_over_3_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::sqrt_3_over_3_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (rug_x, rug_o) =
        rug_sqrt_3_over_3_prec_round(prec, rug_round_try_from_rounding_mode(Nearest).unwrap());
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_x)),
        ComparableFloatRef(&x)
    );
    assert_eq!(rug_o, o);
}

#[test]
pub fn test_sqrt_3_over_3_prec() {
    test_sqrt_3_over_3_prec_helper(1, "0.5", "0x0.8#1", Less);
    test_sqrt_3_over_3_prec_helper(2, "0.5", "0x0.8#2", Less);
    test_sqrt_3_over_3_prec_helper(3, "0.6", "0x0.a#3", Greater);
    test_sqrt_3_over_3_prec_helper(4, "0.56", "0x0.9#4", Less);
    test_sqrt_3_over_3_prec_helper(5, "0.56", "0x0.90#5", Less);
    test_sqrt_3_over_3_prec_helper(6, "0.58", "0x0.94#6", Greater);
    test_sqrt_3_over_3_prec_helper(7, "0.58", "0x0.94#7", Greater);
    test_sqrt_3_over_3_prec_helper(8, "0.578", "0x0.94#8", Greater);
    test_sqrt_3_over_3_prec_helper(9, "0.578", "0x0.940#9", Greater);
    test_sqrt_3_over_3_prec_helper(10, "0.577", "0x0.93c#10", Less);
    test_sqrt_3_over_3_prec_helper(
        100,
        "0.577350269189625764509148780502",
        "0x0.93cd3a2c8198e2690c7c0f258#100",
        Greater,
    );
    test_sqrt_3_over_3_prec_helper(
        1000,
        "0.577350269189625764509148780501957455647601751270126876018602326483977672302933345693715\
        395585749525225208713805135567676656648364999650826270551837364791216176031077300768527355\
        991606700361558307755005104114422301107628883557418222973945990409015710553455953862673016\
        6621791266197964892167825021920169",
        "0x0.93cd3a2c8198e2690c7c0f257d92be830c9d66eec69e17dd97b58cc2cf6c8cf61859454874fb1f3f38865\
        8e4b0b62fd7eec450c48be0a422f044668ab39de12be8a060f75bd968f79f74f4be9c0f3d100923fc713a7e58a\
        9cf94fbbe338aa1d2631625104c830e3c7b80799a541d88671fd505ebf61a67cd8c97ee859c#1000",
        Less,
    );
    test_sqrt_3_over_3_prec_helper(
        10000,
        "0.577350269189625764509148780501957455647601751270126876018602326483977672302933345693715\
        395585749525225208713805135567676656648364999650826270551837364791216176031077300768527355\
        991606700361558307755005104114422301107628883557418222973945990409015710553455953862673016\
        662179126619796489216782502192016918872782709868700315867395730108361048609841319944332596\
        608169429571487944305782408054661529285132555986021272784555370281057926964792772034029435\
        174967223341173570381429565330318788599029056166024299831098828094340262136201329579565845\
        860772439277131997661002612900959017971123187770701235754673083035589410399762791880380474\
        005580917367457664756943686632819825329221429632598715945986130076284950967858677950936021\
        460657448220353229907624288421384607554899473340398494718426147060428844835678397216672229\
        764805160282023759047999920975611543192479453965036709049546248992181994150596183658459671\
        293554987303987407652370185080974574397399246087520543822947344284572227646216537305683246\
        278798498646048585890617791211041970299885141541611595732535078665923064107856740101767613\
        461971804990357476519735568734169840058654395290922119991761221143693383555284570215776179\
        541974086860740170134560090099168266242693153886033472350893980006365244487391981297890083\
        106807563678123270821240037061753694756303433240103734033354502392124860801734615851732426\
        644599204968702631661848074442001339617101210520381628949093863096054415062421688881794959\
        199720858880957370413614022672254505700034314393571838396987214153635679802709738967839164\
        320454668046556810347084544109010307525775986764125532581845698489705247380141410260664109\
        205800635474837484959172287565368444564738737868464868081884713376177669271211149952245468\
        901325780743270653680975178167200468646826515718178182802423912552087455516388880077668671\
        088581356702612286156150771053489251493500134080213303990120739534306746289050237005723133\
        432289586554500013631772071411141742652278022337823395300676120238920715914497816521198602\
        694043480862156489507536969421117669585537534486890731801351994709637658664936666924019146\
        743349224419600030504790496491814959719063764886361190291320505179265880087356123616015357\
        656460906528563253253208701386384219192596927444993522613340967177350712978457925178855475\
        682275624763321147131914555357914815194362159297737661767382732465113826056838871650850587\
        763997899855248174213037506364513795663122272476979408808245642067357663655943718043982161\
        459305153305307976095904898383091342136404548551347214860496733981916473101756334204657548\
        693688357146534402025836864729266094931328731749988055071046571764533140632472215691729078\
        596209822036186723476289868599464684302135810270611129559293854211955718669465586748449743\
        314646289019918105147647810319086156996071821037086922258022621523859353549149166583451514\
        853104428849729949778857582542099138246078562304945943787787094542634608280054602223869059\
        949576185280977116408697746855319184550378918214795944761104488881518060130273966327764875\
        44833872370733897653982005644020701101324831",
        "0x0.93cd3a2c8198e2690c7c0f257d92be830c9d66eec69e17dd97b58cc2cf6c8cf61859454874fb1f3f38865\
        8e4b0b62fd7eec450c48be0a422f044668ab39de12be8a060f75bd968f79f74f4be9c0f3d100923fc713a7e58a\
        9cf94fbbe338aa1d2631625104c830e3c7b80799a541d88671fd505ebf61a67cd8c97ee859c7100c574a55c425\
        8f3b4eb3e51da5816bbeaf1ccdce013487648166b318e3b219cc0fd24b98529ea476e689e4653527b79786596d\
        f1855449605b18cebce639255d8fcc95afd578da9a6b2da5127f0f1e7545dc0f7a9c0a89b692228069fc0ae784\
        65be6828f559d35f49c038cde91bc64670b70de26233a5e44cb9810c9a459b2eb6cbc9ccb453c22ea0903254cd\
        28d2d1cffc23c7024600e9391f66706707d202af7c5f7e24f0f99474390116a77d3eba6f100ae5b97369f1c07a\
        c5c509db04218d2cf351a9f4bacf544af4d6cce59590587d55bc2e3187e437c949b3b992e01611053c371500fe\
        a00e594773bd64c6827ede691b2fd67fade8a36a39e19f7d17c19777fa27d8d4a0b004fbd6d1d1482ea73127de\
        5208c179c9a7cebdf891f68da2ba39e69392e94f1d142b666433cdb8f2d86213122d419a9b783fd95b51a686da\
        bd5cab015c40bb47391a31c4436708ae87e895f54340cbe95daa0902ddfa8f7f415f215359b88ca1f1c0a70cb2\
        8e6cd099f9d1dc124baf36116db3b30f4245daff79074d0f58b1643710ebdd0a04c7e1af1b2661e70c074450a1\
        aa3dfa93695030a896aa913890c13e555e1fe2ededb7b5b1ec9352450310ca4421de917e13b9ea77597ea9c017\
        808c95595ae96ed3284c61ca95e35b6597d22fcbb443e1ff8265c1b7c12540a081b554502585755c091f7ed59b\
        a8041d4bfc50616217496d364d73eea192f28d5d74d00052771c3337806cd47b0003f01b6c46d317ddb4ab480b\
        69807ec1d2acb29635ff9d506fbbc2073a74467d90dd9678ff6dd4f1a247c3da8265b997f34013b3b758f704e4\
        d69060a7322ccd78c15b1adc1cb7c644da2a4fb93a712be3a77ec3deebf5b9cf9fb6b6fe3b4b0a9f8d46bbe98c\
        19c560f9c25f0ae4caf26e8714103e958d98ccda8240baffc3c0f5c4f75a8b7c64c20cc0ef93c968148ac2658a\
        54ecfffb433913d324fd9b3c17a79292d17139a697d499e3d474e65afe83ad355c4deabf5a5f5a99f2c094174b\
        5d78ff72f585f779df25bdc653fb9a52d732e4a6ef62500ac19c64fcdf4370619db39eaad8dbd1f37f973c2871\
        0550b179808e58754ae331e15da474fa88e59016facbcdf14bd3b3dd3e505a4b227ab38b9499f62f691390861a\
        35313aa9ca15642bfe9674340a3f59187e5f90a906669b3126a1e7bb7e5f372fa8896009fa4b02e2d753472346\
        7d6bf7d1e13d37a3f6b413ada7c0180dbc1b341480ee3f23eb26dd8f346d6c623b8fc8a3183b15400337e4dd47\
        f9b49b89f79743faa59f9152565190572d12c7fba8d17074437afd6c69e53c947b018b1e4861dd62417eb28ecd\
        e1aee762ac1d38d3a236f4511dcb2d5ce77737318758807553be88c98bf3cdc08e5a8e8b32a482d02158ad06e9\
        96874906af139129fd1e1eb13a7ff6f2e7d6f2c86e1de2219900a43cdb03f77b0e5fc0f2ab82cfc64475feb6e2\
        b1456e7de1bea7e9d41e10d7578c57f209884c01c5f9adb694b0428629a9666e88d0c2cf544a607bdc9a7d05bb\
        6008aeb2b16188af9f27d0866a8c8b318082865064e58f9d4076e78dc967d7aa76cbcb47c65#10000",
        Greater,
    );

    let sqrt_3_over_3_f32 = Float::sqrt_3_over_3_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(sqrt_3_over_3_f32.to_string(), "0.57735026");
    assert_eq!(to_hex_string(&sqrt_3_over_3_f32), "0x0.93cd3a#24");
    assert_eq!(sqrt_3_over_3_f32, f32::SQRT_3_OVER_3);

    let sqrt_3_over_3_f64 = Float::sqrt_3_over_3_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(sqrt_3_over_3_f64.to_string(), "0.5773502691896257");
    assert_eq!(to_hex_string(&sqrt_3_over_3_f64), "0x0.93cd3a2c8198e0#53");
    assert_eq!(sqrt_3_over_3_f64, f64::SQRT_3_OVER_3);
}

fn test_sqrt_3_over_3_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::sqrt_3_over_3_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_x, rug_o) = rug_sqrt_3_over_3_prec_round(prec, rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_x)),
            ComparableFloatRef(&x)
        );
        assert_eq!(rug_o, o);
    }
}

#[test]
pub fn test_sqrt_3_over_3_prec_round() {
    test_sqrt_3_over_3_prec_round_helper(1, Floor, "0.5", "0x0.8#1", Less);
    test_sqrt_3_over_3_prec_round_helper(1, Ceiling, "1.0", "0x1.0#1", Greater);
    test_sqrt_3_over_3_prec_round_helper(1, Down, "0.5", "0x0.8#1", Less);
    test_sqrt_3_over_3_prec_round_helper(1, Up, "1.0", "0x1.0#1", Greater);
    test_sqrt_3_over_3_prec_round_helper(1, Nearest, "0.5", "0x0.8#1", Less);

    test_sqrt_3_over_3_prec_round_helper(2, Floor, "0.5", "0x0.8#2", Less);
    test_sqrt_3_over_3_prec_round_helper(2, Ceiling, "0.8", "0x0.c#2", Greater);
    test_sqrt_3_over_3_prec_round_helper(2, Down, "0.5", "0x0.8#2", Less);
    test_sqrt_3_over_3_prec_round_helper(2, Up, "0.8", "0x0.c#2", Greater);
    test_sqrt_3_over_3_prec_round_helper(2, Nearest, "0.5", "0x0.8#2", Less);

    test_sqrt_3_over_3_prec_round_helper(3, Floor, "0.5", "0x0.8#3", Less);
    test_sqrt_3_over_3_prec_round_helper(3, Ceiling, "0.6", "0x0.a#3", Greater);
    test_sqrt_3_over_3_prec_round_helper(3, Down, "0.5", "0x0.8#3", Less);
    test_sqrt_3_over_3_prec_round_helper(3, Up, "0.6", "0x0.a#3", Greater);
    test_sqrt_3_over_3_prec_round_helper(3, Nearest, "0.6", "0x0.a#3", Greater);

    test_sqrt_3_over_3_prec_round_helper(4, Floor, "0.56", "0x0.9#4", Less);
    test_sqrt_3_over_3_prec_round_helper(4, Ceiling, "0.62", "0x0.a#4", Greater);
    test_sqrt_3_over_3_prec_round_helper(4, Down, "0.56", "0x0.9#4", Less);
    test_sqrt_3_over_3_prec_round_helper(4, Up, "0.62", "0x0.a#4", Greater);
    test_sqrt_3_over_3_prec_round_helper(4, Nearest, "0.56", "0x0.9#4", Less);

    test_sqrt_3_over_3_prec_round_helper(5, Floor, "0.56", "0x0.90#5", Less);
    test_sqrt_3_over_3_prec_round_helper(5, Ceiling, "0.59", "0x0.98#5", Greater);
    test_sqrt_3_over_3_prec_round_helper(5, Down, "0.56", "0x0.90#5", Less);
    test_sqrt_3_over_3_prec_round_helper(5, Up, "0.59", "0x0.98#5", Greater);
    test_sqrt_3_over_3_prec_round_helper(5, Nearest, "0.56", "0x0.90#5", Less);

    test_sqrt_3_over_3_prec_round_helper(6, Floor, "0.56", "0x0.90#6", Less);
    test_sqrt_3_over_3_prec_round_helper(6, Ceiling, "0.58", "0x0.94#6", Greater);
    test_sqrt_3_over_3_prec_round_helper(6, Down, "0.56", "0x0.90#6", Less);
    test_sqrt_3_over_3_prec_round_helper(6, Up, "0.58", "0x0.94#6", Greater);
    test_sqrt_3_over_3_prec_round_helper(6, Nearest, "0.58", "0x0.94#6", Greater);

    test_sqrt_3_over_3_prec_round_helper(7, Floor, "0.57", "0x0.92#7", Less);
    test_sqrt_3_over_3_prec_round_helper(7, Ceiling, "0.58", "0x0.94#7", Greater);
    test_sqrt_3_over_3_prec_round_helper(7, Down, "0.57", "0x0.92#7", Less);
    test_sqrt_3_over_3_prec_round_helper(7, Up, "0.58", "0x0.94#7", Greater);
    test_sqrt_3_over_3_prec_round_helper(7, Nearest, "0.58", "0x0.94#7", Greater);

    test_sqrt_3_over_3_prec_round_helper(8, Floor, "0.574", "0x0.93#8", Less);
    test_sqrt_3_over_3_prec_round_helper(8, Ceiling, "0.578", "0x0.94#8", Greater);
    test_sqrt_3_over_3_prec_round_helper(8, Down, "0.574", "0x0.93#8", Less);
    test_sqrt_3_over_3_prec_round_helper(8, Up, "0.578", "0x0.94#8", Greater);
    test_sqrt_3_over_3_prec_round_helper(8, Nearest, "0.578", "0x0.94#8", Greater);

    test_sqrt_3_over_3_prec_round_helper(9, Floor, "0.576", "0x0.938#9", Less);
    test_sqrt_3_over_3_prec_round_helper(9, Ceiling, "0.578", "0x0.940#9", Greater);
    test_sqrt_3_over_3_prec_round_helper(9, Down, "0.576", "0x0.938#9", Less);
    test_sqrt_3_over_3_prec_round_helper(9, Up, "0.578", "0x0.940#9", Greater);
    test_sqrt_3_over_3_prec_round_helper(9, Nearest, "0.578", "0x0.940#9", Greater);

    test_sqrt_3_over_3_prec_round_helper(10, Floor, "0.577", "0x0.93c#10", Less);
    test_sqrt_3_over_3_prec_round_helper(10, Ceiling, "0.578", "0x0.940#10", Greater);
    test_sqrt_3_over_3_prec_round_helper(10, Down, "0.577", "0x0.93c#10", Less);
    test_sqrt_3_over_3_prec_round_helper(10, Up, "0.578", "0x0.940#10", Greater);
    test_sqrt_3_over_3_prec_round_helper(10, Nearest, "0.577", "0x0.93c#10", Less);

    test_sqrt_3_over_3_prec_round_helper(
        100,
        Floor,
        "0.577350269189625764509148780501",
        "0x0.93cd3a2c8198e2690c7c0f257#100",
        Less,
    );
    test_sqrt_3_over_3_prec_round_helper(
        100,
        Ceiling,
        "0.577350269189625764509148780502",
        "0x0.93cd3a2c8198e2690c7c0f258#100",
        Greater,
    );
    test_sqrt_3_over_3_prec_round_helper(
        100,
        Down,
        "0.577350269189625764509148780501",
        "0x0.93cd3a2c8198e2690c7c0f257#100",
        Less,
    );
    test_sqrt_3_over_3_prec_round_helper(
        100,
        Up,
        "0.577350269189625764509148780502",
        "0x0.93cd3a2c8198e2690c7c0f258#100",
        Greater,
    );
    test_sqrt_3_over_3_prec_round_helper(
        100,
        Nearest,
        "0.577350269189625764509148780502",
        "0x0.93cd3a2c8198e2690c7c0f258#100",
        Greater,
    );
}

#[test]
#[should_panic]
fn sqrt_3_over_3_prec_round_fail_1() {
    Float::sqrt_3_over_3_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn sqrt_3_over_3_prec_round_fail_2() {
    Float::sqrt_3_over_3_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn sqrt_3_over_3_prec_round_fail_3() {
    Float::sqrt_3_over_3_prec_round(1000, Exact);
}

#[test]
fn sqrt_3_over_3_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (sqrt_3_over_3, o) = Float::sqrt_3_over_3_prec(prec);
        assert!(sqrt_3_over_3.is_valid());
        assert_eq!(sqrt_3_over_3.get_prec(), Some(prec));
        assert_eq!(sqrt_3_over_3.get_exponent(), Some(0));
        assert_ne!(o, Equal);
        if o == Less {
            let (sqrt_3_over_3_alt, o_alt) = Float::sqrt_3_over_3_prec_round(prec, Ceiling);
            let mut next_upper = sqrt_3_over_3.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(
                    ComparableFloat(sqrt_3_over_3_alt),
                    ComparableFloat(next_upper)
                );
                assert_eq!(o_alt, Greater);
            }
        } else if !sqrt_3_over_3.is_power_of_2() {
            let (sqrt_3_over_3_alt, o_alt) = Float::sqrt_3_over_3_prec_round(prec, Floor);
            let mut next_lower = sqrt_3_over_3.clone();
            next_lower.decrement();
            assert_eq!(
                ComparableFloat(sqrt_3_over_3_alt),
                ComparableFloat(next_lower)
            );
            assert_eq!(o_alt, Less);
        }
        let (sqrt_3_over_3_alt, o_alt) = Float::sqrt_3_over_3_prec_round(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&sqrt_3_over_3_alt),
            ComparableFloatRef(&sqrt_3_over_3)
        );
        assert_eq!(o_alt, o);

        let (rug_sqrt_3_over_3, rug_o) =
            rug_sqrt_3_over_3_prec_round(prec, rug_round_try_from_rounding_mode(Nearest).unwrap());
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_sqrt_3_over_3)),
            ComparableFloatRef(&sqrt_3_over_3)
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn sqrt_3_over_3_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (sqrt_3_over_3, o) = Float::sqrt_3_over_3_prec_round(prec, rm);
        assert!(sqrt_3_over_3.is_valid());
        assert_eq!(sqrt_3_over_3.get_prec(), Some(prec));
        assert_eq!(
            sqrt_3_over_3.get_exponent(),
            Some(if prec == 1 && (rm == Ceiling || rm == Up) {
                1
            } else {
                0
            })
        );
        assert_ne!(o, Equal);
        if o == Less {
            let (sqrt_3_over_3_alt, o_alt) = Float::sqrt_3_over_3_prec_round(prec, Ceiling);
            let mut next_upper = sqrt_3_over_3.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(
                    ComparableFloat(sqrt_3_over_3_alt),
                    ComparableFloat(next_upper)
                );
                assert_eq!(o_alt, Greater);
            }
        } else if !sqrt_3_over_3.is_power_of_2() {
            let (sqrt_3_over_3_alt, o_alt) = Float::sqrt_3_over_3_prec_round(prec, Floor);
            let mut next_lower = sqrt_3_over_3.clone();
            next_lower.decrement();
            assert_eq!(
                ComparableFloat(sqrt_3_over_3_alt),
                ComparableFloat(next_lower)
            );
            assert_eq!(o_alt, Less);
        }

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_sqrt_3_over_3, rug_o) = rug_sqrt_3_over_3_prec_round(prec, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_sqrt_3_over_3)),
                ComparableFloatRef(&sqrt_3_over_3)
            );
            assert_eq!(rug_o, o);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::sqrt_3_over_3_prec_round(prec, Exact));
    });

    test_constant(Float::sqrt_3_over_3_prec_round, 10000);
}
