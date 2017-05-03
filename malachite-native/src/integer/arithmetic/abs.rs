use integer::Integer;
use natural::Natural;

impl Integer {
    //TODO test
    pub fn abs(&mut self) -> &mut Integer {
        self.sign = true;
        self
    }

    //TODO test
    pub fn unsigned_abs(self) -> Natural {
        self.abs
    }
}
