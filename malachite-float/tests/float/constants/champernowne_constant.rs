// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::apply_fn_to_primitive_floats;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::ChampernowneConstant;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_gen_var_31, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::float::constants::champernowne_constant::*;
use malachite_float::test_util::common::{test_constant, to_hex_string};
use malachite_float::test_util::float::constants::digit_constants::*;
use malachite_float::test_util::generators::{
    unsigned_pair_gen_var_51, unsigned_unsigned_rounding_mode_triple_gen_var_10,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};

fn test_champernowne_constant_base_prec_helper(
    base: u64,
    prec: u64,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::champernowne_constant_base_prec(base, prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = champernowne_constant_base_prec_round_naive(base, prec, Nearest);
    assert_eq!(x, x_alt);
    assert_eq!(o, o_alt);
}

#[test]
fn test_champernowne_constant_base_prec() {
    test_champernowne_constant_base_prec_helper(
        10,
        100,
        "0.12345678910111213141516171819207",
        "0x0.1f9add37a88fe81c1a98fb84de#100",
        Greater,
    );
    test_champernowne_constant_base_prec_helper(
        2,
        100,
        "0.86224012586805457155779028324965",
        "0x0.dcbbc4d5e6f7c2329d2b6be34#100",
        Greater,
    );
    test_champernowne_constant_base_prec_helper(
        3,
        100,
        "0.59895816753843399250017221792911",
        "0x0.9955528d3fe9e7128c2dbcb82#100",
        Less,
    );
    test_champernowne_constant_base_prec_helper(
        4,
        100,
        "0.42611111111111106576455657142028",
        "0x0.6d159e26af37bd04524d455660#100",
        Greater,
    );
    test_champernowne_constant_base_prec_helper(
        5,
        100,
        "0.31073611111111111111111111111114",
        "0x0.4f8c66dae88fd0ab1f2cd414f0#100",
        Greater,
    );
    test_champernowne_constant_base_prec_helper(
        6,
        100,
        "0.23986268581506676744771982867220",
        "0x0.3d67a4171b3f50ceda8a8275c4#100",
        Less,
    );
    test_champernowne_constant_base_prec_helper(
        7,
        100,
        "0.19443553508624052147584009308287",
        "0x0.31c686f8602b7d0383e0cfe8f4#100",
        Less,
    );
    test_champernowne_constant_base_prec_helper(
        8,
        100,
        "0.16326481210521679736709498614267",
        "0x0.29cbb9049459869c7a08a49a8c#100",
        Greater,
    );
    test_champernowne_constant_base_prec_helper(
        9,
        100,
        "0.14062497611969678247966900893567",
        "0x0.23ffff996f54353fd1d7a62f44#100",
        Greater,
    );
    test_champernowne_constant_base_prec_helper(
        11,
        100,
        "0.10999999996074151907704705547499",
        "0x0.1c28f5c26431e14773a1301afc#100",
        Greater,
    );
    test_champernowne_constant_base_prec_helper(
        12,
        100,
        "0.099173553717641915700526025243643",
        "0x0.19637021d86a3fff4a343f44ca#100",
        Greater,
    );
    test_champernowne_constant_base_prec_helper(
        13,
        100,
        "0.090277777777734303758808704490754",
        "0x0.171c71c71c658a7a57b8fb74d6#100",
        Less,
    );
    test_champernowne_constant_base_prec_helper(
        14,
        100,
        "0.082840236686389258763040020838405",
        "0x0.1535048b5c66a5baab2acc5ac6#100",
        Less,
    );
    test_champernowne_constant_base_prec_helper(
        15,
        100,
        "0.076530612244897924601788946896970",
        "0x0.1397829cbc14e362bad9ee240c#100",
        Less,
    );
    test_champernowne_constant_base_prec_helper(
        16,
        100,
        "0.071111111111111110236506352381062",
        "0x0.123456789abcdef10111213142#100",
        Greater,
    );
}

fn test_champernowne_constant_base_prec_round_helper(
    base: u64,
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::champernowne_constant_base_prec_round(base, prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = champernowne_constant_base_prec_round_naive(base, prec, rm);
    assert_eq!(x, x_alt);
    assert_eq!(o, o_alt);
}

#[test]
fn test_champernowne_constant_base_prec_round() {
    test_champernowne_constant_base_prec_round_helper(
        2,
        100,
        Floor,
        "0.86224012586805457155779028324886",
        "0x0.dcbbc4d5e6f7c2329d2b6be33#100",
        Less,
    );
    test_champernowne_constant_base_prec_round_helper(
        2,
        100,
        Ceiling,
        "0.86224012586805457155779028324965",
        "0x0.dcbbc4d5e6f7c2329d2b6be34#100",
        Greater,
    );
    test_champernowne_constant_base_prec_round_helper(
        2,
        100,
        Down,
        "0.86224012586805457155779028324886",
        "0x0.dcbbc4d5e6f7c2329d2b6be33#100",
        Less,
    );
    test_champernowne_constant_base_prec_round_helper(
        2,
        100,
        Up,
        "0.86224012586805457155779028324965",
        "0x0.dcbbc4d5e6f7c2329d2b6be34#100",
        Greater,
    );
    test_champernowne_constant_base_prec_round_helper(
        2,
        100,
        Nearest,
        "0.86224012586805457155779028324965",
        "0x0.dcbbc4d5e6f7c2329d2b6be34#100",
        Greater,
    );
    test_champernowne_constant_base_prec_round_helper(
        10,
        100,
        Floor,
        "0.12345678910111213141516171819197",
        "0x0.1f9add37a88fe81c1a98fb84dc#100",
        Less,
    );
    test_champernowne_constant_base_prec_round_helper(
        10,
        100,
        Ceiling,
        "0.12345678910111213141516171819207",
        "0x0.1f9add37a88fe81c1a98fb84de#100",
        Greater,
    );
    test_champernowne_constant_base_prec_round_helper(
        10,
        100,
        Down,
        "0.12345678910111213141516171819197",
        "0x0.1f9add37a88fe81c1a98fb84dc#100",
        Less,
    );
    test_champernowne_constant_base_prec_round_helper(
        10,
        100,
        Up,
        "0.12345678910111213141516171819207",
        "0x0.1f9add37a88fe81c1a98fb84de#100",
        Greater,
    );
    test_champernowne_constant_base_prec_round_helper(
        10,
        100,
        Nearest,
        "0.12345678910111213141516171819207",
        "0x0.1f9add37a88fe81c1a98fb84de#100",
        Greater,
    );
}

// Successive Floor approximations must be bit-prefixes of one another, in every base.
#[test]
fn test_champernowne_constant_base_prefixes() {
    for base in 2..=16 {
        test_constant(
            |prec, rm| Float::champernowne_constant_base_prec_round(base, prec, rm),
            100,
        );
    }
}

#[test]
#[should_panic]
fn champernowne_constant_base_prec_round_fail_1() {
    Float::champernowne_constant_base_prec_round(10, 0, Floor);
}

#[test]
#[should_panic]
fn champernowne_constant_base_prec_round_fail_2() {
    Float::champernowne_constant_base_prec_round(10, 100, Exact);
}

#[test]
#[should_panic]
fn champernowne_constant_base_prec_round_fail_3() {
    Float::champernowne_constant_base_prec_round(1, 100, Floor);
}

#[test]
#[should_panic]
fn champernowne_constant_base_prec_round_fail_4() {
    Float::champernowne_constant_base_prec_round(0, 100, Floor);
}

#[test]
#[should_panic]
fn champernowne_constant_base_prec_fail_1() {
    Float::champernowne_constant_base_prec(10, 0);
}

#[test]
#[should_panic]
fn champernowne_constant_base_prec_fail_2() {
    Float::champernowne_constant_base_prec(1, 100);
}

#[test]
fn champernowne_constant_base_prec_properties() {
    unsigned_pair_gen_var_51().test_properties(|(base, prec)| {
        let (x, o) = Float::champernowne_constant_base_prec(base, prec);
        assert!(x.is_valid());
        assert_eq!(x.get_prec(), Some(prec));
        // The constant is irrational, so no precision can represent it exactly.
        assert_ne!(o, Equal);
        if o == Less {
            let (x_alt, o_alt) = Float::champernowne_constant_base_prec_round(base, prec, Ceiling);
            let mut next_upper = x.clone();
            next_upper.increment();
            assert_eq!(ComparableFloat(x_alt), ComparableFloat(next_upper));
            assert_eq!(o_alt, Greater);
        } else {
            let (x_alt, o_alt) = Float::champernowne_constant_base_prec_round(base, prec, Floor);
            let mut next_lower = x.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(x_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
        let (x_alt, o_alt) = Float::champernowne_constant_base_prec_round(base, prec, Nearest);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
        assert_eq!(o_alt, o);

        let (x_alt, o_alt) = champernowne_constant_base_prec_round_naive(base, prec, Nearest);
        assert_eq!(x, x_alt);
        assert_eq!(o, o_alt);
    });
}

#[test]
fn champernowne_constant_base_prec_round_properties() {
    unsigned_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(base, prec, rm)| {
        let (x, o) = Float::champernowne_constant_base_prec_round(base, prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.get_prec(), Some(prec));
        assert_ne!(o, Equal);
        // The constant is positive, so `Down` is `Floor` and `Up` is `Ceiling`.
        match rm {
            Floor | Down => assert_eq!(o, Less),
            Ceiling | Up => assert_eq!(o, Greater),
            Nearest => {}
            Exact => unreachable!(),
        }
        let (x_alt, o_alt) = Float::champernowne_constant_base_prec_round(
            base,
            prec,
            if o == Less { Floor } else { Ceiling },
        );
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
        assert_eq!(o_alt, o);

        let (x_alt, o_alt) = champernowne_constant_base_prec_round_naive(base, prec, rm);
        assert_eq!(x, x_alt);
        assert_eq!(o, o_alt);
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_champernowne_constant_base() {
    fn test<T: PrimitiveFloat>(base: u64, out: T)
    where
        Float: PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_champernowne_constant_base::<T>(base)),
            NiceFloat(out)
        );
    }
    test::<f32>(2, 0.86224014);
    test::<f32>(3, 0.5989582);
    test::<f32>(4, 0.4261111);
    test::<f32>(5, 0.31073612);
    test::<f32>(6, 0.23986268);
    test::<f32>(7, 0.19443554);
    test::<f32>(8, 0.16326481);
    test::<f32>(9, 0.14062497);
    test::<f32>(10, 0.12345679);
    test::<f32>(11, 0.11);
    test::<f32>(12, 0.09917355);
    test::<f32>(13, 0.090277776);
    test::<f32>(14, 0.082840234);
    test::<f32>(15, 0.07653061);
    test::<f32>(16, 0.07111111);
    test::<f32>(62, 0.016662188);
    test::<f32>(1000, 0.001002003);
    test::<f32>(18446744073709551615, 5.421011e-20);

    test::<f64>(2, 0.8622401258680545);
    test::<f64>(3, 0.598958167538434);
    test::<f64>(4, 0.42611111111111105);
    test::<f64>(5, 0.3107361111111111);
    test::<f64>(6, 0.23986268581506676);
    test::<f64>(7, 0.19443553508624054);
    test::<f64>(8, 0.1632648121052168);
    test::<f64>(9, 0.1406249761196968);
    test::<f64>(10, 0.12345678910111213);
    test::<f64>(11, 0.10999999996074152);
    test::<f64>(12, 0.09917355371764192);
    test::<f64>(13, 0.0902777777777343);
    test::<f64>(14, 0.08284023668638926);
    test::<f64>(15, 0.07653061224489792);
    test::<f64>(16, 0.07111111111111111);
    test::<f64>(62, 0.0166621875839828);
    test::<f64>(1000, 0.001002003004005006);
    test::<f64>(18446744073709551615, 5.421010862427522e-20);
}

#[test]
#[should_panic]
fn primitive_float_champernowne_constant_base_fail_1() {
    primitive_float_champernowne_constant_base::<f32>(1);
}

#[test]
#[should_panic]
fn primitive_float_champernowne_constant_base_fail_2() {
    primitive_float_champernowne_constant_base::<f64>(0);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_champernowne_constant_base_properties_helper<T: PrimitiveFloat>()
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    unsigned_gen_var_31::<u64>().test_properties(|base| {
        let x = primitive_float_champernowne_constant_base::<T>(base);
        // The constant lies in [1/base, 1), so it is always finite, positive, and normal.
        assert!(x.is_finite());
        assert!(x > T::ZERO);
        assert!(x < T::ONE);
        // Computing at a much higher precision and rounding once must give the same answer, which
        // is what correct rounding means.
        let (y, _) = Float::champernowne_constant_base_prec(base, 200);
        assert_eq!(NiceFloat(x), NiceFloat(T::rounding_from(&y, Nearest).0));
    });
}

#[test]
fn primitive_float_champernowne_constant_base_properties() {
    apply_fn_to_primitive_floats!(primitive_float_champernowne_constant_base_properties_helper);
}

fn test_champernowne_constant_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::champernowne_constant_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    // The base-10 specialization must agree with the general function at base 10.
    let (x_alt, o_alt) = Float::champernowne_constant_base_prec(10, prec);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);

    let (x_alt, o_alt) = champernowne_constant_base_prec_round_naive(10, prec, Nearest);
    assert_eq!(x, x_alt);
    assert_eq!(o, o_alt);
}

#[test]
fn test_champernowne_constant_prec() {
    test_champernowne_constant_prec_helper(1, "0.12", "0x0.2#1", Greater);
    test_champernowne_constant_prec_helper(2, "0.12", "0x0.2#2", Greater);
    test_champernowne_constant_prec_helper(3, "0.12", "0x0.20#3", Greater);
    test_champernowne_constant_prec_helper(4, "0.125", "0x0.20#4", Greater);
    test_champernowne_constant_prec_helper(5, "0.125", "0x0.20#5", Greater);
    test_champernowne_constant_prec_helper(10, "0.12341", "0x0.1f98#10", Less);
    test_champernowne_constant_prec_helper(
        100,
        "0.12345678910111213141516171819207",
        "0x0.1f9add37a88fe81c1a98fb84de#100",
        Greater,
    );
    test_champernowne_constant_prec_helper(
        1000,
        "0.123456789101112131415161718192021222324252627282930313233343536373839404142434445464748\
        495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293\
        949596979899100101102103104105106107108109110111112113114115116117118119120121122123124125\
        126127128129130131132133134135136137",
        "0x0.1f9add37a88fe81c1a98fb84dd03ffd2a0e956d9ab6c0a76231c1609c4d8baceb0a5f71dc5c87c7adb2b1\
        aab60892e7095a4bdff939e440fce4d283b14ca6696866323a94c229109deac0c34acd009079b01b539f7609ec\
        2aa96c766936f327a27f40d7b4a9a57f14addee830577afa7dd7620f4bbd34d89e400ce07470#1000",
        Greater,
    );
    test_champernowne_constant_prec_helper(
        10000,
        "0.123456789101112131415161718192021222324252627282930313233343536373839404142434445464748\
        495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293\
        949596979899100101102103104105106107108109110111112113114115116117118119120121122123124125\
        126127128129130131132133134135136137138139140141142143144145146147148149150151152153154155\
        156157158159160161162163164165166167168169170171172173174175176177178179180181182183184185\
        186187188189190191192193194195196197198199200201202203204205206207208209210211212213214215\
        216217218219220221222223224225226227228229230231232233234235236237238239240241242243244245\
        246247248249250251252253254255256257258259260261262263264265266267268269270271272273274275\
        276277278279280281282283284285286287288289290291292293294295296297298299300301302303304305\
        306307308309310311312313314315316317318319320321322323324325326327328329330331332333334335\
        336337338339340341342343344345346347348349350351352353354355356357358359360361362363364365\
        366367368369370371372373374375376377378379380381382383384385386387388389390391392393394395\
        396397398399400401402403404405406407408409410411412413414415416417418419420421422423424425\
        426427428429430431432433434435436437438439440441442443444445446447448449450451452453454455\
        456457458459460461462463464465466467468469470471472473474475476477478479480481482483484485\
        486487488489490491492493494495496497498499500501502503504505506507508509510511512513514515\
        516517518519520521522523524525526527528529530531532533534535536537538539540541542543544545\
        546547548549550551552553554555556557558559560561562563564565566567568569570571572573574575\
        576577578579580581582583584585586587588589590591592593594595596597598599600601602603604605\
        606607608609610611612613614615616617618619620621622623624625626627628629630631632633634635\
        636637638639640641642643644645646647648649650651652653654655656657658659660661662663664665\
        666667668669670671672673674675676677678679680681682683684685686687688689690691692693694695\
        696697698699700701702703704705706707708709710711712713714715716717718719720721722723724725\
        726727728729730731732733734735736737738739740741742743744745746747748749750751752753754755\
        756757758759760761762763764765766767768769770771772773774775776777778779780781782783784785\
        786787788789790791792793794795796797798799800801802803804805806807808809810811812813814815\
        816817818819820821822823824825826827828829830831832833834835836837838839840841842843844845\
        846847848849850851852853854855856857858859860861862863864865866867868869870871872873874875\
        876877878879880881882883884885886887888889890891892893894895896897898899900901902903904905\
        906907908909910911912913914915916917918919920921922923924925926927928929930931932933934935\
        936937938939940941942943944945946947948949950951952953954955956957958959960961962963964965\
        966967968969970971972973974975976977978979980981982983984985986987988989990991992993994995\
        996997998999100010011002100310041005100610071008100910101011101210131014101510161017101810\
        191020102110221023102410251026102710281029101",
        "0x0.1f9add37a88fe81c1a98fb84dd03ffd2a0e956d9ab6c0a76231c1609c4d8baceb0a5f71dc5c87c7adb2b1\
        aab60892e7095a4bdff939e440fce4d283b14ca6696866323a94c229109deac0c34acd009079b01b539f7609ec\
        2aa96c766936f327a27f40d7b4a9a57f14addee830577afa7dd7620f4bbd34d89e400ce0746ff68445d2c9ddca\
        d1e468b18eddfaadd9a605a4ef399561030f5aea3f8e5c6495271bd236ffa21b5375403321356ac63748e7507d\
        c863e924a3e240ccbcc665e83ed17ae6c80dd5251088febed64e8b8040da60dd9cbf059f5e9eccd8fdfe3b1865\
        a61018a42ec9295c399594aa4c95349d43c3f995a4ee30eb2303583a0baaf6fa8072d0b7da67a256a44c680d43\
        e31fb97d59f325e02eb1b0804149d2ef807a9022e6f19c11ce305c8c927eb7b184fcef03e241fa8b16a39679fe\
        2f7b059cb3c6a5e788be02cc5485629c4f9742013cdf53fc1d1b4d88221a9504b8d0cce5b1b209ae89b2173f8a\
        02f88a556cc5be7ba20da6af1882d594d9897e8a6df77fedc99267d21e6d204619569916413739edf5458ea51c\
        1c8f9183842f5499b5b380b05e8e6ca9dc3f32ac25d34737ff4db7e851c03b4dced529c0f827f2c044665dfb7c\
        57a2338e81357a1005ca95f286a21608db74c7ffb95d7f490b28edc7e93041b3583e42a690d9c9188e27896936\
        dd31d4df5d706d068564fd21ed42163d212b62ed035d9bef15943d3a0baa2d867888bebbc4684985d46ed75fe0\
        11ebbc27e679c990f58aee80dbf2114d28b81de36c9476e538eb4bb0dd05c93aee7600fc63624e9dc1235eeee2\
        771764fef34f5f75efa1675c86862dcd75210b9d8747e90b039594d3428ade5374ccb858bf87e50c1b5f2e0ad3\
        ef80598d07b8b610b6be98ed534042089d2b40225a5fc28cb6537503dd981cc9adf6d452a11dee8de49a89716c\
        9d4eda8be0e65dd14b6208869cf03c63a5442fd0ac592c1432f9579811a22df8c342c0f90e0cc24984b4ed12ae\
        4cdc6225475841369401342ba5982452918172662f5bff6c01e47da7e9e22af500546490c6eca45684e7d5433e\
        70fb6cf42bcdc203af3d48ab5eb190dd197f8238442294dcdf12eb182bc5a1623fae47e909a6f116be32801d62\
        a37a27ff0be8df2caa3a69d701e2990486378ab7e40a1ff912c40cc37a043f1b282c978db8752430551dfc3284\
        ab69406fa6d01e12cec6020ba54aa806921f97692143f9d0972ea1fbf6caba38738138b5c59577127aa8bf147e\
        06154ab7a17fd71e0997d8c52d342bafab7763e590f96dc87376329663156d772f113a5662565d73c7c564cab9\
        952607819b4e021e55f26719f43625028e5c9e116d5a84b449673f587a5866479b0fccf49c6c452658174ebf65\
        5692beadae95497ae00e6d47efcc552e5d2ade18e665bf9f50923a3d94a28f0da03ef95eb8c56b5879111fff67\
        2ba564b1ee50314a7e497c2b81a79296511f9f524a2fdb171fc31b693a65a728474257c77236b59c659bbd0a91\
        5ac3900a65b5df8c1213cce8d9d79e696fd8ed96c0724870e56b3777482bc588adf1e314bf3aa3a98a7ed93f02\
        2197b124a9870865496415c6f5c4e341f135232a99e50d3fa118c86cc53e9d9a613f21e8ab85c8b40cba56de46\
        7399ebf54f11084ff5b4b47e23c3c89cd75e53fe6425dfe47ae7824a925355a7f535a669adb9934d2deaae6eb0\
        40c64d789fb13cb19b80ef02309a7f9ff0fc4b95159437809e8b86e0ec86a2bc29fec1f52f1c#10000",
        Less,
    );

    let x_f32 = Float::champernowne_constant_prec(24).0;
    assert_eq!(x_f32.to_string(), "0.123456791");
    assert_eq!(to_hex_string(&x_f32), "0x0.1f9add4#24");
    assert_eq!(x_f32, f32::CHAMPERNOWNE_CONSTANT);

    let x_f64 = Float::champernowne_constant_prec(53).0;
    assert_eq!(x_f64.to_string(), "0.12345678910111213");
    assert_eq!(to_hex_string(&x_f64), "0x0.1f9add37a88fe8#53");
    assert_eq!(x_f64, f64::CHAMPERNOWNE_CONSTANT);
}

fn test_champernowne_constant_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::champernowne_constant_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = Float::champernowne_constant_base_prec_round(10, prec, rm);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);

    let (x_alt, o_alt) = champernowne_constant_base_prec_round_naive(10, prec, rm);
    assert_eq!(x, x_alt);
    assert_eq!(o, o_alt);
}

