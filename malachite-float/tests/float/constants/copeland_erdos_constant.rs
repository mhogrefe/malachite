// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::apply_fn_to_primitive_floats;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::CopelandErdosConstant;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_gen_var_31, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::float::constants::copeland_erdos_constant::*;
use malachite_float::test_util::common::{test_constant, to_hex_string};
use malachite_float::test_util::float::constants::digit_constants::*;
use malachite_float::test_util::generators::{
    unsigned_pair_gen_var_51, unsigned_unsigned_rounding_mode_triple_gen_var_10,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};

fn test_copeland_erdos_constant_base_prec_helper(
    base: u64,
    prec: u64,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::copeland_erdos_constant_base_prec(base, prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = copeland_erdos_constant_base_prec_round_naive(base, prec, Nearest);
    assert_eq!(x, x_alt);
    assert_eq!(o, o_alt);
}

#[test]
fn test_copeland_erdos_constant_base_prec() {
    test_copeland_erdos_constant_base_prec_helper(
        10,
        100,
        "0.23571113171923293137414347535966",
        "0x0.3c579092098975475a5c13b988#100",
        Greater,
    );
    test_copeland_erdos_constant_base_prec_helper(
        2,
        100,
        "0.73412151540828612060627828845711",
        "0x0.bbef633bf7f2d35dfaf7ec38f#100",
        Less,
    );
    test_copeland_erdos_constant_base_prec_helper(
        3,
        100,
        "0.80174949296954506695856974572093",
        "0x0.cd3f746be386f441c9c14f7e6#100",
        Greater,
    );
    test_copeland_erdos_constant_base_prec_helper(
        4,
        100,
        "0.70892073664967333384159187864799",
        "0x0.b57bd4535dd7e5a6bbf5efd43#100",
        Less,
    );
    test_copeland_erdos_constant_base_prec_helper(
        5,
        100,
        "0.52847756409722636280369942963898",
        "0x0.874a4e3e77a3010169d5efc32#100",
        Less,
    );
    test_copeland_erdos_constant_base_prec_helper(
        6,
        100,
        "0.44075565361303488943827290921585",
        "0x0.70d55ccdcb8af56491e75744d0#100",
        Less,
    );
    test_copeland_erdos_constant_base_prec_helper(
        7,
        100,
        "0.36194621598641193889354243902374",
        "0x0.5ca881d892955decc775717290#100",
        Less,
    );
    test_copeland_erdos_constant_base_prec_helper(
        8,
        100,
        "0.30839236201960848961509949241377",
        "0x0.4ef2cd4535dd7e5a6bbf5efd20#100",
        Less,
    );
    test_copeland_erdos_constant_base_prec_helper(
        9,
        100,
        "0.26720588573555705887578396963783",
        "0x0.44679adc886e21a532d48e4c68#100",
        Greater,
    );
    test_copeland_erdos_constant_base_prec_helper(
        11,
        100,
        "0.21085252419192612191515695111220",
        "0x0.35fa6e57aef16f9d2b9c870d04#100",
        Greater,
    );
    test_copeland_erdos_constant_base_prec_helper(
        12,
        100,
        "0.19077566833857803164153539470077",
        "0x0.30d6ac98630115ae31d2752ea8#100",
        Less,
    );
    test_copeland_erdos_constant_base_prec_helper(
        13,
        100,
        "0.17414838826010437219140026117107",
        "0x0.2c94fd203a6f282c65980b8f44#100",
        Greater,
    );
    test_copeland_erdos_constant_base_prec_helper(
        14,
        100,
        "0.16018982939086204811564349544946",
        "0x0.2902335e62b4da5ebfba656cac#100",
        Less,
    );
    test_copeland_erdos_constant_base_prec_helper(
        15,
        100,
        "0.14830205330549419117160249160159",
        "0x0.25f71f94e072712ee8e353ae50#100",
        Less,
    );
    test_copeland_erdos_constant_base_prec_helper(
        16,
        100,
        "0.13805753390178350683643564212329",
        "0x0.2357bd1113171d1f25292b2f34#100",
        Less,
    );
}

fn test_copeland_erdos_constant_base_prec_round_helper(
    base: u64,
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::copeland_erdos_constant_base_prec_round(base, prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = copeland_erdos_constant_base_prec_round_naive(base, prec, rm);
    assert_eq!(x, x_alt);
    assert_eq!(o, o_alt);
}

#[test]
fn test_copeland_erdos_constant_base_prec_round() {
    test_copeland_erdos_constant_base_prec_round_helper(
        2,
        100,
        Floor,
        "0.73412151540828612060627828845711",
        "0x0.bbef633bf7f2d35dfaf7ec38f#100",
        Less,
    );
    test_copeland_erdos_constant_base_prec_round_helper(
        2,
        100,
        Ceiling,
        "0.73412151540828612060627828845790",
        "0x0.bbef633bf7f2d35dfaf7ec390#100",
        Greater,
    );
    test_copeland_erdos_constant_base_prec_round_helper(
        2,
        100,
        Down,
        "0.73412151540828612060627828845711",
        "0x0.bbef633bf7f2d35dfaf7ec38f#100",
        Less,
    );
    test_copeland_erdos_constant_base_prec_round_helper(
        2,
        100,
        Up,
        "0.73412151540828612060627828845790",
        "0x0.bbef633bf7f2d35dfaf7ec390#100",
        Greater,
    );
    test_copeland_erdos_constant_base_prec_round_helper(
        2,
        100,
        Nearest,
        "0.73412151540828612060627828845711",
        "0x0.bbef633bf7f2d35dfaf7ec38f#100",
        Less,
    );
    test_copeland_erdos_constant_base_prec_round_helper(
        10,
        100,
        Floor,
        "0.23571113171923293137414347535946",
        "0x0.3c579092098975475a5c13b984#100",
        Less,
    );
    test_copeland_erdos_constant_base_prec_round_helper(
        10,
        100,
        Ceiling,
        "0.23571113171923293137414347535966",
        "0x0.3c579092098975475a5c13b988#100",
        Greater,
    );
    test_copeland_erdos_constant_base_prec_round_helper(
        10,
        100,
        Down,
        "0.23571113171923293137414347535946",
        "0x0.3c579092098975475a5c13b984#100",
        Less,
    );
    test_copeland_erdos_constant_base_prec_round_helper(
        10,
        100,
        Up,
        "0.23571113171923293137414347535966",
        "0x0.3c579092098975475a5c13b988#100",
        Greater,
    );
    test_copeland_erdos_constant_base_prec_round_helper(
        10,
        100,
        Nearest,
        "0.23571113171923293137414347535966",
        "0x0.3c579092098975475a5c13b988#100",
        Greater,
    );
}

// Successive Floor approximations must be bit-prefixes of one another, in every base.
#[test]
fn test_copeland_erdos_constant_base_prefixes() {
    for base in 2..=16 {
        test_constant(
            |prec, rm| Float::copeland_erdos_constant_base_prec_round(base, prec, rm),
            100,
        );
    }
}

#[test]
#[should_panic]
fn copeland_erdos_constant_base_prec_round_fail_1() {
    Float::copeland_erdos_constant_base_prec_round(10, 0, Floor);
}

#[test]
#[should_panic]
fn copeland_erdos_constant_base_prec_round_fail_2() {
    Float::copeland_erdos_constant_base_prec_round(10, 100, Exact);
}

#[test]
#[should_panic]
fn copeland_erdos_constant_base_prec_round_fail_3() {
    Float::copeland_erdos_constant_base_prec_round(1, 100, Floor);
}

#[test]
#[should_panic]
fn copeland_erdos_constant_base_prec_round_fail_4() {
    Float::copeland_erdos_constant_base_prec_round(0, 100, Floor);
}

#[test]
#[should_panic]
fn copeland_erdos_constant_base_prec_fail_1() {
    Float::copeland_erdos_constant_base_prec(10, 0);
}

#[test]
#[should_panic]
fn copeland_erdos_constant_base_prec_fail_2() {
    Float::copeland_erdos_constant_base_prec(1, 100);
}

#[test]
fn copeland_erdos_constant_base_prec_properties() {
    unsigned_pair_gen_var_51().test_properties(|(base, prec)| {
        let (x, o) = Float::copeland_erdos_constant_base_prec(base, prec);
        assert!(x.is_valid());
        assert_eq!(x.get_prec(), Some(prec));
        // The constant is irrational, so no precision can represent it exactly.
        assert_ne!(o, Equal);
        if o == Less {
            let (x_alt, o_alt) =
                Float::copeland_erdos_constant_base_prec_round(base, prec, Ceiling);
            let mut next_upper = x.clone();
            next_upper.increment();
            assert_eq!(ComparableFloat(x_alt), ComparableFloat(next_upper));
            assert_eq!(o_alt, Greater);
        } else {
            let (x_alt, o_alt) = Float::copeland_erdos_constant_base_prec_round(base, prec, Floor);
            let mut next_lower = x.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(x_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
        let (x_alt, o_alt) = Float::copeland_erdos_constant_base_prec_round(base, prec, Nearest);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
        assert_eq!(o_alt, o);

        let (x_alt, o_alt) = copeland_erdos_constant_base_prec_round_naive(base, prec, Nearest);
        assert_eq!(x, x_alt);
        assert_eq!(o, o_alt);
    });
}

#[test]
fn copeland_erdos_constant_base_prec_round_properties() {
    unsigned_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(base, prec, rm)| {
        let (x, o) = Float::copeland_erdos_constant_base_prec_round(base, prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.get_prec(), Some(prec));
        assert_ne!(o, Equal);
        // The constant is positive, so `Down` is `Floor` and `Up` is `Ceiling`.
        match rm {
            Floor | Down => assert_eq!(o, Less),
            Ceiling | Up => assert_eq!(o, Greater),
            Nearest => {}
            Exact => unreachable!(),
        }
        let (x_alt, o_alt) = Float::copeland_erdos_constant_base_prec_round(
            base,
            prec,
            if o == Less { Floor } else { Ceiling },
        );
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
        assert_eq!(o_alt, o);

        let (x_alt, o_alt) = copeland_erdos_constant_base_prec_round_naive(base, prec, rm);
        assert_eq!(x, x_alt);
        assert_eq!(o, o_alt);
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_copeland_erdos_constant_base() {
    fn test<T: PrimitiveFloat>(base: u64, out: T)
    where
        Float: PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_copeland_erdos_constant_base::<T>(base)),
            NiceFloat(out)
        );
    }
    test::<f32>(2, 0.7341215);
    test::<f32>(3, 0.80174947);
    test::<f32>(4, 0.7089207);
    test::<f32>(5, 0.52847755);
    test::<f32>(6, 0.44075567);
    test::<f32>(7, 0.36194623);
    test::<f32>(8, 0.30839238);
    test::<f32>(9, 0.2672059);
    test::<f32>(10, 0.23571113);
    test::<f32>(11, 0.21085252);
    test::<f32>(12, 0.19077566);
    test::<f32>(13, 0.1741484);
    test::<f32>(14, 0.16018982);
    test::<f32>(15, 0.14830205);
    test::<f32>(16, 0.13805753);
    test::<f32>(62, 0.033059966);
    test::<f32>(1000, 0.002003005);
    test::<f32>(18446744073709551615, 1.0842022e-19);

    test::<f64>(2, 0.7341215154082861);
    test::<f64>(3, 0.8017494929695451);
    test::<f64>(4, 0.7089207366496734);
    test::<f64>(5, 0.5284775640972263);
    test::<f64>(6, 0.44075565361303487);
    test::<f64>(7, 0.3619462159864119);
    test::<f64>(8, 0.3083923620196085);
    test::<f64>(9, 0.26720588573555704);
    test::<f64>(10, 0.23571113171923294);
    test::<f64>(11, 0.21085252419192613);
    test::<f64>(12, 0.19077566833857804);
    test::<f64>(13, 0.17414838826010437);
    test::<f64>(14, 0.16018982939086204);
    test::<f64>(15, 0.1483020533054942);
    test::<f64>(16, 0.13805753390178352);
    test::<f64>(62, 0.033059967022534574);
    test::<f64>(1000, 0.002003005007011013);
    test::<f64>(18446744073709551615, 1.0842021724855044e-19);
}

#[test]
#[should_panic]
fn primitive_float_copeland_erdos_constant_base_fail_1() {
    primitive_float_copeland_erdos_constant_base::<f32>(1);
}

#[test]
#[should_panic]
fn primitive_float_copeland_erdos_constant_base_fail_2() {
    primitive_float_copeland_erdos_constant_base::<f64>(0);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_copeland_erdos_constant_base_properties_helper<T: PrimitiveFloat>()
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    unsigned_gen_var_31::<u64>().test_properties(|base| {
        let x = primitive_float_copeland_erdos_constant_base::<T>(base);
        // The constant lies in [1/base, 1), so it is always finite, positive, and normal.
        assert!(x.is_finite());
        assert!(x > T::ZERO);
        assert!(x < T::ONE);
        // Computing at a much higher precision and rounding once must give the same answer, which
        // is what correct rounding means.
        let (y, _) = Float::copeland_erdos_constant_base_prec(base, 200);
        assert_eq!(NiceFloat(x), NiceFloat(T::rounding_from(&y, Nearest).0));
    });
}

#[test]
fn primitive_float_copeland_erdos_constant_base_properties() {
    apply_fn_to_primitive_floats!(primitive_float_copeland_erdos_constant_base_properties_helper);
}

fn test_copeland_erdos_constant_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::copeland_erdos_constant_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    // The base-10 specialization must agree with the general function at base 10.
    let (x_alt, o_alt) = Float::copeland_erdos_constant_base_prec(10, prec);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);

    let (x_alt, o_alt) = copeland_erdos_constant_base_prec_round_naive(10, prec, Nearest);
    assert_eq!(x, x_alt);
    assert_eq!(o, o_alt);
}

#[test]
fn test_copeland_erdos_constant_prec() {
    test_copeland_erdos_constant_prec_helper(1, "0.25", "0x0.4#1", Greater);
    test_copeland_erdos_constant_prec_helper(2, "0.25", "0x0.4#2", Greater);
    test_copeland_erdos_constant_prec_helper(3, "0.25", "0x0.4#3", Greater);
    test_copeland_erdos_constant_prec_helper(4, "0.234", "0x0.3c#4", Less);
    test_copeland_erdos_constant_prec_helper(5, "0.234", "0x0.3c#5", Less);
    test_copeland_erdos_constant_prec_helper(10, "0.23560", "0x0.3c5#10", Less);
    test_copeland_erdos_constant_prec_helper(
        100,
        "0.23571113171923293137414347535966",
        "0x0.3c579092098975475a5c13b988#100",
        Greater,
    );
    test_copeland_erdos_constant_prec_helper(
        1000,
        "0.235711131719232931374143475359616771737983899710110310710911312713113713914915115716316\
        717317918119119319719921122322722923323924125125726326927127728128329330731131331733133734\
        734935335936737337938338939740140941942143143343944344945746146346747948749149950350952152\
        354154755756356957157758759359960162",
        "0x0.3c579092098975475a5c13b98722201f14db50c3913347edebb63403a0bb0bdb97267f94e7e509bf9fb19\
        a9154b69e1d96c28992ce8978f8f424559175799faaa6e74125c0eae32b01777b65c803d69737f894725c68fda\
        1c8abba0baf34c3be801fa7f89a3e96b8ca6f32164ff6eede402bb6bbe4d3a37e80f01969054#1000",
        Greater,
    );
    test_copeland_erdos_constant_prec_helper(
        10000,
        "0.235711131719232931374143475359616771737983899710110310710911312713113713914915115716316\
        717317918119119319719921122322722923323924125125726326927127728128329330731131331733133734\
        734935335936737337938338939740140941942143143343944344945746146346747948749149950350952152\
        354154755756356957157758759359960160761361761963164164364765365966167367768369170170971972\
        773373974375175776176977378779780981182182382782983985385785986387788188388790791191992993\
        794194795396797197798399199710091013101910211031103310391049105110611063106910871091109310\
        971103110911171123112911511153116311711181118711931201121312171223122912311237124912591277\
        127912831289129112971301130313071319132113271361136713731381139914091423142714291433143914\
        471451145314591471148114831487148914931499151115231531154315491553155915671571157915831597\
        160116071609161316191621162716371657166316671669169316971699170917211723173317411747175317\
        591777178317871789180118111823183118471861186718711873187718791889190119071913193119331949\
        195119731979198719931997199920032011201720272029203920532063206920812083208720892099211121\
        132129213121372141214321532161217922032207221322212237223922432251226722692273228122872293\
        229723092311233323392341234723512357237123772381238323892393239924112417242324372441244724\
        592467247324772503252125312539254325492551255725792591259326092617262126332647265726592663\
        267126772683268726892693269927072711271327192729273127412749275327672777278927912797280128\
        032819283328372843285128572861287928872897290329092917292729392953295729632969297129993001\
        301130193023303730413049306130673079308330893109311931213137316331673169318131873191320332\
        093217322132293251325332573259327132993301330733133319332333293331334333473359336133713373\
        338933913407341334333449345734613463346734693491349935113517352735293533353935413547355735\
        593571358135833593360736133617362336313637364336593671367336773691369737013709371937273733\
        373937613767376937793793379738033821382338333847385138533863387738813889390739113917391939\
        233929393139433947396739894001400340074013401940214027404940514057407340794091409340994111\
        412741294133413941534157415941774201421142174219422942314241424342534259426142714273428342\
        894297432743374339434943574363437343914397440944214423444144474451445744634481448344934507\
        451345174519452345474549456145674583459145974603462146374639464346494651465746634673467946\
        914703472147234729473347514759478347874789479347994801481348174831486148714877488949034909\
        491949314933493749434951495749674969497349874993499950035009501150215023503950515059507750\
        815087509951015107511351195147515351675171517951895197520952275231523352375261527352795281\
        529753035309532353335347535153815387539353995407541354175419543154375441544354495471547754\
        795483550155035507551955215527553155575563556955735581559156235639564156475651565356575659\
        566956835689569357015711571757375741574357495779578357915801580758135821582758395843584958\
        515857586158675869587958815897590359235927593959535981598760076011602960376043604760536067\
        607360796089609161016113612161316133614361514",
        "0x0.3c579092098975475a5c13b98722201f14db50c3913347edebb63403a0bb0bdb97267f94e7e509bf9fb19\
        a9154b69e1d96c28992ce8978f8f424559175799faaa6e74125c0eae32b01777b65c803d69737f894725c68fda\
        1c8abba0baf34c3be801fa7f89a3e96b8ca6f32164ff6eede402bb6bbe4d3a37e80f01969053c038036759ab70\
        f746f350db5a8876f64721cbffe558d8443495d7a287d089085b9c55b67c1bc2c39cbcb1e52526df097719a098\
        8b5ef9b02aa3873e85fccced8fdee46af5f56563a3feed663d1044e08412a5577450c0aa65aeef1b15e1952e42\
        ddea2eb912b75faf3be2d4190ba6b8d442073f2b5d2527f16d2d48967ee649933f58f2f2dbb06cdc2b7309bb1e\
        bcb2bf7845c8043bf0a6bab9193981b989ed76dfd23b04134757ad09f05db5956af9432054878d7e665db278cf\
        6bd49c4417f3080d9aa67b49c6d971c3e72663880325ce2a9b5f9ca9e7d18c432ddde2d8246052feec2f1677ae\
        79d8e6e13494088566d137638356bcf84ff9df34e9024712cea1ac986e7ac2497cbe534d70cda928d3315e2faf\
        08150ae7f3a4e389b66052f6e5a9dee6e85ac512d6284488849ea0b48bb78a424d8fa095d0871ccc208adff598\
        db6458b084c930ba6808cbfb69966b14e5160459e98abe38d8b311b355da611920350209795bd1f054ca5aee93\
        83bfcb468dac0f4660095e60d4398c0f27c39a3f0da5307c1189f5a4a8951c7be59060a38209329e7607c65aef\
        c19ed7e8c3d33de67ea3abf680a35c523f2982a12ff1f426e4739173d8819ebd1d39b891c62e076c9532942ca7\
        af0197824fd722e963e3306e393ada4efb2a5ba54c2c0362c64fc21027e918e309a8925d1eb64e25d91f291413\
        cb7bcf7accf8a549d03679122c88562d30023ff746454e3323fa5e765ad54f56eda55fdfacdb23d4a208d58268\
        11e016413829d6fe84ae28e4e5b1edc06154c7f49717f3c30ecb702dbb8ef03e347815cfdc3ea7f45b56d306fc\
        326b67ca0fe82b71e0d6a0355ab774d63a863e85dcf4bb80f5f84acc60cc69c4dad72ac075bd3821f016c04bd9\
        aa722ccd4fa27246fb133df7c3c04150d8a3b31ee914945faf773a11871ffaba423cbff238b53b9de7269b7128\
        1b92a9e638d11d03e585c23a3e8be4885250b0699de8919b18c1458782009ca0620ac04b25f4ad6e68939ec7b9\
        064ad47453e5b7a45e7ab3b972b9c05d483c20c11ddf007f6f640fee228377193b15e93ecbe214504ed9bf02e5\
        74cf3ff8f25312e50138045fc8739bd2ce12ac2b4cba1609790219d4e1e373c9b06a41aa6fa31fc74eba7c7b88\
        aebe5cbb0c431a4d67a20136111afecbe9714ec4e354f85476ba7d2ceb4e9e0d690c19c578130e51cc12761bab\
        f4fd27ba8462075d434371157e0ced613536751d7b96b183c959e1f8ef23755d6561d42deac90cfe2f27aec1ee\
        dba554df1e8af71086eea9591cd2d75ebc996ffcc6da7e32df5ca575c3e9052a26d22d7917b55f4f26f78fa5a9\
        07207ed971e54662ca26d08f117b06c2d476ce55a34e34d409cd12f705274417016c5e8b5878022d1e4ad467d2\
        5e9ee7a22b1b11d62cc54badd29a1c8368439862fdd4e6b1c547d1db0e96bf0c069f611195b67d51d198a8776d\
        8b3890fecaf3c68963949a62dda06eb318ca666ee013fc64de2ed90167a70eb4dce12421248173297af4337b73\
        bb4103e9d97d32f0e368456709e87d76c38ee64f977ca03e17a51b0102f182bfce53af09400c#10000",
        Less,
    );

    let x_f32 = Float::copeland_erdos_constant_prec(24).0;
    assert_eq!(x_f32.to_string(), "0.235711128");
    assert_eq!(to_hex_string(&x_f32), "0x0.3c57908#24");
    assert_eq!(x_f32, f32::COPELAND_ERDOS_CONSTANT);

    let x_f64 = Float::copeland_erdos_constant_prec(53).0;
    assert_eq!(x_f64.to_string(), "0.23571113171923294");
    assert_eq!(to_hex_string(&x_f64), "0x0.3c579092098976#53");
    assert_eq!(x_f64, f64::COPELAND_ERDOS_CONSTANT);
}

fn test_copeland_erdos_constant_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::copeland_erdos_constant_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = Float::copeland_erdos_constant_base_prec_round(10, prec, rm);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);

    let (x_alt, o_alt) = copeland_erdos_constant_base_prec_round_naive(10, prec, rm);
    assert_eq!(x, x_alt);
    assert_eq!(o, o_alt);
}

