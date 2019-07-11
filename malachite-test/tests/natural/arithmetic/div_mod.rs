use common::test_properties;
use malachite_base::num::arithmetic::traits::{
    CeilingDivAssignNegMod, CeilingDivNegMod, DivAssignMod, DivAssignRem, DivMod, DivRem,
};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::JoinHalves;
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_nz::natural::arithmetic::div_mod::mpn_tdiv_qr;
use malachite_nz::natural::arithmetic::div_mod::{
    limbs_div_mod_by_two_limb, limbs_div_mod_schoolbook, limbs_div_mod_three_limb_by_two_limb,
    limbs_two_limb_inverse_helper,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::{DoubleLimb, Limb};
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::base::{
    pairs_of_unsigneds_var_2, quadruples_of_three_unsigned_vecs_and_unsigned_var_1,
    sextuples_of_limbs_var_1, triples_of_unsigned_vec_var_37,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_positive_natural, pairs_of_natural_and_positive_natural_var_1,
    positive_naturals,
};
use malachite_test::natural::arithmetic::div_mod::rug_ceiling_div_neg_mod;
use num::{BigUint, Integer};
use rug;
use std::str::FromStr;

fn verify_limbs_two_limb_inverse_helper(hi: Limb, lo: Limb, result: Limb) {
    let b = Natural::ONE << Limb::WIDTH;
    let b_cubed_minus_1 = (Natural::ONE << (Limb::WIDTH * 3)) - 1 as Limb;
    let x = Natural::from(DoubleLimb::join_halves(hi, lo));
    //TODO use /
    let expected_result = (&b_cubed_minus_1).div_mod(&x).0 - &b;
    assert_eq!(result, expected_result);
    assert!(b_cubed_minus_1 - (result + b) * &x < x);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_two_limb_inverse_helper() {
    let test = |hi, lo, result| {
        assert_eq!(limbs_two_limb_inverse_helper(hi, lo), result);
        verify_limbs_two_limb_inverse_helper(hi, lo, result);
    };
    // hi_product >= lo
    // hi_product >= lo_product_hi
    test(0x8000_0000, 0, 0xffff_ffff);
    test(0x8000_0000, 123, 0xffff_ffff);
    test(0x8000_0123, 1, 0xffff_fb74);
    test(0xffff_ffff, 0, 1);
    // hi_product < lo
    test(0xffff_ffff, 123, 0);
    test(0xffff_f123, 1, 0xedd);
    test(0xffff_ffff, 0xffff_ffff, 0);
    // hi_product < lo_product_hi
    // !(hi_product > hi || hi_product == hi && lo_product_lo >= lo)
    test(0x8000_0001, 3, 0xffff_fffb);
    // hi_product > hi || hi_product == hi && lo_product_lo >= lo
    test(2325651385, 3907343530, 3636893938);
}

#[test]
#[should_panic]
fn limbs_two_limb_inverse_helper_fail() {
    limbs_two_limb_inverse_helper(0, 10);
}

fn verify_limbs_div_mod_three_limb_by_two_limb(
    n_2: Limb,
    n_1: Limb,
    n_0: Limb,
    d_1: Limb,
    d_0: Limb,
    q: Limb,
    r: DoubleLimb,
) {
    let n = Natural::from_owned_limbs_asc(vec![n_0, n_1, n_2]);
    let d = Natural::from(DoubleLimb::join_halves(d_1, d_0));
    let r = Natural::from(r);
    assert_eq!((&n).div_mod(&d), (Natural::from(q), r.clone()));
    assert!(r < d);
    assert_eq!(q * d + r, n);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_div_mod_three_limb_by_two_limb() {
    let test = |n_2, n_1, n_0, d_1, d_0, q, r| {
        assert_eq!(
            limbs_div_mod_three_limb_by_two_limb(
                n_2,
                n_1,
                n_0,
                d_1,
                d_0,
                limbs_two_limb_inverse_helper(d_1, d_0)
            ),
            (q, r)
        );
        verify_limbs_div_mod_three_limb_by_two_limb(n_2, n_1, n_0, d_1, d_0, q, r);
    };
    // r < d
    // r.upper_half() >= q_0
    test(1, 2, 3, 0x8000_0004, 5, 1, 0x7fff_fffd_ffff_fffe);
    test(2, 0x4000_0000, 4, 0x8000_0000, 0, 4, 0x4000_0000_0000_0004);
    // r >= d
    // r.upper_half() < q_0
    test(
        1614123406,
        3687984980,
        2695202596,
        2258238141,
        1642523191,
        3069918587,
        274277675918877623,
    );
}

fn verify_limbs_div_mod_by_two_limb(
    quotient_limbs_in: &[Limb],
    numerator_limbs_in: &[Limb],
    denominator_limbs: &[Limb],
    quotient_hi: bool,
    quotient_limbs_out: &[Limb],
    numerator_limbs_out: &[Limb],
) {
    let numerator = Natural::from_limbs_asc(numerator_limbs_in);
    let denominator = Natural::from_limbs_asc(denominator_limbs);
    let (expected_quotient, expected_remainder) = (&numerator).div_mod(&denominator);
    let base_quotient_length = numerator_limbs_in.len() - 2;
    let mut quotient_limbs = quotient_limbs_out[..base_quotient_length].to_vec();
    if quotient_hi {
        quotient_limbs.push(1);
    }
    let quotient = Natural::from_owned_limbs_asc(quotient_limbs);
    let remainder = Natural::from_limbs_asc(&numerator_limbs_out[..2]);
    assert_eq!(quotient, expected_quotient);
    assert_eq!(remainder, expected_remainder);
    assert_eq!(
        &quotient_limbs_in[base_quotient_length..],
        &quotient_limbs_out[base_quotient_length..]
    );
    assert_eq!(&numerator_limbs_in[2..], &numerator_limbs_out[2..]);

    assert!(remainder < denominator);
    assert_eq!(quotient * denominator + remainder, numerator);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_div_mod_by_two_limb() {
    let test = |quotient_limbs_in: &[Limb],
                numerator_limbs_in: &[Limb],
                denominator_limbs,
                quotient_hi,
                quotient_limbs_out: &[Limb],
                numerator_limbs_out: &[Limb]| {
        let mut quotient_limbs = quotient_limbs_in.to_vec();
        let mut numerator_limbs = numerator_limbs_in.to_vec();
        assert_eq!(
            limbs_div_mod_by_two_limb(&mut quotient_limbs, &mut numerator_limbs, denominator_limbs),
            quotient_hi
        );
        assert_eq!(quotient_limbs, quotient_limbs_out);
        assert_eq!(numerator_limbs, numerator_limbs_out);
        verify_limbs_div_mod_by_two_limb(
            quotient_limbs_in,
            numerator_limbs_in,
            denominator_limbs,
            quotient_hi,
            &quotient_limbs,
            &numerator_limbs,
        );
    };
    // !most_significant_quotient_limb
    test(&[10], &[1, 2], &[3, 0x8000_0000], false, &[10], &[1, 2]);
    test(
        &[10, 10, 10, 10],
        &[1, 2, 3, 4, 5],
        &[3, 0x8000_0000],
        false,
        &[4294967241, 7, 10, 10],
        &[166, 2147483626, 3, 4, 5],
    );
    // most_significant_quotient_limb
    test(
        &[0, 0],
        &[4142767597, 2922703399, 3921445909],
        &[2952867570, 2530544119],
        true,
        &[2360708771, 0],
        &[3037232599, 1218898013, 3921445909],
    );
}

#[test]
#[should_panic]
fn limbs_div_mod_by_two_limb_fail_1() {
    limbs_div_mod_by_two_limb(&mut [10], &mut [1, 2], &[3, 4]);
}

#[test]
#[should_panic]
fn limbs_div_mod_by_two_limb_fail_2() {
    limbs_div_mod_by_two_limb(&mut [10], &mut [1, 2], &[3, 0x8000_0000, 4]);
}

#[test]
#[should_panic]
fn limbs_div_mod_by_two_limb_fail_3() {
    limbs_div_mod_by_two_limb(&mut [10], &mut [1, 2, 3, 4], &[3, 0x8000_0000]);
}

fn verify_limbs_div_mod_schoolbook(
    quotient_limbs_in: &[Limb],
    numerator_limbs_in: &[Limb],
    denominator_limbs: &[Limb],
    quotient_hi: bool,
    quotient_limbs_out: &[Limb],
    numerator_limbs_out: &[Limb],
) {
    let numerator = Natural::from_limbs_asc(numerator_limbs_in);
    let denominator = Natural::from_limbs_asc(denominator_limbs);
    let (expected_quotient, expected_remainder) = (&numerator).div_mod(&denominator);
    let base_quotient_length = numerator_limbs_in.len() - denominator_limbs.len();
    let mut quotient_limbs = quotient_limbs_out[..base_quotient_length].to_vec();
    if quotient_hi {
        quotient_limbs.push(1);
    }
    let quotient = Natural::from_owned_limbs_asc(quotient_limbs);
    let remainder = Natural::from_limbs_asc(&numerator_limbs_out[..denominator_limbs.len()]);
    assert_eq!(quotient, expected_quotient);
    assert_eq!(remainder, expected_remainder,);
    assert_eq!(
        &quotient_limbs_in[base_quotient_length..],
        &quotient_limbs_out[base_quotient_length..]
    );
    assert!(remainder < denominator);
    assert_eq!(quotient * denominator + remainder, numerator);
}

#[test]
fn test_limbs_div_mod_schoolbook() {
    let test = |quotient_limbs_in: &[Limb],
                numerator_limbs_in: &[Limb],
                denominator_limbs: &[Limb],
                quotient_hi,
                quotient_limbs_out: &[Limb],
                numerator_limbs_out: &[Limb]| {
        let mut quotient_limbs = quotient_limbs_in.to_vec();
        let mut numerator_limbs = numerator_limbs_in.to_vec();
        let inverse = limbs_two_limb_inverse_helper(
            denominator_limbs[denominator_limbs.len() - 1],
            denominator_limbs[denominator_limbs.len() - 2],
        );
        assert_eq!(
            limbs_div_mod_schoolbook(
                &mut quotient_limbs,
                &mut numerator_limbs,
                denominator_limbs,
                inverse
            ),
            quotient_hi
        );
        assert_eq!(quotient_limbs, quotient_limbs_out);
        assert_eq!(numerator_limbs, numerator_limbs_out);
        verify_limbs_div_mod_schoolbook(
            quotient_limbs_in,
            numerator_limbs_in,
            denominator_limbs,
            quotient_hi,
            &quotient_limbs,
            &numerator_limbs,
        );
    };
    #[cfg(feature = "32_bit_limbs")]
    {
        test(
            &[10],
            &[1, 2, 3],
            &[3, 4, 0x8000_0000],
            false,
            &[10],
            &[1, 2, 3],
        );
        test(
            &[10, 10, 10, 10],
            &[1, 2, 3, 4, 5, 6],
            &[3, 4, 0x8000_0000],
            false,
            &[4294967207, 9, 12, 10],
            &[268, 328, 2147483575, 4294967251, 5, 6],
        );
    }
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        test(
            &[10; 17],
            &[
                9995257893114397114,
                9401504468144459131,
                558615837638945228,
                10733662027974786928,
                18295107704289976677,
                1814706268673753787,
                12474943759854623335,
                8814778832826774208,
                9159057654048965906,
                4451260977376821357,
                18241701617364042056,
                6169989192350218482,
                15071965537117101028,
                13509168527678537782,
                12224278653171635329,
                16077066393714953826,
                1433938684868066489,
                13014970036232570373,
                899282336249563956,
                3089487642230339536,
                3787737519477527148,
                16667686214395942740,
                8787122953224574943,
                7841835218775877827,
                9693303502025838409,
                16122224776459879427,
                144327425397945219,
            ],
            &[
                2350654041004706911,
                7834348511584604247,
                12756796070221345724,
                3842923787777653903,
                12373799197090248752,
                9712029403347085570,
                1426676505264168302,
                10586232903332693517,
                8387833601131974459,
                6290888746273553243,
                9503969704425173615,
            ],
            false,
            &[
                89235393247566392,
                5198286616477507104,
                15671556528191444298,
                6642842185819876016,
                1703950202232719208,
                6620591674460885314,
                9897211438557358662,
                12382449603707212210,
                13586842887558233290,
                11884313943008627054,
                3205830138969300059,
                4257812936318957065,
                11084100237971796628,
                13937343901544333624,
                11743372027422931451,
                280132530083052382,
                10,
            ],
            &[
                12688955427180652274,
                7641660693922643933,
                8789985477567049482,
                5698832637416200787,
                14684840547760545685,
                2822100467869581421,
                3557573565928866957,
                4409631974409684922,
                16994214656621423610,
                4513108841166793667,
                9009005527785483287,
                4330767427200269309,
                11409205475757922767,
                12430752173702915207,
                11990819624778098799,
                4145020291153594556,
                7099200056207569977,
                3510167930325480168,
                4731667122118327121,
                10720310942075546738,
                5799804483118787221,
                17268037247251138479,
                13305947798457087249,
                1405091439325174594,
                13072181651983436371,
                16122224776459879427,
                144327425397945219,
            ],
        )
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_mpn_tdiv_qr() {
    let test = |quotient_limbs_in: &[Limb],
                remainder_limbs_in: &[Limb],
                numerator_limbs: &[Limb],
                denominator_limbs: &[Limb],
                quotient_limbs_out: &[Limb],
                remainder_limbs_out: &[Limb]| {
        let mut quotient_limbs = quotient_limbs_in.to_vec();
        let mut remainder_limbs = remainder_limbs_in.to_vec();
        mpn_tdiv_qr(
            &mut quotient_limbs,
            &mut remainder_limbs,
            numerator_limbs,
            denominator_limbs,
        );
        assert_eq!(quotient_limbs, quotient_limbs_out);
        assert_eq!(remainder_limbs, remainder_limbs_out);
    };
    test(
        &[
            10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
        ],
        &[
            10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
        ],
        &[
            9995257893114397114,
            9401504468144459131,
            558615837638945228,
            10733662027974786928,
            18295107704289976677,
            1814706268673753787,
            12474943759854623335,
            8814778832826774208,
            9159057654048965906,
            4451260977376821357,
            18241701617364042056,
            6169989192350218482,
            15071965537117101028,
            13509168527678537782,
            12224278653171635329,
            16077066393714953826,
            1433938684868066489,
            13014970036232570373,
            899282336249563956,
            3089487642230339536,
            3787737519477527148,
            16667686214395942740,
            8787122953224574943,
            7841835218775877827,
            9693303502025838409,
            16122224776459879427,
            144327425397945219,
        ],
        &[
            2350654041004706911,
            7834348511584604247,
            12756796070221345724,
            3842923787777653903,
            12373799197090248752,
            9712029403347085570,
            1426676505264168302,
            10586232903332693517,
            8387833601131974459,
            6290888746273553243,
            9503969704425173615,
        ],
        &[
            89235393247566392,
            5198286616477507104,
            15671556528191444298,
            6642842185819876016,
            1703950202232719208,
            6620591674460885314,
            9897211438557358662,
            12382449603707212210,
            13586842887558233290,
            11884313943008627054,
            3205830138969300059,
            4257812936318957065,
            11084100237971796628,
            13937343901544333624,
            11743372027422931451,
            280132530083052382,
            0,
        ],
        &[
            12688955427180652274,
            7641660693922643933,
            8789985477567049482,
            5698832637416200787,
            14684840547760545685,
            2822100467869581421,
            3557573565928866957,
            4409631974409684922,
            16994214656621423610,
            4513108841166793667,
            9009005527785483287,
            10,
            10,
            10,
            10,
            10,
            10,
        ],
    );
    test(
        &[10, 10, 10],
        &[
            10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10,
        ],
        &[
            748159237152854524,
            14199895651244313572,
            9044210482484213648,
            3880401870711113518,
            1694971440240542063,
            13547801197479934494,
            5244069077418598572,
            17329479401291658084,
            12613311850003558282,
            5618071535926791206,
            16954511293879569524,
            8600749590433482901,
            11708546551548237376,
            10879843710159659952,
            9101678715417935644,
            12126242459863584426,
            17259866272884195621,
            4418382641453775715,
            542305129955142216,
            6563442437678466173,
            12794875758080454756,
            7461769876910639905,
            17925257245127463276,
            5137728719899113924,
            12905981752247605071,
        ],
        &[
            2654882163556630563,
            2047318842992691178,
            17944530594807555614,
            17278864523505748498,
            1160166728089482341,
            18368953657130322418,
            3937719995815345698,
            12007028340444721520,
            1496744539933999053,
            1476923054783110845,
            6551619938265612084,
            16801911333947266527,
            13986495313155597995,
            6571595571877061463,
            10140569634762389822,
            16210530410764331582,
            15172903143228403872,
            5831780706385794192,
            12288937301416472500,
            16224579586702000460,
            14545605105156691376,
            8614987803254853144,
            16629891239728134900,
        ],
        &[17831022488782895576, 14315989140983049585, 0],
        &[
            18140975738986113396,
            16765596268029991308,
            14497740378349400824,
            8834432760455669008,
            2081502095596466916,
            16785570606386467383,
            5299348241512211807,
            17503170383548190207,
            16775442261989831354,
            8131705923782084593,
            266320274487676679,
            6602256474512308593,
            2102043233085822823,
            11614561527212258722,
            17915538208051341722,
            5710195504177465517,
            2094480568485157388,
            14339014023087152780,
            6947889352398323832,
            10985139413433625547,
            12373170520775701923,
            9198039438688117621,
            15475638737141339650,
        ],
    );
    test(
        &[10; 60],
        &[10; 56],
        &[
            14660214196707223375,
            14265972253040120215,
            15506320303100465818,
            17085621003033826581,
            11203337550022453944,
            15493204961705835371,
            5803021083410871755,
            8112917457002746745,
            12663484193891261040,
            1721048899893287199,
            8062187621610464306,
            13431761655884620090,
            7331427712144411262,
            3626934647030185267,
            13231383914073320042,
            11637171044660683638,
            15189928975258171045,
            941827519265124224,
            2992792486091076914,
            2044203374633195985,
            8310380355675814732,
            1677894573715118386,
            1863631713396879617,
            13750903464355877990,
            13561054993991137710,
            6643134394212488277,
            9782189322903525535,
            7987880548748269544,
            17396502810230452231,
            9355336424066456608,
            6974435047841500624,
            4695995454788932008,
            9790410161672155866,
            7324176676989916049,
            14873447357313289350,
            17933513319573948354,
            16221633809094225356,
            1119296061370324791,
            13659405622992751643,
            10536448431317839371,
            15771892335411705715,
            6450515195565208913,
            12583173873673842188,
            8943105588740166659,
            16781237121411387206,
            7355272525679995848,
            8924936502454129260,
            9464007023044637842,
            2392086820925613645,
            6952992660961663836,
            15709161892606831425,
            15961199354349516091,
            8170938350051511007,
            10106337242460916657,
            4519632767875399815,
            13966478644099829332,
            18146666299243951179,
            18001892575388798951,
            17442461326088111501,
            12996149925790510613,
            15125238000270787220,
            13458137050174539117,
            7565676737178758148,
            7820895745333505106,
            18391820881894926862,
            17227107494212736312,
            16170524482788524562,
            18292226432698054709,
            16409124153431213414,
            2622798522164114141,
            2030148142272451724,
            12631034221630749586,
            12521714531249855181,
            4869764655816857917,
            18312880399388298885,
            1881841240505020002,
            16686085102712131293,
            1638984612454565124,
            5980766772519196081,
            14473546029553426533,
            2610255570241349719,
            4121823778233332328,
            15196027812344512481,
            17634932614139407184,
            14566629132274047837,
            6629067916649366603,
            39453246491293667,
            4118307938296638515,
            176389639877922730,
            2385844666265721927,
            14424300909552701177,
            2596064544694255252,
            9262830285738421829,
            8366979142044016136,
            12451088247268499723,
            16456341544263224076,
            405434591376297036,
            5989071471671786526,
            17922319711997177283,
            12402685985480014221,
            11440567647536028583,
            17109382986734751589,
            1165111999013207871,
            9042409351611763515,
            335396288523389342,
            6889397323074150916,
            13998858741906849976,
            15927944587197048898,
            10995067153735213576,
            13255077995174337515,
            11985913648073551062,
            16606199253171990948,
            16615211378568935152,
            13000672060735124358,
        ],
        &[
            6726150808576237754,
            9590776370558469124,
            4613857594775205869,
            5605914158178321857,
            12627075307783464761,
            456502911636413728,
            6201419543988208076,
            12457367465345491402,
            9194484469177303126,
            14469237774454463326,
            8872571916644400618,
            10371861714649740250,
            9551882050917532587,
            1418647961867356190,
            11742587182398063873,
            11015016132415914044,
            8777839015232205587,
            11080046461630228193,
            13740325869131645472,
            17716201322003396844,
            2184375889136968144,
            2744007897878529583,
            10107840174031679018,
            6807210551800087042,
            3927845063936277496,
            4657264236265855475,
            18202437017170404187,
            5332422779150911238,
            15515262280249200267,
            248667350560422394,
            3473467338029486524,
            5450666559053310869,
            9114347711968955703,
            1001965327187909086,
            9391480248060184246,
            9069754537718985217,
            6108113375902101471,
            615335597740998377,
            7341924484422171664,
            7557688311245960406,
            10629369615492290302,
            6551022068682485711,
            13009629572214277263,
            9801266711191462998,
            12475469715378400041,
            16817728089246511388,
            5318131496704799888,
            14034696640350324685,
            173195053797772988,
            9465580662794117123,
            9395502290798332505,
            172507413604644051,
            13462235362634225088,
            9267822876689174860,
            12978933587961252639,
        ],
        &[
            12372756710207599663,
            9737052986771636298,
            16735862446672978006,
            1139195382411501599,
            4025384807176208306,
            10128156782936698507,
            7100085357301525578,
            10639782880668134749,
            3972383448210895518,
            16316091826865092258,
            14638110565144662169,
            17027377005940147919,
            1984424298563015784,
            10943215534705396352,
            4761407742818533080,
            536799158643182373,
            3577912885973196462,
            8426618872156874849,
            13718975316423099691,
            9890119685862829437,
            1661366149680121631,
            18221664832966866708,
            1501909944594354041,
            15664453277583965124,
            3204453056814894230,
            11234664797845870989,
            865170089562739167,
            15036893469165510103,
            9555056751383235767,
            10793253279766963078,
            10975966662822330260,
            6344197561810800775,
            10052816891387114632,
            5489737378772055553,
            3577007843046523907,
            5025363426761413084,
            11669827237042875622,
            15298941946562692234,
            5287362685718508737,
            14167437013528222514,
            108442285706035530,
            12321077902001896155,
            4987860952577552150,
            4822344167562733502,
            5046873607058225743,
            15023457088946801127,
            10073890866526654379,
            9395914048369797781,
            12331509678230261831,
            4207910636930067124,
            13640015182632895728,
            16512336849198622133,
            750194286339711619,
            3343827571253159031,
            1179021970615059386,
            9309853498190567264,
            8323638524074867625,
            2319424490723820181,
            30896532530597901,
            1,
        ],
        &[
            16979197013852036393,
            4534519222829727882,
            5127955051936920534,
            5669732551578654322,
            13787946500638697314,
            2666880029397285003,
            18286001525339884787,
            3747928243980886079,
            5670276194023029484,
            15201258611907138387,
            6046915833599742673,
            13282924752646783062,
            18026143804639821221,
            10186643213552896189,
            17209309200088910354,
            13215180252119768256,
            1246399679408038126,
            4186715523775575401,
            16756959752065842207,
            6600048850655585015,
            4543693866439677976,
            15594233518271892275,
            15247811862837572166,
            6322126320582019533,
            649809830609098083,
            5229876751712742127,
            17719948521867410031,
            10737539927122287433,
            12476905306147178753,
            1539850235988803702,
            13572545877865905325,
            11163694899331373883,
            7882148214994127637,
            8164419266634080608,
            5782587821804107698,
            12155391719814216620,
            8020222143449740150,
            8489927257914490530,
            15688922762526028920,
            207673185831465902,
            13825819490340731785,
            14207999229863934400,
            10163751595898713958,
            17777080404153962435,
            17016927136773389232,
            3820023214020965653,
            1892439588667561762,
            16909683715900311832,
            11919385779232783009,
            11201007117990222527,
            8700983269916503928,
            5034192113764375450,
            12790439085134048151,
            17790018876931315900,
            5953092655978688336,
            10,
        ],
    );
}

#[test]
fn test_div_mod() {
    let test = |u, v, quotient, remainder| {
        let mut x = Natural::from_str(u).unwrap();
        let r = x.div_assign_mod(Natural::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), quotient);

        let mut x = Natural::from_str(u).unwrap();
        let r = x.div_assign_mod(&Natural::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), quotient);

        let (q, r) = Natural::from_str(u)
            .unwrap()
            .div_mod(Natural::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = Natural::from_str(u)
            .unwrap()
            .div_mod(&Natural::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = (&Natural::from_str(u).unwrap()).div_mod(Natural::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = (&Natural::from_str(u).unwrap()).div_mod(&Natural::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let mut x = Natural::from_str(u).unwrap();
        let r = x.div_assign_rem(Natural::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), quotient);

        let mut x = Natural::from_str(u).unwrap();
        let r = x.div_assign_rem(&Natural::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), quotient);

        let (q, r) = Natural::from_str(u)
            .unwrap()
            .div_rem(Natural::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = Natural::from_str(u)
            .unwrap()
            .div_rem(&Natural::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = (&Natural::from_str(u).unwrap()).div_rem(Natural::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = (&Natural::from_str(u).unwrap()).div_rem(&Natural::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = BigUint::from_str(u)
            .unwrap()
            .div_mod_floor(&BigUint::from_str(v).unwrap());
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r.to_string(), remainder);

        let (q, r) = BigUint::from_str(u)
            .unwrap()
            .div_rem(&BigUint::from_str(v).unwrap());
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r.to_string(), remainder);

        let (q, r) = rug::Integer::from_str(u)
            .unwrap()
            .div_rem_floor(rug::Integer::from_str(v).unwrap());
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r.to_string(), remainder);

        let (q, r) = rug::Integer::from_str(u)
            .unwrap()
            .div_rem(rug::Integer::from_str(v).unwrap());
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r.to_string(), remainder);

        //TODO
        /*
        let (q, r) = (
            Natural::from_str(u).unwrap() / v,
            Natural::from_str(u).unwrap() % v,
        );
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r, remainder);
        */
    };
    test("0", "1", "0", "0");
    test("0", "123", "0", "0");
    test("1", "1", "1", "0");
    test("123", "1", "123", "0");
    test("123", "123", "1", "0");
    test("123", "456", "0", "123");
    test("456", "123", "3", "87");
    test("4294967295", "1", "4294967295", "0");
    test("4294967295", "4294967295", "1", "0");
    test("1000000000000", "1", "1000000000000", "0");
    test("1000000000000", "3", "333333333333", "1");
    test("1000000000000", "123", "8130081300", "100");
    test("1000000000000", "4294967295", "232", "3567587560");
    test(
        "1000000000000000000000000",
        "1",
        "1000000000000000000000000",
        "0",
    );
    test(
        "1000000000000000000000000",
        "3",
        "333333333333333333333333",
        "1",
    );
    test(
        "1000000000000000000000000",
        "123",
        "8130081300813008130081",
        "37",
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        "232830643708079",
        "3167723695",
    );
    test(
        "1000000000000000000000000",
        "1234567890987",
        "810000006723",
        "530068894399",
    );
    test(
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0",
        "1234567890987654321234567890987654321",
        "810000006723000055638900467181273922269593923137018654",
        "779655053998040854338961591319296066",
    );
    test(
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0",
        "316049380092839506236049380092839506176",
        "3164062526261718967339454949926851258865601262253979",
        "37816691783627670491375998320948925696",
    );
    test(
        "253640751230376270397812803167",
        "2669936877441",
        "94998781946290113",
        "1520301762334",
    );
    test(
        "3768477692975601",
        "11447376614057827956",
        "0",
        "3768477692975601",
    );
    test(
        "3356605361737854",
        "3081095617839357",
        "1",
        "275509743898497",
    );
    test(
        "1098730198198174614195",
        "953382298040157850476",
        "1",
        "145347900158016763719",
    );
    test(
        "69738658860594537152875081748",
        "69738658860594537152875081748",
        "1",
        "0",
    );
    test(
        "1000000000000000000000000",
        "1000000000000000000000000",
        "1",
        "0",
    );
    test("0", "1000000000000000000000000", "0", "0");
    test("123", "1000000000000000000000000", "0", "123");
}

#[test]
#[should_panic]
fn div_assign_mod_fail() {
    Natural::from(10u32).div_assign_mod(Natural::ZERO);
}

#[test]
#[should_panic]
fn div_assign_mod_ref_fail() {
    Natural::from(10u32).div_assign_mod(&Natural::ZERO);
}

#[test]
#[should_panic]
fn div_mod_fail() {
    Natural::from(10u32).div_mod(Natural::ZERO);
}

#[test]
#[should_panic]
fn div_mod_val_ref_fail() {
    Natural::from(10u32).div_mod(&Natural::ZERO);
}

#[test]
#[should_panic]
fn div_mod_ref_val_fail() {
    (&Natural::from(10u32)).div_mod(Natural::ZERO);
}

#[test]
#[should_panic]
fn div_mod_ref_ref_fail() {
    (&Natural::from(10u32)).div_mod(&Natural::ZERO);
}

#[test]
#[should_panic]
fn div_assign_rem_fail() {
    Natural::from(10u32).div_assign_rem(Natural::ZERO);
}

#[test]
#[should_panic]
fn div_assign_rem_ref_fail() {
    Natural::from(10u32).div_assign_rem(&Natural::ZERO);
}

#[test]
#[should_panic]
fn div_rem_fail() {
    Natural::from(10u32).div_rem(Natural::ZERO);
}

#[test]
#[should_panic]
fn div_rem_val_ref_fail() {
    Natural::from(10u32).div_rem(&Natural::ZERO);
}

#[test]
#[should_panic]
fn div_rem_ref_val_fail() {
    (&Natural::from(10u32)).div_rem(Natural::ZERO);
}

#[test]
#[should_panic]
fn div_rem_ref_ref_fail() {
    (&Natural::from(10u32)).div_rem(&Natural::ZERO);
}

#[test]
fn test_ceiling_div_neg_mod() {
    let test = |u, v, quotient, remainder| {
        let mut x = Natural::from_str(u).unwrap();
        let r = x.ceiling_div_assign_neg_mod(Natural::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), quotient);

        let mut x = Natural::from_str(u).unwrap();
        let r = x.ceiling_div_assign_neg_mod(&Natural::from_str(v).unwrap());
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), quotient);

        let (q, r) = Natural::from_str(u)
            .unwrap()
            .ceiling_div_neg_mod(Natural::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = Natural::from_str(u)
            .unwrap()
            .ceiling_div_neg_mod(&Natural::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) =
            (&Natural::from_str(u).unwrap()).ceiling_div_neg_mod(Natural::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) =
            (&Natural::from_str(u).unwrap()).ceiling_div_neg_mod(&Natural::from_str(v).unwrap());
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), remainder);

        let (q, r) = rug_ceiling_div_neg_mod(
            rug::Integer::from_str(u).unwrap(),
            rug::Integer::from_str(v).unwrap(),
        );
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r.to_string(), remainder);

        //TODO
        /*
        let (q, r) = (
            Natural::from_str(u).unwrap().div_round(v, RoundingMode::Ceiling),
            Natural::from_str(u).unwrap().neg_mod(v),
        );
        assert_eq!(q.to_string(), quotient);
        assert_eq!(r, remainder);
        */
    };
    test("0", "1", "0", "0");
    test("0", "123", "0", "0");
    test("1", "1", "1", "0");
    test("123", "1", "123", "0");
    test("123", "123", "1", "0");
    test("123", "456", "1", "333");
    test("456", "123", "4", "36");
    test("4294967295", "1", "4294967295", "0");
    test("4294967295", "4294967295", "1", "0");
    test("1000000000000", "1", "1000000000000", "0");
    test("1000000000000", "3", "333333333334", "2");
    test("1000000000000", "123", "8130081301", "23");
    test("1000000000000", "4294967295", "233", "727379735");
    test(
        "1000000000000000000000000",
        "1",
        "1000000000000000000000000",
        "0",
    );
    test(
        "1000000000000000000000000",
        "3",
        "333333333333333333333334",
        "2",
    );
    test(
        "1000000000000000000000000",
        "123",
        "8130081300813008130082",
        "86",
    );
    test(
        "1000000000000000000000000",
        "4294967295",
        "232830643708080",
        "1127243600",
    );
    test(
        "1000000000000000000000000",
        "1234567890987",
        "810000006724",
        "704498996588",
    );
    test(
        "10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
         00",
        "1234567890987654321234567890987654321",
        "810000006723000055638900467181273922269593923137018655",
        "454912836989613466895606299668358255",
    );
    test(
        "10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
         00",
        "316049380092839506236049380092839506176",
        "3164062526261718967339454949926851258865601262253980",
        "278232688309211835744673381771890580480",
    );
    test(
        "253640751230376270397812803167",
        "2669936877441",
        "94998781946290114",
        "1149635115107",
    );
    test(
        "3768477692975601",
        "11447376614057827956",
        "1",
        "11443608136364852355",
    );
    test(
        "3356605361737854",
        "3081095617839357",
        "2",
        "2805585873940860",
    );
    test(
        "1098730198198174614195",
        "953382298040157850476",
        "2",
        "808034397882141086757",
    );
    test(
        "69738658860594537152875081748",
        "69738658860594537152875081748",
        "1",
        "0",
    );
    test(
        "1000000000000000000000000",
        "1000000000000000000000000",
        "1",
        "0",
    );
    test("0", "1000000000000000000000000", "0", "0");
    test(
        "123",
        "1000000000000000000000000",
        "1",
        "999999999999999999999877",
    );
}

