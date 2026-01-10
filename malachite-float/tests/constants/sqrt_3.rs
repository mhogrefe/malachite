// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::traits::Sqrt3;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::test_util::common::{
    rug_round_try_from_rounding_mode, test_constant, to_hex_string,
};
use malachite_float::test_util::constants::sqrt_3::rug_sqrt_3_prec_round;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

fn test_sqrt_3_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::sqrt_3_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (rug_x, rug_o) =
        rug_sqrt_3_prec_round(prec, rug_round_try_from_rounding_mode(Nearest).unwrap());
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_x)),
        ComparableFloatRef(&x)
    );
    assert_eq!(rug_o, o);
}

#[test]
pub fn test_sqrt_3_prec() {
    test_sqrt_3_prec_helper(1, "2.0", "0x2.0#1", Greater);
    test_sqrt_3_prec_helper(2, "1.5", "0x1.8#2", Less);
    test_sqrt_3_prec_helper(3, "1.8", "0x1.c#3", Greater);
    test_sqrt_3_prec_helper(4, "1.8", "0x1.c#4", Greater);
    test_sqrt_3_prec_helper(5, "1.75", "0x1.c#5", Greater);
    test_sqrt_3_prec_helper(6, "1.72", "0x1.b8#6", Less);
    test_sqrt_3_prec_helper(7, "1.73", "0x1.bc#7", Greater);
    test_sqrt_3_prec_helper(8, "1.734", "0x1.bc#8", Greater);
    test_sqrt_3_prec_helper(9, "1.73", "0x1.bb#9", Less);
    test_sqrt_3_prec_helper(10, "1.732", "0x1.bb8#10", Greater);
    test_sqrt_3_prec_helper(
        100,
        "1.732050807568877293527446341506",
        "0x1.bb67ae8584caa73b25742d708#100",
        Greater,
    );
    test_sqrt_3_prec_helper(
        1000,
        "1.732050807568877293527446341505872366942805253810380628055806979451933016908800037081146\
        186757248575675626141415406703029969945094998952478811655512094373648528093231902305582067\
        974820101084674923265015312343266903322886650672254668921837971227047131660367861588019049\
        9865373798593894676503475065760508",
        "0x1.bb67ae8584caa73b25742d7078b83b8925d834cc53da4798c720a6486e45a6e2490bcfd95ef15dbda9930\
        aae12228f87cc4cf24da3a1ec68d0cd33a01ad9a383b9e122e6138c3ae6de5ede3bd42db7301b6bf553af7b09f\
        d6ebef33a9a9fe57729426f30e5892ab572816ccefc5899355f7f11c3e24f3768a5c7cb90d6#1000",
        Greater,
    );
    test_sqrt_3_prec_helper(
        10000,
        "1.732050807568877293527446341505872366942805253810380628055806979451933016908800037081146\
        186757248575675626141415406703029969945094998952478811655512094373648528093231902305582067\
        974820101084674923265015312343266903322886650672254668921837971227047131660367861588019049\
        986537379859389467650347506576050756618348129606100947602187190325083145829523959832997789\
        824508288714463832917347224163984587855397667958063818353666110843173780894378316102088305\
        524901670023520711144288695990956365797087168498072899493296484283020786408603988738697537\
        582317317831395992983007838702877053913369563312103707264019249106768231199288375641141422\
        016742752102372994270831059898459475987664288897796147837958390228854852903576033852808064\
        381972344661059689722872865264153822664698420021195484155278441181286534507035191650016689\
        294415480846071277143999762926834629577438361895110127148638746976545982451788550975379013\
        880664961911962222957110555242923723192197738262561631468842032853716682938649611917049738\
        836395495938145757671853373633125910899655424624834787197605235997769192323570220305302840\
        385915414971072429559206706202509520175963185872766359975283663431080150665853710647328538\
        625922260582220510403680270297504798728079461658100417052681940019095733462175943893670249\
        320422691034369812463720111185261084268910299720311202100063507176374582405203847555197279\
        933797614906107894985544223326004018851303631561144886847281589288163245187265066645384877\
        599162576642872111240842068016763517100102943180715515190961642460907039408129216903517492\
        961364004139670431041253632327030922577327960292376597745537095469115742140424230781992327\
        617401906424512454877516862696105333694216213605394604245654140128533007813633449856736406\
        703977342229811961042925534501601405940479547154534548407271737656262366549166640233006013\
        265744070107836858468452313160467754480500402240639911970362218602920238867150711017169400\
        296868759663500040895316214233425227956834067013470185902028360716762147743493449563595808\
        082130442586469468522610908263353008756612603460672195404055984128912975994810000772057440\
        230047673258800091514371489475444879157191294659083570873961515537797640262068370848046072\
        969382719585689759759626104159152657577790782334980567840022901532052138935373775536566427\
        046826874289963441395743666073744445583086477893212985302148197395341478170516614952551763\
        291993699565744522639112519093541386989366817430938226424736926202072990967831154131946484\
        377915459915923928287714695149274026409213645654041644581490201945749419305269002613972646\
        081065071439603206077510594187798284793986195249964165213139715293599421897416647075187235\
        788629466108560170428869605798394052906407430811833388677881562635867156008396760245349229\
        943938867059754315442943430957258470988215463111260766774067864571578060647447499750354544\
        559313286549189849336572747626297414738235686914837831363361283627903824840163806671607179\
        848728555842931349226093240565957553651136754644387834283313466644554180390821898983294626\
        3450161711220169296194601693206210330397449",
        "0x1.bb67ae8584caa73b25742d7078b83b8925d834cc53da4798c720a6486e45a6e2490bcfd95ef15dbda9930\
        aae12228f87cc4cf24da3a1ec68d0cd33a01ad9a383b9e122e6138c3ae6de5ede3bd42db7301b6bf553af7b09f\
        d6ebef33a9a9fe57729426f30e5892ab572816ccefc5899355f7f11c3e24f3768a5c7cb90d55302505df014c70\
        adb1ec1baf58f084433c0d56696a039d962d8434194aab164d642f76e2c8f7dbed64b39dad2f9f7726c6930c49\
        d48ffcdc21114a6c36b2ab7018af65c10f806a8fcf4188ef377d2d5b5fd1942e6fd41f9d23b667813df420b68d\
        313b387ae00d7a1ddd40aa69bb5352d3522529a7269af1ace62c8325ced0d18c24635d661cfb468be1b096fe67\
        7a78756ff46b5506d202bbab5e3351351776080e751e7a6ed2ecbd5cab0343f677bc2f4d3020b12c5a3dd54170\
        514f1d910c64a786d9f4fdde306dfce0de8466b0c0b1097801348a9497aca75bdd1b2cb8a042330fb4a53f02fb\
        e02b0bd65b382e53877c9b3b518f837f09b9ea3eada4de774744c667ee778a7de2100ef3847573d88bf593779a\
        f61a446d5cf76c39e9b5e3a8e82eadb3bab8bbed573c82332c9b692ad88926393687c4cfd268bf8c11f4f39490\
        3816010414c231d5ab4e954cca351a0b97b9c1dfc9c263bc18fe1b0899efae7dc41d63fa0d29a5e5d541f52617\
        ab4671cded759436e30da234491b192dc6d190fe6b15e72e0a142ca532c3971e0e57a50d517325b52415ccf1e4\
        feb9efba3bf091f9c3ffb3a9b243bb001a5fa8c9c9272115c5b9f6cf09325ecc659bb47a3b2dbf660c7bfd4046\
        81a5c00c10bc4c7978e5255fc1aa1230c7768f631ccba5fe87314527436fc1e1851ffcf0709060141b5e7c80d2\
        f80c57e3f4f1242645dc47a2e85bcbe4b8d7a8185e7000f7655499a681467d71000bd05244d47947991e01d822\
        3c817c45780617c2a1fed7f14f334615af5cd378b298c36afe497ed4e6d74b8f87312cc7d9c03b1b260ae50eae\
        83b121f59686686a44115094562752ce8e7eef2baf5383aaf67c4b9cc3e12d6edf2424fab1e11fdea7d433bca4\
        4d5022ed471d20ae60d74b953c30bbc0a8ca668f86c230ff4b42e14ee60fa2752e462642cebb5c383da047309e\
        fec6fff1c9ab3b796ef8d1b446f6b7b87453acf3c77dcdab7d5eb310fb8b07a014e9c03e0f1e0fcdd841bc45e2\
        186afe58e091e66d9d713952fbf2cef88598adf4ce26f02044d52ef69dca5124d91adc008a9375da7ec5b47953\
        0ff2146c81ab095fe0a995a418ed5eef9ab0b044f06369d3e37b1b97baf10ee167701aa2bdcde28e3b3ab1924e\
        9f93affd5e402c83fbc35c9c1ebe0b497b1eb1fb1333d19373e5b7327b1da58ef99c201deee108a885f9d569d3\
        7843e775a3b7a6ebe41c3b08f740482934519c3d82cabd6bc17498ad9d484526b2af59e948b13fc009a7ae97d7\
        ed1dd29de6c5cbeff0deb3f702f4b105873857f2fa74515cca70f8453dafb5bd7104a15ad9259826c47c17ac69\
        a50cb6280457aa7ae6a4dcf359618816b665a5949609815ffb3b9a5ca3db6941ab0faba197ed8870640a0714bc\
        c395db140d3ab37df75a5c13af7fe4d8b784d8594a59a664cb01ecb6910be6712b1f42d802886f52cd61fc24a8\
        13d04b79a53bf7bd7c5a328606a507d61c98e40551ed0923be10c7927cfc334b9a72486dfcdf217395cf771132\
        201a0c1814249a0edd7771933fa5a194818792f12eb0aed7c164b6a95c3786ff646361d752e#10000",
        Greater,
    );

    let sqrt_3_f32 = Float::sqrt_3_prec(u64::from(f32::MANTISSA_DIGITS)).0;
    assert_eq!(sqrt_3_f32.to_string(), "1.7320508");
    assert_eq!(to_hex_string(&sqrt_3_f32), "0x1.bb67ae#24");
    assert_eq!(sqrt_3_f32, f32::SQRT_3);

    let sqrt_3_f64 = Float::sqrt_3_prec(u64::from(f64::MANTISSA_DIGITS)).0;
    assert_eq!(sqrt_3_f64.to_string(), "1.7320508075688772");
    assert_eq!(to_hex_string(&sqrt_3_f64), "0x1.bb67ae8584caa#53");
    assert_eq!(sqrt_3_f64, f64::SQRT_3);
}

