// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::Tau;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{test_constant, to_hex_string};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_tau_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::tau_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);
}

#[test]
pub fn test_tau_prec() {
    test_tau_prec_helper(1, "8.0", "0x8.0#1", Greater);
    test_tau_prec_helper(2, "6.0", "0x6.0#2", Less);
    test_tau_prec_helper(3, "6.0", "0x6.0#3", Less);
    test_tau_prec_helper(4, "6.5", "0x6.8#4", Greater);
    test_tau_prec_helper(5, "6.2", "0x6.4#5", Less);
    test_tau_prec_helper(6, "6.2", "0x6.4#6", Less);
    test_tau_prec_helper(7, "6.3", "0x6.5#7", Greater);
    test_tau_prec_helper(8, "6.28", "0x6.48#8", Less);
    test_tau_prec_helper(9, "6.28", "0x6.48#9", Less);
    test_tau_prec_helper(10, "6.28", "0x6.48#10", Less);
    test_tau_prec_helper(
        100,
        "6.283185307179586476925286766559",
        "0x6.487ed5110b4611a62633145c0#100",
        Less,
    );
    test_tau_prec_helper(
        1000,
        "6.283185307179586476925286766559005768394338798750211641949889184615632812572417997256069\
        650684234135964296173026564613294187689219101164463450718816256962234900568205403877042211\
        119289245897909860763928857621951331866892256951296467573566330542403818291297133846920697\
        220908653296426787214520498282547",
        "0x6.487ed5110b4611a62633145c06e0e68948127044533e63a0105df531d89cd9128a5043cc71a026ef7ca8c\
        d9e69d218d98158536f92f8a1ba7f09ab6b6a8e122f242dabb312f3f637a262174d31bf6b585ffae5b7a035bf6\
        f71c35fdad44cfd2d74f9208be258ff324943328f6722d9ee1003e5c50b1df82cc6d241b0e0#1000",
        Less,
    );
    test_tau_prec_helper(
        10000,
        "6.283185307179586476925286766559005768394338798750211641949889184615632812572417997256069\
        650684234135964296173026564613294187689219101164463450718816256962234900568205403877042211\
        119289245897909860763928857621951331866892256951296467573566330542403818291297133846920697\
        220908653296426787214520498282547449174013212631176349763041841925658508183430728735785180\
        720022661061097640933042768293903883023218866114540731519183906184372234763865223586210237\
        096148924759925499134703771505449782455876366023898259667346724881313286172042789892790449\
        474381404359721887405541078434352586353504769349636935338810264001136254290527121655571542\
        685515579218347274357442936881802449906860293099170742101584559378517847084039912224258043\
        921728068836319627259549542619921037414422699999996745956099902119463465632192637190048918\
        910693816605285044616506689370070523862376342020006275677505773175066416762841234355338294\
        607196506980857510937462319125727764707575187503915563715561064342453613226003855753222391\
        818432840397876190514402130971726557731872306763655936460603904070603705937991547245198827\
        782499443550566958263031149714484908301391901659066233723455711778150196763509274929878638\
        510120801855403342278019697648025716723207127415320209420363885911192397893535674898896510\
        759549453694208095069292416093368518138982586627354057978304209504324113932048116076300387\
        022506764860071175280494992946527828398545208539845593564709563272018683443282439849172630\
        060572365949111413499677010989177173853991381854421595018605910642330689974405511920472961\
        330998239763669595507132739614853085055725103636835149345781955545587600163294120032290498\
        384346434429544700282883947137096322722314705104266951483698936877046647814788286669095524\
        833725037967138971124198438444368545100508513775343580989203306933609977254465583572171568\
        767655935953362908201907767572721901360128450250410234785969792168256977253891208483930570\
        044421322372613488557244078389890094247427573921912728743834574935529315147924827781731665\
        291991626780956055180198931528157902538936796705191419651645241044978815453438956536965202\
        953981805280272788874910610136406992504903498799302862859618381318501874443392923031419716\
        774821195771919545950997860323507856936276537367737885548311983711850491907918862099945049\
        361691974547289391697307673472445252198249216102487768780902488273099525561595431382871995\
        400259232178883389737111696812706844144451656977296316912057012033685478904534935357790504\
        277045099909333455647972913192232709772461154912996071187269136348648225030152138958902193\
        192188050457759421786291338273734457497881120203006617235857361841749521835649877178019429\
        819351970522731099563786259569643365997897445317609715128028540955110264759282903047492468\
        729085716889590531735642102282709471479046226854332204271939072462885904969874374220291530\
        807180559868807484014621157078124396774895616956979366642891427737503887012860436906382096\
        962010741229361349838556382395879904122839326857508881287490247436384359996782031839123629\
        3502853824794978818143729884639231358904161",
        "0x6.487ed5110b4611a62633145c06e0e68948127044533e63a0105df531d89cd9128a5043cc71a026ef7ca8c\
        d9e69d218d98158536f92f8a1ba7f09ab6b6a8e122f242dabb312f3f637a262174d31bf6b585ffae5b7a035bf6\
        f71c35fdad44cfd2d74f9208be258ff324943328f6722d9ee1003e5c50b1df82cc6d241b0e2ae9cd348b1fd47e\
        9267afc1b2ae91ee51d6cb0e3179ab1042a95dcf6a9483b84b4b36b3861aa7255e4c0278ba3604650c10be1948\
        2f23171b671df1cf3b960c074301cd93c1d17603d147dae2aef837a62964ef15e5fb4aac0b8c1ccaa4be754ab5\
        728ae9130c4c7d02880ab9472d45556216d6998b8682283d19d42a90d5ef8e5d32767dc2822c6df785457538ab\
        ae83063ed9cb87c2d370f263d5fad7466d8499eb8f464a702512b0cee771e9130d697735f897fd036cc504326c\
        3b01399f643532290f958c0bbd90065df08babbd30aeb63b84c4605d6ca371047127d03a72d598a1edadfe707e\
        884725c16890549084008d391e0953c3f36bc438cd085edd2d934ce1938c357a711e0d4a341a5b0a85ed12c1f4\
        e5156a26746ddde16d826f477c97477e0a0fdf6553143e2ca3a735e02eccd94b27d04861d1119dd0c328adf3f6\
        8fb094b867716bd7dc0deebb10b8240e68034893ead82d54c9da754c46c7eee0c37fdbee48536047a6fa1ae49a\
        0142491b61fd5a693e381360ea6e593013236f64ba8f3b1edd1bdefc7fca0356cf298772ed9c17a09800d75835\
        29f6c813ec188bcb93d8432d448c6d1f6df5e7cd8a76a267365d676a5d8dedbf8a23f36612a5999028a895ebd7\
        a137dc7a009bc6695facc1e500e325c9767819750ae8b90e81fa416be7373a7f7b6aaf3817a34c06415ad42018\
        c8058e4f2cf3e4bfdf63f47991d4bd3f1b66445f078ea2dbffac2d62a5ea03d915a0aa556647b6bf5fa470ec0a\
        662f6907c01bf053cb8af7794df1940350eac5dbe2ed3b7aa8551ec50fdff8758ce658d189eaae6d2b64f61779\
        4b191c3ff46bb71e0234021f47b31fa43077095f96ad85ba3a6b734a7c8f36df08acba51c937897f72f21c3bbe\
        5b54996fc66c5f626839dc98dd1de4195b46cee9803a0fd3dfc57e23f692bb7b49b5d212331d55b1ce2d727ab4\
        1a11da3a15f8e4bc11c78b65f1ceb296f1fedc5f7e42456c911117025201be0389f5abd40d11f8639a39fe3236\
        751835a5e5e44317c1c2eefd4ea5bfd16043f43cb41981f6adee9d03159e7ad9d13c53369509fc1fa27c16ef98\
        87703a55b51b22cbf44cd012aee0b2798e628423428efcd5a40caef6bf50d8ea885ebf73a6b9fd79b5e18f67d1\
        341ac8237a75c3cfc92004a1c5a40e366bc44d00176af71c15e48c86d37e013723caac7223ab3bf4d54f182871\
        3b2b4a6fe40fab74405cb738b064c06ecc52b9f52239032d09ce69483668e5b94f629529436a200a534a6522ba\
        c1eae7f79378dc856c148ed03cce8001174df6aae37d23fe52dd8d6541bb22b6cc6ca43cf73f36dfe680a5d8b0\
        aacc8a7605abb533f1f4211748f32dd0a0ed496f4e1d36b665289b6ea125d88324c475add4d60934fbefb39dcc\
        1711fdb64cdd95518e2d3342ffeacc8a4d985633dc2326c054aea12985340b22681c830747c7ea93134b6853ac\
        ad71fc9ad33ee7fad43a5380fdfd061ea9a5a71cde04abb8299ba410d08e1d64cc705d38c043d98bc12d0d67e7\
        d75df92792e3028d6d4e142d0fe6b08a541c50d6f38a60b54a00e6e7c0f0838ffbd5cb91cf8#10000",
        Less,
    );

    let tau_f32 = Float::tau_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(tau_f32.to_string(), "6.2831855");
    assert_eq!(to_hex_string(&tau_f32), "0x6.487ed8#24");
    assert_eq!(tau_f32, f32::TAU);

    let tau_f64 = Float::tau_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(tau_f64.to_string(), "6.283185307179586");
    assert_eq!(to_hex_string(&tau_f64), "0x6.487ed5110b460#53");
    assert_eq!(tau_f64, f64::TAU);
}