#[test]
#[should_panic]
fn ceiling_div_assign_neg_mod_fail() {
    Natural::from(10u32).ceiling_div_assign_neg_mod(Natural::ZERO);
}

#[test]
#[should_panic]
fn ceiling_div_assign_neg_mod_ref_fail() {
    Natural::from(10u32).ceiling_div_assign_neg_mod(&Natural::ZERO);
}

#[test]
#[should_panic]
fn ceiling_div_neg_mod_fail() {
    Natural::from(10u32).ceiling_div_neg_mod(Natural::ZERO);
}

#[test]
#[should_panic]
fn ceiling_div_neg_mod_val_ref_fail() {
    Natural::from(10u32).ceiling_div_neg_mod(&Natural::ZERO);
}

#[test]
#[should_panic]
fn ceiling_div_neg_mod_ref_val_fail() {
    (&Natural::from(10u32)).ceiling_div_neg_mod(Natural::ZERO);
}

#[test]
#[should_panic]
fn ceiling_div_neg_mod_ref_ref_fail() {
    (&Natural::from(10u32)).ceiling_div_neg_mod(&Natural::ZERO);
}

#[test]
fn limbs_two_limb_inverse_helper_properties() {
    test_properties(pairs_of_unsigneds_var_2, |&(hi, lo)| {
        let result = limbs_two_limb_inverse_helper(hi, lo);
        verify_limbs_two_limb_inverse_helper(hi, lo, result);
    });
}

