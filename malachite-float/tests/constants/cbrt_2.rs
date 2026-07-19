// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::arithmetic::root::primitive_float_root_u;
use malachite_float::test_util::common::{
    rug_round_try_from_rounding_mode, test_constant, to_hex_string,
};
use malachite_float::test_util::constants::cbrt_2::rug_cbrt_2_prec_round;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_cbrt_2_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::cbrt_2_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (rug_x, rug_o) =
        rug_cbrt_2_prec_round(prec, rug_round_try_from_rounding_mode(Nearest).unwrap());
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_x)),
        ComparableFloatRef(&x)
    );
    assert_eq!(rug_o, o);
}

#[test]
pub fn test_cbrt_2_prec() {
    test_cbrt_2_prec_helper(1, "1.0", "0x1.0#1", Less);
    test_cbrt_2_prec_helper(2, "1.5", "0x1.8#2", Greater);
    test_cbrt_2_prec_helper(3, "1.2", "0x1.4#3", Less);
    test_cbrt_2_prec_helper(4, "1.25", "0x1.4#4", Less);
    test_cbrt_2_prec_helper(5, "1.25", "0x1.4#5", Less);
    test_cbrt_2_prec_helper(6, "1.25", "0x1.40#6", Less);
    test_cbrt_2_prec_helper(7, "1.266", "0x1.44#7", Greater);
    test_cbrt_2_prec_helper(8, "1.258", "0x1.42#8", Less);
    test_cbrt_2_prec_helper(9, "1.262", "0x1.43#9", Greater);
    test_cbrt_2_prec_helper(10, "1.2598", "0x1.428#10", Less);
    test_cbrt_2_prec_helper(
        100,
        "1.2599210498948731647672106072783",
        "0x1.428a2f98d728ae223ddab715c#100",
        Greater,
    );
    test_cbrt_2_prec_helper(
        1000,
        "1.259921049894873164767210607278228350570251464701507980081975112155299676513959483729396\
        562436255094154310256035615665259399024040613737228459110304269355246960642616625000977474\
        526565480306867185405518689245872516764199373709695098382783161399155129313695366183947463\
        44857657030311909589598474110598107",
        "0x1.428a2f98d728ae223ddab715be250d0c288f10291631fbc061800cc36fa2dd3a60b7d03da26f0840f25cc\
        16b0f95075c30967a6b5ac6827ab31f06edf4a3487081621806c6b0920d371a0053d2fe49d4ddd52be49cc3c86\
        14c865b771ec003a0dce3c57c1bfbdf43d747b840a80e6b17be47fd1b669f88963a78a3aeb0#1000",
        Less,
    );
    test_cbrt_2_prec_helper(
        10000,
        "1.259921049894873164767210607278228350570251464701507980081975112155299676513959483729396\
        562436255094154310256035615665259399024040613737228459110304269355246960642616625000977474\
        526565480306867185405518689245872516764199373709695098382783161399155129313695366183947463\
        448576570303119095895984741105981162907053590816478011473521325484771297880242208582053257\
        972526662202669005665608199471562817640506066482677357267041948620762144296569420507931917\
        244148092044823284012747032196428208120190571418899645999831750380188868959420205592202115\
        472997384880260736369741788779215798467509953963007826095962420348323866013985736343390973\
        712652799599196996837791316816815442885027965152927810767971400204060567480393856125171835\
        700690798499634197629147404483454026971547622851317802064387804764932257905289846708580528\
        625813000542938856072060974722304063135723493645840657591691691672706012440289670000106908\
        103531385290270041508423233623988938649678219414983802707295717681287900144574622714770234\
        835715190550672208481848500928723920928264660671717424775370973703001274291809405442569659\
        207503635757037518960370747399346101449014515763596047111197384529913296572625890486097885\
        618013867738361577300986598366080597575601278712148685624268455641165155817935322801589629\
        129944500401208425414160157525841629881423097358215306040577242538364532533565955117252285\
        579562277240366562846875901543066753519085484511818175204291241233780963172521357541141811\
        466127366045783036057440265130960709681640068881856572310090084284526086414059503369003079\
        186993556913351834285693826255431355897354450233302853149322455134121955457821196500833957\
        714266850633284196196865121092557895588508996861901546700438968786655453098545057637650360\
        089433065103569357775372495484368213703171621621834958093562087260096267851834183456522397\
        445400044760217788942081838027866653065326632618641160074007474754735585277016895020637541\
        322323296942437017423434916176906007238539022276811297774138720798234303910316285464520831\
        111225468283531830470613350996299644553317659489214051165570188505816337759025958459804954\
        207116242949057680526343989563098840948676345728459791892101766948457248520366994268712410\
        121775277712917316420384063490921165834272096177151409399524727550256541206956883979739039\
        050328337538665635436780931508621222709639685474010272293755350110622620742504810764026657\
        891889273399055910808781239396445781159916798188575061726269178320334090130419077072717829\
        496457167600719020576195215315246386367321302738318790366955893425062362570657996711791354\
        312373563603076243394093802780847716478898902529065742892077837823177086899890726460960715\
        082065910698646237116601885470807716422828403838636766541958725106267326588663504087641031\
        952538583385388576087728395489306437438979807835834449139190339547132068082078908990913108\
        486410647047795570845898857484858727228437184120073544794473152128794407364750342346723174\
        986125696712349795418169252652549838100098101177829499369705040110300480631571340262578601\
        40247662483603681916526298276852880134669379",
        "0x1.428a2f98d728ae223ddab715be250d0c288f10291631fbc061800cc36fa2dd3a60b7d03da26f0840f25cc\
        16b0f95075c30967a6b5ac6827ab31f06edf4a3487081621806c6b0920d371a0053d2fe49d4ddd52be49cc3c86\
        14c865b771ec003a0dce3c57c1bfbdf43d747b840a80e6b17be47fd1b669f88963a78a3aeb0f203190956d75c5\
        6ea1d1130bf996f19f57700d75db34ce62922449d3c832bce3b0dd1a350ec848fff8beb6529e36b13007cf3e2d\
        dc15cd01ebc1eca20e6f14b34e369b35d7cd922bf3d35b799b082a5daef2476f31b56a6817b091fc97f620d71c\
        b6ec1a5684621a5de77928e529f906227637639fdfc7f3ebc8fd38c0e244e552e8fc5383b34ab212433929e2c1\
        662340aaceed5ab723fffa2abf1815259dc94f2c88944df1e65d74e7d68bb7d0795b4fa8184667dcd99557750f\
        8d90bedb8db9d42f3a859107ea904b91f5771ae7b9c43e609be02f87fbad4a413c4eeef11d063327310138fe9a\
        29a856902d586ac29b077ebe8d96246707bb862e045c8d9a0211162d1ac855f8143290a3d865db1174fd1f3fba\
        7ced00a03c7650920e664b5dce7da136e8aa39954f84a8f961e39a3d27004e623ddfb93ad47f181a40afd739ea\
        7e33fef27bb5c60f3485b34ef2656a770513db1c4b1745409a8981317347d11150c86cc6345ad183bb00c307d3\
        62861dca8c48cd8d240c56ba126043fa54890e71fef8edf8a4a393447b0887eef2392a14a1852a65fd89b54d27\
        97fb65e9cdb53cd8ed333feef71db2c75c2472f03da08b6965a743690a2c95d97f86b295447583450b3099fc47\
        96d7a1cc580941d4f53b6ac8e294c27f038be4eb61b553368437f746ea1df50ba238d3c33181ec088bab5bc34d\
        021fa4f53c2a214f4241290f1c9adddd21755066cf014ad371430c305d1bd3e8003fbe02126540f36911b5018e\
        9e107b6112a30143306844fd3b34feb321d6195152c1258a3a7518e8075eed807359aa36f0e8790e1301220014\
        41ac1320458dd91955c4b5642baad4c6844fc732326923be553142203301a35edeeb664df5e79cf95577a9f7f1\
        bec06651c4a1b43647dcc30c2aec64867c35928c7cf8f50f2305ee12f797b7b06ddc856ad8236fa456107b9a0f\
        fa2388fb4cc669560dac52b743c113597699b4f62fd1c212c7a1130ad4ea60a8b89f74dede03c26bc4ba9afd0d\
        5ab3553d809aea7a925d053843f8d7d64e784c880eab1076f752d2ab81475b080d47eb5377adf13ad176d51044\
        60a2571078f80ce35d3c881667cfb7aad3c2b6e3e02af52d5687372268c8aaab4f9a1826a1b64f434d29e5a5b3\
        19052e582a0323ec8627355064e1bdbce037ba27d01eb61c794a6568a435a1e8cdba6aef051655c794188aecc4\
        6786339d238db4b77bd7a338061816e6c9dff8fe11f2a9e1406aba878ad1d179f57aaefb9944020a02bf022ee9\
        bea40dac7a91c282da763315c862495d6f4be456be3e0259fc329c7b7c03c0db344ad6c1482b2d43fbcaf4992d\
        5bda80cb02d337782884e4199abdb0492eda588d0be8cb062113371ea274233bda6337820b7285fdb8c95d9e15\
        c68d774377f3fa81f9c81473a1abf2625b9042416621a2370a6967a39face78f43914fba4c20f47010e1c9f682\
        a8eb381e05d9d8a199bb3ed645e84b925f114c42e14ec0853669475c26aace80cf508d4fb9d19b12b851e5f952\
        eb13f56397d974ca8098ee64997d1492bf369ce0eca470e191115803ad43bf4ac07107df18c#10000",
        Greater,
    );

    let cbrt_2_f32 = Float::cbrt_2_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(cbrt_2_f32.to_string(), "1.25992107");
    assert_eq!(to_hex_string(&cbrt_2_f32), "0x1.428a30#24");
    assert_eq!(cbrt_2_f32, primitive_float_root_u(2.0f32, 3));

    let cbrt_2_f64 = Float::cbrt_2_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(cbrt_2_f64.to_string(), "1.2599210498948732");
    assert_eq!(to_hex_string(&cbrt_2_f64), "0x1.428a2f98d728b#53");
    assert_eq!(cbrt_2_f64, primitive_float_root_u(2.0f64, 3));
}

