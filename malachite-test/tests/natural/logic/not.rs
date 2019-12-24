use std::str::FromStr;

use malachite_nz::natural::logic::not::{limbs_not, limbs_not_in_place, limbs_not_to_out};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use rug;

use malachite_test::common::test_properties;
use malachite_test::common::{natural_to_rug_integer, rug_integer_to_integer};
use malachite_test::inputs::base::{pairs_of_unsigned_vec_var_3, vecs_of_unsigned};
use malachite_test::inputs::natural::naturals;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_not_and_limbs_not_in_place() {
    let test = |limbs_in: &[Limb], out: &[Limb]| {
        assert_eq!(limbs_not(limbs_in), out);

        let mut mut_limbs = limbs_in.to_vec();
        limbs_not_in_place(&mut mut_limbs);
        assert_eq!(mut_limbs, out);
    };
    test(&[], &[]);
    test(&[0, 1, 2], &[0xffff_ffff, 0xffff_fffe, 0xffff_fffd]);
    test(&[0xffff_ffff, 0xffff_fffe, 0xffff_fffd], &[0, 1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_not_to_out() {
    let test = |limbs_in: &[Limb], out_before: &[Limb], out_after: &[Limb]| {
        let mut mut_out = out_before.to_vec();
        limbs_not_to_out(&mut mut_out, limbs_in);
        assert_eq!(mut_out, out_after);
    };
    test(&[], &[], &[]);
    test(&[0x1111_1111], &[5], &[0xeeee_eeee]);
    test(
        &[0xffff_0000, 0xf0f0_f0f0],
        &[0, 1, 2],
        &[0x0000_ffff, 0x0f0f_0f0f, 2],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_not_to_out_fail() {
    let mut out = vec![1, 2];
    limbs_not_to_out(&mut out, &[1, 2, 3]);
}

#[test]
fn test_not() {
    let test = |s, out| {
        let not = !Natural::from_str(s).unwrap();
        assert!(not.is_valid());
        assert_eq!(not.to_string(), out);

        let not = !(&Natural::from_str(s).unwrap());
        assert!(not.is_valid());
        assert_eq!(not.to_string(), out);

        assert_eq!((!rug::Integer::from_str(s).unwrap()).to_string(), out);
    };
    test("0", "-1");
    test("123", "-124");
    test("1000000000000", "-1000000000001");
    test("2147483647", "-2147483648");
}

#[test]
fn limbs_not_properties() {
    test_properties(vecs_of_unsigned, |limbs| {
        assert_eq!(limbs_not(&limbs_not(limbs)), *limbs);
    });
}

#[test]
fn limbs_not_to_out_properties() {
    test_properties(pairs_of_unsigned_vec_var_3, |&(ref out, ref limbs_in)| {
        let mut mut_out = out.to_vec();
        limbs_not_to_out(&mut mut_out, limbs_in);
        limbs_not_in_place(&mut mut_out[..limbs_in.len()]);
        assert_eq!(mut_out[..limbs_in.len()], **limbs_in);
    });
}

#[test]
fn limbs_not_in_place_properties() {
    test_properties(vecs_of_unsigned, |limbs| {
        let mut mut_limbs = limbs.to_vec();
        limbs_not_in_place(&mut mut_limbs);
        limbs_not_in_place(&mut mut_limbs);
        assert_eq!(mut_limbs, *limbs);
    });
}

#[test]
fn not_properties() {
    test_properties(naturals, |x| {
        let not = !x.clone();
        assert!(not.is_valid());

        let rug_not = !natural_to_rug_integer(x);
        assert_eq!(rug_integer_to_integer(&rug_not), not);

        let not_alt = !x;
        assert!(not_alt.is_valid());
        assert_eq!(not_alt, not);

        assert!(not < 0 as Limb);
        assert_ne!(not, *x);
        assert_eq!(!&not, *x);
    });
}