#[test]
fn test_champernowne_constant_prec_round() {
    test_champernowne_constant_prec_round_helper(1, Floor, "0.062", "0x0.1#1", Less);
    test_champernowne_constant_prec_round_helper(1, Ceiling, "0.12", "0x0.2#1", Greater);
    test_champernowne_constant_prec_round_helper(1, Down, "0.062", "0x0.1#1", Less);
    test_champernowne_constant_prec_round_helper(1, Up, "0.12", "0x0.2#1", Greater);
    test_champernowne_constant_prec_round_helper(1, Nearest, "0.12", "0x0.2#1", Greater);
    test_champernowne_constant_prec_round_helper(2, Floor, "0.094", "0x0.18#2", Less);
    test_champernowne_constant_prec_round_helper(2, Ceiling, "0.12", "0x0.2#2", Greater);
    test_champernowne_constant_prec_round_helper(2, Down, "0.094", "0x0.18#2", Less);
    test_champernowne_constant_prec_round_helper(2, Up, "0.12", "0x0.2#2", Greater);
    test_champernowne_constant_prec_round_helper(2, Nearest, "0.12", "0x0.2#2", Greater);
    test_champernowne_constant_prec_round_helper(3, Floor, "0.11", "0x0.1c#3", Less);
    test_champernowne_constant_prec_round_helper(3, Ceiling, "0.12", "0x0.20#3", Greater);
    test_champernowne_constant_prec_round_helper(3, Down, "0.11", "0x0.1c#3", Less);
    test_champernowne_constant_prec_round_helper(3, Up, "0.12", "0x0.20#3", Greater);
    test_champernowne_constant_prec_round_helper(3, Nearest, "0.12", "0x0.20#3", Greater);
    test_champernowne_constant_prec_round_helper(4, Floor, "0.117", "0x0.1e#4", Less);
    test_champernowne_constant_prec_round_helper(4, Ceiling, "0.125", "0x0.20#4", Greater);
    test_champernowne_constant_prec_round_helper(4, Down, "0.117", "0x0.1e#4", Less);
    test_champernowne_constant_prec_round_helper(4, Up, "0.125", "0x0.20#4", Greater);
    test_champernowne_constant_prec_round_helper(4, Nearest, "0.125", "0x0.20#4", Greater);
    test_champernowne_constant_prec_round_helper(5, Floor, "0.121", "0x0.1f#5", Less);
    test_champernowne_constant_prec_round_helper(5, Ceiling, "0.125", "0x0.20#5", Greater);
    test_champernowne_constant_prec_round_helper(5, Down, "0.121", "0x0.1f#5", Less);
    test_champernowne_constant_prec_round_helper(5, Up, "0.125", "0x0.20#5", Greater);
    test_champernowne_constant_prec_round_helper(5, Nearest, "0.125", "0x0.20#5", Greater);
    test_champernowne_constant_prec_round_helper(
        100,
        Floor,
        "0.12345678910111213141516171819197",
        "0x0.1f9add37a88fe81c1a98fb84dc#100",
        Less,
    );
    test_champernowne_constant_prec_round_helper(
        100,
        Ceiling,
        "0.12345678910111213141516171819207",
        "0x0.1f9add37a88fe81c1a98fb84de#100",
        Greater,
    );
    test_champernowne_constant_prec_round_helper(
        100,
        Down,
        "0.12345678910111213141516171819197",
        "0x0.1f9add37a88fe81c1a98fb84dc#100",
        Less,
    );
    test_champernowne_constant_prec_round_helper(
        100,
        Up,
        "0.12345678910111213141516171819207",
        "0x0.1f9add37a88fe81c1a98fb84de#100",
        Greater,
    );
    test_champernowne_constant_prec_round_helper(
        100,
        Nearest,
        "0.12345678910111213141516171819207",
        "0x0.1f9add37a88fe81c1a98fb84de#100",
        Greater,
    );
}

#[test]
#[should_panic]
fn champernowne_constant_prec_fail() {
    Float::champernowne_constant_prec(0);
}

#[test]
#[should_panic]
fn champernowne_constant_prec_round_fail_1() {
    Float::champernowne_constant_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn champernowne_constant_prec_round_fail_2() {
    Float::champernowne_constant_prec_round(100, Exact);
}

#[test]
fn champernowne_constant_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (x, o) = Float::champernowne_constant_prec(prec);
        assert!(x.is_valid());
        assert_eq!(x.get_prec(), Some(prec));
        assert_ne!(o, Equal);
        let (x_alt, o_alt) = Float::champernowne_constant_base_prec(10, prec);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn champernowne_constant_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (x, o) = Float::champernowne_constant_prec_round(prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.get_prec(), Some(prec));
        assert_ne!(o, Equal);
        let (x_alt, o_alt) = Float::champernowne_constant_base_prec_round(10, prec, rm);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
        assert_eq!(o_alt, o);
    });
}
