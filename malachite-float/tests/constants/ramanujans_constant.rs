// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::RamanujansConstant;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{test_constant, to_hex_string};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_ramanujans_constant_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::ramanujans_constant_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);
}

#[test]
pub fn test_ramanujans_constant_prec() {
    test_ramanujans_constant_prec_helper(1, "2.9e17", "0x4.0E+14#1", Greater);
    test_ramanujans_constant_prec_helper(2, "2.9e17", "0x4.0E+14#2", Greater);
    test_ramanujans_constant_prec_helper(3, "2.5e17", "0x3.8E+14#3", Less);
    test_ramanujans_constant_prec_helper(4, "2.70e17", "0x3.cE+14#4", Greater);
    test_ramanujans_constant_prec_helper(5, "2.61e17", "0x3.aE+14#5", Less);
    test_ramanujans_constant_prec_helper(6, "2.61e17", "0x3.aE+14#6", Less);
    test_ramanujans_constant_prec_helper(7, "2.635e17", "0x3.a8E+14#7", Greater);
    test_ramanujans_constant_prec_helper(8, "2.623e17", "0x3.a4E+14#8", Less);
    test_ramanujans_constant_prec_helper(9, "2.623e17", "0x3.a4E+14#9", Less);
    test_ramanujans_constant_prec_helper(10, "2.6262e17", "0x3.a5E+14#10", Greater);
    test_ramanujans_constant_prec_helper(
        100,
        "262537412640768743.99999999999932",
        "0x3a4b862c4b402e7.ffffffffff4#100",
        Greater,
    );
    test_ramanujans_constant_prec_helper(
        1000,
        "262537412640768743.9999999999992500725971981856888793538563373369908627075374103782106479\
        101186073129511813461860645041930838879497538640449057287144771968148523224320391164782914\
        886422827201311783170650104522268780144484177034696946335570768172388768100092370653951938\
        65063627576578885582239481142769130",
        "0x3a4b862c4b402e7.ffffffffff2cea09206c58b2d1ca4e519d7f66bb8691a3762cba0a6aceb7e9258d991bb\
        1d5095a089378a623790a612c091cd2881b186308b069f28ca6f648af42c404a81b21ccb48a1a9342da5ff2047\
        2a7d697610a633170c0a991a65fdf75f351ac20aab506a92ae858e97c7af8f523103543c17c#1000",
        Greater,
    );
    test_ramanujans_constant_prec_helper(
        10000,
        "262537412640768743.9999999999992500725971981856888793538563373369908627075374103782106479\
        101186073129511813461860645041930838879497538640449057287144771968148523224320391164782914\
        886422827201311783170650104522268780144484177034696946335570768172388768100092370653951938\
        650636275765788855822394811427691210083088665110728471062346581129818301245913283610006498\
        266592365172617883086371078645219552815427466510961100147250209790463938177871257500980365\
        779223064312165113108738059929824233558494561239956769997843596486409600326648244352130649\
        159930327053075325656861838826548330980284669624287388475184443683853073411504446947884005\
        946446913168212059294605454216375489189006015035687286293314006363226814635161216376486413\
        142934235160021418051352828773196017981391788440715066299491909349627739620723413530255757\
        818028118021020634097499392383729033036173981663360032261262088666411718053832855897000273\
        572264523328701064958636772669868738485916569826626174198855115684430332735123103243307572\
        733164953615262048268479830605398100315775980251114459577418359648909422020347719677848308\
        224500701911820610847877622573587858440231909195321642076341400568039943154652667379435021\
        699213474771326112851913317849160665806840348978781443132267941083951936026502896072653729\
        127622693824271755127827965375070078400119001924171335832713470151875695231895057752289614\
        968282165078216685560521862228376151104529070465198135062406401569955505560772352723589835\
        926799382090532418405891274480143947457095064758655519475606634710797836661292764792090968\
        790313186555428273206260659324841326152370589009827537071537363077258081275582692087259158\
        190200503975119272628142051529584828462860484071480674993375689754816989791166125032073839\
        963294719747506608074391228225161029871531215392867328905645516851109451085024186881335775\
        393831998875131625734479994110811874009677068257745095059279517790053422922762513515767139\
        335255350869819364953815338823987075967976476825091344242721153756294609357278002807451188\
        973584431225994073585639937178482999019455197453216330691894985428497108376036006088281142\
        358970010776438775787338789506927974059618510030176331055733786186789238723504548703620375\
        534878041637366435761803513284257042277082795793559561740617723075535097271865491795833260\
        420705412631518615493490778827289893046588024881551068362451062692196559941008962328222320\
        186125967905769981531677723342367740913687912670785481270636628254240029955507859313928331\
        785279580112213820854062652731129732781256987562020396980309871413981173122272065819084970\
        477525183568120917096914644562495538767166940341409979580481054165989157336670072695476032\
        220847827954505380838224802227696669876222593469145527179487040465499553485002957732099916\
        663231068725820594375782665630645092618726673113428285093066405798758727962135070191510824\
        311376615161961330343357722688150780339616565636676188564495279356510031365413512680600769\
        856385813896652089834655566441369276451782847707309141450280537986608302380855821335675523\
        25827810870152478104518731920176125106167383",
        "0x3a4b862c4b402e7.ffffffffff2cea09206c58b2d1ca4e519d7f66bb8691a3762cba0a6aceb7e9258d991bb\
        1d5095a089378a623790a612c091cd2881b186308b069f28ca6f648af42c404a81b21ccb48a1a9342da5ff2047\
        2a7d697610a633170c0a991a65fdf75f351ac20aab506a92ae858e97c7af8f523103543c17aac4625b53e25501\
        6c2dc95d70783bea183f5d82015b6bdb93d4e0e86db4a45c6eca52627dee845d53da5f3f43caa141efb9b5084b\
        546f44b44798edd8d857fad5800392cd35030f900c8865fc2a312c81bdccba4195044573884c8866dde8714c0d\
        17b4bd7b6d334b20e7d0316e0eefece99e6c870963478b9d969642828e1c6630f7c3ccd6236b2d69678d4119c7\
        80ad00a0a5041d55b45151416bf975cd73d3a3d2b731915d8d9499539aa09bf2fa4e1cc4cc693790b60b286e35\
        25f130d15ae9f7ec3454911efa776bca6e35e3fbbe7e45f08443b1f0112f6feb1c566f83bad5407a4566703606\
        afd78e87ce74498e37b56d2504bdd72d3ed4857e9ecbe7c69077a20fb948e08d3ddfa5fa56af2fd8758c764ed1\
        dcf65b37eacce4a3707c59bbba24f7bd5b31512d5a0c8cbebfe3f7b008fb448cfb7dca467908cbefeb29ed96b4\
        bbcb6482d2ce8b76f6cb73e24757fcca8ef7227c45fcb636d362d95992b4bcdbc8dd5f731d0841fc4afebce004\
        020dd47cba5b27f34d1fd399acbec03c3bb328fa46503b5a45000704726b55334f81f5a53bfdd1d001100600a6\
        537cd0457900276a8e2484489412203adedbd9c6cd0554bb08b483e7d9257097ab2e5c7f58ba781df3129069d7\
        60cd1401b64f47dbe5ecc1e1a6019555e36abc0d2407d12733c642a8e5a8edd6aca0e6f6e2a6da61fc95d550ad\
        aece18059c84c8d3d2cf832745a03fd7b683287f266491a788fd3aea8b5a276d877da6607ef271791dbe6eb466\
        d016488a2def3e0c125aad5104951b80a4be4cc6f46e5acd43935844600c40febbb4bf8f3abd4d59bf4ed57ca5\
        e1e910ad22760a1bee8cb1ca991df8140b49130c846ea9f2877f5e006913615c53a45d31d45b2fb9a3512e938d\
        b70b0e73601c14a72be563804d89b63ec11d31fa96aa971ec53189aa3b490cfb6cae7652347f65074721cf3e74\
        8d13faca8ef41c36b275f69d9f82012c703c06b5f91bcd3f04acbcd8373c870b709c891457cce475a4c0499807\
        df71d4b70ffc5293cb84d852f4b7b6db0477397084f2c138f0783589a0323369f6dff744f7ae3bc233a29cd1f5\
        510c373e5892abd632bcae406dc9115f3e8a7de7d1ccd02d9b63489beb8a5060e807614dd44c34f55481ff657a\
        4bde0312350d095271b63f550261ee7c9ab18a212c15e509d90f1d965019eab94b77e4be03b884d789ac888fe1\
        4d3cf5e6dfe3feafbaed2ac40408ecb697d4d355a2ea3d14f109be974acd22b9884b31d185f0fcfd41c3d2a151\
        ed35aae3f568599c501cf6ffefe6a1b48bc708d8c68b06724356258f9b5f2200c500571475616d2a78ff26f9ce\
        bfe87deb64efad32f2976a8b21a64299269dd251995a8d10d8c74369b55fad2b828663f01e028da3d1ee339c67\
        b32da0b7c713f0e3ac63f05e2609ab251ccb47e1718c586b3249716cfca9a14431682903d548c0ea3c12f854ed\
        610d8db89958b1662d760810ad2a9d8322a594359d46ceec6b828000d5d60cc22dcbd1148b1e7060695a4a956d\
        d17f68b47d534f801eccdeb20847f76022b912e9ad79dd45c3cb7083aefd40bd6529e34cf50#10000",
        Less,
    );

    let ramanujans_constant_f32 =
        Float::ramanujans_constant_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(ramanujans_constant_f32.to_string(), "2.62537418e17");
    assert_eq!(to_hex_string(&ramanujans_constant_f32), "0x3.a4b864E+14#24");
    assert_eq!(ramanujans_constant_f32, f32::RAMANUJANS_CONSTANT);

    let ramanujans_constant_f64 =
        Float::ramanujans_constant_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(ramanujans_constant_f64.to_string(), "2.6253741264076874e17");
    assert_eq!(
        to_hex_string(&ramanujans_constant_f64),
        "0x3.a4b862c4b402eE+14#53"
    );
    assert_eq!(ramanujans_constant_f64, f64::RAMANUJANS_CONSTANT);
}

