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
use malachite_float::test_util::common::{
    rug_round_try_from_rounding_mode, test_constant, to_hex_string,
};
use malachite_float::test_util::constants::e::rug_e_prec_round;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_e_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::e_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (rug_x, rug_o) = rug_e_prec_round(prec, rug_round_try_from_rounding_mode(Nearest).unwrap());
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_x)),
        ComparableFloatRef(&x)
    );
    assert_eq!(rug_o, o);
}

#[test]
pub fn test_e_prec() {
    test_e_prec_helper(1, "2.0", "0x2.0#1", Less);
    test_e_prec_helper(2, "3.0", "0x3.0#2", Greater);
    test_e_prec_helper(3, "2.5", "0x2.8#3", Less);
    test_e_prec_helper(4, "2.75", "0x2.c#4", Greater);
    test_e_prec_helper(5, "2.75", "0x2.c#5", Greater);
    test_e_prec_helper(6, "2.69", "0x2.b#6", Less);
    test_e_prec_helper(7, "2.719", "0x2.b8#7", Greater);
    test_e_prec_helper(8, "2.719", "0x2.b8#8", Greater);
    test_e_prec_helper(9, "2.719", "0x2.b8#9", Greater);
    test_e_prec_helper(10, "2.7188", "0x2.b8#10", Greater);
    test_e_prec_helper(
        100,
        "2.7182818284590452353602874713512",
        "0x2.b7e151628aed2a6abf7158808#100",
        Less,
    );
    test_e_prec_helper(
        1000,
        "2.718281828459045235360287471352662497757247093699959574966967627724076630353547594571382\
        178525166427427466391932003059921817413596629043572900334295260595630738132328627943490763\
        233829880753195251019011573834187930702154089149934884167509244761460668082264800168477411\
        85374234544243710753907774499206950",
        "0x2.b7e151628aed2a6abf7158809cf4f3c762e7160f38b4da56a784d9045190cfef324e7738926cfbe5f4bf8\
        d8d8c31d763da06c80abb1185eb4f7c7b5757f5958490cfd47d7c19bb42158d9554f7b46bced55c4d79fd5f24d\
        6613c31c3839a2ddf8a9a276bcfbfa1c877c56284dab79cd4c2b3293d20e9e5eaf02ac60acc#1000",
        Less,
    );
    test_e_prec_helper(
        10000,
        "2.718281828459045235360287471352662497757247093699959574966967627724076630353547594571382\
        178525166427427466391932003059921817413596629043572900334295260595630738132328627943490763\
        233829880753195251019011573834187930702154089149934884167509244761460668082264800168477411\
        853742345442437107539077744992069551702761838606261331384583000752044933826560297606737113\
        200709328709127443747047230696977209310141692836819025515108657463772111252389784425056953\
        696770785449969967946864454905987931636889230098793127736178215424999229576351482208269895\
        193668033182528869398496465105820939239829488793320362509443117301238197068416140397019837\
        679320683282376464804295311802328782509819455815301756717361332069811250996181881593041690\
        351598888519345807273866738589422879228499892086805825749279610484198444363463244968487560\
        233624827041978623209002160990235304369941849146314093431738143640546253152096183690888707\
        016768396424378140592714563549061303107208510383750510115747704171898610687396965521267154\
        688957035035402123407849819334321068170121005627880235193033224745015853904730419957777093\
        503660416997329725088687696640355570716226844716256079882651787134195124665201030592123667\
        719432527867539855894489697096409754591856956380236370162112047742722836489613422516445078\
        182442352948636372141740238893441247963574370263755294448337998016125492278509257782562092\
        622648326277933386566481627725164019105900491644998289315056604725802778631864155195653244\
        258698294695930801915298721172556347546396447910145904090586298496791287406870504895858671\
        747985466775757320568128845920541334053922000113786300945560688166740016984205580403363795\
        376452030402432256613527836951177883863874439662532249850654995886234281899707733276171783\
        928034946501434558897071942586398772754710962953741521115136835062752602326484728703920764\
        310059584116612054529703023647254929666938115137322753645098889031360205724817658511806303\
        644281231496550704751025446501172721155519486685080036853228183152196003735625279449515828\
        418829478761085263981395599006737648292244375287184624578036192981971399147564488262603903\
        381441823262515097482798777996437308997038886778227138360577297882412561190717663946507063\
        304527954661855096666185664709711344474016070462621568071748187784437143698821855967095910\
        259686200235371858874856965220005031173439207321139080329363447972735595527734907178379342\
        163701205005451326383544000186323991490705479778056697853358048966906295119432473099587655\
        236812859041383241160722602998330535370876138939639177957454016137223618789365260538155841\
        587186925538606164779834025435128439612946035291332594279490433729908573158029095863138268\
        329147711639633709240031689458636060645845925126994655724839186564209752685082307544254599\
        376917041977780085362730941710163434907696423722294352366125572508814779223151974778060569\
        672538017180776360346245927877846585065605078084421152969752189087401966090665180351650179\
        250461950136658543663271254963990854914420001457476081930221206602433009641270489439039717\
        71951806990869986066365832322787093765022602",
        "0x2.b7e151628aed2a6abf7158809cf4f3c762e7160f38b4da56a784d9045190cfef324e7738926cfbe5f4bf8\
        d8d8c31d763da06c80abb1185eb4f7c7b5757f5958490cfd47d7c19bb42158d9554f7b46bced55c4d79fd5f24d\
        6613c31c3839a2ddf8a9a276bcfbfa1c877c56284dab79cd4c2b3293d20e9e5eaf02ac60acc93ed874422a52ec\
        b238feee5ab6add835fd1a0753d0a8f78e537d2b95bb79d8dcaec642c1e9f23b829b5c2780bf38737df8bb300d\
        01334a0d0bd8645cbfa73a6160ffe393c48cbbbca060f0ff8ec6d31beb5cceed7f2f0bb088017163bc60df45a0\
        ecb1bcd289b06cbbfea21ad08e1847f3f7378d56ced94640d6ef0d3d37be67008e186d1bf275b9b241deb64749\
        a47dfdfb96632c3eb061b6472bbf84c26144e49c2d04c324ef10de513d3f5114b8b5d374d93cb8879c7d52ffd7\
        2ba0aae7277da7ba1b4af1488d8e836af14865e6c37ab6876fe690b571121382af341afe94f77bcf06c83b8ff5\
        675f0979074ad9a787bc5b9bd4b0c5937d3ede4c3a79396215edab1f57d0b5a7db461dd8f3c75540d00121fd56\
        e95f8c731e9c4d7221bbed0c62bb5a87804b679a0caa41d802a4604c311b71de3e5c6b400e024a6668ccf2e2de\
        86876e4f5c50000f0a93b3aa7e6342b302a0a47373b25f73e3b26d569fe2291ad36d6a147d1060b871a2801f97\
        83764082ff592d9140db1e9399df4b0e14ca8e88ee9110b2bd4fa98eed150ca6dd8932245ef7592c703f532ce3\
        a30cd31c070eb36b4195ff33fb1c66c7d70f93918107ce2051fed33f6d1de9491c7dea6a5a442e154c8bb6d8d0\
        362803bc248d414478c2afb07ffe78e89b9feca7e3060c08f0d61f8e36801df66d1d8f9392e52caef065319947\
        9df2be64bbaab008ca8a06fdace9ce70489845a082ba36d611e99f2fbe724246d18b54e335cac0dd1ab9dfd798\
        8a4b0c4558aa119417720b6e150ce2b927d48d7256e445e333cb7572b3bd00fb2746043189cac116cedc7e771a\
        e0358ff752a3a6b6c79a58a9a549b50c5870690755c35e4e36b529038ca733fd1aaa8dab40133d80320e079096\
        8c76546b993f6c8ff3b2542750da1ffada7b74731782e330ef7d92c43be1ad8c50a8eae20a5556cbdd1f24c999\
        72cb03c73006f5c08a4e220e74abc179151412b1e2dd60a08a11b02e8d70d7d71645833011bf60945507f1a327\
        21ac08aedc2661da91839d146a2a4c425c0ffb87085f9b0e09b94b146a9a4783908f3f267a78c59430485ed892\
        05b36b66a57e756e006522367028287f8c1d695df88c60fe07528fcbe915c7bf23382ea293fa2da1577f9cac29\
        9bb7b4beeafef9628c3ebeaf87175c6a1f8bdd07be307fa1bfa9aeff794c19dfc365f447527dea110f4208b941\
        aa7d185380478aa520e3fe2335a322edf147bbdb527aa2ad3cb0f7d6ed381cd6ac35a1d24bf89b75019605aee9\
        dfaba5cfced033ba2102a0bdbe3b49d7272f89e09d008e5d5bd99239362861eb426297c5841397515473cf2a3d\
        6de58c4bb1b91ad97abf028e9665da4ece80ddc13e0df4322eda0fd389b175e8d10d08c5230a6b576c94fc52b4\
        e74b2e3420e902c82ee7aeb805c2eb76517eabffb8777378c3806c91d07f109d67527c185683448154d6921610\
        755bef30b8027e5e8b04e592e8b2ea354a261eabb48516472d469a66824f24d44b15ce142fcb65307f21cdabbd\
        6858cc7ae3cee06ae64bed91a0412e6f33ba04fab57b8e52165ca81d0c7d118385d6d230a48#10000",
        Greater,
    );

    let e_f32 = Float::e_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(e_f32.to_string(), "2.71828175");
    assert_eq!(to_hex_string(&e_f32), "0x2.b7e150#24");
    assert_eq!(e_f32, core::f32::consts::E);

    let e_f64 = Float::e_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(e_f64.to_string(), "2.7182818284590451");
    assert_eq!(to_hex_string(&e_f64), "0x2.b7e151628aed2#53");
    assert_eq!(e_f64, core::f64::consts::E);
}

