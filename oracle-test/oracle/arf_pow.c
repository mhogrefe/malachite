/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs BigExponentFloat's general powers against FLINT. The mode `arf_pow` accepts lines of the
    forms `X.f_prec_round(Y, p, rm) = (Z, O)` for f among pow, powr, and pow_rational, and
    `BigExponentFloat::f_prec_round(X, Y, p, rm) = (Z, O)` for f among rational_pow,
    rational_pow_rational, unsigned_pow, unsigned_pow_rational, and unsigned_pow_unsigned, and their
    `_prec` forms. BigExponentFloats are in Malachite's exact hexadecimal format, Rationals are
    written as `a/b` or `a`, and u64s in decimal.

    The special cases follow Malachite's documentation (those of MPFR's pow and powr, with 0^y NaN
    for a negative odd integer y, and no negative zero).

    For finite nonzero values with |x| not 1, y is written as r/s in lowest terms (s = 1 for an
    integer) and the sign of an odd power of a negative x is set aside. The power |x|^y is a dyadic
    rational, and so possibly a rounding breakpoint, only in two ways that are computed exactly:
    |x| = 2^b with s dividing b r, giving 2^(b r / s); or |x|^(1/s) a dyadic rational t (with
    perfect s-th powers for numerator and denominator), when s <= 64 and |r| <= 2^16, giving t^r
    with fmpq arithmetic. (Otherwise, for a larger s an odd number greater than 1 would need more
    than s bits to be an s-th power, and for a larger |r| an exact power of a non-power of 2 has far
    more bits than any precision used here, so it is not a breakpoint.) Every other power is
    computed with arb_pow and rounded by bracketing it between consecutive numbers with one more bit
    than the target precision, the working precision doubling while a comparison is undecided.

    Any other nonempty line is an error, as is an input with no lines at all.
*/

#include <stdlib.h>
#include <string.h>

/* mpfr.h comes first, so that arf.h declares its MPFR conversions. */
#include <mpfr.h>

#include <flint/arb.h>
#include <flint/arf.h>
#include <flint/fmpq.h>
#include <flint/fmpz.h>

#include "oracle.h"

#define MAX_ARGS 4
#define MAX_PREC (1L << 24)

static long checked;

static int
report(int line_number, const char * what)
{
    flint_printf("line %d: %s\n", line_number, what);
    return 1;
}

static char *
split(char * s, const char * sep)
{
    char * p = strstr(s, sep);
    if (p == NULL)
        return NULL;
    *p = '\0';
    return p + strlen(sep);
}

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

static int
parse_rational(fmpq_t q, const char * s)
{
    size_t n = strlen(s);
    if (n == 0)
        return -1;
    char * buf = flint_malloc(n + 1);
    memcpy(buf, s, n + 1);
    char * slash = strchr(buf, '/');
    if (slash != NULL)
        *slash = '\0';
    int bad = fmpz_set_str(fmpq_numref(q), buf, 10) != 0;
    if (!bad)
    {
        if (slash == NULL)
            fmpz_one(fmpq_denref(q));
        else
            bad = fmpz_set_str(fmpq_denref(q), slash + 1, 10) != 0
                  || fmpz_sgn(fmpq_denref(q)) <= 0;
    }
    flint_free(buf);
    if (bad)
        return -1;
    fmpq_t c;
    fmpq_init(c);
    fmpq_set(c, q);
    fmpq_canonicalise(c);
    int ok = fmpq_equal(c, q);
    fmpq_clear(c);
    return ok ? 0 : -1;
}

/* A value that is NaN, an infinity, or a finite number held exactly as an fmpq. */
typedef struct
{
    int nan;
    int inf; /* 1 or -1 for an infinity, else 0 */
    fmpq_t q;
} value_t;

static void
value_init(value_t * v)
{
    v->nan = 0;
    v->inf = 0;
    fmpq_init(v->q);
}

static void
value_clear(value_t * v)
{
    fmpq_clear(v->q);
}

/* Parses a hexadecimal BigExponentFloat (only finite values with moderate exponents can become
   fmpqs; others are kept as arfs and converted lazily), a Rational, or a u64. */
typedef struct
{
    value_t v;
    int is_arf;
    arf_t a;
} arg_t;

