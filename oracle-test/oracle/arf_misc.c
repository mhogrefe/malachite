/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs BigExponentFloat's averages, positive differences, remainders, rounding to integers,
    fractional parts, and unit and Gaussian-unit functions against MPFR and FLINT. Values are
    written in Malachite's exact hexadecimal format. The mode `arf_misc` accepts lines of the forms

    - `X.op_prec_round(args, p, rm) = R`, `X.op_prec(args, p) = R`, and `X.op(args) = R`, where
      the args are values, integers, and rounding modes, and R is a value, an integer, `true` or
      `false`, an ordering, or a parenthesized tuple of these (possibly nested);
    - `X % Y = Z`, the exact remainder;
    - `BigExponentFloat::power_of_2(k) = X`, checked against 2^k built with arf_mul_2exp_fmpz;
    - `X is a power of 2` and `X is not a power of 2`.

    A remainder by a `u64`, `rem_unsigned`, is checked as the remainder by the exact value of the
    `u64`.

    Averages and positive differences are computed with FLINT's arf_add and arf_sub (an average is a
    rounded sum, halved exactly), whose exponents are unbounded; the ordering of a rounded result
    follows from the rounding mode, the result's sign, and, for `Nearest`, a comparison with the
    result rounded toward zero. The other references are MPFR's mpfr_fmod, mpfr_remainder,
    mpfr_remquo, mpfr_fmodquo, mpfr_rint, mpfr_round, mpfr_rint_round, mpfr_rint_trunc, and
    mpfr_frac, with each value converted exactly to MPFR and MPFR's exponent range widened to its
    maximum, about 2^62; an MPFR ternary value gives the ordering directly, and mpfr_rint's also
    tells whether the input was an integer. A two-step rounding to an integer and then to a
    precision is an exact mpfr_rint followed by mpfr_set.

    Exponents beyond MPFR's range are reduced first. A remainder scales with its operands, so both
    are divided by 2^EXP(y); if x's exponent is then still out of range, x is far larger than y, and
    with x = a * 2^s and y = b * 2^t for integers a and b, the remainder is (a * (2^(s - t) mod B)
    mod B) mod b * 2^t for B = b * 2^63, computed with fmpz_powm, and the low 63 bits of the
    truncated quotient are (that product mod B) / b; if x is far smaller than y, the remainder is
    x and the quotient 0. For rounding to an integer and fractional parts, an x with a huge
    exponent is an integer, and stays one, with the same rounding to `prec` bits, when divided by
    a power of 2 that leaves its exponent above its bit count and `prec`; an x with a hugely
    negative exponent is less than 1/2 in magnitude, and stays so, with the same roundings to an
    integer, when its exponent is moved to -1, while its fractional part, x itself, scales back.

    The special cases follow Malachite's documentation where they differ from MPFR's: there is no
    negative zero; rounding NaN or an infinity to an integer gives it back, as a non-integer; the
    fractional part of an infinity is zero; and the remainders' quotient bits are 0 in the special
    cases. MPFR's quotient bits of -2^63, which it reports for a quotient whose magnitude is a
    multiple of 2^63, are read as 0. An exact remainder is computed at a precision large enough to
    hold it, and the oracle aborts if that precision would exceed 2^20 bits.

    Any other nonempty line is an error, as is an input with no lines at all.
*/

#include <limits.h>
#include <stdlib.h>
#include <string.h>

/* mpfr.h comes first, so that arf.h declares its MPFR conversions. */
#include <mpfr.h>

#include <flint/arf.h>
#include <flint/fmpz.h>
#include <flint/fmpz_vec.h>

#include "oracle.h"

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

