// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::PiOver4;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{test_constant, to_hex_string};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_pi_over_4_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::pi_over_4_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);
}

#[test]
pub fn test_pi_over_4_prec() {
    test_pi_over_4_prec_helper(1, "1.0", "0x1.0#1", Greater);
    test_pi_over_4_prec_helper(2, "0.8", "0x0.c#2", Less);
    test_pi_over_4_prec_helper(3, "0.8", "0x0.c#3", Less);
    test_pi_over_4_prec_helper(4, "0.81", "0x0.d#4", Greater);
    test_pi_over_4_prec_helper(5, "0.78", "0x0.c8#5", Less);
    test_pi_over_4_prec_helper(6, "0.78", "0x0.c8#6", Less);
    test_pi_over_4_prec_helper(7, "0.79", "0x0.ca#7", Greater);
    test_pi_over_4_prec_helper(8, "0.785", "0x0.c9#8", Less);
    test_pi_over_4_prec_helper(9, "0.785", "0x0.c90#9", Less);
    test_pi_over_4_prec_helper(10, "0.785", "0x0.c90#10", Less);
    test_pi_over_4_prec_helper(
        100,
        "0.78539816339744830961566084582",
        "0x0.c90fdaa22168c234c4c6628b8#100",
        Less,
    );
    test_pi_over_4_prec_helper(
        1000,
        "0.785398163397448309615660845819875721049292349843776455243736148076954101571552249657008\
        706335529266995537021628320576661773461152387645557931339852032120279362571025675484630276\
        389911155737238732595491107202743916483361532118912058446695791317800477286412141730865087\
        1526135816620533484018150622853184",
        "0x0.c90fdaa22168c234c4c6628b80dc1cd129024e088a67cc74020bbea63b139b22514a08798e3404ddef951\
        9b3cd3a431b302b0a6df25f14374fe1356d6d51c245e485b576625e7ec6f44c42e9a637ed6b0bff5cb6f406b7e\
        dee386bfb5a899fa5ae9f24117c4b1fe649286651ece45b3dc2007cb8a163bf0598da48361c#1000",
        Less,
    );
    test_pi_over_4_prec_helper(
        10000,
        "0.785398163397448309615660845819875721049292349843776455243736148076954101571552249657008\
        706335529266995537021628320576661773461152387645557931339852032120279362571025675484630276\
        389911155737238732595491107202743916483361532118912058446695791317800477286412141730865087\
        152613581662053348401815062285318431146751651578897043720380230240707313522928841091973147\
        590002832632637205116630346036737985377902358264317591439897988273046529345483152948276279\
        637018615594990687391837971438181222806984545752987282458418340610164160771505348736598806\
        184297675544965235925692634804294073294188096168704616917351283000142031786315890206946442\
        835689447402293409294680367110225306238357536637396342762698069922314730885504989028032255\
        490216008604539953407443692827490129676802837499999593244512487764932933204024079648756114\
        863836727075660630577063336171258815482797042752500784459688221646883302095355154294417286\
        825899563372607188867182789890715970588446898437989445464445133042806701653250481969152798\
        977304105049734523814300266371465819716484038345456992057575488008825463242248943405649853\
        472812430443820869782878893714310613537673987707383279215431963972268774595438659366234829\
        813765100231925417784752462206003214590400890926915026177545485738899049736691959362362063\
        844943681711776011883661552011671064767372823328419257247288026188040514241506014509537548\
        377813345607508896910061874118315978549818151067480699195588695409002335430410304981146578\
        757571545743638926687459626373647146731748922731802699377325738830291336246800688990059120\
        166374779970458699438391592451856635631965637954604393668222744443198450020411765004036312\
        298043304303693087535360493392137040340289338138033368935462367109630830976848535833636940\
        604215629745892371390524804805546068137563564221917947623650413366701247156808197946521446\
        095956991994170363525238470946590237670016056281301279348246224021032122156736401060491321\
        255552665296576686069655509798736261780928446740239091092979321866941164393490603472716458\
        161498953347619506897524866441019737817367099588148927456455655130622351931679869567120650\
        369247725660034098609363826267050874063112937349912857857452297664812734305424115378927464\
        596852649471489943243874732540438482117034567170967235693538997963981311488489857762493131\
        170211496818411173962163459184055656524781152012810971097612811034137440695199428922858999\
        425032404022360423717138962101588355518056457122162039614007126504210684863066866919723813\
        034630637488666681955996614149029088721557644364124508898408642043581028128769017369862774\
        149023506307219927723286417284216807187235140025375827154482170230218690229456234647252428\
        727418996315341387445473282446205420749737180664701214391003567619388783094910362880936558\
        591135714611198816466955262785338683934880778356791525533992384057860738121234296777536441\
        350897569983600935501827644634765549596861952119622420830361428467187985876607554613297762\
        120251342653670168729819547799484988015354915857188610160936280929548044999597753979890453\
        66878567280993723522679662355799039198630201",
        "0x0.c90fdaa22168c234c4c6628b80dc1cd129024e088a67cc74020bbea63b139b22514a08798e3404ddef951\
        9b3cd3a431b302b0a6df25f14374fe1356d6d51c245e485b576625e7ec6f44c42e9a637ed6b0bff5cb6f406b7e\
        dee386bfb5a899fa5ae9f24117c4b1fe649286651ece45b3dc2007cb8a163bf0598da48361c55d39a69163fa8f\
        d24cf5f83655d23dca3ad961c62f356208552bb9ed529077096966d670c354e4abc9804f1746c08ca18217c329\
        05e462e36ce3be39e772c180e86039b2783a2ec07a28fb5c55df06f4c52c9de2bcbf6955817183995497cea956\
        ae515d2261898fa051015728e5a8aaac42dad33170d04507a33a85521abdf1cba64ecfb850458dbef0a8aea715\
        75d060c7db3970f85a6e1e4c7abf5ae8cdb0933d71e8c94e04a25619dcee3d2261ad2ee6bf12ffa06d98a0864d\
        87602733ec86a64521f2b18177b200cbbe117577a615d6c770988c0bad946e208e24fa074e5ab3143db5bfce0f\
        d108e4b82d120a92108011a723c12a787e6d788719a10bdba5b2699c327186af4e23c1a946834b6150bda2583e\
        9ca2ad44ce8dbbbc2db04de8ef92e8efc141fbecaa6287c59474e6bc05d99b2964fa090c3a2233ba186515be7e\
        d1f612970cee2d7afb81bdd762170481cd0069127d5b05aa993b4ea988d8fddc186ffb7dc90a6c08f4df435c93\
        402849236c3fab4d27c7026c1d4dcb2602646dec9751e763dba37bdf8ff9406ad9e530ee5db382f413001aeb06\
        a53ed9027d831179727b0865a8918da3edbebcf9b14ed44ce6cbaced4bb1bdb7f1447e6cc254b332051512bd7a\
        f426fb8f401378cd2bf5983ca01c64b92ecf032ea15d1721d03f482d7ce6e74fef6d55e702f46980c82b5a8403\
        1900b1c9e59e7c97fbec7e8f323a97a7e36cc88be0f1d45b7ff585ac54bd407b22b4154aacc8f6d7ebf48e1d81\
        4cc5ed20f8037e0a79715eef29be32806a1d58bb7c5da76f550aa3d8a1fbff0eb19ccb1a313d55cda56c9ec2ef\
        29632387fe8d76e3c0468043e8f663f4860ee12bf2d5b0b7474d6e694f91e6dbe115974a3926f12fee5e438777\
        cb6a932df8cd8bec4d073b931ba3bc832b68d9dd300741fa7bf8afc47ed2576f6936ba424663aab639c5ae4f56\
        83423b4742bf1c978238f16cbe39d652de3fdb8befc848ad922222e04a4037c0713eb57a81a23f0c73473fc646\
        cea306b4bcbc8862f8385ddfa9d4b7fa2c087e879683303ed5bdd3a062b3cf5b3a278a66d2a13f83f44f82ddf3\
        10ee074ab6a364597e899a0255dc164f31cc50846851df9ab48195ded7ea1b1d510bd7ee74d73faf36bc31ecfa\
        268359046f4eb879f924009438b481c6cd7889a002ed5ee382bc9190da6fc026e479558e4475677e9aa9e3050e\
        2765694dfc81f56e880b96e7160c980dd98a573ea4472065a139cd2906cd1cb729ec52a5286d44014a694ca457\
        583d5cfef26f1b90ad8291da0799d00022e9bed55c6fa47fca5bb1aca8376456d98d94879ee7e6dbfcd014bb16\
        1559914ec0b576a67e3e8422e91e65ba141da92de9c3a6d6cca5136dd424bb1064988eb5ba9ac1269f7df673b9\
        82e23fb6c99bb2aa31c5a6685ffd599149b30ac67b8464d80a95d42530a681644d039060e8f8fd5262696d0a75\
        95ae3f935a67dcff5a874a701fbfa0c3d534b4e39bc09577053374821a11c3ac998e0ba718087b317825a1acfc\
        faebbf24f25c6051ada9c285a1fcd6114a838a1ade714c16a9401cdcf81e1071ff7ab97239f#10000",
        Less,
    );

    let pi_over_4_f32 = Float::pi_over_4_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(pi_over_4_f32.to_string(), "0.78539819");
    assert_eq!(to_hex_string(&pi_over_4_f32), "0x0.c90fdb#24");
    assert_eq!(pi_over_4_f32, f32::PI_OVER_4);

    let pi_over_4_f64 = Float::pi_over_4_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(pi_over_4_f64.to_string(), "0.7853981633974483");
    assert_eq!(to_hex_string(&pi_over_4_f64), "0x0.c90fdaa22168c0#53");
    assert_eq!(pi_over_4_f64, f64::PI_OVER_4);
}

