use malachite_base::num::arithmetic::traits::{
    DivRound, DoubleFactorial, Factorial, Multifactorial, Parity, Subfactorial,
};
use malachite_base::num::basic::traits::One;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::generators::{unsigned_gen_var_5, unsigned_pair_gen_var_18};
use malachite_nz::natural::arithmetic::factorial::{
    double_factorial_naive, factorial_naive, multifactorial_naive, subfactorial_naive,
};
use malachite_nz::natural::Natural;

#[test]
fn test_factorial() {
    fn test(n: u64, out: &str) {
        let f = Natural::factorial(n);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(factorial_naive(n), f);
    }
    test(0, "1");
    test(1, "1");
    test(2, "2");
    test(3, "6");
    test(4, "24");
    test(5, "120");
    test(10, "3628800");
    test(
        100,
        "93326215443944152681699238856266700490715968264381621468592963895217599993229915608941463\
        976156518286253697920827223758251185210916864000000000000000000000000",
    );
}

#[test]
fn test_double_factorial() {
    fn test(n: u64, out: &str) {
        let f = Natural::double_factorial(n);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(double_factorial_naive(n), f);
    }
    test(0, "1");
    test(1, "1");
    test(2, "2");
    test(3, "3");
    test(4, "8");
    test(5, "15");
    test(6, "48");
    test(7, "105");
    test(19, "654729075");
    test(20, "3715891200");
    test(
        99,
        "2725392139750729502980713245400918633290796330545803413734328823443106201171875",
    );
    test(
        100,
        "34243224702511976248246432895208185975118675053719198827915654463488000000000000",
    );
}

#[test]
fn test_multifactorial() {
    fn test(n: u64, m: u64, out: &str) {
        let f = Natural::multifactorial(n, m);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(multifactorial_naive(n, m), f);
    }
    test(0, 1, "1");
    test(1, 1, "1");
    test(2, 1, "2");
    test(3, 1, "6");
    test(4, 1, "24");
    test(5, 1, "120");

    test(0, 2, "1");
    test(1, 2, "1");
    test(2, 2, "2");
    test(3, 2, "3");
    test(4, 2, "8");
    test(5, 2, "15");
    test(6, 2, "48");
    test(7, 2, "105");

    test(0, 3, "1");
    test(1, 3, "1");
    test(2, 3, "2");
    test(3, 3, "3");
    test(4, 3, "4");
    test(5, 3, "10");
    test(6, 3, "18");
    test(7, 3, "28");
    test(8, 3, "80");
    test(9, 3, "162");

    test(10, 1, "3628800");
    test(20, 2, "3715891200");
    test(25, 3, "608608000");

    test(
        100,
        1,
        "93326215443944152681699238856266700490715968264381621468592963895217599993229915608941463\
        976156518286253697920827223758251185210916864000000000000000000000000",
    );
    test(
        100,
        2,
        "34243224702511976248246432895208185975118675053719198827915654463488000000000000",
    );
    test(100, 3, "174548867015437739741494347897360069928419328000000000");
}

#[test]
fn test_subfactorial() {
    fn test(n: u64, out: &str) {
        let f = Natural::subfactorial(n);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(subfactorial_naive(n), f);
    }
    test(0, "1");
    test(1, "0");
    test(2, "1");
    test(3, "2");
    test(4, "9");
    test(5, "44");
    test(10, "1334961");
    test(
        100,
        "34332795984163804765195977526776142032365783805375784983543400282685180793327632432791396\
        429850988990237345920155783984828001486412574060553756854137069878601",
    );
}

#[test]
fn factorial_properties() {
    unsigned_gen_var_5().test_properties(|n| {
        let f = Natural::factorial(n);
        assert!(f.is_valid());
        assert_eq!(factorial_naive(n), f);
        assert_eq!(Natural::multifactorial(n, 1), f);
        assert_ne!(f, 0u32);
        if n != 0 {
            assert_eq!(
                f.div_round(Natural::factorial(n - 1), RoundingMode::Exact),
                n
            );
        }
    });
}

#[test]
fn double_factorial_properties() {
    unsigned_gen_var_5().test_properties(|n| {
        let f = Natural::double_factorial(n);
        assert!(f.is_valid());
        assert_eq!(double_factorial_naive(n), f);
        assert_eq!(Natural::multifactorial(n, 2), f);
        assert_ne!(f, 0);
        if n > 1 {
            assert_eq!(
                f.div_round(Natural::double_factorial(n - 2), RoundingMode::Exact),
                n
            );
        }
    });
}

#[test]
fn multifactorial_properties() {
    unsigned_pair_gen_var_18().test_properties(|(n, m)| {
        let f = Natural::multifactorial(n, m);
        assert!(f.is_valid());
        assert_eq!(multifactorial_naive(n, m), f);
        assert_ne!(f, 0u32);
        if n >= m {
            assert_eq!(
                f.div_round(Natural::multifactorial(n - m, m), RoundingMode::Exact),
                n
            );
        }
    });
}

#[test]
fn subfactorial_properties() {
    unsigned_gen_var_5().test_properties(|n| {
        let f = Natural::subfactorial(n);
        assert!(f.is_valid());
        assert_eq!(subfactorial_naive(n), f);
        if n != 1 {
            assert_ne!(f, 0u32);
        }
        if n != 0 && n != 2 {
            let g = if n.even() {
                f - Natural::ONE
            } else {
                f + Natural::ONE
            };
            assert_eq!(
                g.div_round(Natural::subfactorial(n - 1), RoundingMode::Exact),
                n
            );
        }
    });
}
