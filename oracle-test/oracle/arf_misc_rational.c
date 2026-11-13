/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs BigExponentFloat's positive differences and remainders that mix a BigExponentFloat with a
    Rational against exact FLINT arithmetic. The mode `arf_misc_rational` accepts lines of the forms

    - `X.op_prec_round(Q, p, rm) = R` and `X.op_prec(Q, p) = R`, for op one of
      positive_difference_rational, rem_rational, ieee_remainder_rational,
      rem_rational_and_quotient_bits, and ieee_remainder_rational_and_quotient_bits;
    - `BigExponentFloat::op_prec_round(Q, Y, p, rm) = R` and `BigExponentFloat::op_prec(Q, Y, p) = R`,
      for op one of rational_positive_difference_big_exponent_float,
      rational_rem_big_exponent_float, rational_ieee_remainder_big_exponent_float, and their
      `_and_quotient_bits` versions (except for the positive difference);

    where X and Y are BigExponentFloats in Malachite's exact hexadecimal format, Q is a Rational
    written as `a/b` or `a`, and R is `(Z, O)` or, with quotient bits, `(Z, O, q)`.

    When the BigExponentFloat's exponent is moderate (below 2^16 in magnitude), the exact result is
    computed with fmpq arithmetic: the quotient is rounded toward zero or to the nearest integer with
    ties to even, the remainder is x - y * q, and the low quotient bits are ±(|q| mod 2^63), with
    q's sign. The exact result is rounded with arf_fmpz_div_fmpz, and the ordering found by comparing
    the rounded value with the exact one.

    A BigExponentFloat with a huge exponent is far larger or far smaller than the Rational. A far
    smaller operand is its own remainder, with quotient 0, and is negligible in a difference; there,
    it is replaced by an infinitesimal of the same sign, a power of 2 below the spacing of the
    rounding grid near the other operand and below that operand's distance to the grid, so that the
    rounding and its direction are unchanged. A far larger BigExponentFloat m * 2^s is reduced with
    fmpz_powm: by a Rational a / b, its remainder is ((m * b * 2^s) mod a) / b, computed modulo
    a * 2^63 to give the low quotient bits as well; and a Rational a / b divided by a tiny
    BigExponentFloat m * 2^-t leaves ((a * 2^t) mod (b * m)) / (b * 2^t), rounded as a quotient of
    integers and then scaled by 2^-t, which is exact for an arf_t.

    The special cases follow Malachite's documentation: there is no negative zero, and the quotient
    bits are 0 in the special cases.

    Any other nonempty line is an error, as is an input with no lines at all.
*/

#include <stdlib.h>
#include <string.h>

/* mpfr.h comes first, so that arf.h declares its MPFR conversions. */
#include <mpfr.h>

#include <flint/arf.h>
#include <flint/fmpq.h>
#include <flint/fmpz.h>

#include "oracle.h"

#define MAX_LEAVES 4
#define MAX_ARGS 6

static long checked;

static int
report(int line_number, const char * what)
{
    flint_printf("line %d: %s\n", line_number, what);
    return 1;
}

/* Splits `s` at the first occurrence of `sep`, returning the text after it, or NULL. */
static char *
split(char * s, const char * sep)
{
    char * p = strstr(s, sep);
    if (p == NULL)
        return NULL;
    *p = '\0';
    return p + strlen(sep);
}

/* The rounding mode named `s`. `Exact` maps to rounding down and sets `*exact`. Returns 0 on
   success. */
static int
parse_rm(const char * s, arf_rnd_t * rnd, int * exact)
{
    *exact = 0;
    if (strcmp(s, "Floor") == 0)
        *rnd = ARF_RND_FLOOR;
    else if (strcmp(s, "Ceiling") == 0)
        *rnd = ARF_RND_CEIL;
    else if (strcmp(s, "Down") == 0)
        *rnd = ARF_RND_DOWN;
    else if (strcmp(s, "Up") == 0)
        *rnd = ARF_RND_UP;
    else if (strcmp(s, "Nearest") == 0)
        *rnd = ARF_RND_NEAR;
    else if (strcmp(s, "Exact") == 0)
    {
        *rnd = ARF_RND_DOWN;
        *exact = 1;
    }
    else
        return -1;
    return 0;
}

