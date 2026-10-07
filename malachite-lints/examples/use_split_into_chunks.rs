fn consume<T>(_: T) {}

fn main() {
    let c = 3;
    let mut buffer = vec![0u32; 50];
    let (xs, scratch) = buffer.split_at_mut(20);
    {
        // Two equal-length mutable chunks, each split off the previous remainder: flagged.
        let (x_sum, scratch) = scratch.split_at_mut(c);
        let (y_sum, scratch) = scratch.split_at_mut(c);
        let middle = scratch.split_at_mut((c << 1) - 1).0;
        consume(x_sum);
        consume(y_sum);
        consume(middle);
    }
    let xs = &*xs;
    {
        // Three immutable chunks, with the remainder kept: flagged.
        let (a, rest) = xs.split_at(4);
        let (b, rest) = rest.split_at(4);
        let (d, rest) = rest.split_at(4);
        consume(a);
        consume(b);
        consume(d);
        consume(rest);
    }
    {
        // Different lengths: fine.
        let (a, rest) = xs.split_at(4);
        let (b, rest) = rest.split_at(5);
        consume(a);
        consume(b);
        consume(rest);
    }
    {
        // A single split: fine.
        let (a, rest) = xs.split_at(4);
        consume(a);
        consume(rest);
    }
    {
        // The first remainder is used again later, so it cannot be dropped: fine.
        let (a, rest) = xs.split_at(4);
        let (b, rest_2) = rest.split_at(4);
        consume(a);
        consume(b);
        consume(rest);
        consume(rest_2);
    }
    {
        // A `mut` chunk binding has no macro equivalent: fine.
        let (mut a, rest) = xs.split_at(4);
        let (b, rest) = rest.split_at(4);
        consume(a);
        a = b;
        consume(a);
        consume(rest);
    }
}
