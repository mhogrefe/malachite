// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::GelfondsConstant;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{test_constant, to_hex_string};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_gelfonds_constant_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::gelfonds_constant_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);
}

#[test]
pub fn test_gelfonds_constant_prec() {
    test_gelfonds_constant_prec_helper(1, "16.0", "0x1.0E+1#1", Less);
    test_gelfonds_constant_prec_helper(2, "24.0", "0x18.0#2", Greater);
    test_gelfonds_constant_prec_helper(3, "24.0", "0x18.0#3", Greater);
    test_gelfonds_constant_prec_helper(4, "24.0", "0x18.0#4", Greater);
    test_gelfonds_constant_prec_helper(5, "23.0", "0x17.0#5", Less);
    test_gelfonds_constant_prec_helper(6, "23.0", "0x17.0#6", Less);
    test_gelfonds_constant_prec_helper(7, "23.25", "0x17.4#7", Greater);
    test_gelfonds_constant_prec_helper(8, "23.12", "0x17.2#8", Less);
    test_gelfonds_constant_prec_helper(9, "23.12", "0x17.2#9", Less);
    test_gelfonds_constant_prec_helper(10, "23.156", "0x17.28#10", Greater);
    test_gelfonds_constant_prec_helper(
        100,
        "23.140692632779269005729086367940",
        "0x17.24046eb093399ecda7489f9a#100",
        Less,
    );

    test_gelfonds_constant_prec_helper(
        1000,
        "23.14069263277926900572908636794854738026610624260021199344504640952434235069045278351697\
        199706754921967595270480108777314442804441469383584471744587960984936532796586366924223026\
        899101374176468440141039518386847724306805958816244984449143096677841367163196341478403821\
        65112876377314703473538331628212929",
        "0x17.24046eb093399ecda7489f9ab7694d31fa196e922e92d71525f22b5462101f2fe65329dc0ccb33897389\
        efc492a27b2eedcb7d6314ad6a048f425828bd1354252136801ace3c0ddbb994283d926f3977fc1fc4e78d00a0\
        165822f8d6e69023e5f27556f1494741bfa2bf486b3c701cbfa86ff2bc1915b9f352ec67d80#1000",
        Less,
    );
    test_gelfonds_constant_prec_helper(
        10000,
        "23.14069263277926900572908636794854738026610624260021199344504640952434235069045278351697\
        199706754921967595270480108777314442804441469383584471744587960984936532796586366924223026\
        899101374176468440141039518386847724306805958816244984449143096677841367163196341478403821\
        651128763773147034735383316282129404789193622482022100603206544336273655727182374498961885\
        805959168487264547901339783402659510149964379242296816079956538142353620695760077059046089\
        988300225430487121179130084932737958072942730193104260169193932585320342896866189528329052\
        171115718518550680225419720456637086556838683054479927817040749776854036755653495721886788\
        256399438471822458588942853524726056821027107601849153451846806488738677443963051400516944\
        054066526543096886906393731535983731104217443302396789669003504118148605339028720375991858\
        688689748732432172158559607433467642616785611735333642126563191566545489228969224577388957\
        090536180383619751032656794362408835990642234712846533437314871706517894637427341269479680\
        432104147666823028642934446787458303621896352392132932437552023312634782441475080062783128\
        292715193651668309296252268297155212574109382863300341189714250703916188001555050380355384\
        371886160328888614425089988100319744833912230377252936172730757413436275904426995406289225\
        790875770529110148763026010205247800420627723513971950783044887623839816394117444682797473\
        080248658854369627196272388054661855341972937363736841914869102703532815372438674875511721\
        425831084741016922539843403898085349174038445072151140876121258119083213326492383559349175\
        416796228232219122801115327815520421901802590889176538395367450289090224797040969240578508\
        275103691356035442087170650862040710980760476496644983731616360345082793701653404409744164\
        202370341080612286153530513874618327401153045280107170901049021662163537764649903326493228\
        066278702876113222326880334273947975829783042817194068968326186918979220437234695267954251\
        078157169330375128609847471145764999385293955317299318522403635851666432346020669175389368\
        691242250703352067335863354735887582568507415384203309430889313134570571603308838153880050\
        445290786370398159817197092171790251188769672402863284170362567249479165132023889515459608\
        840805018629083858048103584308518959435853216816139639351780745406344633996874965017305777\
        774425811653546425773077142002945412340478784731624846674515236105088290570360449823037670\
        334024022650423538946590022309978334764961491035658229035655073176685397591191074620268890\
        838874513561540323928802222172545592051269104397447301379132164386204701151140195225233145\
        661609573011527519810579492138905758189960406959631201840034818116949390050063276292347524\
        327792193832821638502127016426368150289697069239711867591420434602520956737058748493968648\
        173683530989088617875705535009819023338259998045572785243439927128330568736470863977189362\
        176010012417529770566414179217936485567739690856358946541734849076896992356866164051822488\
        460309479056959102249586601201144560501387093048909238411447236603895750763668801624092787\
        43823737448149166506149644178354996904405785",
        "0x17.24046eb093399ecda7489f9ab7694d31fa196e922e92d71525f22b5462101f2fe65329dc0ccb33897389\
        efc492a27b2eedcb7d6314ad6a048f425828bd1354252136801ace3c0ddbb994283d926f3977fc1fc4e78d00a0\
        165822f8d6e69023e5f27556f1494741bfa2bf486b3c701cbfa86ff2bc1915b9f352ec67d80bf0790ed02d09ff\
        1bb2391008f6e1d20119b5776cc47bfcef8ad71ce016f13b710e049e9a00078cfa2151b076b7044b6bc705a917\
        15d171850453c283fc1efb3347583b1a0c81f714d28a713513d49da045d9b13345b23b8b09d3fda119abf78e32\
        27c5a24cf37a7118b6a0bc9d191af325ebc675577a7a8539710dcc106c05816c7d11359d44f588b0a50a1d9c50\
        92c61305bc12a8ed3e40e07efd5334fd099285eff2d5ba288e7f1f3f2394206c9f4bd30a6fc57f6a4f41a9f1c9\
        84b4f9667d54615aa36776b0e410ee860876b48bd9a9775a912228cfce9a1d5a047398698b474538bffe56375f\
        8123b8d21e3f98f8dbcd9b4aba6c4b06f486d05ab0c064c0ec7091e50bebb763066f178b11c270c69e425f426e\
        f4a94ec12c35cfd203a4d43ab03883aa728a3bc27b5d106904122f01a41dbad4ba216cf0ddcf4cdb7e003e1e81\
        891ff5420d63b7f84846c480eb1dad8f91225bcb7faf4c82a00a183073f517e7493214212c02fcea147ef3c7f7\
        e38005f2db5948a1989251e7464a7bc3f3ec834d0bd2f8529a2f47bb047e088c18b12770694fc5966bd7ee33cb\
        dcfd16ea2cc503887b4735c0531be9e281e0cd5df8e6db0537384bdb5deb7b617bbbc34881422fcbbfc6ff7acf\
        3a67b5c160b0258d973b503c8c0ab7281a614e763c390bf344b248b0f2037b9d0de5c24bbb5c8a006b359fd8b4\
        1c1b0da255cde73dface96483d33ad737febf197e66a3b99047cf62c41697add4b4a309e5e947adea5463402cf\
        54717a2ad895d75c18e16416d0fb2a372060e5a7001c56acb762403cc74ed8ebc20bd821aa32863e18d7c2917b\
        a83a9bb0443ea0aeb6e7972cb85920f70e738dacc3f052be55a025e60fac153ac5301cbdff8159f294e5cb8ca7\
        17790b0ee634b05280b7a5ee2978bb5709e2fffe9c76e6a0086637530e551235cd3af610c952d1c1407dd91436\
        f7a53d503d6dc4cff303ad89c5dfa45d628c13f665cb312752f48e84907a8dfda3ce33575a76a2b4cba87887c2\
        19eceac0fca31d98d1f2633ded43fd6aa755b271fbbe87073f7bd3edb33044bc9df4a2cf122074a0bbc1e069d0\
        f8edb48657ccea2eb0777c854d7987fda5f29aa76a91e5293634d6d9e099bf9737b55d5cabb61e8b5d0249fdb4\
        6defadafffae39852d45cee9326cde399be5962af5a6740ec74ea747633029383079f16f569e443818f6252870\
        0c35deed59c3282fc4ff16c08b23ef187da4da5a514952e0facb42340cc95ee072cd1dc1b6d75f5cad76d69c50\
        527cbf0bab1436ac601f329b23239272fa3fe135f6ce16d4e5c048c0cf47e3820eb053d0e3b1c9f95effbcc7c6\
        3e3cfb41d093a267f5bd8333f30a0b536a9ba1941743d3f40352f892a86e1525b002174b460fb0c5b241461b29\
        def962d1af1e1ecef4d39751836015d9be761440739a7677bfdd382f50e91de55cf54d7819f2435a805e33fa41\
        9171bf1617a73a44a6ec5c2444a9900bb47c20e62b169a766ce75aa4ccced6138576883427061e6624b97e1fb2\
        3ee2df7f70281420a9577b27bcdefa3181805b9428bd1c08da4348700844cbed2269afd16ba#10000",
        Greater,
    );
    let gelfonds_constant_f32 = Float::gelfonds_constant_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(gelfonds_constant_f32.to_string(), "23.1406918");
    assert_eq!(to_hex_string(&gelfonds_constant_f32), "0x17.24046#24");
    assert_eq!(gelfonds_constant_f32, f32::GELFONDS_CONSTANT);

    let gelfonds_constant_f64 = Float::gelfonds_constant_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(gelfonds_constant_f64.to_string(), "23.140692632779270");
    assert_eq!(
        to_hex_string(&gelfonds_constant_f64),
        "0x17.24046eb0933a#53"
    );
    assert_eq!(gelfonds_constant_f64, f64::GELFONDS_CONSTANT);
}

