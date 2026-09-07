// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::DottieNumber;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, test_constant, to_hex_string,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_dottie_number_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::dottie_number_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);
}

#[test]
pub fn test_dottie_number_prec() {
    test_dottie_number_prec_helper(1, "0.50", "0x0.8#1", Less);
    test_dottie_number_prec_helper(2, "0.75", "0x0.c#2", Greater);
    test_dottie_number_prec_helper(3, "0.75", "0x0.c#3", Greater);
    test_dottie_number_prec_helper(4, "0.750", "0x0.c#4", Greater);
    test_dottie_number_prec_helper(5, "0.750", "0x0.c0#5", Greater);
    test_dottie_number_prec_helper(6, "0.734", "0x0.bc#6", Less);
    test_dottie_number_prec_helper(7, "0.7422", "0x0.be#7", Greater);
    test_dottie_number_prec_helper(8, "0.7383", "0x0.bd#8", Less);
    test_dottie_number_prec_helper(9, "0.7383", "0x0.bd0#9", Less);
    test_dottie_number_prec_helper(10, "0.73926", "0x0.bd4#10", Greater);
    test_dottie_number_prec_helper(
        100,
        "0.73908513321516064165531208767425",
        "0x0.bd34aeec1e716dcad51803875#100",
        Greater,
    );
    test_dottie_number_prec_helper(
        1000,
        "0.7390851332151606416553120876738734040134117589007574649656806357732846548835475945993761\
        069317665318498012466439871630277149036913084203157804405746207786885249038915392894388450\
        952348013356312767722315809563537765724512043734199364335125384097800343406467004794021434\
        78080271801883771136138204206631661",
        "0x0.bd34aeec1e716dcad5180387486254b0c8893e28fac2ce7b72156c4067d7bfcc9ab4778787d84d948aee55\
        9c69bb822e11e651278998a10ffc188cc842eba179a4064443fa38e727dcf2fbc8ce211ddb25c68c298c88adc2\
        90cc379360d9d72844c8dd337e8d0a064c054614ae7351e265e9c9206d7f7866dca83de85f#1000",
        Greater,
    );
    test_dottie_number_prec_helper(
        10000,
        "0.7390851332151606416553120876738734040134117589007574649656806357732846548835475945993761\
        069317665318498012466439871630277149036913084203157804405746207786885249038915392894388450\
        952348013356312767722315809563537765724512043734199364335125384097800343406467004794021434\
        780802718018837711361382042066316335037277991696731223230061388658203621770810997897062684\
        240588094898683261860600485898958548725736764015075227608180391459518101628159120096461646\
        067544051326415171064466281109360825848783713839555561751414947159390062775275632586349388\
        697301408366515251152042678851530252941718036517642017708607189927601609874327154552267565\
        798246297611775539616699549311158566534834953838523159636025274995587252506666401313187401\
        392538888055206186985921392525285415411079100299828292986405216904655473669687143873564600\
        652122546891499759209699758501364249508565047324972584248371554836483437275837467525453358\
        006642004788397188584890145311550604178123370477739534717103451195854600726561464721419787\
        537388023680295534412794853016207743743315901339193323148766282855217782700523111178246862\
        295712786199584905892978171806015671585092537140418146882858245404644526558831579859786672\
        829905207226868709453130864953504448138762323677656923613259715229415582293341522369636983\
        226580517766853663775937066436792956598287119249110947930112676011522614292437112171487029\
        354310293038780654230310930076000240980335567273089151766682475624772917202594563473838581\
        899954842071818256128193120907757374022398858539642200631322702532845511601107629867410602\
        309969624600462885397784470794286690748442619861922957526396751591784325955509775394210809\
        673139981533825549480527727298065955518624348739998944589817508621057831260139278308279729\
        069169442203868065297723102014746196867501169790022336525951797367054931319760515085764336\
        746344904471871961060181776202361888583971514153183342332512717216270117517974302602487768\
        220006745587381850296594214704617042342065974651396668395615572910071089572813096830388123\
        014338535545478789896281835044064133387698673154620972617717695755498163643567722127665519\
        386653921658520530892128320382599753972069860762847135925096322405907805113805833533570682\
        177053489926852365769625507231160800086376153806565656670581539235783351405567304088330130\
        273342342550323385668941089287721731079810697896029317369565489491916665645882867743651824\
        303351491843591206472177909168120565882794671372581176632650537643078742432427219062371036\
        125236095803622163785931901054471273452764497697458444377581277937535492719697070019285394\
        853609091250594148354515532971701095854976965370516806041703036225081787858695664241897081\
        737878441465912530866010331333706926076970490678392180258192448231784211791452124749034386\
        185410003252547712217909462368047535886848094384022033671353812849934835710899604144065382\
        654052253693850874567201041258945221760471657968923896739493206453806410035347461909144857\
        672342631098976462979337595912127138116962790337085944112162868470924812281618835621762281\
        74636719331427169557473111084668580050788590",
        "0x0.bd34aeec1e716dcad5180387486254b0c8893e28fac2ce7b72156c4067d7bfcc9ab4778787d84d948aee55\
        9c69bb822e11e651278998a10ffc188cc842eba179a4064443fa38e727dcf2fbc8ce211ddb25c68c298c88adc2\
        90cc379360d9d72844c8dd337e8d0a064c054614ae7351e265e9c9206d7f7866dca83de85eb5dd2fe335807d1f\
        c015f2af2a887c3985675efcfde0ea599c4136675bdddc1814b0d0c5e77fc9e3b8966a63dfaec97e1d7283fa3e\
        958e4482484441f42e12b24f417679bc9ec3c12b2ec58fd3432d681b3a72f0d41772610c2ee2c97a5f1dc97322\
        fb48445716f29c1ffeb2d48748d801f5f1e8a2f08db4b051ec398d1b7f8c557877ea97544eaa9f3d7e04bed391\
        65bbb208fe3698fafd69cccd712620dfb9e11ebe8d5cd764e3211090b7114ce59d4f3c334bde782737bd09edbe\
        eb151718c2b4c7804839ff6e588ab7f8212c50750b2cca30b818664b7a21038e4a46c8da2cb4058e186e0725b5\
        e6b61ec68f5b8da0becf6cf943952f0c2efd394bcae417ad57884c5990a0528e952e5b6981cc8d211f69e2e36d\
        73d2777e94ae32236689c9ff59625ee6ffaca8b49a0b0e56e6ec34b4b6cb01e9de1ef00cd304b30f9457c8b1c5\
        aa8dc7ae16878515f122c60b2dd99497b27f676264d11868c6a88d8193494785960a62b7f6620422ff36190845\
        84d7560f063357d31c785653fe283c6155fa24a39cc4478401ccc93f5892a4a7f55b7829d2dac095da5cdd5e03\
        4b29a628275094c668c831288dce648e1458389f0525546ad98be6f7815e5adaa756164c47e8e0e051881eb39b\
        fc88402983f72ee2a5b3e49c71fd3d5453cc6c423a8183faac8aa808142512df66077a0a89c279dbc1bbdbbbcc\
        d814b3487cc455dba26eb3f1a9a0132544927008dbe3b47608d69cacf94c309e6f5bd0ccf6faf785a416d3156a\
        fb8f87b85521645de55962288fd3b09394413c0d5519f63d7ad42c39b6b23e85ae8951a1e18f19e9b780dc2562\
        c4fa306fb415b5c96a72d6074fcd8e4ac947b56da352867493038cc1cc0c643f41aeae723c33006ab8fe42db49\
        e427eeb58af808bb08549ea15a661cc95563a262c7f47db4b6bca1eba43fbc9222062c3a6c9b9c3455ad2b5eeb\
        f975446885afcded853b6e719d24a184da5d41dbb82f0bbbd3e6943ef85179ec242dcbf871107b61b71d308bcf\
        a7eb1739085eb1424539e5f8bf67d9a4a984ae853f4c4f21207aceb531998c9249bc496dc761201a45330eee13\
        a93e7303b5eb7f7b0cb5a6fbd3502f890b7a0c5d6feefc32d87e078c646a63887231ed17db11bf8e7ca6b9ae16\
        41f0a131c65469ffc0534d92da9b298b5a0c24127249dc0859623aeda2124ffc712842c5227a23afd5807aaa7f\
        eb13977fec9d80b290bf9f4651ff5ec5d66b3425f0f5e1ae93bbd4f5fbb7c70d72fabcd2a7e2a2d22d13b0e041\
        7ff3b63cc804e3368b120309f432ad7cc232066d30bef1012cac31b30642cfec32d2a64003dd01daea4891ad27\
        2f115b7b36e60b678d9aedd350a925067a6624bb5e2e7c9edfeeb66034af155303a06c95fd6f5f9d4c08097864\
        d8f58e1126f3a8350b0bb6deedb4bac0b8c00d122e167ef6fa3e31be961795770f6ca4fba1a0b187c4aed2d8b1\
        1c1e4001d99f364f56348bc17766458dd245df16bd147212ebadd08152d113bbcaf8d3bd8de824cb1028c82029\
        e6929bd6033894e8bbe6e045239b7df5b7401e74e9c453a9e2734fef9c3d444b2c5cf6f730#10000",
        Greater,
    );

    let dottie_number_f32 = Float::dottie_number_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(dottie_number_f32.to_string(), "0.739085138");
    assert_eq!(to_hex_string(&dottie_number_f32), "0x0.bd34af#24");
    assert_eq!(dottie_number_f32, f32::DOTTIE_NUMBER);

    let dottie_number_f64 = Float::dottie_number_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(dottie_number_f64.to_string(), "0.73908513321516067");
    assert_eq!(to_hex_string(&dottie_number_f64), "0x0.bd34aeec1e7170#53");
    assert_eq!(dottie_number_f64, f64::DOTTIE_NUMBER);
}

