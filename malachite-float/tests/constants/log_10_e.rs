// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::Log10E;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{test_constant, to_hex_string};
use malachite_float::test_util::constants::log_10_e::log_10_e_prec_round_simple;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_log_10_e_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::log_10_e_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = log_10_e_prec_round_simple(prec, Nearest);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);
}

#[test]
pub fn test_log_10_e_prec() {
    test_log_10_e_prec_helper(1, "0.50", "0x0.8#1", Greater);
    test_log_10_e_prec_helper(2, "0.38", "0x0.6#2", Less);
    test_log_10_e_prec_helper(3, "0.44", "0x0.7#3", Greater);
    test_log_10_e_prec_helper(4, "0.438", "0x0.70#4", Greater);
    test_log_10_e_prec_helper(5, "0.438", "0x0.70#5", Greater);
    test_log_10_e_prec_helper(6, "0.438", "0x0.70#6", Greater);
    test_log_10_e_prec_helper(7, "0.4336", "0x0.6f#7", Less);
    test_log_10_e_prec_helper(8, "0.4336", "0x0.6f0#8", Less);
    test_log_10_e_prec_helper(9, "0.4346", "0x0.6f4#9", Greater);
    test_log_10_e_prec_helper(10, "0.43408", "0x0.6f2#10", Less);
    test_log_10_e_prec_helper(
        100,
        "0.43429448190325182765112891891667",
        "0x0.6f2dec549b9438ca9aadd557d8#100",
        Greater,
    );
    test_log_10_e_prec_helper(
        1000,
        "0.434294481903251827651128918916605082294397005803666566114453783165864649208870774729224\
        949338431748318706106744766303733641679287158963906569221064662812265852127086568670329593\
        370869658826688331163607738490514284434866676864658608513556148212348765343543435731725383\
        562228139560304864665236609553937742",
        "0x0.6f2dec549b9438ca9aadd557d699ee191f71a30122e4d1011d1f96a27bc7529e3aa1277d0a0179f94911a\
        ac96323250a8c671decfe9c6e5e37d15c696466d3d9a1ab5e8ca46837fca0039002c60ee26d32c5b0f5216426b\
        52859b6f6979b9ceaaa1810957346026a32476644e628fc9a6bca6b2793e4b475d9ff2061768#1000",
        Greater,
    );
    test_log_10_e_prec_helper(
        10000,
        "0.434294481903251827651128918916605082294397005803666566114453783165864649208870774729224\
        949338431748318706106744766303733641679287158963906569221064662812265852127086568670329593\
        370869658826688331163607738490514284434866676864658608513556148212348765343543435731725383\
        562228139560304864665236609553937735617632343191671099141159789496299351245793492635765546\
        907767108241915047991098967490010327753765357027008732855095173144067469795189951359408804\
        042393151886810840254465408979702986328682876262414401345704354613292060071260510402836712\
        595484628770786199899232674843990234817153593455107947549255248257782067922014093146816446\
        738103056047563572040888338320948899652271749454133179141764024740750578876786097109925754\
        773004604865604951561005798574134027267520143924791797085904793128521249334119732987722646\
        388535022608388162631646388355368550176846029528639939163351064755570405051318234298887488\
        212064359502381890264331771153738220336263441647839714600185839609300631733398613403513574\
        178714497145307649296833139239981060850573481616980928001619952352311723767656198922812701\
        381580424871597834492721594756205717999348381403194016677152010478719758253161795149037559\
        751424657073664643975686314932516249872799485263744879116595921970172066270455928465703646\
        263567573357573936967399457090960252635095719346883995123681135642801095877831375944271304\
        998064379875041447209597487267406016065010537528700049116786713330915476144100505477593089\
        076788559653343219076312835357030485402097994161401080791060749887175249584146130386753208\
        600132448639254557307284238617597067798935484457031835933652301602797162653572651442851986\
        606376863533818195487638916134365237475946566392138073614450368379787682436902880449364049\
        675187172061413073180441718021644099320065106969695124707266622457000422934140792336168530\
        241886027241186780627257033755256287076769663217367245475813333926384013032003859889994733\
        228570349419583769147209060881244782507873671157303393156562515790709324537045074432662334\
        980714303805958177695794407004220254543053191088898275406226360060187915226747778823209602\
        522876676241633229681246450257729504022662362753631179853215378088327232692078598099075743\
        443736724871035585330654658165353515794399007032643622252001033698041984301552452417319052\
        024721224111092732442530293020087103733750486749868911722567206726827524657879044673526857\
        579405998334659587859262497872538018550638960237530429453996373736743468076751524998629767\
        673240490336317548819532368008766864866606928208234253631130493997270285887284908625845868\
        704556924454853860720249739663112637212249753885496798158028481049472414045334119267424083\
        967306116723425684312962466624625954276067718285896330658651395093204902303280635753624280\
        431548065836885225783290153078748314198592907412141534477216539821484761928840657134543879\
        860789519943501153282645774231126681718328496869789090432442100527223347505314162598164645\
        704453890114831376070844548345795572830386647363846853758717221068599393300837853436755269\
        989918515087905591152528266400289234793788120",
        "0x0.6f2dec549b9438ca9aadd557d699ee191f71a30122e4d1011d1f96a27bc7529e3aa1277d0a0179f94911a\
        ac96323250a8c671decfe9c6e5e37d15c696466d3d9a1ab5e8ca46837fca0039002c60ee26d32c5b0f5216426b\
        52859b6f6979b9ceaaa1810957346026a32476644e628fc9a6bca6b2793e4b475d9ff2061766d8fb66890d6e32\
        8632f4a3eeb60438f3fb1641589c2a337e6e2cc6b892ef890a72b2f15d285ec76de0544ddc9254dd9bd4601639\
        3aa8b7e1d3e8e0be626710264fb0433b4a146bc6ab49b130a5886e5e298699333bdfa95f0b39b0fd7768f99b6b\
        0a0c424a9dd4f8bef2b1fe8d8e367317b4aee29fdacdd143b20d769e0a57a3fdda0aef3043208709c5f2284679\
        bf8732c846a6c330e023e596e955875c9ce40b5b5f75f372ca5d35bc8fc049d3a4eba62ae3021c0d0fe93dbba3\
        186b9221c793dbe90ae7a91473a519eda056576649e29810dcef926cc72f3ddd21903a75b3ef41ba280d548165\
        f7a80e836c7c02f05c234a12e161a000e999a426689b4a566cf652728dde83b141906918025112d76354844105\
        5d17023b624d0032b980611125c424ad56a1b0d677e85496bebcadcdfe4b1668c679ed40c74976bb2f8d115a1d\
        f98080668760310b38f8b545b847644c5e7c2c08e61ca0983e242924d37e8acea1093012456988a1a1bcd639f5\
        db60fdcf681be0e70292d19329a47b9eccd411fd8475cbc68cae2d2acadb2f17c9e517df1d7bf9d43dd2cfd938\
        1b74693e7ed8ac7c30af3df283a5b58f52b26d3dd500ff206d7c64054568b18edfd7052b9a76a15cc708f90766\
        68fbb60e8199cd8ff320f2300e8a6608a5f5b2eee1820a5465a51ef865b64a29e420a3ad13f9a292fc7251861f\
        a954e37e083f68ed115c3ec75d80ca2328ca9148e1b21f2df677e91727946a868882e1ecf452dc7f9039d3025d\
        8f4bf17665b3ffb12ccfa79a4bb8823214a65021fa5405d51b4d416fe42c2cd36843a3ee4f637d4f7f9d109933\
        63ecd39e3a837c8a95cd3a8a9632a6247d77534f307e1f83cbba7c9238d5bdda37666880b221b2f8c74b3747cd\
        d0d7d539a0b4d36f0b0cb3d64588ece9584f03e8d22e2a35aaaaa1c57be3c02d372150ee57f5b8366f1ad83ca0\
        1394a23bbed5c03b33f006e74d91d8da079a9fafb39908bf2da8d020e8621224b9a368cdb6023962c254a690d7\
        6e48531510a454189621b61947c15eda18ff948f0f0265a618e0631b34457e80159cb440981af4e75144ebb427\
        5bb4fb130e612f8aa50be62b78dee6a56ba9573bed3b2f7bd6b85f2c687d72cda4d9612703830bf92bb847c8ac\
        6a8157c1e5c289cb5ceb001fe67f25c8ddec9df12b377bc753e2519d6d21eeb6f6153a94b6cd836e4be1e9705d\
        097dbed945ce0758362aab84c5dc5f4dfeb3b7215b79e46928732c413cf89e9f2def7ab9527e0266d807d20701\
        5ba163500b84f9bec33265e9f9803ca87a44fc2d814433109b88b82bf5a748347d5eb59b390866398dd87624d3\
        66234303848d9d85b039f39a1b1b41a40c180cb26e8dfb7b22f91d7d75deefd1c37fb9b2881aa4cc08dddb3a3b\
        37913d35fccfe3541a5322f0a1880ff62921fc48222912f12b4224de62c104cbe2ef379fc195fe89a632be4631\
        67421b7e53ab00a9feccca0f80853a70c3171a3afe35279b9192941eb243ca1a90726f2a979364e8491426c235\
        5313000eff34794e5c5447b587eaf3d32d64eafd8748dc3ff9e23e0085c98d7d28b809f940d0#10000",
        Greater,
    );

    let log_10_e_f32 = Float::log_10_e_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(log_10_e_f32.to_string(), "0.434294492");
    assert_eq!(to_hex_string(&log_10_e_f32), "0x0.6f2dec8#24");
    assert_eq!(log_10_e_f32, f32::LOG_10_E);

    let log_10_e_f64 = Float::log_10_e_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(log_10_e_f64.to_string(), "0.43429448190325182");
    assert_eq!(to_hex_string(&log_10_e_f64), "0x0.6f2dec549b9438#53");
    assert_eq!(log_10_e_f64, f64::LOG_10_E);
}