#[test]
#[should_panic]
fn e_prec_fail_1() {
    Float::e_prec(0);
}

fn test_e_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::e_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_x, rug_o) = rug_e_prec_round(prec, rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_x)),
            ComparableFloatRef(&x)
        );
        assert_eq!(rug_o, o);
    }
}

#[test]
pub fn test_e_prec_round() {
    test_e_prec_round_helper(1, Floor, "2.0", "0x2.0#1", Less);
    test_e_prec_round_helper(1, Ceiling, "4.0", "0x4.0#1", Greater);
    test_e_prec_round_helper(1, Down, "2.0", "0x2.0#1", Less);
    test_e_prec_round_helper(1, Up, "4.0", "0x4.0#1", Greater);
    test_e_prec_round_helper(1, Nearest, "2.0", "0x2.0#1", Less);
    test_e_prec_round_helper(2, Floor, "2.0", "0x2.0#2", Less);
    test_e_prec_round_helper(2, Ceiling, "3.0", "0x3.0#2", Greater);
    test_e_prec_round_helper(2, Down, "2.0", "0x2.0#2", Less);
    test_e_prec_round_helper(2, Up, "3.0", "0x3.0#2", Greater);
    test_e_prec_round_helper(2, Nearest, "3.0", "0x3.0#2", Greater);
    test_e_prec_round_helper(3, Floor, "2.5", "0x2.8#3", Less);
    test_e_prec_round_helper(3, Ceiling, "3.0", "0x3.0#3", Greater);
    test_e_prec_round_helper(3, Down, "2.5", "0x2.8#3", Less);
    test_e_prec_round_helper(3, Up, "3.0", "0x3.0#3", Greater);
    test_e_prec_round_helper(3, Nearest, "2.5", "0x2.8#3", Less);
    test_e_prec_round_helper(4, Floor, "2.50", "0x2.8#4", Less);
    test_e_prec_round_helper(4, Ceiling, "2.75", "0x2.c#4", Greater);
    test_e_prec_round_helper(4, Down, "2.50", "0x2.8#4", Less);
    test_e_prec_round_helper(4, Up, "2.75", "0x2.c#4", Greater);
    test_e_prec_round_helper(4, Nearest, "2.75", "0x2.c#4", Greater);
    test_e_prec_round_helper(5, Floor, "2.62", "0x2.a#5", Less);
    test_e_prec_round_helper(5, Ceiling, "2.75", "0x2.c#5", Greater);
    test_e_prec_round_helper(5, Down, "2.62", "0x2.a#5", Less);
    test_e_prec_round_helper(5, Up, "2.75", "0x2.c#5", Greater);
    test_e_prec_round_helper(5, Nearest, "2.75", "0x2.c#5", Greater);
    test_e_prec_round_helper(6, Floor, "2.69", "0x2.b#6", Less);
    test_e_prec_round_helper(6, Ceiling, "2.75", "0x2.c#6", Greater);
    test_e_prec_round_helper(6, Down, "2.69", "0x2.b#6", Less);
    test_e_prec_round_helper(6, Up, "2.75", "0x2.c#6", Greater);
    test_e_prec_round_helper(6, Nearest, "2.69", "0x2.b#6", Less);
    test_e_prec_round_helper(7, Floor, "2.688", "0x2.b0#7", Less);
    test_e_prec_round_helper(7, Ceiling, "2.719", "0x2.b8#7", Greater);
    test_e_prec_round_helper(7, Down, "2.688", "0x2.b0#7", Less);
    test_e_prec_round_helper(7, Up, "2.719", "0x2.b8#7", Greater);
    test_e_prec_round_helper(7, Nearest, "2.719", "0x2.b8#7", Greater);
    test_e_prec_round_helper(8, Floor, "2.703", "0x2.b4#8", Less);
    test_e_prec_round_helper(8, Ceiling, "2.719", "0x2.b8#8", Greater);
    test_e_prec_round_helper(8, Down, "2.703", "0x2.b4#8", Less);
    test_e_prec_round_helper(8, Up, "2.719", "0x2.b8#8", Greater);
    test_e_prec_round_helper(8, Nearest, "2.719", "0x2.b8#8", Greater);
    test_e_prec_round_helper(9, Floor, "2.711", "0x2.b6#9", Less);
    test_e_prec_round_helper(9, Ceiling, "2.719", "0x2.b8#9", Greater);
    test_e_prec_round_helper(9, Down, "2.711", "0x2.b6#9", Less);
    test_e_prec_round_helper(9, Up, "2.719", "0x2.b8#9", Greater);
    test_e_prec_round_helper(9, Nearest, "2.719", "0x2.b8#9", Greater);
    test_e_prec_round_helper(10, Floor, "2.7148", "0x2.b7#10", Less);
    test_e_prec_round_helper(10, Ceiling, "2.7188", "0x2.b8#10", Greater);
    test_e_prec_round_helper(10, Down, "2.7148", "0x2.b7#10", Less);
    test_e_prec_round_helper(10, Up, "2.7188", "0x2.b8#10", Greater);
    test_e_prec_round_helper(10, Nearest, "2.7188", "0x2.b8#10", Greater);
    test_e_prec_round_helper(
        100,
        Floor,
        "2.7182818284590452353602874713512",
        "0x2.b7e151628aed2a6abf7158808#100",
        Less,
    );
    test_e_prec_round_helper(
        100,
        Ceiling,
        "2.7182818284590452353602874713544",
        "0x2.b7e151628aed2a6abf715880c#100",
        Greater,
    );
    test_e_prec_round_helper(
        100,
        Down,
        "2.7182818284590452353602874713512",
        "0x2.b7e151628aed2a6abf7158808#100",
        Less,
    );
    test_e_prec_round_helper(
        100,
        Up,
        "2.7182818284590452353602874713544",
        "0x2.b7e151628aed2a6abf715880c#100",
        Greater,
    );
    test_e_prec_round_helper(
        100,
        Nearest,
        "2.7182818284590452353602874713512",
        "0x2.b7e151628aed2a6abf7158808#100",
        Less,
    );
}