#[test]
#[should_panic]
fn dottie_number_prec_fail_1() {
    Float::dottie_number_prec(0);
}

fn test_dottie_number_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::dottie_number_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);
}

#[test]
pub fn test_dottie_number_prec_round() {
    test_dottie_number_prec_round_helper(1, Floor, "0.50", "0x0.8#1", Less);
    test_dottie_number_prec_round_helper(1, Ceiling, "1.0", "0x1.0#1", Greater);
    test_dottie_number_prec_round_helper(1, Down, "0.50", "0x0.8#1", Less);
    test_dottie_number_prec_round_helper(1, Up, "1.0", "0x1.0#1", Greater);
    test_dottie_number_prec_round_helper(1, Nearest, "0.50", "0x0.8#1", Less);
    test_dottie_number_prec_round_helper(2, Floor, "0.50", "0x0.8#2", Less);
    test_dottie_number_prec_round_helper(2, Ceiling, "0.75", "0x0.c#2", Greater);
    test_dottie_number_prec_round_helper(2, Down, "0.50", "0x0.8#2", Less);
    test_dottie_number_prec_round_helper(2, Up, "0.75", "0x0.c#2", Greater);
    test_dottie_number_prec_round_helper(2, Nearest, "0.75", "0x0.c#2", Greater);
    test_dottie_number_prec_round_helper(3, Floor, "0.62", "0x0.a#3", Less);
    test_dottie_number_prec_round_helper(3, Ceiling, "0.75", "0x0.c#3", Greater);
    test_dottie_number_prec_round_helper(3, Down, "0.62", "0x0.a#3", Less);
    test_dottie_number_prec_round_helper(3, Up, "0.75", "0x0.c#3", Greater);
    test_dottie_number_prec_round_helper(3, Nearest, "0.75", "0x0.c#3", Greater);
    test_dottie_number_prec_round_helper(4, Floor, "0.688", "0x0.b#4", Less);
    test_dottie_number_prec_round_helper(4, Ceiling, "0.750", "0x0.c#4", Greater);
    test_dottie_number_prec_round_helper(4, Down, "0.688", "0x0.b#4", Less);
    test_dottie_number_prec_round_helper(4, Up, "0.750", "0x0.c#4", Greater);
    test_dottie_number_prec_round_helper(4, Nearest, "0.750", "0x0.c#4", Greater);
    test_dottie_number_prec_round_helper(5, Floor, "0.719", "0x0.b8#5", Less);
    test_dottie_number_prec_round_helper(5, Ceiling, "0.750", "0x0.c0#5", Greater);
    test_dottie_number_prec_round_helper(5, Down, "0.719", "0x0.b8#5", Less);
    test_dottie_number_prec_round_helper(5, Up, "0.750", "0x0.c0#5", Greater);
    test_dottie_number_prec_round_helper(5, Nearest, "0.750", "0x0.c0#5", Greater);
    test_dottie_number_prec_round_helper(6, Floor, "0.734", "0x0.bc#6", Less);
    test_dottie_number_prec_round_helper(6, Ceiling, "0.750", "0x0.c0#6", Greater);
    test_dottie_number_prec_round_helper(6, Down, "0.734", "0x0.bc#6", Less);
    test_dottie_number_prec_round_helper(6, Up, "0.750", "0x0.c0#6", Greater);
    test_dottie_number_prec_round_helper(6, Nearest, "0.734", "0x0.bc#6", Less);
    test_dottie_number_prec_round_helper(7, Floor, "0.7344", "0x0.bc#7", Less);
    test_dottie_number_prec_round_helper(7, Ceiling, "0.7422", "0x0.be#7", Greater);
    test_dottie_number_prec_round_helper(7, Down, "0.7344", "0x0.bc#7", Less);
    test_dottie_number_prec_round_helper(7, Up, "0.7422", "0x0.be#7", Greater);
    test_dottie_number_prec_round_helper(7, Nearest, "0.7422", "0x0.be#7", Greater);
    test_dottie_number_prec_round_helper(8, Floor, "0.7383", "0x0.bd#8", Less);
    test_dottie_number_prec_round_helper(8, Ceiling, "0.7422", "0x0.be#8", Greater);
    test_dottie_number_prec_round_helper(8, Down, "0.7383", "0x0.bd#8", Less);
    test_dottie_number_prec_round_helper(8, Up, "0.7422", "0x0.be#8", Greater);
    test_dottie_number_prec_round_helper(8, Nearest, "0.7383", "0x0.bd#8", Less);
    test_dottie_number_prec_round_helper(9, Floor, "0.7383", "0x0.bd0#9", Less);
    test_dottie_number_prec_round_helper(9, Ceiling, "0.7402", "0x0.bd8#9", Greater);
    test_dottie_number_prec_round_helper(9, Down, "0.7383", "0x0.bd0#9", Less);
    test_dottie_number_prec_round_helper(9, Up, "0.7402", "0x0.bd8#9", Greater);
    test_dottie_number_prec_round_helper(9, Nearest, "0.7383", "0x0.bd0#9", Less);
    test_dottie_number_prec_round_helper(10, Floor, "0.73828", "0x0.bd0#10", Less);
    test_dottie_number_prec_round_helper(10, Ceiling, "0.73926", "0x0.bd4#10", Greater);
    test_dottie_number_prec_round_helper(10, Down, "0.73828", "0x0.bd0#10", Less);
    test_dottie_number_prec_round_helper(10, Up, "0.73926", "0x0.bd4#10", Greater);
    test_dottie_number_prec_round_helper(10, Nearest, "0.73926", "0x0.bd4#10", Greater);
    test_dottie_number_prec_round_helper(
        100,
        Floor,
        "0.73908513321516064165531208767346",
        "0x0.bd34aeec1e716dcad51803874#100",
        Less,
    );
    test_dottie_number_prec_round_helper(
        100,
        Ceiling,
        "0.73908513321516064165531208767425",
        "0x0.bd34aeec1e716dcad51803875#100",
        Greater,
    );
}

