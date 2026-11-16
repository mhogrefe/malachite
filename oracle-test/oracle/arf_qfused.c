/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs BigExponentFloat's fused operations with a Rational term against FLINT's exact arithmetic.
    The mode `arf_qfused` accepts lines of the forms `X.f_prec_round(Y, Q, p, rm) = (Z, O)` for f
    among add_mul_rational and sub_mul_rational (X + YQ and X - YQ), and
    `X.f_prec_round(Y, W, Q, p, rm) = (Z, O)` for f among mul_add_mul_rational and
    mul_sub_mul_rational (XY + WQ and XY - WQ), and their `_prec` forms. X, Y, W, and Z are
    BigExponentFloats in Malachite's exact hexadecimal format, and Q is a Rational written as `a/b`
    or `a`.

    For the four-argument forms, the product XY is formed exactly with arf_mul, and the line is
    checked as the three-argument form on XY. NaN gives NaN, an infinity times zero gives NaN, an
    infinity times a nonzero value is an infinity with the product's sign, and infinities of
    opposite signs (after the sign of the subtraction) sum to NaN.

    For finite values, with A = X (or XY) and T = YQ (or WQ, negated for a subtraction): a zero T
    gives A rounded; a dyadic Q makes T an exact arf, and A + T is rounded by arf_add, which rounds
    correctly whatever the gap between the exponents. Otherwise T is not a dyadic rational. When A
    is zero, or the exponents of A and T are within 2^22 of each other, A + T is formed exactly as
    an fmpq scaled by a power of 2 and rounded by arf_set_fmpq. When they are farther apart, the
    smaller term is replaced by a stand-in that rounds the same way (as for BigExponentFloat's
    add_rational): a T far below A by sign(T) 2^(g - 1), where g = min(the position of A's lowest
    bit, EXP(A) - p - 2) and |T| < 2^g; and an A far below T by sign(A) 2^(h - 1), where, writing
    T = (n/d) 2^b with n/d in lowest terms (d not a power of 2) and j = floor(log2(|n/d|)) - p - 2,
    h = min(j, 0) - ceil(log2(d)) and |A 2^(-b)| < 2^h: then n/d is at least 2^h from every
    multiple of 2^j, so A and the stand-in leave the rounding unchanged. In every case the ordering
    comes from the results of rounding down and up.

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

#define MAX_ARGS 6
#define FAR_GAP (1L << 22)

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

/* The ordering of the result `z` of rounding with `rnd`, given the results of rounding down and
   up. */
static int
ordering_of(const arf_t z, const arf_t lo, const arf_t hi)
{
    if (arf_equal(lo, hi))
        return 0;
    return arf_equal(z, lo) ? -1 : 1;
}

/* a + b, both exact, rounded; returns the ordering. */
static int
round_sum(arf_t z, const arf_t a, const arf_t b, slong prec, arf_rnd_t rnd)
{
    arf_t lo, hi;
    arf_init(lo);
    arf_init(hi);
    arf_add(lo, a, b, prec, ARF_RND_FLOOR);
    arf_add(hi, a, b, prec, ARF_RND_CEIL);
    arf_add(z, a, b, prec, rnd);
    int o = ordering_of(z, lo, hi);
    arf_clear(lo);
    arf_clear(hi);
    return o;
}

/* v 2^shift rounded; returns the ordering. */
static int
round_fmpq_2exp(arf_t z, const fmpq_t v, const fmpz_t shift, slong prec, arf_rnd_t rnd)
{
    arf_t lo, hi;
    arf_init(lo);
    arf_init(hi);
    arf_set_fmpq(lo, v, prec, ARF_RND_FLOOR);
    arf_set_fmpq(hi, v, prec, ARF_RND_CEIL);
    arf_set_fmpq(z, v, prec, rnd);
    int o = ordering_of(z, lo, hi);
    arf_mul_2exp_fmpz(z, z, shift);
    arf_clear(lo);
    arf_clear(hi);
    return o;
}

/* The position of the lowest bit of the finite nonzero x: x = m 2^e with m odd. */
static void
lowest_bit(fmpz_t e, const arf_t x)
{
    fmpz_t m;
    fmpz_init(m);
    arf_get_fmpz_2exp(m, e, x);
    fmpz_clear(m);
}

