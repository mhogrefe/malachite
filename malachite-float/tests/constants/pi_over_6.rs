// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::PiOver6;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{test_constant, to_hex_string};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_pi_over_6_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::pi_over_6_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);
}

#[test]
pub fn test_pi_over_6_prec() {
    test_pi_over_6_prec_helper(1, "0.5", "0x0.8#1", Less);
    test_pi_over_6_prec_helper(2, "0.5", "0x0.8#2", Less);
    test_pi_over_6_prec_helper(3, "0.5", "0x0.8#3", Less);
    test_pi_over_6_prec_helper(4, "0.5", "0x0.8#4", Less);
    test_pi_over_6_prec_helper(5, "0.53", "0x0.88#5", Greater);
    test_pi_over_6_prec_helper(6, "0.53", "0x0.88#6", Greater);
    test_pi_over_6_prec_helper(7, "0.523", "0x0.86#7", Less);
    test_pi_over_6_prec_helper(8, "0.523", "0x0.86#8", Less);
    test_pi_over_6_prec_helper(9, "0.523", "0x0.860#9", Less);
    test_pi_over_6_prec_helper(10, "0.523", "0x0.860#10", Less);
    test_pi_over_6_prec_helper(
        100,
        "0.523598775598298873077107230547",
        "0x0.860a91c16b9b2c232dd99707b#100",
        Greater,
    );
    test_pi_over_6_prec_helper(
        1000,
        "0.523598775598298873077107230546583814032861566562517636829157432051302734381034833104672\
        470890352844663691347752213717774515640768258430371954226568021413519575047350450323086850\
        926607437158159155063660738135162610988907688079274705631130527545200318190941427820576724\
        7684090544413688989345433748568789",
        "0x0.860a91c16b9b2c232dd99707ab3d688b70ac3405b19a884d56b27f197cb7bcc18b86b0510978033e9fb8b\
        bcd337c2cbccac75c494c3f62cf8a96239e48e12c2e985923a441945484a2dd81f1197a9e475d54e879f8047a9\
        e9ed047fce7066a6e746a180ba832154430c5998bf342e77e8155a87b16427f591091857968#1000",
        Less,
    );
    test_pi_over_6_prec_helper(
        10000,
        "0.523598775598298873077107230546583814032861566562517636829157432051302734381034833104672\
        470890352844663691347752213717774515640768258430371954226568021413519575047350450323086850\
        926607437158159155063660738135162610988907688079274705631130527545200318190941427820576724\
        768409054441368898934543374856878954097834434385931362480253486827138209015285894061315431\
        726668555088424803411086897357825323585268238842878394293265325515364352896988768632184186\
        424679077063327124927891980958787481871323030501991521638945560406776107181003565824399204\
        122865117029976823950461756536196048862792064112469744611567522000094687857543926804630961\
        890459631601528939529786911406816870825571691091597561841798713281543153923669992685354836\
        993477339069693302271629128551660086451201891666666395496341658509955288802682719765837409\
        909224484717107087051375557447505876988531361835000522973125481097922201396903436196278191\
        217266375581738125911455193260477313725631265625326296976296755361871134435500321312768532\
        651536070033156349209533510914310546477656025563637994705050325339216975494832628937099902\
        315208286962547246521919262476207075691782658471588852810287975981512516396959106244156553\
        209176733487950278523168308137335476393600593951276684118363657159266033157794639574908042\
        563295787807850674589107701341114043178248548885612838164858684125360342827670676339691698\
        918542230405005931273374582745543985699878767378320466130392463606001556953606869987431052\
        505047697162425951124973084249098097821165948487868466251550492553527557497867125993372746\
        777583186646972466292261061634571090421310425303069595778815162962132300013607843336024208\
        198695536202462058356906995594758026893526225425355579290308244739753887317899023889091293\
        736143753163928247593683203203697378758375709481278631749100275577800831437872131964347630\
        730637994662780242350158980631060158446677370854200852898830816014021414771157600706994214\
        170368443531051124046437006532490841187285631160159394061986214577960776262327068981810972\
        107665968898413004598349910960679825211578066392099284970970436753748234621119913044747100\
        246165150440022732406242550844700582708741958233275238571634865109875156203616076919284976\
        397901766314326628829249821693625654744689711447311490462359331975987540992326571841662087\
        446807664545607449308108972789370437683187434675207314065075207356091627130132952615239332\
        950021602681573615811425974734392237012037638081441359742671417669473789908711244613149208\
        689753758325777787970664409432686059147705096242749672598939094695720685419179344913241849\
        432682337538146618482190944856144538124823426683583884769654780153479126819637489764834952\
        484945997543560924963648854964136947166491453776467476260669045079592522063273575253957705\
        727423809740799210977970175190225789289920518904527683689328256038573825414156197851690960\
        900598379989067290334551763089843699731241301413081613886907618978125323917738369742198508\
        080167561769113445819879698532989992010236610571459073440624187286365363333065169319926969\
        11252378187329149015119774903866026132420136",
        "0x0.860a91c16b9b2c232dd99707ab3d688b70ac3405b19a884d56b27f197cb7bcc18b86b0510978033e9fb8b\
        bcd337c2cbccac75c494c3f62cf8a96239e48e12c2e985923a441945484a2dd81f1197a9e475d54e879f8047a9\
        e9ed047fce7066a6e746a180ba832154430c5998bf342e77e8155a87b16427f5910918579683937bc460ed51b5\
        36ddf950243936d3dc273b96841f78ec058e1d269e370afa0646448ef5d78dedc7dbaadf64d9d5b31656ba821b\
        5942ec979ded297befa1d655f0402676fad174805170a792e3ea04a32e1dbe97287f9b8e564bad10e30fdf1b8f\
        1ee0e8c1965bb5158b563a1b43c5c71d81e737764b35835a6cd1ae36bc7ea1326edf35258ad90929f5c5c9c4b8\
        f93595da92264b503c49698851d4e745de75b77e4bf086340316e4113df428c196737499d4b755159e65c05989\
        04eac4cd4859c42e16a1cbaba52155dd2960f8fa6eb939da4b105d5d1e62f415b418a6af8991ccb82923d5340a\
        8b5b432573615c616055611a17d61c505448fb04bbc0b2926e76f112cc4baf1f896d2bc62f023240e07e6c3ad4\
        686c7383345e7d281e758945f50c9b4a80d6a7f31c41afd90da3447d593bbcc643515b5d7c16cd26baee0e7eff\
        36a40c64b34973a752567e8f96ba030133559b61a8e7591c662789c65b3b53e8104aa7a930b19d5b4dea2ce862\
        2ac586179d7fc788c52f56f2be33dcc40198494864e144ed3d17a7ea5ffb80473bee209ee922574d620011f204\
        6e29e601a9020ba64c5205991b0bb3c29e7f28a676348d8899dd1df387cbd3cff62da99dd6e3222158b8b728fc\
        a2c4a7b4d5625088c7f910286abd987b748a021f163e0f6be02a301e53449a354a48e3ef574d9bab301ce702ac\
        bb55cbdbee69a865529da9b4cc270fc5424885b295f68d92554e591d8dd380521722b8dc7330a48ff2a3096900\
        ddd948c0a557a95c50f63f4a1bd421aaf168e5d252e91a4a38b1c2906bfd54b476688766cb7e3933c39dbf2c9f\
        70ecc25aa9b3a497d58455829b4eed4daeb4961d4c8e75cf84de499b8a614492960e64dc2619f61ff43ed7afa5\
        32470cc9508907f2de04d262126d28577245e693755a2bfc52a5ca82ff36e4f4f0cf26d6d997c7242683c98a39\
        acd6d22f81d4bdba56d0a0f3297be4373ed53d07f5303073b6c16c9586d57a804b7f23a70116d4b2f784d52ed9\
        df1759cdd3285aeca57ae93fc68dcffc1d5aff050f022029e3d3e26aec77df9226c506ef37162a57f83501e94c\
        b5f404dc79c242e6545bbc018e92b98a2132e058458bea6723010e948ff16768e0b28ff44de4d51f79d2cbf351\
        6f023b584a347afbfb6d55b825cdabd9de505bc001f39497ac7db660919fd56f42fb8e5ed84e44ff11c69758b4\
        1a439b8953014e49b007b9ef6408655e9106e4d46d84c043c0d13370af33687a1bf2e1c37048d800dc4633183a\
        3ad39354a19f67b5c901b6915a668aaac1f129e392f51855319276731acf9839e65e6305149a99e7fde00dd20e\
        b8e660df2b23a46efed45817461443d162be70c946826f39ddc3624938187cb59865b4792711d619bfa94ef7d1\
        01ec2a79dbbd21c6cbd9199aeaa8e660dbccb1d9a7ad9890070e8d6e206f00ed88ad0aeb45fb538c419b9e06f9\
        0e742a623c453dff91af86f56a7fc0828e23234267d5b8fa0377a30166b6827311095d1a1005a7765019167353\
        51f27f6df6e8403673c681ae6bfde40b87025c11e9a0dd6470d5689350140af6aa51d0f6d15#10000",
        Greater,
    );

    let pi_over_6_f32 = Float::pi_over_6_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(pi_over_6_f32.to_string(), "0.52359879");
    assert_eq!(to_hex_string(&pi_over_6_f32), "0x0.860a92#24");
    assert_eq!(pi_over_6_f32, f32::PI_OVER_6);

    let pi_over_6_f64 = Float::pi_over_6_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(pi_over_6_f64.to_string(), "0.5235987755982989");
    assert_eq!(to_hex_string(&pi_over_6_f64), "0x0.860a91c16b9b30#53");
    assert_eq!(pi_over_6_f64, f64::PI_OVER_6);
}