#[test]
#[should_panic]
fn pi_over_4_prec_fail_1() {
    Float::pi_over_4_prec(0);
}

fn test_pi_over_4_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::pi_over_4_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);
}

#[test]
pub fn test_pi_over_4_prec_round() {
    test_pi_over_4_prec_round_helper(1, Floor, "0.5", "0x0.8#1", Less);
    test_pi_over_4_prec_round_helper(1, Ceiling, "1.0", "0x1.0#1", Greater);
    test_pi_over_4_prec_round_helper(1, Down, "0.5", "0x0.8#1", Less);
    test_pi_over_4_prec_round_helper(1, Up, "1.0", "0x1.0#1", Greater);
    test_pi_over_4_prec_round_helper(1, Nearest, "1.0", "0x1.0#1", Greater);

    test_pi_over_4_prec_round_helper(2, Floor, "0.8", "0x0.c#2", Less);
    test_pi_over_4_prec_round_helper(2, Ceiling, "1.0", "0x1.0#2", Greater);
    test_pi_over_4_prec_round_helper(2, Down, "0.8", "0x0.c#2", Less);
    test_pi_over_4_prec_round_helper(2, Up, "1.0", "0x1.0#2", Greater);
    test_pi_over_4_prec_round_helper(2, Nearest, "0.8", "0x0.c#2", Less);

    test_pi_over_4_prec_round_helper(3, Floor, "0.8", "0x0.c#3", Less);
    test_pi_over_4_prec_round_helper(3, Ceiling, "0.9", "0x0.e#3", Greater);
    test_pi_over_4_prec_round_helper(3, Down, "0.8", "0x0.c#3", Less);
    test_pi_over_4_prec_round_helper(3, Up, "0.9", "0x0.e#3", Greater);
    test_pi_over_4_prec_round_helper(3, Nearest, "0.8", "0x0.c#3", Less);

    test_pi_over_4_prec_round_helper(4, Floor, "0.75", "0x0.c#4", Less);
    test_pi_over_4_prec_round_helper(4, Ceiling, "0.81", "0x0.d#4", Greater);
    test_pi_over_4_prec_round_helper(4, Down, "0.75", "0x0.c#4", Less);
    test_pi_over_4_prec_round_helper(4, Up, "0.81", "0x0.d#4", Greater);
    test_pi_over_4_prec_round_helper(4, Nearest, "0.81", "0x0.d#4", Greater);

    test_pi_over_4_prec_round_helper(5, Floor, "0.78", "0x0.c8#5", Less);
    test_pi_over_4_prec_round_helper(5, Ceiling, "0.81", "0x0.d0#5", Greater);
    test_pi_over_4_prec_round_helper(5, Down, "0.78", "0x0.c8#5", Less);
    test_pi_over_4_prec_round_helper(5, Up, "0.81", "0x0.d0#5", Greater);
    test_pi_over_4_prec_round_helper(5, Nearest, "0.78", "0x0.c8#5", Less);

    test_pi_over_4_prec_round_helper(6, Floor, "0.78", "0x0.c8#6", Less);
    test_pi_over_4_prec_round_helper(6, Ceiling, "0.8", "0x0.cc#6", Greater);
    test_pi_over_4_prec_round_helper(6, Down, "0.78", "0x0.c8#6", Less);
    test_pi_over_4_prec_round_helper(6, Up, "0.8", "0x0.cc#6", Greater);
    test_pi_over_4_prec_round_helper(6, Nearest, "0.78", "0x0.c8#6", Less);

    test_pi_over_4_prec_round_helper(7, Floor, "0.78", "0x0.c8#7", Less);
    test_pi_over_4_prec_round_helper(7, Ceiling, "0.79", "0x0.ca#7", Greater);
    test_pi_over_4_prec_round_helper(7, Down, "0.78", "0x0.c8#7", Less);
    test_pi_over_4_prec_round_helper(7, Up, "0.79", "0x0.ca#7", Greater);
    test_pi_over_4_prec_round_helper(7, Nearest, "0.79", "0x0.ca#7", Greater);

    test_pi_over_4_prec_round_helper(8, Floor, "0.785", "0x0.c9#8", Less);
    test_pi_over_4_prec_round_helper(8, Ceiling, "0.789", "0x0.ca#8", Greater);
    test_pi_over_4_prec_round_helper(8, Down, "0.785", "0x0.c9#8", Less);
    test_pi_over_4_prec_round_helper(8, Up, "0.789", "0x0.ca#8", Greater);
    test_pi_over_4_prec_round_helper(8, Nearest, "0.785", "0x0.c9#8", Less);

    test_pi_over_4_prec_round_helper(9, Floor, "0.785", "0x0.c90#9", Less);
    test_pi_over_4_prec_round_helper(9, Ceiling, "0.787", "0x0.c98#9", Greater);
    test_pi_over_4_prec_round_helper(9, Down, "0.785", "0x0.c90#9", Less);
    test_pi_over_4_prec_round_helper(9, Up, "0.787", "0x0.c98#9", Greater);
    test_pi_over_4_prec_round_helper(9, Nearest, "0.785", "0x0.c90#9", Less);

    test_pi_over_4_prec_round_helper(10, Floor, "0.785", "0x0.c90#10", Less);
    test_pi_over_4_prec_round_helper(10, Ceiling, "0.786", "0x0.c94#10", Greater);
    test_pi_over_4_prec_round_helper(10, Down, "0.785", "0x0.c90#10", Less);
    test_pi_over_4_prec_round_helper(10, Up, "0.786", "0x0.c94#10", Greater);
    test_pi_over_4_prec_round_helper(10, Nearest, "0.785", "0x0.c90#10", Less);

    test_pi_over_4_prec_round_helper(
        100,
        Floor,
        "0.78539816339744830961566084582",
        "0x0.c90fdaa22168c234c4c6628b8#100",
        Less,
    );
    test_pi_over_4_prec_round_helper(
        100,
        Ceiling,
        "0.7853981633974483096156608458206",
        "0x0.c90fdaa22168c234c4c6628b9#100",
        Greater,
    );
    test_pi_over_4_prec_round_helper(
        100,
        Down,
        "0.78539816339744830961566084582",
        "0x0.c90fdaa22168c234c4c6628b8#100",
        Less,
    );
    test_pi_over_4_prec_round_helper(
        100,
        Up,
        "0.7853981633974483096156608458206",
        "0x0.c90fdaa22168c234c4c6628b9#100",
        Greater,
    );
    test_pi_over_4_prec_round_helper(
        100,
        Nearest,
        "0.78539816339744830961566084582",
        "0x0.c90fdaa22168c234c4c6628b8#100",
        Less,
    );
}