static const char *
ordering_name(int c)
{
    return c < 0 ? "Less" : c > 0 ? "Greater" : "Equal";
}

/* Splits a tuple like `(A, B, C)` into its leaves. Returns the number of leaves, at most `max`. */
static int
leaves(char * s, char ** out, int max)
{
    int n = 0;
    char * p = s;
    while (*p != '\0')
    {
        while (*p == '(' || *p == ' ' || *p == ',' || *p == ')')
            p++;
        if (*p == '\0')
            break;
        if (n == max)
            return -1;
        out[n++] = p;
        while (*p != '\0' && *p != ',' && *p != ')')
            p++;
        if (*p != '\0')
            *p++ = '\0';
    }
    return n;
}

/* Parses a Rational written as `a/b` or `a`, in lowest terms with b > 0. Returns 0 on success. */
static int
parse_rational(fmpq_t q, const char * s)
{
    if (*s == '\0' || fmpq_set_str(q, s, 10) != 0 || fmpz_sgn(fmpq_denref(q)) <= 0)
        return -1;
    fmpq_t c;
    fmpq_init(c);
    fmpq_set(c, q);
    fmpq_canonicalise(c);
    int ok = fmpq_equal(c, q);
    fmpq_clear(c);
    return ok ? 0 : -1;
}

/* Whether the finite nonzero `x` has a moderate exponent: below 2^16 in magnitude. */
static int
moderate(const arf_t x)
{
    return fmpz_bits(ARF_EXPREF(x)) <= 16;
}

/* An upper bound for log2 |q|, for nonzero q: |q| < 2^(bits(a) - bits(b) + 1). */
static slong
rational_log_bound(const fmpq_t q)
{
    return (slong) fmpz_bits(fmpq_numref(q)) - (slong) fmpz_bits(fmpq_denref(q)) + 1;
}

/* The sign of the comparison of the finite x with the finite q. */
static int
cmp_arf_fmpq(const arf_t x, const fmpq_t q)
{
    if (arf_is_zero(x))
        return -fmpq_sgn(q);
    if (fmpq_is_zero(q) || arf_sgn(x) != fmpq_sgn(q))
        return arf_sgn(x);
    if (!moderate(x))
    {
        /* |x| is far from |q|: compare magnitudes by exponent, then apply the common sign. */
        int bigger = fmpz_sgn(ARF_EXPREF(x)) > 0;
        return bigger ? arf_sgn(x) : -arf_sgn(x);
    }
    fmpq_t t;
    fmpq_init(t);
    arf_get_fmpq(t, x);
    int c = fmpq_cmp(t, q);
    fmpq_clear(t);
    return c < 0 ? -1 : c > 0;
}

/* q rounded to `prec` bits with `rnd`, returning the ordering of the result relative to q. */
static int
round_fmpq(arf_t z, const fmpq_t q, slong prec, arf_rnd_t rnd)
{
    arf_fmpz_div_fmpz(z, fmpq_numref(q), fmpq_denref(q), prec, rnd);
    fmpq_t t;
    fmpq_init(t);
    arf_get_fmpq(t, z);
    int c = fmpq_cmp(t, q);
    fmpq_clear(t);
    return c < 0 ? -1 : c > 0;
}

/* q plus an infinitesimal with the sign `s` (nonzero), rounded to `prec` bits with `rnd`, returning
   the ordering of the result relative to the perturbed value. The infinitesimal is 2^-L, with L
   large enough that 2^-L is below the spacing of the rounding grid near q and below q's distance
   from that grid and its midpoints, when q is off it: such distances are multiples of 1 / (b *
   2^k) for q = a / b and a grid spacing 2^-k. */
