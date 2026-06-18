// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::Log102;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{
    rug_round_try_from_rounding_mode, test_constant, to_hex_string,
};
use malachite_float::test_util::constants::log_10_2::rug_log_10_2_prec_round;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_log_10_2_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::log_10_2_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = Float::log_base_10_prec(Float::from(2), prec);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);

    let (rug_x, rug_o) =
        rug_log_10_2_prec_round(prec, rug_round_try_from_rounding_mode(Nearest).unwrap());
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_x)),
        ComparableFloatRef(&x)
    );
    assert_eq!(rug_o, o);
}

#[test]
pub fn test_log_10_2_prec() {
    test_log_10_2_prec_helper(1, "0.2", "0x0.4#1", Less);
    test_log_10_2_prec_helper(2, "0.2", "0x0.4#2", Less);
    test_log_10_2_prec_helper(3, "0.3", "0x0.5#3", Greater);
    test_log_10_2_prec_helper(4, "0.31", "0x0.50#4", Greater);
    test_log_10_2_prec_helper(5, "0.3", "0x0.4c#5", Less);
    test_log_10_2_prec_helper(6, "0.305", "0x0.4e#6", Greater);
    test_log_10_2_prec_helper(7, "0.301", "0x0.4d#7", Less);
    test_log_10_2_prec_helper(8, "0.301", "0x0.4d0#8", Less);
    test_log_10_2_prec_helper(9, "0.301", "0x0.4d0#9", Less);
    test_log_10_2_prec_helper(10, "0.3013", "0x0.4d2#10", Greater);
    test_log_10_2_prec_helper(
        100,
        "0.3010299956639811952137388947246",
        "0x0.4d104d427de7fbcc47c4acd608#100",
        Greater,
    );
    test_log_10_2_prec_helper(
        1000,
        "0.301029995663981195213738894724493026768189881462108541310427461127108189274424509486927\
        252118186172040684477191430995379094767881133523505999692333704695575064502964254193402661\
        819734311602943501183902898178582617154439531861929046353884699520239310849612462540400263\
        31259462147884584731828267268398233",
        "0x0.4d104d427de7fbcc47c4acd605be48bc13569862a1e8f9a4c52f37935be631e5943516c0c8cfd5e84f2e5\
        e399a38de8948a39a4ad8c5c90f2c5a93fa92a969662fc1bef7012aae5e4e78e8c862030172e9361397ef38817\
        a75c8894d8ac96cee0246bf52cf58a9ec058419e2ca0d5c10b51b3dc09e7a647def7518bbe48#1000",
        Greater,
    );
    test_log_10_2_prec_helper(
        10000,
        "0.301029995663981195213738894724493026768189881462108541310427461127108189274424509486927\
        252118186172040684477191430995379094767881133523505999692333704695575064502964254193402661\
        819734311602943501183902898178582617154439531861929046353884699520239310849612462540400263\
        312594621478845847318282672683982326196542793507631317548350927138964946917785768918050790\
        007599548087815459714585031964877626122492290829118190951498997171619860477676500067820517\
        912557328628668342000402920509837084572224895494297562149707244659708613689609221909482761\
        214391496528235167826492314804027746243244163311538738259303883039380633216130239051880582\
        131915685461692905301505131926985378488418718320065753569468392971742132010905896890850585\
        624640987218396876648539856235161277302638927878260849836681030308431415560813943617674548\
        856663424538123733932422469594349060212044504296827460688478546115684768410643797950046596\
        991774565754086401846407945652954434107740829399974540073721701680194889055485691069400375\
        411689963415759297218064430381028152033923880856331986854539873935485606578428968489826139\
        442608466327829526028766212762304341922026289121120836126005583686254899999092794878431974\
        744338886862911771315741314322282416907299585472526615701683786532484377248450149423107098\
        105754764423911116694691455465315821308754571485915526406466945939738727466262648155637313\
        532726933795969680246236373580370170278652787138236826674951982888462336755746230644779336\
        477698037147068313325888187313121386474029603878418357067784098967293223092283636409020167\
        703716182733692845408721808014477176262550695347616088679696249376657532044344868795328929\
        392535511146831725226726902757448067802376817553483740570438218122322533316789620797559903\
        229305975967472086664842304173923792599862534979783093955793905853103797525214306877880559\
        061734489219110902602582677330757355925788842287779292103675340786349085530479489195412741\
        918499599847200289651248252290074764446323588420890650395495995855849103511504849272182404\
        980745441559971498947788737868250072879592234300982294231924966949141757391254082349655397\
        653341386972420309417367538419661786709957833897027278700463997487224093475301726282776037\
        837004173822886358937792496986238232587518046329823253854659034188442660722774644793627247\
        990376912933346554600009355169558242485853202891805973612538480182323344238210359767882413\
        103921664132636490923692956109730629584223001270161789239083304966085658078167369231856383\
        258483946220865233022880717918719236248933318301507731107074568912179691465936726313589320\
        801125976235437730408406869123712855981276022342800437948287640512505691330382612640301053\
        054427352245875538942628240351420297781857455215238445695936344703460370007508830173546384\
        743853762640381345410143839715861887851093305552261243889568585931454598178939878809581309\
        770329441845499753967558468852151992898007843947785274994069368329626033764696951428191654\
        710113484828098115990389170163226019800333588334384534738283702567802318386186513050622079\
        61406496165171974419833626052512494354623335",
        "0x0.4d104d427de7fbcc47c4acd605be48bc13569862a1e8f9a4c52f37935be631e5943516c0c8cfd5e84f2e5\
        e399a38de8948a39a4ad8c5c90f2c5a93fa92a969662fc1bef7012aae5e4e78e8c862030172e9361397ef38817\
        a75c8894d8ac96cee0246bf52cf58a9ec058419e2ca0d5c10b51b3dc09e7a647def7518bbe47c46555ffc0050a\
        d6c33aea8d285fba1ab2d6e288c95f6af2aa4d14352a8f0a1dcfc706500669070af1e4f89da654523cdb17ab2f\
        53e2c74acdee71d94b39a13dc3affbf6fd7d6cc79ca604fa769af1756432207a86596e7286174adf1e94f7580d\
        7f68fd3a6f0499133d03cf3af715e4eae6d1ad0dd66a7db0a257c5f98d9b512bd96eed3363423e0ecbfb271253\
        f1ef918a66c0ceac64f57a4136ea4bb27771cf7873763a9faa1f6d1850941788999ccb7965e1601ead41f9377d\
        ccb6eab6a4940db2b6143dbfe8050b03a8295590fcf5b180ece39ebb8cb9b2d398cf47e27b6d182c13dc84b48b\
        a461ff46aa4a31a1e97d0929b3b1f6bcc919fb23c94998246320f913d90c2d068cdf176038ccc5f861bd05d7cc\
        d4dd34581c6a5bec508322251648a3c316ad321512c83048a2f1ed7e0a0d45ce80c136721627798f7d7d4f8d90\
        eaaa8030664481805066b2eccb791542b81035497287fe1a461661976461b18c5c0275752bf60bc5cabd055ce5\
        1cf436b4b63f6d56993e58e7786dc8e233838b56844e22640802f58710422fbecf0ede15a0cba47a3fbec14792\
        904f82e1978e349094a4ecc0575b27a13bf8fd000a706b5d47e9466721f1e293425ad54a77d7c48d8a24f9d83b\
        c4531148dcebc35713370dc96259c2de21cc5634af7c61a21ec5f909e1740a6b352622a62a2244cde2a9627fca\
        c4e0b168814421493e5d692826c6ebb6309ad985ef27c2fdf02cab4e4f937fef75622e1e19802ed79b40f823e4\
        71bbf5e837288b8eeac5bbe3608fdbfe072c908e4d3455da552ac4df2bf55be63f43c05ddc8a4f8de18c48a3ec\
        fad3389c6942332c438dffa1b1d7ee579e93f3be1e11fde04c9d90ad1e8ae0400626167d5c7997e38e28824b33\
        2c66330d80b24031a7bdd328113a16880cb6611680f647b2b2eca1c827ba818acf134e709e5f77bef9f17a4147\
        0010cfa425b1e8820dda774c3884388e2621770dab07de8049deb57e90ba51835b80ecaf08bacef9459cbac729\
        2eba5d1665f777dfe3f5c8a87b23c30723223cd99a356aa7e83ee88b334e785fdc8a047cc83b4fc56e44ada9d0\
        5519bde829901cf13601246524bfd72d99575195065f2d63feade6422b34275e8a11b31f5b247aaf078ca46458\
        af573448e76e194c4eb866bec8c16e77eead315b887054b00d3e2164b34bb3eff5a0a547418faebfe76587b359\
        89bea7f12c8c2d450fad9c767744c60116b47918b888cb6c591f3f34a02eb8d34fd081001e4c475351b19b56c9\
        f06a67da6ae7fc481c2a728ea7999b60e3c20c8123dab4d5f7e80b75ae7b6bc00b65ee5055ab4c9a23ab846193\
        2bacbeb065dc78d5374210e343393444f2679c6ccae15ce9461f772ae739afbe00084f6e86a16dea8b3fb1a1b0\
        09c59da6be8bcafe8da2d64d02b9135b47fafd65826937b36c3ea02441e565168f796b3679426b7aa3e887b9a3\
        601d6bb9e422125a57d896aecbc68b6fb8254b72cd9f173970ec596ddb070f189e3e93723aa54a7a1f2bf8be51\
        98b26a78d77802877d0a0ed67338bd6d51e110f100256197424b903dff9fe8bb460dff5addc8#10000",
        Greater,
    );

    let log_10_2_f32 = Float::log_10_2_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(log_10_2_f32.to_string(), "0.30103001");
    assert_eq!(to_hex_string(&log_10_2_f32), "0x0.4d104d8#24");
    assert_eq!(log_10_2_f32, f32::LOG_10_2);

    let log_10_2_f64 = Float::log_10_2_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(log_10_2_f64.to_string(), "0.3010299956639812");
    assert_eq!(to_hex_string(&log_10_2_f64), "0x0.4d104d427de7fc#53");
    assert_eq!(log_10_2_f64, f64::LOG_10_2);
}