#[test]
#[should_panic]
fn ramanujans_constant_prec_fail_1() {
    Float::ramanujans_constant_prec(0);
}

fn test_ramanujans_constant_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::ramanujans_constant_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);
}

#[test]
pub fn test_ramanujans_constant_prec_round() {
    test_ramanujans_constant_prec_round_helper(1, Floor, "1.4e17", "0x2.0E+14#1", Less);
    test_ramanujans_constant_prec_round_helper(1, Ceiling, "2.9e17", "0x4.0E+14#1", Greater);
    test_ramanujans_constant_prec_round_helper(1, Down, "1.4e17", "0x2.0E+14#1", Less);
    test_ramanujans_constant_prec_round_helper(1, Up, "2.9e17", "0x4.0E+14#1", Greater);
    test_ramanujans_constant_prec_round_helper(1, Nearest, "2.9e17", "0x4.0E+14#1", Greater);
    test_ramanujans_constant_prec_round_helper(2, Floor, "2.2e17", "0x3.0E+14#2", Less);
    test_ramanujans_constant_prec_round_helper(2, Ceiling, "2.9e17", "0x4.0E+14#2", Greater);
    test_ramanujans_constant_prec_round_helper(2, Down, "2.2e17", "0x3.0E+14#2", Less);
    test_ramanujans_constant_prec_round_helper(2, Up, "2.9e17", "0x4.0E+14#2", Greater);
    test_ramanujans_constant_prec_round_helper(2, Nearest, "2.9e17", "0x4.0E+14#2", Greater);
    test_ramanujans_constant_prec_round_helper(3, Floor, "2.5e17", "0x3.8E+14#3", Less);
    test_ramanujans_constant_prec_round_helper(3, Ceiling, "2.9e17", "0x4.0E+14#3", Greater);
    test_ramanujans_constant_prec_round_helper(3, Down, "2.5e17", "0x3.8E+14#3", Less);
    test_ramanujans_constant_prec_round_helper(3, Up, "2.9e17", "0x4.0E+14#3", Greater);
    test_ramanujans_constant_prec_round_helper(3, Nearest, "2.5e17", "0x3.8E+14#3", Less);
    test_ramanujans_constant_prec_round_helper(4, Floor, "2.52e17", "0x3.8E+14#4", Less);
    test_ramanujans_constant_prec_round_helper(4, Ceiling, "2.70e17", "0x3.cE+14#4", Greater);
    test_ramanujans_constant_prec_round_helper(4, Down, "2.52e17", "0x3.8E+14#4", Less);
    test_ramanujans_constant_prec_round_helper(4, Up, "2.70e17", "0x3.cE+14#4", Greater);
    test_ramanujans_constant_prec_round_helper(4, Nearest, "2.70e17", "0x3.cE+14#4", Greater);
    test_ramanujans_constant_prec_round_helper(5, Floor, "2.61e17", "0x3.aE+14#5", Less);
    test_ramanujans_constant_prec_round_helper(5, Ceiling, "2.70e17", "0x3.cE+14#5", Greater);
    test_ramanujans_constant_prec_round_helper(5, Down, "2.61e17", "0x3.aE+14#5", Less);
    test_ramanujans_constant_prec_round_helper(5, Up, "2.70e17", "0x3.cE+14#5", Greater);
    test_ramanujans_constant_prec_round_helper(5, Nearest, "2.61e17", "0x3.aE+14#5", Less);
    test_ramanujans_constant_prec_round_helper(6, Floor, "2.61e17", "0x3.aE+14#6", Less);
    test_ramanujans_constant_prec_round_helper(6, Ceiling, "2.66e17", "0x3.bE+14#6", Greater);
    test_ramanujans_constant_prec_round_helper(6, Down, "2.61e17", "0x3.aE+14#6", Less);
    test_ramanujans_constant_prec_round_helper(6, Up, "2.66e17", "0x3.bE+14#6", Greater);
    test_ramanujans_constant_prec_round_helper(6, Nearest, "2.61e17", "0x3.aE+14#6", Less);
    test_ramanujans_constant_prec_round_helper(7, Floor, "2.612e17", "0x3.a0E+14#7", Less);
    test_ramanujans_constant_prec_round_helper(7, Ceiling, "2.635e17", "0x3.a8E+14#7", Greater);
    test_ramanujans_constant_prec_round_helper(7, Down, "2.612e17", "0x3.a0E+14#7", Less);
    test_ramanujans_constant_prec_round_helper(7, Up, "2.635e17", "0x3.a8E+14#7", Greater);
    test_ramanujans_constant_prec_round_helper(7, Nearest, "2.635e17", "0x3.a8E+14#7", Greater);
    test_ramanujans_constant_prec_round_helper(8, Floor, "2.623e17", "0x3.a4E+14#8", Less);
    test_ramanujans_constant_prec_round_helper(8, Ceiling, "2.635e17", "0x3.a8E+14#8", Greater);
    test_ramanujans_constant_prec_round_helper(8, Down, "2.623e17", "0x3.a4E+14#8", Less);
    test_ramanujans_constant_prec_round_helper(8, Up, "2.635e17", "0x3.a8E+14#8", Greater);
    test_ramanujans_constant_prec_round_helper(8, Nearest, "2.623e17", "0x3.a4E+14#8", Less);
    test_ramanujans_constant_prec_round_helper(9, Floor, "2.623e17", "0x3.a4E+14#9", Less);
    test_ramanujans_constant_prec_round_helper(9, Ceiling, "2.629e17", "0x3.a6E+14#9", Greater);
    test_ramanujans_constant_prec_round_helper(9, Down, "2.623e17", "0x3.a4E+14#9", Less);
    test_ramanujans_constant_prec_round_helper(9, Up, "2.629e17", "0x3.a6E+14#9", Greater);
    test_ramanujans_constant_prec_round_helper(9, Nearest, "2.623e17", "0x3.a4E+14#9", Less);
    test_ramanujans_constant_prec_round_helper(10, Floor, "2.6233e17", "0x3.a4E+14#10", Less);
    test_ramanujans_constant_prec_round_helper(10, Ceiling, "2.6262e17", "0x3.a5E+14#10", Greater);
    test_ramanujans_constant_prec_round_helper(10, Down, "2.6233e17", "0x3.a4E+14#10", Less);
    test_ramanujans_constant_prec_round_helper(10, Up, "2.6262e17", "0x3.a5E+14#10", Greater);
    test_ramanujans_constant_prec_round_helper(10, Nearest, "2.6262e17", "0x3.a5E+14#10", Greater);
    test_ramanujans_constant_prec_round_helper(
        100,
        Floor,
        "262537412640768743.99999999999909",
        "0x3a4b862c4b402e7.ffffffffff0#100",
        Less,
    );
    test_ramanujans_constant_prec_round_helper(
        100,
        Ceiling,
        "262537412640768743.99999999999932",
        "0x3a4b862c4b402e7.ffffffffff4#100",
        Greater,
    );
    test_ramanujans_constant_prec_round_helper(
        100,
        Down,
        "262537412640768743.99999999999909",
        "0x3a4b862c4b402e7.ffffffffff0#100",
        Less,
    );
    test_ramanujans_constant_prec_round_helper(
        100,
        Up,
        "262537412640768743.99999999999932",
        "0x3a4b862c4b402e7.ffffffffff4#100",
        Greater,
    );
    test_ramanujans_constant_prec_round_helper(
        100,
        Nearest,
        "262537412640768743.99999999999932",
        "0x3a4b862c4b402e7.ffffffffff4#100",
        Greater,
    );
}

