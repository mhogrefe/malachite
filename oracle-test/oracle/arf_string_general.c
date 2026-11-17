/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs BigExponentFloat's string conversions in bases that are not powers of 2 against exact
    FLINT arithmetic and arb ball arithmetic. The mode `arf_string_general` accepts lines of the
    forms

    - `(X).to_string() = S`, the round-trip digit count in base 10, rounded to nearest;
    - `(X).to_string_base(B) = S` and `(X).to_string_base_upper(B) = S`, the same in base B;
    - `(X).to_sci_with_options(B, SIZE, RM) = S`, with SIZE one of `precision N`, `scale N`, and
      `complete`;
    - `from_sci_string_with_options_prec("S", B, P, RM) = (Y, O)`, reading S in base B and rounding
      it to P bits;

    where X and Y are in Malachite's exact format (`m*2^k`, or the hexadecimal `{:#x}` form for
    zero, NaN, and the infinities).

    For the digit lines, S is read as D * B^F, with trailing zero digits folded into F. The
    position F' of the last digit the rounding keeps follows from the size: with precision N it is
    the leading digit's position minus N - 1, with scale N it is -N; for the round-trip digit count,
    N = 1 + ceil(p log_B(2)), where p is the number of significant bits of X. S must then be the
    rounding of |X| B^(-F') to an integer, in the rounding mode, times B^F', with the sign of X. A
    nearest-rounding tie goes to the even neighbor in a power-of-2 base, and may go to either in any
    other: MPFR's get_str, which Float ports and BigExponentFloat follows within Float's range, does
    not round those ties to even (it writes 1.5 in base 3 with three digits as 1.11, and in base 7
    with two as 1.4). That rounding is computed exactly with fmpq arithmetic when the
    numbers involved are of moderate size, which decides every tie and every exact case; otherwise
    with arb balls whose precision doubles until the floors of both ends agree. (A tie or an exact
    case there would need the numbers to be no larger than the output, which they are not.) With
    `complete`, S must equal X exactly.

    For the parsing lines, the value of S is rounded with arf_set_round: exactly, by
    arf_fmpz_div_fmpz, when moderate, and otherwise by rounding both ends of an arb ball, whose
    precision doubles until they agree. Y must be that rounding, and O its comparison with the value.

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

static long checked;

/* The largest number of bits an exact computation may involve. */
#define EXACT_BITS 1000000
/* The largest working precision of a ball computation. */
#define MAX_WP (1L << 24)

static int
fail(int line_number, const char * what)
{
    flint_printf("error in arf_string_general test, line %d: %s\n", line_number, what);
    return 1;
}

/* The value of a digit character, case-insensitively, or -1. */
static int
digit_value(char c)
{
    if (c >= '0' && c <= '9')
        return c - '0';
    if (c >= 'a' && c <= 'z')
        return c - 'a' + 10;
    if (c >= 'A' && c <= 'Z')
        return c - 'A' + 10;
    return -1;
}

/* Reads a number string in base `base` as (-1)^neg * d * base^f, with d's trailing zero digits
   folded into f. Returns nonzero if it is not one. */
static int
parse_number(const char * s, ulong base, int * neg, fmpz_t d, fmpz_t f)
{
    *neg = 0;
    if (*s == '-')
    {
        *neg = 1;
        s++;
    }
    size_t n = strlen(s);
    /* the exponent marker: the last e or E, followed by a sign in bases 15 and up */
    size_t mark = n;
    for (size_t i = n; i-- > 0;)
    {
        if (s[i] == 'e' || s[i] == 'E')
        {
            if (base < 15 || (i + 1 < n && (s[i + 1] == '+' || s[i + 1] == '-')))
            {
                mark = i;
                break;
            }
        }
    }
    fmpz_zero(d);
    fmpz_zero(f);
    if (mark < n)
    {
        const char * e = s + mark + 1;
        if (*e == '+')
            e++;
        if (fmpz_set_str(f, e, 10))
            return 1;
    }
    slong frac = 0;
    int seen_point = 0, seen_digit = 0;
    for (size_t i = 0; i < mark; i++)
    {
        if (s[i] == '.')
        {
            if (seen_point)
                return 1;
            seen_point = 1;
            continue;
        }
        int v = digit_value(s[i]);
        if (v < 0 || (ulong) v >= base)
            return 1;
        fmpz_mul_ui(d, d, base);
        fmpz_add_ui(d, d, v);
        seen_digit = 1;
        if (seen_point)
            frac++;
    }
    if (!seen_digit)
        return 1;
    fmpz_sub_si(f, f, frac);
    if (!fmpz_is_zero(d))
    {
        while (fmpz_divisible_si(d, base))
        {
            fmpz_divexact_ui(d, d, base);
            fmpz_add_ui(f, f, 1);
        }
    }
    else
        fmpz_zero(f);
    return 0;
}

/* The number of base-`base` digits of the positive d. */
static slong
digit_count(const fmpz_t d, ulong base)
{
    return fmpz_sizeinbase(d, base);
}

/* Malachite's rounding mode names. */
typedef enum { RM_FLOOR, RM_CEILING, RM_DOWN, RM_UP, RM_NEAREST, RM_EXACT, RM_INVALID } rm_t;

static rm_t
parse_rm(const char * s)
{
    if (strcmp(s, "Floor") == 0)
        return RM_FLOOR;
    if (strcmp(s, "Ceiling") == 0)
        return RM_CEILING;
    if (strcmp(s, "Down") == 0)
        return RM_DOWN;
    if (strcmp(s, "Up") == 0)
        return RM_UP;
    if (strcmp(s, "Nearest") == 0)
        return RM_NEAREST;
    if (strcmp(s, "Exact") == 0)
        return RM_EXACT;
    return RM_INVALID;
}

/* The rounding of the magnitude of a value with the given sign. */
static rm_t
magnitude_rm(rm_t rm, int neg)
{
    if (rm == RM_FLOOR)
        return neg ? RM_UP : RM_DOWN;
    if (rm == RM_CEILING)
        return neg ? RM_DOWN : RM_UP;
    return rm;
}

/* An upper bound on the bits of |x| b^e as a fraction, for the finite nonzero x. */
static int
exact_is_cheap(const arf_t x, const fmpz_t e, ulong base)
{
    fmpz_t bits;
    fmpz_init(bits);
    fmpz_abs(bits, ARF_EXPREF(x));
    fmpz_add_ui(bits, bits, arf_bits(x));
    fmpz_t t;
    fmpz_init(t);
    fmpz_abs(t, e);
    fmpz_mul_ui(t, t, FLINT_BIT_COUNT(base));
    fmpz_add(bits, bits, t);
    int cheap = fmpz_cmp_ui(bits, EXACT_BITS) <= 0;
    fmpz_clear(bits);
    fmpz_clear(t);
    return cheap;
}

/* q = |x| base^e as an fmpq, exactly. */
static void
abs_times_power(fmpq_t q, const arf_t x, const fmpz_t e, ulong base)
{
    fmpz_t m, k, p, ea;
    fmpz_init(m);
    fmpz_init(k);
    fmpz_init(p);
    fmpz_init(ea);
    arf_get_fmpz_2exp(m, k, x);
    fmpz_abs(m, m);
    fmpq_set_fmpz(q, m);
    if (fmpz_sgn(k) >= 0)
        fmpq_mul_2exp(q, q, fmpz_get_ui(k));
    else
    {
        fmpz_neg(k, k);
        fmpq_div_2exp(q, q, fmpz_get_ui(k));
    }
    fmpz_set_ui(p, base);
    fmpz_abs(ea, e);
    fmpz_pow_fmpz(p, p, ea);
    if (fmpz_sgn(e) >= 0)
        fmpq_mul_fmpz(q, q, p);
    else
        fmpq_div_fmpz(q, q, p);
    fmpz_clear(m);
    fmpz_clear(k);
    fmpz_clear(p);
    fmpz_clear(ea);
}

/* The rounding r of |x| base^e to an integer with the magnitude rounding mode `mrm` (not exact).
   Returns nonzero if it cannot be decided. */
static int
round_scaled(fmpz_t r, const arf_t x, const fmpz_t e, ulong base, rm_t mrm, slong digits_hint,
             int * tie)
{
    *tie = 0;
    if ((base & (base - 1)) == 0)
    {
        /* a power-of-2 base: |x| base^e = |x| 2^(a e) is an arf, exactly */
        arf_t y, t;
        arf_init(y);
        arf_init(t);
        fmpz_t s;
        fmpz_init(s);
        fmpz_mul_ui(s, e, FLINT_BIT_COUNT(base) - 1);
        arf_abs(y, x);
        arf_mul_2exp_fmpz(y, y, s);
        int bad = 0;
        if (mrm == RM_EXACT)
        {
            if (arf_is_int(y))
                arf_get_fmpz(r, y, ARF_RND_DOWN);
            else
                bad = 1;
        }
        else if (mrm == RM_DOWN)
            arf_get_fmpz(r, y, ARF_RND_FLOOR);
        else if (mrm == RM_UP)
            arf_get_fmpz(r, y, ARF_RND_CEIL);
        else
        {
            arf_mul_2exp_si(t, y, 1);
            if (arf_is_int(t) && !arf_is_int(y))
            {
                /* an exact tie: r is the upper neighbor */
                *tie = 1;
                arf_get_fmpz(r, y, ARF_RND_CEIL);
            }
            else
                arf_get_fmpz(r, y, ARF_RND_NEAR);
        }
        arf_clear(y);
        arf_clear(t);
        fmpz_clear(s);
        return bad;
    }
    if (exact_is_cheap(x, e, base))
    {
        fmpq_t q;
        fmpq_init(q);
        abs_times_power(q, x, e, base);
        const fmpz * num = fmpq_numref(q);
        const fmpz * den = fmpq_denref(q);
        if (mrm == RM_DOWN)
            fmpz_fdiv_q(r, num, den);
        else if (mrm == RM_UP)
            fmpz_cdiv_q(r, num, den);
        else if (mrm == RM_EXACT)
        {
            if (!fmpz_is_one(den))
            {
                fmpq_clear(q);
                return 1;
            }
            fmpz_set(r, num);
        }
        else
        {
            /* nearest, ties to even: floor(q + 1/2), less one on an exact tie with an odd result */
            fmpz_t n2, d2, rem;
            fmpz_init(n2);
            fmpz_init(d2);
            fmpz_init(rem);
            fmpz_mul_2exp(n2, num, 1);
            fmpz_add(n2, n2, den);
            fmpz_mul_2exp(d2, den, 1);
            fmpz_fdiv_qr(r, rem, n2, d2);
            /* on an exact tie, r is the upper neighbor; the caller picks */
            if (fmpz_is_zero(rem))
                *tie = 1;
            fmpz_clear(n2);
            fmpz_clear(d2);
            fmpz_clear(rem);
        }
        fmpq_clear(q);
        return 0;
    }
    if (mrm == RM_EXACT)
        return 1;
    arb_t t, b;
    arb_init(t);
    arb_init(b);
    arf_t lo, hi;
    arf_init(lo);
    arf_init(hi);
    fmpz_t rl, rh;
    fmpz_init(rl);
    fmpz_init(rh);
    int undecided = 1;
    for (slong wp = digits_hint * 6 + 128; wp <= MAX_WP; wp *= 2)
    {
        arb_set_ui(b, base);
        arb_pow_fmpz(t, b, e, wp);
        arb_mul_arf(t, t, x, wp);
        arb_abs(t, t);
        if (mrm == RM_NEAREST)
        {
            arb_t h;
            arb_init(h);
            arb_set_d(h, 0.5);
            arb_add(t, t, h, wp);
            arb_clear(h);
        }
        arb_get_lbound_arf(lo, t, wp);
        arb_get_ubound_arf(hi, t, wp);
        arf_rnd_t rnd = mrm == RM_UP ? ARF_RND_CEIL : ARF_RND_FLOOR;
        arf_get_fmpz(rl, lo, rnd);
        arf_get_fmpz(rh, hi, rnd);
        if (fmpz_equal(rl, rh))
        {
            fmpz_set(r, rl);
            undecided = 0;
            break;
        }
    }
    arb_clear(t);
    arb_clear(b);
    arf_clear(lo);
    arf_clear(hi);
    fmpz_clear(rl);
    fmpz_clear(rh);
    return undecided;
}

/* 1 + ceil(p log_base(2)), the round-trip digit count, for a base that is not a power of 2. */
static slong
round_trip_digits(slong p, ulong base)
{
    arb_t a, b;
    arb_init(a);
    arb_init(b);
    arf_t lo, hi;
    arf_init(lo);
    arf_init(hi);
    fmpz_t cl, ch;
    fmpz_init(cl);
    fmpz_init(ch);
    slong result = -1;
    for (slong wp = 128; wp <= 4096; wp *= 2)
    {
        arb_const_log2(a, wp);
        arb_mul_si(a, a, p, wp);
        arb_log_ui(b, base, wp);
        arb_div(a, a, b, wp);
        arb_get_lbound_arf(lo, a, wp);
        arb_get_ubound_arf(hi, a, wp);
        arf_get_fmpz(cl, lo, ARF_RND_CEIL);
        arf_get_fmpz(ch, hi, ARF_RND_CEIL);
        if (fmpz_equal(cl, ch))
        {
            result = 1 + fmpz_get_si(cl);
            break;
        }
    }
    arb_clear(a);
    arb_clear(b);
    arf_clear(lo);
    arf_clear(hi);
    fmpz_clear(cl);
    fmpz_clear(ch);
    return result;
}

/* The number of significant bits of the finite nonzero x. */
static slong
significant_bits(const arf_t x)
{
    return arf_bits(x);
}

/* Checks a digit line: the output s of x in base `base`, with the size given by `mode` (0:
   precision n, 1: scale n, 2: complete) and rounding mode `rm`. */
static int
check_digits(int line_number, const arf_t x, const char * s, ulong base, int mode, slong n, rm_t rm)
{
    if (arf_is_nan(x))
        return strcmp(s, "NaN") == 0 ? 0 : fail(line_number, "NaN expected");
    if (arf_is_pos_inf(x))
        return strcmp(s, "Infinity") == 0 ? 0 : fail(line_number, "Infinity expected");
    if (arf_is_neg_inf(x))
        return strcmp(s, "-Infinity") == 0 ? 0 : fail(line_number, "-Infinity expected");
    int neg;
    fmpz_t d, f, fp, r, t;
    fmpz_init(d);
    fmpz_init(f);
    fmpz_init(fp);
    fmpz_init(r);
    fmpz_init(t);
    int result = 0;
    if (parse_number(s, base, &neg, d, f))
    {
        result = fail(line_number, "unparseable output");
        goto cleanup;
    }
    if (arf_is_zero(x))
    {
        if (!fmpz_is_zero(d) || neg)
            result = fail(line_number, "zero expected");
        goto cleanup;
    }
    if (neg != arf_sgn(x) < 0)
    {
        result = fail(line_number, "wrong sign");
        goto cleanup;
    }
    if (mode == 2)
    {
        /* complete: s is x exactly */
        if (fmpz_is_zero(d) || ((base & (base - 1)) != 0 && !exact_is_cheap(x, f, base)))
        {
            result = fail(line_number, "complete output too large to check");
            goto cleanup;
        }
        fmpz_neg(t, f);
        int tie;
        if (round_scaled(r, x, t, base, RM_EXACT, 0, &tie) || !fmpz_equal(r, d))
            result = fail(line_number, "wrong value");
        goto cleanup;
    }
    if (mode == 0)
    {
        if (fmpz_is_zero(d))
        {
            result = fail(line_number, "nonzero expected");
            goto cleanup;
        }
        /* the leading digit is at f + digits - 1, and the last kept one n - 1 below it */
        fmpz_add_si(fp, f, digit_count(d, base) - n);
    }
    else
        fmpz_set_si(fp, -n);
    /* the printed digits must stop at or before position fp */
    if (!fmpz_is_zero(d) && fmpz_cmp(f, fp) < 0)
    {
        result = fail(line_number, "too many digits");
        goto cleanup;
    }
    /* the printed value, as an integer multiple of base^fp */
    if (!fmpz_is_zero(d))
    {
        fmpz_sub(t, f, fp);
        fmpz_t p;
        fmpz_init_set_ui(p, base);
        fmpz_pow_fmpz(p, p, t);
        fmpz_mul(d, d, p);
        fmpz_clear(p);
    }
    fmpz_neg(t, fp);
    rm_t mrm = magnitude_rm(rm, neg);
    int tie;
    if (round_scaled(r, x, t, base, mrm, n, &tie))
    {
        result = fail(line_number, mrm == RM_EXACT ? "inexact" : "undecided");
        goto cleanup;
    }
    /* With a precision, the printed value has exactly n digits, which pins down its exponent.
       (When the rounding carries to base^n, the output is written as base^(n - 1) one place up,
       and rounding at that coarser scale gives base^(n - 1) too.) */
    if (mode == 0)
    {
        fmpz_t lo, hi;
        fmpz_init_set_ui(lo, base);
        fmpz_init_set_ui(hi, base);
        fmpz_pow_ui(lo, lo, n - 1);
        fmpz_pow_ui(hi, hi, n);
        int in_range = fmpz_cmp(d, lo) >= 0 && fmpz_cmp(d, hi) < 0;
        fmpz_clear(lo);
        fmpz_clear(hi);
        if (!in_range)
        {
            result = fail(line_number, "wrong exponent");
            goto cleanup;
        }
    }
    /* On an exact tie, r is the upper neighbor. In a power-of-2 base the tie goes to the even
       neighbor; in any other base it may go either way, since MPFR's get_str, which Float ports
       and BigExponentFloat follows within Float's range, does not round such ties to even. */
    if (tie)
    {
        fmpz_t lower;
        fmpz_init(lower);
        fmpz_sub_ui(lower, r, 1);
        int ok;
        if ((base & (base - 1)) != 0)
            ok = fmpz_equal(d, r) || fmpz_equal(d, lower);
        else
            ok = fmpz_equal(d, fmpz_is_even(r) ? r : lower);
        fmpz_clear(lower);
        if (!ok)
            result = fail(line_number, "wrong value at a tie");
    }
    else if (!fmpz_equal(r, d))
        result = fail(line_number, "wrong value");
cleanup:
    fmpz_clear(d);
    fmpz_clear(f);
    fmpz_clear(fp);
    fmpz_clear(r);
    fmpz_clear(t);
    return result;
}

/* Checks a parsing line: `s` read in `base` and rounded to `prec` bits with `rm` gives y, with the
   ordering named `o`. */
static int
check_parse(int line_number, const char * s, ulong base, slong prec, rm_t rm, const arf_t y,
            const char * o)
{
    int neg;
    fmpz_t d, f;
    fmpz_init(d);
    fmpz_init(f);
    int result = 0;
    arf_t z;
    arf_init(z);
    int ord = 2; /* -1 Less, 0 Equal, 1 Greater */
    if (strcmp(s, "NaN") == 0 || strcmp(s, "Infinity") == 0 || strcmp(s, "-Infinity") == 0)
    {
        arf_t w;
        arf_init(w);
        if (strcmp(s, "NaN") == 0)
            arf_nan(w);
        else if (s[0] == '-')
            arf_neg_inf(w);
        else
            arf_pos_inf(w);
        result = arf_equal(w, y) && strcmp(o, "Equal") == 0 ? 0 : fail(line_number, "wrong special");
        arf_clear(w);
        goto cleanup;
    }
    if (parse_number(s, base, &neg, d, f))
    {
        result = fail(line_number, "unparseable input");
        goto cleanup;
    }
    if (fmpz_is_zero(d))
    {
        result = arf_is_zero(y) && strcmp(o, "Equal") == 0 ? 0 : fail(line_number, "zero expected");
        goto cleanup;
    }
    if (neg)
        fmpz_neg(d, d);
    arf_rnd_t rnd = rm == RM_FLOOR ? ARF_RND_FLOOR : rm == RM_CEILING ? ARF_RND_CEIL
                  : rm == RM_DOWN ? ARF_RND_DOWN : rm == RM_UP ? ARF_RND_UP : ARF_RND_NEAR;
    /* moderate: exact */
    fmpz_t fa;
    fmpz_init(fa);
    fmpz_abs(fa, f);
    int cheap = fmpz_cmp_ui(fa, EXACT_BITS / 8) <= 0;
    fmpz_clear(fa);
    if ((base & (base - 1)) == 0)
    {
        /* a power-of-2 base: the value d 2^(a f) is an arf, exactly */
        arf_t v;
        arf_init(v);
        fmpz_t s2;
        fmpz_init(s2);
        fmpz_mul_ui(s2, f, FLINT_BIT_COUNT(base) - 1);
        arf_set_fmpz(v, d);
        arf_mul_2exp_fmpz(v, v, s2);
        int inexact = arf_set_round(z, v, prec, rnd);
        if (rm == RM_EXACT && inexact)
            result = fail(line_number, "Exact, but inexact");
        ord = arf_cmp(z, v);
        ord = ord < 0 ? -1 : ord > 0 ? 1 : 0;
        arf_clear(v);
        fmpz_clear(s2);
    }
    else if (cheap)
    {
        fmpz_t num, den;
        fmpz_init(num);
        fmpz_init(den);
        fmpz_set(num, d);
        fmpz_one(den);
        fmpz_t p;
        fmpz_init_set_ui(p, base);
        if (fmpz_sgn(f) >= 0)
        {
            fmpz_pow_ui(p, p, fmpz_get_ui(f));
            fmpz_mul(num, num, p);
        }
        else
        {
            fmpz_pow_ui(p, p, -fmpz_get_si(f));
            fmpz_set(den, p);
        }
        int inexact = arf_fmpz_div_fmpz(z, num, den, prec, rnd);
        if (rm == RM_EXACT && inexact)
        {
            result = fail(line_number, "Exact, but inexact");
        }
        else if (!inexact)
            ord = 0;
        else
        {
            fmpq_t zq, v;
            fmpq_init(zq);
            fmpq_init(v);
            arf_get_fmpq(zq, z);
            fmpq_set_fmpz_frac(v, num, den);
            ord = fmpq_cmp(zq, v) < 0 ? -1 : 1;
            fmpq_clear(zq);
            fmpq_clear(v);
        }
        fmpz_clear(num);
        fmpz_clear(den);
        fmpz_clear(p);
    }
    else
    {
        if (rm == RM_EXACT)
        {
            result = fail(line_number, "Exact on a huge input");
            goto cleanup;
        }
        arb_t v, b;
        arb_init(v);
        arb_init(b);
        arf_t lo, hi, z2;
        arf_init(lo);
        arf_init(hi);
        arf_init(z2);
        int decided = 0;
        for (slong wp = prec + fmpz_bits(d) + 128; wp <= MAX_WP; wp *= 2)
        {
            arb_set_ui(b, base);
            arb_pow_fmpz(v, b, f, wp);
            arb_mul_fmpz(v, v, d, wp);
            arb_get_lbound_arf(lo, v, wp);
            arb_get_ubound_arf(hi, v, wp);
            arf_set_round(z, lo, prec, rnd);
            arf_set_round(z2, hi, prec, rnd);
            if (arf_equal(z, z2))
            {
                if (arf_cmp(z, hi) > 0)
                    ord = 1;
                else if (arf_cmp(z, lo) < 0)
                    ord = -1;
                else
                    continue;
                decided = 1;
                break;
            }
        }
        arb_clear(v);
        arb_clear(b);
        arf_clear(lo);
        arf_clear(hi);
        arf_clear(z2);
        if (!decided)
        {
            result = fail(line_number, "undecided");
            goto cleanup;
        }
    }
    if (result == 0)
    {
        const char * want = ord < 0 ? "Less" : ord > 0 ? "Greater" : "Equal";
        if (!arf_equal(z, y))
            result = fail(line_number, "wrong value");
        else if (strcmp(o, want) != 0)
            result = fail(line_number, "wrong ordering");
    }
cleanup:
    fmpz_clear(d);
    fmpz_clear(f);
    arf_clear(z);
    return result;
}

/* Splits `line` at the first occurrence of `sep`, in place, returning the text after it, or NULL. */
static char *
split(char * line, const char * sep)
{
    char * p = strstr(line, sep);
    if (p == NULL)
        return NULL;
    *p = '\0';
    return p + strlen(sep);
}

static int
check_line(char * line, int line_number)
{
    line[strcspn(line, "\r\n")] = '\0';
    if (line[0] == '\0')
        return 0;
    checked++;
    char * rest;
    if (strncmp(line, "from_sci_string_with_options_prec(\"", 35) == 0)
    {
        /* from_sci_string_with_options_prec("S", B, P, RM) = (Y, O) */
        char * s = line + 35;
        rest = split(s, "\", ");
        if (rest == NULL)
            return fail(line_number, "malformed line");
        char * b = rest;
        char * p = split(b, ", ");
        char * r = p == NULL ? NULL : split(p, ", ");
        char * out = r == NULL ? NULL : split(r, ") = (");
        char * ord = out == NULL ? NULL : split(out, ", ");
        if (ord == NULL || ord[strlen(ord) - 1] != ')')
            return fail(line_number, "malformed line");
        ord[strlen(ord) - 1] = '\0';
        rm_t rm = parse_rm(r);
        arf_t y;
        arf_init(y);
        int result;
        if (rm == RM_INVALID || arf_parse_exact(out, y))
            result = fail(line_number, "malformed line");
        else
            result = check_parse(line_number, s, strtoul(b, NULL, 10), strtol(p, NULL, 10), rm, y, ord);
        arf_clear(y);
        return result;
    }
    char * x_str;
    rest = arf_split_receiver(line, &x_str);
    if (rest == NULL)
        return fail(line_number, "malformed line");
    char * out = split(rest, " = ");
    if (out == NULL)
        return fail(line_number, "malformed line");
    arf_t x;
    arf_init(x);
    int result = 0;
    if (arf_parse_exact(x_str, x))
    {
        arf_clear(x);
        return fail(line_number, "unparseable input");
    }
    if (strcmp(rest, "to_string()") == 0)
    {
        slong n = arf_is_finite(x) && !arf_is_zero(x) ? round_trip_digits(significant_bits(x), 10) : 1;
        result = check_digits(line_number, x, out, 10, 0, n, RM_NEAREST);
    }
    else if (strncmp(rest, "to_string_base(", 15) == 0 || strncmp(rest, "to_string_base_upper(", 21) == 0)
    {
        const char * b = strchr(rest, '(') + 1;
        ulong base = strtoul(b, NULL, 10);
        slong n = arf_is_finite(x) && !arf_is_zero(x) ? round_trip_digits(significant_bits(x), base) : 1;
        result = check_digits(line_number, x, out, base, 0, n, RM_NEAREST);
    }
    else if (strncmp(rest, "to_sci_with_options(", 20) == 0)
    {
        /* to_sci_with_options(B, SIZE, RM) */
        char * b = rest + 20;
        char * size = split(b, ", ");
        char * r = size == NULL ? NULL : split(size, ", ");
        if (r == NULL || r[strlen(r) - 1] != ')')
        {
            arf_clear(x);
            return fail(line_number, "malformed line");
        }
        r[strlen(r) - 1] = '\0';
        rm_t rm = parse_rm(r);
        int mode;
        slong n = 0;
        if (strncmp(size, "precision ", 10) == 0)
        {
            mode = 0;
            n = strtol(size + 10, NULL, 10);
        }
        else if (strncmp(size, "scale ", 6) == 0)
        {
            mode = 1;
            n = strtol(size + 6, NULL, 10);
        }
        else if (strcmp(size, "complete") == 0)
            mode = 2;
        else
            mode = -1;
        if (mode < 0 || rm == RM_INVALID)
            result = fail(line_number, "malformed line");
        else
            result = check_digits(line_number, x, out, strtoul(b, NULL, 10), mode, n, rm);
    }
    else
        result = fail(line_number, "malformed line");
    arf_clear(x);
    return result;
}

int
run_arf_string_general(const char * arg)
{
    checked = 0;
    int result = arf_for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines("arf_string_general", checked);
}