static int
round_fmpq_plus_tiny(arf_t z, const fmpq_t q, int s, slong prec, arf_rnd_t rnd)
{
    slong e = fmpq_is_zero(q) ? 0 : rational_log_bound(q);
    slong l = 2 * (slong) fmpz_bits(fmpq_denref(q)) + FLINT_ABS(e) + prec + 64;
    fmpq_t t, d;
    fmpq_init(t);
    fmpq_init(d);
    fmpz_one(fmpq_numref(d));
    fmpz_one_2exp(fmpq_denref(d), l);
    if (s < 0)
        fmpq_neg(d, d);
    fmpq_add(t, q, d);
    int c = round_fmpq(z, t, prec, rnd);
    fmpq_clear(t);
    fmpq_clear(d);
    return c;
}

/* x rounded to `prec` bits with `rnd`, returning the ordering of the result relative to x. */
static int
round_arf(arf_t z, const arf_t x, slong prec, arf_rnd_t rnd)
{
    if (!arf_set_round(z, x, prec, rnd))
        return 0;
    switch (rnd)
    {
        case ARF_RND_FLOOR: return -1;
        case ARF_RND_CEIL: return 1;
        case ARF_RND_DOWN: return -arf_sgn(z);
        case ARF_RND_UP: return arf_sgn(z);
        default:
        {
            arf_t down;
            arf_init(down);
            arf_set_round(down, x, prec, ARF_RND_DOWN);
            int toward_zero = arf_equal(down, z);
            arf_clear(down);
            return toward_zero ? -arf_sgn(z) : arf_sgn(z);
        }
    }
}

/* The finite nonzero dyadic x plus an infinitesimal with the sign `s` (nonzero), rounded to `prec`
   bits with `rnd`. The infinitesimal is a power of 2 below both x's lowest set bit and the spacing
   of the rounding grid near x, so that it changes neither the rounding nor its direction. */
static int
round_arf_plus_tiny(arf_t z, const arf_t x, int s, slong prec, arf_rnd_t rnd)
{
    fmpz_t e;
    fmpz_init(e);
    fmpz_sub_ui(e, ARF_EXPREF(x), (ulong) FLINT_MAX(arf_bits(x), prec) + 2);
    arf_t t, tiny;
    arf_init(t);
    arf_init(tiny);
    arf_one(tiny);
    arf_mul_2exp_fmpz(tiny, tiny, e);
    if (s < 0)
        arf_neg(tiny, tiny);
    arf_add(t, x, tiny, ARF_PREC_EXACT, ARF_RND_DOWN);
    int c = round_arf(z, t, prec, rnd);
    arf_clear(t);
    arf_clear(tiny);
    fmpz_clear(e);
    return c;
}

/* Sets `q` to ±(|quo| mod 2^63) as a signed value, with the sign `sign` (nonzero). */
static long
low_quotient_bits(const fmpz_t quo, int sign)
{
    fmpz_t t;
    fmpz_init(t);
    fmpz_abs(t, quo);
    fmpz_fdiv_r_2exp(t, t, 63);
    long q = (long) fmpz_get_ui(t);
    fmpz_clear(t);
    return sign < 0 ? -q : q;
}

/* For the magnitudes N / D (the dividend) and A / D (the divisor), with remainder R = N mod A and
   truncated quotient `quo`: applies the nearest-quotient adjustment when `nearest` is set, turning R
   into R - A (negative) and incrementing quo exactly when 2R > A, or 2R = A and quo is odd. */
static void
nearest_adjust(fmpz_t r, fmpz_t quo, const fmpz_t a, int nearest)
{
    if (!nearest || fmpz_is_zero(r))
        return;
    fmpz_t twice;
    fmpz_init(twice);
    fmpz_mul_2exp(twice, r, 1);
    int c = fmpz_cmp(twice, a);
    if (c > 0 || (c == 0 && fmpz_is_odd(quo)))
    {
        fmpz_sub(r, r, a);
        fmpz_add_ui(quo, quo, 1);
    }
    fmpz_clear(twice);
}

/* The exact remainder of the rationals x by y (y nonzero), with the quotient rounded toward zero
   or to nearest with ties to even, rounded to `prec` bits; sets `*q` to the low quotient bits and
   returns the ordering. */
