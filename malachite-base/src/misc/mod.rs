pub trait Named {
    const NAME: &'static str;
}

//TODO docs
pub trait Walkable: Copy + Eq + Ord {
    fn increment(&mut self);

    fn decrement(&mut self);
}
