// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::PiOver3;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{test_constant, to_hex_string};
use malachite_float::test_util::constants::pi_over_3::pi_over_3_prec_round_simple;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_pi_over_3_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::pi_over_3_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = pi_over_3_prec_round_simple(prec, Nearest);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);
}

#[test]
pub fn test_pi_over_3_prec() {
    test_pi_over_3_prec_helper(1, "1.0", "0x1.0#1", Less);
    test_pi_over_3_prec_helper(2, "1.0", "0x1.0#2", Less);
    test_pi_over_3_prec_helper(3, "1.0", "0x1.0#3", Less);
    test_pi_over_3_prec_helper(4, "1.0", "0x1.0#4", Less);
    test_pi_over_3_prec_helper(5, "1.06", "0x1.1#5", Greater);
    test_pi_over_3_prec_helper(6, "1.06", "0x1.10#6", Greater);
    test_pi_over_3_prec_helper(7, "1.05", "0x1.0c#7", Less);
    test_pi_over_3_prec_helper(8, "1.047", "0x1.0c#8", Less);
    test_pi_over_3_prec_helper(9, "1.047", "0x1.0c#9", Less);
    test_pi_over_3_prec_helper(10, "1.047", "0x1.0c0#10", Less);
    test_pi_over_3_prec_helper(
        100,
        "1.047197551196597746154214461094",
        "0x1.0c152382d73658465bb32e0f6#100",
        Greater,
    );
    test_pi_over_3_prec_helper(
        1000,
        "1.047197551196597746154214461093167628065723133125035273658314864102605468762069666209344\
        941780705689327382695504427435549031281536516860743908453136042827039150094700900646173701\
        853214874316318310127321476270325221977815376158549411262261055090400636381882855641153449\
        5368181088827377978690867497137579",
        "0x1.0c152382d73658465bb32e0f567ad116e158680b6335109aad64fe32f96f7983170d60a212f0067d3f717\
        79a66f85979958eb892987ec59f152c473c91c2585d30b247488328a90945bb03e232f53c8ebaa9d0f3f008f53\
        d3da08ff9ce0cd4dce8d4301750642a88618b3317e685cefd02ab50f62c84feb221230af2d0#1000",
        Less,
    );
    test_pi_over_3_prec_helper(
        10000,
        "1.047197551196597746154214461093167628065723133125035273658314864102605468762069666209344\
        941780705689327382695504427435549031281536516860743908453136042827039150094700900646173701\
        853214874316318310127321476270325221977815376158549411262261055090400636381882855641153449\
        536818108882737797869086749713757908195668868771862724960506973654276418030571788122630863\
        453337110176849606822173794715650647170536477685756788586530651030728705793977537264368372\
        849358154126654249855783961917574963742646061003983043277891120813552214362007131648798408\
        245730234059953647900923513072392097725584128224939489223135044000189375715087853609261923\
        780919263203057879059573822813633741651143382183195123683597426563086307847339985370709673\
        986954678139386604543258257103320172902403783333332790992683317019910577605365439531674819\
        818448969434214174102751114895011753977062723670001045946250962195844402793806872392556382\
        434532751163476251822910386520954627451262531250652593952593510723742268871000642625537065\
        303072140066312698419067021828621092955312051127275989410100650678433950989665257874199804\
        630416573925094493043838524952414151383565316943177705620575951963025032793918212488313106\
        418353466975900557046336616274670952787201187902553368236727314318532066315589279149816085\
        126591575615701349178215402682228086356497097771225676329717368250720685655341352679383397\
        837084460810011862546749165491087971399757534756640932260784927212003113907213739974862105\
        010095394324851902249946168498196195642331896975736932503100985107055114995734251986745493\
        555166373293944932584522123269142180842620850606139191557630325924264600027215686672048416\
        397391072404924116713813991189516053787052450850711158580616489479507774635798047778182587\
        472287506327856495187366406407394757516751418962557263498200551155601662875744263928695261\
        461275989325560484700317961262120316893354741708401705797661632028042829542315201413988428\
        340736887062102248092874013064981682374571262320318788123972429155921552524654137963621944\
        215331937796826009196699821921359650423156132784198569941940873507496469242239826089494200\
        492330300880045464812485101689401165417483916466550477143269730219750312407232153838569952\
        795803532628653257658499643387251309489379422894622980924718663951975081984653143683324174\
        893615329091214898616217945578740875366374869350414628130150414712183254260265905230478665\
        900043205363147231622851949468784474024075276162882719485342835338947579817422489226298417\
        379507516651555575941328818865372118295410192485499345197878189391441370838358689826483698\
        865364675076293236964381889712289076249646853367167769539309560306958253639274979529669904\
        969891995087121849927297709928273894332982907552934952521338090159185044126547150507915411\
        454847619481598421955940350380451578579841037809055367378656512077147650828312395703381921\
        801196759978134580669103526179687399462482602826163227773815237956250647835476739484397016\
        160335123538226891639759397065979984020473221142918146881248374572730726666130338639853938\
        2250475637465829803023954980773205226484027",
        "0x1.0c152382d73658465bb32e0f567ad116e158680b6335109aad64fe32f96f7983170d60a212f0067d3f717\
        79a66f85979958eb892987ec59f152c473c91c2585d30b247488328a90945bb03e232f53c8ebaa9d0f3f008f53\
        d3da08ff9ce0cd4dce8d4301750642a88618b3317e685cefd02ab50f62c84feb221230af2d0726f788c1daa36a\
        6dbbf2a048726da7b84e772d083ef1d80b1c3a4d3c6e15f40c8c891debaf1bdb8fb755bec9b3ab662cad750436\
        b285d92f3bda52f7df43acabe0804cedf5a2e900a2e14f25c7d409465c3b7d2e50ff371cac975a21c61fbe371e\
        3dc1d1832cb76a2b16ac7436878b8e3b03ce6eec966b06b4d9a35c6d78fd4264ddbe6a4b15b21253eb8b938971\
        f26b2bb5244c96a07892d310a3a9ce8bbceb6efc97e10c68062dc8227be851832ce6e933a96eaa2b3ccb80b312\
        09d5899a90b3885c2d4397574a42abba52c1f1f4dd7273b49620baba3cc5e82b68314d5f132399705247aa6815\
        16b6864ae6c2b8c2c0aac2342fac38a0a891f60977816524dcede22598975e3f12da578c5e046481c0fcd875a8\
        d0d8e70668bcfa503ceb128bea19369501ad4fe638835fb21b4688fab277798c86a2b6baf82d9a4d75dc1cfdfe\
        6d4818c96692e74ea4acfd1f2d74060266ab36c351ceb238cc4f138cb676a7d020954f5261633ab69bd459d0c4\
        558b0c2f3aff8f118a5eade57c67b98803309290c9c289da7a2f4fd4bff7008e77dc413dd244ae9ac40023e408\
        dc53cc035204174c98a40b32361767853cfe514cec691b1133ba3be70f97a79fec5b533badc64442b1716e51f9\
        45894f69aac4a1118ff22050d57b30f6e914043e2c7c1ed7c054603ca689346a9491c7deae9b37566039ce0559\
        76ab97b7dcd350caa53b5369984e1f8a84910b652bed1b24aa9cb23b1ba700a42e4571b8e661491fe54612d201\
        bbb291814aaf52b8a1ec7e9437a84355e2d1cba4a5d2349471638520d7faa968ecd10ecd96fc7267873b7e593e\
        e1d984b55367492fab08ab05369dda9b5d692c3a991ceb9f09bc933714c289252c1cc9b84c33ec3fe87daf5f4a\
        648e1992a1120fe5bc09a4c424da50aee48bcd26eab457f8a54b9505fe6dc9e9e19e4dadb32f8e484d07931473\
        59ada45f03a97b74ada141e652f7c86e7daa7a0fea6060e76d82d92b0daaf50096fe474e022da965ef09aa5db3\
        be2eb39ba650b5d94af5d27f8d1b9ff83ab5fe0a1e044053c7a7c4d5d8efbf244d8a0dde6e2c54aff06a03d299\
        6be809b8f38485cca8b778031d2573144265c0b08b17d4ce46021d291fe2ced1c1651fe89bc9aa3ef3a597e6a2\
        de0476b09468f5f7f6daab704b9b57b3bca0b78003e7292f58fb6cc1233faade85f71cbdb09c89fe238d2eb168\
        34873712a6029c93600f73dec810cabd220dc9a8db09808781a266e15e66d0f437e5c386e091b001b88c663074\
        75a726a9433ecf6b92036d22b4cd155583e253c725ea30aa6324ece6359f3073ccbcc60a293533cffbc01ba41d\
        71ccc1be564748ddfda8b02e8c2887a2c57ce1928d04de73bb86c4927030f96b30cb68f24e23ac337f529defa2\
        03d854f3b77a438d97b23335d551ccc1b79963b34f5b31200e1d1adc40de01db115a15d68bf6a71883373c0df2\
        1ce854c4788a7bff235f0dead4ff81051c464684cfab71f406ef4602cd6d04e62212ba34200b4eeca0322ce6a6\
        a3e4fedbedd0806ce78d035cd7fbc8170e04b823d341bac8e1aad126a02815ed54a3a1eda2a#10000",
        Greater,
    );

    let pi_over_3_f32 = Float::pi_over_3_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(pi_over_3_f32.to_string(), "1.0471976");
    assert_eq!(to_hex_string(&pi_over_3_f32), "0x1.0c1524#24");
    assert_eq!(pi_over_3_f32, f32::PI_OVER_3);

    let pi_over_3_f64 = Float::pi_over_3_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(pi_over_3_f64.to_string(), "1.0471975511965979");
    assert_eq!(to_hex_string(&pi_over_3_f64), "0x1.0c152382d7366#53");
    assert_eq!(pi_over_3_f64, f64::PI_OVER_3);
}