static int
rem_exact(arf_t z, long * q, const fmpq_t x, const fmpq_t y, int nearest, slong prec,
          arf_rnd_t rnd)
{
    fmpq_t ratio, r;
    fmpz_t quo, num, den;
    fmpq_init(ratio);
    fmpq_init(r);
    fmpz_init(quo);
    fmpz_init(num);
    fmpz_init(den);
    fmpq_div(ratio, x, y);
    fmpz_set(num, fmpq_numref(ratio));
    fmpz_set(den, fmpq_denref(ratio));
    if (!nearest)
        fmpz_tdiv_q(quo, num, den);
    else
    {
        /* floor(ratio + 1/2), lowered by 1 at an exact tie with an odd result */
        fmpz_t n2, d2, rem;
        fmpz_init(n2);
        fmpz_init(d2);
        fmpz_init(rem);
        fmpz_mul_2exp(n2, num, 1);
        fmpz_add(n2, n2, den);
        fmpz_mul_2exp(d2, den, 1);
        fmpz_fdiv_qr(quo, rem, n2, d2);
        if (fmpz_is_zero(rem) && fmpz_is_odd(quo))
            fmpz_sub_ui(quo, quo, 1);
        fmpz_clear(n2);
        fmpz_clear(d2);
        fmpz_clear(rem);
    }
    fmpq_mul_fmpz(r, y, quo);
    fmpq_sub(r, x, r);
    int c = round_fmpq(z, r, prec, rnd);
    *q = low_quotient_bits(quo, fmpz_sgn(quo));
    fmpq_clear(ratio);
    fmpq_clear(r);
    fmpz_clear(quo);
    fmpz_clear(num);
    fmpz_clear(den);
    return c;
}

/* c * 2^e mod m, for e >= 0, with fmpz_powm. */
static void
mul_power_of_2_mod(fmpz_t w, const fmpz_t c, const fmpz_t e, const fmpz_t m)
{
    fmpz_set_ui(w, 2);
    fmpz_powm(w, w, e, m);
    fmpz_mul(w, w, c);
    fmpz_mod(w, w, m);
}

/* An expected leaf: a value, an ordering, or an integer. */
typedef struct
{
    int kind; /* 0 value, 1 ordering, 2 integer */
    arf_t value;
    int ordering;
    slong integer;
} leaf_t;

static int
leaf_matches(const leaf_t * l, const char * s)
{
    switch (l->kind)
    {
        case 0:
        {
            arf_t v;
            arf_init(v);
            int ok = arf_parse_base(s, 4, v) == 0 && arf_equal(v, l->value);
            arf_clear(v);
            return ok;
        }
        case 1: return strcmp(s, ordering_name(l->ordering)) == 0;
        default:
        {
            char * end = NULL;
            long long k = strtoll(s, &end, 10);
            return *s != '\0' && *end == '\0' && k == l->integer;
        }
    }
}

/* x - y if x > y and 0 otherwise, for a BigExponentFloat x and a Rational y (when `flip` is 0),
   or for a Rational x and a BigExponentFloat y (when `flip` is 1, in which case `b` is y and `q`
   is x). Returns the ordering. */
