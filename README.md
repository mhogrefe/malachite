<img width="500" src="docs/assets/logo-and-name.svg" alt="Logo">

An arbitrary-precision arithmetic library for Rust.

Parts of Malachite are derived from [GMP](https://gmplib.org/),
[FLINT](https://www.flintlib.org/), and [MPFR](https://www.mpfr.org/).

The documentation for Malachite is [here](https://docs.rs/malachite/latest/malachite/), and its crate is [here](https://crates.io/crates/malachite).

Floats (arbitrary-precision floating-point numbers) are in development and are currently experimental. They are missing many important functions. However, the floating-point functions that *are* currently implemented are thoroughly tested and documented, with the exception of string conversion functions. The current floating-point string conversion functions are incomplete and will be changed in the future to match MPFR's behavior.

Malachite is developed by Mikhail Hogrefe. Thanks to 43615, b4D8, coolreader18, Duncan Freeman, florian1345, konstin, Rowan Hart, YunWon Jeong, Park Joon-Kyu, Antonio Mamić, OliverNChalk, Kevin Phoenix, probablykasper, shekohex, skycloudd, John Vandenberg, Brandon Weeks, and Will Youmans for additional contributions.

<https://malachite.rs/>

Copyright © 2025 Mikhail Hogrefe