static int
parse_arg(arg_t * x, const char * s, char kind)
{
    value_init(&x->v);
    arf_init(x->a);
    x->is_arf = 0;
    if (kind == 'B')
    {
        if (arf_parse_base(s, 4, x->a) != 0)
            return -1;
        x->is_arf = 1;
        if (arf_is_nan(x->a))
            x->v.nan = 1;
        else if (arf_is_inf(x->a))
            x->v.inf = arf_sgn(x->a);
        return 0;
    }
    if (kind == 'R')
        return parse_rational(x->v.q, s);
    fmpz_t n;
    fmpz_init(n);
    int bad = fmpz_set_str(n, s, 10) != 0 || fmpz_sgn(n) < 0;
    fmpz_set(fmpq_numref(x->v.q), n);
    fmpz_one(fmpq_denref(x->v.q));
    fmpz_clear(n);
    return bad ? -1 : 0;
}

static void
arg_clear(arg_t * x)
{
    value_clear(&x->v);
    arf_clear(x->a);
}

static int
arg_sgn(const arg_t * x)
{
    if (x->v.inf != 0)
        return x->v.inf;
    return x->is_arf ? arf_sgn(x->a) : fmpq_sgn(x->v.q);
}

static int
arg_is_zero(const arg_t * x)
{
    return !x->v.nan && x->v.inf == 0 && arg_sgn(x) == 0;
}

/* The finite x as m * 2^e (or exactly as the fmpq), for the dyadic case m odd. */
static void
arg_cmpabs_one(const arg_t * x, int * c)
{
    if (x->is_arf)
    {
        arf_t one;
        arf_init(one);
        arf_one(one);
        *c = arf_cmpabs(x->a, one);
        arf_clear(one);
    }
    else
        *c = fmpz_cmpabs(fmpq_numref(x->v.q), fmpq_denref(x->v.q));
}

/* y as r / s in lowest terms, for a finite y; returns 0 if that is too large to form (an arf y with
   a huge exponent is an integer: s = 1 and r is formed only when its exponent is moderate). */
static int
exponent_as_fraction(fmpz_t r, fmpz_t s, const arg_t * y)
{
    if (!y->is_arf)
    {
        fmpz_set(r, fmpq_numref(y->v.q));
        fmpz_set(s, fmpq_denref(y->v.q));
        return 1;
    }
    fmpz_t m, e;
    fmpz_init(m);
    fmpz_init(e);
    arf_get_fmpz_2exp(m, e, y->a);
    int ok = 1;
    if (fmpz_sgn(e) >= 0)
    {
        if (fmpz_cmp_ui(e, 1L << 20) > 0)
            ok = 0;
        else
        {
            fmpz_mul_2exp(r, m, fmpz_get_ui(e));
            fmpz_one(s);
        }
    }
    else
    {
        fmpz_set(r, m);
        fmpz_one(s);
        fmpz_neg(e, e);
        if (fmpz_cmp_ui(e, 1L << 20) > 0)
            ok = 0;
        else
            fmpz_mul_2exp(s, s, fmpz_get_ui(e));
    }
    fmpz_clear(m);
    fmpz_clear(e);
    return ok;
}

/* Whether the finite nonzero y is an integer, and if so whether it is odd. */
static void
exponent_integrality(const arg_t * y, int * integer, int * odd)
{
    if (!y->is_arf)
    {
        *integer = fmpz_is_one(fmpq_denref(y->v.q));
        *odd = *integer && fmpz_is_odd(fmpq_numref(y->v.q));
        return;
    }
    fmpz_t m, e;
    fmpz_init(m);
    fmpz_init(e);
    arf_get_fmpz_2exp(m, e, y->a);
    *integer = fmpz_sgn(e) >= 0;
    *odd = fmpz_is_zero(e);
    fmpz_clear(m);
    fmpz_clear(e);
}