#[test]
#[should_panic]
fn ramanujans_constant_prec_round_fail_1() {
    Float::ramanujans_constant_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn ramanujans_constant_prec_round_fail_2() {
    Float::ramanujans_constant_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn ramanujans_constant_prec_round_fail_3() {
    Float::ramanujans_constant_prec_round(1000, Exact);
}

#[test]
fn ramanujans_constant_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (ramanujans_constant, o) = Float::ramanujans_constant_prec(prec);
        assert!(ramanujans_constant.is_valid());
        assert_eq!(ramanujans_constant.get_prec(), Some(prec));
        // e^(pi*sqrt(163)) is just below 2^58; at very low precision it rounds up to 2^58 (exponent
        // 59).
        assert_eq!(
            ramanujans_constant.get_exponent(),
            Some(if prec <= 2 { 59 } else { 58 })
        );
        assert_ne!(o, Equal);
        if o == Less {
            let (alt, o_alt) = Float::ramanujans_constant_prec_round(prec, Ceiling);
            let mut next_upper = ramanujans_constant.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !ramanujans_constant.is_power_of_2() {
            let (alt, o_alt) = Float::ramanujans_constant_prec_round(prec, Floor);
            let mut next_lower = ramanujans_constant.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
        let (alt, o_alt) = Float::ramanujans_constant_prec_round(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&alt),
            ComparableFloatRef(&ramanujans_constant)
        );
        assert_eq!(o_alt, o);
    });
}

#[test]
fn ramanujans_constant_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (ramanujans_constant, o) = Float::ramanujans_constant_prec_round(prec, rm);
        assert!(ramanujans_constant.is_valid());
        assert_eq!(ramanujans_constant.get_prec(), Some(prec));
        // e^(pi*sqrt(163)) is just below 2^58, so the result has exponent 58 unless it rounds up
        // across 2^58 (exponent 59) at very low precision.
        let expected_exponent = match (prec, rm) {
            (1 | 2, Ceiling | Up | Nearest) | (3, Ceiling | Up) => 59,
            _ => 58,
        };
        assert_eq!(ramanujans_constant.get_exponent(), Some(expected_exponent));
        assert_ne!(o, Equal);
        if o == Less {
            let (alt, o_alt) = Float::ramanujans_constant_prec_round(prec, Ceiling);
            let mut next_upper = ramanujans_constant.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !ramanujans_constant.is_power_of_2() {
            let (alt, o_alt) = Float::ramanujans_constant_prec_round(prec, Floor);
            let mut next_lower = ramanujans_constant.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::ramanujans_constant_prec_round(prec, Exact));
    });

    test_constant(Float::ramanujans_constant_prec_round, 10000);
}