#[test]
#[should_panic]
fn cbrt_2_prec_fail_1() {
    Float::cbrt_2_prec(0);
}

fn test_cbrt_2_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::cbrt_2_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_x, rug_o) = rug_cbrt_2_prec_round(prec, rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_x)),
            ComparableFloatRef(&x)
        );
        assert_eq!(rug_o, o);
    }
}

#[test]
pub fn test_cbrt_2_prec_round() {
    test_cbrt_2_prec_round_helper(1, Floor, "1.0", "0x1.0#1", Less);
    test_cbrt_2_prec_round_helper(1, Ceiling, "2.0", "0x2.0#1", Greater);
    test_cbrt_2_prec_round_helper(1, Down, "1.0", "0x1.0#1", Less);
    test_cbrt_2_prec_round_helper(1, Up, "2.0", "0x2.0#1", Greater);
    test_cbrt_2_prec_round_helper(1, Nearest, "1.0", "0x1.0#1", Less);

    test_cbrt_2_prec_round_helper(2, Floor, "1.0", "0x1.0#2", Less);
    test_cbrt_2_prec_round_helper(2, Ceiling, "1.5", "0x1.8#2", Greater);
    test_cbrt_2_prec_round_helper(2, Down, "1.0", "0x1.0#2", Less);
    test_cbrt_2_prec_round_helper(2, Up, "1.5", "0x1.8#2", Greater);
    test_cbrt_2_prec_round_helper(2, Nearest, "1.5", "0x1.8#2", Greater);

    test_cbrt_2_prec_round_helper(3, Floor, "1.2", "0x1.4#3", Less);
    test_cbrt_2_prec_round_helper(3, Ceiling, "1.5", "0x1.8#3", Greater);
    test_cbrt_2_prec_round_helper(3, Down, "1.2", "0x1.4#3", Less);
    test_cbrt_2_prec_round_helper(3, Up, "1.5", "0x1.8#3", Greater);
    test_cbrt_2_prec_round_helper(3, Nearest, "1.2", "0x1.4#3", Less);

    test_cbrt_2_prec_round_helper(4, Floor, "1.25", "0x1.4#4", Less);
    test_cbrt_2_prec_round_helper(4, Ceiling, "1.38", "0x1.6#4", Greater);
    test_cbrt_2_prec_round_helper(4, Down, "1.25", "0x1.4#4", Less);
    test_cbrt_2_prec_round_helper(4, Up, "1.38", "0x1.6#4", Greater);
    test_cbrt_2_prec_round_helper(4, Nearest, "1.25", "0x1.4#4", Less);

    test_cbrt_2_prec_round_helper(5, Floor, "1.25", "0x1.4#5", Less);
    test_cbrt_2_prec_round_helper(5, Ceiling, "1.31", "0x1.5#5", Greater);
    test_cbrt_2_prec_round_helper(5, Down, "1.25", "0x1.4#5", Less);
    test_cbrt_2_prec_round_helper(5, Up, "1.31", "0x1.5#5", Greater);
    test_cbrt_2_prec_round_helper(5, Nearest, "1.25", "0x1.4#5", Less);

    test_cbrt_2_prec_round_helper(6, Floor, "1.25", "0x1.40#6", Less);
    test_cbrt_2_prec_round_helper(6, Ceiling, "1.28", "0x1.48#6", Greater);
    test_cbrt_2_prec_round_helper(6, Down, "1.25", "0x1.40#6", Less);
    test_cbrt_2_prec_round_helper(6, Up, "1.28", "0x1.48#6", Greater);
    test_cbrt_2_prec_round_helper(6, Nearest, "1.25", "0x1.40#6", Less);

    test_cbrt_2_prec_round_helper(7, Floor, "1.250", "0x1.40#7", Less);
    test_cbrt_2_prec_round_helper(7, Ceiling, "1.266", "0x1.44#7", Greater);
    test_cbrt_2_prec_round_helper(7, Down, "1.250", "0x1.40#7", Less);
    test_cbrt_2_prec_round_helper(7, Up, "1.266", "0x1.44#7", Greater);
    test_cbrt_2_prec_round_helper(7, Nearest, "1.266", "0x1.44#7", Greater);

    test_cbrt_2_prec_round_helper(8, Floor, "1.258", "0x1.42#8", Less);
    test_cbrt_2_prec_round_helper(8, Ceiling, "1.266", "0x1.44#8", Greater);
    test_cbrt_2_prec_round_helper(8, Down, "1.258", "0x1.42#8", Less);
    test_cbrt_2_prec_round_helper(8, Up, "1.266", "0x1.44#8", Greater);
    test_cbrt_2_prec_round_helper(8, Nearest, "1.258", "0x1.42#8", Less);

    test_cbrt_2_prec_round_helper(9, Floor, "1.258", "0x1.42#9", Less);
    test_cbrt_2_prec_round_helper(9, Ceiling, "1.262", "0x1.43#9", Greater);
    test_cbrt_2_prec_round_helper(9, Down, "1.258", "0x1.42#9", Less);
    test_cbrt_2_prec_round_helper(9, Up, "1.262", "0x1.43#9", Greater);
    test_cbrt_2_prec_round_helper(9, Nearest, "1.262", "0x1.43#9", Greater);

    test_cbrt_2_prec_round_helper(10, Floor, "1.2598", "0x1.428#10", Less);
    test_cbrt_2_prec_round_helper(10, Ceiling, "1.2617", "0x1.430#10", Greater);
    test_cbrt_2_prec_round_helper(10, Down, "1.2598", "0x1.428#10", Less);
    test_cbrt_2_prec_round_helper(10, Up, "1.2617", "0x1.430#10", Greater);
    test_cbrt_2_prec_round_helper(10, Nearest, "1.2598", "0x1.428#10", Less);

    test_cbrt_2_prec_round_helper(
        100,
        Floor,
        "1.2599210498948731647672106072767",
        "0x1.428a2f98d728ae223ddab715a#100",
        Less,
    );
    test_cbrt_2_prec_round_helper(
        100,
        Ceiling,
        "1.2599210498948731647672106072783",
        "0x1.428a2f98d728ae223ddab715c#100",
        Greater,
    );
    test_cbrt_2_prec_round_helper(
        100,
        Down,
        "1.2599210498948731647672106072767",
        "0x1.428a2f98d728ae223ddab715a#100",
        Less,
    );
    test_cbrt_2_prec_round_helper(
        100,
        Up,
        "1.2599210498948731647672106072783",
        "0x1.428a2f98d728ae223ddab715c#100",
        Greater,
    );
    test_cbrt_2_prec_round_helper(
        100,
        Nearest,
        "1.2599210498948731647672106072783",
        "0x1.428a2f98d728ae223ddab715c#100",
        Greater,
    );
}

