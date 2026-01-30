// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::PiOver8;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{test_constant, to_hex_string};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_pi_over_8_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::pi_over_8_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);
}

#[test]
pub fn test_pi_over_8_prec() {
    test_pi_over_8_prec_helper(1, "0.5", "0x0.8#1", Greater);
    test_pi_over_8_prec_helper(2, "0.4", "0x0.6#2", Less);
    test_pi_over_8_prec_helper(3, "0.38", "0x0.6#3", Less);
    test_pi_over_8_prec_helper(4, "0.41", "0x0.68#4", Greater);
    test_pi_over_8_prec_helper(5, "0.39", "0x0.64#5", Less);
    test_pi_over_8_prec_helper(6, "0.39", "0x0.64#6", Less);
    test_pi_over_8_prec_helper(7, "0.395", "0x0.65#7", Greater);
    test_pi_over_8_prec_helper(8, "0.393", "0x0.648#8", Less);
    test_pi_over_8_prec_helper(9, "0.393", "0x0.648#9", Less);
    test_pi_over_8_prec_helper(10, "0.3926", "0x0.648#10", Less);
    test_pi_over_8_prec_helper(
        100,
        "0.3926990816987241548078304229099",
        "0x0.6487ed5110b4611a62633145c0#100",
        Less,
    );
    test_pi_over_8_prec_helper(
        1000,
        "0.392699081698724154807830422909937860524646174921888227621868074038477050785776124828504\
        353167764633497768510814160288330886730576193822778965669926016060139681285512837742315138\
        194955577868619366297745553601371958241680766059456029223347895658900238643206070865432543\
        5763067908310266742009075311426592",
        "0x0.6487ed5110b4611a62633145c06e0e68948127044533e63a0105df531d89cd9128a5043cc71a026ef7ca8\
        cd9e69d218d98158536f92f8a1ba7f09ab6b6a8e122f242dabb312f3f637a262174d31bf6b585ffae5b7a035bf\
        6f71c35fdad44cfd2d74f9208be258ff324943328f6722d9ee1003e5c50b1df82cc6d241b0e0#1000",
        Less,
    );
    test_pi_over_8_prec_helper(
        10000,
        "0.392699081698724154807830422909937860524646174921888227621868074038477050785776124828504\
        353167764633497768510814160288330886730576193822778965669926016060139681285512837742315138\
        194955577868619366297745553601371958241680766059456029223347895658900238643206070865432543\
        576306790831026674200907531142659215573375825789448521860190115120353656761464420545986573\
        795001416316318602558315173018368992688951179132158795719948994136523264672741576474138139\
        818509307797495343695918985719090611403492272876493641229209170305082080385752674368299403\
        092148837772482617962846317402147036647094048084352308458675641500071015893157945103473221\
        417844723701146704647340183555112653119178768318698171381349034961157365442752494514016127\
        745108004302269976703721846413745064838401418749999796622256243882466466602012039824378057\
        431918363537830315288531668085629407741398521376250392229844110823441651047677577147208643\
        412949781686303594433591394945357985294223449218994722732222566521403350826625240984576399\
        488652052524867261907150133185732909858242019172728496028787744004412731621124471702824926\
        736406215221910434891439446857155306768836993853691639607715981986134387297719329683117414\
        906882550115962708892376231103001607295200445463457513088772742869449524868345979681181031\
        922471840855888005941830776005835532383686411664209628623644013094020257120753007254768774\
        188906672803754448455030937059157989274909075533740349597794347704501167715205152490573289\
        378785772871819463343729813186823573365874461365901349688662869415145668123400344495029560\
        083187389985229349719195796225928317815982818977302196834111372221599225010205882502018156\
        149021652151846543767680246696068520170144669069016684467731183554815415488424267916818470\
        302107814872946185695262402402773034068781782110958973811825206683350623578404098973260723\
        047978495997085181762619235473295118835008028140650639674123112010516061078368200530245660\
        627776332648288343034827754899368130890464223370119545546489660933470582196745301736358229\
        080749476673809753448762433220509868908683549794074463728227827565311175965839934783560325\
        184623862830017049304681913133525437031556468674956428928726148832406367152712057689463732\
        298426324735744971621937366270219241058517283585483617846769498981990655744244928881246565\
        585105748409205586981081729592027828262390576006405485548806405517068720347599714461429499\
        712516202011180211858569481050794177759028228561081019807003563252105342431533433459861906\
        517315318744333340977998307074514544360778822182062254449204321021790514064384508684931387\
        074511753153609963861643208642108403593617570012687913577241085115109345114728117323626214\
        363709498157670693722736641223102710374868590332350607195501783809694391547455181440468279\
        295567857305599408233477631392669341967440389178395762766996192028930369060617148388768220\
        675448784991800467750913822317382774798430976059811210415180714233593992938303777306648881\
        060125671326835084364909773899742494007677457928594305080468140464774022499798876989945226\
        834392836404968617613398311778995195993151",
        "0x0.6487ed5110b4611a62633145c06e0e68948127044533e63a0105df531d89cd9128a5043cc71a026ef7ca8\
        cd9e69d218d98158536f92f8a1ba7f09ab6b6a8e122f242dabb312f3f637a262174d31bf6b585ffae5b7a035bf\
        6f71c35fdad44cfd2d74f9208be258ff324943328f6722d9ee1003e5c50b1df82cc6d241b0e2ae9cd348b1fd47\
        e9267afc1b2ae91ee51d6cb0e3179ab1042a95dcf6a9483b84b4b36b3861aa7255e4c0278ba3604650c10be194\
        82f23171b671df1cf3b960c074301cd93c1d17603d147dae2aef837a62964ef15e5fb4aac0b8c1ccaa4be754ab\
        5728ae9130c4c7d02880ab9472d45556216d6998b8682283d19d42a90d5ef8e5d32767dc2822c6df785457538a\
        bae83063ed9cb87c2d370f263d5fad7466d8499eb8f464a702512b0cee771e9130d697735f897fd036cc504326\
        c3b01399f643532290f958c0bbd90065df08babbd30aeb63b84c4605d6ca371047127d03a72d598a1edadfe707\
        e884725c16890549084008d391e0953c3f36bc438cd085edd2d934ce1938c357a711e0d4a341a5b0a85ed12c1f\
        4e5156a26746ddde16d826f477c97477e0a0fdf6553143e2ca3a735e02eccd94b27d04861d1119dd0c328adf3f\
        68fb094b867716bd7dc0deebb10b8240e68034893ead82d54c9da754c46c7eee0c37fdbee48536047a6fa1ae49\
        a0142491b61fd5a693e381360ea6e593013236f64ba8f3b1edd1bdefc7fca0356cf298772ed9c17a09800d7583\
        529f6c813ec188bcb93d8432d448c6d1f6df5e7cd8a76a267365d676a5d8dedbf8a23f36612a5999028a895ebd\
        7a137dc7a009bc6695facc1e500e325c9767819750ae8b90e81fa416be7373a7f7b6aaf3817a34c06415ad4201\
        8c8058e4f2cf3e4bfdf63f47991d4bd3f1b66445f078ea2dbffac2d62a5ea03d915a0aa556647b6bf5fa470ec0\
        a662f6907c01bf053cb8af7794df1940350eac5dbe2ed3b7aa8551ec50fdff8758ce658d189eaae6d2b64f6177\
        94b191c3ff46bb71e0234021f47b31fa43077095f96ad85ba3a6b734a7c8f36df08acba51c937897f72f21c3bb\
        e5b54996fc66c5f626839dc98dd1de4195b46cee9803a0fd3dfc57e23f692bb7b49b5d212331d55b1ce2d727ab\
        41a11da3a15f8e4bc11c78b65f1ceb296f1fedc5f7e42456c911117025201be0389f5abd40d11f8639a39fe323\
        6751835a5e5e44317c1c2eefd4ea5bfd16043f43cb41981f6adee9d03159e7ad9d13c53369509fc1fa27c16ef9\
        887703a55b51b22cbf44cd012aee0b2798e628423428efcd5a40caef6bf50d8ea885ebf73a6b9fd79b5e18f67d\
        1341ac8237a75c3cfc92004a1c5a40e366bc44d00176af71c15e48c86d37e013723caac7223ab3bf4d54f18287\
        13b2b4a6fe40fab74405cb738b064c06ecc52b9f52239032d09ce69483668e5b94f629529436a200a534a6522b\
        ac1eae7f79378dc856c148ed03cce8001174df6aae37d23fe52dd8d6541bb22b6cc6ca43cf73f36dfe680a5d8b\
        0aacc8a7605abb533f1f4211748f32dd0a0ed496f4e1d36b665289b6ea125d88324c475add4d60934fbefb39dc\
        c1711fdb64cdd95518e2d3342ffeacc8a4d985633dc2326c054aea12985340b22681c830747c7ea93134b6853a\
        cad71fc9ad33ee7fad43a5380fdfd061ea9a5a71cde04abb8299ba410d08e1d64cc705d38c043d98bc12d0d67e\
        7d75df92792e3028d6d4e142d0fe6b08a541c50d6f38a60b54a00e6e7c0f0838ffbd5cb91cf8#10000",
        Less,
    );

    let pi_over_8_f32 = Float::pi_over_8_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(pi_over_8_f32.to_string(), "0.39269909");
    assert_eq!(to_hex_string(&pi_over_8_f32), "0x0.6487ed8#24");
    assert_eq!(pi_over_8_f32, f32::PI_OVER_8);

    let pi_over_8_f64 = Float::pi_over_8_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(pi_over_8_f64.to_string(), "0.39269908169872414");
    assert_eq!(to_hex_string(&pi_over_8_f64), "0x0.6487ed5110b460#53");
    assert_eq!(pi_over_8_f64, f64::PI_OVER_8);
}