#[test]
#[should_panic]
fn dottie_number_prec_round_fail_1() {
    Float::dottie_number_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn dottie_number_prec_round_fail_2() {
    Float::dottie_number_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn dottie_number_prec_round_fail_3() {
    Float::dottie_number_prec_round(1000, Exact);
}

#[test]
fn dottie_number_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (dottie_number, o) = Float::dottie_number_prec(prec);
        assert!(dottie_number.is_valid());
        assert_rounding_ordering_consistent(&dottie_number, Nearest, o);
        assert_eq!(dottie_number.get_prec(), Some(prec));
        assert_eq!(dottie_number.get_exponent(), Some(0));
        assert_ne!(o, Equal);
        if o == Less {
            let (dottie_number_alt, o_alt) = Float::dottie_number_prec_round(prec, Ceiling);
            let mut next_upper = dottie_number.clone();
            next_upper.increment();
            assert_eq!(
                ComparableFloat(dottie_number_alt),
                ComparableFloat(next_upper)
            );
            assert_eq!(o_alt, Greater);
        } else {
            let (dottie_number_alt, o_alt) = Float::dottie_number_prec_round(prec, Floor);
            let mut next_lower = dottie_number.clone();
            next_lower.decrement();
            assert_eq!(
                ComparableFloat(dottie_number_alt),
                ComparableFloat(next_lower)
            );
            assert_eq!(o_alt, Less);
        }
        let (dottie_number_alt, o_alt) = Float::dottie_number_prec_round(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&dottie_number_alt),
            ComparableFloatRef(&dottie_number)
        );
        assert_eq!(o_alt, o);
    });
}

