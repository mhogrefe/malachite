/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs BigExponentFloat's roots of Rationals against exact integer arithmetic. The mode
    `arf_qroot` accepts lines of the forms `BigExponentFloat::f_rational_prec_round(Q, p, rm) =
    (Z, O)` for f among sqrt, cbrt, and reciprocal_sqrt, and `BigExponentFloat::f_rational_prec_round(Q,
    K, p, rm) = (Z, O)` for f among root_u and root_s, and their `_prec` forms. Q is a Rational
    written as `a/b` or `a`, and Z is a BigExponentFloat in Malachite's exact hexadecimal format.

    The special cases follow Malachite's documentation: the root of order 0 is NaN; at 0 the root
    is 0 for a positive order, infinity for a negative even order, and NaN for a negative odd
    order; and a negative Q with an even order gives NaN. A negative Q with an odd order has the
    negated root of -Q.

    For Q = n/d > 0 and an order K, the root of order K is that of order |K| of n/d or, for a
    negative K, of d/n. With n/d renamed so, and K > 0, the integer Y = floor((n 2^(K S) / d)^(1/K))
    is computed with fmpz_root, for an S that gives Y at least p + 2 bits; the root lies in [Y, Y +
    1) 2^(-S), and is Y 2^(-S) exactly when Y^K d = n 2^(K S). Rounding Y with that sticky bit gives
    the rounded root and its ordering. This needs about K (p + 2) + bits(n) + bits(d) bits; when
    that is more than 2^24 and K exceeds bits(n) + bits(d), the root is instead bracketed with arb's
    arb_root_ui, the working precision doubling while undecided. Such a root is a rounding
    breakpoint only if it is a power of 2, since an odd m > 1 has an m^K with more than K bits;
    powers of 2 are handled exactly.

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
#define MAX_PREC (1L << 26)
#define INTEGER_BITS_LIMIT (1L << 24)

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

/* The rounding mode for the magnitude of a negative value. */
static arf_rnd_t
mirror(arf_rnd_t rnd)
{
    return rnd == ARF_RND_FLOOR ? ARF_RND_CEIL : rnd == ARF_RND_CEIL ? ARF_RND_FLOOR : rnd;
}

/* Rounds the positive value (y + f) 2^e, where y is an integer with at least prec + 2 bits and f
   is 0 if !sticky and in (0, 1) otherwise, to prec bits in mode rnd; returns the ordering. */
static int
round_sticky(arf_t z, const fmpz_t y, int sticky, const fmpz_t e, slong prec, arf_rnd_t rnd)
{
    slong t = fmpz_bits(y) - prec;
    fmpz_t hi, rem, half, ee;
    fmpz_init(hi);
    fmpz_init(rem);
    fmpz_init(half);
    fmpz_init(ee);
    fmpz_fdiv_q_2exp(hi, y, t);
    fmpz_fdiv_r_2exp(rem, y, t);
    int inexact = sticky || !fmpz_is_zero(rem);
    int up = 0;
    if (inexact)
    {
        if (rnd == ARF_RND_CEIL || rnd == ARF_RND_UP)
            up = 1;
        else if (rnd == ARF_RND_NEAR)
        {
            fmpz_one(half);
            fmpz_mul_2exp(half, half, t - 1);
            int c = fmpz_cmp(rem, half);
            if (c > 0 || (c == 0 && sticky))
                up = 1;
            else if (c == 0)
                up = fmpz_is_odd(hi);
        }
    }
    if (up)
        fmpz_add_ui(hi, hi, 1);
    fmpz_add_si(ee, e, t);
    arf_set_fmpz_2exp(z, hi, ee);
    fmpz_clear(hi);
    fmpz_clear(rem);
    fmpz_clear(half);
    fmpz_clear(ee);
    return !inexact ? 0 : up ? 1 : -1;
}