fn test_sqrt_3_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::sqrt_3_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_x, rug_o) = rug_sqrt_3_prec_round(prec, rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_x)),
            ComparableFloatRef(&x)
        );
        assert_eq!(rug_o, o);
    }
}

#[test]
pub fn test_sqrt_3_prec_round() {
    test_sqrt_3_prec_round_helper(1, Floor, "1.0", "0x1.0#1", Less);
    test_sqrt_3_prec_round_helper(1, Ceiling, "2.0", "0x2.0#1", Greater);
    test_sqrt_3_prec_round_helper(1, Down, "1.0", "0x1.0#1", Less);
    test_sqrt_3_prec_round_helper(1, Up, "2.0", "0x2.0#1", Greater);
    test_sqrt_3_prec_round_helper(1, Nearest, "2.0", "0x2.0#1", Greater);

    test_sqrt_3_prec_round_helper(2, Floor, "1.5", "0x1.8#2", Less);
    test_sqrt_3_prec_round_helper(2, Ceiling, "2.0", "0x2.0#2", Greater);
    test_sqrt_3_prec_round_helper(2, Down, "1.5", "0x1.8#2", Less);
    test_sqrt_3_prec_round_helper(2, Up, "2.0", "0x2.0#2", Greater);
    test_sqrt_3_prec_round_helper(2, Nearest, "1.5", "0x1.8#2", Less);

    test_sqrt_3_prec_round_helper(3, Floor, "1.5", "0x1.8#3", Less);
    test_sqrt_3_prec_round_helper(3, Ceiling, "1.8", "0x1.c#3", Greater);
    test_sqrt_3_prec_round_helper(3, Down, "1.5", "0x1.8#3", Less);
    test_sqrt_3_prec_round_helper(3, Up, "1.8", "0x1.c#3", Greater);
    test_sqrt_3_prec_round_helper(3, Nearest, "1.8", "0x1.c#3", Greater);

    test_sqrt_3_prec_round_helper(4, Floor, "1.6", "0x1.a#4", Less);
    test_sqrt_3_prec_round_helper(4, Ceiling, "1.8", "0x1.c#4", Greater);
    test_sqrt_3_prec_round_helper(4, Down, "1.6", "0x1.a#4", Less);
    test_sqrt_3_prec_round_helper(4, Up, "1.8", "0x1.c#4", Greater);
    test_sqrt_3_prec_round_helper(4, Nearest, "1.8", "0x1.c#4", Greater);

    test_sqrt_3_prec_round_helper(5, Floor, "1.7", "0x1.b#5", Less);
    test_sqrt_3_prec_round_helper(5, Ceiling, "1.75", "0x1.c#5", Greater);
    test_sqrt_3_prec_round_helper(5, Down, "1.7", "0x1.b#5", Less);
    test_sqrt_3_prec_round_helper(5, Up, "1.75", "0x1.c#5", Greater);
    test_sqrt_3_prec_round_helper(5, Nearest, "1.75", "0x1.c#5", Greater);

    test_sqrt_3_prec_round_helper(6, Floor, "1.72", "0x1.b8#6", Less);
    test_sqrt_3_prec_round_helper(6, Ceiling, "1.75", "0x1.c0#6", Greater);
    test_sqrt_3_prec_round_helper(6, Down, "1.72", "0x1.b8#6", Less);
    test_sqrt_3_prec_round_helper(6, Up, "1.75", "0x1.c0#6", Greater);
    test_sqrt_3_prec_round_helper(6, Nearest, "1.72", "0x1.b8#6", Less);

    test_sqrt_3_prec_round_helper(7, Floor, "1.72", "0x1.b8#7", Less);
    test_sqrt_3_prec_round_helper(7, Ceiling, "1.73", "0x1.bc#7", Greater);
    test_sqrt_3_prec_round_helper(7, Down, "1.72", "0x1.b8#7", Less);
    test_sqrt_3_prec_round_helper(7, Up, "1.73", "0x1.bc#7", Greater);
    test_sqrt_3_prec_round_helper(7, Nearest, "1.73", "0x1.bc#7", Greater);

    test_sqrt_3_prec_round_helper(8, Floor, "1.727", "0x1.ba#8", Less);
    test_sqrt_3_prec_round_helper(8, Ceiling, "1.734", "0x1.bc#8", Greater);
    test_sqrt_3_prec_round_helper(8, Down, "1.727", "0x1.ba#8", Less);
    test_sqrt_3_prec_round_helper(8, Up, "1.734", "0x1.bc#8", Greater);
    test_sqrt_3_prec_round_helper(8, Nearest, "1.734", "0x1.bc#8", Greater);

    test_sqrt_3_prec_round_helper(9, Floor, "1.73", "0x1.bb#9", Less);
    test_sqrt_3_prec_round_helper(9, Ceiling, "1.734", "0x1.bc#9", Greater);
    test_sqrt_3_prec_round_helper(9, Down, "1.73", "0x1.bb#9", Less);
    test_sqrt_3_prec_round_helper(9, Up, "1.734", "0x1.bc#9", Greater);
    test_sqrt_3_prec_round_helper(9, Nearest, "1.73", "0x1.bb#9", Less);

    test_sqrt_3_prec_round_helper(10, Floor, "1.73", "0x1.bb0#10", Less);
    test_sqrt_3_prec_round_helper(10, Ceiling, "1.732", "0x1.bb8#10", Greater);
    test_sqrt_3_prec_round_helper(10, Down, "1.73", "0x1.bb0#10", Less);
    test_sqrt_3_prec_round_helper(10, Up, "1.732", "0x1.bb8#10", Greater);
    test_sqrt_3_prec_round_helper(10, Nearest, "1.732", "0x1.bb8#10", Greater);

    test_sqrt_3_prec_round_helper(
        100,
        Floor,
        "1.732050807568877293527446341505",
        "0x1.bb67ae8584caa73b25742d706#100",
        Less,
    );
    test_sqrt_3_prec_round_helper(
        100,
        Ceiling,
        "1.732050807568877293527446341506",
        "0x1.bb67ae8584caa73b25742d708#100",
        Greater,
    );
    test_sqrt_3_prec_round_helper(
        100,
        Down,
        "1.732050807568877293527446341505",
        "0x1.bb67ae8584caa73b25742d706#100",
        Less,
    );
    test_sqrt_3_prec_round_helper(
        100,
        Up,
        "1.732050807568877293527446341506",
        "0x1.bb67ae8584caa73b25742d708#100",
        Greater,
    );
    test_sqrt_3_prec_round_helper(
        100,
        Nearest,
        "1.732050807568877293527446341506",
        "0x1.bb67ae8584caa73b25742d708#100",
        Greater,
    );
}

