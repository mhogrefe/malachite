// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::Phi;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{test_constant, to_hex_string};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_phi_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::phi_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);
}

#[test]
pub fn test_phi_prec() {
    test_phi_prec_helper(1, "2.0", "0x2.0#1", Greater);
    test_phi_prec_helper(2, "1.5", "0x1.8#2", Less);
    test_phi_prec_helper(3, "1.5", "0x1.8#3", Less);
    test_phi_prec_helper(4, "1.6", "0x1.a#4", Greater);
    test_phi_prec_helper(5, "1.62", "0x1.a#5", Greater);
    test_phi_prec_helper(6, "1.62", "0x1.a0#6", Greater);
    test_phi_prec_helper(7, "1.62", "0x1.a0#7", Greater);
    test_phi_prec_helper(8, "1.617", "0x1.9e#8", Less);
    test_phi_prec_helper(9, "1.617", "0x1.9e#9", Less);
    test_phi_prec_helper(10, "1.617", "0x1.9e0#10", Less);
    test_phi_prec_helper(
        100,
        "1.618033988749894848204586834366",
        "0x1.9e3779b97f4a7c15f39cc0606#100",
        Greater,
    );
    test_phi_prec_helper(
        1000,
        "1.618033988749894848204586834365638117720309179805762862135448622705260462818902449707207\
        204189391137484754088075386891752126633862223536931793180060766726354433389086595939582905\
        638322661319928290267880675208766892501711696207032221043216269548626296313614438149758701\
        2203408058879544547492461856953648",
        "0x1.9e3779b97f4a7c15f39cc0605cedc8341082276bf3a27251f86c6a11d0c18e952767f0b153d27b7f03470\
        45b5bf1827f01886f0928403002c1d64ba40f335e36f06ad7ae9717877e85839d6effbd7dc664d325d1c537168\
        2cadd0cccfdffbbe1626e33b8d04b4331bbf73c790d94f79d471c4ab3ed3d82a5fec507705e#1000",
        Less,
    );
    test_phi_prec_helper(
        10000,
        "1.618033988749894848204586834365638117720309179805762862135448622705260462818902449707207\
        204189391137484754088075386891752126633862223536931793180060766726354433389086595939582905\
        638322661319928290267880675208766892501711696207032221043216269548626296313614438149758701\
        220340805887954454749246185695364864449241044320771344947049565846788509874339442212544877\
        066478091588460749988712400765217057517978834166256249407589069704000281210427621771117778\
        053153171410117046665991466979873176135600670874807101317952368942752194843530567830022878\
        569978297783478458782289110976250030269615617002504643382437764861028383126833037242926752\
        631165339247316711121158818638513316203840052221657912866752946549068113171599343235973494\
        985090409476213222981017261070596116456299098162905552085247903524060201727997471753427775\
        927786256194320827505131218156285512224809394712341451702237358057727861600868838295230459\
        264787801788992199027077690389532196819861514378031499741106926088674296226757560523172777\
        520353613936210767389376455606060592165894667595519004005559089502295309423124823552122124\
        154440064703405657347976639723949499465845788730396230903750339938562102423690251386804145\
        779956981224457471780341731264532204163972321340444494873023154176768937521030687378803441\
        700939544096279558986787232095124268935573097045095956844017555198819218020640529055189349\
        475926007348522821010881946445442223188913192946896220023014437702699230078030852611807545\
        192887705021096842493627135925187607778846658361502389134933331223105339232136243192637289\
        106705033992822652635562090297986424727597725655086154875435748264718141451270006023890162\
        077732244994353088999095016803281121943204819643876758633147985719113978153978074761507722\
        117508269458639320456520989698555678141069683728840587461033781054443909436835835813811311\
        689938555769754841491445341509129540700501947754861630754226417293946803673198058618339183\
        285991303960720144559504497792120761247856459161608370594987860069701894098864007644361709\
        334172709191433650137157660114803814306262380514321173481510055901345610118007905063814215\
        270930858809287570345050780814545881990633612982798141174533927312080928972792221329806429\
        468782427487401745055406778757083237310975915117762978443284747908176518097787268416117632\
        503861211291436834376702350371116330725869883258710336322238109809012110198991768414917512\
        331340152733843837234500934786049792945991582201258104598230925528721241370436149102054718\
        554961180876426576511060545881475604431784798584539731286301625448761148520217064404111660\
        766950597757832570395110878230827106478939021115691039276838453863333215658296597731034360\
        323225457436372041244064088826737584339536795931232213437320995749889469956564736007295999\
        839128810319742631251797141432012311279551894778172691415891177991956481255800184550656329\
        528598591000908621802977563789259991649946428193022293552346674759326951654214021091363018\
        194722707890122087287361707348649998156255472811373479871656952748900814438405327483781378\
        2466917444229634914708157007352545707089773",
        "0x1.9e3779b97f4a7c15f39cc0605cedc8341082276bf3a27251f86c6a11d0c18e952767f0b153d27b7f03470\
        45b5bf1827f01886f0928403002c1d64ba40f335e36f06ad7ae9717877e85839d6effbd7dc664d325d1c537168\
        2cadd0cccfdffbbe1626e33b8d04b4331bbf73c790d94f79d471c4ab3ed3d82a5fec507705e4ae6e5e73a9b91f\
        3aa4db287ae44f332e923a73cb91648e428e975a3781eb01b49d8674fa1508419e0eaa4038b352d9bad30f4485\
        b71a8ef64452a0dd40dc8cb8f9a2d4c514f1b229dcaa222ac268e9666e4a866769145f5f5880a9d0acd3b9e8c6\
        82f4f810320abeb94034e70f21608c061ab1c1caef1ebdcefbc72134ecf06ed82bfb7d8eb1a41901d65f5c8cab\
        2accbc32eab1fbe8284f2b44ba2e834c5893a39ea7865443f489c37f8742acd895afd87b467d22a40d098f30dd\
        2cafdeb3abb3a13507b46b3d757fc04001906e1767d40c3a3792a26eeef2ab5bd6685b915b5629400faa684ecb\
        a752dddcb5d18576d77b652ac0d999973686604128f0cd42743596deb2d42c789d64b92658610b5b95c719adeb\
        8ce287326344e31d59a59963220b1a4c42771d454ec78f12393bfb4ac54188e59113d7c35441f15aa34a114070\
        35f006fc3ad70fc0d6331c6d479f484bf39cbfbd95e664e39bebdc5b09d9d5b8f8905d9805c8199457d444a909\
        316cc58d38b3a25c714be8d422964e9724c033b6748acbc2e05caaf737c95622e6483fa0707999afc482995170\
        1c6ed5f27ce231770965541f5f28797f2fecbd984d3435ce547c88c38466e2865235683d4447e4b32757903058\
        ca11a903f3baa3864932b80a7d02d61b50c1ff281d5dd2947da4624bbc7c7b94962717aed284395c42f253c17f\
        492f401bb6c17a95c3296b424db05afa8e7ad2561aca1f4fd6e0eb57f831e40227fbacd467ff8b7f84370aef71\
        1857634d0cf4189002dde8787576c631f0b9a173c16b80e7d941c128a540c91d33a1daf0f0222986e3c7661b2a\
        7b374dc06d8396659e0fde9b7b41dc1bbb0f42988d4d2525906bdf64a6ea5b159f2adf7c883ce8c4808a9b46eb\
        960f74192010862bd1c306500c6795a0a29df4a34d42b1a418a9fbcabd51e1887aa0f8ebdf4452d6efb8d2948d\
        8878256e2c6c33dc8ca89d68e06aeabd56bca17dea2358e443b567201dbe25e06a7451a736b14cfe3af4385221\
        86524dad69f054b61cdbf1ba9a54f46ff7a2a1e5090e0a26dbe7766ebf40475a2f166ab6b1791b0ee0b48d863c\
        08f809967ac68a158a6b4eefe4fae8eb1d32cfa6a4573468c137401262ab7893a30cb015245b76fa16cf89db93\
        647bb74e2581ae8823d14c0343efb790b35764216428d6e9856dd074ee6f411ee1b5dd2a670cce122bf868be81\
        8525e08f5b8b3d940252b853b7248a83aa561610f17a35fc83e542a94fcb71694c229ef98f50746a08c3fcd875\
        ca4b94d390bba41e657f21892b6ee0ef705034accc46fc4e1af7e9a90ff984fc20bd9596a3da5beca81a0ac599\
        947c1f5eebf18b16c96a12face93501968c59c16f7cd92e8a58d472335d816bbb85e1caa23ed2a61cc0cdbbb37\
        00b8bb3f9669fc3590347bc63f4c5dcb9b2a8c174acbadbdef7dd61aa4dff59fb3adca34b37fd54fcdbdeb6824\
        16a89edc65754aadaa3d5d5f329b92a98a8f08c8d6b6197dec9b63286673d2123a78adb6e162b938081682e154\
        56856ea498bee3ac11a03804078c8426b1eeba9afe94a602b0aca31dcc753b038d74573c896#10000",
        Greater,
    );

    let phi_f32 = Float::phi_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(phi_f32.to_string(), "1.618034");
    assert_eq!(to_hex_string(&phi_f32), "0x1.9e377a#24");
    assert_eq!(phi_f32, f32::PHI);

    let phi_f64 = Float::phi_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(phi_f64.to_string(), "1.6180339887498949");
    assert_eq!(to_hex_string(&phi_f64), "0x1.9e3779b97f4a8#53");
    assert_eq!(phi_f64, f64::PHI);
}

