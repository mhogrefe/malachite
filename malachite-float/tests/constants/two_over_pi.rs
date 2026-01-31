// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::TwoOverPi;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{test_constant, to_hex_string};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_two_over_pi_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::two_over_pi_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);
}

#[test]
pub fn test_two_over_pi_prec() {
    test_two_over_pi_prec_helper(1, "0.5", "0x0.8#1", Less);
    test_two_over_pi_prec_helper(2, "0.8", "0x0.c#2", Greater);
    test_two_over_pi_prec_helper(3, "0.6", "0x0.a#3", Less);
    test_two_over_pi_prec_helper(4, "0.62", "0x0.a#4", Less);
    test_two_over_pi_prec_helper(5, "0.62", "0x0.a0#5", Less);
    test_two_over_pi_prec_helper(6, "0.64", "0x0.a4#6", Greater);
    test_two_over_pi_prec_helper(7, "0.63", "0x0.a2#7", Less);
    test_two_over_pi_prec_helper(8, "0.637", "0x0.a3#8", Greater);
    test_two_over_pi_prec_helper(9, "0.637", "0x0.a30#9", Greater);
    test_two_over_pi_prec_helper(10, "0.637", "0x0.a30#10", Greater);
    test_two_over_pi_prec_helper(
        100,
        "0.63661977236758134307553505349",
        "0x0.a2f9836e4e441529fc2757d1f#100",
        Less,
    );
    test_two_over_pi_prec_helper(
        1000,
        "0.636619772367581343075535053490057448137838582961825794990669376235587190536906140360455\
        211065012343824291370907031832147571647384458314611511869642926799356916959867749636310292\
        310985587701230754869571584869590646773449560966894516047329520456890799022863761847560347\
        6106958244819576437477513763421149",
        "0x0.a2f9836e4e441529fc2757d1f534ddc0db6295993c439041fe5163abdebbc561b7246e3a424dd2e006492\
        eea09d1921cfe1deb1cb129a73ee88235f52ebb4484e99c7026b45f7e413991d639835339f49c845f8bbdf9283\
        b1ff897ffde05980fef2f118b5a0a6d1f6d367ecf27cb09b74f463f669e5fea2d7527bac7ec#1000",
        Greater,
    );
    test_two_over_pi_prec_helper(
        10000,
        "0.636619772367581343075535053490057448137838582961825794990669376235587190536906140360455\
        211065012343824291370907031832147571647384458314611511869642926799356916959867749636310292\
        310985587701230754869571584869590646773449560966894516047329520456890799022863761847560347\
        610695824481957643747751376342114892399785773600994689390957838443593292387132299624667945\
        851218797794608751526299146267856964155983496557394439935472396799849771502340684715433724\
        470075068642186190147952038957841459037335072237209977986541221308627102012881299111265588\
        664091786992478392663362424067212143992535647949995331146617741119020280064962710257555398\
        285243520488797504590725511058951562532272185831913927045249709256279843100098001191039428\
        356227611187140526100840065270984083699246424962245824812585936356993836765740846301630224\
        803486106427208868636563029898330890390985141599500621317563255927089637433019188293314876\
        162799903630630831397388157435931234869370256146758046650182823773310525074600104490871884\
        612845039801754671780150502243345268467810390325128997664933372580424494147514252454546768\
        668568278987840517002313344212478434378039358226874839818986041726495262070323357771919883\
        998021017550264517783533227384203141166060564161957195402555264310478797229364155998314767\
        562392374951088247501728908757205465021044955121550155524427256270617363313114107733707198\
        224283161544241410955984980503982997105188094376382337204659318564742310849623017797828087\
        159079169637961309179080866598414261272614176015362759498870766355052763866027857619107882\
        750734627112419119181801413583033207527354751751064499259812239862320876334395004140508516\
        172926321994878747511037862653848841368177634219914015170954777174146477511317149437513738\
        812920948583351694228474545367717840732729167856660035132317325413991163989834597161069802\
        439574756378353220134812215221892492863237727907041291325256759238999289753340697427959390\
        004158002735520159146894398432096010956043499819419151694273044559795613075989708333984459\
        683315615107138972142018273824334685917233826893308141941570224808347357296398248847013273\
        576083883174283099861995234744265443874647868149898168411324877007384899339964644598266224\
        151878704559725131984310433111960403132144009353091951634160955046229781723704047640217351\
        993556186196849931806428291412020908840944070093252692719037244201312620437495654558581223\
        170428720334471819506898583921895909169792436803748503147673331583545135961743474666559026\
        937805638014549308766972455522655322903692110389380242192851112148261351132128683950939866\
        273963201307954026967165858734033126467413257344642923980599412479278935033776839366623816\
        609002573577251457761535534246035190865800682588270075098242366434867431431756904939025326\
        844531994623766387562879402754976920230076790822760152873570248813549694145027233416626069\
        188435246887183747330259540749998994834212466393224405568578178406459538110810045644280994\
        086958980415466945615491440398699572694247248284696191559747554622769231394009222822857625\
        45545280947408042964022994369124462887872014",
        "0x0.a2f9836e4e441529fc2757d1f534ddc0db6295993c439041fe5163abdebbc561b7246e3a424dd2e006492\
        eea09d1921cfe1deb1cb129a73ee88235f52ebb4484e99c7026b45f7e413991d639835339f49c845f8bbdf9283\
        b1ff897ffde05980fef2f118b5a0a6d1f6d367ecf27cb09b74f463f669e5fea2d7527bac7ebe5f17b3d0739f78\
        a5292ea6bfb5fb11f8d5d0856033046fc7b6babf0cfbc209af4361da9e391615ee61b086599855f14a068408df\
        fd8804d73273106061556ca73a8c960e27bc08c6b47c419c367cddce8092a8359c4768b961ca6ddaf44d157190\
        53ea5ff07053f7e33e832c2de4f98327dbbc33d26ef6b1e5ef89f3a1f35caf27f1d87f121907c7c246afa6ed57\
        72d30433b15c614b59d19c3c2c4ad414d2c5d000c467d862d71e39ac69b0062337cd2b497a7b4d55537f63ed71\
        810a3fc764d2a9d64abd770f87c6357b07ae715175649c0d9d63b3884a7cb2324778ad623545ab91f001b0af1d\
        fce19ff319f6a1e6661579947fbacd87f7eb7652289e83260bfe6cdc4ef09366cd43f5dd7de16de3b58929bde2\
        822d2e886284d58e232cac616e308cb7de050c017a71df35be01834132e6212830148835b8ef57fb0adf2e91e4\
        34a48d36710d8ddaa425faece616aa4280ab499d3f2a6067f775c83c2a3883c6178738a5a8cafbdd76f63a62dc\
        bbff4ef818d67c12645ca5536d9cad2a8288d61c277c9121426049b4612c459c444c5c891b24df31700ad43d4e\
        5492910d5fdfcbe00cc941eeece70f53e1380f1ecc3e7b328f8c79405933e71c1b3092ef3450b9c12887b20ab9\
        fb52ec292472f327b6d550c90a7721fe76b96cb314a1679e2794189dff49794e884e6e29731996bed88365f5f0\
        efdbbb49a486ca467427271325d8db8159f09e5bc25318d3974f71c0530010c0d68084b58ee2c90aa4702e7742\
        4d6bda67df772486eef169fa6948ef691b45153d1f20acf3398207e4bf56863b25f3edd035d407f8985295255c\
        0643710d86d324832754c5bd4714e6e5445c1090b69f52ad566149d072750045ddb3bb4c576ea17f9877d6b49b\
        a271d296996acccc65414ad6ae29089d98850722cbea4049407777030f327fc00a871ea49c2663de06483dd979\
        73fa3fd94438c860dde41319d39928c70dde7b7173bdf082b3715a0805c93805a921110d8e80faf806c4bffdb0\
        f903876185915a562bbcb61b989c7bd401004f2d2277549f6b6ebbb22dbaa140a2f2689768364333b091a940ea\
        a3a51c2a31daeedaf12265c4dc26d9c7a2d9756c0833f03f6f0098c402b99316d07b43915200c5bc3d8c492f54\
        badc6a5ca4ecd37a736a9e69492ab6842ddde6319ef8c76528b6837dbfcaba1ae3115dfa1ae00dafb0c664d64b\
        705ed306529bf56573aff47b9f96af3be75df93283080abf68c6615cb040622fa1de4d9a4b33d8f1b5709cd36e\
        9424ea4be13b523331aaaf0a8654fa5c1d20f3f0bcd785b76f923048b7b72178953a6c6e26e6f00ebef584a9bb\
        7dac4ba66aacfcf761d02d12df1b1c1998c77adc3da4886a05df7f480c62ff0ac9aecddbc5c3f6dded01fc790b\
        6db2a3a25a39aaf009353ad0457b6b42d297e804ba707da0eaa76a1597b2a12162db7dcfde5fafedb89fdbe896\
        c76e4fca90670803e156e85ff87fd073e2833676186182aeabd4dafe7b36e6d8f3967955bbf3148d78416df304\
        32dc7356125ce70c9b8cb30fd6cbfa200a4e46c05a0dd5a476f21d21262845cb9496170e056#10000",
        Less,
    );

    let two_over_pi_f32 = Float::two_over_pi_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(two_over_pi_f32.to_string(), "0.63661975");
    assert_eq!(to_hex_string(&two_over_pi_f32), "0x0.a2f983#24");
    assert_eq!(two_over_pi_f32, f32::TWO_OVER_PI);

    let two_over_pi_f64 = Float::two_over_pi_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(two_over_pi_f64.to_string(), "0.6366197723675814");
    assert_eq!(to_hex_string(&two_over_pi_f64), "0x0.a2f9836e4e4418#53");
    assert_eq!(two_over_pi_f64, f64::TWO_OVER_PI);
}

