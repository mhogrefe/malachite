use malachite_base::num::logic::traits::LowMask;
use malachite_nz::integer::Integer;

#[test]
fn test_low_mask() {
    let test = |bits, out| assert_eq!(Integer::low_mask(bits).to_string(), out);
    test(0, "0");
    test(1, "1");
    test(2, "3");
    test(3, "7");
    test(32, "4294967295");
    test(100, "1267650600228229401496703205375");
}
