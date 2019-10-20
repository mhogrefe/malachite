use malachite_base::num::arithmetic::traits::{EqModPowerOfTwo, ModPowerOfTwo};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::div_exact::{
    _limbs_modular_div_mod_schoolbook, _limbs_modular_div_schoolbook,
};
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::arithmetic::div_exact_limb::limbs_modular_invert_limb;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use common::test_properties;
use malachite_test::inputs::base::{
    quadruples_of_three_unsigned_vecs_and_unsigned_var_3,
    quadruples_of_three_unsigned_vecs_and_unsigned_var_4,
};

fn verify_limbs_modular_div_mod(ns: &[Limb], ds: &[Limb], borrow: bool, qs: &[Limb], rs: &[Limb]) {
    let n = Natural::from_limbs_asc(ns);
    let d = Natural::from_limbs_asc(ds);
    let q = Natural::from_limbs_asc(qs);
    let r = Natural::from_limbs_asc(rs);
    let n_len = ns.len();
    let d_len = ds.len();
    let q_len = n_len - d_len;
    let qd = q * d;
    assert_eq!(n < qd, borrow);
    assert!(qd.eq_mod_power_of_two(&n, u64::wrapping_from(q_len) << Limb::LOG_WIDTH));
    let expected_r = (Integer::from(n) - Integer::from(qd))
        .mod_power_of_two(u64::wrapping_from(n_len) << Limb::LOG_WIDTH)
        >> (u64::wrapping_from(q_len) << Limb::LOG_WIDTH);
    assert_eq!(expected_r, r, "{:?}, {:?}", ns, ds);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_modular_div_mod_schoolbook() {
    let test =
        |qs_in: &[Limb], ns_in: &[Limb], ds: &[Limb], borrow, qs_out: &[Limb], ns_out: &[Limb]| {
            let mut qs = qs_in.to_vec();
            let mut ns = ns_in.to_vec();
            let inverse = limbs_modular_invert_limb(ds[0]).wrapping_neg();
            assert_eq!(
                _limbs_modular_div_mod_schoolbook(&mut qs, &mut ns, ds, inverse),
                borrow
            );
            let q_len = ns.len() - ds.len();
            assert_eq!(&qs[..q_len], qs_out);
            assert_eq!(&ns[q_len..], ns_out);
            verify_limbs_modular_div_mod(ns_in, ds, borrow, qs_out, ns_out);
        };
    // ql > 0
    test(&[10; 3], &[0, 0, 0], &[1, 2], false, &[0], &[0, 0]);
    // ql == 0
    test(&[10; 3], &[1, 2, 3], &[1, 2], false, &[1], &[0, 3]);
    test(
        &[10; 3],
        &[1, 2, 3],
        &[5, 6],
        true,
        &[3435973837],
        &[858993456, 4294967294],
    );
    test(&[10; 3], &[1, 2, 3, 4], &[1, 2, 3], false, &[1], &[0, 0, 4]);
    test(
        &[10; 3],
        &[1, 2, 3, 4],
        &[1, 0xffff_ffff, 3],
        false,
        &[1],
        &[3, 4294967295, 3],
    );
    test(&[10; 3], &[0, 1], &[1], false, &[0], &[1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_modular_div_mod_schoolbook_fail_1() {
    _limbs_modular_div_mod_schoolbook(&mut [10; 3], &mut [1, 2, 3], &[], 1);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_modular_div_mod_schoolbook_fail_2() {
    let ds = &[1, 2, 3];
    let inverse = limbs_modular_invert_limb(ds[0]).wrapping_neg();
    _limbs_modular_div_mod_schoolbook(&mut [10; 3], &mut [1, 2, 3], ds, inverse);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_modular_div_mod_schoolbook_fail_3() {
    let ds = &[1, 2];
    let inverse = limbs_modular_invert_limb(ds[0]).wrapping_neg();
    _limbs_modular_div_mod_schoolbook(&mut [], &mut [1, 2, 3], ds, inverse);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_modular_div_mod_schoolbook_fail_4() {
    let ds = &[4, 5];
    let inverse = limbs_modular_invert_limb(ds[0]).wrapping_neg();
    _limbs_modular_div_mod_schoolbook(&mut [10; 3], &mut [1, 2, 3], ds, inverse);
}

fn verify_limbs_modular_div(ns: &[Limb], ds: &[Limb], qs: &[Limb]) {
    let n = Natural::from_limbs_asc(ns);
    let d = Natural::from_limbs_asc(ds);
    let q = Natural::from_limbs_asc(qs);
    assert_eq!(
        (q * d).mod_power_of_two(u64::wrapping_from(ns.len()) << Limb::LOG_WIDTH),
        n
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_modular_div_schoolbook() {
    let test = |qs_in: &[Limb], ns_in: &[Limb], ds: &[Limb], qs_out: &[Limb]| {
        let mut qs = qs_in.to_vec();
        let mut ns = ns_in.to_vec();
        let inverse = limbs_modular_invert_limb(ds[0]).wrapping_neg();
        _limbs_modular_div_schoolbook(&mut qs, &mut ns, ds, inverse);
        assert_eq!(&qs[..ns.len()], qs_out);
        verify_limbs_modular_div(ns_in, ds, qs_out);
    };
    test(&[10; 3], &[0, 0, 0], &[1, 2], &[0, 0, 0]);
    test(&[10; 3], &[1, 2, 3], &[1, 2], &[1, 0, 3]);
    test(
        &[10; 3],
        &[1, 2, 3],
        &[5, 6],
        &[3435973837, 3607772528, 3401614098],
    );
    test(&[10; 3], &[1, 2, 3], &[1, 2, 3], &[1, 0, 0]);
    test(&[10; 3], &[1, 2, 3], &[1, 0xffff_ffff, 3], &[1, 3, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_modular_div_schoolbook_fail_1() {
    _limbs_modular_div_schoolbook(&mut [10; 3], &mut [1, 2, 3], &[], 1);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_modular_div_schoolbook_fail_2() {
    let ds = &[1, 2, 3, 4];
    let inverse = limbs_modular_invert_limb(ds[0]).wrapping_neg();
    _limbs_modular_div_schoolbook(&mut [10; 3], &mut [1, 2, 3], ds, inverse);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_modular_div_schoolbook_fail_3() {
    let ds = &[1, 2];
    let inverse = limbs_modular_invert_limb(ds[0]).wrapping_neg();
    _limbs_modular_div_schoolbook(&mut [10, 10], &mut [1, 2, 3], ds, inverse);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_modular_div_schoolbook_fail_4() {
    let ds = &[4, 5];
    let inverse = limbs_modular_invert_limb(ds[0]).wrapping_neg();
    _limbs_modular_div_schoolbook(&mut [10; 3], &mut [1, 2, 3], ds, inverse);
}

#[test]
fn limbs_modular_div_mod_schoolbook_properties() {
    test_properties(
        quadruples_of_three_unsigned_vecs_and_unsigned_var_4,
        |&(ref qs, ref ns, ref ds, inverse)| {
            let ns_old = ns;
            let mut qs = qs.to_vec();
            let mut ns = ns.to_vec();
            let borrow = _limbs_modular_div_mod_schoolbook(&mut qs, &mut ns, ds, inverse);
            let q_len = ns.len() - ds.len();
            verify_limbs_modular_div_mod(ns_old, ds, borrow, &qs[..q_len], &ns[q_len..]);
        },
    );
}

#[test]
fn limbs_modular_div_schoolbook_properties() {
    test_properties(
        quadruples_of_three_unsigned_vecs_and_unsigned_var_3,
        |&(ref qs, ref ns, ref ds, inverse)| {
            let ns_old = ns;
            let mut qs = qs.to_vec();
            let mut ns = ns.to_vec();
            _limbs_modular_div_schoolbook(&mut qs, &mut ns, ds, inverse);
            verify_limbs_modular_div(ns_old, ds, &qs);
        },
    );
}