#[test]
#[should_panic]
fn two_over_pi_prec_fail_1() {
    Float::two_over_pi_prec(0);
}

fn test_two_over_pi_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::two_over_pi_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);
}

#[test]
pub fn test_two_over_pi_prec_round() {
    test_two_over_pi_prec_round_helper(1, Floor, "0.5", "0x0.8#1", Less);
    test_two_over_pi_prec_round_helper(1, Ceiling, "1.0", "0x1.0#1", Greater);
    test_two_over_pi_prec_round_helper(1, Down, "0.5", "0x0.8#1", Less);
    test_two_over_pi_prec_round_helper(1, Up, "1.0", "0x1.0#1", Greater);
    test_two_over_pi_prec_round_helper(1, Nearest, "0.5", "0x0.8#1", Less);

    test_two_over_pi_prec_round_helper(2, Floor, "0.5", "0x0.8#2", Less);
    test_two_over_pi_prec_round_helper(2, Ceiling, "0.8", "0x0.c#2", Greater);
    test_two_over_pi_prec_round_helper(2, Down, "0.5", "0x0.8#2", Less);
    test_two_over_pi_prec_round_helper(2, Up, "0.8", "0x0.c#2", Greater);
    test_two_over_pi_prec_round_helper(2, Nearest, "0.8", "0x0.c#2", Greater);

    test_two_over_pi_prec_round_helper(3, Floor, "0.6", "0x0.a#3", Less);
    test_two_over_pi_prec_round_helper(3, Ceiling, "0.8", "0x0.c#3", Greater);
    test_two_over_pi_prec_round_helper(3, Down, "0.6", "0x0.a#3", Less);
    test_two_over_pi_prec_round_helper(3, Up, "0.8", "0x0.c#3", Greater);
    test_two_over_pi_prec_round_helper(3, Nearest, "0.6", "0x0.a#3", Less);

    test_two_over_pi_prec_round_helper(4, Floor, "0.62", "0x0.a#4", Less);
    test_two_over_pi_prec_round_helper(4, Ceiling, "0.7", "0x0.b#4", Greater);
    test_two_over_pi_prec_round_helper(4, Down, "0.62", "0x0.a#4", Less);
    test_two_over_pi_prec_round_helper(4, Up, "0.7", "0x0.b#4", Greater);
    test_two_over_pi_prec_round_helper(4, Nearest, "0.62", "0x0.a#4", Less);

    test_two_over_pi_prec_round_helper(5, Floor, "0.62", "0x0.a0#5", Less);
    test_two_over_pi_prec_round_helper(5, Ceiling, "0.66", "0x0.a8#5", Greater);
    test_two_over_pi_prec_round_helper(5, Down, "0.62", "0x0.a0#5", Less);
    test_two_over_pi_prec_round_helper(5, Up, "0.66", "0x0.a8#5", Greater);
    test_two_over_pi_prec_round_helper(5, Nearest, "0.62", "0x0.a0#5", Less);

    test_two_over_pi_prec_round_helper(6, Floor, "0.62", "0x0.a0#6", Less);
    test_two_over_pi_prec_round_helper(6, Ceiling, "0.64", "0x0.a4#6", Greater);
    test_two_over_pi_prec_round_helper(6, Down, "0.62", "0x0.a0#6", Less);
    test_two_over_pi_prec_round_helper(6, Up, "0.64", "0x0.a4#6", Greater);
    test_two_over_pi_prec_round_helper(6, Nearest, "0.64", "0x0.a4#6", Greater);

    test_two_over_pi_prec_round_helper(7, Floor, "0.63", "0x0.a2#7", Less);
    test_two_over_pi_prec_round_helper(7, Ceiling, "0.64", "0x0.a4#7", Greater);
    test_two_over_pi_prec_round_helper(7, Down, "0.63", "0x0.a2#7", Less);
    test_two_over_pi_prec_round_helper(7, Up, "0.64", "0x0.a4#7", Greater);
    test_two_over_pi_prec_round_helper(7, Nearest, "0.63", "0x0.a2#7", Less);

    test_two_over_pi_prec_round_helper(8, Floor, "0.633", "0x0.a2#8", Less);
    test_two_over_pi_prec_round_helper(8, Ceiling, "0.637", "0x0.a3#8", Greater);
    test_two_over_pi_prec_round_helper(8, Down, "0.633", "0x0.a2#8", Less);
    test_two_over_pi_prec_round_helper(8, Up, "0.637", "0x0.a3#8", Greater);
    test_two_over_pi_prec_round_helper(8, Nearest, "0.637", "0x0.a3#8", Greater);

    test_two_over_pi_prec_round_helper(9, Floor, "0.635", "0x0.a28#9", Less);
    test_two_over_pi_prec_round_helper(9, Ceiling, "0.637", "0x0.a30#9", Greater);
    test_two_over_pi_prec_round_helper(9, Down, "0.635", "0x0.a28#9", Less);
    test_two_over_pi_prec_round_helper(9, Up, "0.637", "0x0.a30#9", Greater);
    test_two_over_pi_prec_round_helper(9, Nearest, "0.637", "0x0.a30#9", Greater);

    test_two_over_pi_prec_round_helper(10, Floor, "0.636", "0x0.a2c#10", Less);
    test_two_over_pi_prec_round_helper(10, Ceiling, "0.637", "0x0.a30#10", Greater);
    test_two_over_pi_prec_round_helper(10, Down, "0.636", "0x0.a2c#10", Less);
    test_two_over_pi_prec_round_helper(10, Up, "0.637", "0x0.a30#10", Greater);
    test_two_over_pi_prec_round_helper(10, Nearest, "0.637", "0x0.a30#10", Greater);

    test_two_over_pi_prec_round_helper(
        100,
        Floor,
        "0.63661977236758134307553505349",
        "0x0.a2f9836e4e441529fc2757d1f#100",
        Less,
    );
    test_two_over_pi_prec_round_helper(
        100,
        Ceiling,
        "0.6366197723675813430755350534906",
        "0x0.a2f9836e4e441529fc2757d20#100",
        Greater,
    );
    test_two_over_pi_prec_round_helper(
        100,
        Down,
        "0.63661977236758134307553505349",
        "0x0.a2f9836e4e441529fc2757d1f#100",
        Less,
    );
    test_two_over_pi_prec_round_helper(
        100,
        Up,
        "0.6366197723675813430755350534906",
        "0x0.a2f9836e4e441529fc2757d20#100",
        Greater,
    );
    test_two_over_pi_prec_round_helper(
        100,
        Nearest,
        "0.63661977236758134307553505349",
        "0x0.a2f9836e4e441529fc2757d1f#100",
        Less,
    );
}