/* a + y q for finite a and y, rounded; returns the ordering. */
static int
fused_finite(arf_t z, const arf_t a, const arf_t y, const fmpq_t q, slong prec, arf_rnd_t rnd)
{
    if (arf_is_zero(y) || fmpq_is_zero(q))
    {
        arf_set_round(z, a, prec, rnd);
        return arf_cmp(z, a);
    }
    int o;
    const fmpz * n = fmpq_numref(q);
    const fmpz * d = fmpq_denref(q);
    if ((slong) fmpz_val2(d) + 1 == (slong) fmpz_bits(d))
    {
        /* T = y n / d is an exact arf */
        arf_t t;
        arf_init(t);
        arf_set_fmpz(t, n);
        arf_mul(t, t, y, ARF_PREC_EXACT, ARF_RND_DOWN);
        arf_mul_2exp_si(t, t, -(slong) fmpz_val2(d));
        o = round_sum(z, a, t, prec, rnd);
        arf_clear(t);
        return o;
    }
    /* y = Y 2^b, with Y an odd integer */
    fmpz_t ym, b, shift, et, ea, la, h, tmp;
    fmpz_init(ym);
    fmpz_init(b);
    fmpz_init(shift);
    fmpz_init(et);
    fmpz_init(ea);
    fmpz_init(la);
    fmpz_init(h);
    fmpz_init(tmp);
    arf_get_fmpz_2exp(ym, b, y);
    fmpq_t r, v;
    fmpq_init(r);
    fmpq_init(v);
    /* r = Y n / d, so that T = r 2^b */
    fmpz_mul(fmpq_numref(r), ym, n);
    fmpz_set(fmpq_denref(r), d);
    fmpq_canonicalise(r);
    /* et: an upper bound for log2 |T|, within 1 */
    fmpz_set_si(et, (slong) fmpz_bits(fmpq_numref(r)) - (slong) fmpz_bits(fmpq_denref(r)) + 1);
    fmpz_add(et, et, b);
    if (arf_is_zero(a))
    {
        o = round_fmpq_2exp(z, r, b, prec, rnd);
        goto done;
    }
    fmpz_set(ea, ARF_EXPREF(a));
    lowest_bit(la, a);
    fmpz_sub(tmp, ea, et);
    if (fmpz_cmp_si(tmp, FAR_GAP) > 0)
    {
        /* T may be far below A: g = min(la, EXP(A) - prec - 2) */
        fmpz_sub_si(h, ea, prec + 2);
        if (fmpz_cmp(la, h) < 0)
            fmpz_set(h, la);
        if (fmpz_cmp(et, h) <= 0)
        {
            arf_t s;
            arf_init(s);
            arf_set_si(s, fmpq_sgn(r));
            fmpz_sub_ui(h, h, 1);
            arf_mul_2exp_fmpz(s, s, h);
            o = round_sum(z, a, s, prec, rnd);
            arf_clear(s);
            goto done;
        }
    }
    else if (fmpz_cmp_si(tmp, -FAR_GAP) < 0)
    {
        /* A may be far below T: in units of 2^b, j = floor(log2 |r|) - prec - 2 and h = min(j, 0) -
           ceil(log2 d) */
        slong j = (slong) fmpz_bits(fmpq_numref(r)) - (slong) fmpz_bits(fmpq_denref(r)) - 1 - prec - 2;
        slong hh = (j < 0 ? j : 0) - (slong) fmpz_clog_ui(fmpq_denref(r), 2);
        /* |A 2^(-b)| < 2^(EXP(A) - b) */
        fmpz_sub(tmp, ea, b);
        if (fmpz_cmp_si(tmp, hh) <= 0)
        {
            fmpq_t s;
            fmpq_init(s);
            fmpz_set_si(fmpq_numref(s), arf_sgn(a));
            fmpz_one(fmpq_denref(s));
            if (hh - 1 >= 0)
                fmpz_mul_2exp(fmpq_numref(s), fmpq_numref(s), hh - 1);
            else
                fmpz_mul_2exp(fmpq_denref(s), fmpq_denref(s), 1 - hh);
            fmpq_add(v, r, s);
            o = round_fmpq_2exp(z, v, b, prec, rnd);
            fmpq_clear(s);
            goto done;
        }
    }
    /* exact: A + T = (A 2^(-k) + r 2^(b - k)) 2^k with k = min(la, b) */
    {
        fmpz_t k, am;
        fmpz_init(k);
        fmpz_init(am);
        fmpz_set(k, fmpz_cmp(la, b) < 0 ? la : b);
        arf_get_fmpz_2exp(am, tmp, a);
        fmpz_sub(tmp, tmp, k);
        fmpz_mul_2exp(am, am, fmpz_get_ui(tmp));
        fmpz_sub(tmp, b, k);
        fmpq_set(v, r);
        fmpz_mul_2exp(fmpq_numref(v), fmpq_numref(v), fmpz_get_ui(tmp));
        fmpq_canonicalise(v);
        fmpq_add_fmpz(v, v, am);
        o = round_fmpq_2exp(z, v, k, prec, rnd);
        fmpz_clear(k);
        fmpz_clear(am);
    }
done:
    fmpq_clear(r);
    fmpq_clear(v);
    fmpz_clear(ym);
    fmpz_clear(b);
    fmpz_clear(shift);
    fmpz_clear(et);
    fmpz_clear(ea);
    fmpz_clear(la);
    fmpz_clear(h);
    fmpz_clear(tmp);
    return o;
}

