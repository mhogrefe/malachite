// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::Log210;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{
    rug_round_try_from_rounding_mode, test_constant, to_hex_string,
};
use malachite_float::test_util::constants::log_2_10::rug_log_2_10_prec_round;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_log_2_10_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::log_2_10_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = Float::log_base_2_prec(Float::from(10), prec);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);

    let (rug_x, rug_o) =
        rug_log_2_10_prec_round(prec, rug_round_try_from_rounding_mode(Nearest).unwrap());
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_x)),
        ComparableFloatRef(&x)
    );
    assert_eq!(rug_o, o);
}

#[test]
pub fn test_log_2_10_prec() {
    test_log_2_10_prec_helper(1, "4.0", "0x4.0#1", Greater);
    test_log_2_10_prec_helper(2, "3.0", "0x3.0#2", Less);
    test_log_2_10_prec_helper(3, "3.5", "0x3.8#3", Greater);
    test_log_2_10_prec_helper(4, "3.2", "0x3.4#4", Less);
    test_log_2_10_prec_helper(5, "3.4", "0x3.6#5", Greater);
    test_log_2_10_prec_helper(6, "3.3", "0x3.5#6", Less);
    test_log_2_10_prec_helper(7, "3.31", "0x3.50#7", Less);
    test_log_2_10_prec_helper(8, "3.33", "0x3.54#8", Greater);
    test_log_2_10_prec_helper(9, "3.32", "0x3.52#9", Less);
    test_log_2_10_prec_helper(10, "3.32", "0x3.52#10", Less);
    test_log_2_10_prec_helper(
        100,
        "3.32192809488736234787031942949",
        "0x3.5269e12f346e2bf924afdbfd4#100",
        Greater,
    );
    test_log_2_10_prec_helper(
        1000,
        "3.321928094887362347870319429489390175864831393024580612054756395815934776608625215850139\
        743359370155099657371710250251826824096984263526888275302772998655393851951352657505568643\
        017609190024891666941433374011903124187375109715866467540179189655806735830779688432725883\
        2749925224489023835599764173941381",
        "0x3.5269e12f346e2bf924afdbfd36bf6d3365b157f8deceb53a46dab2020b9e167419943f7a77547ce8f892f\
        aad8eb42f5850d7b920159729533fc58b353d80ce07d93e5b05f66d7537fb9b09ae3e326f40e1797e40730b46b\
        23a3dd5dd44254c2e19631b7fa8fbb5c4c13e22d0facae1beaa4d8f3e706623135a712f0824#1000",
        Greater,
    );
    test_log_2_10_prec_helper(
        10000,
        "3.321928094887362347870319429489390175864831393024580612054756395815934776608625215850139\
        743359370155099657371710250251826824096984263526888275302772998655393851951352657505568643\
        017609190024891666941433374011903124187375109715866467540179189655806735830779688432725883\
        274992522448902383559976417394137928009772756686355477901486745057845884780271042254560972\
        234657956955415370191576411717792471651350023921127147339361440723397211574851007094987891\
        658880831322194806793298232325931195067139950783700336734248070663527500840691762638625354\
        688015368621618418860858994835381321499893027044179207865922601822965371575367239660695116\
        486836846623858508486062990542699469279116273206134006446704847634070437352336742212830896\
        703645790921677219090214219621424574446585245359484488154834592514295409373539065494486327\
        792984242915911811311632981257694501981575037921855384878203551601973782772888817598743328\
        660727123938252022133328052551248827434448842453165465061241489182286793252664292811659922\
        851627345081860071446839558804633121279264003631201457736887904048271052865203359481532478\
        070748327125903362829769991028816810404197503735586238049254996720862167754810108834579898\
        042144858441997382120653126115252962870828403579279034958040901220864616278339249716537639\
        993209963332453859887857132513514559752100369439584477273831743594697315309378516265846495\
        665876896412147100663514475807900689771854296851504183578945223943713440629222149119070350\
        168420234727467080115458756840107582043672758575312668712636649023867841677127620023283187\
        671685822439769327574127351721319292291252568344526849709026431722445452968913731342733143\
        230858772164273850022766731110007331933592405443085102541294139861698326920147956744493995\
        320581621532281327456453275242315871549699501319636430009420623187004290412273916837009448\
        577274992945247549060387705549497436171799326792474787031785343897776848272445670238223213\
        463988792370470555413191641912433993149631531689462270600499475229169461363640721292924705\
        748175032181010687038003936486327388765775205414308484448383154896604574427286093113137509\
        901744242608092622545913233727012788545449265854339599396717438498092334316359449036069091\
        957232317586529195095145815602507991416222632800552057509261182546702115276525341856442030\
        186468532660206842719994804982412454498309495161890652253233620743364532859807087997715019\
        713456162738417484249144311918988862036586590319012292027206040606684106922072273041239754\
        297228029803905668549966597093867974954348590331570172779031448989652923298736411085383736\
        587609465609452281213087618120436345169233425766903840451993551378285506956771616722607288\
        189670164061065121921089215398332484274821017384866323532160779003021649871213093027605607\
        156375417475025931270895620693556694116700409091241908852446830082928454384872630095589739\
        830574794318400859528930617895990017668017704636109243378017728228476523056456319217419861\
        639740754290890015553994202977067380285480595951702256693624444589667951779325892311151517\
        4390137271216196873040425386861356531616902",
        "0x3.5269e12f346e2bf924afdbfd36bf6d3365b157f8deceb53a46dab2020b9e167419943f7a77547ce8f892f\
        aad8eb42f5850d7b920159729533fc58b353d80ce07d93e5b05f66d7537fb9b09ae3e326f40e1797e40730b46b\
        23a3dd5dd44254c2e19631b7fa8fbb5c4c13e22d0facae1beaa4d8f3e706623135a712f0822787d85aeedcb0fa\
        6953f746978fc464846c577be8fea5e31087e198c7f04169234c3b2c46ed153cabf1a480c5b5fb3ba8bc6764c0\
        a945689afbb3ca3410dc05d4ac6f9169cba6972f4d4e14403853d32cf1bc5c6b7ebe5e3259a23c5c87e3de379b\
        81d5c2ce01aa3e549645f83b088d27f8a1471c310e001b75b24d249f15002c13b6ff5a5f4021f74d4812fb2d61\
        4509785145e5b03750de99b270d1f868e773350012c1940f51c1b099a12c3133333d81df2faf7c15c37d50aa13\
        9b57e5b2fea11ca373be65a92674a3c735c81bd3817c8e80af3b835cabb22adcc9a0502b2191381f7b05d6b05b\
        7803a63a871969ab077c446ee9017e4307f42b3fe641c52494c8c08d8ca6facd1b4d7b43e6fef6431029d883b4\
        786ddc24f45d642fdd9f2a1c918ba88fb796db05ace02863533556667fff2d73c9b670680bb9da827494a7d14c\
        c638664d3014632db2f3d05395c3ba838e3aeeb556a34561beba2b78bd29aee2dcabb49030b015816d49cb1157\
        557ab1be4598ea8ebad8c813257c7b83f5b0b9f4f4fd84a2bf7a2d7bdf26dad1d8ac59a42ca97b0cf0ab0a05e9\
        62ff07154573663c4b4cdd85067fa7406b09a9c90d93f784cde4ffe576426d3f39b32a6594d9b6c5318e70a13a\
        45d41d86b9e22d309fcfbcec9d7940c90be984d4fe1f5cedf5e6a286572b9df125cd45fb4feadaea9d98585520\
        ecb62b4256824d7b9e2a9d878e9bc99d9633a565cbf5c0c04ed54e1c06fbefff78acb9f41a16c91fcf5bec8e3f\
        82f0ee9955e1151612d2521b902fcadb2845efa65c3621a6e65c1768a851fb93bb97517e4693228458fc1f2f88\
        71d52b99f6f5b1f00c74ac584da71adcaa4f816b42160e3a7e584aca27ad169be720afd25fabf8a0dd8a488750\
        5e24f050307f4887a1c9152d9c2a2e4129e0bd5356160eb80744c7f9bcc8f16b474e1d39f755b7e63fe8b60b74\
        971984d40376c8b898613012f8cb6833734099498e4c92aa7a9cee261d088c184e4f265bd9ffe1d1ddb769fcad\
        9afc8a9c2ee5d52523f7d1e61ab8f81782df7e516d30bf6329d5e712940044e1f80925ca1ff01a8e01914bf5b0\
        463f8207b8b1bdc87e4b0d9a66f794c5b1ad509f3ed84746fb5ab5006696d23e14f22f83d623113e6c0de7fb1e\
        30b3e4dd3749955905a9c879f8ad03dfd66ac4cf5edf4cb472d4912f5315fcc8f4f22862776b18f226b0a931a2\
        a15a2db4d150d8dfddb44884d4e4a1b2edcdd558ed7db6f286658880f58cc9f460802b35796fd5f51bbf012260\
        d80f4d858735512cf9be5262420990a7b67523954acfb1403c460c5f4e757dd87df2ea71cb93118bc8579e955d\
        386fb1407bb5482df9fb51a188ed57787c932a78004e211861bd791298e8236fdc4c3a4e3ddf731912c369f8a0\
        e2e63d3283f9aa8c6d536ea57a5fd9c44b7af2256b039248a57ff4a746a4f692dbe9a47f10669b8c0403c7d4e9\
        cce4479e06b74e5e1fcd329dffaff7e495a41d4aa3ca6775de68285f5545d1b9a133b7cd5a174a2f70e55a62f4\
        472fb3d324d3c2093ddcf9abefbfb2b97d40b81d8bb85389014399cd13b0e9aecb1ba307bdc#10000",
        Greater,
    );

    let log_2_10_f32 = Float::log_2_10_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(log_2_10_f32.to_string(), "3.321928");
    assert_eq!(to_hex_string(&log_2_10_f32), "0x3.5269e0#24");
    assert_eq!(log_2_10_f32, f32::LOG_2_10);

    let log_2_10_f64 = Float::log_2_10_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(log_2_10_f64.to_string(), "3.3219280948873622");
    assert_eq!(to_hex_string(&log_2_10_f64), "0x3.5269e12f346e2#53");
    assert_eq!(log_2_10_f64, f64::LOG_2_10);
}

