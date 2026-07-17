struct Foo(i32);

impl Foo {
    // A public function whose whole body forwards to another method, without `#[inline]`: flagged.
    pub fn delegates(&self, x: i32) -> i32 {
        self.helper(x)
    }

    // Already marked `#[inline]`: fine.
    #[inline]
    pub fn delegates_inlined(&self, x: i32) -> i32 {
        self.helper(x)
    }

    // Not public -- the compiler inlines small same-crate functions on its own: fine.
    fn private_delegates(&self, x: i32) -> i32 {
        self.helper(x)
    }

    // Not a pure delegation (an extra statement precedes the call): fine.
    pub fn not_just_delegation(&self, x: i32) -> i32 {
        let y = x + 1;
        self.helper(y)
    }

    // A real body rather than a single forwarding call: fine.
    pub fn real_body(&self) -> i32 {
        self.0 + 1
    }

    // A constructor call builds a value rather than delegating: fine.
    pub fn wrap(x: i32) -> Foo {
        Foo(x)
    }

    fn helper(&self, x: i32) -> i32 {
        self.0 + x
    }
}

// A public free function that only delegates: flagged.
pub fn free_delegates(x: i32) -> i32 {
    free_helper(x)
}

fn free_helper(x: i32) -> i32 {
    x + 1
}

fn main() {
    let f = Foo(1);
    let _ = f.delegates(2);
    let _ = f.delegates_inlined(2);
    let _ = f.private_delegates(2);
    let _ = f.not_just_delegation(2);
    let _ = f.real_body();
    let _ = Foo::wrap(3);
    let _ = free_delegates(4);
}