#[test]
#[should_panic]
fn log_10_e_prec_fail_1() {
    Float::log_10_e_prec(0);
}

fn test_log_10_e_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::log_10_e_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = log_10_e_prec_round_simple(prec, rm);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);
}

#[test]
pub fn test_log_10_e_prec_round() {
    test_log_10_e_prec_round_helper(1, Floor, "0.25", "0x0.4#1", Less);
    test_log_10_e_prec_round_helper(1, Ceiling, "0.50", "0x0.8#1", Greater);
    test_log_10_e_prec_round_helper(1, Down, "0.25", "0x0.4#1", Less);
    test_log_10_e_prec_round_helper(1, Up, "0.50", "0x0.8#1", Greater);
    test_log_10_e_prec_round_helper(1, Nearest, "0.50", "0x0.8#1", Greater);

    test_log_10_e_prec_round_helper(2, Floor, "0.38", "0x0.6#2", Less);
    test_log_10_e_prec_round_helper(2, Ceiling, "0.50", "0x0.8#2", Greater);
    test_log_10_e_prec_round_helper(2, Down, "0.38", "0x0.6#2", Less);
    test_log_10_e_prec_round_helper(2, Up, "0.50", "0x0.8#2", Greater);
    test_log_10_e_prec_round_helper(2, Nearest, "0.38", "0x0.6#2", Less);

    test_log_10_e_prec_round_helper(3, Floor, "0.38", "0x0.6#3", Less);
    test_log_10_e_prec_round_helper(3, Ceiling, "0.44", "0x0.7#3", Greater);
    test_log_10_e_prec_round_helper(3, Down, "0.38", "0x0.6#3", Less);
    test_log_10_e_prec_round_helper(3, Up, "0.44", "0x0.7#3", Greater);
    test_log_10_e_prec_round_helper(3, Nearest, "0.44", "0x0.7#3", Greater);

    test_log_10_e_prec_round_helper(4, Floor, "0.406", "0x0.68#4", Less);
    test_log_10_e_prec_round_helper(4, Ceiling, "0.438", "0x0.70#4", Greater);
    test_log_10_e_prec_round_helper(4, Down, "0.406", "0x0.68#4", Less);
    test_log_10_e_prec_round_helper(4, Up, "0.438", "0x0.70#4", Greater);
    test_log_10_e_prec_round_helper(4, Nearest, "0.438", "0x0.70#4", Greater);

    test_log_10_e_prec_round_helper(5, Floor, "0.422", "0x0.6c#5", Less);
    test_log_10_e_prec_round_helper(5, Ceiling, "0.438", "0x0.70#5", Greater);
    test_log_10_e_prec_round_helper(5, Down, "0.422", "0x0.6c#5", Less);
    test_log_10_e_prec_round_helper(5, Up, "0.438", "0x0.70#5", Greater);
    test_log_10_e_prec_round_helper(5, Nearest, "0.438", "0x0.70#5", Greater);

    test_log_10_e_prec_round_helper(6, Floor, "0.430", "0x0.6e#6", Less);
    test_log_10_e_prec_round_helper(6, Ceiling, "0.438", "0x0.70#6", Greater);
    test_log_10_e_prec_round_helper(6, Down, "0.430", "0x0.6e#6", Less);
    test_log_10_e_prec_round_helper(6, Up, "0.438", "0x0.70#6", Greater);
    test_log_10_e_prec_round_helper(6, Nearest, "0.438", "0x0.70#6", Greater);

    test_log_10_e_prec_round_helper(7, Floor, "0.4336", "0x0.6f#7", Less);
    test_log_10_e_prec_round_helper(7, Ceiling, "0.4375", "0x0.70#7", Greater);
    test_log_10_e_prec_round_helper(7, Down, "0.4336", "0x0.6f#7", Less);
    test_log_10_e_prec_round_helper(7, Up, "0.4375", "0x0.70#7", Greater);
    test_log_10_e_prec_round_helper(7, Nearest, "0.4336", "0x0.6f#7", Less);

    test_log_10_e_prec_round_helper(8, Floor, "0.4336", "0x0.6f0#8", Less);
    test_log_10_e_prec_round_helper(8, Ceiling, "0.4355", "0x0.6f8#8", Greater);
    test_log_10_e_prec_round_helper(8, Down, "0.4336", "0x0.6f0#8", Less);
    test_log_10_e_prec_round_helper(8, Up, "0.4355", "0x0.6f8#8", Greater);
    test_log_10_e_prec_round_helper(8, Nearest, "0.4336", "0x0.6f0#8", Less);

    test_log_10_e_prec_round_helper(9, Floor, "0.4336", "0x0.6f0#9", Less);
    test_log_10_e_prec_round_helper(9, Ceiling, "0.4346", "0x0.6f4#9", Greater);
    test_log_10_e_prec_round_helper(9, Down, "0.4336", "0x0.6f0#9", Less);
    test_log_10_e_prec_round_helper(9, Up, "0.4346", "0x0.6f4#9", Greater);
    test_log_10_e_prec_round_helper(9, Nearest, "0.4346", "0x0.6f4#9", Greater);

    test_log_10_e_prec_round_helper(10, Floor, "0.43408", "0x0.6f2#10", Less);
    test_log_10_e_prec_round_helper(10, Ceiling, "0.43457", "0x0.6f4#10", Greater);
    test_log_10_e_prec_round_helper(10, Down, "0.43408", "0x0.6f2#10", Less);
    test_log_10_e_prec_round_helper(10, Up, "0.43457", "0x0.6f4#10", Greater);
    test_log_10_e_prec_round_helper(10, Nearest, "0.43408", "0x0.6f2#10", Less);

    test_log_10_e_prec_round_helper(
        100,
        Floor,
        "0.43429448190325182765112891891628",
        "0x0.6f2dec549b9438ca9aadd557d0#100",
        Less,
    );
    test_log_10_e_prec_round_helper(
        100,
        Ceiling,
        "0.43429448190325182765112891891667",
        "0x0.6f2dec549b9438ca9aadd557d8#100",
        Greater,
    );
    test_log_10_e_prec_round_helper(
        100,
        Down,
        "0.43429448190325182765112891891628",
        "0x0.6f2dec549b9438ca9aadd557d0#100",
        Less,
    );
    test_log_10_e_prec_round_helper(
        100,
        Up,
        "0.43429448190325182765112891891667",
        "0x0.6f2dec549b9438ca9aadd557d8#100",
        Greater,
    );
    test_log_10_e_prec_round_helper(
        100,
        Nearest,
        "0.43429448190325182765112891891667",
        "0x0.6f2dec549b9438ca9aadd557d8#100",
        Greater,
    );
}