/* The special values; returns 1 and sets z, or 0. */
static int
special(arf_t z, const arg_t * x, const arg_t * y, int powr)
{
    int x_nan = x->v.nan, y_nan = y->v.nan;
    int x_inf = x->v.inf, y_inf = y->v.inf;
    int x_zero = arg_is_zero(x), y_zero = arg_is_zero(y);
    int x_finite = !x_nan && !x_inf && !x_zero;
    int c = 0;
    if (x_finite)
        arg_cmpabs_one(x, &c);
    int x_one = x_finite && c == 0 && arg_sgn(x) > 0;
    int x_neg_one = x_finite && c == 0 && arg_sgn(x) < 0;
    int integer = 0, odd = 0;
    int y_finite = !y_nan && !y_inf && !y_zero;
    if (y_finite)
        exponent_integrality(y, &integer, &odd);
    int ys = arg_sgn(y);
    if (powr)
    {
        if (x_one && y_nan)
        {
            arf_one(z);
            return 1;
        }
        if (x_nan || y_nan || x_inf < 0 || (x_finite && arg_sgn(x) < 0)
            || ((x_zero || x_inf > 0) && y_zero) || (x_one && y_inf))
        {
            arf_nan(z);
            return 1;
        }
        if (y_zero || x_one)
        {
            arf_one(z);
            return 1;
        }
        if (x_inf > 0 || x_zero)
        {
            int big = (x_inf > 0) == (ys > 0);
            if (big)
                arf_pos_inf(z);
            else
                arf_zero(z);
            return 1;
        }
        if (y_inf)
        {
            if ((c > 0) == (y_inf > 0))
                arf_pos_inf(z);
            else
                arf_zero(z);
            return 1;
        }
        return 0;
    }
    if (y_zero || x_one)
    {
        arf_one(z);
        return 1;
    }
    if (x_nan || y_nan)
    {
        arf_nan(z);
        return 1;
    }
    if (y_inf)
    {
        if (x_neg_one)
            arf_one(z);
        else if (x_inf || (x_finite && c > 0))
        {
            if (y_inf > 0)
                arf_pos_inf(z);
            else
                arf_zero(z);
        }
        else /* |x| < 1, including 0 */
        {
            if (y_inf > 0)
                arf_zero(z);
            else
                arf_pos_inf(z);
        }
        return 1;
    }
    if (x_inf > 0)
    {
        if (ys > 0)
            arf_pos_inf(z);
        else
            arf_zero(z);
        return 1;
    }
    if (x_inf < 0)
    {
        if (ys > 0 && odd)
            arf_neg_inf(z);
        else if (ys > 0)
            arf_pos_inf(z);
        else
            arf_zero(z);
        return 1;
    }
    if (x_zero)
    {
        if (ys > 0)
            arf_zero(z);
        else if (odd)
            arf_nan(z);
        else
            arf_pos_inf(z);
        return 1;
    }
    if (x_neg_one)
    {
        if (!integer)
            arf_nan(z);
        else if (odd)
            arf_set_si(z, -1);
        else
            arf_one(z);
        return 1;
    }
    if (arg_sgn(x) < 0 && !integer)
    {
        arf_nan(z);
        return 1;
    }
    return 0;
}

static void
neighbor(arf_t r, const arf_t a, slong q, int up)
{
    fmpz_t e;
    fmpz_init(e);
    fmpz_set(e, ARF_EXPREF(a));
    fmpz_sub_ui(e, e, q + 10);
    arf_t tiny;
    arf_init(tiny);
    arf_one(tiny);
    arf_mul_2exp_fmpz(tiny, tiny, e);
    if (up)
    {
        arf_add(r, a, tiny, ARF_PREC_EXACT, ARF_RND_DOWN);
        arf_set_round(r, r, q, ARF_RND_CEIL);
    }
    else
    {
        arf_sub(r, a, tiny, ARF_PREC_EXACT, ARF_RND_DOWN);
        arf_set_round(r, r, q, ARF_RND_FLOOR);
    }
    arf_clear(tiny);
    fmpz_clear(e);
}

static int
sign_vs(const arb_t v, const arf_t w, slong wp)
{
    arb_t d;
    arb_init(d);
    arb_sub_arf(d, v, w, wp);
    int s = arb_is_positive(d) ? 1 : arb_is_negative(d) ? -1 : 2;
    arb_clear(d);
    return s;
}