#[test]
#[should_panic]
fn log_10_2_prec_fail_1() {
    Float::log_10_2_prec(0);
}

fn test_log_10_2_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::log_10_2_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_x, rug_o) = rug_log_10_2_prec_round(prec, rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_x)),
            ComparableFloatRef(&x)
        );
        assert_eq!(rug_o, o);
    }
}

#[test]
pub fn test_log_10_2_prec_round() {
    test_log_10_2_prec_round_helper(1, Floor, "0.2", "0x0.4#1", Less);
    test_log_10_2_prec_round_helper(1, Ceiling, "0.5", "0x0.8#1", Greater);
    test_log_10_2_prec_round_helper(1, Down, "0.2", "0x0.4#1", Less);
    test_log_10_2_prec_round_helper(1, Up, "0.5", "0x0.8#1", Greater);
    test_log_10_2_prec_round_helper(1, Nearest, "0.2", "0x0.4#1", Less);

    test_log_10_2_prec_round_helper(2, Floor, "0.2", "0x0.4#2", Less);
    test_log_10_2_prec_round_helper(2, Ceiling, "0.4", "0x0.6#2", Greater);
    test_log_10_2_prec_round_helper(2, Down, "0.2", "0x0.4#2", Less);
    test_log_10_2_prec_round_helper(2, Up, "0.4", "0x0.6#2", Greater);
    test_log_10_2_prec_round_helper(2, Nearest, "0.2", "0x0.4#2", Less);

    test_log_10_2_prec_round_helper(3, Floor, "0.25", "0x0.4#3", Less);
    test_log_10_2_prec_round_helper(3, Ceiling, "0.3", "0x0.5#3", Greater);
    test_log_10_2_prec_round_helper(3, Down, "0.25", "0x0.4#3", Less);
    test_log_10_2_prec_round_helper(3, Up, "0.3", "0x0.5#3", Greater);
    test_log_10_2_prec_round_helper(3, Nearest, "0.3", "0x0.5#3", Greater);

    test_log_10_2_prec_round_helper(4, Floor, "0.28", "0x0.48#4", Less);
    test_log_10_2_prec_round_helper(4, Ceiling, "0.31", "0x0.50#4", Greater);
    test_log_10_2_prec_round_helper(4, Down, "0.28", "0x0.48#4", Less);
    test_log_10_2_prec_round_helper(4, Up, "0.31", "0x0.50#4", Greater);
    test_log_10_2_prec_round_helper(4, Nearest, "0.31", "0x0.50#4", Greater);

    test_log_10_2_prec_round_helper(5, Floor, "0.3", "0x0.4c#5", Less);
    test_log_10_2_prec_round_helper(5, Ceiling, "0.31", "0x0.50#5", Greater);
    test_log_10_2_prec_round_helper(5, Down, "0.3", "0x0.4c#5", Less);
    test_log_10_2_prec_round_helper(5, Up, "0.31", "0x0.50#5", Greater);
    test_log_10_2_prec_round_helper(5, Nearest, "0.3", "0x0.4c#5", Less);

    test_log_10_2_prec_round_helper(6, Floor, "0.297", "0x0.4c#6", Less);
    test_log_10_2_prec_round_helper(6, Ceiling, "0.305", "0x0.4e#6", Greater);
    test_log_10_2_prec_round_helper(6, Down, "0.297", "0x0.4c#6", Less);
    test_log_10_2_prec_round_helper(6, Up, "0.305", "0x0.4e#6", Greater);
    test_log_10_2_prec_round_helper(6, Nearest, "0.305", "0x0.4e#6", Greater);

    test_log_10_2_prec_round_helper(7, Floor, "0.301", "0x0.4d#7", Less);
    test_log_10_2_prec_round_helper(7, Ceiling, "0.305", "0x0.4e#7", Greater);
    test_log_10_2_prec_round_helper(7, Down, "0.301", "0x0.4d#7", Less);
    test_log_10_2_prec_round_helper(7, Up, "0.305", "0x0.4e#7", Greater);
    test_log_10_2_prec_round_helper(7, Nearest, "0.301", "0x0.4d#7", Less);

    test_log_10_2_prec_round_helper(8, Floor, "0.301", "0x0.4d0#8", Less);
    test_log_10_2_prec_round_helper(8, Ceiling, "0.303", "0x0.4d8#8", Greater);
    test_log_10_2_prec_round_helper(8, Down, "0.301", "0x0.4d0#8", Less);
    test_log_10_2_prec_round_helper(8, Up, "0.303", "0x0.4d8#8", Greater);
    test_log_10_2_prec_round_helper(8, Nearest, "0.301", "0x0.4d0#8", Less);

    test_log_10_2_prec_round_helper(9, Floor, "0.301", "0x0.4d0#9", Less);
    test_log_10_2_prec_round_helper(9, Ceiling, "0.302", "0x0.4d4#9", Greater);
    test_log_10_2_prec_round_helper(9, Down, "0.301", "0x0.4d0#9", Less);
    test_log_10_2_prec_round_helper(9, Up, "0.302", "0x0.4d4#9", Greater);
    test_log_10_2_prec_round_helper(9, Nearest, "0.301", "0x0.4d0#9", Less);

    test_log_10_2_prec_round_helper(10, Floor, "0.3008", "0x0.4d0#10", Less);
    test_log_10_2_prec_round_helper(10, Ceiling, "0.3013", "0x0.4d2#10", Greater);
    test_log_10_2_prec_round_helper(10, Down, "0.3008", "0x0.4d0#10", Less);
    test_log_10_2_prec_round_helper(10, Up, "0.3013", "0x0.4d2#10", Greater);
    test_log_10_2_prec_round_helper(10, Nearest, "0.3013", "0x0.4d2#10", Greater);

    test_log_10_2_prec_round_helper(
        100,
        Floor,
        "0.3010299956639811952137388947242",
        "0x0.4d104d427de7fbcc47c4acd600#100",
        Less,
    );
    test_log_10_2_prec_round_helper(
        100,
        Ceiling,
        "0.3010299956639811952137388947246",
        "0x0.4d104d427de7fbcc47c4acd608#100",
        Greater,
    );
    test_log_10_2_prec_round_helper(
        100,
        Down,
        "0.3010299956639811952137388947242",
        "0x0.4d104d427de7fbcc47c4acd600#100",
        Less,
    );
    test_log_10_2_prec_round_helper(
        100,
        Up,
        "0.3010299956639811952137388947246",
        "0x0.4d104d427de7fbcc47c4acd608#100",
        Greater,
    );
    test_log_10_2_prec_round_helper(
        100,
        Nearest,
        "0.3010299956639811952137388947246",
        "0x0.4d104d427de7fbcc47c4acd608#100",
        Greater,
    );
}