#[test]
#[should_panic]
fn log_2_10_prec_fail_1() {
    Float::log_2_10_prec(0);
}

fn test_log_2_10_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::log_2_10_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_x, rug_o) = rug_log_2_10_prec_round(prec, rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_x)),
            ComparableFloatRef(&x)
        );
        assert_eq!(rug_o, o);
    }
}

#[test]
pub fn test_log_2_10_prec_round() {
    test_log_2_10_prec_round_helper(1, Floor, "2.0", "0x2.0#1", Less);
    test_log_2_10_prec_round_helper(1, Ceiling, "4.0", "0x4.0#1", Greater);
    test_log_2_10_prec_round_helper(1, Down, "2.0", "0x2.0#1", Less);
    test_log_2_10_prec_round_helper(1, Up, "4.0", "0x4.0#1", Greater);
    test_log_2_10_prec_round_helper(1, Nearest, "4.0", "0x4.0#1", Greater);

    test_log_2_10_prec_round_helper(2, Floor, "3.0", "0x3.0#2", Less);
    test_log_2_10_prec_round_helper(2, Ceiling, "4.0", "0x4.0#2", Greater);
    test_log_2_10_prec_round_helper(2, Down, "3.0", "0x3.0#2", Less);
    test_log_2_10_prec_round_helper(2, Up, "4.0", "0x4.0#2", Greater);
    test_log_2_10_prec_round_helper(2, Nearest, "3.0", "0x3.0#2", Less);

    test_log_2_10_prec_round_helper(3, Floor, "3.0", "0x3.0#3", Less);
    test_log_2_10_prec_round_helper(3, Ceiling, "3.5", "0x3.8#3", Greater);
    test_log_2_10_prec_round_helper(3, Down, "3.0", "0x3.0#3", Less);
    test_log_2_10_prec_round_helper(3, Up, "3.5", "0x3.8#3", Greater);
    test_log_2_10_prec_round_helper(3, Nearest, "3.5", "0x3.8#3", Greater);

    test_log_2_10_prec_round_helper(4, Floor, "3.2", "0x3.4#4", Less);
    test_log_2_10_prec_round_helper(4, Ceiling, "3.5", "0x3.8#4", Greater);
    test_log_2_10_prec_round_helper(4, Down, "3.2", "0x3.4#4", Less);
    test_log_2_10_prec_round_helper(4, Up, "3.5", "0x3.8#4", Greater);
    test_log_2_10_prec_round_helper(4, Nearest, "3.2", "0x3.4#4", Less);

    test_log_2_10_prec_round_helper(5, Floor, "3.2", "0x3.4#5", Less);
    test_log_2_10_prec_round_helper(5, Ceiling, "3.4", "0x3.6#5", Greater);
    test_log_2_10_prec_round_helper(5, Down, "3.2", "0x3.4#5", Less);
    test_log_2_10_prec_round_helper(5, Up, "3.4", "0x3.6#5", Greater);
    test_log_2_10_prec_round_helper(5, Nearest, "3.4", "0x3.6#5", Greater);

    test_log_2_10_prec_round_helper(6, Floor, "3.3", "0x3.5#6", Less);
    test_log_2_10_prec_round_helper(6, Ceiling, "3.38", "0x3.6#6", Greater);
    test_log_2_10_prec_round_helper(6, Down, "3.3", "0x3.5#6", Less);
    test_log_2_10_prec_round_helper(6, Up, "3.38", "0x3.6#6", Greater);
    test_log_2_10_prec_round_helper(6, Nearest, "3.3", "0x3.5#6", Less);

    test_log_2_10_prec_round_helper(7, Floor, "3.31", "0x3.50#7", Less);
    test_log_2_10_prec_round_helper(7, Ceiling, "3.34", "0x3.58#7", Greater);
    test_log_2_10_prec_round_helper(7, Down, "3.31", "0x3.50#7", Less);
    test_log_2_10_prec_round_helper(7, Up, "3.34", "0x3.58#7", Greater);
    test_log_2_10_prec_round_helper(7, Nearest, "3.31", "0x3.50#7", Less);

    test_log_2_10_prec_round_helper(8, Floor, "3.31", "0x3.50#8", Less);
    test_log_2_10_prec_round_helper(8, Ceiling, "3.33", "0x3.54#8", Greater);
    test_log_2_10_prec_round_helper(8, Down, "3.31", "0x3.50#8", Less);
    test_log_2_10_prec_round_helper(8, Up, "3.33", "0x3.54#8", Greater);
    test_log_2_10_prec_round_helper(8, Nearest, "3.33", "0x3.54#8", Greater);

    test_log_2_10_prec_round_helper(9, Floor, "3.32", "0x3.52#9", Less);
    test_log_2_10_prec_round_helper(9, Ceiling, "3.33", "0x3.54#9", Greater);
    test_log_2_10_prec_round_helper(9, Down, "3.32", "0x3.52#9", Less);
    test_log_2_10_prec_round_helper(9, Up, "3.33", "0x3.54#9", Greater);
    test_log_2_10_prec_round_helper(9, Nearest, "3.32", "0x3.52#9", Less);

    test_log_2_10_prec_round_helper(10, Floor, "3.32", "0x3.52#10", Less);
    test_log_2_10_prec_round_helper(10, Ceiling, "3.324", "0x3.53#10", Greater);
    test_log_2_10_prec_round_helper(10, Down, "3.32", "0x3.52#10", Less);
    test_log_2_10_prec_round_helper(10, Up, "3.324", "0x3.53#10", Greater);
    test_log_2_10_prec_round_helper(10, Nearest, "3.32", "0x3.52#10", Less);

    test_log_2_10_prec_round_helper(
        100,
        Floor,
        "3.321928094887362347870319429487",
        "0x3.5269e12f346e2bf924afdbfd0#100",
        Less,
    );
    test_log_2_10_prec_round_helper(
        100,
        Ceiling,
        "3.32192809488736234787031942949",
        "0x3.5269e12f346e2bf924afdbfd4#100",
        Greater,
    );
    test_log_2_10_prec_round_helper(
        100,
        Down,
        "3.321928094887362347870319429487",
        "0x3.5269e12f346e2bf924afdbfd0#100",
        Less,
    );
    test_log_2_10_prec_round_helper(
        100,
        Up,
        "3.32192809488736234787031942949",
        "0x3.5269e12f346e2bf924afdbfd4#100",
        Greater,
    );
    test_log_2_10_prec_round_helper(
        100,
        Nearest,
        "3.32192809488736234787031942949",
        "0x3.5269e12f346e2bf924afdbfd4#100",
        Greater,
    );
}