#[test]
#[should_panic]
fn gelfonds_constant_prec_fail_1() {
    Float::gelfonds_constant_prec(0);
}

fn test_gelfonds_constant_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::gelfonds_constant_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);
}

#[test]
pub fn test_gelfonds_constant_prec_round() {
    test_gelfonds_constant_prec_round_helper(1, Floor, "16.0", "0x1.0E+1#1", Less);
    test_gelfonds_constant_prec_round_helper(1, Ceiling, "32.0", "0x2.0E+1#1", Greater);
    test_gelfonds_constant_prec_round_helper(1, Down, "16.0", "0x1.0E+1#1", Less);
    test_gelfonds_constant_prec_round_helper(1, Up, "32.0", "0x2.0E+1#1", Greater);
    test_gelfonds_constant_prec_round_helper(1, Nearest, "16.0", "0x1.0E+1#1", Less);

    test_gelfonds_constant_prec_round_helper(2, Floor, "16.0", "0x10.0#2", Less);
    test_gelfonds_constant_prec_round_helper(2, Ceiling, "24.0", "0x18.0#2", Greater);
    test_gelfonds_constant_prec_round_helper(2, Down, "16.0", "0x10.0#2", Less);
    test_gelfonds_constant_prec_round_helper(2, Up, "24.0", "0x18.0#2", Greater);
    test_gelfonds_constant_prec_round_helper(2, Nearest, "24.0", "0x18.0#2", Greater);

    test_gelfonds_constant_prec_round_helper(3, Floor, "20.0", "0x14.0#3", Less);
    test_gelfonds_constant_prec_round_helper(3, Ceiling, "24.0", "0x18.0#3", Greater);
    test_gelfonds_constant_prec_round_helper(3, Down, "20.0", "0x14.0#3", Less);
    test_gelfonds_constant_prec_round_helper(3, Up, "24.0", "0x18.0#3", Greater);
    test_gelfonds_constant_prec_round_helper(3, Nearest, "24.0", "0x18.0#3", Greater);

    test_gelfonds_constant_prec_round_helper(4, Floor, "22.0", "0x16.0#4", Less);
    test_gelfonds_constant_prec_round_helper(4, Ceiling, "24.0", "0x18.0#4", Greater);
    test_gelfonds_constant_prec_round_helper(4, Down, "22.0", "0x16.0#4", Less);
    test_gelfonds_constant_prec_round_helper(4, Up, "24.0", "0x18.0#4", Greater);
    test_gelfonds_constant_prec_round_helper(4, Nearest, "24.0", "0x18.0#4", Greater);

    test_gelfonds_constant_prec_round_helper(5, Floor, "23.0", "0x17.0#5", Less);
    test_gelfonds_constant_prec_round_helper(5, Ceiling, "24.0", "0x18.0#5", Greater);
    test_gelfonds_constant_prec_round_helper(5, Down, "23.0", "0x17.0#5", Less);
    test_gelfonds_constant_prec_round_helper(5, Up, "24.0", "0x18.0#5", Greater);
    test_gelfonds_constant_prec_round_helper(5, Nearest, "23.0", "0x17.0#5", Less);

    test_gelfonds_constant_prec_round_helper(6, Floor, "23.0", "0x17.0#6", Less);
    test_gelfonds_constant_prec_round_helper(6, Ceiling, "23.5", "0x17.8#6", Greater);
    test_gelfonds_constant_prec_round_helper(6, Down, "23.0", "0x17.0#6", Less);
    test_gelfonds_constant_prec_round_helper(6, Up, "23.5", "0x17.8#6", Greater);
    test_gelfonds_constant_prec_round_helper(6, Nearest, "23.0", "0x17.0#6", Less);

    test_gelfonds_constant_prec_round_helper(7, Floor, "23.00", "0x17.0#7", Less);
    test_gelfonds_constant_prec_round_helper(7, Ceiling, "23.25", "0x17.4#7", Greater);
    test_gelfonds_constant_prec_round_helper(7, Down, "23.00", "0x17.0#7", Less);
    test_gelfonds_constant_prec_round_helper(7, Up, "23.25", "0x17.4#7", Greater);
    test_gelfonds_constant_prec_round_helper(7, Nearest, "23.25", "0x17.4#7", Greater);

    test_gelfonds_constant_prec_round_helper(8, Floor, "23.12", "0x17.2#8", Less);
    test_gelfonds_constant_prec_round_helper(8, Ceiling, "23.25", "0x17.4#8", Greater);
    test_gelfonds_constant_prec_round_helper(8, Down, "23.12", "0x17.2#8", Less);
    test_gelfonds_constant_prec_round_helper(8, Up, "23.25", "0x17.4#8", Greater);
    test_gelfonds_constant_prec_round_helper(8, Nearest, "23.12", "0x17.2#8", Less);

    test_gelfonds_constant_prec_round_helper(9, Floor, "23.12", "0x17.2#9", Less);
    test_gelfonds_constant_prec_round_helper(9, Ceiling, "23.19", "0x17.3#9", Greater);
    test_gelfonds_constant_prec_round_helper(9, Down, "23.12", "0x17.2#9", Less);
    test_gelfonds_constant_prec_round_helper(9, Up, "23.19", "0x17.3#9", Greater);
    test_gelfonds_constant_prec_round_helper(9, Nearest, "23.12", "0x17.2#9", Less);

    test_gelfonds_constant_prec_round_helper(10, Floor, "23.125", "0x17.20#10", Less);
    test_gelfonds_constant_prec_round_helper(10, Ceiling, "23.156", "0x17.28#10", Greater);
    test_gelfonds_constant_prec_round_helper(10, Down, "23.125", "0x17.20#10", Less);
    test_gelfonds_constant_prec_round_helper(10, Up, "23.156", "0x17.28#10", Greater);
    test_gelfonds_constant_prec_round_helper(10, Nearest, "23.156", "0x17.28#10", Greater);

    test_gelfonds_constant_prec_round_helper(
        100,
        Floor,
        "23.140692632779269005729086367940",
        "0x17.24046eb093399ecda7489f9a#100",
        Less,
    );
    test_gelfonds_constant_prec_round_helper(
        100,
        Ceiling,
        "23.140692632779269005729086367965",
        "0x17.24046eb093399ecda7489f9c#100",
        Greater,
    );
    test_gelfonds_constant_prec_round_helper(
        100,
        Down,
        "23.140692632779269005729086367940",
        "0x17.24046eb093399ecda7489f9a#100",
        Less,
    );
    test_gelfonds_constant_prec_round_helper(
        100,
        Up,
        "23.140692632779269005729086367965",
        "0x17.24046eb093399ecda7489f9c#100",
        Greater,
    );
    test_gelfonds_constant_prec_round_helper(
        100,
        Nearest,
        "23.140692632779269005729086367940",
        "0x17.24046eb093399ecda7489f9a#100",
        Less,
    );
}

