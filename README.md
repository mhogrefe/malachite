<img width="500" src="docs/assets/logo-and-name.svg" alt="Logo">

An arbitrary-precision arithmetic library for Rust.

Parts of Malachite are derived from [GMP](https://gmplib.org/),
[FLINT](https://www.flintlib.org/), and [MPFR](https://www.mpfr.org/).

Floats (arbitrary-precision floating-point numbers) are in development and are currently
experimental. They are missing many important functions. However, the floating-point functions that *are* currently
implemented are thoroughly tested and documented, with the exception of string conversion
functions. The current floating-point string conversion functions are incomplete and will be changed in the future to
match MPFR's behavior.

Malachite is developed by Mikhail Hogrefe. Thanks to Rowan Hart, YunWon Jeong, Park Joon-Kyu, Antonio Mamić, John Vandenberg, b4D8, florian1345, konstin, OliverNChalk, and shekohex for additional contributions.

<https://www.malachite.rs/>

Copyright © 2024 Mikhail Hogrefe