#[test]
#[should_panic]
fn pi_over_8_prec_fail_1() {
    Float::pi_over_8_prec(0);
}

fn test_pi_over_8_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::pi_over_8_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);
}

#[test]
pub fn test_pi_over_8_prec_round() {
    test_pi_over_8_prec_round_helper(1, Floor, "0.2", "0x0.4#1", Less);
    test_pi_over_8_prec_round_helper(1, Ceiling, "0.5", "0x0.8#1", Greater);
    test_pi_over_8_prec_round_helper(1, Down, "0.2", "0x0.4#1", Less);
    test_pi_over_8_prec_round_helper(1, Up, "0.5", "0x0.8#1", Greater);
    test_pi_over_8_prec_round_helper(1, Nearest, "0.5", "0x0.8#1", Greater);

    test_pi_over_8_prec_round_helper(2, Floor, "0.4", "0x0.6#2", Less);
    test_pi_over_8_prec_round_helper(2, Ceiling, "0.5", "0x0.8#2", Greater);
    test_pi_over_8_prec_round_helper(2, Down, "0.4", "0x0.6#2", Less);
    test_pi_over_8_prec_round_helper(2, Up, "0.5", "0x0.8#2", Greater);
    test_pi_over_8_prec_round_helper(2, Nearest, "0.4", "0x0.6#2", Less);

    test_pi_over_8_prec_round_helper(3, Floor, "0.38", "0x0.6#3", Less);
    test_pi_over_8_prec_round_helper(3, Ceiling, "0.44", "0x0.7#3", Greater);
    test_pi_over_8_prec_round_helper(3, Down, "0.38", "0x0.6#3", Less);
    test_pi_over_8_prec_round_helper(3, Up, "0.44", "0x0.7#3", Greater);
    test_pi_over_8_prec_round_helper(3, Nearest, "0.38", "0x0.6#3", Less);

    test_pi_over_8_prec_round_helper(4, Floor, "0.38", "0x0.60#4", Less);
    test_pi_over_8_prec_round_helper(4, Ceiling, "0.41", "0x0.68#4", Greater);
    test_pi_over_8_prec_round_helper(4, Down, "0.38", "0x0.60#4", Less);
    test_pi_over_8_prec_round_helper(4, Up, "0.41", "0x0.68#4", Greater);
    test_pi_over_8_prec_round_helper(4, Nearest, "0.41", "0x0.68#4", Greater);

    test_pi_over_8_prec_round_helper(5, Floor, "0.39", "0x0.64#5", Less);
    test_pi_over_8_prec_round_helper(5, Ceiling, "0.41", "0x0.68#5", Greater);
    test_pi_over_8_prec_round_helper(5, Down, "0.39", "0x0.64#5", Less);
    test_pi_over_8_prec_round_helper(5, Up, "0.41", "0x0.68#5", Greater);
    test_pi_over_8_prec_round_helper(5, Nearest, "0.39", "0x0.64#5", Less);

    test_pi_over_8_prec_round_helper(6, Floor, "0.39", "0x0.64#6", Less);
    test_pi_over_8_prec_round_helper(6, Ceiling, "0.4", "0x0.66#6", Greater);
    test_pi_over_8_prec_round_helper(6, Down, "0.39", "0x0.64#6", Less);
    test_pi_over_8_prec_round_helper(6, Up, "0.4", "0x0.66#6", Greater);
    test_pi_over_8_prec_round_helper(6, Nearest, "0.39", "0x0.64#6", Less);

    test_pi_over_8_prec_round_helper(7, Floor, "0.391", "0x0.64#7", Less);
    test_pi_over_8_prec_round_helper(7, Ceiling, "0.395", "0x0.65#7", Greater);
    test_pi_over_8_prec_round_helper(7, Down, "0.391", "0x0.64#7", Less);
    test_pi_over_8_prec_round_helper(7, Up, "0.395", "0x0.65#7", Greater);
    test_pi_over_8_prec_round_helper(7, Nearest, "0.395", "0x0.65#7", Greater);

    test_pi_over_8_prec_round_helper(8, Floor, "0.393", "0x0.648#8", Less);
    test_pi_over_8_prec_round_helper(8, Ceiling, "0.395", "0x0.650#8", Greater);
    test_pi_over_8_prec_round_helper(8, Down, "0.393", "0x0.648#8", Less);
    test_pi_over_8_prec_round_helper(8, Up, "0.395", "0x0.650#8", Greater);
    test_pi_over_8_prec_round_helper(8, Nearest, "0.393", "0x0.648#8", Less);

    test_pi_over_8_prec_round_helper(9, Floor, "0.393", "0x0.648#9", Less);
    test_pi_over_8_prec_round_helper(9, Ceiling, "0.394", "0x0.64c#9", Greater);
    test_pi_over_8_prec_round_helper(9, Down, "0.393", "0x0.648#9", Less);
    test_pi_over_8_prec_round_helper(9, Up, "0.394", "0x0.64c#9", Greater);
    test_pi_over_8_prec_round_helper(9, Nearest, "0.393", "0x0.648#9", Less);

    test_pi_over_8_prec_round_helper(10, Floor, "0.3926", "0x0.648#10", Less);
    test_pi_over_8_prec_round_helper(10, Ceiling, "0.3931", "0x0.64a#10", Greater);
    test_pi_over_8_prec_round_helper(10, Down, "0.3926", "0x0.648#10", Less);
    test_pi_over_8_prec_round_helper(10, Up, "0.3931", "0x0.64a#10", Greater);
    test_pi_over_8_prec_round_helper(10, Nearest, "0.3926", "0x0.648#10", Less);

    test_pi_over_8_prec_round_helper(
        100,
        Floor,
        "0.3926990816987241548078304229099",
        "0x0.6487ed5110b4611a62633145c0#100",
        Less,
    );
    test_pi_over_8_prec_round_helper(
        100,
        Ceiling,
        "0.3926990816987241548078304229103",
        "0x0.6487ed5110b4611a62633145c8#100",
        Greater,
    );
    test_pi_over_8_prec_round_helper(
        100,
        Down,
        "0.3926990816987241548078304229099",
        "0x0.6487ed5110b4611a62633145c0#100",
        Less,
    );
    test_pi_over_8_prec_round_helper(
        100,
        Up,
        "0.3926990816987241548078304229103",
        "0x0.6487ed5110b4611a62633145c8#100",
        Greater,
    );
    test_pi_over_8_prec_round_helper(
        100,
        Nearest,
        "0.3926990816987241548078304229099",
        "0x0.6487ed5110b4611a62633145c0#100",
        Less,
    );
}