#[test]
#[should_panic]
fn gelfonds_constant_prec_round_fail_1() {
    Float::gelfonds_constant_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn gelfonds_constant_prec_round_fail_2() {
    Float::gelfonds_constant_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn gelfonds_constant_prec_round_fail_3() {
    Float::gelfonds_constant_prec_round(1000, Exact);
}

#[test]
fn gelfonds_constant_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (gelfonds_constant, o) = Float::gelfonds_constant_prec(prec);
        assert!(gelfonds_constant.is_valid());
        assert_eq!(gelfonds_constant.get_prec(), Some(prec));
        assert_eq!(gelfonds_constant.get_exponent(), Some(5));
        assert_ne!(o, Equal);
        if o == Less {
            let (gelfonds_constant_alt, o_alt) = Float::gelfonds_constant_prec_round(prec, Ceiling);
            let mut next_upper = gelfonds_constant.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(
                    ComparableFloat(gelfonds_constant_alt),
                    ComparableFloat(next_upper)
                );
                assert_eq!(o_alt, Greater);
            }
        } else if !gelfonds_constant.is_power_of_2() {
            let (gelfonds_constant_alt, o_alt) = Float::gelfonds_constant_prec_round(prec, Floor);
            let mut next_lower = gelfonds_constant.clone();
            next_lower.decrement();
            assert_eq!(
                ComparableFloat(gelfonds_constant_alt),
                ComparableFloat(next_lower)
            );
            assert_eq!(o_alt, Less);
        }
        let (gelfonds_constant_alt, o_alt) = Float::gelfonds_constant_prec_round(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&gelfonds_constant_alt),
            ComparableFloatRef(&gelfonds_constant)
        );
        assert_eq!(o_alt, o);
    });
}