static int
positive_difference(arf_t z, const arf_t b, const fmpq_t q, int flip, slong prec, arf_rnd_t rnd)
{
    if (arf_is_nan(b))
    {
        arf_nan(z);
        return 0;
    }
    if (arf_is_inf(b))
    {
        /* With flip, x - y is +inf exactly when y = -inf. */
        if ((!flip && arf_sgn(b) > 0) || (flip && arf_sgn(b) < 0))
            arf_pos_inf(z);
        else
            arf_zero(z);
        return 0;
    }
    int c = cmp_arf_fmpq(b, q);
    if (flip)
        c = -c;
    if (c <= 0)
    {
        arf_zero(z);
        return 0;
    }
    /* x - y = s * (b - q), for s = 1 without flip and -1 with it. */
    int s = flip ? -1 : 1;
    int result;
    if (arf_is_zero(b))
    {
        fmpq_t t;
        fmpq_init(t);
        fmpq_neg(t, q);
        if (flip)
            fmpq_neg(t, t);
        result = round_fmpq(z, t, prec, rnd);
        fmpq_clear(t);
    }
    else if (moderate(b))
    {
        fmpq_t t;
        fmpq_init(t);
        arf_get_fmpq(t, b);
        fmpq_sub(t, t, q);
        if (flip)
            fmpq_neg(t, t);
        result = round_fmpq(z, t, prec, rnd);
        fmpq_clear(t);
    }
    else if (fmpz_sgn(ARF_EXPREF(b)) > 0)
    {
        /* b is far larger than q: s * (b - q) = s * b - s * q, with q negligible. */
        if (rational_log_bound(q) + 2 >= 1 << 16 && !fmpq_is_zero(q))
            flint_abort();
        arf_t sb;
        arf_init(sb);
        arf_set(sb, b);
        if (flip)
            arf_neg(sb, sb);
        if (fmpq_is_zero(q))
            result = round_arf(z, sb, prec, rnd);
        else
            result = round_arf_plus_tiny(z, sb, -s * fmpq_sgn(q), prec, rnd);
        arf_clear(sb);
    }
    else
    {
        /* b is far smaller than q (which is nonzero, since b - q has the sign of s): s * (b - q) =
           -s * q + s * b, with b negligible. */
        if (-rational_log_bound(q) + 64 >= 1 << 16)
            flint_abort();
        fmpq_t t;
        fmpq_init(t);
        fmpq_set(t, q);
        if (!flip)
            fmpq_neg(t, t);
        result = round_fmpq_plus_tiny(z, t, s * arf_sgn(b), prec, rnd);
        fmpq_clear(t);
    }
    return result;
}

/* The remainder of a BigExponentFloat x by a Rational y, or of a Rational x by a BigExponentFloat y
   when `flip` is set (then `b` is y and `qv` is x). Sets `*q` to the quotient bits and returns the
   ordering. */
static int
remainder(arf_t z, long * q, const arf_t b, const fmpq_t qv, int flip, int nearest, slong prec,
          arf_rnd_t rnd)
{
    *q = 0;
    if (!flip)
    {
        if (arf_is_nan(b) || arf_is_inf(b) || fmpq_is_zero(qv))
        {
            arf_nan(z);
            return 0;
        }
        if (arf_is_zero(b))
        {
            arf_zero(z);
            return 0;
        }
    }
    else
    {
        if (arf_is_nan(b) || arf_is_zero(b))
        {
            arf_nan(z);
            return 0;
        }
        if (arf_is_inf(b))
            return round_fmpq(z, qv, prec, rnd);
        if (fmpq_is_zero(qv))
        {
            arf_zero(z);
            return 0;
        }
    }
    int sign_q = arf_sgn(b) * fmpq_sgn(qv);
    if (moderate(b))
    {
        fmpq_t bq;
        fmpq_init(bq);
        arf_get_fmpq(bq, b);
        int c = flip ? rem_exact(z, q, qv, bq, nearest, prec, rnd)
                     : rem_exact(z, q, bq, qv, nearest, prec, rnd);
        fmpq_clear(bq);
        return c;
    }
    int b_large = fmpz_sgn(ARF_EXPREF(b)) > 0;
    if (b_large == flip)
    {
        /* The dividend is far smaller than the divisor (by more than a factor of 2): it is its own
           remainder, with quotient 0. */
        return flip ? round_fmpq(z, qv, prec, rnd) : round_arf(z, b, prec, rnd);
    }
    /* The dividend is far larger than the divisor. */
    fmpz_t m, e, a, d, num, mod, w, quo, r;
    fmpz_init(m);
    fmpz_init(e);
    fmpz_init(a);
    fmpz_init(d);
    fmpz_init(num);
    fmpz_init(mod);
    fmpz_init(w);
    fmpz_init(quo);
    fmpz_init(r);
    arf_get_fmpz_2exp(m, e, b);
    fmpz_abs(m, m);
    fmpz_abs(a, fmpq_numref(qv));
    fmpz_set(d, fmpq_denref(qv));
    int dividend_sign = flip ? fmpq_sgn(qv) : arf_sgn(b);
    fmpq_t rq;
    fmpq_init(rq);
    int c;
    if (!flip)
    {
        /* |x| = m * 2^e with e huge, |y| = a / d: the remainder is ((m * d * 2^e) mod a) / d. */
        fmpz_mul_2exp(mod, a, 63);
        fmpz_mul(num, m, d);
        mul_power_of_2_mod(w, num, e, mod);
        fmpz_fdiv_qr(quo, r, w, a);
        nearest_adjust(r, quo, a, nearest);
        if (dividend_sign < 0)
            fmpz_neg(r, r);
        fmpq_set_fmpz_frac(rq, r, d);
        c = round_fmpq(z, rq, prec, rnd);
    }
    else
    {
        /* |x| = a / d, |y| = m * 2^e with e = -t hugely negative: the remainder is
           ((a * 2^t) mod (d * m)) / (d * 2^t). */
        fmpz_t t, dm;
        fmpz_init(t);
        fmpz_init(dm);
        fmpz_neg(t, e);
        fmpz_mul(dm, d, m);
        fmpz_mul_2exp(mod, dm, 63);
        mul_power_of_2_mod(w, a, t, mod);
        fmpz_fdiv_qr(quo, r, w, dm);
        nearest_adjust(r, quo, dm, nearest);
        if (dividend_sign < 0)
            fmpz_neg(r, r);
        fmpq_set_fmpz_frac(rq, r, d);
        c = round_fmpq(z, rq, prec, rnd);
        arf_mul_2exp_fmpz(z, z, e);
        fmpz_clear(t);
        fmpz_clear(dm);
    }
    *q = low_quotient_bits(quo, sign_q);
    fmpq_clear(rq);
    fmpz_clear(m);
    fmpz_clear(e);
    fmpz_clear(a);
    fmpz_clear(d);
    fmpz_clear(num);
    fmpz_clear(mod);
    fmpz_clear(w);
    fmpz_clear(quo);
    fmpz_clear(r);
    return c;
}