#[test]
#[should_panic]
fn cbrt_2_prec_round_fail_1() {
    Float::cbrt_2_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn cbrt_2_prec_round_fail_2() {
    Float::cbrt_2_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn cbrt_2_prec_round_fail_3() {
    Float::cbrt_2_prec_round(1000, Exact);
}

#[test]
fn cbrt_2_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (cbrt_2, o) = Float::cbrt_2_prec(prec);
        assert!(cbrt_2.is_valid());
        assert_eq!(cbrt_2.get_prec(), Some(prec));
        assert_eq!(cbrt_2.get_exponent(), Some(1));
        assert_ne!(o, Equal);
        if o == Less {
            let (cbrt_2_alt, o_alt) = Float::cbrt_2_prec_round(prec, Ceiling);
            let mut next_upper = cbrt_2.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(cbrt_2_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !cbrt_2.is_power_of_2() {
            let (cbrt_2_alt, o_alt) = Float::cbrt_2_prec_round(prec, Floor);
            let mut next_lower = cbrt_2.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(cbrt_2_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
        let (cbrt_2_alt, o_alt) = Float::cbrt_2_prec_round(prec, Nearest);
        assert_eq!(ComparableFloatRef(&cbrt_2_alt), ComparableFloatRef(&cbrt_2));
        assert_eq!(o_alt, o);

        let (rug_cbrt_2, rug_o) =
            rug_cbrt_2_prec_round(prec, rug_round_try_from_rounding_mode(Nearest).unwrap());
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_cbrt_2)),
            ComparableFloatRef(&cbrt_2)
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn cbrt_2_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (cbrt_2, o) = Float::cbrt_2_prec_round(prec, rm);
        assert!(cbrt_2.is_valid());
        assert_eq!(cbrt_2.get_prec(), Some(prec));
        assert_eq!(
            cbrt_2.get_exponent(),
            Some(if prec == 1 && (rm == Ceiling || rm == Up) {
                2
            } else {
                1
            })
        );
        assert_ne!(o, Equal);
        if o == Less {
            let (cbrt_2_alt, o_alt) = Float::cbrt_2_prec_round(prec, Ceiling);
            let mut next_upper = cbrt_2.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(cbrt_2_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !cbrt_2.is_power_of_2() {
            let (cbrt_2_alt, o_alt) = Float::cbrt_2_prec_round(prec, Floor);
            let mut next_lower = cbrt_2.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(cbrt_2_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_cbrt_2, rug_o) = rug_cbrt_2_prec_round(prec, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_cbrt_2)),
                ComparableFloatRef(&cbrt_2)
            );
            assert_eq!(rug_o, o);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::cbrt_2_prec_round(prec, Exact));
    });

    test_constant(Float::cbrt_2_prec_round, 10000);
}
