// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::Pi;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{
    rug_round_try_from_rounding_mode, test_constant, to_hex_string,
};
use malachite_float::test_util::constants::pi::rug_pi_prec_round;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_pi_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::pi_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (rug_x, rug_o) =
        rug_pi_prec_round(prec, rug_round_try_from_rounding_mode(Nearest).unwrap());
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_x)),
        ComparableFloatRef(&x)
    );
    assert_eq!(rug_o, o);
}

#[test]
pub fn test_pi_prec() {
    test_pi_prec_helper(1, "4.0", "0x4.0#1", Greater);
    test_pi_prec_helper(2, "3.0", "0x3.0#2", Less);
    test_pi_prec_helper(3, "3.0", "0x3.0#3", Less);
    test_pi_prec_helper(4, "3.2", "0x3.4#4", Greater);
    test_pi_prec_helper(5, "3.1", "0x3.2#5", Less);
    test_pi_prec_helper(6, "3.12", "0x3.2#6", Less);
    test_pi_prec_helper(7, "3.16", "0x3.28#7", Greater);
    test_pi_prec_helper(8, "3.14", "0x3.24#8", Less);
    test_pi_prec_helper(9, "3.14", "0x3.24#9", Less);
    test_pi_prec_helper(10, "3.141", "0x3.24#10", Less);
    test_pi_prec_helper(
        100,
        "3.141592653589793238462643383279",
        "0x3.243f6a8885a308d313198a2e0#100",
        Less,
    );
    test_pi_prec_helper(
        1000,
        "3.141592653589793238462643383279502884197169399375105820974944592307816406286208998628034\
        825342117067982148086513282306647093844609550582231725359408128481117450284102701938521105\
        559644622948954930381964428810975665933446128475648233786783165271201909145648566923460348\
        6104543266482133936072602491412736",
        "0x3.243f6a8885a308d313198a2e03707344a4093822299f31d0082efa98ec4e6c89452821e638d01377be546\
        6cf34e90c6cc0ac29b7c97c50dd3f84d5b5b54709179216d5d98979fb1bd1310ba698dfb5ac2ffd72dbd01adfb\
        7b8e1afed6a267e96ba7c9045f12c7f9924a19947b3916cf70801f2e2858efc16636920d870#1000",
        Less,
    );
    test_pi_prec_helper(
        10000,
        "3.141592653589793238462643383279502884197169399375105820974944592307816406286208998628034\
        825342117067982148086513282306647093844609550582231725359408128481117450284102701938521105\
        559644622948954930381964428810975665933446128475648233786783165271201909145648566923460348\
        610454326648213393607260249141273724587006606315588174881520920962829254091715364367892590\
        360011330530548820466521384146951941511609433057270365759591953092186117381932611793105118\
        548074462379962749567351885752724891227938183011949129833673362440656643086021394946395224\
        737190702179860943702770539217176293176752384674818467669405132000568127145263560827785771\
        342757789609173637178721468440901224953430146549585371050792279689258923542019956112129021\
        960864034418159813629774771309960518707211349999998372978049951059731732816096318595024459\
        455346908302642522308253344685035261931188171010003137838752886587533208381420617177669147\
        303598253490428755468731159562863882353787593751957781857780532171226806613001927876611195\
        909216420198938095257201065485863278865936153381827968230301952035301852968995773622599413\
        891249721775283479131515574857242454150695950829533116861727855889075098381754637464939319\
        255060400927701671139009848824012858361603563707660104710181942955596198946767837449448255\
        379774726847104047534646208046684259069491293313677028989152104752162056966024058038150193\
        511253382430035587640247496473263914199272604269922796782354781636009341721641219924586315\
        030286182974555706749838505494588586926995690927210797509302955321165344987202755960236480\
        665499119881834797753566369807426542527862551818417574672890977772793800081647060016145249\
        192173217214772350141441973568548161361157352552133475741849468438523323907394143334547762\
        416862518983569485562099219222184272550254256887671790494601653466804988627232791786085784\
        383827967976681454100953883786360950680064225125205117392984896084128488626945604241965285\
        022210661186306744278622039194945047123713786960956364371917287467764657573962413890865832\
        645995813390478027590099465764078951269468398352595709825822620522489407726719478268482601\
        476990902640136394437455305068203496252451749399651431429809190659250937221696461515709858\
        387410597885959772975498930161753928468138268683868942774155991855925245953959431049972524\
        680845987273644695848653836736222626099124608051243884390451244136549762780797715691435997\
        700129616089441694868555848406353422072225828488648158456028506016842739452267467678895252\
        138522549954666727823986456596116354886230577456498035593634568174324112515076069479451096\
        596094025228879710893145669136867228748940560101503308617928680920874760917824938589009714\
        909675985261365549781893129784821682998948722658804857564014270477555132379641451523746234\
        364542858444795265867821051141354735739523113427166102135969536231442952484937187110145765\
        403590279934403742007310578539062198387447808478489683321445713868751943506430218453191048\
        481005370614680674919278191197939952061419663428754440643745123718192179998391015919561814\
        675142691239748940907186494231961567945208",
        "0x3.243f6a8885a308d313198a2e03707344a4093822299f31d0082efa98ec4e6c89452821e638d01377be546\
        6cf34e90c6cc0ac29b7c97c50dd3f84d5b5b54709179216d5d98979fb1bd1310ba698dfb5ac2ffd72dbd01adfb\
        7b8e1afed6a267e96ba7c9045f12c7f9924a19947b3916cf70801f2e2858efc16636920d871574e69a458fea3f\
        4933d7e0d95748f728eb658718bcd5882154aee7b54a41dc25a59b59c30d5392af26013c5d1b023286085f0ca4\
        17918b8db38ef8e79dcb0603a180e6c9e0e8bb01e8a3ed71577c1bd314b2778af2fda55605c60e65525f3aa55a\
        b945748986263e8144055ca396a2aab10b6b4cc5c341141e8cea15486af7c72e993b3ee1411636fbc2a2ba9c55\
        d741831f6ce5c3e169b87931eafd6ba336c24cf5c7a325381289586773b8f48986b4bb9afc4bfe81b662821936\
        1d809ccfb21a991487cac605dec8032ef845d5de98575b1dc262302eb651b8823893e81d396acc50f6d6ff383f\
        442392e0b4482a484200469c8f04a9e1f9b5e21c66842f6e96c9a670c9c61abd388f06a51a0d2d8542f68960fa\
        728ab5133a36eef0b6c137a3be4ba3bf0507efb2a98a1f1651d39af017666ca593e82430e888cee8619456f9fb\
        47d84a5c33b8b5ebee06f75d885c12073401a449f56c16aa64ed3aa62363f77061bfedf72429b023d37d0d724d\
        00a1248db0fead349f1c09b075372c980991b7b25d479d8f6e8def7e3fe501ab6794c3b976ce0bd04c006bac1a\
        94fb6409f60c45e5c9ec2196a246368fb6faf3e6c53b51339b2eb3b52ec6f6dfc511f9b30952ccc814544af5eb\
        d09bee3d004de334afd660f2807192e4bb3c0cba85745c8740fd20b5f39b9d3fbdb5579c0bd1a60320ad6a100c\
        6402c7279679f25fefb1fa3cc8ea5e9f8db3222f83c7516dffd616b152f501ec8ad0552ab323db5fafd2387605\
        3317b483e00df829e5c57bbca6f8ca01a87562edf1769dbd542a8f6287effc3ac6732c68c4f5573695b27b0bbc\
        a58c8e1ffa35db8f011a010fa3d98fd2183b84afcb56c2dd1d35b9a53e479b6f84565d28e49bc4bfb9790e1ddf\
        2daa4cb7e3362fb1341cee4c6e8ef20cada36774c01d07e9efe2bf11fb495dbda4dae909198eaad8e716b93d5a\
        0d08ed1d0afc725e08e3c5b2f8e7594b78ff6e2fbf2122b648888b812900df01c4fad5ea0688fc31cd1cff191b\
        3a8c1ad2f2f2218be0e1777ea752dfe8b021fa1e5a0cc0fb56f74e818acf3d6ce89e299b4a84fe0fd13e0b77cc\
        43b81d2ada8d9165fa2668095770593cc7314211a1477e6ad206577b5fa86c75442f5fb9d35cfebcdaf0c7b3e8\
        9a0d6411bd3ae1e7e4900250e2d2071b35e226800bb57b8e0af2464369bf009b91e5563911d59dfa6aa78c1438\
        9d95a537f207d5ba202e5b9c5832603766295cfa911c819684e734a41b3472dca7b14a94a1b5100529a532915d\
        60f573fbc9bc6e42b60a47681e6740008ba6fb5571be91ff296ec6b2a0dd915b6636521e7b9f9b6ff34052ec58\
        5566453b02d5da99f8fa108ba47996e85076a4b7a70e9b5b32944db75092ec4192623ad6ea6b049a7df7d9cee6\
        0b88fedb266ecaa8c71699a17ff5664526cc2b19ee1193602a575094c29a0591340e4183a3e3f54989a5b429d6\
        56b8fe4d699f73fd6a1d29c07efe830f54d2d38e6f0255dc14cdd20868470eb266382e9c6021ecc5e09686b3f3\
        ebaefc93c9718146b6a70a1687f358452a0e286b79c5305aa5007373e07841c7fdeae5c8e7c#10000",
        Less,
    );

    let pi_f32 = Float::pi_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(pi_f32.to_string(), "3.1415927");
    assert_eq!(to_hex_string(&pi_f32), "0x3.243f6c#24");
    assert_eq!(pi_f32, f32::PI);

    let pi_f64 = Float::pi_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(pi_f64.to_string(), "3.1415926535897931");
    assert_eq!(to_hex_string(&pi_f64), "0x3.243f6a8885a30#53");
    assert_eq!(pi_f64, f64::PI);
}

