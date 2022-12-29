pub trait Primes {
    type I: Iterator<Item = Self>;
    type LI: Iterator<Item = Self>;

    fn primes_less_than(n: &Self) -> Self::LI;

    fn primes_less_than_or_equal_to(n: &Self) -> Self::LI;

    fn primes() -> Self::I;
}
