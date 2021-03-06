use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_nz::natural::Natural;

#[test]
fn test_power_of_2() {
    let test = |pow, out| assert_eq!(Natural::power_of_2(pow).to_string(), out);
    test(0, "1");
    test(1, "2");
    test(2, "4");
    test(3, "8");
    test(32, "4294967296");
    test(100, "1267650600228229401496703205376");
}