#[test]
fn limbs_div_mod_three_limb_by_two_limb_properties() {
    test_properties(
        sextuples_of_limbs_var_1,
        |&(n_2, n_1, n_0, d_1, d_0, inverse)| {
            let (q, r) = limbs_div_mod_three_limb_by_two_limb(n_2, n_1, n_0, d_1, d_0, inverse);
            verify_limbs_div_mod_three_limb_by_two_limb(n_2, n_1, n_0, d_1, d_0, q, r);
        },
    );
}

#[test]
fn limbs_div_mod_by_two_limb_properties() {
    test_properties(
        triples_of_unsigned_vec_var_37,
        |(quotient_limbs_in, numerator_limbs_in, denominator_limbs)| {
            let mut quotient_limbs = quotient_limbs_in.clone();
            let mut numerator_limbs = numerator_limbs_in.clone();
            let quotient_hi = limbs_div_mod_by_two_limb(
                &mut quotient_limbs,
                &mut numerator_limbs,
                &denominator_limbs,
            );
            verify_limbs_div_mod_by_two_limb(
                &quotient_limbs_in,
                &numerator_limbs_in,
                &denominator_limbs,
                quotient_hi,
                &quotient_limbs,
                &numerator_limbs,
            );
        },
    );
}

#[test]
fn limbs_div_mod_schoolbook_properties() {
    test_properties(
        quadruples_of_three_unsigned_vecs_and_unsigned_var_1,
        |(ref quotient_limbs_in, ref numerator_limbs_in, ref denominator_limbs, inverse)| {
            let mut quotient_limbs = quotient_limbs_in.clone();
            let mut numerator_limbs = numerator_limbs_in.clone();
            let quotient_hi = limbs_div_mod_schoolbook(
                &mut quotient_limbs,
                &mut numerator_limbs,
                denominator_limbs,
                *inverse,
            );
            verify_limbs_div_mod_schoolbook(
                quotient_limbs_in,
                numerator_limbs_in,
                denominator_limbs,
                quotient_hi,
                &quotient_limbs,
                &numerator_limbs,
            );
        },
    );
}