#[test]
#[should_panic]
fn log_10_2_prec_round_fail_1() {
    Float::log_10_2_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn log_10_2_prec_round_fail_2() {
    Float::log_10_2_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn log_10_2_prec_round_fail_3() {
    Float::log_10_2_prec_round(1000, Exact);
}

#[test]
fn log_10_2_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (log_10_2, o) = Float::log_10_2_prec(prec);
        assert!(log_10_2.is_valid());
        assert_eq!(log_10_2.get_prec(), Some(prec));
        assert_eq!(log_10_2.get_exponent(), Some(-1));
        assert_ne!(o, Equal);
        if o == Less {
            let (log_10_2_alt, o_alt) = Float::log_10_2_prec_round(prec, Ceiling);
            let mut next_upper = log_10_2.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(log_10_2_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !log_10_2.is_power_of_2() {
            let (log_10_2_alt, o_alt) = Float::log_10_2_prec_round(prec, Floor);
            let mut next_lower = log_10_2.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(log_10_2_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
        let (log_10_2_alt, o_alt) = Float::log_10_2_prec_round(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&log_10_2_alt),
            ComparableFloatRef(&log_10_2)
        );
        assert_eq!(o_alt, o);

        let (log_10_2_alt, o_alt) = Float::log_base_10_prec(Float::from(2), prec);
        assert_eq!(
            ComparableFloatRef(&log_10_2_alt),
            ComparableFloatRef(&log_10_2)
        );
        assert_eq!(o_alt, o);

        let (rug_log_10_2, rug_o) =
            rug_log_10_2_prec_round(prec, rug_round_try_from_rounding_mode(Nearest).unwrap());
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log_10_2)),
            ComparableFloatRef(&log_10_2)
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn log_10_2_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (log_10_2, o) = Float::log_10_2_prec_round(prec, rm);
        assert!(log_10_2.is_valid());
        assert_eq!(log_10_2.get_prec(), Some(prec));
        assert_eq!(
            log_10_2.get_exponent(),
            Some(if prec == 1 && (rm == Ceiling || rm == Up) {
                0
            } else {
                -1
            })
        );
        assert_ne!(o, Equal);
        if o == Less {
            let (log_10_2_alt, o_alt) = Float::log_10_2_prec_round(prec, Ceiling);
            let mut next_upper = log_10_2.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(log_10_2_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !log_10_2.is_power_of_2() {
            let (log_10_2_alt, o_alt) = Float::log_10_2_prec_round(prec, Floor);
            let mut next_lower = log_10_2.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(log_10_2_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_log_10_2, rug_o) = rug_log_10_2_prec_round(prec, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_log_10_2)),
                ComparableFloatRef(&log_10_2)
            );
            assert_eq!(rug_o, o);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::log_10_2_prec_round(prec, Exact));
    });

    test_constant(Float::log_10_2_prec_round, 10000);
}