fn test_phi_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::phi_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);
}

#[test]
pub fn test_phi_prec_round() {
    test_phi_prec_round_helper(1, Floor, "1.0", "0x1.0#1", Less);
    test_phi_prec_round_helper(1, Ceiling, "2.0", "0x2.0#1", Greater);
    test_phi_prec_round_helper(1, Down, "1.0", "0x1.0#1", Less);
    test_phi_prec_round_helper(1, Up, "2.0", "0x2.0#1", Greater);
    test_phi_prec_round_helper(1, Nearest, "2.0", "0x2.0#1", Greater);

    test_phi_prec_round_helper(2, Floor, "1.5", "0x1.8#2", Less);
    test_phi_prec_round_helper(2, Ceiling, "2.0", "0x2.0#2", Greater);
    test_phi_prec_round_helper(2, Down, "1.5", "0x1.8#2", Less);
    test_phi_prec_round_helper(2, Up, "2.0", "0x2.0#2", Greater);
    test_phi_prec_round_helper(2, Nearest, "1.5", "0x1.8#2", Less);

    test_phi_prec_round_helper(3, Floor, "1.5", "0x1.8#3", Less);
    test_phi_prec_round_helper(3, Ceiling, "1.8", "0x1.c#3", Greater);
    test_phi_prec_round_helper(3, Down, "1.5", "0x1.8#3", Less);
    test_phi_prec_round_helper(3, Up, "1.8", "0x1.c#3", Greater);
    test_phi_prec_round_helper(3, Nearest, "1.5", "0x1.8#3", Less);

    test_phi_prec_round_helper(4, Floor, "1.5", "0x1.8#4", Less);
    test_phi_prec_round_helper(4, Ceiling, "1.6", "0x1.a#4", Greater);
    test_phi_prec_round_helper(4, Down, "1.5", "0x1.8#4", Less);
    test_phi_prec_round_helper(4, Up, "1.6", "0x1.a#4", Greater);
    test_phi_prec_round_helper(4, Nearest, "1.6", "0x1.a#4", Greater);

    test_phi_prec_round_helper(5, Floor, "1.56", "0x1.9#5", Less);
    test_phi_prec_round_helper(5, Ceiling, "1.62", "0x1.a#5", Greater);
    test_phi_prec_round_helper(5, Down, "1.56", "0x1.9#5", Less);
    test_phi_prec_round_helper(5, Up, "1.62", "0x1.a#5", Greater);
    test_phi_prec_round_helper(5, Nearest, "1.62", "0x1.a#5", Greater);

    test_phi_prec_round_helper(6, Floor, "1.59", "0x1.98#6", Less);
    test_phi_prec_round_helper(6, Ceiling, "1.62", "0x1.a0#6", Greater);
    test_phi_prec_round_helper(6, Down, "1.59", "0x1.98#6", Less);
    test_phi_prec_round_helper(6, Up, "1.62", "0x1.a0#6", Greater);
    test_phi_prec_round_helper(6, Nearest, "1.62", "0x1.a0#6", Greater);

    test_phi_prec_round_helper(7, Floor, "1.61", "0x1.9c#7", Less);
    test_phi_prec_round_helper(7, Ceiling, "1.62", "0x1.a0#7", Greater);
    test_phi_prec_round_helper(7, Down, "1.61", "0x1.9c#7", Less);
    test_phi_prec_round_helper(7, Up, "1.62", "0x1.a0#7", Greater);
    test_phi_prec_round_helper(7, Nearest, "1.62", "0x1.a0#7", Greater);

    test_phi_prec_round_helper(8, Floor, "1.617", "0x1.9e#8", Less);
    test_phi_prec_round_helper(8, Ceiling, "1.625", "0x1.a0#8", Greater);
    test_phi_prec_round_helper(8, Down, "1.617", "0x1.9e#8", Less);
    test_phi_prec_round_helper(8, Up, "1.625", "0x1.a0#8", Greater);
    test_phi_prec_round_helper(8, Nearest, "1.617", "0x1.9e#8", Less);

    test_phi_prec_round_helper(9, Floor, "1.617", "0x1.9e#9", Less);
    test_phi_prec_round_helper(9, Ceiling, "1.621", "0x1.9f#9", Greater);
    test_phi_prec_round_helper(9, Down, "1.617", "0x1.9e#9", Less);
    test_phi_prec_round_helper(9, Up, "1.621", "0x1.9f#9", Greater);
    test_phi_prec_round_helper(9, Nearest, "1.617", "0x1.9e#9", Less);

    test_phi_prec_round_helper(10, Floor, "1.617", "0x1.9e0#10", Less);
    test_phi_prec_round_helper(10, Ceiling, "1.619", "0x1.9e8#10", Greater);
    test_phi_prec_round_helper(10, Down, "1.617", "0x1.9e0#10", Less);
    test_phi_prec_round_helper(10, Up, "1.619", "0x1.9e8#10", Greater);
    test_phi_prec_round_helper(10, Nearest, "1.617", "0x1.9e0#10", Less);

    test_phi_prec_round_helper(
        100,
        Floor,
        "1.618033988749894848204586834364",
        "0x1.9e3779b97f4a7c15f39cc0604#100",
        Less,
    );
    test_phi_prec_round_helper(
        100,
        Ceiling,
        "1.618033988749894848204586834366",
        "0x1.9e3779b97f4a7c15f39cc0606#100",
        Greater,
    );
    test_phi_prec_round_helper(
        100,
        Down,
        "1.618033988749894848204586834364",
        "0x1.9e3779b97f4a7c15f39cc0604#100",
        Less,
    );
    test_phi_prec_round_helper(
        100,
        Up,
        "1.618033988749894848204586834366",
        "0x1.9e3779b97f4a7c15f39cc0606#100",
        Greater,
    );
    test_phi_prec_round_helper(
        100,
        Nearest,
        "1.618033988749894848204586834366",
        "0x1.9e3779b97f4a7c15f39cc0606#100",
        Greater,
    );
}