fn div_mod_properties_helper(x: &Natural, y: &Natural) {
    let mut mut_x = x.clone();
    let remainder = mut_x.div_assign_mod(y);
    assert!(mut_x.is_valid());
    assert!(remainder.is_valid());
    let quotient = mut_x;

    let mut mut_x = x.clone();
    let remainder_alt = mut_x.div_assign_mod(y.clone());
    let quotient_alt = mut_x;
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.div_mod(y);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.div_mod(y.clone());
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.clone().div_mod(y);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.clone().div_mod(y.clone());
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let mut quotient_alt = x.clone();
    let remainder_alt = quotient_alt.div_assign_rem(y);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let mut quotient_alt = x.clone();
    let remainder_alt = quotient_alt.div_assign_rem(y.clone());
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.div_rem(y);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.div_rem(y.clone());
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.clone().div_rem(y);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.clone().div_rem(y.clone());
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    //TODO
    /*
    let (quotient_alt, remainder_alt) = (x / y, x % y);
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);
    */

    let (num_quotient, num_remainder) = natural_to_biguint(x).div_mod_floor(&natural_to_biguint(y));
    assert_eq!(biguint_to_natural(&num_quotient), quotient);
    assert_eq!(biguint_to_natural(&num_remainder), remainder);

    let (num_quotient, num_remainder) = natural_to_biguint(x).div_rem(&natural_to_biguint(y));
    assert_eq!(biguint_to_natural(&num_quotient), quotient);
    assert_eq!(biguint_to_natural(&num_remainder), remainder);

    let (rug_quotient, rug_remainder) =
        natural_to_rug_integer(x).div_rem_floor(natural_to_rug_integer(y));
    assert_eq!(rug_integer_to_natural(&rug_quotient), quotient);
    assert_eq!(rug_integer_to_natural(&rug_remainder), remainder);

    let (rug_quotient, rug_remainder) =
        natural_to_rug_integer(x).div_rem(natural_to_rug_integer(y));
    assert_eq!(rug_integer_to_natural(&rug_quotient), quotient);
    assert_eq!(rug_integer_to_natural(&rug_remainder), remainder);

    assert!(remainder < *y);
    assert_eq!(quotient * y + remainder, *x);
}

