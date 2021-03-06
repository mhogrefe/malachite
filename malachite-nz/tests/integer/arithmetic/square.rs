use malachite_base::num::arithmetic::traits::{Square, SquareAssign};
use malachite_nz::integer::Integer;
use std::str::FromStr;

#[test]
fn test_square() {
    let test = |x, out| {
        let u = Integer::from_str(x).unwrap();

        assert_eq!(u.clone().square().to_string(), out);
        assert_eq!((&u).square().to_string(), out);

        let mut x = u;
        x.square_assign();
        assert_eq!(x.to_string(), out);
    };
    test("0", "0");
    test("1", "1");
    test("2", "4");
    test("3", "9");
    test("10", "100");
    test("123", "15129");
    test("1000", "1000000");
    test("123456789", "15241578750190521");

    test("-1", "1");
    test("-2", "4");
    test("-3", "9");
    test("-10", "100");
    test("-123", "15129");
    test("-1000", "1000000");
    test("-123456789", "15241578750190521");
}
