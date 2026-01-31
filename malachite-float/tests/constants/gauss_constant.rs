// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::GaussConstant;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{test_constant, to_hex_string};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_gauss_constant_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::gauss_constant_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);
}

#[test]
pub fn test_gauss_constant_prec() {
    test_gauss_constant_prec_helper(1, "1.0", "0x1.0#1", Greater);
    test_gauss_constant_prec_helper(2, "0.8", "0x0.c#2", Less);
    test_gauss_constant_prec_helper(3, "0.9", "0x0.e#3", Greater);
    test_gauss_constant_prec_helper(4, "0.81", "0x0.d#4", Less);
    test_gauss_constant_prec_helper(5, "0.84", "0x0.d8#5", Greater);
    test_gauss_constant_prec_helper(6, "0.83", "0x0.d4#6", Less);
    test_gauss_constant_prec_helper(7, "0.836", "0x0.d6#7", Greater);
    test_gauss_constant_prec_helper(8, "0.836", "0x0.d6#8", Greater);
    test_gauss_constant_prec_helper(9, "0.834", "0x0.d58#9", Less);
    test_gauss_constant_prec_helper(10, "0.835", "0x0.d5c#10", Greater);
    test_gauss_constant_prec_helper(
        100,
        "0.834626841674073186281429732799",
        "0x0.d5aa1acd5a9a1f6b126ed4160#100",
        Less,
    );
    test_gauss_constant_prec_helper(
        1000,
        "0.834626841674073186281429732799046808993993013490347002449827370103681992709526411869691\
        160351275324129067850352412010086724789007634750392659060526742712560320685998973751521461\
        574107190667714762311244616644051871383967845140283869200451381335890758553020498480052804\
        4693164358093689789568906891741207",
        "0x0.d5aa1acd5a9a1f6b126ed416015390b8dc5fceee4c86afc8c271f049361406e4cdd9089ef0715384d6c72\
        c4bdd754b2bc1ecc5c199c686439032cbb16813342ded4cffc19dcab8db34c27b6168ceb88298ed572b191c8f3\
        05743e1f293dfd41f82b0210f65e6163e5434a891261dc7440e2093b1a156a8e05ae8319d30#1000",
        Greater,
    );
    test_gauss_constant_prec_helper(
        10000,
        "0.834626841674073186281429732799046808993993013490347002449827370103681992709526411869691\
        160351275324129067850352412010086724789007634750392659060526742712560320685998973751521461\
        574107190667714762311244616644051871383967845140283869200451381335890758553020498480052804\
        469316435809368978956890689174120737291094066558175187025477401074678379716526356753331906\
        545452517692961030560054068494662548964218614294720752148653658161076202146037520348775359\
        620856771168292200325359109589899893069833199696977335883739104959085967402918246956874691\
        648574519106727574078611438142761058018432034542615493754610095708911752106842013580331247\
        539219381390232390909523434088742226918438886299422225926335216394780110220990752149069396\
        430836038257717941194870596739642726558350848321693971336950595116238828151547470228642077\
        757753710601888047232489046185253018895970256898180138788048957359293465403946331321583387\
        855328545224934416417651732023496827033747209414764964964606237379439957051908666982548125\
        962465776495889388368485992365411189017490504837785319323332016970056895534269527949656522\
        591691440712533718989871283605282652705310788877986415746880714285132363349955706585164159\
        241908262606339549845864348827078931363082390967437220434580842087307738346482679475197152\
        331746478597653712438272228263231954311739836817821079539242107816783313273533095656482023\
        584565825460564836279530167452398262582723504220166256671818402486313168189187792969542212\
        049101757196085075106858844201730767102408667492711144920904088412131757268261002709591485\
        892572769623193922808548994645936128594734522557112869973628301289234178756660025395220971\
        552551454810467511328914938148018407394784903305080042952463744828221428366721013966075942\
        176793399501825273207819753276434605496999824553994689519587067068339386727318342025784480\
        812767824268457965573220366984271657187184409113540094514027074844640093106297080444683824\
        377405875474810493604461011348533762134305687849293031921110040424810842901352348537310269\
        206343713905334007912336241882284201504403580321299174976797884816704896804726358250985559\
        799985988186958064670880371662368698113531302487616884846859115258187407737403728723368707\
        323846651707183803490851996303220801277270427901677230978957397647160240115820018502810895\
        154331934225555668095902625013638806356733750900152886422306611378111421987550050293430845\
        245916855621247493144225520107062263328132881734912860608708345845489197324468978356848814\
        153016313843435670624732166396406812362503527093774812335463520844348918966133198354326738\
        496326752080510329079510994322418056607631925724340293558660223517800973234536235348928246\
        383901175664232107529211451027625980008171999768528451504191728443523334216985440123487511\
        813092014431776472273139813296105395494814393977203673426345745101500312996611664855878256\
        912980607401095185355559714095036916705107898036880918260750972195965805299757075605874207\
        106344998902706621017515778393586780555045138364087216847932503796899969299231484446771505\
        66063511739512880228167240763662857429382489",
        "0x0.d5aa1acd5a9a1f6b126ed416015390b8dc5fceee4c86afc8c271f049361406e4cdd9089ef0715384d6c72\
        c4bdd754b2bc1ecc5c199c686439032cbb16813342ded4cffc19dcab8db34c27b6168ceb88298ed572b191c8f3\
        05743e1f293dfd41f82b0210f65e6163e5434a891261dc7440e2093b1a156a8e05ae8319d2ff97641b5a7fd78f\
        7cf6dc96868e4130d819e5f2ca3a94c6112e9b5a5801bfe4a11e5a42582d5081ac41fa9b728b4ac35e244d5a85\
        69a511aa70984b50c4070684a6ac159bdc9325d1fde1271f02527cc064b69ec913f415531392712d3fb1c34ed3\
        c075bef86d6a059f7a22834767ad03e455760919f51006e91df313136656ef23512f1e105bfa2eb5f06fef2837\
        36474330c7c0bac59657d9963f201efba238eaa0c933f0989864323803599e6dc26aef9c7fa5f268f531f503b3\
        af964f171b4980fb0cb845df9f68e7393d3471de7195e248e795b9225a0b176c4deecd2390e150a7ca76c1f3dd\
        f86cce329f30053a570d269f38de7c3d7475abb273c84dbbbc9e2469b486e86b3f792f8c039bb07b126676c07b\
        33332595fc7d9f877476f088f075265ed7f675bbcc8e6d8ada2f52abb307a3ebc73a2639ecf3287b4c2a68c18e\
        03b158d2ddfabb3f14826dc5a16e5b0d15b6dc83d22f25505ed5a58a81e4fb874d6fa23df6989cd319003989f0\
        8e14d201f949eff049dcb40d2e4ee6a09a9cb8a8ed98c3cf66f7aadd8ab1107e0777170849e24b32d5342ba17d\
        e3db8816045423a00fd6feebadc650d05a47f26ef7b184793feab8a0f72fda537794ec1db7ba60bae81a34044d\
        f64da299718b09ab02787b15c0cc87c047594f33d4955d2fa6b272582fedc6252a9c2143e9b5c510a52bd37d1b\
        8b03a88f3f3981ee8fb77b336a301bcf19ad8bffac5815954340690f235ebc2f763fd7beff9bee4393243bd467\
        8ba486780982c6ddb3e52a1a7c220d85f80cd263b3c39039c67d2105008bfc82b656b0a569fef2af16435a097a\
        066864fa7f39d24e36a9e4ee1e48fb7520d941e1c3d998a423d7323624de96e0862f19531652e4a4c7fd843112\
        48118ebe1f71610abf8bd41e859c8fd1087afdb24745cb6cd03af58f79904a9b1aa2e2dc5571c564cc3276b76f\
        a2003f5fc9e83956ab4c37848e2537ea2bbb81600b49441a0246fa5972e98b073069cc693abad679c6e5af259b\
        9a3418f9a2c0eb07f4bee61de65e0c4cac92dc7e814e9e710fef4117be6ceae26e295b32403823e99471d39256\
        22751aa788ff76be3ade62a196e546ab1f24bb973e914cc5b9fdf6baa5b5ae0ec5513fb76772007bacb0b1d514\
        f9045622b309cf750c162aaf417ff93536fc9a21c95909021f3f92c404825a0615e27cc5b5c309636ee7d0ade9\
        e7974b4a7c5dbb74ac75f2adc6929afe15b9891b7d9217c366f601132c9e5e7f7a2d05a80035e9cebc097affd5\
        0a7e73a3fc24a15efc02146f08b666cb29b7855e06e9bff03beaf3d27b2d8c1986ee6b77d51078df5b8194951c\
        2f9cc61e37de23b068699ee0af89a23c09235b52b15c7003c8a4ebb021a6e1e30411487ce16815548f919f2288\
        209aca02dad50dae05cf6669f49c173ec9e988c850db3ab92226478c1a6349300af076f75432e01bcebf00fc94\
        e171e049a257068125002014c8ec94daab6f5fc58d480f3ea11937610c5c72e1ba18d8440d6c4f583cb3453b9d\
        9f5effe308d79101081796211c92bf38dcd19353835166bb09cc2bd541798f16eaa8df0183d#10000",
        Greater,
    );

    let gauss_constant_f32 = Float::gauss_constant_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(gauss_constant_f32.to_string(), "0.83462685");
    assert_eq!(to_hex_string(&gauss_constant_f32), "0x0.d5aa1b#24");
    assert_eq!(gauss_constant_f32, f32::GAUSS_CONSTANT);

    let gauss_constant_f64 = Float::gauss_constant_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(gauss_constant_f64.to_string(), "0.8346268416740732");
    assert_eq!(to_hex_string(&gauss_constant_f64), "0x0.d5aa1acd5a9a20#53");
    assert_eq!(gauss_constant_f64, f64::GAUSS_CONSTANT);
}