#[test]
#[should_panic]
fn log_10_e_prec_round_fail_1() {
    Float::log_10_e_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn log_10_e_prec_round_fail_2() {
    Float::log_10_e_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn log_10_e_prec_round_fail_3() {
    Float::log_10_e_prec_round(1000, Exact);
}

#[test]
fn log_10_e_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (log_10_e, o) = Float::log_10_e_prec(prec);
        assert!(log_10_e.is_valid());
        assert_eq!(log_10_e.get_prec(), Some(prec));
        assert_eq!(
            log_10_e.get_exponent(),
            Some(if prec == 1 { 0 } else { -1 })
        );
        assert_ne!(o, Equal);
        if o == Less {
            let (log_10_e_alt, o_alt) = Float::log_10_e_prec_round(prec, Ceiling);
            let mut next_upper = log_10_e.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(log_10_e_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !log_10_e.is_power_of_2() {
            let (log_10_e_alt, o_alt) = Float::log_10_e_prec_round(prec, Floor);
            let mut next_lower = log_10_e.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(log_10_e_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
        let (log_10_e_alt, o_alt) = Float::log_10_e_prec_round(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&log_10_e_alt),
            ComparableFloatRef(&log_10_e)
        );
        assert_eq!(o_alt, o);

        let (log_10_e_alt, o_alt) = log_10_e_prec_round_simple(prec, Nearest);
        assert_eq!(
            ComparableFloatRef(&log_10_e_alt),
            ComparableFloatRef(&log_10_e)
        );
        assert_eq!(o_alt, o);
    });
}

#[test]
fn log_10_e_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (log_10_e, o) = Float::log_10_e_prec_round(prec, rm);
        assert!(log_10_e.is_valid());
        assert_eq!(log_10_e.get_prec(), Some(prec));
        let expected_exponent = match (prec, rm) {
            (1, Ceiling | Up | Nearest) | (2, Ceiling | Up) => 0,
            _ => -1,
        };
        assert_eq!(log_10_e.get_exponent(), Some(expected_exponent));
        assert_ne!(o, Equal);
        if o == Less {
            let (log_10_e_alt, o_alt) = Float::log_10_e_prec_round(prec, Ceiling);
            let mut next_upper = log_10_e.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(log_10_e_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !log_10_e.is_power_of_2() {
            let (log_10_e_alt, o_alt) = Float::log_10_e_prec_round(prec, Floor);
            let mut next_lower = log_10_e.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(log_10_e_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }

        let (log_10_e_alt, o_alt) = log_10_e_prec_round_simple(prec, rm);
        assert_eq!(
            ComparableFloatRef(&log_10_e_alt),
            ComparableFloatRef(&log_10_e)
        );
        assert_eq!(o_alt, o);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::log_10_e_prec_round(prec, Exact));
    });

    test_constant(Float::log_10_e_prec_round, 10000);
}