/* The root of order k > 0 of n/d > 0, by exact integer arithmetic. */
static int
root_integer(arf_t z, const fmpz_t n, const fmpz_t d, ulong k, slong prec, arf_rnd_t rnd)
{
    /* With bits(n) - bits(d) - 1 <= log2(n/d) < bits(n) - bits(d) + 1, choose S so that
       (n/d)^(1/k) 2^S >= 2^(prec + 2). */
    slong lg = (slong) fmpz_bits(n) - (slong) fmpz_bits(d) - 1;
    slong floor_lg_root = lg >= 0 ? lg / (slong) k : -((-lg + (slong) k - 1) / (slong) k);
    slong s = prec + 3 - floor_lg_root;
    fmpz_t num, y, check, e;
    fmpz_init(num);
    fmpz_init(y);
    fmpz_init(check);
    fmpz_init(e);
    /* num = floor(n 2^(k s) / d), with k s possibly negative. */
    fmpz_t shifted_n, shifted_d;
    fmpz_init(shifted_n);
    fmpz_init(shifted_d);
    fmpz_set(shifted_n, n);
    fmpz_set(shifted_d, d);
    slong ks = (slong) k * s;
    if (ks >= 0)
        fmpz_mul_2exp(shifted_n, shifted_n, ks);
    else
        fmpz_mul_2exp(shifted_d, shifted_d, -ks);
    fmpz_fdiv_q(num, shifted_n, shifted_d);
    fmpz_root(y, num, k);
    fmpz_pow_ui(check, y, k);
    fmpz_mul(check, check, shifted_d);
    int sticky = !fmpz_equal(check, shifted_n);
    fmpz_set_si(e, -s);
    int o = round_sticky(z, y, sticky, e, prec, rnd);
    fmpz_clear(num);
    fmpz_clear(y);
    fmpz_clear(check);
    fmpz_clear(e);
    fmpz_clear(shifted_n);
    fmpz_clear(shifted_d);
    return o;
}

/* The root of order k > 0 of n/d > 0 by arb bracketing, for k > bits(n) + bits(d); returns 2 if
   undecided. The root is a power of 2 only if n/d = 2^(j k); otherwise it is not a rounding
   breakpoint, and the bracketing terminates. */
static int
root_arb(arf_t z, const fmpz_t n, const fmpz_t d, ulong k, slong prec, arf_rnd_t rnd)
{
    fmpz_t one;
    fmpz_init(one);
    fmpz_one(one);
    int power_of_2_num = fmpz_is_one(d) && fmpz_bits(n) == fmpz_val2(n) + 1;
    int power_of_2_den = fmpz_is_one(n) && fmpz_bits(d) == fmpz_val2(d) + 1;
    if (power_of_2_num || power_of_2_den)
    {
        slong j = power_of_2_num ? (slong) fmpz_val2(n) : -(slong) fmpz_val2(d);
        /* |j| < k, so the root is a power of 2 only for j = 0, which is n/d = 1. */
        if (j == 0)
        {
            arf_one(z);
            fmpz_clear(one);
            return 0;
        }
    }
    fmpz_clear(one);
    fmpq_t q;
    fmpq_init(q);
    fmpz_set(fmpq_numref(q), n);
    fmpz_set(fmpq_denref(q), d);
    arb_t r;
    arb_init(r);
    arf_t a, b, lo, hi;
    arf_init(a);
    arf_init(b);
    arf_init(lo);
    arf_init(hi);
    int result = 2;
    for (slong wp = prec + 64; result == 2 && wp <= MAX_PREC; wp *= 2)
    {
        arb_set_fmpq(r, q, wp + 64);
        arb_root_ui(r, r, k, wp);
        arb_get_lbound_arf(lo, r, wp);
        arb_get_ubound_arf(hi, r, wp);
        arf_set_round(a, lo, prec, rnd);
        arf_set_round(b, hi, prec, rnd);
        /* Rounding is monotonic, so if both bounds round to a, so does the root; the ordering is
           decided once a is outside the bracket. */
        if (arf_equal(a, b))
        {
            arf_set(z, a);
            if (arf_cmp(a, lo) < 0)
                result = -1;
            else if (arf_cmp(a, hi) > 0)
                result = 1;
        }
    }
    arb_clear(r);
    arf_clear(a);
    arf_clear(b);
    arf_clear(lo);
    arf_clear(hi);
    fmpq_clear(q);
    return result;
}

typedef enum { F_SQRT, F_CBRT, F_RECIPROCAL_SQRT, F_ROOT_U, F_ROOT_S } fn_t;

