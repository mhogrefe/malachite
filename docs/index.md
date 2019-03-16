# Why Malachite?

Malachite is an alternative to Rust's built-in
[num](https://github.com/rust-num/num) library. It will
eventually provide a large number of mathematical types
and state-of-the-art algorithms, often ported from
established open-source libraries like
[GMP](https://gmplib.org/), [MPFR](https://www.mpfr.org/),
[FLINT](http://www.flintlib.org/), and
[Arb](http://arblib.org/). Some of its defining features
are
* A rich API, aiming to match the features of
  the libraries mentioned above (excluding GMP's
  cryptographically secure functions)
* Excellent performance while only using safe Rust code
* Thorough documentation, including doctests and big-O
  complexities for each function
* Very thorough testing; see the usage guide for the
  `malachite-test` crate (TODO)

# Why Not Malachite?

* There are some features outside the scope of Malachite.
  * **Cryptographically secure functions:** implementing
    these would require expertise that the Malachite
    contributors do not currently have.
* Malachite is released under
  [LGPL-3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html),
  which places some limits on its usage and
  redistribution. This is because much of Malachite's
  code is derived from GMP, MPFR, FLINT, and Arb, which are
  themselves released under LGPL.
* Malachite is very much under development, and not yet
  ready for general use. At some point in 2019, when
  `malachite-nz` is stabilized, this warning will be
  removed. 

# Usage Guide
TODO!