#[test]
#[should_panic]
fn pi_over_6_prec_fail_1() {
    Float::pi_over_6_prec(0);
}

fn test_pi_over_6_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::pi_over_6_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);
}

#[test]
pub fn test_pi_over_6_prec_round() {
    test_pi_over_6_prec_round_helper(1, Floor, "0.5", "0x0.8#1", Less);
    test_pi_over_6_prec_round_helper(1, Ceiling, "1.0", "0x1.0#1", Greater);
    test_pi_over_6_prec_round_helper(1, Down, "0.5", "0x0.8#1", Less);
    test_pi_over_6_prec_round_helper(1, Up, "1.0", "0x1.0#1", Greater);
    test_pi_over_6_prec_round_helper(1, Nearest, "0.5", "0x0.8#1", Less);

    test_pi_over_6_prec_round_helper(2, Floor, "0.5", "0x0.8#2", Less);
    test_pi_over_6_prec_round_helper(2, Ceiling, "0.8", "0x0.c#2", Greater);
    test_pi_over_6_prec_round_helper(2, Down, "0.5", "0x0.8#2", Less);
    test_pi_over_6_prec_round_helper(2, Up, "0.8", "0x0.c#2", Greater);
    test_pi_over_6_prec_round_helper(2, Nearest, "0.5", "0x0.8#2", Less);

    test_pi_over_6_prec_round_helper(3, Floor, "0.5", "0x0.8#3", Less);
    test_pi_over_6_prec_round_helper(3, Ceiling, "0.6", "0x0.a#3", Greater);
    test_pi_over_6_prec_round_helper(3, Down, "0.5", "0x0.8#3", Less);
    test_pi_over_6_prec_round_helper(3, Up, "0.6", "0x0.a#3", Greater);
    test_pi_over_6_prec_round_helper(3, Nearest, "0.5", "0x0.8#3", Less);

    test_pi_over_6_prec_round_helper(4, Floor, "0.5", "0x0.8#4", Less);
    test_pi_over_6_prec_round_helper(4, Ceiling, "0.56", "0x0.9#4", Greater);
    test_pi_over_6_prec_round_helper(4, Down, "0.5", "0x0.8#4", Less);
    test_pi_over_6_prec_round_helper(4, Up, "0.56", "0x0.9#4", Greater);
    test_pi_over_6_prec_round_helper(4, Nearest, "0.5", "0x0.8#4", Less);

    test_pi_over_6_prec_round_helper(5, Floor, "0.5", "0x0.80#5", Less);
    test_pi_over_6_prec_round_helper(5, Ceiling, "0.53", "0x0.88#5", Greater);
    test_pi_over_6_prec_round_helper(5, Down, "0.5", "0x0.80#5", Less);
    test_pi_over_6_prec_round_helper(5, Up, "0.53", "0x0.88#5", Greater);
    test_pi_over_6_prec_round_helper(5, Nearest, "0.53", "0x0.88#5", Greater);

    test_pi_over_6_prec_round_helper(6, Floor, "0.52", "0x0.84#6", Less);
    test_pi_over_6_prec_round_helper(6, Ceiling, "0.53", "0x0.88#6", Greater);
    test_pi_over_6_prec_round_helper(6, Down, "0.52", "0x0.84#6", Less);
    test_pi_over_6_prec_round_helper(6, Up, "0.53", "0x0.88#6", Greater);
    test_pi_over_6_prec_round_helper(6, Nearest, "0.53", "0x0.88#6", Greater);

    test_pi_over_6_prec_round_helper(7, Floor, "0.523", "0x0.86#7", Less);
    test_pi_over_6_prec_round_helper(7, Ceiling, "0.53", "0x0.88#7", Greater);
    test_pi_over_6_prec_round_helper(7, Down, "0.523", "0x0.86#7", Less);
    test_pi_over_6_prec_round_helper(7, Up, "0.53", "0x0.88#7", Greater);
    test_pi_over_6_prec_round_helper(7, Nearest, "0.523", "0x0.86#7", Less);

    test_pi_over_6_prec_round_helper(8, Floor, "0.523", "0x0.86#8", Less);
    test_pi_over_6_prec_round_helper(8, Ceiling, "0.527", "0x0.87#8", Greater);
    test_pi_over_6_prec_round_helper(8, Down, "0.523", "0x0.86#8", Less);
    test_pi_over_6_prec_round_helper(8, Up, "0.527", "0x0.87#8", Greater);
    test_pi_over_6_prec_round_helper(8, Nearest, "0.523", "0x0.86#8", Less);

    test_pi_over_6_prec_round_helper(9, Floor, "0.523", "0x0.860#9", Less);
    test_pi_over_6_prec_round_helper(9, Ceiling, "0.525", "0x0.868#9", Greater);
    test_pi_over_6_prec_round_helper(9, Down, "0.523", "0x0.860#9", Less);
    test_pi_over_6_prec_round_helper(9, Up, "0.525", "0x0.868#9", Greater);
    test_pi_over_6_prec_round_helper(9, Nearest, "0.523", "0x0.860#9", Less);

    test_pi_over_6_prec_round_helper(10, Floor, "0.523", "0x0.860#10", Less);
    test_pi_over_6_prec_round_helper(10, Ceiling, "0.524", "0x0.864#10", Greater);
    test_pi_over_6_prec_round_helper(10, Down, "0.523", "0x0.860#10", Less);
    test_pi_over_6_prec_round_helper(10, Up, "0.524", "0x0.864#10", Greater);
    test_pi_over_6_prec_round_helper(10, Nearest, "0.523", "0x0.860#10", Less);

    test_pi_over_6_prec_round_helper(
        100,
        Floor,
        "0.523598775598298873077107230546",
        "0x0.860a91c16b9b2c232dd99707a#100",
        Less,
    );
    test_pi_over_6_prec_round_helper(
        100,
        Ceiling,
        "0.523598775598298873077107230547",
        "0x0.860a91c16b9b2c232dd99707b#100",
        Greater,
    );
    test_pi_over_6_prec_round_helper(
        100,
        Down,
        "0.523598775598298873077107230546",
        "0x0.860a91c16b9b2c232dd99707a#100",
        Less,
    );
    test_pi_over_6_prec_round_helper(
        100,
        Up,
        "0.523598775598298873077107230547",
        "0x0.860a91c16b9b2c232dd99707b#100",
        Greater,
    );
    test_pi_over_6_prec_round_helper(
        100,
        Nearest,
        "0.523598775598298873077107230547",
        "0x0.860a91c16b9b2c232dd99707b#100",
        Greater,
    );
}

