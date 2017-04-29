pub trait Assign<Rhs = Self> {
    fn assign(&mut self, rhs: Rhs);
}
