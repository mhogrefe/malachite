// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::One;
use malachite_base::num::modular::traits::{ModPowerOf2IsReduced, ModPowerOf2Mul};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::modular::traits::{ModPowerOf2Evaluate, ModPowerOf2FromRoots};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_vec_unsigned_pair_gen_var_1;
use malachite_nz::test_util::natural_polynomial::modular::mod_power_of_2_from_roots::*;

fn naturals(xs: &[&str]) -> Vec<Natural> {
    xs.iter().map(|x| Natural::from_str(x).unwrap()).collect()
}

#[test]
fn test_mod_power_of_2_from_roots() {
    let test = |roots: &[&str], pow, out| {
        let roots = naturals(roots);
        let p = NaturalPolynomial::mod_power_of_2_from_roots(&roots, pow);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(
            mod_power_of_2_from_roots_naive(&roots, pow).to_string(),
            out
        );
    };
    // - modulo 1
    test(&[], 0, "0");
    test(&["0", "0"], 0, "0");
    // - no roots
    test(&[], 3, "1");
    // - a modulus that fits in a limb
    test(&["5", "1", "3"], 3, "x^3+7*x^2+7*x+1");
    // - a modulus of 2^64, with roots split into two products
    test(
        &[
            "15037404958031165010",
            "18134202829000061405",
            "8816671109623896822",
            "6909801505670644180",
            "536629792805628473",
            "10006589572760746539",
            "15758992454174395339",
            "8359512107684579415",
            "907877893701031397",
            "5233514391084077678",
            "3401026325537279538",
            "5372732944782975236",
            "14740057818129543753",
            "208679309773752889",
            "16968812783439595967",
            "10331098841686358083",
            "8880072476336853376",
            "14857237693797248255",
            "16840394181511641709",
            "12871203113371454004",
            "1810750037659508232",
        ],
        64,
        "x^21+6930922670242630476*x^20+6597304733988420658*x^19+13481953505664873856*x^18+29567436\
        75757066383*x^17+12887532952829806604*x^16+1355516333137224980*x^15+6347774054333322912*x^\
        14+14143531478952713951*x^13+1052259270799135076*x^12+15044245793501060226*x^11+1730734489\
        08432416*x^10+15680319562350280929*x^9+13855041157946014852*x^8+2744341342154266648*x^7+14\
        544740918946461376*x^6+5550766073248584912*x^5+13481864893264667584*x^4+170565212237839488\
        0*x^3+608407341137138688*x^2+118588125569720320*x+7796954676432207872",
    );
    // - a modulus of 2^128
    test(
        &[
            "42518599028234605290224957564028652016",
            "235913643902640621070836760321905684135",
            "60105336372619999943216389965408972872",
            "205664621802234169523703499632930846775",
            "284718353315923781327199608059294885578",
        ],
        128,
        "x^5+191926546341162213234942606751735592992*x^4+205589006408783107578491936283361445197*x\
        ^3+331148523064498413028654980611819130270*x^2+265579640505344681967291298316317077424*x+1\
        41425442943997628064870449601735085312",
    );
    // - a modulus of more than two limbs, with roots split into two products
    test(
        &[
            "825845327543134647802815536632",
            "1109447283175227551986030734511",
            "334077659919270464208301299607",
            "422738534881999622948482420427",
            "420938797730581434507006983723",
            "1199687919976813272306360766305",
            "582022849247669197238868916665",
            "962462588719875040619021972796",
            "72434853633028620570934762091",
            "454126825090113562458780411773",
            "552309552590203515457212567345",
            "1202496348036949791187515872351",
            "493143043310044177075358179773",
            "205360572364899935032469416711",
            "684873537296659967191685654291",
            "554606729842888990010762304214",
            "213788381516484998436680158854",
            "798459314186504913796528231083",
            "800049483465498979041455058348",
            "716601734297505998646702006336",
            "486404194297218051536630275528",
        ],
        100,
        "x^21+852281071387950684404131729772*x^20+127671563755173967684361384105*x^19+784896658256\
        054699307519778500*x^18+125648785871400373963892686057*x^17+275989424147393889026944241796\
        *x^16+902382383706191547611483232781*x^15+1194311115036189607726158681932*x^14+88995600298\
        8004296282543003639*x^13+164190835235077048642395702900*x^12+18369667737042207333560519804\
        3*x^11+137927735859337461081386263676*x^10+838618642326280425128786241731*x^9+507607742599\
        602926886903903548*x^8+393863780482457645174108250223*x^7+408931093500213202289404877812*x\
        ^6+161400800751484140100861615516*x^5+181205445440181285662558594016*x^4+29834835535242033\
        9321835236544*x^3+337830910193438997910617081856*x^2+659016218537815426444716052480*x+5058\
        40928762787873834701422592",
    );
    // - a large modulus
    test(
        &[
            "229104837172448584978535302825610305567349406493502598221858",
            "1415696215550448328078354435948313807625949342680353732326901",
            "721727038018511078208412845502121555433126633106839993309231",
        ],
        200,
        "x^3+847347997776572559818621600406279536417980605284889346744762*x^2+12903202936954498757\
        17911220972808780445498666061907924353219*x+7729374231760834198812872669264024119805221204\
        96314712363178",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_from_roots_fail() {
    // A root is not reduced.
    NaturalPolynomial::mod_power_of_2_from_roots(&[Natural::ONE, Natural::from(8u32)], 3);
}

#[test]
fn mod_power_of_2_from_roots_properties() {
    let test = |(xs, pow): (Vec<Natural>, u64)| {
        let p = NaturalPolynomial::mod_power_of_2_from_roots(&xs, pow);
        assert!(p.is_valid());
        assert!(p.mod_power_of_2_is_reduced(pow));
        assert_eq!(mod_power_of_2_from_roots_naive(&xs, pow), p);
        assert_eq!(p.len(), u64::try_from(xs.len()).unwrap() + 1);
        assert_eq!(*p.leading_coefficient(), 1u32);
        for x in &xs {
            assert_eq!((&p).mod_power_of_2_evaluate(x, pow), 0u32);
        }
        let mut ys = xs.clone();
        ys.reverse();
        assert_eq!(NaturalPolynomial::mod_power_of_2_from_roots(&ys, pow), p);
        let (xs_1, xs_2) = xs.split_at(xs.len() >> 1);
        assert_eq!(
            NaturalPolynomial::mod_power_of_2_from_roots(xs_1, pow)
                .mod_power_of_2_mul(NaturalPolynomial::mod_power_of_2_from_roots(xs_2, pow), pow),
            p
        );
    };
    natural_vec_unsigned_pair_gen_var_1().test_properties(test);

    // Long enough to split into products of the halves.
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_length_d", 1);
    natural_vec_unsigned_pair_gen_var_1().test_properties_with_config(&config, test);
}