#[test]
fn div_mod_properties() {
    test_properties(pairs_of_natural_and_positive_natural, |&(ref x, ref y)| {
        div_mod_properties_helper(x, y);
    });

    test_properties(
        pairs_of_natural_and_positive_natural_var_1,
        |&(ref x, ref y)| {
            div_mod_properties_helper(x, y);
        },
    );

    test_properties(naturals, |n| {
        let (q, r) = n.div_mod(Natural::ONE);
        assert_eq!(q, *n);
        assert_eq!(r, 0 as Limb);
    });

    test_properties(positive_naturals, |n| {
        assert_eq!(n.div_mod(Natural::ONE), (n.clone(), Natural::ZERO));
        assert_eq!(n.div_mod(n), (Natural::ONE, Natural::ZERO));
        assert_eq!(Natural::ZERO.div_mod(n), (Natural::ZERO, Natural::ZERO));
        if *n > 1 as Limb {
            assert_eq!(Natural::ONE.div_mod(n), (Natural::ZERO, Natural::ONE));
        }
    });
}

fn ceiling_div_neg_mod_properties_helper(x: &Natural, y: &Natural) {
    let mut mut_x = x.clone();
    let remainder = mut_x.ceiling_div_assign_neg_mod(y);
    assert!(mut_x.is_valid());
    assert!(remainder.is_valid());
    let quotient = mut_x;

    let mut mut_x = x.clone();
    let remainder_alt = mut_x.ceiling_div_assign_neg_mod(y.clone());
    let quotient_alt = mut_x;
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.ceiling_div_neg_mod(y);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.ceiling_div_neg_mod(y.clone());
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.clone().ceiling_div_neg_mod(y);
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    let (quotient_alt, remainder_alt) = x.clone().ceiling_div_neg_mod(y.clone());
    assert!(quotient_alt.is_valid());
    assert_eq!(quotient_alt, quotient);
    assert!(remainder_alt.is_valid());
    assert_eq!(remainder_alt, remainder);

    //TODO
    /*
    let (quotient_alt, remainder_alt) = (x.div_round(y, RoundingMode::Ceiling), x.neg_mod(y));
    assert_eq!(quotient_alt, quotient);
    assert_eq!(remainder_alt, remainder);
    */

    let (rug_quotient, rug_remainder) =
        rug_ceiling_div_neg_mod(natural_to_rug_integer(x), natural_to_rug_integer(y));
    assert_eq!(rug_integer_to_natural(&rug_quotient), quotient);
    assert_eq!(rug_integer_to_natural(&rug_remainder), remainder);

    assert!(remainder < *y);
    assert_eq!(quotient * y - remainder, *x);
}

#[test]
fn ceiling_div_neg_mod_limb_properties() {
    test_properties(pairs_of_natural_and_positive_natural, |&(ref x, ref y)| {
        ceiling_div_neg_mod_properties_helper(x, y);
    });

    test_properties(
        pairs_of_natural_and_positive_natural_var_1,
        |&(ref x, ref y)| {
            ceiling_div_neg_mod_properties_helper(x, y);
        },
    );

    test_properties(naturals, |n| {
        let (q, r) = n.ceiling_div_neg_mod(Natural::ONE);
        assert_eq!(q, *n);
        assert_eq!(r, 0 as Limb);
    });

    test_properties(positive_naturals, |n| {
        assert_eq!(
            n.ceiling_div_neg_mod(Natural::ONE),
            (n.clone(), Natural::ZERO)
        );
        assert_eq!(n.ceiling_div_neg_mod(n), (Natural::ONE, Natural::ZERO));
        assert_eq!(
            Natural::ZERO.ceiling_div_neg_mod(n),
            (Natural::ZERO, Natural::ZERO)
        );
        if *n > 1 as Limb {
            assert_eq!(
                Natural::ONE.ceiling_div_neg_mod(n),
                (Natural::ONE, n - 1 as Limb)
            );
        }
    });
}
