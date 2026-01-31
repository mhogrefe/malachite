// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::SqrtPi;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{test_constant, to_hex_string};
use malachite_float::test_util::constants::sqrt_pi::sqrt_pi_prec_round_simple;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_sqrt_pi_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::sqrt_pi_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = sqrt_pi_prec_round_simple(prec, Nearest);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);
}

#[test]
pub fn test_sqrt_pi_prec() {
    test_sqrt_pi_prec_helper(1, "2.0", "0x2.0#1", Greater);
    test_sqrt_pi_prec_helper(2, "2.0", "0x2.0#2", Greater);
    test_sqrt_pi_prec_helper(3, "1.8", "0x1.c#3", Less);
    test_sqrt_pi_prec_helper(4, "1.8", "0x1.c#4", Less);
    test_sqrt_pi_prec_helper(5, "1.75", "0x1.c#5", Less);
    test_sqrt_pi_prec_helper(6, "1.78", "0x1.c8#6", Greater);
    test_sqrt_pi_prec_helper(7, "1.77", "0x1.c4#7", Less);
    test_sqrt_pi_prec_helper(8, "1.773", "0x1.c6#8", Greater);
    test_sqrt_pi_prec_helper(9, "1.773", "0x1.c6#9", Greater);
    test_sqrt_pi_prec_helper(10, "1.771", "0x1.c58#10", Less);
    test_sqrt_pi_prec_helper(
        100,
        "1.772453850905516027298167483341",
        "0x1.c5bf891b4ef6aa79c3b0520d6#100",
        Greater,
    );
    test_sqrt_pi_prec_helper(
        1000,
        "1.772453850905516027298167483341145182797549456122387128213807789852911284591032181374950\
        656738544665416226823624282570666236152865724422602525093709602787068462037698653105122849\
        925173028950826228932095379267962800174639015351479720516700190185234018585446974494912640\
        3139217755259062164054193325009063",
        "0x1.c5bf891b4ef6aa79c3b0520d5db9383fe3921546f63b252dca100bd3ea14746ed76ffd6f941f1dbacd8e7\
        6141e0c77476c521446ddc35851524490c34a42174dcbafdf109d1b3acfb553ade465bea6c5032aa8a111a626e\
        2c702f124d16fb3a90b8b1717835c4e1ec9c08f40a3db6d8123741ca788a38e7a619ffb5bae#1000",
        Less,
    );
    test_sqrt_pi_prec_helper(
        10000,
        "1.772453850905516027298167483341145182797549456122387128213807789852911284591032181374950\
        656738544665416226823624282570666236152865724422602525093709602787068462037698653105122849\
        925173028950826228932095379267962800174639015351479720516700190185234018585446974494912640\
        313921775525906216405419332500906398407613733477475153433667989789365851836408795451165161\
        738760059067393431791332809854846248184902054654852195613251561647467515042738761056107996\
        127107210060372044483672365296613708094323498831668424213845709609120420427785778068694766\
        570005218305685125413396636944654181510716693883321942929357062268865224420542149948049920\
        756486398874838505930640218214029285811233064978945203621149078962287389403245978198513134\
        871266512506293260044656382109675026812496930595420461560761952217391525070207792758099054\
        332900662223067614469661248188743069978835205061464443854185307973574257179185635959749959\
        952263849242203889103966406447293972841345043002140564233433039261756134176336320017037654\
        163476320669276541812835762490326904508485320134192435989730871193799482938730111262561658\
        818884785977875963761363218634246546641333954355703201522654193952186030497310513829498439\
        659165614245955421226615102478536098095510395600789402188099613382854025016800745802729119\
        366425192820510001936350073914643295493433951928853735459200563766502880540575532123189009\
        126322819150914980836695624483100852221923973646324842863261145766932425371577377894414090\
        544573595351225626391080239236909732127905807617134603914574791879794124850218445145811341\
        888880413220955332184646709727491028565262707845453262227848800982385836300754950954764062\
        377083388357225436621567481327668384244972420874516161833205077991480184666814236693651902\
        845463857614827857037774388376297479982737705431583682410998683228503805526355369722293133\
        805264428410372312043967004307612454138311792278275363715598398376884537027842985707090511\
        223840536779013385414585316208073043138069739987436693166013817079272056041954882858063093\
        111636297047867814026963272962701226135985897754505289483113016684001532074851982402463337\
        555851713568019341228975980719568740250571502141783792543643030365928211250925880618903117\
        074543127903953553660682611001188965742048727593919976995538352115086696255596441370503829\
        244953590310636234530564717116216858725458687440029611757921723190554057198681727588419089\
        649657906696515601728351482903856551169807210795330916130843598524389465440682165500327537\
        996023866503798886481521186579995857186563775113315974753596043413776645119143460134292508\
        116324806409073773212629335747472967679341271602966512080989809057799660305102181625557978\
        907487076211076238026267854297015027109153504985351492390832484280828987595575678848892608\
        420885521269510357370208661259115541320440373560864338837123962064293902378666311632616788\
        841922798194995240394245784220443030420430420710969273392946085104969289739161855607837870\
        336428902342932718872968029721581659426129728366395905041130374745743509749058016326916537\
        5769098109748562537785034287994219223771859",
        "0x1.c5bf891b4ef6aa79c3b0520d5db9383fe3921546f63b252dca100bd3ea14746ed76ffd6f941f1dbacd8e7\
        6141e0c77476c521446ddc35851524490c34a42174dcbafdf109d1b3acfb553ade465bea6c5032aa8a111a626e\
        2c702f124d16fb3a90b8b1717835c4e1ec9c08f40a3db6d8123741ca788a38e7a619ffb5baebf7b166df28b5d2\
        ce89be636c040bb4096804fd2cd93812fbb09908b40ae5eede4891596c00af6afa05395faf6c86dc08b42b2eb0\
        dfb3a4876c1b058529354f2d429308d212dfcafadc0b046f3bc362555d119aaf416e879d013f2c8f4a2d55dab8\
        c4a398e161c9a5ba77ff284c3fdf892bf0f36805c5b0f813e47b36432ae9abfb8c2df6620a343b71239bd6f0ff\
        6e4525e14f2fd52776201d3e8ef4114462465d2552a8f30a26da0b28395985eaf57168b0fc66f7c6a5b8866f58\
        404028f9bf801f152865bb9b5adc90f6db648f9cfb609ca9193da29454c09450a4c8efbec6b5aeb4f075432e78\
        235da1447372389183627d449bafefcf9fd8a7267a98772baffa71b8d310c9d4f58bfd3d7299db12456357ddd5\
        2d6a9c56cf29d0303cbc0bf54a3ba39a725c78e8f19a0e59bfa4c7f915be3ff44c6c5d7b325585276abd43e221\
        ca9aafdd05c54a3fed2de03ba9b2606683f2870665e21a663c1387b4e02b4b90cc94033cc9775f2372e04174f5\
        19c42b9f65ec003149951eab1053ac0b7b77d251a898259637e3f17aea75bf15a08060348c31a61bd0b7e27e7c\
        68c614857ab238b36c1c64718de50ddc4293720630dbd5452b0d542e3f897e6bde585711e016e030e5dec6161b\
        a79dff01f061ca3b961fc02e77db5d73f2ef6d47c471d57f97a9a134cce64b48479d0628117f4350aff9d0d8ac\
        b474bdea6af441f7ade16dd2b1d90e128a91425fbef3609a23ee55c413660fee0224e4ff76fd23e43f11038c51\
        fef6ab1c0b9c2a50c5e15f92f5c998d5fac7d979367efd2277ccb66a735d3480c02f6dbdff7ac4e13dc41ec140\
        1b7424ad067f21c1f72464177ce2d902a6d60868b14aeed4385e55c94e0264549528cff2d94ecb0f9ea6febc17\
        8e7c440d9f1fa1e2c8bce21f2aa74067c08073707df38ff04fb731fe11753c63e5178fb6e0856db51bf8ae3f46\
        f3db7248a242f320803db8570f9ba75cea2e97d479ecfa6c84a3d9fe177eb2610ead682efed92bc297e843d696\
        f9797852fe0da2230c230757c48bf6172d2a8f6c94af315ffd09d9fc3fbe5f9146b2da2de051f6b0914fe89a2e\
        142e02ab85f76391911ff1eaaa0fe03cea3f7ccb93a1581dec1650dd6b1a1ff53eca03f427bf879f02ec73b2c0\
        cc2109c261083c4810c681d4fcdc970239432382a9cae18656ca343b831e1e720d4b4121472f039ac06c3dd3a6\
        ee46f70e2e929ca487dda488e4f4f80a00ad9bf6f7aaed026102f31a2d2f858c91ec18d5d8948940b306095746\
        8b4536d01bd00313ddf4685220b51c9b72856b906526488af9b2aaf0850915918a621fe23a63fb2594cf359db7\
        0377bb3a408a210dc230fa803135e46c1f2914f43ad8275f98dde221a0c93c23905fd6ae77eb55dc2c017d7e3b\
        84c564107577e6bd0f9817cc73db32a279d38fe586e954296ace2e7901a15b8974ec1b87c419d75684f61076cb\
        f653eae047e7f31c0c50e27f992c19f3059f632baa5aae158021bb870bbc9162f579d3ca45d2208e000d91a4fb\
        a0750031b19a4ab6e88733c2a86de645b7bf87676bcfa4347b1b40ad01adaba2967096f969e#10000",
        Greater,
    );

    let sqrt_pi_f32 = Float::sqrt_pi_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(sqrt_pi_f32.to_string(), "1.7724539");
    assert_eq!(to_hex_string(&sqrt_pi_f32), "0x1.c5bf8a#24");
    assert_eq!(sqrt_pi_f32, f32::SQRT_PI);

    let sqrt_pi_f64 = Float::sqrt_pi_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(sqrt_pi_f64.to_string(), "1.7724538509055161");
    assert_eq!(to_hex_string(&sqrt_pi_f64), "0x1.c5bf891b4ef6b#53");
    assert_eq!(sqrt_pi_f64, f64::SQRT_PI);
}