#[test]
fn test_copeland_erdos_constant_prec_round() {
    test_copeland_erdos_constant_prec_round_helper(1, Floor, "0.12", "0x0.2#1", Less);
    test_copeland_erdos_constant_prec_round_helper(1, Ceiling, "0.25", "0x0.4#1", Greater);
    test_copeland_erdos_constant_prec_round_helper(1, Down, "0.12", "0x0.2#1", Less);
    test_copeland_erdos_constant_prec_round_helper(1, Up, "0.25", "0x0.4#1", Greater);
    test_copeland_erdos_constant_prec_round_helper(1, Nearest, "0.25", "0x0.4#1", Greater);
    test_copeland_erdos_constant_prec_round_helper(2, Floor, "0.19", "0x0.3#2", Less);
    test_copeland_erdos_constant_prec_round_helper(2, Ceiling, "0.25", "0x0.4#2", Greater);
    test_copeland_erdos_constant_prec_round_helper(2, Down, "0.19", "0x0.3#2", Less);
    test_copeland_erdos_constant_prec_round_helper(2, Up, "0.25", "0x0.4#2", Greater);
    test_copeland_erdos_constant_prec_round_helper(2, Nearest, "0.25", "0x0.4#2", Greater);
    test_copeland_erdos_constant_prec_round_helper(3, Floor, "0.22", "0x0.38#3", Less);
    test_copeland_erdos_constant_prec_round_helper(3, Ceiling, "0.25", "0x0.4#3", Greater);
    test_copeland_erdos_constant_prec_round_helper(3, Down, "0.22", "0x0.38#3", Less);
    test_copeland_erdos_constant_prec_round_helper(3, Up, "0.25", "0x0.4#3", Greater);
    test_copeland_erdos_constant_prec_round_helper(3, Nearest, "0.25", "0x0.4#3", Greater);
    test_copeland_erdos_constant_prec_round_helper(4, Floor, "0.234", "0x0.3c#4", Less);
    test_copeland_erdos_constant_prec_round_helper(4, Ceiling, "0.250", "0x0.40#4", Greater);
    test_copeland_erdos_constant_prec_round_helper(4, Down, "0.234", "0x0.3c#4", Less);
    test_copeland_erdos_constant_prec_round_helper(4, Up, "0.250", "0x0.40#4", Greater);
    test_copeland_erdos_constant_prec_round_helper(4, Nearest, "0.234", "0x0.3c#4", Less);
    test_copeland_erdos_constant_prec_round_helper(5, Floor, "0.234", "0x0.3c#5", Less);
    test_copeland_erdos_constant_prec_round_helper(5, Ceiling, "0.242", "0x0.3e#5", Greater);
    test_copeland_erdos_constant_prec_round_helper(5, Down, "0.234", "0x0.3c#5", Less);
    test_copeland_erdos_constant_prec_round_helper(5, Up, "0.242", "0x0.3e#5", Greater);
    test_copeland_erdos_constant_prec_round_helper(5, Nearest, "0.234", "0x0.3c#5", Less);
    test_copeland_erdos_constant_prec_round_helper(
        100,
        Floor,
        "0.23571113171923293137414347535946",
        "0x0.3c579092098975475a5c13b984#100",
        Less,
    );
    test_copeland_erdos_constant_prec_round_helper(
        100,
        Ceiling,
        "0.23571113171923293137414347535966",
        "0x0.3c579092098975475a5c13b988#100",
        Greater,
    );
    test_copeland_erdos_constant_prec_round_helper(
        100,
        Down,
        "0.23571113171923293137414347535946",
        "0x0.3c579092098975475a5c13b984#100",
        Less,
    );
    test_copeland_erdos_constant_prec_round_helper(
        100,
        Up,
        "0.23571113171923293137414347535966",
        "0x0.3c579092098975475a5c13b988#100",
        Greater,
    );
    test_copeland_erdos_constant_prec_round_helper(
        100,
        Nearest,
        "0.23571113171923293137414347535966",
        "0x0.3c579092098975475a5c13b988#100",
        Greater,
    );
}

#[test]
#[should_panic]
fn copeland_erdos_constant_prec_fail() {
    Float::copeland_erdos_constant_prec(0);
}

#[test]
#[should_panic]
fn copeland_erdos_constant_prec_round_fail_1() {
    Float::copeland_erdos_constant_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn copeland_erdos_constant_prec_round_fail_2() {
    Float::copeland_erdos_constant_prec_round(100, Exact);
}

#[test]
fn copeland_erdos_constant_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (x, o) = Float::copeland_erdos_constant_prec(prec);
        assert!(x.is_valid());
        assert_eq!(x.get_prec(), Some(prec));
        assert_ne!(o, Equal);
        let (x_alt, o_alt) = Float::copeland_erdos_constant_base_prec(10, prec);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn copeland_erdos_constant_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (x, o) = Float::copeland_erdos_constant_prec_round(prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.get_prec(), Some(prec));
        assert_ne!(o, Equal);
        let (x_alt, o_alt) = Float::copeland_erdos_constant_base_prec_round(10, prec, rm);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
        assert_eq!(o_alt, o);
    });
}