/* The rounding mode named `s`, or -1. `Exact` maps to MPFR_RNDZ and sets `*exact`. */
static int
parse_rm(const char * s, mpfr_rnd_t * rnd, int * exact)
{
    *exact = 0;
    if (strcmp(s, "Floor") == 0)
        *rnd = MPFR_RNDD;
    else if (strcmp(s, "Ceiling") == 0)
        *rnd = MPFR_RNDU;
    else if (strcmp(s, "Down") == 0)
        *rnd = MPFR_RNDZ;
    else if (strcmp(s, "Up") == 0)
        *rnd = MPFR_RNDA;
    else if (strcmp(s, "Nearest") == 0)
        *rnd = MPFR_RNDN;
    else if (strcmp(s, "Exact") == 0)
    {
        *rnd = MPFR_RNDZ;
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

/* Splits a tuple like `((A, B), (C, D))` into its leaves; parentheses carry no information here,
   since every leaf is self-describing. Returns the number of leaves, at most `max`. */
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

/* The `.` that begins the method call in `line`, as in arf_arithmetic.c. */
static char *
find_method(char * line)
{
    for (char * p = strchr(line, '.'); p != NULL; p = strchr(p + 1, '.'))
    {
        char * q = p + 1;
        while ((*q >= 'a' && *q <= 'z') || *q == '_' || (*q >= '0' && *q <= '9'))
            q++;
        if (q > p + 1 && *q == '(')
            return p;
    }
    return NULL;
}

/* An MPFR copy of `x`, exact, with `x`'s precision (at least 1). */
static void
to_mpfr(mpfr_t f, const arf_t x)
{
    mpfr_init2(f, FLINT_MAX(arf_bits(x), 1));
    arf_get_mpfr(f, x, MPFR_RNDN);
}

/* The lowest set bit position of the finite nonzero `x`: its exponent minus its bit count. */
static void
low_bit(fmpz_t t, const arf_t x)
{
    fmpz_sub_ui(t, ARF_EXPREF(x), arf_bits(x));
}

/* A precision large enough to hold any remainder of x by y exactly: the remainder is a multiple of
   2^min(low(x), low(y)) and smaller than 2^EXP(y) in magnitude. */
static slong
remainder_prec(const arf_t x, const arf_t y)
{
    if (!arf_is_finite(x) || !arf_is_finite(y) || arf_is_zero(x) || arf_is_zero(y))
        return FLINT_MAX(arf_bits(x), 1);
    fmpz_t lx, ly, p;
    fmpz_init(lx);
    fmpz_init(ly);
    fmpz_init(p);
    low_bit(lx, x);
    low_bit(ly, y);
    if (fmpz_cmp(lx, ly) > 0)
        fmpz_swap(lx, ly);
    fmpz_sub(p, ARF_EXPREF(y), lx);
    slong prec = FLINT_MAX(fmpz_get_si(p), (slong) arf_bits(x));
    if (fmpz_cmp_si(p, WORD(1) << 20) > 0)
        flint_abort();
    fmpz_clear(lx);
    fmpz_clear(ly);
    fmpz_clear(p);
    return FLINT_MAX(prec, 1);
}

/* An expected leaf: a value, an ordering, an integer, or a boolean. */
typedef struct
{
    int kind; /* 0 value, 1 ordering, 2 integer, 3 boolean */
    arf_t value;
    int ordering;
    slong integer;
} leaf_t;

/* Whether the exponent of the finite nonzero `x` is safely inside MPFR's widened range. */
static int
exponent_in_range(const arf_t x)
{
    return fmpz_bits(ARF_EXPREF(x)) <= 61;
}

/* The direction of a rounded arf result z of an operation `op` (0: x + y, 1: x - y), from FLINT's
   inexact flag: known for the directed modes, and, for `Nearest`, found by recomputing the result
   rounded toward zero. */
static int
arf_direction(int op, const arf_t z, const arf_t x, const arf_t y, int inexact, slong prec,
              mpfr_rnd_t rnd)
{
    if (!inexact)
        return 0;
    switch (rnd)
    {
        case MPFR_RNDD: return -1;
        case MPFR_RNDU: return 1;
        case MPFR_RNDZ: return -arf_sgn(z);
        case MPFR_RNDA: return arf_sgn(z);
        default:
        {
            arf_t down;
            arf_init(down);
            if (op == 0)
                arf_add(down, x, y, prec, ARF_RND_DOWN);
            else
                arf_sub(down, x, y, prec, ARF_RND_DOWN);
            int toward_zero = arf_equal(down, z);
            arf_clear(down);
            return toward_zero ? -arf_sgn(z) : arf_sgn(z);
        }
    }
}

static arf_rnd_t
arf_rnd(mpfr_rnd_t rnd)
{
    switch (rnd)
    {
        case MPFR_RNDD: return ARF_RND_FLOOR;
        case MPFR_RNDU: return ARF_RND_CEIL;
        case MPFR_RNDZ: return ARF_RND_DOWN;
        case MPFR_RNDA: return ARF_RND_UP;
        default: return ARF_RND_NEAR;
    }
}

/* Whether |x| <= |y| / 2, for finite nonzero x and y. */
static int
half_or_less(const arf_t x, const arf_t y)
{
    arf_t t;
    arf_init(t);
    arf_mul_2exp_si(t, x, 1);
    int c = arf_cmpabs(t, y);
    arf_clear(t);
    return c <= 0;
}

/* `x` rounded to `prec` bits (0: exact) with `rnd`, returning the ordering of the result relative
   to `x`. */
static int
round_with_direction(arf_t z, const arf_t x, slong prec, mpfr_rnd_t rnd)
{
    if (prec == 0)
    {
        arf_set(z, x);
        return 0;
    }
    if (!arf_set_round(z, x, prec, arf_rnd(rnd)))
        return 0;
    switch (rnd)
    {
        case MPFR_RNDD: return -1;
        case MPFR_RNDU: return 1;
        case MPFR_RNDZ: return -arf_sgn(z);
        case MPFR_RNDA: return arf_sgn(z);
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

/* The remainder of x by y, finite and nonzero, when x's exponent is far beyond MPFR's range and
   y's is 0, as described above; with `nearest`, the quotient is rounded to the nearest integer
   with ties to even. Sets `*q` to the low quotient bits, and returns the ordering of the rounded
   remainder. */
static int
rem_huge(leaf_t * out, long * q, const arf_t x, const arf_t y, int nearest, slong prec,
         mpfr_rnd_t rnd)
{
    fmpz_t a, s, b, t, g, m, w, quo, r;
    fmpz_init(a);
    fmpz_init(s);
    fmpz_init(b);
    fmpz_init(t);
    fmpz_init(g);
    fmpz_init(m);
    fmpz_init(w);
    fmpz_init(quo);
    fmpz_init(r);
    arf_get_fmpz_2exp(a, s, x);
    arf_get_fmpz_2exp(b, t, y);
    fmpz_abs(a, a);
    fmpz_abs(b, b);
    fmpz_sub(g, s, t);
    fmpz_mul_2exp(m, b, 63);
    fmpz_set_ui(w, 2);
    fmpz_powm(w, w, g, m);
    fmpz_mul(w, w, a);
    fmpz_mod(w, w, m);
    fmpz_fdiv_qr(quo, r, w, b);
    if (nearest)
    {
        fmpz_t twice;
        fmpz_init(twice);
        fmpz_mul_2exp(twice, r, 1);
        int c = fmpz_cmp(twice, b);
        if (c > 0 || (c == 0 && fmpz_is_odd(quo)))
        {
            fmpz_sub(r, r, b);
            fmpz_add_ui(quo, quo, 1);
            fmpz_fdiv_r_2exp(quo, quo, 63);
        }
        fmpz_clear(twice);
    }
    if (arf_sgn(x) < 0)
        fmpz_neg(r, r);
    arf_t exact;
    arf_init(exact);
    arf_set_fmpz_2exp(exact, r, t);
    out->kind = 0;
    int dir = round_with_direction(out->value, exact, prec, rnd);
    *q = (long) fmpz_get_ui(quo);
    if ((arf_sgn(x) < 0) != (arf_sgn(y) < 0))
        *q = -*q;
    arf_clear(exact);
    fmpz_clear(a);
    fmpz_clear(s);
    fmpz_clear(b);
    fmpz_clear(t);
    fmpz_clear(g);
    fmpz_clear(m);
    fmpz_clear(w);
    fmpz_clear(quo);
    fmpz_clear(r);
    return dir;
}


static void
set_value_mpfr(leaf_t * l, const mpfr_t f)
{
    l->kind = 0;
    arf_set_mpfr(l->value, f);
}

static void
set_value_arf(leaf_t * l, const arf_t x)
{
    l->kind = 0;
    arf_set(l->value, x);
}

static void
set_ordering(leaf_t * l, int t)
{
    l->kind = 1;
    l->ordering = t;
}

/* Compares a computed leaf with its printed form. */
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
        case 2:
        {
            char * end = NULL;
            long long k = strtoll(s, &end, 10);
            return *s != '\0' && *end == '\0' && k == l->integer;
        }
        default: return strcmp(s, l->integer ? "true" : "false") == 0;
    }
}

/* Rounds x to an integer exactly with `rnd` (or, with `ties_away`, to the nearest integer with ties
   away from zero), returning mpfr_rint's (or mpfr_round's) ternary value. */
static int
rint_exact(mpfr_t z, const mpfr_t x, mpfr_rnd_t rnd, int ties_away)
{
    mpfr_set_prec(z, mpfr_get_prec(x) + 1);
    return ties_away ? mpfr_round(z, x) : mpfr_rint(z, x, rnd);
}

/* The value, ordering, and integer flag of a rounding of x to an integer, with Malachite's special
   cases; `prec` 0 means exact. */
static void
round_to_integer(leaf_t * out, const arf_t x, slong prec, mpfr_rnd_t rnd, int ties_away)
{
    if (arf_is_nan(x) || arf_is_inf(x) || arf_is_zero(x))
    {
        set_value_arf(&out[0], x);
        set_ordering(&out[1], 0);
        out[2].kind = 3;
        out[2].integer = arf_is_zero(x);
        return;
    }
    mpfr_t xf, z;
    to_mpfr(xf, x);
    mpfr_init2(z, prec == 0 ? mpfr_get_prec(xf) + 1 : prec);
    int t = ties_away ? mpfr_round(z, xf) : mpfr_rint(z, xf, rnd);
    set_value_mpfr(&out[0], z);
    set_ordering(&out[1], t);
    out[2].kind = 3;
    out[2].integer = t != 2 && t != -2;
    mpfr_clear(xf);
    mpfr_clear(z);
}

/* The fractional part of x, with Malachite's special cases; `prec` 0 means exact. */
static void
fractional_part(leaf_t * out, const mpfr_t xf, const arf_t x, slong prec, mpfr_rnd_t rnd)
{
    if (arf_is_nan(x) || arf_is_inf(x) || arf_is_zero(x))
    {
        if (arf_is_nan(x))
            arf_nan(out[0].value);
        else
            arf_zero(out[0].value);
        out[0].kind = 0;
        set_ordering(&out[1], 0);
        return;
    }
    mpfr_t z;
    mpfr_init2(z, prec == 0 ? mpfr_get_prec(xf) : prec);
    int t = mpfr_frac(z, xf, rnd);
    set_value_mpfr(&out[0], z);
    set_ordering(&out[1], t);
    mpfr_clear(z);
}

/* The integer part (trunc) of x, rounded to `prec` bits (0: exact), with Malachite's special
   cases. */
static void
integer_part(leaf_t * out, const mpfr_t xf, const arf_t x, slong prec, mpfr_rnd_t rnd)
{
    if (arf_is_nan(x) || arf_is_inf(x) || arf_is_zero(x))
    {
        set_value_arf(&out[0], x);
        set_ordering(&out[1], 0);
        return;
    }
    mpfr_t z;
    mpfr_init2(z, prec == 0 ? mpfr_get_prec(xf) : prec);
    int t = mpfr_rint_trunc(z, xf, rnd);
    set_value_mpfr(&out[0], z);
    set_ordering(&out[1], t);
    mpfr_clear(z);
}

/* Computes the expected leaves of `X.name(args)`, where `name` has had any `_prec_round` or `_prec`
   suffix removed (`with_prec` and `with_rm` record which), `xs` holds the value arguments, `ks`
   the integer arguments, and `rms` the rounding-mode arguments, in order. Returns the number of
   leaves, or -1 for an unrecognized method. */
static int
compute(leaf_t * out, const char * name, const arf_t x, arf_srcptr ys, int n_ys, const slong * ks,
        int n_ks, const mpfr_rnd_t * rms, int n_rms, int with_prec, int with_rm)
{
    slong prec = with_prec ? ks[0] : 0;
    mpfr_rnd_t rnd = with_rm ? rms[n_rms - 1] : MPFR_RNDN;
    mpfr_t xf, z;
    int n = -1;
    /* For the unary operations, an x with an exponent beyond MPFR's range is moved into it, as
       described above: `scale` is 1 if integer-valued results must be multiplied back by 2^shift,
       and 2 if fractional ones must. */
    arf_t xr;
    fmpz_t shift;
    arf_init(xr);
    fmpz_init(shift);
    arf_set(xr, x);
    int scale = 0;
    int rounding_op = strncmp(name, "round_to_integer", 16) == 0
        || strcmp(name, "fractional_part") == 0
        || strcmp(name, "integer_and_fractional_parts") == 0;
    if (rounding_op && arf_is_finite(x) && !arf_is_zero(x) && !exponent_in_range(x))
    {
        slong target;
        if (fmpz_sgn(ARF_EXPREF(x)) > 0)
        {
            slong p = 0;
            for (int i = 0; i < n_ks; i++)
                p = FLINT_MAX(p, ks[i]);
            target = arf_bits(x) + p + 2;
            scale = 1;
        }
        else
        {
            target = -1;
            scale = 2;
        }
        fmpz_sub_si(shift, ARF_EXPREF(x), target);
        fmpz_t neg;
        fmpz_init(neg);
        fmpz_neg(neg, shift);
        arf_mul_2exp_fmpz(xr, xr, neg);
        fmpz_clear(neg);
    }
    if (n_ys == 0)
        to_mpfr(xf, xr);
    else
        mpfr_init2(xf, 2);
    x = xr;
    mpfr_init2(z, 2);
    int binary = n_ys == 1 && n_ks == with_prec && n_rms == with_rm;
    if (binary && (strcmp(name, "average") == 0 || strcmp(name, "positive_difference") == 0))
    {
        int average = name[0] == 'a';
        if (prec == 0)
        {
            arf_t t;
            arf_init(t);
            if (average)
            {
                arf_add(t, x, ys, ARF_PREC_EXACT, ARF_RND_DOWN);
                arf_mul_2exp_si(t, t, -1);
            }
            else if (arf_is_nan(x) || arf_is_nan(ys))
                arf_nan(t);
            else if (arf_cmp(x, ys) > 0)
                arf_sub(t, x, ys, ARF_PREC_EXACT, ARF_RND_DOWN);
            else
                arf_zero(t);
            set_value_arf(&out[0], t);
            arf_clear(t);
            n = 1;
        }
        else
        {
            arf_t t;
            arf_init(t);
            int dir = 0;
            if (average)
            {
                int inexact = arf_add(t, x, ys, prec, arf_rnd(rnd));
                dir = arf_direction(0, t, x, ys, inexact, prec, rnd);
                arf_mul_2exp_si(t, t, -1);
            }
            else if (arf_is_nan(x) || arf_is_nan(ys))
                arf_nan(t);
            else if (arf_cmp(x, ys) > 0)
            {
                int inexact = arf_sub(t, x, ys, prec, arf_rnd(rnd));
                dir = arf_direction(1, t, x, ys, inexact, prec, rnd);
            }
            else
                arf_zero(t);
            set_value_arf(&out[0], t);
            set_ordering(&out[1], dir);
            arf_clear(t);
            n = 2;
        }
    }
    else if (binary && (strcmp(name, "rem") == 0 || strcmp(name, "ieee_remainder") == 0
                        || strcmp(name, "rem_and_quotient_bits") == 0
                        || strcmp(name, "ieee_remainder_and_quotient_bits") == 0))
    {
        long q = 0;
        int t = 0;
        fmpz_t shift;
        fmpz_init(shift);
        int finite = arf_is_finite(x) && arf_is_finite(ys) && !arf_is_zero(x) && !arf_is_zero(ys);
        arf_t xs, yy;
        arf_init(xs);
        arf_init(yy);
        arf_set(xs, x);
        arf_set(yy, ys);
        if (finite)
        {
            /* Divide both by 2^EXP(y). */
            fmpz_set(shift, ARF_EXPREF(ys));
            fmpz_t neg;
            fmpz_init(neg);
            fmpz_neg(neg, shift);
            arf_mul_2exp_fmpz(xs, xs, neg);
            arf_mul_2exp_fmpz(yy, yy, neg);
            fmpz_clear(neg);
        }
        if (finite && !exponent_in_range(xs) && fmpz_sgn(ARF_EXPREF(xs)) > 0)
        {
            t = rem_huge(&out[0], &q, xs, yy, name[0] == 'i', prec, rnd);
            arf_mul_2exp_fmpz(out[0].value, out[0].value, shift);
        }
        else if (finite && half_or_less(xs, yy))
        {
            /* |x| <= |y| / 2: the remainder is x, for both kinds of quotient (a quotient of 1/2
               rounds to the even 0). */
            out[0].kind = 0;
            t = round_with_direction(out[0].value, x, prec, rnd);
        }
        else
        {
            mpfr_t xsf, ysf;
            to_mpfr(xsf, xs);
            to_mpfr(ysf, yy);
            mpfr_set_prec(z, prec == 0 ? remainder_prec(xs, yy) : prec);
            if (name[0] == 'i')
                t = mpfr_remquo(z, &q, xsf, ysf, rnd);
            else
                t = mpfr_fmodquo(z, &q, xsf, ysf, rnd);
            set_value_mpfr(&out[0], z);
            arf_mul_2exp_fmpz(out[0].value, out[0].value, shift);
            mpfr_clear(xsf);
            mpfr_clear(ysf);
        }
        if (!finite)
            q = 0;
        /* MPFR reports a quotient whose magnitude is a positive multiple of 2^63 as LONG_MIN,
           -2^63, rather than as 0, its low 63 bits; Malachite reports ±(|q| mod 2^63). */
        if (q == LONG_MIN)
            q = 0;
        arf_clear(xs);
        arf_clear(yy);
        fmpz_clear(shift);
        n = 1;
        if (prec != 0)
            set_ordering(&out[n++], t);
        if (strcmp(name, "rem_and_quotient_bits") == 0
            || strcmp(name, "ieee_remainder_and_quotient_bits") == 0)
        {
            out[n].kind = 2;
            out[n++].integer = q;
        }
    }
    else if (strncmp(name, "round_to_integer", 16) == 0 && n_ys == 0)
    {
        const char * rest = name + 16;
        if (strcmp(rest, "_then") == 0 && n_rms == 2 && n_ks == 1 && with_prec && with_rm)
        {
            /* round_to_integer_then_prec_round(irm, p, rm) */
            if (arf_is_nan(x) || arf_is_inf(x) || arf_is_zero(x))
            {
                set_value_arf(&out[0], x);
                set_ordering(&out[1], 0);
            }
            else
            {
                mpfr_t i;
                mpfr_init2(i, 2);
                rint_exact(i, xf, rms[0], 0);
                mpfr_set_prec(z, prec);
                int t = mpfr_set(z, i, rnd);
                set_value_mpfr(&out[0], z);
                set_ordering(&out[1], t);
                mpfr_clear(i);
            }
            n = 2;
        }
        else if (strcmp(rest, "_ties_away_then") == 0 && n_rms == 1 && n_ks == 1 && with_prec
                 && with_rm)
        {
            if (arf_is_nan(x) || arf_is_inf(x) || arf_is_zero(x))
            {
                set_value_arf(&out[0], x);
                set_ordering(&out[1], 0);
            }
            else
            {
                mpfr_set_prec(z, prec);
                int t = mpfr_rint_round(z, xf, rnd);
                set_value_mpfr(&out[0], z);
                set_ordering(&out[1], t);
            }
            n = 2;
        }
        else if (strcmp(rest, "_round") == 0 && n_rms == 1 && n_ks == 0)
        {
            round_to_integer(out, x, 0, rms[0], 0);
            n = 3;
        }
        else if ((*rest == '\0' || strcmp(rest, "_ties_away") == 0) && n_rms == with_rm
                 && n_ks == with_prec)
        {
            round_to_integer(out, x, prec, rnd, *rest != '\0');
            n = 3;
        }
    }
    else if (strcmp(name, "fractional_part") == 0 && n_ys == 0 && n_ks == with_prec
             && n_rms == with_rm)
    {
        fractional_part(out, xf, x, prec, rnd);
        n = prec == 0 ? 1 : 2;
    }
    else if (strcmp(name, "integer_and_fractional_parts") == 0 && n_ys == 0)
    {
        if (n_ks == 0 && n_rms == 0)
        {
            leaf_t parts[2];
            arf_init(parts[0].value);
            arf_init(parts[1].value);
            integer_part(parts, xf, x, 0, MPFR_RNDZ);
            arf_set(out[0].value, parts[0].value);
            out[0].kind = 0;
            fractional_part(parts, xf, x, 0, MPFR_RNDZ);
            arf_set(out[1].value, parts[0].value);
            out[1].kind = 0;
            arf_clear(parts[0].value);
            arf_clear(parts[1].value);
            n = 2;
        }
        else if (n_ks == 2 && n_rms == with_rm)
        {
            integer_part(out, xf, x, ks[0], rnd);
            fractional_part(out + 2, xf, x, ks[1], rnd);
            n = 4;
        }
    }
    else if (n_ys == 0 && n_ks == 0 && n_rms == 0 && !with_prec)
    {
        int finite_nonzero = arf_is_finite(x) && !arf_is_zero(x);
        arf_t t;
        arf_init(t);
        n = 1;
        if (strcmp(name, "conjugate") == 0)
            set_value_arf(&out[0], x);
        else if (strcmp(name, "abs_squared") == 0)
        {
            arf_mul(t, x, x, ARF_PREC_EXACT, ARF_RND_DOWN);
            set_value_arf(&out[0], t);
        }
        else if (strcmp(name, "is_unit") == 0)
        {
            out[0].kind = 3;
            out[0].integer = finite_nonzero;
        }
        else if (strcmp(name, "canonicalize_unit") == 0)
        {
            if (finite_nonzero)
                arf_one(t);
            else
                arf_abs(t, x);
            set_value_arf(&out[0], t);
        }
        else if (strcmp(name, "canonicalize_gaussian_unit") == 0)
        {
            arf_abs(t, x);
            set_value_arf(&out[0], t);
        }
        else if (strcmp(name, "canonical_unit_i_pow") == 0)
        {
            out[0].kind = 2;
            out[0].integer = !arf_is_nan(x) && arf_sgn(x) < 0 ? 2 : 0;
        }
        else
            n = -1;
        arf_clear(t);
    }
    if (scale != 0 && n > 0)
    {
        /* Which leaves are integer-valued and which fractional. */
        int integer_leaves[2] = {-1, -1}, fraction_leaves[2] = {-1, -1};
        if (strncmp(name, "round_to_integer", 16) == 0)
            integer_leaves[0] = 0;
        else if (strcmp(name, "fractional_part") == 0)
            fraction_leaves[0] = 0;
        else if (strcmp(name, "integer_and_fractional_parts") == 0)
        {
            integer_leaves[0] = 0;
            fraction_leaves[0] = n == 2 ? 1 : 2;
        }
        int * leaves_to_scale = scale == 1 ? integer_leaves : fraction_leaves;
        for (int i = 0; i < 2; i++)
            if (leaves_to_scale[i] >= 0)
                arf_mul_2exp_fmpz(out[leaves_to_scale[i]].value, out[leaves_to_scale[i]].value,
                                  shift);
    }
    arf_clear(xr);
    fmpz_clear(shift);
    mpfr_clear(xf);
    mpfr_clear(z);
    return n;
}

#define MAX_LEAVES 8
#define MAX_ARGS 8

static int
check_line(char * line, int line_number)
{
    size_t len = strlen(line);
    if (len != 0 && line[len - 1] == '\n')
        line[len - 1] = '\0';
    if (line[0] == '\0')
        return 0;
    /* `X is a power of 2` and `X is not a power of 2`: a positive value with a one-bit
       significand. */
    const char * pow2_suffixes[2] = {" is a power of 2", " is not a power of 2"};
    for (int k = 0; k < 2; k++)
    {
        size_t ll = strlen(line), sl = strlen(pow2_suffixes[k]);
        if (ll > sl && strcmp(line + ll - sl, pow2_suffixes[k]) == 0)
        {
            line[ll - sl] = '\0';
            arf_t v;
            arf_init(v);
            int res = 0;
            if (arf_parse_base(line, 4, v))
                res = report(line_number, "unreadable value");
            else
            {
                int is = arf_is_finite(v) && arf_sgn(v) > 0 && arf_bits(v) == 1;
                if (is != (k == 0))
                    res = report(line_number, "wrong power-of-2 test");
            }
            arf_clear(v);
            if (res == 0)
                checked++;
            return res;
        }
    }
    char * result = split(line, " = ");
    /* `BigExponentFloat::power_of_2(k) = X`: X must be 2^k, exactly. */
    const char * pow2_prefix = "BigExponentFloat::power_of_2(";
    if (result != NULL && strncmp(line, pow2_prefix, strlen(pow2_prefix)) == 0)
    {
        char * k = line + strlen(pow2_prefix);
        size_t kl = strlen(k);
        int res = 0;
        fmpz_t e;
        arf_t expected, v;
        fmpz_init(e);
        arf_init(expected);
        arf_init(v);
        if (kl == 0 || k[kl - 1] != ')')
            res = report(line_number, "unrecognized line");
        else
        {
            k[kl - 1] = '\0';
            if (fmpz_set_str(e, k, 10) != 0)
                res = report(line_number, "unreadable exponent");
            else if (arf_parse_base(result, 4, v))
                res = report(line_number, "unreadable value");
            else
            {
                arf_one(expected);
                arf_mul_2exp_fmpz(expected, expected, e);
                if (!arf_equal(expected, v))
                    res = report(line_number, "wrong value");
            }
        }
        fmpz_clear(e);
        arf_clear(expected);
        arf_clear(v);
        if (res == 0)
            checked++;
        return res;
    }
    if (result == NULL)
        return report(line_number, "unrecognized line");
    leaf_t out[MAX_LEAVES];
    for (int i = 0; i < MAX_LEAVES; i++)
        arf_init(out[i].value);
    arf_t x;
    arf_ptr ys = _arf_vec_init(1);
    arf_init(x);
    int n = -1, res = 0;
    char * rhs = strstr(line, " % ");
    if (rhs != NULL)
    {
        *rhs = '\0';
        rhs += 3;
        if (arf_parse_base(line, 4, x) || arf_parse_base(rhs, 4, ys))
            res = report(line_number, "unreadable value");
        else
            n = compute(out, "rem", x, ys, 1, NULL, 0, NULL, 0, 0, 0);
    }
    else
    {
        char * dot = find_method(line);
        char * args = dot == NULL ? NULL : split(dot + 1, "(");
        size_t al = args == NULL ? 0 : strlen(args);
        if (dot == NULL || args == NULL || al == 0 || args[al - 1] != ')')
            res = report(line_number, "unrecognized line");
        else
        {
            *dot = '\0';
            args[al - 1] = '\0';
            char * name = dot + 1;
            int with_prec = 0, with_rm = 0;
            char * suffix = strstr(name, "_prec");
            if (suffix != NULL)
            {
                if (strcmp(suffix, "_prec_round") == 0)
                    with_prec = with_rm = 1;
                else if (strcmp(suffix, "_prec") == 0)
                    with_prec = 1;
                *suffix = '\0';
            }
            slong ks[MAX_ARGS];
            mpfr_rnd_t rms[MAX_ARGS];
            int n_ys = 0, n_ks = 0, n_rms = 0, exact_rm = 0;
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
            /* `rem_unsigned`'s first argument is a `u64` modulus, which may exceed an `slong`: it
               is read exactly and the line is checked as the remainder by that value. */
            const char * method = name;
            int first = 0;
            if (strcmp(name, "rem_unsigned") == 0)
            {
                fmpz_t m;
                fmpz_init(m);
                if (n_items >= 1 && fmpz_set_str(m, items[0], 10) == 0 && fmpz_sgn(m) >= 0)
                {
                    arf_set_fmpz(ys, m);
                    n_ys = 1;
                    first = 1;
                    method = "rem";
                }
                else
                    res = report(line_number, "unreadable modulus");
                fmpz_clear(m);
            }
            for (int i = first; i < n_items && res == 0; i++)
            {
                mpfr_rnd_t r;
                int exact;
                char * end = NULL;
                long long k = strtoll(items[i], &end, 10);
                if (parse_rm(items[i], &r, &exact) == 0)
                {
                    rms[n_rms++] = r;
                    exact_rm |= exact;
                }
                else if (*items[i] != '\0' && *end == '\0' && strncmp(items[i], "0x", 2) != 0)
                    ks[n_ks++] = k;
                else if (n_ys == 0 && arf_parse_base(items[i], 4, ys) == 0)
                    n_ys = 1;
                else
                    res = report(line_number, "unreadable arguments");
            }
            if (res == 0 && arf_parse_base(line, 4, x))
                res = report(line_number, "unreadable value");
            /* With a precision, the precisions come right after the values. */
            if (res == 0 && with_prec && n_ks == 0)
                res = report(line_number, "missing precision");
            if (res == 0)
            {
                n = compute(out, method, x, ys, n_ys, ks, n_ks, rms, n_rms, with_prec, with_rm);
                if (n >= 0 && exact_rm)
                {
                    /* With `Exact`, every ordering must be `Equal`. */
                    for (int i = 0; i < n; i++)
                        if (out[i].kind == 1 && out[i].ordering != 0)
                            res = report(line_number, "Exact rounding of an inexact result");
                }
            }
        }
    }
    if (res == 0)
    {
        char * got[MAX_LEAVES];
        int m = leaves(result, got, MAX_LEAVES);
        if (n < 0)
            res = report(line_number, "unrecognized method");
        else if (m != n)
            res = report(line_number, "wrong number of results");
        else
            for (int i = 0; i < n && res == 0; i++)
                if (!leaf_matches(&out[i], got[i]))
                    res = report(line_number, out[i].kind == 1 ? "wrong ordering" : "wrong value");
    }
    for (int i = 0; i < MAX_LEAVES; i++)
        arf_clear(out[i].value);
    arf_clear(x);
    _arf_vec_clear(ys, 1);
    if (res == 0)
        checked++;
    return res;
}

int
run_arf_misc(const char * arg)
{
    checked = 0;
    mpfr_set_emin(mpfr_get_emin_min());
    mpfr_set_emax(mpfr_get_emax_max());
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines("arf_misc", checked);
}