#[test]
#[should_panic]
fn pi_over_8_prec_round_fail_1() {
    Float::pi_over_8_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn pi_over_8_prec_round_fail_2() {
    Float::pi_over_8_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn pi_over_8_prec_round_fail_3() {
    Float::pi_over_8_prec_round(1000, Exact);
}

#[test]
fn pi_over_8_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (pi_over_8, o) = Float::pi_over_8_prec(prec);
        assert!(pi_over_8.is_valid());
        assert_eq!(pi_over_8.get_prec(), Some(prec));
        assert_eq!(
            pi_over_8.get_exponent(),
            Some(if prec == 1 { 0 } else { -1 })
        );
        assert_ne!(o, Equal);
        if o == Less {
            let (pi_over_8_alt, o_alt) = Float::pi_over_8_prec_round(prec, Ceiling);
            let mut next_upper = pi_over_8.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(pi_over_8_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !pi_over_8.is_power_of_2() {
            let (pi_over_8_alt, o_alt) = Float::pi_over_8_prec_round(prec, Floor);
            let mut next_lower = pi_over_8.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(pi_over_8_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
        let (pi_over_8_alt, o_alt) = Float::pi_over_8_prec_round(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&pi_over_8_alt),
            ComparableFloatRef(&pi_over_8)
        );
        assert_eq!(o_alt, o);
    });
}

#[test]
fn pi_over_8_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (pi_over_8, o) = Float::pi_over_8_prec_round(prec, rm);
        assert!(pi_over_8.is_valid());
        assert_eq!(pi_over_8.get_prec(), Some(prec));
        let expected_exponent = match (prec, rm) {
            (1, Ceiling | Up | Nearest) | (2, Ceiling | Up) => 0,
            _ => -1,
        };
        assert_eq!(pi_over_8.get_exponent(), Some(expected_exponent));
        assert_ne!(o, Equal);
        if o == Less {
            let (pi_over_8_alt, o_alt) = Float::pi_over_8_prec_round(prec, Ceiling);
            let mut next_upper = pi_over_8.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(pi_over_8_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !pi_over_8.is_power_of_2() {
            let (pi_over_8_alt, o_alt) = Float::pi_over_8_prec_round(prec, Floor);
            let mut next_lower = pi_over_8.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(pi_over_8_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::pi_over_8_prec_round(prec, Exact));
    });

    test_constant(Float::pi_over_8_prec_round, 10000);
}