#[test]
#[should_panic]
fn two_over_pi_prec_round_fail_1() {
    Float::two_over_pi_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn two_over_pi_prec_round_fail_2() {
    Float::two_over_pi_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn two_over_pi_prec_round_fail_3() {
    Float::two_over_pi_prec_round(1000, Exact);
}

#[test]
fn two_over_pi_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (two_over_pi, o) = Float::two_over_pi_prec(prec);
        assert!(two_over_pi.is_valid());
        assert_eq!(two_over_pi.get_prec(), Some(prec));
        assert_eq!(two_over_pi.get_exponent(), Some(0));
        assert_ne!(o, Equal);
        if o == Less {
            let (two_over_pi_alt, o_alt) = Float::two_over_pi_prec_round(prec, Ceiling);
            let mut next_upper = two_over_pi.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(
                    ComparableFloat(two_over_pi_alt),
                    ComparableFloat(next_upper)
                );
                assert_eq!(o_alt, Greater);
            }
        } else if !two_over_pi.is_power_of_2() {
            let (two_over_pi_alt, o_alt) = Float::two_over_pi_prec_round(prec, Floor);
            let mut next_lower = two_over_pi.clone();
            next_lower.decrement();
            assert_eq!(
                ComparableFloat(two_over_pi_alt),
                ComparableFloat(next_lower)
            );
            assert_eq!(o_alt, Less);
        }
        let (two_over_pi_alt, o_alt) = Float::two_over_pi_prec_round(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&two_over_pi_alt),
            ComparableFloatRef(&two_over_pi)
        );
        assert_eq!(o_alt, o);
    });
}

