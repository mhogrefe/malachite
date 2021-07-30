use malachite_base::num::arithmetic::traits::{
    CeilingSqrt, CeilingSqrtAssign, CheckedSqrt, FloorSqrt, FloorSqrtAssign, SqrtRem,
    SqrtRemAssign, Square,
};
#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::arithmetic::traits::{Parity, ShrRound};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{Iverson, One};
use malachite_base::num::conversion::traits::{ExactFrom, JoinHalves};
use malachite_base::num::logic::traits::BitAccess;
#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::logic::traits::LeadingZeros;
#[cfg(feature = "32_bit_limbs")]
use malachite_base::rounding_modes::RoundingMode;
use malachite_base_test_util::generators::common::GenConfig;
use malachite_base_test_util::generators::{
    large_type_gen_var_2, unsigned_gen, unsigned_pair_gen_var_31, unsigned_vec_pair_gen_var_4,
};
use malachite_nz::natural::arithmetic::sqrt::{
    _ceiling_sqrt_binary, _checked_sqrt_binary, _floor_sqrt_binary, _limbs_sqrt_helper,
    _limbs_sqrt_rem_helper, _limbs_sqrt_rem_helper_scratch_len, _sqrt_rem_2_newton,
    _sqrt_rem_binary,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::{DoubleLimb, Limb};
use malachite_nz_test_util::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_nz_test_util::generators::natural_gen;
use std::panic::catch_unwind;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_sqrt_rem_2_newton() {
    fn test(n_hi: Limb, n_lo: Limb, sqrt: Limb, r_hi: bool, r_lo: Limb) {
        assert_eq!(_sqrt_rem_2_newton(n_hi, n_lo), (sqrt, r_hi, r_lo));
        assert_eq!(
            DoubleLimb::from(sqrt)
                .square()
                .checked_add(DoubleLimb::join_halves(Limb::iverson(r_hi), r_lo))
                .unwrap(),
            DoubleLimb::join_halves(n_hi, n_lo)
        );
    }
    // no adjustment needed
    test(2000000000, 123, 2930859019, false, 2746357762);
    test(u32::MAX, 123, 4294967295, true, 122);
    // adjustment needed
    test(1073741825, 0, 2147483648, true, 0);
}

#[test]
fn sqrt_rem_2_newton_fail() {
    assert_panic!(_sqrt_rem_2_newton(1, Limb::MAX));
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sqrt_rem_helper() {
    fn test(old_xs: &[Limb], n: usize, out_out: &[Limb], xs_out: &[Limb], r_hi: bool) {
        assert!(old_xs.len() >= n << 1);
        let mut out = vec![0; n];
        let mut xs = old_xs.to_vec();
        let mut scratch = vec![0; _limbs_sqrt_rem_helper_scratch_len(n)];
        assert_eq!(
            _limbs_sqrt_rem_helper(&mut out, &mut xs, 0, &mut scratch),
            r_hi
        );
        assert_eq!(out, out_out);
        assert_eq!(xs, xs_out);

        let x = Natural::from_limbs_asc(&old_xs[..n << 1]);
        let sqrt = Natural::from_limbs_asc(&out[..n]);
        let mut rem = Natural::from_limbs_asc(&xs[..n]);
        if r_hi {
            rem.set_bit(u64::exact_from(n) << Limb::LOG_WIDTH);
        }
        assert_eq!((&x).sqrt_rem(), (sqrt.clone(), rem.clone()));
        assert_eq!((&sqrt).square() + rem, x);
        assert!((&sqrt).square() <= x);
        assert!((sqrt + Natural::ONE).square() > x);
    }
    // h2 == 1
    // !q
    // r_hi
    // h1 == h2
    // r_hi >= 0
    test(
        &[123, 456, 789, 2000000000],
        2,
        &[2012297342, 2930859019],
        &[1917624951, 2990305571, 2377342468, 942810576],
        false,
    );
    // !r_hi
    test(&[0, 0, 0, 1073741824], 2, &[0, 2147483648], &[0; 4], false);
    // q
    // r_hi < 0
    test(
        &[0, 0, 0, 1073741825],
        2,
        &[4294967295, 2147483648],
        &[4294967295, 1, 0, 0],
        false,
    );
    // h2 > 1
    // h1 != h2
    test(
        &[0, 0, 0, 0, 0, 1073741824],
        3,
        &[0, 0, 2147483648],
        &[0; 6],
        false,
    );
}

#[test]
fn limbs_sqrt_rem_helper_fail() {
    // out too short
    assert_panic!({
        let out = &mut [];
        let xs = &mut [1, 2, 3];
        let mut scratch = vec![0; _limbs_sqrt_rem_helper_scratch_len(out.len())];
        _limbs_sqrt_rem_helper(out, xs, 0, &mut scratch)
    });

    // xs too short
    assert_panic!({
        let out = &mut [1, 2, 3];
        let xs = &mut [1, 2, 3, 4, Limb::MAX];
        let mut scratch = vec![0; _limbs_sqrt_rem_helper_scratch_len(out.len())];
        _limbs_sqrt_rem_helper(out, xs, 0, &mut scratch)
    });

    // (2 * n - 1)th element of xs too small
    assert_panic!({
        let out = &mut [1, 2, 3];
        let xs = &mut [1, 2, 3, 4, 5, 6];
        let mut scratch = vec![0; _limbs_sqrt_rem_helper_scratch_len(out.len())];
        _limbs_sqrt_rem_helper(out, xs, 0, &mut scratch)
    });
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sqrt_helper() {
    fn test(xs: &[Limb], out_out: &[Limb], has_remainder: bool) {
        let n = xs.len().shr_round(1, RoundingMode::Ceiling);
        let odd = xs.len().odd();
        let shift = LeadingZeros::leading_zeros(*xs.last().unwrap()) >> 1;
        let mut out = vec![0; n];
        assert_eq!(_limbs_sqrt_helper(&mut out, xs, shift, odd), has_remainder);
        assert_eq!(out, out_out);

        let x = Natural::from_limbs_asc(xs);
        let sqrt = Natural::from_limbs_asc(&out);
        let (sqrt_alt, rem) = (&x).sqrt_rem();
        assert_eq!(sqrt, sqrt_alt);
        assert_eq!(has_remainder, rem != 0);
        assert!((&sqrt).square() <= x);
        assert!((sqrt + Natural::ONE).square() > x);
    }
    // shift != 0 first time
    // !q
    // qs_last != 0 <= 1
    // (*qs_head >> 3) | qs_tail[0].mod_power_of_2(Limb::WIDTH - s) != 0
    // odd == 1 || shift != 0
    test(
        &[1, 2, 3, 4, 5, 6, 7, 8, 9],
        &[406519538, 874900746, 1431655766, 1, 3],
        true,
    );
    // q
    test(
        &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        &[1984678003, 1990369149, 2938805076, 123516853, 207243],
        true,
    );
    // (*qs_head >> 3) | qs_tail[0].mod_power_of_2(Limb::WIDTH - s) == 0
    // cmp != Ordering::Less first time
    // slice_test_zero(&scratch_hi[h1 + 1..h2 + 1])
    // cmp == Ordering::Equal
    // shift != 0 second time
    // cmp != Ordering::Less second time
    test(&[0, 0, 0, 0, 0, 0, 0, 0, 1], &[0, 0, 0, 0, 1], false);
    // !slice_test_zero(&scratch_hi[h1 + 1..h2 + 1])
    test(
        &[0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
        &[4294959104, 4294967295, 32767, 0, 65536],
        true,
    );
    // cmp != Ordering::Equal
    // cmp == Ordering::Less second time
    test(
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
        &[4095, 0, 4294959104, 4294967295, 32767, 0, 65536],
        true,
    );
    // shift == 0 first time
    // odd == 0 && shift == 0
    test(
        &[
            2193991530, 2899278504, 3717617329, 1249076698, 879590153, 4210532297, 3303769392,
            1147691304, 3624392894, 1881877405, 1728780505, 931615955, 1096404509, 1326003135,
            370549396, 1987183422, 851586836, 2796582944, 2985872407, 2598301166, 356639089,
            2071312376, 1106184443, 3682669872, 1019062633, 3091583269, 502719907, 1700144765,
            1387866140, 1704631346, 2770368441, 1350352297,
        ],
        &[
            1761794912, 1811046967, 2629492573, 855368414, 1733978088, 3870361288, 2771154681,
            1755982555, 443586875, 4077786862, 37569597, 1268220478, 2132005861, 2264846737,
            2419409154, 2408260566,
        ],
        true,
    );
    // qs_last != 0 > 1
    test(
        &[
            4294967295, 4194303, 0, 0, 3758096384, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        ],
        &[
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295, 4294967295, 4294967295, 65535,
        ],
        true,
    );
    // shift == 0 second time
    test(
        &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 4294966784, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295, 4294967295, 4294967295,
        ],
        &[
            4278190079, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            4294967295, 65535,
        ],
        true,
    );
    // cmp == Ordering::Less first time
    test(
        &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4294967232, 4294967295, 4294967295,
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
            2147483679, 16777215, 0, 0, 0, 0, 4294967040, 4294967295, 4294967295, 4294967295,
            4294967295, 4294967295,
        ],
        &[
            4293920767, 1073741791, 0, 0, 0, 0, 3221217296, 8388607, 0, 0, 0, 0, 4294967168,
            4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        ],
        true,
    );
}

#[test]
fn limbs_sqrt_helper_fail() {
    // out too short
    assert_panic!({
        let out = &mut [1, 2, 3, 4];
        let xs = &mut [Limb::MAX; 8];
        _limbs_sqrt_helper(out, xs, 0, false);
    });
    // last element of xs is 0
    assert_panic!({
        let out = &mut [1, 2, 3, 4, 5];
        let xs = &mut [10, 10, 10, 10, 10, 10, 10, 10, 10, 0];
        _limbs_sqrt_helper(out, xs, 15, false);
    });
    // shift too high
    assert_panic!({
        let out = &mut [1, 2, 3, 4, 5];
        let xs = &mut [Limb::MAX; 10];
        _limbs_sqrt_helper(out, xs, 16, false);
    });
}

#[test]
fn test_floor_sqrt() {
    let test = |s, out| {
        let n = Natural::from_str(s).unwrap();
        assert_eq!(n.clone().floor_sqrt().to_string(), out);
        assert_eq!((&n).floor_sqrt().to_string(), out);
        assert_eq!(_floor_sqrt_binary(&n).to_string(), out);

        let mut n = n;
        n.floor_sqrt_assign();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0");
    test("1", "1");
    test("2", "1");
    test("3", "1");
    test("4", "2");
    test("5", "2");
    test("10", "3");
    test("100", "10");
    test("1000000000", "31622");
    test("152415765279683", "12345677");
    test("152415765279684", "12345678");
    test("152415765279685", "12345678");
    test(
        "10000000000000000000000000000000000000000",
        "100000000000000000000",
    );
    test(
        "100000000000000000000000000000000000000000",
        "316227766016837933199",
    );
}

#[test]
fn test_ceiling_sqrt() {
    let test = |s, out| {
        let n = Natural::from_str(s).unwrap();
        assert_eq!(n.clone().ceiling_sqrt().to_string(), out);
        assert_eq!((&n).ceiling_sqrt().to_string(), out);
        assert_eq!(_ceiling_sqrt_binary(&n).to_string(), out);

        let mut n = n;
        n.ceiling_sqrt_assign();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0");
    test("1", "1");
    test("2", "2");
    test("3", "2");
    test("4", "2");
    test("5", "3");
    test("10", "4");
    test("100", "10");
    test("1000000000", "31623");
    test("152415765279683", "12345678");
    test("152415765279684", "12345678");
    test("152415765279685", "12345679");
    test(
        "10000000000000000000000000000000000000000",
        "100000000000000000000",
    );
    test(
        "100000000000000000000000000000000000000000",
        "316227766016837933200",
    );
}

#[allow(clippy::redundant_closure_for_method_calls)]
#[test]
fn test_checked_sqrt() {
    let test = |s, out: Option<&str>| {
        let n = Natural::from_str(s).unwrap();
        let out = out.map(|s| s.to_string());

        assert_eq!(n.clone().checked_sqrt().map(|x| x.to_string()), out);
        assert_eq!((&n).checked_sqrt().map(|x| x.to_string()), out);
        assert_eq!(_checked_sqrt_binary(&n).map(|x| x.to_string()), out);
    };
    test("0", Some("0"));
    test("1", Some("1"));
    test("2", None);
    test("3", None);
    test("4", Some("2"));
    test("5", None);
    test("10", None);
    test("100", Some("10"));
    test("1000000000", None);
    test("152415765279683", None);
    test("152415765279684", Some("12345678"));
    test("152415765279685", None);
    test(
        "10000000000000000000000000000000000000000",
        Some("100000000000000000000"),
    );
    test("100000000000000000000000000000000000000000", None);
}

#[test]
fn test_sqrt_rem() {
    let test = |s, sqrt_out, rem_out| {
        let n = Natural::from_str(s).unwrap();

        let (sqrt, rem) = n.clone().sqrt_rem();
        assert_eq!(sqrt.to_string(), sqrt_out);
        assert_eq!(rem.to_string(), rem_out);

        let (sqrt, rem) = (&n).sqrt_rem();
        assert_eq!(sqrt.to_string(), sqrt_out);
        assert_eq!(rem.to_string(), rem_out);

        let (sqrt, rem) = _sqrt_rem_binary(&n);
        assert_eq!(sqrt.to_string(), sqrt_out);
        assert_eq!(rem.to_string(), rem_out);

        let mut n = n;
        assert_eq!(n.sqrt_rem_assign().to_string(), rem_out);
        assert_eq!(n.to_string(), sqrt_out);
    };
    test("0", "0", "0");
    test("1", "1", "0");
    test("2", "1", "1");
    test("3", "1", "2");
    test("4", "2", "0");
    test("5", "2", "1");
    test("10", "3", "1");
    test("100", "10", "0");
    test("1000000000", "31622", "49116");
    test("152415765279683", "12345677", "24691354");
    test("152415765279684", "12345678", "0");
    test("152415765279685", "12345678", "1");
    test(
        "10000000000000000000000000000000000000000",
        "100000000000000000000",
        "0",
    );
    test(
        "100000000000000000000000000000000000000000",
        "316227766016837933199",
        "562477137586013626399",
    );
}

#[test]
fn sqrt_rem_2_newton_properties() {
    unsigned_pair_gen_var_31().test_properties(|(n_hi, n_lo)| {
        let (sqrt, r_hi, r_lo) = _sqrt_rem_2_newton(n_hi, n_lo);
        assert_eq!(
            DoubleLimb::from(sqrt)
                .square()
                .checked_add(DoubleLimb::join_halves(Limb::iverson(r_hi), r_lo))
                .unwrap(),
            DoubleLimb::join_halves(n_hi, n_lo)
        );
    });
}

#[test]
fn limbs_sqrt_rem_helper_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_4().test_properties_with_config(&config, |(mut out, mut xs)| {
        let n = out.len();
        let mut scratch = vec![0; _limbs_sqrt_rem_helper_scratch_len(n)];
        let old_xs = xs.clone();
        let r_hi = _limbs_sqrt_rem_helper(&mut out, &mut xs, 0, &mut scratch);
        let x = Natural::from_limbs_asc(&old_xs[..n << 1]);
        let sqrt = Natural::from_limbs_asc(&out);
        let mut rem = Natural::from_limbs_asc(&xs[..n]);
        if r_hi {
            rem.set_bit(u64::exact_from(n) << Limb::LOG_WIDTH);
        }
        assert_eq!((&x).sqrt_rem(), (sqrt.clone(), rem.clone()));
        assert_eq!((&sqrt).square() + rem, x);
        assert!((&sqrt).square() <= x);
        assert!((sqrt + Natural::ONE).square() > x);
    });
}

#[test]
fn limbs_sqrt_helper_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    large_type_gen_var_2().test_properties_with_config(&config, |(mut out, xs, shift, odd)| {
        let has_remainder = _limbs_sqrt_helper(&mut out, &xs, shift, odd);
        let x = Natural::from_limbs_asc(&xs);
        let sqrt = Natural::from_limbs_asc(&out);
        let (sqrt_alt, rem) = (&x).sqrt_rem();
        assert_eq!(sqrt, sqrt_alt);
        assert_eq!(has_remainder, rem != 0);
        assert!((&sqrt).square() <= x);
        assert!((sqrt + Natural::ONE).square() > x);
    });
}

#[test]
fn floor_sqrt_properties() {
    natural_gen().test_properties(|n| {
        let sqrt = n.clone().floor_sqrt();
        assert_eq!((&n).floor_sqrt(), sqrt);
        let mut n_alt = n.clone();
        n_alt.floor_sqrt_assign();
        assert_eq!(n_alt, sqrt);
        assert_eq!(_floor_sqrt_binary(&n), sqrt);
        assert_eq!(biguint_to_natural(&natural_to_biguint(&n).sqrt()), sqrt);
        assert_eq!(
            rug_integer_to_natural(&natural_to_rug_integer(&n).sqrt()),
            sqrt
        );

        let square = (&sqrt).square();
        let ceiling_sqrt = (&n).ceiling_sqrt();
        if square == n {
            assert_eq!(ceiling_sqrt, sqrt);
        } else {
            assert_eq!(ceiling_sqrt, &sqrt + Natural::ONE);
        }
        assert!(square <= n);
        assert!((sqrt + Natural::ONE).square() > n);
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        assert_eq!(u.floor_sqrt(), Natural::from(u).floor_sqrt());
    });
}

#[test]
fn ceiling_sqrt_properties() {
    natural_gen().test_properties(|n| {
        let sqrt = n.clone().ceiling_sqrt();
        assert_eq!((&n).ceiling_sqrt(), sqrt);
        let mut n_alt = n.clone();
        n_alt.ceiling_sqrt_assign();
        assert_eq!(n_alt, sqrt);
        assert_eq!(_ceiling_sqrt_binary(&n), sqrt);
        let square = (&sqrt).square();
        let floor_sqrt = (&n).floor_sqrt();
        if square == n {
            assert_eq!(floor_sqrt, sqrt);
        } else {
            assert_eq!(floor_sqrt, &sqrt - Natural::ONE);
        }
        assert!(square >= n);
        if n != 0 {
            assert!((sqrt - Natural::ONE).square() < n);
        }
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        assert_eq!(u.ceiling_sqrt(), Natural::from(u).ceiling_sqrt());
    });
}

#[test]
fn checked_sqrt_properties() {
    natural_gen().test_properties(|n| {
        let sqrt = n.clone().checked_sqrt();
        assert_eq!((&n).checked_sqrt(), sqrt);
        assert_eq!(_checked_sqrt_binary(&n), sqrt);
        if let Some(sqrt) = sqrt {
            assert_eq!((&sqrt).square(), n);
            assert_eq!((&n).floor_sqrt(), sqrt);
            assert_eq!(n.ceiling_sqrt(), sqrt);
        }
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        assert_eq!(
            u.checked_sqrt().map(Natural::from),
            Natural::from(u).checked_sqrt()
        );
    });
}

#[test]
fn sqrt_rem_properties() {
    natural_gen().test_properties(|n| {
        let (sqrt, rem) = n.clone().sqrt_rem();
        assert_eq!((&n).sqrt_rem(), (sqrt.clone(), rem.clone()));
        let mut n_alt = n.clone();
        assert_eq!(n_alt.sqrt_rem_assign(), rem);
        assert_eq!(n_alt, sqrt);
        assert_eq!(_sqrt_rem_binary(&n), (sqrt.clone(), rem.clone()));
        let (rug_sqrt, rug_rem) = natural_to_rug_integer(&n).sqrt_rem(rug::Integer::new());
        assert_eq!(rug_integer_to_natural(&rug_sqrt), sqrt);
        assert_eq!(rug_integer_to_natural(&rug_rem), rem);

        assert_eq!((&n).floor_sqrt(), sqrt);
        assert!(rem <= &sqrt << 1);
        assert_eq!(sqrt.square() + rem, n);
    });

    unsigned_gen::<Limb>().test_properties(|u| {
        let (sqrt, rem) = u.sqrt_rem();
        assert_eq!(
            (Natural::from(sqrt), Natural::from(rem)),
            Natural::from(u).sqrt_rem()
        );
    });
}
