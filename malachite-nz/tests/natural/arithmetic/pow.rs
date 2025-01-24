// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CheckedLogBase, CheckedRoot, Pow, PowAssign, PowerOf2, Square,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    unsigned_gen_var_5, unsigned_pair_gen_var_29, unsigned_vec_unsigned_pair_gen_var_31,
};
use malachite_nz::natural::arithmetic::pow::limbs_pow;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_gen, natural_natural_unsigned_triple_gen_var_1, natural_unsigned_pair_gen_var_4,
    natural_unsigned_unsigned_triple_gen_var_5,
};
use malachite_nz::test_util::natural::arithmetic::pow::{
    natural_pow_naive, natural_pow_simple_binary,
};
use num::traits::Pow as NumPow;
use num::BigUint;
use rug::ops::Pow as RugPow;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_pow() {
    let test = |xs: &[Limb], exp: u64, out: &[Limb]| {
        assert_eq!(limbs_pow(xs, exp), out);
    };
    test(&[2], 2, &[4]);
    test(&[2], 10, &[1024]);
    test(&[2], 100, &[0, 0, 0, 16]);
    test(
        &[3],
        100,
        &[3476558801, 3600055637, 1531049845, 1731684438, 1514558410],
    );
    test(&[10], 9, &[1000000000]);
    test(
        &[10],
        100,
        &[
            0, 0, 0, 2821623568, 2863809288, 2384534140, 4085960256, 2227490315, 2095778599,
            2904921283, 4681,
        ],
    );
    test(&[1, 1], 2, &[1, 2, 1]);
    test(
        &[1, 1],
        10,
        &[1, 10, 45, 120, 210, 252, 210, 120, 45, 10, 1],
    );
    test(
        &[1, 1],
        100,
        &[
            1, 100, 4950, 161700, 3921225, 75287520, 1192052400, 3122658912, 1404300575,
            3856263611, 1591254002, 3258062030, 899556955, 803216378, 1310151248, 1499375850,
            2332619193, 3968431524, 4085749530, 1136679331, 273489285, 3338642246, 3544105701,
            844866219, 1175366874, 758834048, 646998201, 3049457170, 733614013, 1400647268,
            3979434005, 2770685599, 4038926785, 264011298, 2126789533, 3205011234, 3674235589,
            3864120130, 1717547830, 3817918906, 2976806973, 2930427077, 3798905017, 2672346699,
            2426398649, 1834316445, 4131270269, 887459147, 1698650569, 3123194158, 2868271121,
            3706731654, 2073725215, 570568395, 729640870, 1868322716, 3338774936, 3779458204,
            3890882669, 1414262575, 1200805502, 3641833126, 342345696, 1286652406, 3703949518,
            47177294, 1562872441, 3379562707, 2490682825, 640606377, 1577764504, 3545174992,
            2808433500, 3939117033, 343741199, 2292546107, 1377056316, 1693477863, 368237605,
            832210433, 2481934560, 2826277781, 3285796914, 2204777130, 3821989418, 1802445372,
            1367480655, 813259882, 901179532, 3258302570, 1591286535, 3856267598, 1404301014,
            3122658955, 1192052403, 75287520, 3921225, 161700, 4950, 100, 1,
        ],
    );
    test(
        &[1, 2, 3],
        10,
        &[
            1, 20, 210, 1500, 8085, 34704, 122520, 363120, 915570, 1980440, 3692140, 5941320,
            8240130, 9804240, 9924120, 8433072, 5893965, 3280500, 1377810, 393660, 59049,
        ],
    );
    test(
        &[u32::MAX; 3],
        5,
        &[
            u32::MAX,
            u32::MAX,
            u32::MAX,
            4,
            0,
            0,
            0xfffffff6,
            u32::MAX,
            u32::MAX,
            9,
            0,
            0,
            0xfffffffb,
            u32::MAX,
            u32::MAX,
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pow_fail_1() {
    limbs_pow(&[], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pow_fail_2() {
    limbs_pow(&[], 1);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pow_fail_3() {
    limbs_pow(&[1, 1], 0);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pow_fail_4() {
    limbs_pow(&[1, 1], 1);
}

#[test]
fn test_pow() {
    let test = |s, exp, out| {
        let u = Natural::from_str(s).unwrap();

        let mut x = u.clone();
        x.pow_assign(exp);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = u.clone().pow(exp);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = (&u).pow(exp);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());

        let x = BigUint::from_str(s).unwrap().pow(exp);
        assert_eq!(x.to_string(), out);

        let x = rug::Integer::from_str(s).unwrap().pow(u32::exact_from(exp));
        assert_eq!(x.to_string(), out);

        assert_eq!(natural_pow_naive(&u, exp).to_string(), out);
        assert_eq!(natural_pow_simple_binary(&u, exp).to_string(), out);
        assert_eq!(u.pow_ref_alt(exp).to_string(), out);
    };
    test("0", 0, "1");
    test("1", 0, "1");
    test("2", 0, "1");
    test("1000", 0, "1");
    test("1000000000000", 0, "1");
    test("0", 1, "0");
    test("1", 1, "1");
    test("2", 1, "2");
    test("1000", 1, "1000");
    test("1000000000000", 1, "1000000000000");
    test("0", 2, "0");
    test("1", 2, "1");
    test("2", 2, "4");
    test("3", 2, "9");
    test("1000", 2, "1000000");
    test("1000000000000", 2, "1000000000000000000000000");
    // - xs.len() == 1 first time
    // - *x <= HALF_MASK in bsize_1_helper
    // - exp.even() in bsize_1_helper
    // - *exp != 0 in bsize_1_helper
    // - *trailing_zero_bits_out == 0 || *out_0 == 1 || *out_0 >> (Limb::WIDTH -
    //   *trailing_zero_bits_out) != 0 in bsize_1_helper
    // - exp != 0
    // - len == 1 || exp.even()
    // - len == 1
    // - bits.odd()
    // - len == 1 && bit
    // - len == 1 && !bit
    // - out_0 == 1
    // - trailing_zero_bits_out == 0
    test(
        "123",
        456,
        "992500687720988567008314620574696326372959408198869005198162988813828671047493990779211286\
        6142614463805542423693627187249280035274164990211814381967260156999810012079049675951763646\
        5445895625741609866209900500198407153244604778968016963028050310261417615914468729918240685\
        4878786176459769390634643579861657117309763994785076492286863414669671679101266533421349427\
        4485146389992748709248661097714611276356710167264595313219648143933987301708814041466127119\
        8500333255713096142335151414630651683065518784081203678487703002802082091236603519026256880\
        6244996817813872275740354848312715156831237421490955692604636096559777009388445806119312464\
        9516620869554031369814001163802732256625268978083813635182879531427216211122223117090171561\
        2355701347552371530013693855379834865667060014643302459100429783653966913783002290784283455\
        6282833554705299329560514844771293338811599302127586876027950885792304316616960102321873904\
        36601614145603241902386663442520160735566561",
    );
    // - exp.odd() in bsize_1_helper
    // - out_0 != 1
    test(
        "123",
        457,
        "122077584589681593742022698330687648143874007208460887639374047624100926538841760865842988\
        2535541579048081718114316144031661444338722293796053168981972999310976631485723110142066928\
        5249845161966218013543817761524404079849086387813066086452450188162154366757479653779943604\
        3150090699704551635048061160322983825429100971358564408551284200004369616529455783610825979\
        5761673005969108091237585315018897186991875350573545223526016721703880438110184127100333635\
        7415540990452710825507223623999570157017058810441988052453987469344656097222102232840229596\
        3168134608591106289916063646342463964290242202843387550190370239876852572154778834152675433\
        1890544366955145858487122143147736067564908084304309077127494182365547593968033443402091102\
        0319751265748941698191684344211719688477048381801126202469352863389437930395309281766466865\
        0422788527228751817535943325906869080673826714161693185751437958952453430943886092585590490\
        23701998539909198753993559603429979770474687003",
    );
    // - *trailing_zero_bits_out != 0 && *out_0 != 1 && *out_0 >> (Limb::WIDTH -
    //   *trailing_zero_bits_out) == 0 in bsize_1_helper
    // - bits.even()
    test(
        "10",
        100,
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000",
    );
    test(
        "10",
        101,
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000",
    );
    // - *exp == 0 in bsize_1_helper
    // - exp == 0
    // - trailing_zero_bits_out != 0
    test("2", 100, "1267650600228229401496703205376");
    // - xs.len() == 2
    // - xs.len() == 2 && trailing_zero_bits_in == 0
    // - xs.len() == 2 && x_1 != 0
    // - len >> 1 && exp.odd()
    // - len != 1
    // - !CountOnes::count_ones(exp).eq_mod_power_of_2(bits, 1)
    // - len != 1 && !bit
    // - len != 1 && bit
    test(
        "12345678987654321",
        5,
        "286797196211272153445658333952148540084053148773966942500383143133144940612575601",
    );
    test(
        "12345678987654321",
        6,
        "354070611898367606555445954656918550154832595750628623501854823341167403514406003590115726\
        6821921",
    );
    // - xs.len() == 2 && trailing_zero_bits_in != 0
    // - CountOnes::count_ones(exp).eq_mod_power_of_2(bits, 1)
    test(
        "26872817533106",
        12,
        "141826844138959364185940647071896554485587687634106997683499856265150283163630969349636867\
        234174443605205673555107919666790176734540047683138771242870390213185536",
    );
    // - xs.len() == 2 && x_1 == 0
    test(
        "14823718968",
        5,
        "715790392004360489610590536764569459261039357165568",
    );
    // - xs.len() > 2
    // - xs.len() > 2 && trailing_zero_bits_in != 0
    test(
        "24424421844081699326",
        55,
        "213961716190310170274088594850318375142996259342375501592273155327928007898048540877403565\
        3947261530991648842767731271386297041920019953715121571673691873897348960187317337771890905\
        1572382143119842882294517424053653145781261386562757014940642327399405478861990592953674053\
        5435746102693851943687705326429729387827027180615610631114411764811616316442929883964904298\
        3315499400867326194538467726927965720256457038271587840064522521610550522662098778092390775\
        1204011661884808984320689270774425563736748276177153496839435580581401399443258277883965208\
        0795662477157903348885713252035517047912637599810655108674777287567258783790305252352655254\
        9161655860569081456083564805569388857136788414537348033500867479587662770873397446881847014\
        6730827427892395144171556599844946073138166223624637280926661575895662911794190561258748348\
        7934048838204018355648003975520634897314818368802033588189644263971368238570985509229153518\
        4180400851608974878145053640167246336219940448713158030020593993974770074718137925127520351\
        9045385528768578776802376700963217886663279210963568902459805925376",
    );
    // - xs.len() > 2 && trailing_zero_bits_in == 0
    test(
        "762845729280891732629",
        6,
        "197069908665560609451994985160700801551868029168403239983377351218306354370509861303202043\
        757725875289425891538656609168767721",
    );
    // - x == 0
    test(
        "576460717943685120",
        9,
        "702954903302866311524348366058883550484761300374963994159100861262489870076604686210427395\
        8248045324225063677698694075567381512070601339698619038564352000000000",
    );
}

#[test]
fn limbs_pow_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_31().test_properties_with_config(&config, |(xs, exp)| {
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_pow(&xs, exp)),
            Natural::from_owned_limbs_asc(xs).pow(exp)
        );
    });
}

#[test]
fn pow_properties() {
    natural_unsigned_pair_gen_var_4().test_properties(|(x, exp)| {
        let power = (&x).pow(exp);
        assert!(power.is_valid());

        let power_alt = x.clone().pow(exp);
        assert!(power_alt.is_valid());
        assert_eq!(power_alt, power);

        let mut power_alt = x.clone();
        power_alt.pow_assign(exp);
        assert!(power_alt.is_valid());
        assert_eq!(power_alt, power);

        assert_eq!(Natural::from(&BigUint::from(&x).pow(exp)), power);
        assert_eq!(
            Natural::exact_from(&rug::Integer::from(&x).pow(u32::exact_from(exp))),
            power
        );

        assert_eq!(power, natural_pow_naive(&x, exp));
        assert_eq!(power, natural_pow_simple_binary(&x, exp));
        assert_eq!(power, x.pow_ref_alt(exp));
        if exp != 0 {
            assert_eq!((&power).checked_root(exp).unwrap(), x);
        }
        if x > 1 {
            assert_eq!(power.checked_log_base(&x).unwrap(), exp);
        }
    });

    natural_gen().test_properties(|x| {
        assert_eq!((&x).pow(0), 1);
        assert_eq!((&x).pow(1), x);
        assert_eq!((&x).pow(2), (&x).square());
    });

    unsigned_gen_var_5().test_properties(|exp| {
        assert_eq!(Natural::ZERO.pow(exp), u64::from(exp == 0));
        assert_eq!(Natural::ONE.pow(exp), 1);
        assert_eq!(Natural::TWO.pow(exp), Natural::power_of_2(exp));
    });

    natural_natural_unsigned_triple_gen_var_1().test_properties(|(x, y, exp)| {
        assert_eq!((&x * &y).pow(exp), x.pow(exp) * y.pow(exp));
    });

    natural_unsigned_unsigned_triple_gen_var_5().test_properties(|(x, e, f)| {
        assert_eq!((&x).pow(e + f), (&x).pow(e) * (&x).pow(f));
        assert_eq!((&x).pow(e * f), x.pow(e).pow(f));
    });

    unsigned_pair_gen_var_29::<Limb>().test_properties(|(x, y)| {
        assert_eq!(Pow::pow(x, y), Natural::from(x).pow(y));
    });
}