/* |x|^y as an arb ball. */
static void
power_ball(arb_t v, const arg_t * x, const arg_t * y, slong wp)
{
    arb_t xb, yb;
    arb_init(xb);
    arb_init(yb);
    if (x->is_arf)
        arb_set_arf(xb, x->a);
    else
        arb_set_fmpq(xb, x->v.q, wp + 64);
    arb_abs(xb, xb);
    if (y->is_arf)
        arb_set_arf(yb, y->a);
    else
        arb_set_fmpq(yb, y->v.q, wp + 64);
    arb_pow(v, xb, yb, wp);
    arb_clear(xb);
    arb_clear(yb);
}

/* Rounds the positive |x|^y, known not to be a rounding breakpoint, by bracketing; returns the
   ordering, or 2 if undecided. */
static int
round_by_bracketing(arf_t z, const arg_t * x, const arg_t * y, slong prec, arf_rnd_t rnd)
{
    slong q = prec + 1;
    arb_t v;
    arf_t a, b, c;
    arb_init(v);
    arf_init(a);
    arf_init(b);
    arf_init(c);
    int result = 2;
    for (slong wp = prec + 64; result == 2 && wp <= MAX_PREC; wp *= 2)
    {
        power_ball(v, x, y, wp);
        if (!arb_is_finite(v) || arf_is_zero(arb_midref(v)))
            continue;
        arf_set_round(a, arb_midref(v), q, ARF_RND_FLOOR);
        for (int step = 0; step < 8; step++)
        {
            int sa = sign_vs(v, a, wp);
            if (sa == 2)
                break;
            if (sa < 0)
            {
                neighbor(a, a, q, 0);
                continue;
            }
            neighbor(b, a, q, 1);
            int sb = sign_vs(v, b, wp);
            if (sb == 2)
                break;
            if (sb > 0)
            {
                arf_set(a, b);
                continue;
            }
            /* a < |x|^y < b, so it rounds as their midpoint does */
            arf_add(c, a, b, ARF_PREC_EXACT, ARF_RND_DOWN);
            arf_mul_2exp_si(c, c, -1);
            arf_set_round(z, c, prec, rnd);
            result = arf_cmp(z, c) < 0 ? -1 : 1;
            break;
        }
    }
    arb_clear(v);
    arf_clear(a);
    arf_clear(b);
    arf_clear(c);
    return result;
}

/* Whether |x|^y is computed exactly here (see the header); if so, rounds it into z, sets *o, and
   returns 1. */
