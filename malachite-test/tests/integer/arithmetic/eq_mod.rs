use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{DivisibleBy, EqMod};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::integer::arithmetic::eq_mod::{limbs_eq_neg_limb_mod_limb, limbs_pos_eq_mod_neg};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::common::{integer_to_rug_integer, natural_to_rug_integer};
use malachite_test::inputs::base::{
    triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1, triples_of_unsigned_vec_var_58,
    triples_of_unsigned_vec_var_59, triples_of_unsigned_vec_var_60,
};
use malachite_test::inputs::integer::{
    pairs_of_integer_and_natural, pairs_of_integers, triples_of_integer_integer_and_natural,
    triples_of_integer_integer_and_natural_var_1, triples_of_integer_integer_and_natural_var_2,
};

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_eq_neg_limb_mod_limb() {
    let test = |limbs: &[Limb], limb: Limb, modulus: Limb, equal: bool| {
        assert_eq!(limbs_eq_neg_limb_mod_limb(limbs, limb, modulus), equal);
    };
    test(&[6, 7], 4, 2, true);
    test(&[7, 7], 4, 2, false);
    test(&[6, 7], 3, 2, false);
    test(&[7, 7], 3, 2, true);
    test(&[2, 2], 6, 13, true);
    test(&[100, 101, 102], 1_232, 10, true);
    test(&[100, 101, 102], 1_233, 10, false);
    test(&[123, 456], 153, 789, true);
    test(&[123, 456], 1_000, 789, false);
    test(&[0xffff_ffff, 0xffff_ffff], 101, 2, true);
    test(&[0xffff_ffff, 0xffff_ffff], 100, 2, false);
    test(&[0xffff_ffff, 0xffff_ffff], 111, 3, true);
    test(&[0xffff_ffff, 0xffff_ffff], 110, 3, false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_eq_neg_limb_mod_limb_fail() {
    limbs_eq_neg_limb_mod_limb(&[10], 10, 15);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_pos_eq_mod_neg() {
    let test = |xs: &[Limb], ys: &[Limb], modulus: &[Limb], equal: bool| {
        assert_eq!(limbs_pos_eq_mod_neg(xs, ys, modulus), equal);
        assert_eq!(
            Integer::from(Natural::from_limbs_asc(xs)).eq_mod(
                -Natural::from_limbs_asc(ys),
                Natural::from_limbs_asc(modulus)
            ),
            equal
        );
    };
    // xs.len() >= ys.len()
    // m_0 == 0 in limbs_pos_eq_mod_neg_greater
    // x_0.wrapping_sub(y_0) & dmask != 0
    test(&[1, 2], &[3, 4], &[0, 1], false);
    // xs.len() < ys.len()
    // m_0 > 0 in limbs_pos_eq_mod_neg_greater
    // m_0.wrapping_sub(clow) & dmask == 0
    // y_len == 1 in limbs_pos_eq_mod_neg_greater
    // m_len == 1 in limbs_pos_eq_mod_neg_greater
    // m_len == 1 && x_len < BMOD_1_TO_MOD_1_THRESHOLD in limbs_pos_eq_mod_neg_greater
    // m_0.odd() in limbs_pos_eq_mod_neg_greater
    test(&[1], &[0, 1], &[1], true);
    // m_0.even() in limbs_pos_eq_mod_neg_greater
    test(&[0, 1], &[2], &[2], true);
    // y_len > 1 in limbs_pos_eq_mod_neg_greater
    // !carry in limbs_pos_eq_mod_neg_greater
    test(&[0, 1], &[0, 1], &[1], true);
    // m_len > 1 in limbs_pos_eq_mod_neg_greater
    // m_len == 2 && m_0 != 0 in limbs_pos_eq_mod_neg_greater
    // m_1 > dmask in limbs_pos_eq_mod_neg_greater
    test(&[1], &[1], &[1, 1], false);
    // m_1 <= dmask in limbs_pos_eq_mod_neg_greater
    // m_len == 2 && x_len < BMOD_1_TO_MOD_1_THRESHOLD in limbs_pos_eq_mod_neg_greater
    test(&[1], &[1], &[2, 1], false);
    // m_len > 2 || m_0 == 0 in limbs_pos_eq_mod_neg_greater
    test(&[1], &[1], &[1, 0, 1], false);
    // carry in limbs_pos_eq_mod_neg_greater
    test(
        &[
            936369948, 322455623, 3632895046, 978349680, 17000327, 2833388987, 2719643819,
            4166701038,
        ],
        &[
            2342728269, 2320695303, 2977562202, 4108534583, 1505907268, 3739165110, 101046064,
            1901445664,
        ],
        &[
            602975281, 3649288173, 1789153785, 3864060421, 3382875975, 610141130,
        ],
        false,
    );
    // m_len == 1 && x_len >= BMOD_1_TO_MOD_1_THRESHOLD in limbs_pos_eq_mod_neg_greater
    // m_len == 1 && y_0 < m_0 in limbs_pos_eq_mod_neg_greater
    test(
        &[
            3212804911, 2160316770, 3206591581, 2745583315, 2792856428, 2609790999, 254315581,
            3004469490, 508063094, 1353715608, 1367299842, 4069646046, 1440957625, 1524484784,
            288602472, 4194451247, 2499616713, 3803487103, 3975841261, 1755595995, 461380355,
            3965260418, 2520582111, 992340694, 2691611144, 4015051922, 242165777, 2734657368,
            239186072, 3933748819, 3250321923, 1546216191, 4075883378, 1560123823, 1022810314,
        ],
        &[856581460],
        &[1156543657],
        false,
    );
    // m_len == 1 && y_0 >= m_0 in limbs_pos_eq_mod_neg_greater
    test(
        &[
            699451669, 1384063782, 4104156706, 2090365529, 3368513403, 1605027987, 1722318996,
            4090019049, 198101182, 226399264, 4254971267, 3499697654, 1822288851, 3663364198,
            3666563293, 1097460217, 3559303793, 1251556005, 259218385, 1723749986, 2387066577,
            2359913226, 2620380925, 4271675647, 463171233, 2021118504, 680228369, 1524230633,
            1749490053, 3584548683, 2218938430, 397628995, 612178748, 3391998933, 2704442362,
            2016763041, 1217539360, 582274870, 3864136270, 1974721683, 3784392767, 2316608663,
            3356014859, 3336310743, 844138936, 2364943580, 1237091482, 4091627960, 2225587863,
            689029792, 2742163065, 3431595450, 1807072448, 2860125390, 4280571765, 2825707858,
            2354644826, 3589846269, 3304355054, 1056026551, 2556988538, 4285659954, 110421432,
        ],
        &[3262274447],
        &[380871550],
        true,
    );
    // m_len == 2 && x_len >= BMOD_1_TO_MOD_1_THRESHOLD in limbs_pos_eq_mod_neg_greater
    // m_len == 2 && y_0 < m_0 in limbs_pos_eq_mod_neg_greater
    test(
        &[
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 1,
        ],
        &[2],
        &[2, 1],
        false,
    );
}

#[test]
fn test_eq_mod() {
    let test = |x, y, modulus, out| {
        assert_eq!(
            Integer::from_str(x).unwrap().eq_mod(
                Integer::from_str(y).unwrap(),
                Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            Integer::from_str(x).unwrap().eq_mod(
                Integer::from_str(y).unwrap(),
                &Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            Integer::from_str(x).unwrap().eq_mod(
                &Integer::from_str(y).unwrap(),
                Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            Integer::from_str(x).unwrap().eq_mod(
                &Integer::from_str(y).unwrap(),
                &Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            (&Integer::from_str(x).unwrap()).eq_mod(
                Integer::from_str(y).unwrap(),
                Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            (&Integer::from_str(x).unwrap()).eq_mod(
                Integer::from_str(y).unwrap(),
                &Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            (&Integer::from_str(x).unwrap()).eq_mod(
                &Integer::from_str(y).unwrap(),
                Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            (&Integer::from_str(x).unwrap()).eq_mod(
                &Integer::from_str(y).unwrap(),
                &Natural::from_str(modulus).unwrap()
            ),
            out
        );

        assert_eq!(
            Integer::from_str(y).unwrap().eq_mod(
                Integer::from_str(x).unwrap(),
                Natural::from_str(modulus).unwrap()
            ),
            out
        );
        assert_eq!(
            rug::Integer::from_str(x).unwrap().is_congruent(
                &rug::Integer::from_str(y).unwrap(),
                &rug::Integer::from_str(modulus).unwrap()
            ),
            out
        );
    };
    test("0", "0", "0", true);
    test("0", "1", "0", false);
    test("57", "57", "0", true);
    test("57", "58", "0", false);
    test("1000000000000", "57", "0", false);
    test("0", "256", "256", true);
    test("0", "256", "512", false);
    test("13", "23", "10", true);
    test("13", "24", "10", false);
    test("13", "21", "1", true);
    test("13", "21", "2", true);
    test("13", "21", "4", true);
    test("13", "21", "8", true);
    test("13", "21", "16", false);
    test("13", "21", "3", false);
    test("1000000000001", "1", "4096", true);
    test("1000000000001", "1", "8192", false);
    test("12345678987654321", "321", "1000", true);
    test("12345678987654321", "322", "1000", false);
    test("1234", "1234", "1000000000000", true);
    test("1234", "1235", "1000000000000", false);
    test("1000000001234", "1000000002234", "1000", true);
    test("1000000001234", "1000000002235", "1000", false);
    test("1000000001234", "1234", "1000000000000", true);
    test("1000000001234", "1235", "1000000000000", false);
    test("1000000001234", "5000000001234", "1000000000000", true);
    test("1000000001234", "5000000001235", "1000000000000", false);

    test("0", "-1", "0", false);
    test("57", "-57", "0", false);
    test("57", "-58", "0", false);
    test("1000000000000", "-57", "0", false);
    test("0", "-256", "256", true);
    test("0", "-256", "512", false);
    test("13", "-27", "10", true);
    test("13", "-28", "10", false);
    test("29", "-27", "1", true);
    test("29", "-27", "2", true);
    test("29", "-27", "4", true);
    test("29", "-27", "8", true);
    test("29", "-27", "16", false);
    test("29", "-27", "3", false);
    test("999999999999", "-1", "4096", true);
    test("999999999999", "-1", "8192", false);
    test("12345678987654321", "-679", "1000", true);
    test("12345678987654321", "-680", "1000", false);
    test("1000000001234", "-999999999766", "1000", true);
    test("1000000001234", "-999999999767", "1000", false);
    test("1000000001234", "-999999998766", "1000000000000", true);
    test("1000000001234", "-999999998767", "1000000000000", false);

    test("-1", "0", "0", false);
    test("-57", "57", "0", false);
    test("-57", "58", "0", false);
    test("-1000000000000", "57", "0", false);
    test("-256", "0", "256", true);
    test("-256", "0", "512", false);
    test("-13", "27", "10", true);
    test("-13", "28", "10", false);
    test("-29", "27", "1", true);
    test("-29", "27", "2", true);
    test("-29", "27", "4", true);
    test("-29", "27", "8", true);
    test("-29", "27", "16", false);
    test("-29", "27", "3", false);
    test("-999999999999", "1", "4096", true);
    test("-999999999999", "1", "8192", false);
    test("-12345678987654321", "679", "1000", true);
    test("-12345678987654321", "680", "1000", false);
    test("-1000000001234", "999999999766", "1000", true);
    test("-1000000001234", "999999999767", "1000", false);
    test("-1000000001234", "999999998766", "1000000000000", true);
    test("-1000000001234", "999999998767", "1000000000000", false);

    test("-57", "-57", "0", true);
    test("-57", "-58", "0", false);
    test("-1000000000000", "-57", "0", false);
    test("-13", "-23", "10", true);
    test("-13", "-24", "10", false);
    test("-13", "-21", "1", true);
    test("-13", "-21", "2", true);
    test("-13", "-21", "4", true);
    test("-13", "-21", "8", true);
    test("-13", "-21", "16", false);
    test("-13", "-21", "3", false);
    test("-1000000000001", "-1", "4096", true);
    test("-1000000000001", "-1", "8192", false);
    test("-12345678987654321", "-321", "1000", true);
    test("-12345678987654321", "-322", "1000", false);
    test("-1234", "-1234", "1000000000000", true);
    test("-1234", "-1235", "1000000000000", false);
    test("-1000000001234", "-1000000002234", "1000", true);
    test("-1000000001234", "-1000000002235", "1000", false);
    test("-1000000001234", "-1234", "1000000000000", true);
    test("-1000000001234", "-1235", "1000000000000", false);
    test("-1000000001234", "-5000000001234", "1000000000000", true);
    test("-1000000001234", "-5000000001235", "1000000000000", false);
}

#[test]
fn limbs_eq_neg_limb_mod_limb_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_and_positive_unsigned_var_1,
        |&(ref limbs, limb, modulus)| {
            let equal = limbs_eq_neg_limb_mod_limb(limbs, limb, modulus);
            assert_eq!(
                (-Natural::from_limbs_asc(limbs))
                    .eq_mod(Integer::from(limb), Natural::from(modulus)),
                equal
            );
        },
    );
}

#[test]
fn limbs_pos_eq_mod_neg_properties() {
    test_properties(
        triples_of_unsigned_vec_var_58,
        |&(ref xs, ref ys, ref modulus)| {
            let equal = limbs_pos_eq_mod_neg(xs, ys, modulus);
            assert_eq!(
                Integer::from(Natural::from_limbs_asc(xs)).eq_mod(
                    -Natural::from_limbs_asc(ys),
                    Natural::from_limbs_asc(modulus)
                ),
                equal
            );
        },
    );

    test_properties(
        triples_of_unsigned_vec_var_59,
        |&(ref xs, ref ys, ref modulus)| {
            assert!(limbs_pos_eq_mod_neg(xs, ys, modulus));
            assert!(Integer::from(Natural::from_limbs_asc(xs)).eq_mod(
                -Natural::from_limbs_asc(ys),
                Natural::from_limbs_asc(modulus)
            ));
        },
    );

    test_properties(
        triples_of_unsigned_vec_var_60,
        |&(ref xs, ref ys, ref modulus)| {
            assert!(!limbs_pos_eq_mod_neg(xs, ys, modulus));
            assert!(!Integer::from(Natural::from_limbs_asc(xs)).eq_mod(
                -Natural::from_limbs_asc(ys),
                Natural::from_limbs_asc(modulus)
            ));
        },
    );
}

#[test]
fn eq_mod_properties() {
    test_properties(
        triples_of_integer_integer_and_natural,
        |&(ref x, ref y, ref modulus)| {
            let equal = x.eq_mod(y, modulus);
            assert_eq!(y.eq_mod(x, modulus), equal);

            assert_eq!(x.eq_mod(y, modulus.clone()), equal);
            assert_eq!(x.eq_mod(y.clone(), modulus), equal);
            assert_eq!(x.eq_mod(y.clone(), modulus.clone()), equal);
            assert_eq!(x.clone().eq_mod(y, modulus), equal);
            assert_eq!(x.clone().eq_mod(y, modulus.clone()), equal);
            assert_eq!(x.clone().eq_mod(y.clone(), modulus), equal);
            assert_eq!(x.clone().eq_mod(y.clone(), modulus.clone()), equal);

            assert_eq!((-x).eq_mod(-y, modulus), equal);
            assert_eq!((x - y).divisible_by(Integer::from(modulus)), equal);
            assert_eq!((y - x).divisible_by(Integer::from(modulus)), equal);
            assert_eq!(
                integer_to_rug_integer(x)
                    .is_congruent(&integer_to_rug_integer(y), &natural_to_rug_integer(modulus)),
                equal
            );
        },
    );

    test_properties(
        triples_of_integer_integer_and_natural_var_1,
        |&(ref x, ref y, ref modulus)| {
            assert!(x.eq_mod(y, modulus));
            assert!(y.eq_mod(x, modulus));
            assert!(integer_to_rug_integer(x)
                .is_congruent(&integer_to_rug_integer(y), &natural_to_rug_integer(modulus)));
        },
    );

    test_properties(
        triples_of_integer_integer_and_natural_var_2,
        |&(ref x, ref y, ref modulus)| {
            assert!(!x.eq_mod(y, modulus));
            assert!(!y.eq_mod(x, modulus));
            assert!(!integer_to_rug_integer(x)
                .is_congruent(&integer_to_rug_integer(y), &natural_to_rug_integer(modulus)));
        },
    );

    test_properties(pairs_of_integers, |&(ref x, ref y)| {
        assert!(x.eq_mod(y, Natural::ONE));
        assert_eq!(x.eq_mod(y, Natural::ZERO), x == y);
    });

    test_properties(pairs_of_integer_and_natural, |&(ref x, ref y)| {
        assert_eq!(x.eq_mod(Integer::ZERO, y), x.divisible_by(Integer::from(y)));
        assert!(x.eq_mod(x, y));
    });
}