#[test]
#[should_panic]
fn pi_over_4_prec_round_fail_1() {
    Float::pi_over_4_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn pi_over_4_prec_round_fail_2() {
    Float::pi_over_4_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn pi_over_4_prec_round_fail_3() {
    Float::pi_over_4_prec_round(1000, Exact);
}

#[test]
fn pi_over_4_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (pi_over_4, o) = Float::pi_over_4_prec(prec);
        assert!(pi_over_4.is_valid());
        assert_eq!(pi_over_4.get_prec(), Some(prec));
        assert_eq!(
            pi_over_4.get_exponent(),
            Some(if prec == 1 { 1 } else { 0 })
        );
        assert_ne!(o, Equal);
        if o == Less {
            let (pi_over_4_alt, o_alt) = Float::pi_over_4_prec_round(prec, Ceiling);
            let mut next_upper = pi_over_4.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(pi_over_4_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !pi_over_4.is_power_of_2() {
            let (pi_over_4_alt, o_alt) = Float::pi_over_4_prec_round(prec, Floor);
            let mut next_lower = pi_over_4.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(pi_over_4_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
        let (pi_over_4_alt, o_alt) = Float::pi_over_4_prec_round(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&pi_over_4_alt),
            ComparableFloatRef(&pi_over_4)
        );
        assert_eq!(o_alt, o);
    });
}

#[test]
fn pi_over_4_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (pi_over_4, o) = Float::pi_over_4_prec_round(prec, rm);
        assert!(pi_over_4.is_valid());
        assert_eq!(pi_over_4.get_prec(), Some(prec));
        let expected_exponent = match (prec, rm) {
            (1, Ceiling | Up | Nearest) | (2, Ceiling | Up) => 1,
            _ => 0,
        };
        assert_eq!(pi_over_4.get_exponent(), Some(expected_exponent));
        assert_ne!(o, Equal);
        if o == Less {
            let (pi_over_4_alt, o_alt) = Float::pi_over_4_prec_round(prec, Ceiling);
            let mut next_upper = pi_over_4.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(pi_over_4_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !pi_over_4.is_power_of_2() {
            let (pi_over_4_alt, o_alt) = Float::pi_over_4_prec_round(prec, Floor);
            let mut next_lower = pi_over_4.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(pi_over_4_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::pi_over_4_prec_round(prec, Exact));
    });

    test_constant(Float::pi_over_4_prec_round, 10000);
}