#[test]
#[should_panic]
fn pi_over_6_prec_round_fail_1() {
    Float::pi_over_6_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn pi_over_6_prec_round_fail_2() {
    Float::pi_over_6_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn pi_over_6_prec_round_fail_3() {
    Float::pi_over_6_prec_round(1000, Exact);
}

#[test]
fn pi_over_6_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (pi_over_6, o) = Float::pi_over_6_prec(prec);
        assert!(pi_over_6.is_valid());
        assert_eq!(pi_over_6.get_prec(), Some(prec));
        assert_eq!(pi_over_6.get_exponent(), Some(0));
        assert_ne!(o, Equal);
        if o == Less {
            let (pi_over_6_alt, o_alt) = Float::pi_over_6_prec_round(prec, Ceiling);
            let mut next_upper = pi_over_6.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(pi_over_6_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !pi_over_6.is_power_of_2() {
            let (pi_over_6_alt, o_alt) = Float::pi_over_6_prec_round(prec, Floor);
            let mut next_lower = pi_over_6.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(pi_over_6_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
        let (pi_over_6_alt, o_alt) = Float::pi_over_6_prec_round(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&pi_over_6_alt),
            ComparableFloatRef(&pi_over_6)
        );
        assert_eq!(o_alt, o);
    });
}

#[test]
fn pi_over_6_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (pi_over_6, o) = Float::pi_over_6_prec_round(prec, rm);
        assert!(pi_over_6.is_valid());
        assert_eq!(pi_over_6.get_prec(), Some(prec));
        assert_eq!(
            pi_over_6.get_exponent(),
            Some(if prec == 1 && (rm == Ceiling || rm == Up) {
                1
            } else {
                0
            })
        );
        assert_ne!(o, Equal);
        if o == Less {
            let (pi_over_6_alt, o_alt) = Float::pi_over_6_prec_round(prec, Ceiling);
            let mut next_upper = pi_over_6.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(pi_over_6_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !pi_over_6.is_power_of_2() {
            let (pi_over_6_alt, o_alt) = Float::pi_over_6_prec_round(prec, Floor);
            let mut next_lower = pi_over_6.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(pi_over_6_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::pi_over_6_prec_round(prec, Exact));
    });

    test_constant(Float::pi_over_6_prec_round, 10000);
}