#[test]
#[should_panic]
fn sqrt_3_prec_round_fail_1() {
    Float::sqrt_3_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn sqrt_3_prec_round_fail_2() {
    Float::sqrt_3_prec_round(1, Exact);
}

#[test]
#[should_panic]
fn sqrt_3_prec_round_fail_3() {
    Float::sqrt_3_prec_round(1000, Exact);
}

#[test]
fn sqrt_3_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (sqrt_3, o) = Float::sqrt_3_prec(prec);
        assert!(sqrt_3.is_valid());
        assert_eq!(sqrt_3.get_prec(), Some(prec));
        assert_eq!(sqrt_3.get_exponent(), Some(if prec == 1 { 2 } else { 1 }));
        assert_ne!(o, Equal);
        if o == Less {
            let (sqrt_3_alt, o_alt) = Float::sqrt_3_prec_round(prec, Ceiling);
            let mut next_upper = sqrt_3.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(sqrt_3_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !sqrt_3.is_power_of_2() {
            let (sqrt_3_alt, o_alt) = Float::sqrt_3_prec_round(prec, Floor);
            let mut next_lower = sqrt_3.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(sqrt_3_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
        let (sqrt_3_alt, o_alt) = Float::sqrt_3_prec_round(prec, Nearest);
        assert_eq!(ComparableFloatRef(&sqrt_3_alt), ComparableFloatRef(&sqrt_3));
        assert_eq!(o_alt, o);

        let (rug_sqrt_3, rug_o) =
            rug_sqrt_3_prec_round(prec, rug_round_try_from_rounding_mode(Nearest).unwrap());
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_sqrt_3)),
            ComparableFloatRef(&sqrt_3)
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn sqrt_3_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (sqrt_3, o) = Float::sqrt_3_prec_round(prec, rm);
        assert!(sqrt_3.is_valid());
        assert_eq!(sqrt_3.get_prec(), Some(prec));
        let expected_exponent = match (prec, rm) {
            (1, Ceiling | Up | Nearest) | (2, Ceiling | Up) => 2,
            _ => 1,
        };
        assert_eq!(sqrt_3.get_exponent(), Some(expected_exponent));
        assert_ne!(o, Equal);
        if o == Less {
            let (sqrt_3_alt, o_alt) = Float::sqrt_3_prec_round(prec, Ceiling);
            let mut next_upper = sqrt_3.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(sqrt_3_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !sqrt_3.is_power_of_2() {
            let (sqrt_3_alt, o_alt) = Float::sqrt_3_prec_round(prec, Floor);
            let mut next_lower = sqrt_3.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(sqrt_3_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_sqrt_3, rug_o) = rug_sqrt_3_prec_round(prec, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_sqrt_3)),
                ComparableFloatRef(&sqrt_3)
            );
            assert_eq!(rug_o, o);
        }
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_panic!(Float::sqrt_3_prec_round(prec, Exact));
    });

    test_constant(Float::sqrt_3_prec_round, 10000);
}