#[test]
#[should_panic]
fn phi_prec_round_fail_1() {
    Float::phi_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn phi_prec_round_fail_2() {
    Float::phi_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn phi_prec_round_fail_3() {
    Float::phi_prec_round(1000, Exact);
}

#[test]
fn phi_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (phi, o) = Float::phi_prec(prec);
        assert!(phi.is_valid());
        assert_eq!(phi.get_prec(), Some(prec));
        assert_eq!(phi.get_exponent(), Some(if prec == 1 { 2 } else { 1 }));
        assert_ne!(o, Equal);
        if o == Less {
            let (phi_alt, o_alt) = Float::phi_prec_round(prec, Ceiling);
            let mut next_upper = phi.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(phi_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !phi.is_power_of_2() {
            let (phi_alt, o_alt) = Float::phi_prec_round(prec, Floor);
            let mut next_lower = phi.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(phi_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
        let (phi_alt, o_alt) = Float::phi_prec_round(prec, Nearest);
        assert_eq!(ComparableFloatRef(&phi_alt), ComparableFloatRef(&phi));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn phi_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (phi, o) = Float::phi_prec_round(prec, rm);
        assert!(phi.is_valid());
        assert_eq!(phi.get_prec(), Some(prec));
        let expected_exponent = match (prec, rm) {
            (1, Ceiling | Up | Nearest) | (2, Ceiling | Up) => 2,
            _ => 1,
        };
        assert_eq!(phi.get_exponent(), Some(expected_exponent));
        assert_ne!(o, Equal);
        if o == Less {
            let (phi_alt, o_alt) = Float::phi_prec_round(prec, Ceiling);
            let mut next_upper = phi.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(phi_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !phi.is_power_of_2() {
            let (phi_alt, o_alt) = Float::phi_prec_round(prec, Floor);
            let mut next_lower = phi.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(phi_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::phi_prec_round(prec, Exact));
    });

    test_constant(Float::phi_prec_round, 10000);
}