#[test]
#[should_panic]
fn gauss_constant_prec_fail_1() {
    Float::gauss_constant_prec(0);
}

fn test_gauss_constant_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::gauss_constant_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);
}

#[test]
pub fn test_gauss_constant_prec_round() {
    test_gauss_constant_prec_round_helper(1, Floor, "0.5", "0x0.8#1", Less);
    test_gauss_constant_prec_round_helper(1, Ceiling, "1.0", "0x1.0#1", Greater);
    test_gauss_constant_prec_round_helper(1, Down, "0.5", "0x0.8#1", Less);
    test_gauss_constant_prec_round_helper(1, Up, "1.0", "0x1.0#1", Greater);
    test_gauss_constant_prec_round_helper(1, Nearest, "1.0", "0x1.0#1", Greater);

    test_gauss_constant_prec_round_helper(2, Floor, "0.8", "0x0.c#2", Less);
    test_gauss_constant_prec_round_helper(2, Ceiling, "1.0", "0x1.0#2", Greater);
    test_gauss_constant_prec_round_helper(2, Down, "0.8", "0x0.c#2", Less);
    test_gauss_constant_prec_round_helper(2, Up, "1.0", "0x1.0#2", Greater);
    test_gauss_constant_prec_round_helper(2, Nearest, "0.8", "0x0.c#2", Less);

    test_gauss_constant_prec_round_helper(3, Floor, "0.8", "0x0.c#3", Less);
    test_gauss_constant_prec_round_helper(3, Ceiling, "0.9", "0x0.e#3", Greater);
    test_gauss_constant_prec_round_helper(3, Down, "0.8", "0x0.c#3", Less);
    test_gauss_constant_prec_round_helper(3, Up, "0.9", "0x0.e#3", Greater);
    test_gauss_constant_prec_round_helper(3, Nearest, "0.9", "0x0.e#3", Greater);

    test_gauss_constant_prec_round_helper(4, Floor, "0.81", "0x0.d#4", Less);
    test_gauss_constant_prec_round_helper(4, Ceiling, "0.88", "0x0.e#4", Greater);
    test_gauss_constant_prec_round_helper(4, Down, "0.81", "0x0.d#4", Less);
    test_gauss_constant_prec_round_helper(4, Up, "0.88", "0x0.e#4", Greater);
    test_gauss_constant_prec_round_helper(4, Nearest, "0.81", "0x0.d#4", Less);

    test_gauss_constant_prec_round_helper(5, Floor, "0.81", "0x0.d0#5", Less);
    test_gauss_constant_prec_round_helper(5, Ceiling, "0.84", "0x0.d8#5", Greater);
    test_gauss_constant_prec_round_helper(5, Down, "0.81", "0x0.d0#5", Less);
    test_gauss_constant_prec_round_helper(5, Up, "0.84", "0x0.d8#5", Greater);
    test_gauss_constant_prec_round_helper(5, Nearest, "0.84", "0x0.d8#5", Greater);

    test_gauss_constant_prec_round_helper(6, Floor, "0.83", "0x0.d4#6", Less);
    test_gauss_constant_prec_round_helper(6, Ceiling, "0.84", "0x0.d8#6", Greater);
    test_gauss_constant_prec_round_helper(6, Down, "0.83", "0x0.d4#6", Less);
    test_gauss_constant_prec_round_helper(6, Up, "0.84", "0x0.d8#6", Greater);
    test_gauss_constant_prec_round_helper(6, Nearest, "0.83", "0x0.d4#6", Less);

    test_gauss_constant_prec_round_helper(7, Floor, "0.83", "0x0.d4#7", Less);
    test_gauss_constant_prec_round_helper(7, Ceiling, "0.836", "0x0.d6#7", Greater);
    test_gauss_constant_prec_round_helper(7, Down, "0.83", "0x0.d4#7", Less);
    test_gauss_constant_prec_round_helper(7, Up, "0.836", "0x0.d6#7", Greater);
    test_gauss_constant_prec_round_helper(7, Nearest, "0.836", "0x0.d6#7", Greater);

    test_gauss_constant_prec_round_helper(8, Floor, "0.832", "0x0.d5#8", Less);
    test_gauss_constant_prec_round_helper(8, Ceiling, "0.836", "0x0.d6#8", Greater);
    test_gauss_constant_prec_round_helper(8, Down, "0.832", "0x0.d5#8", Less);
    test_gauss_constant_prec_round_helper(8, Up, "0.836", "0x0.d6#8", Greater);
    test_gauss_constant_prec_round_helper(8, Nearest, "0.836", "0x0.d6#8", Greater);

    test_gauss_constant_prec_round_helper(9, Floor, "0.834", "0x0.d58#9", Less);
    test_gauss_constant_prec_round_helper(9, Ceiling, "0.836", "0x0.d60#9", Greater);
    test_gauss_constant_prec_round_helper(9, Down, "0.834", "0x0.d58#9", Less);
    test_gauss_constant_prec_round_helper(9, Up, "0.836", "0x0.d60#9", Greater);
    test_gauss_constant_prec_round_helper(9, Nearest, "0.834", "0x0.d58#9", Less);

    test_gauss_constant_prec_round_helper(10, Floor, "0.834", "0x0.d58#10", Less);
    test_gauss_constant_prec_round_helper(10, Ceiling, "0.835", "0x0.d5c#10", Greater);
    test_gauss_constant_prec_round_helper(10, Down, "0.834", "0x0.d58#10", Less);
    test_gauss_constant_prec_round_helper(10, Up, "0.835", "0x0.d5c#10", Greater);
    test_gauss_constant_prec_round_helper(10, Nearest, "0.835", "0x0.d5c#10", Greater);

    test_gauss_constant_prec_round_helper(
        100,
        Floor,
        "0.834626841674073186281429732799",
        "0x0.d5aa1acd5a9a1f6b126ed4160#100",
        Less,
    );
    test_gauss_constant_prec_round_helper(
        100,
        Ceiling,
        "0.8346268416740731862814297328",
        "0x0.d5aa1acd5a9a1f6b126ed4161#100",
        Greater,
    );
    test_gauss_constant_prec_round_helper(
        100,
        Down,
        "0.834626841674073186281429732799",
        "0x0.d5aa1acd5a9a1f6b126ed4160#100",
        Less,
    );
    test_gauss_constant_prec_round_helper(
        100,
        Up,
        "0.8346268416740731862814297328",
        "0x0.d5aa1acd5a9a1f6b126ed4161#100",
        Greater,
    );
    test_gauss_constant_prec_round_helper(
        100,
        Nearest,
        "0.834626841674073186281429732799",
        "0x0.d5aa1acd5a9a1f6b126ed4160#100",
        Less,
    );
}