#[test]
#[should_panic]
fn log_2_10_prec_round_fail_1() {
    Float::log_2_10_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn log_2_10_prec_round_fail_2() {
    Float::log_2_10_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn log_2_10_prec_round_fail_3() {
    Float::log_2_10_prec_round(1000, Exact);
}

#[test]
fn log_2_10_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (log_2_10, o) = Float::log_2_10_prec(prec);
        assert!(log_2_10.is_valid());
        assert_eq!(log_2_10.get_prec(), Some(prec));
        assert_eq!(log_2_10.get_exponent(), Some(if prec == 1 { 3 } else { 2 }));
        assert_ne!(o, Equal);
        if o == Less {
            let (log_2_10_alt, o_alt) = Float::log_2_10_prec_round(prec, Ceiling);
            let mut next_upper = log_2_10.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(log_2_10_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !log_2_10.is_power_of_2() {
            let (log_2_10_alt, o_alt) = Float::log_2_10_prec_round(prec, Floor);
            let mut next_lower = log_2_10.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(log_2_10_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
        let (log_2_10_alt, o_alt) = Float::log_2_10_prec_round(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&log_2_10_alt),
            ComparableFloatRef(&log_2_10)
        );
        assert_eq!(o_alt, o);

        let (log_2_10_alt, o_alt) = Float::log_base_2_prec(Float::from(10), prec);
        assert_eq!(
            ComparableFloatRef(&log_2_10_alt),
            ComparableFloatRef(&log_2_10)
        );
        assert_eq!(o_alt, o);

        let (rug_log_2_10, rug_o) =
            rug_log_2_10_prec_round(prec, rug_round_try_from_rounding_mode(Nearest).unwrap());
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log_2_10)),
            ComparableFloatRef(&log_2_10)
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn log_2_10_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (log_2_10, o) = Float::log_2_10_prec_round(prec, rm);
        assert!(log_2_10.is_valid());
        assert_eq!(log_2_10.get_prec(), Some(prec));
        let expected_exponent = match (prec, rm) {
            (1, Ceiling | Up | Nearest) | (2, Ceiling | Up) => 3,
            _ => 2,
        };
        assert_eq!(log_2_10.get_exponent(), Some(expected_exponent));
        assert_ne!(o, Equal);
        if o == Less {
            let (log_2_10_alt, o_alt) = Float::log_2_10_prec_round(prec, Ceiling);
            let mut next_upper = log_2_10.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(log_2_10_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !log_2_10.is_power_of_2() {
            let (log_2_10_alt, o_alt) = Float::log_2_10_prec_round(prec, Floor);
            let mut next_lower = log_2_10.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(log_2_10_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_log_2_10, rug_o) = rug_log_2_10_prec_round(prec, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_log_2_10)),
                ComparableFloatRef(&log_2_10)
            );
            assert_eq!(rug_o, o);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::log_2_10_prec_round(prec, Exact));
    });

    test_constant(Float::log_2_10_prec_round, 10000);
}