#[test]
#[should_panic]
fn sqrt_pi_prec_fail_1() {
    Float::sqrt_pi_prec(0);
}

fn test_sqrt_pi_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::sqrt_pi_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = sqrt_pi_prec_round_simple(prec, rm);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);
}

#[test]
pub fn test_sqrt_pi_prec_round() {
    test_sqrt_pi_prec_round_helper(1, Floor, "1.0", "0x1.0#1", Less);
    test_sqrt_pi_prec_round_helper(1, Ceiling, "2.0", "0x2.0#1", Greater);
    test_sqrt_pi_prec_round_helper(1, Down, "1.0", "0x1.0#1", Less);
    test_sqrt_pi_prec_round_helper(1, Up, "2.0", "0x2.0#1", Greater);
    test_sqrt_pi_prec_round_helper(1, Nearest, "2.0", "0x2.0#1", Greater);

    test_sqrt_pi_prec_round_helper(2, Floor, "1.5", "0x1.8#2", Less);
    test_sqrt_pi_prec_round_helper(2, Ceiling, "2.0", "0x2.0#2", Greater);
    test_sqrt_pi_prec_round_helper(2, Down, "1.5", "0x1.8#2", Less);
    test_sqrt_pi_prec_round_helper(2, Up, "2.0", "0x2.0#2", Greater);
    test_sqrt_pi_prec_round_helper(2, Nearest, "2.0", "0x2.0#2", Greater);

    test_sqrt_pi_prec_round_helper(3, Floor, "1.8", "0x1.c#3", Less);
    test_sqrt_pi_prec_round_helper(3, Ceiling, "2.0", "0x2.0#3", Greater);
    test_sqrt_pi_prec_round_helper(3, Down, "1.8", "0x1.c#3", Less);
    test_sqrt_pi_prec_round_helper(3, Up, "2.0", "0x2.0#3", Greater);
    test_sqrt_pi_prec_round_helper(3, Nearest, "1.8", "0x1.c#3", Less);

    test_sqrt_pi_prec_round_helper(4, Floor, "1.8", "0x1.c#4", Less);
    test_sqrt_pi_prec_round_helper(4, Ceiling, "1.9", "0x1.e#4", Greater);
    test_sqrt_pi_prec_round_helper(4, Down, "1.8", "0x1.c#4", Less);
    test_sqrt_pi_prec_round_helper(4, Up, "1.9", "0x1.e#4", Greater);
    test_sqrt_pi_prec_round_helper(4, Nearest, "1.8", "0x1.c#4", Less);

    test_sqrt_pi_prec_round_helper(5, Floor, "1.75", "0x1.c#5", Less);
    test_sqrt_pi_prec_round_helper(5, Ceiling, "1.81", "0x1.d#5", Greater);
    test_sqrt_pi_prec_round_helper(5, Down, "1.75", "0x1.c#5", Less);
    test_sqrt_pi_prec_round_helper(5, Up, "1.81", "0x1.d#5", Greater);
    test_sqrt_pi_prec_round_helper(5, Nearest, "1.75", "0x1.c#5", Less);

    test_sqrt_pi_prec_round_helper(6, Floor, "1.75", "0x1.c0#6", Less);
    test_sqrt_pi_prec_round_helper(6, Ceiling, "1.78", "0x1.c8#6", Greater);
    test_sqrt_pi_prec_round_helper(6, Down, "1.75", "0x1.c0#6", Less);
    test_sqrt_pi_prec_round_helper(6, Up, "1.78", "0x1.c8#6", Greater);
    test_sqrt_pi_prec_round_helper(6, Nearest, "1.78", "0x1.c8#6", Greater);

    test_sqrt_pi_prec_round_helper(7, Floor, "1.77", "0x1.c4#7", Less);
    test_sqrt_pi_prec_round_helper(7, Ceiling, "1.78", "0x1.c8#7", Greater);
    test_sqrt_pi_prec_round_helper(7, Down, "1.77", "0x1.c4#7", Less);
    test_sqrt_pi_prec_round_helper(7, Up, "1.78", "0x1.c8#7", Greater);
    test_sqrt_pi_prec_round_helper(7, Nearest, "1.77", "0x1.c4#7", Less);

    test_sqrt_pi_prec_round_helper(8, Floor, "1.766", "0x1.c4#8", Less);
    test_sqrt_pi_prec_round_helper(8, Ceiling, "1.773", "0x1.c6#8", Greater);
    test_sqrt_pi_prec_round_helper(8, Down, "1.766", "0x1.c4#8", Less);
    test_sqrt_pi_prec_round_helper(8, Up, "1.773", "0x1.c6#8", Greater);
    test_sqrt_pi_prec_round_helper(8, Nearest, "1.773", "0x1.c6#8", Greater);

    test_sqrt_pi_prec_round_helper(9, Floor, "1.77", "0x1.c5#9", Less);
    test_sqrt_pi_prec_round_helper(9, Ceiling, "1.773", "0x1.c6#9", Greater);
    test_sqrt_pi_prec_round_helper(9, Down, "1.77", "0x1.c5#9", Less);
    test_sqrt_pi_prec_round_helper(9, Up, "1.773", "0x1.c6#9", Greater);
    test_sqrt_pi_prec_round_helper(9, Nearest, "1.773", "0x1.c6#9", Greater);

    test_sqrt_pi_prec_round_helper(10, Floor, "1.771", "0x1.c58#10", Less);
    test_sqrt_pi_prec_round_helper(10, Ceiling, "1.773", "0x1.c60#10", Greater);
    test_sqrt_pi_prec_round_helper(10, Down, "1.771", "0x1.c58#10", Less);
    test_sqrt_pi_prec_round_helper(10, Up, "1.773", "0x1.c60#10", Greater);
    test_sqrt_pi_prec_round_helper(10, Nearest, "1.771", "0x1.c58#10", Less);

    test_sqrt_pi_prec_round_helper(
        100,
        Floor,
        "1.77245385090551602729816748334",
        "0x1.c5bf891b4ef6aa79c3b0520d4#100",
        Less,
    );
    test_sqrt_pi_prec_round_helper(
        100,
        Ceiling,
        "1.772453850905516027298167483341",
        "0x1.c5bf891b4ef6aa79c3b0520d6#100",
        Greater,
    );
    test_sqrt_pi_prec_round_helper(
        100,
        Down,
        "1.77245385090551602729816748334",
        "0x1.c5bf891b4ef6aa79c3b0520d4#100",
        Less,
    );
    test_sqrt_pi_prec_round_helper(
        100,
        Up,
        "1.772453850905516027298167483341",
        "0x1.c5bf891b4ef6aa79c3b0520d6#100",
        Greater,
    );
    test_sqrt_pi_prec_round_helper(
        100,
        Nearest,
        "1.772453850905516027298167483341",
        "0x1.c5bf891b4ef6aa79c3b0520d6#100",
        Greater,
    );
}