/* The operations, by name with any `_prec_round` or `_prec` suffix removed: whether the Rational
   comes first, and whether the operation is a remainder (with quotient rounded to nearest or not,
   and with quotient bits or not). */
typedef struct
{
    const char * name;
    int flip, rem, nearest, qbits;
} op_t;

static const op_t OPS[] = {
    {"positive_difference_rational", 0, 0, 0, 0},
    {"rem_rational", 0, 1, 0, 0},
    {"ieee_remainder_rational", 0, 1, 1, 0},
    {"rem_rational_and_quotient_bits", 0, 1, 0, 1},
    {"ieee_remainder_rational_and_quotient_bits", 0, 1, 1, 1},
    {"rational_positive_difference_big_exponent_float", 1, 0, 0, 0},
    {"rational_rem_big_exponent_float", 1, 1, 0, 0},
    {"rational_ieee_remainder_big_exponent_float", 1, 1, 1, 0},
    {"rational_rem_big_exponent_float_and_quotient_bits", 1, 1, 0, 1},
    {"rational_ieee_remainder_big_exponent_float_and_quotient_bits", 1, 1, 1, 1},
};

static int
check_line(char * line, int line_number)
{
    size_t len = strlen(line);
    if (len != 0 && line[len - 1] == '\n')
        line[len - 1] = '\0';
    if (line[0] == '\0')
        return 0;
    char * result = split(line, " = ");
    if (result == NULL)
        return report(line_number, "unrecognized line");
    /* `X.name(args)` or `BigExponentFloat::name(args)` */
    char * name;
    char * xtext = NULL;
    const char * prefix = "BigExponentFloat::";
    if (strncmp(line, prefix, strlen(prefix)) == 0)
        name = line + strlen(prefix);
    else
    {
        /* The `.` that begins the method call: one followed by an identifier and a `(`. The `.`
           inside a hexadecimal value is followed by hex digits that do not reach a `(`. */
        char * dot = NULL;
        for (char * p = strchr(line, '.'); p != NULL && dot == NULL; p = strchr(p + 1, '.'))
        {
            char * r = p + 1;
            while ((*r >= 'a' && *r <= 'z') || *r == '_' || (*r >= '0' && *r <= '9'))
                r++;
            if (r > p + 1 && *r == '(')
                dot = p;
        }
        if (dot == NULL)
            return report(line_number, "unrecognized line");
        *dot = '\0';
        xtext = line;
        name = dot + 1;
    }
    char * args = split(name, "(");
    size_t al = args == NULL ? 0 : strlen(args);
    if (args == NULL || al == 0 || args[al - 1] != ')')
        return report(line_number, "unrecognized line");
    args[al - 1] = '\0';
    int with_rm = 0;
    char * suffix = strstr(name, "_prec");
    if (suffix == NULL || (strcmp(suffix, "_prec_round") != 0 && strcmp(suffix, "_prec") != 0))
        return report(line_number, "missing precision");
    with_rm = strcmp(suffix, "_prec_round") == 0;
    *suffix = '\0';
    const op_t * op = NULL;
    for (size_t i = 0; i < sizeof(OPS) / sizeof(OPS[0]); i++)
        if (strcmp(OPS[i].name, name) == 0)
            op = &OPS[i];
    if (op == NULL || (xtext == NULL) != op->flip)
        return report(line_number, "unrecognized method");
    char * items[MAX_ARGS];
    int n_items = 0;
    for (char * p = args; *p != '\0' && n_items < MAX_ARGS;)
    {
        items[n_items++] = p;
        char * next = split(p, ", ");
        if (next == NULL)
            break;
        p = next;
    }
    int expected_items = (op->flip ? 2 : 1) + 1 + with_rm;
    if (n_items != expected_items)
        return report(line_number, "wrong number of arguments");
    arf_t b, z;
    fmpq_t q;
    arf_init(b);
    arf_init(z);
    fmpq_init(q);
    int res = 0;
    const char * btext = op->flip ? items[1] : xtext;
    const char * qtext = items[0];
    char * end = NULL;
    long long prec = strtoll(items[op->flip ? 2 : 1], &end, 10);
    arf_rnd_t rnd = ARF_RND_NEAR;
    int exact = 0;
    if (arf_parse_base(btext, 4, b) != 0)
        res = report(line_number, "unreadable value");
    else if (parse_rational(q, qtext) != 0)
        res = report(line_number, "unreadable rational");
    else if (*end != '\0' || prec <= 0)
        res = report(line_number, "unreadable precision");
    else if (with_rm && parse_rm(items[n_items - 1], &rnd, &exact) != 0)
        res = report(line_number, "unreadable rounding mode");
    if (res == 0)
    {
        leaf_t out[MAX_LEAVES];
        for (int i = 0; i < MAX_LEAVES; i++)
            arf_init(out[i].value);
        long qb = 0;
        int o = op->rem ? remainder(z, &qb, b, q, op->flip, op->nearest, prec, rnd)
                        : positive_difference(z, b, q, op->flip, prec, rnd);
        int n = 0;
        out[n].kind = 0;
        arf_set(out[n++].value, z);
        out[n].kind = 1;
        out[n++].ordering = o;
        if (op->qbits)
        {
            out[n].kind = 2;
            out[n++].integer = qb;
        }
        if (exact && o != 0)
            res = report(line_number, "Exact rounding of an inexact result");
        char * got[MAX_LEAVES];
        int m = leaves(result, got, MAX_LEAVES);
        if (res == 0 && m != n)
            res = report(line_number, "wrong number of results");
        for (int i = 0; i < n && res == 0; i++)
            if (!leaf_matches(&out[i], got[i]))
                res = report(line_number, out[i].kind == 1 ? "wrong ordering"
                                          : out[i].kind == 2 ? "wrong quotient bits"
                                                             : "wrong value");
        for (int i = 0; i < MAX_LEAVES; i++)
            arf_clear(out[i].value);
    }
    arf_clear(b);
    arf_clear(z);
    fmpq_clear(q);
    if (res == 0)
        checked++;
    return res;
}

int
run_arf_misc_rational(const char * arg)
{
    checked = 0;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines("arf_misc_rational", checked);
}
