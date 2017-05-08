use malachite::natural::Natural;
use malachite::natural::random::assign_random_up_to_bits::assign_random_up_to_bits;
use rand::{SeedableRng, StdRng};

#[test]
fn test_assign_random_up_to_bits() {
    let test = |bits, out| {
        let seed: &[_] = &[1, 2, 3, 4];
        let mut rng: StdRng = SeedableRng::from_seed(seed);
        let mut x = Natural::new();
        assign_random_up_to_bits(&mut rng, &mut x, bits);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
    };
    test(1, "0");
    test(2, "2");
    test(3, "2");
    test(4, "2");
    test(5, "18");
    test(10, "818");
    test(32, "2562717490");
    test(100, "827890343590397684290531882802");
}