#[test]
fn two_over_pi_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (two_over_pi, o) = Float::two_over_pi_prec_round(prec, rm);
        assert!(two_over_pi.is_valid());
        assert_eq!(two_over_pi.get_prec(), Some(prec));
        assert_eq!(
            two_over_pi.get_exponent(),
            Some(if prec == 1 && (rm == Ceiling || rm == Up) {
                1
            } else {
                0
            })
        );
        assert_ne!(o, Equal);
        if o == Less {
            let (two_over_pi_alt, o_alt) = Float::two_over_pi_prec_round(prec, Ceiling);
            let mut next_upper = two_over_pi.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(
                    ComparableFloat(two_over_pi_alt),
                    ComparableFloat(next_upper)
                );
                assert_eq!(o_alt, Greater);
            }
        } else if !two_over_pi.is_power_of_2() {
            let (two_over_pi_alt, o_alt) = Float::two_over_pi_prec_round(prec, Floor);
            let mut next_lower = two_over_pi.clone();
            next_lower.decrement();
            assert_eq!(
                ComparableFloat(two_over_pi_alt),
                ComparableFloat(next_lower)
            );
            assert_eq!(o_alt, Less);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::two_over_pi_prec_round(prec, Exact));
    });

    test_constant(Float::two_over_pi_prec_round, 10000);
}