#[test]
#[should_panic]
fn pi_over_3_prec_fail_1() {
    Float::pi_over_3_prec(0);
}

fn test_pi_over_3_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::pi_over_3_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = pi_over_3_prec_round_simple(prec, rm);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);
}

#[test]
pub fn test_pi_over_3_prec_round() {
    test_pi_over_3_prec_round_helper(1, Floor, "1.0", "0x1.0#1", Less);
    test_pi_over_3_prec_round_helper(1, Ceiling, "2.0", "0x2.0#1", Greater);
    test_pi_over_3_prec_round_helper(1, Down, "1.0", "0x1.0#1", Less);
    test_pi_over_3_prec_round_helper(1, Up, "2.0", "0x2.0#1", Greater);
    test_pi_over_3_prec_round_helper(1, Nearest, "1.0", "0x1.0#1", Less);

    test_pi_over_3_prec_round_helper(2, Floor, "1.0", "0x1.0#2", Less);
    test_pi_over_3_prec_round_helper(2, Ceiling, "1.5", "0x1.8#2", Greater);
    test_pi_over_3_prec_round_helper(2, Down, "1.0", "0x1.0#2", Less);
    test_pi_over_3_prec_round_helper(2, Up, "1.5", "0x1.8#2", Greater);
    test_pi_over_3_prec_round_helper(2, Nearest, "1.0", "0x1.0#2", Less);

    test_pi_over_3_prec_round_helper(3, Floor, "1.0", "0x1.0#3", Less);
    test_pi_over_3_prec_round_helper(3, Ceiling, "1.2", "0x1.4#3", Greater);
    test_pi_over_3_prec_round_helper(3, Down, "1.0", "0x1.0#3", Less);
    test_pi_over_3_prec_round_helper(3, Up, "1.2", "0x1.4#3", Greater);
    test_pi_over_3_prec_round_helper(3, Nearest, "1.0", "0x1.0#3", Less);

    test_pi_over_3_prec_round_helper(4, Floor, "1.0", "0x1.0#4", Less);
    test_pi_over_3_prec_round_helper(4, Ceiling, "1.1", "0x1.2#4", Greater);
    test_pi_over_3_prec_round_helper(4, Down, "1.0", "0x1.0#4", Less);
    test_pi_over_3_prec_round_helper(4, Up, "1.1", "0x1.2#4", Greater);
    test_pi_over_3_prec_round_helper(4, Nearest, "1.0", "0x1.0#4", Less);

    test_pi_over_3_prec_round_helper(5, Floor, "1.0", "0x1.0#5", Less);
    test_pi_over_3_prec_round_helper(5, Ceiling, "1.06", "0x1.1#5", Greater);
    test_pi_over_3_prec_round_helper(5, Down, "1.0", "0x1.0#5", Less);
    test_pi_over_3_prec_round_helper(5, Up, "1.06", "0x1.1#5", Greater);
    test_pi_over_3_prec_round_helper(5, Nearest, "1.06", "0x1.1#5", Greater);

    test_pi_over_3_prec_round_helper(6, Floor, "1.03", "0x1.08#6", Less);
    test_pi_over_3_prec_round_helper(6, Ceiling, "1.06", "0x1.10#6", Greater);
    test_pi_over_3_prec_round_helper(6, Down, "1.03", "0x1.08#6", Less);
    test_pi_over_3_prec_round_helper(6, Up, "1.06", "0x1.10#6", Greater);
    test_pi_over_3_prec_round_helper(6, Nearest, "1.06", "0x1.10#6", Greater);

    test_pi_over_3_prec_round_helper(7, Floor, "1.05", "0x1.0c#7", Less);
    test_pi_over_3_prec_round_helper(7, Ceiling, "1.06", "0x1.10#7", Greater);
    test_pi_over_3_prec_round_helper(7, Down, "1.05", "0x1.0c#7", Less);
    test_pi_over_3_prec_round_helper(7, Up, "1.06", "0x1.10#7", Greater);
    test_pi_over_3_prec_round_helper(7, Nearest, "1.05", "0x1.0c#7", Less);

    test_pi_over_3_prec_round_helper(8, Floor, "1.047", "0x1.0c#8", Less);
    test_pi_over_3_prec_round_helper(8, Ceiling, "1.055", "0x1.0e#8", Greater);
    test_pi_over_3_prec_round_helper(8, Down, "1.047", "0x1.0c#8", Less);
    test_pi_over_3_prec_round_helper(8, Up, "1.055", "0x1.0e#8", Greater);
    test_pi_over_3_prec_round_helper(8, Nearest, "1.047", "0x1.0c#8", Less);

    test_pi_over_3_prec_round_helper(9, Floor, "1.047", "0x1.0c#9", Less);
    test_pi_over_3_prec_round_helper(9, Ceiling, "1.051", "0x1.0d#9", Greater);
    test_pi_over_3_prec_round_helper(9, Down, "1.047", "0x1.0c#9", Less);
    test_pi_over_3_prec_round_helper(9, Up, "1.051", "0x1.0d#9", Greater);
    test_pi_over_3_prec_round_helper(9, Nearest, "1.047", "0x1.0c#9", Less);

    test_pi_over_3_prec_round_helper(10, Floor, "1.047", "0x1.0c0#10", Less);
    test_pi_over_3_prec_round_helper(10, Ceiling, "1.049", "0x1.0c8#10", Greater);
    test_pi_over_3_prec_round_helper(10, Down, "1.047", "0x1.0c0#10", Less);
    test_pi_over_3_prec_round_helper(10, Up, "1.049", "0x1.0c8#10", Greater);
    test_pi_over_3_prec_round_helper(10, Nearest, "1.047", "0x1.0c0#10", Less);

    test_pi_over_3_prec_round_helper(
        100,
        Floor,
        "1.047197551196597746154214461092",
        "0x1.0c152382d73658465bb32e0f4#100",
        Less,
    );
    test_pi_over_3_prec_round_helper(
        100,
        Ceiling,
        "1.047197551196597746154214461094",
        "0x1.0c152382d73658465bb32e0f6#100",
        Greater,
    );
    test_pi_over_3_prec_round_helper(
        100,
        Down,
        "1.047197551196597746154214461092",
        "0x1.0c152382d73658465bb32e0f4#100",
        Less,
    );
    test_pi_over_3_prec_round_helper(
        100,
        Up,
        "1.047197551196597746154214461094",
        "0x1.0c152382d73658465bb32e0f6#100",
        Greater,
    );
    test_pi_over_3_prec_round_helper(
        100,
        Nearest,
        "1.047197551196597746154214461094",
        "0x1.0c152382d73658465bb32e0f6#100",
        Greater,
    );
}