#[test]
#[should_panic]
fn gauss_constant_prec_round_fail_1() {
    Float::gauss_constant_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn gauss_constant_prec_round_fail_2() {
    Float::gauss_constant_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn gauss_constant_prec_round_fail_3() {
    Float::gauss_constant_prec_round(1000, Exact);
}

#[test]
fn gauss_constant_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (gauss_constant, o) = Float::gauss_constant_prec(prec);
        assert!(gauss_constant.is_valid());
        assert_eq!(gauss_constant.get_prec(), Some(prec));
        assert_eq!(
            gauss_constant.get_exponent(),
            Some(if prec == 1 { 1 } else { 0 })
        );
        assert_ne!(o, Equal);
        if o == Less {
            let (gauss_constant_alt, o_alt) = Float::gauss_constant_prec_round(prec, Ceiling);
            let mut next_upper = gauss_constant.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(
                    ComparableFloat(gauss_constant_alt),
                    ComparableFloat(next_upper)
                );
                assert_eq!(o_alt, Greater);
            }
        } else if !gauss_constant.is_power_of_2() {
            let (gauss_constant_alt, o_alt) = Float::gauss_constant_prec_round(prec, Floor);
            let mut next_lower = gauss_constant.clone();
            next_lower.decrement();
            assert_eq!(
                ComparableFloat(gauss_constant_alt),
                ComparableFloat(next_lower)
            );
            assert_eq!(o_alt, Less);
        }
        let (gauss_constant_alt, o_alt) = Float::gauss_constant_prec_round(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&gauss_constant_alt),
            ComparableFloatRef(&gauss_constant)
        );
        assert_eq!(o_alt, o);
    });
}