/* The expected value and ordering (2 if undecided). */
static int
expected(arf_t z, const fmpq_t x, const fmpz_t k, slong prec, arf_rnd_t rnd)
{
    if (fmpz_is_zero(k))
    {
        arf_nan(z);
        return 0;
    }
    int k_negative = fmpz_sgn(k) < 0;
    int k_odd = fmpz_is_odd(k);
    if (fmpq_is_zero(x))
    {
        if (!k_negative)
            arf_zero(z);
        else if (k_odd)
            arf_nan(z);
        else
            arf_pos_inf(z);
        return 0;
    }
    int negative = fmpq_sgn(x) < 0;
    if (negative && !k_odd)
    {
        arf_nan(z);
        return 0;
    }
    fmpz_t n, d, ka;
    fmpz_init(n);
    fmpz_init(d);
    fmpz_init(ka);
    fmpz_abs(n, fmpq_numref(x));
    fmpz_set(d, fmpq_denref(x));
    if (k_negative)
        fmpz_swap(n, d);
    fmpz_abs(ka, k);
    arf_rnd_t r = negative ? mirror(rnd) : rnd;
    int o;
    slong size = (slong) fmpz_bits(n) + (slong) fmpz_bits(d);
    if (!fmpz_abs_fits_ui(ka))
        o = 2;
    else
    {
        ulong kk = fmpz_get_ui(ka);
        if (kk <= (ulong) size || (kk <= (ulong) INTEGER_BITS_LIMIT
                                   && kk * (ulong) (prec + 2) <= (ulong) INTEGER_BITS_LIMIT))
            o = root_integer(z, n, d, kk, prec, r);
        else
            o = root_arb(z, n, d, kk, prec, r);
    }
    if (negative && o != 2)
    {
        arf_neg(z, z);
        o = -o;
    }
    fmpz_clear(n);
    fmpz_clear(d);
    fmpz_clear(ka);
    return o;
}

static int
check_line(char * line, int line_number)
{
    size_t len = strlen(line);
    if (len != 0 && line[len - 1] == '\n')
        line[len - 1] = '\0';
    if (line[0] == '\0')
        return 0;
    char * result = split(line, " = ");
    const char * prefix = "BigExponentFloat::";
    if (result == NULL || strncmp(line, prefix, strlen(prefix)) != 0)
        return report(line_number, "unrecognized line");
    char * name = line + strlen(prefix);
    char * args = split(name, "(");
    size_t al = args == NULL ? 0 : strlen(args);
    if (args == NULL || al == 0 || args[al - 1] != ')')
        return report(line_number, "unrecognized line");
    args[al - 1] = '\0';
    char * suffix = strstr(name, "_rational_prec");
    if (suffix == NULL
        || (strcmp(suffix, "_rational_prec_round") != 0 && strcmp(suffix, "_rational_prec") != 0))
        return report(line_number, "unrecognized function");
    int with_rm = strcmp(suffix, "_rational_prec_round") == 0;
    *suffix = '\0';
    fn_t f;
    if (strcmp(name, "sqrt") == 0)
        f = F_SQRT;
    else if (strcmp(name, "cbrt") == 0)
        f = F_CBRT;
    else if (strcmp(name, "reciprocal_sqrt") == 0)
        f = F_RECIPROCAL_SQRT;
    else if (strcmp(name, "root_u") == 0)
        f = F_ROOT_U;
    else if (strcmp(name, "root_s") == 0)
        f = F_ROOT_S;
    else
        return report(line_number, "unrecognized function");
    int has_k = f == F_ROOT_U || f == F_ROOT_S;
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
    if (n_items != 2 + has_k + with_rm)
        return report(line_number, "wrong number of arguments");
    fmpq_t x;
    fmpz_t k;
    arf_t z, got;
    fmpq_init(x);
    fmpz_init(k);
    arf_init(z);
    arf_init(got);
    int res = 0;
    char * end = NULL;
    long long prec = strtoll(items[1 + has_k], &end, 10);
    arf_rnd_t rnd = ARF_RND_NEAR;
    int exact = 0;
    if (parse_rational(x, items[0]) != 0)
        res = report(line_number, "unreadable argument");
    else if (*end != '\0' || prec <= 0)
        res = report(line_number, "unreadable precision");
    else if (has_k && (fmpz_set_str(k, items[1], 10) != 0
                       || (f == F_ROOT_U && fmpz_sgn(k) < 0)))
        res = report(line_number, "unreadable order");
    else if (with_rm && parse_rm(items[n_items - 1], &rnd, &exact) != 0)
        res = report(line_number, "unreadable rounding mode");
    if (res == 0)
    {
        if (f == F_SQRT)
            fmpz_set_si(k, 2);
        else if (f == F_CBRT)
            fmpz_set_si(k, 3);
        else if (f == F_RECIPROCAL_SQRT)
            fmpz_set_si(k, -2);
        int o = expected(z, x, k, prec, rnd);
        size_t rl = strlen(result);
        if (rl < 2 || result[0] != '(' || result[rl - 1] != ')')
            res = report(line_number, "unrecognized result");
        else
        {
            result[rl - 1] = '\0';
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
    fmpq_clear(x);
    fmpz_clear(k);
    arf_clear(z);
    arf_clear(got);
    if (res == 0)
        checked++;
    return res;
}

int
run_arf_qroot(const char * arg)
{
    checked = 0;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines("arf_qroot", checked);
}