static int
exact_power(arf_t z, int * o, const arg_t * x, const arg_t * y, slong prec, arf_rnd_t rnd)
{
    fmpz_t r, s, xm, xe, t;
    fmpz_init(r);
    fmpz_init(s);
    fmpz_init(xm);
    fmpz_init(xe);
    fmpz_init(t);
    int done = 0;
    int have_fraction = exponent_as_fraction(r, s, y);
    /* a power of 2 base */
    int pow2 = 0;
    if (x->is_arf)
    {
        arf_get_fmpz_2exp(xm, xe, x->a);
        pow2 = fmpz_is_pm1(xm);
    }
    else if (fmpz_is_one(fmpq_denref(x->v.q)) || fmpz_is_pm1(fmpq_numref(x->v.q)))
    {
        const fmpz * n = fmpq_numref(x->v.q);
        const fmpz * d = fmpq_denref(x->v.q);
        if (fmpz_is_pm1(n) && (slong) fmpz_val2(d) + 1 == (slong) fmpz_bits(d))
        {
            pow2 = 1;
            fmpz_set_si(xe, -(slong) fmpz_val2(d));
        }
        else if (fmpz_is_one(d) && (slong) fmpz_val2(n) + 1 == (slong) fmpz_bits(n))
        {
            pow2 = 1;
            fmpz_set_ui(xe, fmpz_val2(n));
        }
    }
    if (pow2)
    {
        /* |x| = 2^xe; |x|^y = 2^(xe y) */
        if (y->is_arf && !have_fraction)
        {
            /* a huge integer y */
            fmpz_t m, e;
            fmpz_init(m);
            fmpz_init(e);
            arf_get_fmpz_2exp(m, e, y->a);
            fmpz_mul(t, xe, m);
            fmpz_mul_2exp(t, t, fmpz_get_ui(e));
            fmpz_clear(m);
            fmpz_clear(e);
            arf_one(z);
            arf_mul_2exp_fmpz(z, z, t);
            *o = 0;
            done = 1;
        }
        else
        {
            fmpz_mul(t, xe, r);
            if (fmpz_divisible(t, s))
            {
                fmpz_divexact(t, t, s);
                arf_one(z);
                arf_mul_2exp_fmpz(z, z, t);
                *o = 0;
                done = 1;
            }
        }
    }
    else if (have_fraction && fmpz_cmp_ui(s, 64) <= 0 && fmpz_bits(r) <= 16)
    {
        /* |x| = (a / b) 2^e with a and b odd; |x|^(1/s) is a dyadic rational exactly when s
           divides e and a and b are perfect s-th powers */
        fmpz_t a, b, e;
        fmpz_init(a);
        fmpz_init(b);
        fmpz_init(e);
        if (x->is_arf)
        {
            fmpz_abs(a, xm);
            fmpz_one(b);
            fmpz_set(e, xe);
        }
        else
        {
            fmpz_abs(a, fmpq_numref(x->v.q));
            fmpz_set(b, fmpq_denref(x->v.q));
            slong za = fmpz_val2(a), zb = fmpz_val2(b);
            fmpz_tdiv_q_2exp(a, a, za);
            fmpz_tdiv_q_2exp(b, b, zb);
            fmpz_set_si(e, za - zb);
        }
        ulong k = fmpz_get_ui(s);
        fmpz_t ar, br;
        fmpz_init(ar);
        fmpz_init(br);
        int ok = fmpz_divisible(e, s);
        if (ok)
        {
            fmpz_root(ar, a, k);
            fmpz_root(br, b, k);
            fmpz_pow_ui(t, ar, k);
            ok = fmpz_equal(t, a);
            fmpz_pow_ui(t, br, k);
            ok = ok && fmpz_equal(t, b);
        }
        if (ok)
        {
            /* |x|^y = (ar / br)^r 2^(e r / s); a dyadic rational iff the odd part has
               denominator 1 */
            fmpq_t v;
            fmpq_init(v);
            fmpz_set(fmpq_numref(v), ar);
            fmpz_set(fmpq_denref(v), br);
            fmpq_pow_si(v, v, fmpz_get_si(r));
            if (fmpz_is_one(fmpq_denref(v)))
            {
                fmpz_divexact(e, e, s);
                fmpz_mul(e, e, r);
                arf_t lo, hi;
                arf_init(lo);
                arf_init(hi);
                arf_set_fmpq(lo, v, prec, ARF_RND_FLOOR);
                arf_set_fmpq(hi, v, prec, ARF_RND_CEIL);
                arf_set_fmpq(z, v, prec, rnd);
                *o = arf_equal(lo, hi) ? 0 : arf_equal(z, lo) ? -1 : 1;
                arf_mul_2exp_fmpz(z, z, e);
                arf_clear(lo);
                arf_clear(hi);
                done = 1;
            }
            fmpq_clear(v);
        }
        fmpz_clear(ar);
        fmpz_clear(br);
        fmpz_clear(a);
        fmpz_clear(b);
        fmpz_clear(e);
    }
    fmpz_clear(r);
    fmpz_clear(s);
    fmpz_clear(xm);
    fmpz_clear(xe);
    fmpz_clear(t);
    return done;
}

static arf_rnd_t
mirror(arf_rnd_t rnd)
{
    return rnd == ARF_RND_FLOOR ? ARF_RND_CEIL : rnd == ARF_RND_CEIL ? ARF_RND_FLOOR : rnd;
}

static int
expected(arf_t z, const arg_t * x, const arg_t * y, int powr, slong prec, arf_rnd_t rnd)
{
    if (special(z, x, y, powr))
        return 0;
    int integer, odd;
    exponent_integrality(y, &integer, &odd);
    int negate = arg_sgn(x) < 0 && odd;
    arf_rnd_t r = negate ? mirror(rnd) : rnd;
    int o;
    if (!exact_power(z, &o, x, y, prec, r))
        o = round_by_bracketing(z, x, y, prec, r);
    if (negate && o != 2)
    {
        arf_neg(z, z);
        o = -o;
    }
    return o;
}

