use num::traits::NotAssign;

impl NotAssign for bool {
    fn not_assign(&mut self) {
        *self = !*self
    }
}