#[test]
fn dottie_number_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (dottie_number, o) = Float::dottie_number_prec_round(prec, rm);
        assert!(dottie_number.is_valid());
        assert_rounding_ordering_consistent(&dottie_number, rm, o);
        assert_eq!(dottie_number.get_prec(), Some(prec));
        let expected_exponent = match (prec, rm) {
            (1, Ceiling | Up) => 1,
            _ => 0,
        };
        assert_eq!(dottie_number.get_exponent(), Some(expected_exponent));
        assert_ne!(o, Equal);
        if o == Less {
            let (dottie_number_alt, o_alt) = Float::dottie_number_prec_round(prec, Ceiling);
            let mut next_upper = dottie_number.clone();
            next_upper.increment();
            assert_eq!(
                ComparableFloat(dottie_number_alt),
                ComparableFloat(next_upper)
            );
            assert_eq!(o_alt, Greater);
        } else {
            let (dottie_number_alt, o_alt) = Float::dottie_number_prec_round(prec, Floor);
            let mut next_lower = dottie_number.clone();
            next_lower.decrement();
            assert_eq!(
                ComparableFloat(dottie_number_alt),
                ComparableFloat(next_lower)
            );
            assert_eq!(o_alt, Less);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::dottie_number_prec_round(prec, Exact));
    });

    test_constant(Float::dottie_number_prec_round, 10000);
}