#[test]
#[should_panic]
fn tau_prec_fail_1() {
    Float::tau_prec(0);
}

fn test_tau_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::tau_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);
}

#[test]
pub fn test_tau_prec_round() {
    test_tau_prec_round_helper(1, Floor, "4.0", "0x4.0#1", Less);
    test_tau_prec_round_helper(1, Ceiling, "8.0", "0x8.0#1", Greater);
    test_tau_prec_round_helper(1, Down, "4.0", "0x4.0#1", Less);
    test_tau_prec_round_helper(1, Up, "8.0", "0x8.0#1", Greater);
    test_tau_prec_round_helper(1, Nearest, "8.0", "0x8.0#1", Greater);

    test_tau_prec_round_helper(2, Floor, "6.0", "0x6.0#2", Less);
    test_tau_prec_round_helper(2, Ceiling, "8.0", "0x8.0#2", Greater);
    test_tau_prec_round_helper(2, Down, "6.0", "0x6.0#2", Less);
    test_tau_prec_round_helper(2, Up, "8.0", "0x8.0#2", Greater);
    test_tau_prec_round_helper(2, Nearest, "6.0", "0x6.0#2", Less);

    test_tau_prec_round_helper(3, Floor, "6.0", "0x6.0#3", Less);
    test_tau_prec_round_helper(3, Ceiling, "7.0", "0x7.0#3", Greater);
    test_tau_prec_round_helper(3, Down, "6.0", "0x6.0#3", Less);
    test_tau_prec_round_helper(3, Up, "7.0", "0x7.0#3", Greater);
    test_tau_prec_round_helper(3, Nearest, "6.0", "0x6.0#3", Less);

    test_tau_prec_round_helper(4, Floor, "6.0", "0x6.0#4", Less);
    test_tau_prec_round_helper(4, Ceiling, "6.5", "0x6.8#4", Greater);
    test_tau_prec_round_helper(4, Down, "6.0", "0x6.0#4", Less);
    test_tau_prec_round_helper(4, Up, "6.5", "0x6.8#4", Greater);
    test_tau_prec_round_helper(4, Nearest, "6.5", "0x6.8#4", Greater);

    test_tau_prec_round_helper(5, Floor, "6.2", "0x6.4#5", Less);
    test_tau_prec_round_helper(5, Ceiling, "6.5", "0x6.8#5", Greater);
    test_tau_prec_round_helper(5, Down, "6.2", "0x6.4#5", Less);
    test_tau_prec_round_helper(5, Up, "6.5", "0x6.8#5", Greater);
    test_tau_prec_round_helper(5, Nearest, "6.2", "0x6.4#5", Less);

    test_tau_prec_round_helper(6, Floor, "6.2", "0x6.4#6", Less);
    test_tau_prec_round_helper(6, Ceiling, "6.4", "0x6.6#6", Greater);
    test_tau_prec_round_helper(6, Down, "6.2", "0x6.4#6", Less);
    test_tau_prec_round_helper(6, Up, "6.4", "0x6.6#6", Greater);
    test_tau_prec_round_helper(6, Nearest, "6.2", "0x6.4#6", Less);

    test_tau_prec_round_helper(7, Floor, "6.25", "0x6.4#7", Less);
    test_tau_prec_round_helper(7, Ceiling, "6.3", "0x6.5#7", Greater);
    test_tau_prec_round_helper(7, Down, "6.25", "0x6.4#7", Less);
    test_tau_prec_round_helper(7, Up, "6.3", "0x6.5#7", Greater);
    test_tau_prec_round_helper(7, Nearest, "6.3", "0x6.5#7", Greater);

    test_tau_prec_round_helper(8, Floor, "6.28", "0x6.48#8", Less);
    test_tau_prec_round_helper(8, Ceiling, "6.31", "0x6.50#8", Greater);
    test_tau_prec_round_helper(8, Down, "6.28", "0x6.48#8", Less);
    test_tau_prec_round_helper(8, Up, "6.31", "0x6.50#8", Greater);
    test_tau_prec_round_helper(8, Nearest, "6.28", "0x6.48#8", Less);

    test_tau_prec_round_helper(9, Floor, "6.28", "0x6.48#9", Less);
    test_tau_prec_round_helper(9, Ceiling, "6.3", "0x6.4c#9", Greater);
    test_tau_prec_round_helper(9, Down, "6.28", "0x6.48#9", Less);
    test_tau_prec_round_helper(9, Up, "6.3", "0x6.4c#9", Greater);
    test_tau_prec_round_helper(9, Nearest, "6.28", "0x6.48#9", Less);

    test_tau_prec_round_helper(10, Floor, "6.28", "0x6.48#10", Less);
    test_tau_prec_round_helper(10, Ceiling, "6.29", "0x6.4a#10", Greater);
    test_tau_prec_round_helper(10, Down, "6.28", "0x6.48#10", Less);
    test_tau_prec_round_helper(10, Up, "6.29", "0x6.4a#10", Greater);
    test_tau_prec_round_helper(10, Nearest, "6.28", "0x6.48#10", Less);

    test_tau_prec_round_helper(
        100,
        Floor,
        "6.283185307179586476925286766559",
        "0x6.487ed5110b4611a62633145c0#100",
        Less,
    );
    test_tau_prec_round_helper(
        100,
        Ceiling,
        "6.283185307179586476925286766565",
        "0x6.487ed5110b4611a62633145c8#100",
        Greater,
    );
    test_tau_prec_round_helper(
        100,
        Down,
        "6.283185307179586476925286766559",
        "0x6.487ed5110b4611a62633145c0#100",
        Less,
    );
    test_tau_prec_round_helper(
        100,
        Up,
        "6.283185307179586476925286766565",
        "0x6.487ed5110b4611a62633145c8#100",
        Greater,
    );
    test_tau_prec_round_helper(
        100,
        Nearest,
        "6.283185307179586476925286766559",
        "0x6.487ed5110b4611a62633145c0#100",
        Less,
    );
}