#[test]
#[should_panic]
fn pi_over_3_prec_round_fail_1() {
    Float::pi_over_3_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn pi_over_3_prec_round_fail_2() {
    Float::pi_over_3_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn pi_over_3_prec_round_fail_3() {
    Float::pi_over_3_prec_round(1000, Exact);
}

#[test]
fn pi_over_3_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (pi_over_3, o) = Float::pi_over_3_prec(prec);
        assert!(pi_over_3.is_valid());
        assert_eq!(pi_over_3.get_prec(), Some(prec));
        assert_eq!(pi_over_3.get_exponent(), Some(1));
        assert_ne!(o, Equal);
        if o == Less {
            let (pi_over_3_alt, o_alt) = Float::pi_over_3_prec_round(prec, Ceiling);
            let mut next_upper = pi_over_3.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(pi_over_3_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !pi_over_3.is_power_of_2() {
            let (pi_over_3_alt, o_alt) = Float::pi_over_3_prec_round(prec, Floor);
            let mut next_lower = pi_over_3.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(pi_over_3_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
        let (pi_over_3_alt, o_alt) = Float::pi_over_3_prec_round(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&pi_over_3_alt),
            ComparableFloatRef(&pi_over_3)
        );
        assert_eq!(o_alt, o);

        let (pi_over_3_alt, o_alt) = pi_over_3_prec_round_simple(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&pi_over_3_alt),
            ComparableFloatRef(&pi_over_3)
        );
        assert_eq!(o_alt, o);
    });
}

#[test]
fn pi_over_3_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (pi_over_3, o) = Float::pi_over_3_prec_round(prec, rm);
        assert!(pi_over_3.is_valid());
        assert_eq!(pi_over_3.get_prec(), Some(prec));
        assert_eq!(
            pi_over_3.get_exponent(),
            Some(if prec == 1 && (rm == Ceiling || rm == Up) {
                2
            } else {
                1
            })
        );
        assert_ne!(o, Equal);
        if o == Less {
            let (pi_over_3_alt, o_alt) = Float::pi_over_3_prec_round(prec, Ceiling);
            let mut next_upper = pi_over_3.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(pi_over_3_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !pi_over_3.is_power_of_2() {
            let (pi_over_3_alt, o_alt) = Float::pi_over_3_prec_round(prec, Floor);
            let mut next_lower = pi_over_3.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(pi_over_3_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }

        let (pi_over_3_alt, o_alt) = pi_over_3_prec_round_simple(prec, rm);
        assert_eq!(
            ComparableFloatRef(&pi_over_3_alt),
            ComparableFloatRef(&pi_over_3)
        );
        assert_eq!(o_alt, o);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::pi_over_3_prec_round(prec, Exact));
    });

    test_constant(Float::pi_over_3_prec_round, 10000);
}
