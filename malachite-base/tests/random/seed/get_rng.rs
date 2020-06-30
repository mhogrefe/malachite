use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

use malachite_base::random::seed::Seed;
use malachite_base::random::{StandardRand, EXAMPLE_SEED};

#[test]
fn test_get_rng() {
    assert_eq!(
        Seed::standard_gen(&mut EXAMPLE_SEED.get_rng()),
        Seed::standard_gen(&mut ChaCha20Rng::from_seed(EXAMPLE_SEED.bytes))
    );
}