#[test]
#[should_panic]
fn pi_prec_fail_1() {
    Float::pi_prec(0);
}

fn test_pi_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::pi_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_x, rug_o) = rug_pi_prec_round(prec, rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_x)),
            ComparableFloatRef(&x)
        );
        assert_eq!(rug_o, o);
    }
}

#[test]
pub fn test_pi_prec_round_xxx() {
    test_pi_prec_round_helper(1, Floor, "2.0", "0x2.0#1", Less);
    test_pi_prec_round_helper(1, Ceiling, "4.0", "0x4.0#1", Greater);
    test_pi_prec_round_helper(1, Down, "2.0", "0x2.0#1", Less);
    test_pi_prec_round_helper(1, Up, "4.0", "0x4.0#1", Greater);
    test_pi_prec_round_helper(1, Nearest, "4.0", "0x4.0#1", Greater);

    test_pi_prec_round_helper(2, Floor, "3.0", "0x3.0#2", Less);
    test_pi_prec_round_helper(2, Ceiling, "4.0", "0x4.0#2", Greater);
    test_pi_prec_round_helper(2, Down, "3.0", "0x3.0#2", Less);
    test_pi_prec_round_helper(2, Up, "4.0", "0x4.0#2", Greater);
    test_pi_prec_round_helper(2, Nearest, "3.0", "0x3.0#2", Less);

    test_pi_prec_round_helper(3, Floor, "3.0", "0x3.0#3", Less);
    test_pi_prec_round_helper(3, Ceiling, "3.5", "0x3.8#3", Greater);
    test_pi_prec_round_helper(3, Down, "3.0", "0x3.0#3", Less);
    test_pi_prec_round_helper(3, Up, "3.5", "0x3.8#3", Greater);
    test_pi_prec_round_helper(3, Nearest, "3.0", "0x3.0#3", Less);

    test_pi_prec_round_helper(4, Floor, "3.0", "0x3.0#4", Less);
    test_pi_prec_round_helper(4, Ceiling, "3.2", "0x3.4#4", Greater);
    test_pi_prec_round_helper(4, Down, "3.0", "0x3.0#4", Less);
    test_pi_prec_round_helper(4, Up, "3.2", "0x3.4#4", Greater);
    test_pi_prec_round_helper(4, Nearest, "3.2", "0x3.4#4", Greater);

    test_pi_prec_round_helper(5, Floor, "3.1", "0x3.2#5", Less);
    test_pi_prec_round_helper(5, Ceiling, "3.2", "0x3.4#5", Greater);
    test_pi_prec_round_helper(5, Down, "3.1", "0x3.2#5", Less);
    test_pi_prec_round_helper(5, Up, "3.2", "0x3.4#5", Greater);
    test_pi_prec_round_helper(5, Nearest, "3.1", "0x3.2#5", Less);

    test_pi_prec_round_helper(6, Floor, "3.12", "0x3.2#6", Less);
    test_pi_prec_round_helper(6, Ceiling, "3.19", "0x3.3#6", Greater);
    test_pi_prec_round_helper(6, Down, "3.12", "0x3.2#6", Less);
    test_pi_prec_round_helper(6, Up, "3.19", "0x3.3#6", Greater);
    test_pi_prec_round_helper(6, Nearest, "3.12", "0x3.2#6", Less);

    test_pi_prec_round_helper(7, Floor, "3.12", "0x3.20#7", Less);
    test_pi_prec_round_helper(7, Ceiling, "3.16", "0x3.28#7", Greater);
    test_pi_prec_round_helper(7, Down, "3.12", "0x3.20#7", Less);
    test_pi_prec_round_helper(7, Up, "3.16", "0x3.28#7", Greater);
    test_pi_prec_round_helper(7, Nearest, "3.16", "0x3.28#7", Greater);

    test_pi_prec_round_helper(8, Floor, "3.14", "0x3.24#8", Less);
    test_pi_prec_round_helper(8, Ceiling, "3.16", "0x3.28#8", Greater);
    test_pi_prec_round_helper(8, Down, "3.14", "0x3.24#8", Less);
    test_pi_prec_round_helper(8, Up, "3.16", "0x3.28#8", Greater);
    test_pi_prec_round_helper(8, Nearest, "3.14", "0x3.24#8", Less);

    test_pi_prec_round_helper(9, Floor, "3.14", "0x3.24#9", Less);
    test_pi_prec_round_helper(9, Ceiling, "3.15", "0x3.26#9", Greater);
    test_pi_prec_round_helper(9, Down, "3.14", "0x3.24#9", Less);
    test_pi_prec_round_helper(9, Up, "3.15", "0x3.26#9", Greater);
    test_pi_prec_round_helper(9, Nearest, "3.14", "0x3.24#9", Less);

    test_pi_prec_round_helper(10, Floor, "3.141", "0x3.24#10", Less);
    test_pi_prec_round_helper(10, Ceiling, "3.145", "0x3.25#10", Greater);
    test_pi_prec_round_helper(10, Down, "3.141", "0x3.24#10", Less);
    test_pi_prec_round_helper(10, Up, "3.145", "0x3.25#10", Greater);
    test_pi_prec_round_helper(10, Nearest, "3.141", "0x3.24#10", Less);

    test_pi_prec_round_helper(
        100,
        Floor,
        "3.141592653589793238462643383279",
        "0x3.243f6a8885a308d313198a2e0#100",
        Less,
    );
    test_pi_prec_round_helper(
        100,
        Ceiling,
        "3.141592653589793238462643383282",
        "0x3.243f6a8885a308d313198a2e4#100",
        Greater,
    );
    test_pi_prec_round_helper(
        100,
        Down,
        "3.141592653589793238462643383279",
        "0x3.243f6a8885a308d313198a2e0#100",
        Less,
    );
    test_pi_prec_round_helper(
        100,
        Up,
        "3.141592653589793238462643383282",
        "0x3.243f6a8885a308d313198a2e4#100",
        Greater,
    );
    test_pi_prec_round_helper(
        100,
        Nearest,
        "3.141592653589793238462643383279",
        "0x3.243f6a8885a308d313198a2e0#100",
        Less,
    );
}

