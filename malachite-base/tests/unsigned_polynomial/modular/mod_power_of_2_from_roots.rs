// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::modular::traits::{ModPowerOf2IsReduced, ModPowerOf2Mul};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::modular::traits::{ModPowerOf2Evaluate, ModPowerOf2FromRoots};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::unsigned_vec_unsigned_pair_gen_var_10;
use malachite_base::test_util::unsigned_polynomial::modular::mod_power_of_2_from_roots::*;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

#[test]
fn test_mod_power_of_2_from_roots() {
    fn test<T: PrimitiveUnsigned>(roots: &[T], pow: u64, out: &str) {
        let p = UnsignedPolynomial::mod_power_of_2_from_roots(roots, pow);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(mod_power_of_2_from_roots_naive(roots, pow).to_string(), out);
    }
    // - modulo 1
    test::<u8>(&[], 0, "0");
    test::<u8>(&[0, 0], 0, "0");
    // - no roots
    test::<u8>(&[], 3, "1");
    // - one root
    test::<u8>(&[3], 3, "x+5");
    // - fewer roots than the basecase cutoff
    test::<u8>(&[5, 1, 3], 3, "x^3+7*x^2+7*x+1");
    // - 20 roots, split into two products built one factor at a time, with k equal to the width
    test::<u8>(
        &[110, 65, 221, 114, 10, 212, 221, 79, 43, 52, 221, 87, 193, 48, 110, 43, 72, 4, 226, 28],
        8,
        "x^20+145*x^19+214*x^18+116*x^17+94*x^16+22*x^15+184*x^14+140*x^13+217*x^12+17*x^11+154*x^\
        10+56*x^9+128*x^8+16*x^7+32*x^6",
    );
    // - an odd number of roots, split unevenly
    test::<u32>(
        &[
            50973241, 973146280, 848664836, 1982478403, 2023893070, 467747722, 710159615,
            1461357341, 1137900333, 2087209524, 1415704967, 178521718, 1246735108, 858143152,
            1839930494, 192471807, 1942590981, 1197049576, 556455094, 465376621, 1620783379,
        ],
        31,
        "x^21+365026866*x^20+2039115659*x^19+695453126*x^18+1103734474*x^17+893956484*x^16+1937402\
        022*x^15+1358753868*x^14+2005040485*x^13+48791290*x^12+1012085583*x^11+845418350*x^10+9507\
        92928*x^9+702097264*x^8+1085942256*x^7+1899158112*x^6+1709988736*x^5+356395520*x^4+5174620\
        16*x^3+663388160*x^2+321781760*x+1365245952",
    );
    // - a u64 modulus of 2^64
    test::<u64>(
        &[
            17828286969796574832,
            16837170024024197253,
            12781014824837877817,
            14088751969488412359,
            13020347587148933533,
            12667256168937592489,
            17844687615616222295,
            499770761794446537,
            14587981425518884067,
            4439768947547236293,
            5742176072659353864,
            10127289888366242415,
            16794311989436243289,
            13882496220717271223,
            18245576226468142258,
            1520775313091981116,
            10312985460230167224,
            10810567005174108298,
            4568006794153651195,
            1666559499881091008,
            12418145690640195216,
            3732927533856753444,
            3086919838947545122,
            9839843065459543272,
            10825000501820663088,
        ],
        64,
        "x^25+85799636320393117*x^24+9841768632202752788*x^23+2882377288810687486*x^22+14143902107\
        397882219*x^21+16716695251329778363*x^20+3421634336205019090*x^19+10479031422118005056*x^1\
        8+400922965038480223*x^17+12217397083229933027*x^16+7179223217320412032*x^15+6677113041756\
        088134*x^14+15874311426485604749*x^13+899011230610568293*x^12+17233612311636503354*x^11+10\
        437478636870485596*x^10+765798357505780872*x^9+7523148971877250176*x^8+6544530949602749312\
        *x^7+5261156169206893568*x^6+12970943094251610112*x^5+11703081334715514880*x^4+31857819586\
        85753344*x^3+12836917237344370688*x^2+5438031884216958976*x+6298570793714450432",
    );
    // - a u128 modulus of 2^128
    test::<u128>(
        &[
            252207368378859647745713281473655855151,
            289814796952758120916659476443336759018,
            228277309357168308516444812155669554846,
        ],
        128,
        "x^3+250547626074029313211306252222642465353*x^2+50632500064375444203545703429498645604*x+\
        222255973815672481645354588792109673516",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_from_roots_fail_1() {
    // pow is too large.
    UnsignedPolynomial::<u8>::mod_power_of_2_from_roots(&[], 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_from_roots_fail_2() {
    // A root is not reduced.
    UnsignedPolynomial::<u8>::mod_power_of_2_from_roots(&[1, 8], 3);
}

fn mod_power_of_2_from_roots_properties_helper<T: PrimitiveUnsigned>() {
    let test = |(xs, pow): (Vec<T>, u64)| {
        let p = UnsignedPolynomial::mod_power_of_2_from_roots(&xs, pow);
        assert!(p.is_valid());
        assert!(p.mod_power_of_2_is_reduced(pow));
        assert_eq!(mod_power_of_2_from_roots_naive(&xs, pow), p);
        assert_eq!(p.len(), u64::try_from(xs.len()).unwrap() + 1);
        assert_eq!(p.leading_coefficient(), T::ONE);
        for &x in &xs {
            assert_eq!((&p).mod_power_of_2_evaluate(x, pow), T::ZERO);
        }
        let mut ys = xs.clone();
        ys.reverse();
        assert_eq!(UnsignedPolynomial::mod_power_of_2_from_roots(&ys, pow), p);
        let (xs_1, xs_2) = xs.split_at(xs.len() >> 1);
        assert_eq!(
            UnsignedPolynomial::mod_power_of_2_from_roots(xs_1, pow).mod_power_of_2_mul(
                UnsignedPolynomial::mod_power_of_2_from_roots(xs_2, pow),
                pow
            ),
            p
        );
    };
    unsigned_vec_unsigned_pair_gen_var_10::<T>().test_properties(test);

    // Long enough to split into products of the halves.
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_length_d", 1);
    unsigned_vec_unsigned_pair_gen_var_10::<T>().test_properties_with_config(&config, test);
}

#[test]
fn mod_power_of_2_from_roots_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_from_roots_properties_helper);
}