#[test]
fn gauss_constant_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (gauss_constant, o) = Float::gauss_constant_prec_round(prec, rm);
        assert!(gauss_constant.is_valid());
        assert_eq!(gauss_constant.get_prec(), Some(prec));
        let expected_exponent = match (prec, rm) {
            (1, Ceiling | Up | Nearest) | (2, Ceiling | Up) => 1,
            _ => 0,
        };
        assert_eq!(gauss_constant.get_exponent(), Some(expected_exponent));
        assert_ne!(o, Equal);
        if o == Less {
            let (gauss_constant_alt, o_alt) = Float::gauss_constant_prec_round(prec, Ceiling);
            let mut next_upper = gauss_constant.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(
                    ComparableFloat(gauss_constant_alt),
                    ComparableFloat(next_upper)
                );
                assert_eq!(o_alt, Greater);
            }
        } else if !gauss_constant.is_power_of_2() {
            let (gauss_constant_alt, o_alt) = Float::gauss_constant_prec_round(prec, Floor);
            let mut next_lower = gauss_constant.clone();
            next_lower.decrement();
            assert_eq!(
                ComparableFloat(gauss_constant_alt),
                ComparableFloat(next_lower)
            );
            assert_eq!(o_alt, Less);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::gauss_constant_prec_round(prec, Exact));
    });

    test_constant(Float::gauss_constant_prec_round, 10000);
}
