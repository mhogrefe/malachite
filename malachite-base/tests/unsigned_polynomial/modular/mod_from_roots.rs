// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::modular::traits::{ModIsReduced, ModMul};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::modular::traits::{ModEvaluate, ModFromRoots};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::unsigned_vec_unsigned_pair_gen_var_34;
use malachite_base::test_util::unsigned_polynomial::modular::mod_from_roots::mod_from_roots_naive;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

#[test]
fn test_mod_from_roots() {
    fn test<T: PrimitiveUnsigned>(roots: &[T], m: T, out: &str) {
        let p = UnsignedPolynomial::mod_from_roots(roots, m);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(mod_from_roots_naive(roots, m).to_string(), out);
    }
    // - modulo 1
    test::<u8>(&[], 1, "0");
    test::<u8>(&[0, 0], 1, "0");
    // - no roots
    test::<u8>(&[], 7, "1");
    // - one root
    test::<u8>(&[3], 7, "x+4");
    // - fewer roots than the basecase cutoff
    test::<u8>(&[3, 5, 4], 7, "x^3+2*x^2+5*x+3");
    // - 20 roots, split into two products built one factor at a time
    test::<u8>(
        &[107, 3, 174, 156, 17, 19, 232, 95, 152, 17, 28, 121, 11, 19, 134, 49, 214, 197, 122, 135],
        251,
        "x^20+6*x^19+160*x^18+68*x^17+170*x^16+45*x^15+31*x^14+113*x^13+141*x^12+91*x^11+5*x^10+81\
        *x^9+206*x^8+247*x^7+180*x^6+169*x^5+194*x^4+6*x^3+53*x^2+120*x+63",
    );
    // - an odd number of roots, split unevenly
    test::<u16>(
        &[
            2287, 43316, 30492, 15211, 41134, 21113, 63719, 45474, 1819, 63204, 14617, 31817,
            59246, 8185, 62706, 45758, 38453, 13869, 40530, 57610, 40126,
        ],
        65521,
        "x^21+45566*x^20+43321*x^19+35413*x^18+47689*x^17+36418*x^16+47001*x^15+62958*x^14+52423*x\
        ^13+52492*x^12+47208*x^11+13522*x^10+33944*x^9+48330*x^8+348*x^7+42539*x^6+5591*x^5+40915*\
        x^4+60275*x^3+64824*x^2+51589*x+60066",
    );
    // - a u32 modulus
    test::<u32>(
        &[1904934449, 1354701188, 3293243520, 662313742, 2875628830],
        4294967291,
        "x^5+2794080144*x^4+3175994761*x^3+4223006646*x^2+1328284119*x+2984027197",
    );
    // - a u64 modulus with the top bit set
    test::<u64>(
        &[
            10200401960510887120,
            14205491695098479576,
            11254398519682825414,
            1494260083138687851,
            16240430835045676427,
            6069771422625351582,
            10055515508234677369,
            14175128343045152772,
            5717215469279418055,
            16419038998862698252,
            10001913084178345201,
            1253257831325303921,
            10130083450796347303,
            16051573088136482517,
            495031069638841882,
            3246192363884887267,
            15435261850374857239,
            10158735539641076760,
            10513343541701199636,
            17444873137236850778,
            5948901401136144039,
        ],
        18446744073709551557,
        "x^21+14850109690940427723*x^20+11277383220097873938*x^19+4658540181648114428*x^18+7621052\
        43984792631*x^17+11899862640370880089*x^16+1402022702500761607*x^15+1780769355290682011*x^\
        14+14537059947539869480*x^13+2703266506962595367*x^12+4609796718267545893*x^11+15460457612\
        42167224*x^10+7974317823088229630*x^9+5508590693923766888*x^8+7020111009980768578*x^7+1802\
        4070010469928699*x^6+8309053354367065845*x^5+14726085532967654300*x^4+11566819362942002491\
        *x^3+5797743595661809917*x^2+4207491341374361093*x+17265896527904106131",
    );
    // - enough roots that each half is split again
    test::<u64>(
        &[
            881315, 256479, 845559, 22801, 488570, 707264, 472077, 585398, 909795, 797956, 391066,
            157908, 169329, 199906, 120625, 122802, 407229, 485898, 467770, 348843, 97051, 362793,
            633639, 336691, 166143, 489981, 766071, 689377, 471501, 151811, 686769, 887069, 178962,
            459708, 471582, 96266, 124798, 102707, 808071, 512155, 177476, 878520, 574015, 183821,
            837706,
        ],
        1000003,
        "x^45+16787*x^44+847237*x^43+230325*x^42+931375*x^41+550202*x^40+915779*x^39+372232*x^38+6\
        00634*x^37+510933*x^36+380910*x^35+236363*x^34+580287*x^33+581890*x^32+278367*x^31+974237*\
        x^30+717619*x^29+538467*x^28+786981*x^27+354999*x^26+399328*x^25+824488*x^24+584065*x^23+3\
        90225*x^22+583465*x^21+880445*x^20+761929*x^19+930859*x^18+599927*x^17+257144*x^16+794878*\
        x^15+427165*x^14+210263*x^13+403933*x^12+828609*x^11+357968*x^10+130145*x^9+893123*x^8+912\
        881*x^7+593678*x^6+961119*x^5+372235*x^4+33291*x^3+986731*x^2+922379*x+73591",
    );
    // - a u128 modulus
    test::<u128>(
        &[
            103565610381804903626400136349560100373,
            41727932120698399609440533203976871726,
            141779057140763340917676778435968672658,
        ],
        170141183460469231731687303715884105727,
        "x^3+53209767277671819309857159442262566697*x^2+61756011494042082809046349718417411325*x+5\
        3272973319293416035748022862800647324",
    );
}

#[test]
#[should_panic]
fn mod_from_roots_fail_1() {
    // m is 0.
    UnsignedPolynomial::<u8>::mod_from_roots(&[], 0);
}

#[test]
#[should_panic]
fn mod_from_roots_fail_2() {
    // A root is not reduced.
    UnsignedPolynomial::<u8>::mod_from_roots(&[1, 7], 7);
}

fn mod_from_roots_properties_helper<T: PrimitiveUnsigned>() {
    let test = |(xs, m): (Vec<T>, T)| {
        let p = UnsignedPolynomial::mod_from_roots(&xs, m);
        assert!(p.is_valid());
        assert!(p.mod_is_reduced(&m));
        assert_eq!(mod_from_roots_naive(&xs, m), p);
        if m != T::ONE {
            assert_eq!(p.len(), u64::try_from(xs.len()).unwrap() + 1);
            assert_eq!(p.leading_coefficient(), T::ONE);
        }
        for &x in &xs {
            assert_eq!((&p).mod_evaluate(x, m), T::ZERO);
        }
        let mut ys = xs.clone();
        ys.reverse();
        assert_eq!(UnsignedPolynomial::mod_from_roots(&ys, m), p);
        let (xs_1, xs_2) = xs.split_at(xs.len() >> 1);
        assert_eq!(
            UnsignedPolynomial::mod_from_roots(xs_1, m)
                .mod_mul(UnsignedPolynomial::mod_from_roots(xs_2, m), m),
            p
        );
    };
    unsigned_vec_unsigned_pair_gen_var_34::<T>().test_properties(test);

    // Long enough to split into products of the halves.
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_length_d", 1);
    unsigned_vec_unsigned_pair_gen_var_34::<T>().test_properties_with_config(&config, test);
}

#[test]
fn mod_from_roots_properties() {
    apply_fn_to_unsigneds!(mod_from_roots_properties_helper);
}