/* name, base kind, exponent kind, powr, receiver form */
static const struct
{
    const char * name;
    char xk, yk;
    int powr, recv;
} FNS[] = {
    {"pow", 'B', 'B', 0, 1},
    {"powr", 'B', 'B', 1, 1},
    {"pow_rational", 'B', 'R', 0, 1},
    {"rational_pow_rational", 'R', 'R', 0, 0},
    {"rational_pow", 'R', 'B', 0, 0},
    {"unsigned_pow_unsigned", 'U', 'U', 0, 0},
    {"unsigned_pow_rational", 'U', 'R', 0, 0},
    {"unsigned_pow", 'U', 'B', 0, 0},
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
    int f = -1;
    char * receiver = NULL;
    char * rest = NULL;
    const char * prefix = "BigExponentFloat::";
    for (int i = 0; i < 8 && f < 0; i++)
    {
        char pat[64];
        if (FNS[i].recv)
        {
            snprintf(pat, sizeof(pat), ".%s_prec", FNS[i].name);
            char * p = strstr(line, pat);
            if (p != NULL && strncmp(line, prefix, strlen(prefix)) != 0)
            {
                *p = '\0';
                receiver = line;
                rest = p + strlen(pat);
                f = i;
            }
        }
        else
        {
            snprintf(pat, sizeof(pat), "%s%s_prec", prefix, FNS[i].name);
            if (strncmp(line, pat, strlen(pat)) == 0)
            {
                rest = line + strlen(pat);
                f = i;
            }
        }
    }
    if (f < 0)
        return report(line_number, "unrecognized function");
    int with_rm;
    if (strncmp(rest, "_round(", 7) == 0)
    {
        with_rm = 1;
        rest += 7;
    }
    else if (rest[0] == '(')
    {
        with_rm = 0;
        rest += 1;
    }
    else
        return report(line_number, "unrecognized function");
    size_t rl = strlen(rest);
    if (rl == 0 || rest[rl - 1] != ')')
        return report(line_number, "unrecognized line");
    rest[rl - 1] = '\0';
    char * items[MAX_ARGS];
    int n_items = 0;
    for (char * p = rest; *p != '\0' && n_items < MAX_ARGS;)
    {
        items[n_items++] = p;
        char * next = split(p, ", ");
        if (next == NULL)
            break;
        p = next;
    }
    int recv = FNS[f].recv;
    if (n_items != (recv ? 1 : 2) + 1 + with_rm)
        return report(line_number, "wrong number of arguments");
    const char * xs = recv ? receiver : items[0];
    const char * ys = recv ? items[0] : items[1];
    char * end = NULL;
    long long prec = strtoll(items[recv ? 1 : 2], &end, 10);
    arg_t x, y;
    arf_t z, got;
    arf_init(z);
    arf_init(got);
    int res = 0;
    int px = parse_arg(&x, xs, FNS[f].xk);
    int py = parse_arg(&y, ys, FNS[f].yk);
    arf_rnd_t rnd = ARF_RND_NEAR;
    int exact = 0;
    if (px != 0 || py != 0)
        res = report(line_number, "unreadable argument");
    else if (*end != '\0' || prec <= 0)
        res = report(line_number, "unreadable precision");
    else if (with_rm && parse_rm(items[n_items - 1], &rnd, &exact) != 0)
        res = report(line_number, "unreadable rounding mode");
    if (res == 0)
    {
        int o = expected(z, &x, &y, FNS[f].powr, prec, rnd);
        size_t vl = strlen(result);
        if (vl < 2 || result[0] != '(' || result[vl - 1] != ')')
            res = report(line_number, "unrecognized result");
        else
        {
            result[vl - 1] = '\0';
            char * value = result + 1;
            char * ordering = split(value, ", ");
            if (o == 2)
                res = report(line_number, "working precision limit reached");
            else if (exact && o != 0)
                res = report(line_number, "Exact rounding of an inexact result");
            else if (ordering == NULL || arf_parse_base(value, 4, got) != 0)
                res = report(line_number, "unreadable result");
            else if (!arf_equal(got, z))
                res = report(line_number, "wrong value");
            else if (strcmp(ordering, ordering_name(o)) != 0)
                res = report(line_number, "wrong ordering");
        }
    }
    arg_clear(&x);
    arg_clear(&y);
    arf_clear(z);
    arf_clear(got);
    if (res == 0)
        checked++;
    return res;
}

int
run_arf_pow(const char * arg)
{
    checked = 0;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines("arf_pow", checked);
}
