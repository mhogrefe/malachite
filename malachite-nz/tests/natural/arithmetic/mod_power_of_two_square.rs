use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{
    ModPowerOfTwoIsReduced, ModPowerOfTwoSquare, ModPowerOfTwoSquareAssign,
};

use malachite_nz::natural::Natural;

#[test]
fn test_mod_power_of_two_square() {
    let test = |u, pow, out| {
        assert!(Natural::from_str(u)
            .unwrap()
            .mod_power_of_two_is_reduced(pow));
        let n = Natural::from_str(u).unwrap().mod_power_of_two_square(pow);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);
        assert!(n.mod_power_of_two_is_reduced(pow));

        let n = (&Natural::from_str(u).unwrap()).mod_power_of_two_square(pow);
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);

        let mut n = Natural::from_str(u).unwrap();
        n.mod_power_of_two_square_assign(pow);
        assert_eq!(n.to_string(), out);
    };
    test("0", 0, "0");
    test("0", 2, "0");
    test("1", 2, "1");
    test("2", 2, "0");
    test("2", 3, "4");
    test("5", 3, "1");
    test("100", 8, "16");
    test("12345678987654321", 64, "16556040056090124897");
}