#[test]
fn gelfonds_constant_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (gelfonds_constant, o) = Float::gelfonds_constant_prec_round(prec, rm);
        assert!(gelfonds_constant.is_valid());
        assert_eq!(gelfonds_constant.get_prec(), Some(prec));
        // e^pi is in [16, 32), so the result has exponent 5 unless it rounds up to 32.
        let expected_exponent = match (prec, rm) {
            (1, Ceiling | Up) => 6,
            _ => 5,
        };
        assert_eq!(gelfonds_constant.get_exponent(), Some(expected_exponent));
        assert_ne!(o, Equal);
        if o == Less {
            let (gelfonds_constant_alt, o_alt) = Float::gelfonds_constant_prec_round(prec, Ceiling);
            let mut next_upper = gelfonds_constant.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(
                    ComparableFloat(gelfonds_constant_alt),
                    ComparableFloat(next_upper)
                );
                assert_eq!(o_alt, Greater);
            }
        } else if !gelfonds_constant.is_power_of_2() {
            let (gelfonds_constant_alt, o_alt) = Float::gelfonds_constant_prec_round(prec, Floor);
            let mut next_lower = gelfonds_constant.clone();
            next_lower.decrement();
            assert_eq!(
                ComparableFloat(gelfonds_constant_alt),
                ComparableFloat(next_lower)
            );
            assert_eq!(o_alt, Less);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::gelfonds_constant_prec_round(prec, Exact));
    });

    test_constant(Float::gelfonds_constant_prec_round, 10000);
}
