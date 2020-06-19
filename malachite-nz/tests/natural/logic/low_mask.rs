use malachite_base::num::logic::traits::LowMask;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::logic::low_mask::limbs_low_mask;
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_low_mask() {
    let test = |bits, out: &[Limb]| assert_eq!(limbs_low_mask(bits), out);
    test(0, &[]);
    test(1, &[1]);
    test(2, &[3]);
    test(3, &[7]);
    test(32, &[u32::MAX]);
    test(100, &[u32::MAX, u32::MAX, u32::MAX, 15]);
}

#[test]
fn test_low_mask() {
    let test = |bits, out| assert_eq!(Natural::low_mask(bits).to_string(), out);
    test(0, "0");
    test(1, "1");
    test(2, "3");
    test(3, "7");
    test(32, "4294967295");
    test(100, "1267650600228229401496703205375");
}