/* a + y q (q already negated for a subtraction), rounded; returns the ordering. */
static int
expected(arf_t z, const arf_t a, const arf_t y, const fmpq_t q, slong prec, arf_rnd_t rnd)
{
    if (arf_is_nan(a) || arf_is_nan(y))
    {
        arf_nan(z);
        return 0;
    }
    if (arf_is_inf(y))
    {
        if (fmpq_is_zero(q))
        {
            arf_nan(z);
            return 0;
        }
        int s = arf_sgn(y) * fmpq_sgn(q);
        if (arf_is_inf(a) && arf_sgn(a) != s)
            arf_nan(z);
        else if (s > 0)
            arf_pos_inf(z);
        else
            arf_neg_inf(z);
        return 0;
    }
    if (arf_is_inf(a))
    {
        arf_set(z, a);
        return 0;
    }
    return fused_finite(z, a, y, q, prec, rnd);
}

static const char * OPS[] = {".mul_add_mul_rational_prec", ".mul_sub_mul_rational_prec",
                             ".add_mul_rational_prec", ".sub_mul_rational_prec"};

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
    int op = -1;
    char * dot = NULL;
    for (int i = 0; i < 4 && op < 0; i++)
        if ((dot = strstr(line, OPS[i])) != NULL)
            op = i;
    if (op < 0)
        return report(line_number, "unrecognized function");
    *dot = '\0';
    char * rest = dot + strlen(OPS[op]);
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
    int four = op < 2;
    int negate = op == 1 || op == 3;
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
    int n_floats = four ? 2 : 1;
    if (n_items != n_floats + 2 + with_rm)
        return report(line_number, "wrong number of arguments");
    arf_t x, y, w, a, z, got;
    fmpq_t q;
    arf_init(x);
    arf_init(y);
    arf_init(w);
    arf_init(a);
    arf_init(z);
    arf_init(got);
    fmpq_init(q);
    int res = 0;
    char * end = NULL;
    long long prec = strtoll(items[n_floats + 1], &end, 10);
    arf_rnd_t rnd = ARF_RND_NEAR;
    int exact = 0;
    if (arf_parse_base(line, 4, x) != 0 || arf_parse_base(items[0], 4, y) != 0
        || (four && arf_parse_base(items[1], 4, w) != 0))
        res = report(line_number, "unreadable argument");
    else if (parse_rational(q, items[n_floats]) != 0)
        res = report(line_number, "unreadable rational");
    else if (*end != '\0' || prec <= 0)
        res = report(line_number, "unreadable precision");
    else if (with_rm && parse_rm(items[n_items - 1], &rnd, &exact) != 0)
        res = report(line_number, "unreadable rounding mode");
    if (res == 0)
    {
        if (negate)
            fmpq_neg(q, q);
        int o;
        if (four)
        {
            /* a = xy exactly; arf_mul gives NaN for an infinity times zero */
            arf_mul(a, x, y, ARF_PREC_EXACT, ARF_RND_DOWN);
            if (arf_is_nan(a) || arf_is_nan(w))
            {
                arf_nan(z);
                o = 0;
            }
            else
                o = expected(z, a, w, q, prec, rnd);
        }
        else
            o = expected(z, x, y, q, prec, rnd);
        size_t vl = strlen(result);
        if (vl < 2 || result[0] != '(' || result[vl - 1] != ')')
            res = report(line_number, "unrecognized result");
        else
        {
            result[vl - 1] = '\0';
            char * value = result + 1;
            char * ordering = split(value, ", ");
            if (exact && o != 0)
                res = report(line_number, "Exact rounding of an inexact result");
            else if (ordering == NULL || arf_parse_base(value, 4, got) != 0)
                res = report(line_number, "unreadable result");
            else if (!arf_equal(got, z))
                res = report(line_number, "wrong value");
            else if (strcmp(ordering, ordering_name(o)) != 0)
                res = report(line_number, "wrong ordering");
        }
    }
    arf_clear(x);
    arf_clear(y);
    arf_clear(w);
    arf_clear(a);
    arf_clear(z);
    arf_clear(got);
    fmpq_clear(q);
    if (res == 0)
        checked++;
    return res;
}

int
run_arf_qfused(const char * arg)
{
    checked = 0;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines("arf_qfused", checked);
}