#[test]
#[should_panic]
fn sqrt_pi_prec_round_fail_1() {
    Float::sqrt_pi_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn sqrt_pi_prec_round_fail_2() {
    Float::sqrt_pi_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn sqrt_pi_prec_round_fail_3() {
    Float::sqrt_pi_prec_round(1000, Exact);
}

#[test]
fn sqrt_pi_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (sqrt_pi, o) = Float::sqrt_pi_prec(prec);
        assert!(sqrt_pi.is_valid());
        assert_eq!(sqrt_pi.get_prec(), Some(prec));
        assert_eq!(sqrt_pi.get_exponent(), Some(if prec <= 2 { 2 } else { 1 }));
        assert_ne!(o, Equal);
        if o == Less {
            let (sqrt_pi_alt, o_alt) = Float::sqrt_pi_prec_round(prec, Ceiling);
            let mut next_upper = sqrt_pi.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(sqrt_pi_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !sqrt_pi.is_power_of_2() {
            let (sqrt_pi_alt, o_alt) = Float::sqrt_pi_prec_round(prec, Floor);
            let mut next_lower = sqrt_pi.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(sqrt_pi_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
        let (sqrt_pi_alt, o_alt) = Float::sqrt_pi_prec_round(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&sqrt_pi_alt),
            ComparableFloatRef(&sqrt_pi)
        );
        assert_eq!(o_alt, o);

        let (sqrt_pi_alt, o_alt) = sqrt_pi_prec_round_simple(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&sqrt_pi_alt),
            ComparableFloatRef(&sqrt_pi)
        );
        assert_eq!(o_alt, o);
    });
}

#[test]
fn sqrt_pi_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (sqrt_pi, o) = Float::sqrt_pi_prec_round(prec, rm);
        assert!(sqrt_pi.is_valid());
        assert_eq!(sqrt_pi.get_prec(), Some(prec));
        let expected_exponent = match (prec, rm) {
            (1 | 2, Ceiling | Up | Nearest) | (3, Ceiling | Up) => 2,
            _ => 1,
        };
        assert_eq!(sqrt_pi.get_exponent(), Some(expected_exponent));
        assert_ne!(o, Equal);
        if o == Less {
            let (sqrt_pi_alt, o_alt) = Float::sqrt_pi_prec_round(prec, Ceiling);
            let mut next_upper = sqrt_pi.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(sqrt_pi_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !sqrt_pi.is_power_of_2() {
            let (sqrt_pi_alt, o_alt) = Float::sqrt_pi_prec_round(prec, Floor);
            let mut next_lower = sqrt_pi.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(sqrt_pi_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }

        let (sqrt_pi_alt, o_alt) = sqrt_pi_prec_round_simple(prec, rm);
        assert_eq!(
            ComparableFloatRef(&sqrt_pi_alt),
            ComparableFloatRef(&sqrt_pi)
        );
        assert_eq!(o_alt, o);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::sqrt_pi_prec_round(prec, Exact));
    });

    test_constant(Float::sqrt_pi_prec_round, 10000);
}