#[test]
#[should_panic]
fn pi_prec_round_fail_1() {
    Float::pi_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn pi_prec_round_fail_2() {
    Float::pi_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn pi_prec_round_fail_3() {
    Float::pi_prec_round(1000, Exact);
}

#[test]
fn pi_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (pi, o) = Float::pi_prec(prec);
        assert!(pi.is_valid());
        assert_eq!(pi.get_prec(), Some(prec));
        assert_eq!(pi.get_exponent(), Some(if prec == 1 { 3 } else { 2 }));
        assert_ne!(o, Equal);
        if o == Less {
            let (pi_alt, o_alt) = Float::pi_prec_round(prec, Ceiling);
            let mut next_upper = pi.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(pi_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !pi.is_power_of_2() {
            let (pi_alt, o_alt) = Float::pi_prec_round(prec, Floor);
            let mut next_lower = pi.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(pi_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
        let (pi_alt, o_alt) = Float::pi_prec_round(prec, Nearest);
        assert_eq!(ComparableFloatRef(&pi_alt), ComparableFloatRef(&pi));
        assert_eq!(o_alt, o);

        let (rug_pi, rug_o) =
            rug_pi_prec_round(prec, rug_round_try_from_rounding_mode(Nearest).unwrap());
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_pi)),
            ComparableFloatRef(&pi)
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn pi_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (pi, o) = Float::pi_prec_round(prec, rm);
        assert!(pi.is_valid());
        assert_eq!(pi.get_prec(), Some(prec));
        let expected_exponent = match (prec, rm) {
            (1, Ceiling | Up | Nearest) | (2, Ceiling | Up) => 3,
            _ => 2,
        };
        assert_eq!(pi.get_exponent(), Some(expected_exponent));
        assert_ne!(o, Equal);
        if o == Less {
            let (pi_alt, o_alt) = Float::pi_prec_round(prec, Ceiling);
            let mut next_upper = pi.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(pi_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !pi.is_power_of_2() {
            let (pi_alt, o_alt) = Float::pi_prec_round(prec, Floor);
            let mut next_lower = pi.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(pi_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_pi, rug_o) = rug_pi_prec_round(prec, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_pi)),
                ComparableFloatRef(&pi)
            );
            assert_eq!(rug_o, o);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::pi_prec_round(prec, Exact));
    });

    test_constant(Float::pi_prec_round, 10000);
}