#[test]
#[should_panic]
fn e_prec_round_fail_1() {
    Float::e_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn e_prec_round_fail_2() {
    Float::e_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn e_prec_round_fail_3() {
    Float::e_prec_round(1000, Exact);
}

#[test]
fn e_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (e, o) = Float::e_prec(prec);
        assert!(e.is_valid());
        assert_eq!(e.get_prec(), Some(prec));
        assert_eq!(e.get_exponent(), Some(2));
        assert_ne!(o, Equal);
        if o == Less {
            let (e_alt, o_alt) = Float::e_prec_round(prec, Ceiling);
            let mut next_upper = e.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(e_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !e.is_power_of_2() {
            let (e_alt, o_alt) = Float::e_prec_round(prec, Floor);
            let mut next_lower = e.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(e_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
        let (e_alt, o_alt) = Float::e_prec_round(prec, Nearest);
        assert_eq!(ComparableFloatRef(&e_alt), ComparableFloatRef(&e));
        assert_eq!(o_alt, o);

        let (rug_e, rug_o) =
            rug_e_prec_round(prec, rug_round_try_from_rounding_mode(Nearest).unwrap());
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_e)),
            ComparableFloatRef(&e)
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn e_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (e, o) = Float::e_prec_round(prec, rm);
        assert!(e.is_valid());
        assert_eq!(e.get_prec(), Some(prec));
        assert_eq!(
            e.get_exponent(),
            Some(if prec == 1 && (rm == Ceiling || rm == Up) {
                3
            } else {
                2
            })
        );
        assert_ne!(o, Equal);
        if o == Less {
            let (e_alt, o_alt) = Float::e_prec_round(prec, Ceiling);
            let mut next_upper = e.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(e_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !e.is_power_of_2() {
            let (e_alt, o_alt) = Float::e_prec_round(prec, Floor);
            let mut next_lower = e.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(e_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_e, rug_o) = rug_e_prec_round(prec, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_e)),
                ComparableFloatRef(&e)
            );
            assert_eq!(rug_o, o);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::e_prec_round(prec, Exact));
    });

    test_constant(Float::e_prec_round, 10000);
}