#[test]
#[should_panic]
fn tau_prec_round_fail_1() {
    Float::tau_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn tau_prec_round_fail_2() {
    Float::tau_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn tau_prec_round_fail_3() {
    Float::tau_prec_round(1000, Exact);
}

#[test]
fn tau_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (tau, o) = Float::tau_prec(prec);
        assert!(tau.is_valid());
        assert_eq!(tau.get_prec(), Some(prec));
        assert_eq!(tau.get_exponent(), Some(if prec == 1 { 4 } else { 3 }));
        assert_ne!(o, Equal);
        if o == Less {
            let (tau_alt, o_alt) = Float::tau_prec_round(prec, Ceiling);
            let mut next_upper = tau.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(tau_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !tau.is_power_of_2() {
            let (tau_alt, o_alt) = Float::tau_prec_round(prec, Floor);
            let mut next_lower = tau.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(tau_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
        let (tau_alt, o_alt) = Float::tau_prec_round(prec, Nearest);
        assert_eq!(ComparableFloatRef(&tau_alt), ComparableFloatRef(&tau));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn tau_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (tau, o) = Float::tau_prec_round(prec, rm);
        assert!(tau.is_valid());
        assert_eq!(tau.get_prec(), Some(prec));
        let expected_exponent = match (prec, rm) {
            (1, Ceiling | Up | Nearest) | (2, Ceiling | Up) => 4,
            _ => 3,
        };
        assert_eq!(tau.get_exponent(), Some(expected_exponent));
        assert_ne!(o, Equal);
        if o == Less {
            let (tau_alt, o_alt) = Float::tau_prec_round(prec, Ceiling);
            let mut next_upper = tau.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(tau_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !tau.is_power_of_2() {
            let (tau_alt, o_alt) = Float::tau_prec_round(prec, Floor);
            let mut next_lower = tau.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(tau_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::tau_prec_round(prec, Exact));
    });

    test_constant(Float::tau_prec_round, 10000);
}
