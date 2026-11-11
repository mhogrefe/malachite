/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the conversions of Malachite's BigExponentFloat against FLINT's arf_t. A BigExponentFloat
    is written in Malachite's exact hexadecimal format, integers in decimal, rationals as `a/b`, and
    doubles in their shortest round-tripping decimal form. The modes:

    - `arf_set_fmpz`: `BigExponentFloat::from(n) = x` for an integer n, and the
      `from_{natural,integer,unsigned,signed}_prec[_round]` constructors, against arf_set_fmpz and
      arf_set_round_fmpz.
    - `arf_set_d`: `BigExponentFloat::from(d) = x` for a double d, and
      `from_primitive_float_prec_round`, against arf_set_d and arf_set_round.
    - `arf_fmpq`: `from_rational_prec[_round]`, `try_from` and `convertible_from` of a rational,
      against arf_fmpz_div_fmpz and the dyadic test.
    - `arf_get_fmpz`: rounding a BigExponentFloat to a Natural, Integer, u64, or i64, the
      `try_from` and `convertible_from` conversions to those types and to Rational and the
      Gaussian types, and `is_integer`, `is_real`, and `is_gaussian_integer`, against arf_get_fmpz,
      arf_is_int, and arf_get_fmpq. A negative value rounds to the Natural or u64 zero, as
      Malachite documents.
    - `arf_get_d`: `f64::rounding_from`, `f64::try_from`, and `f64::convertible_from`, against
      arf_get_d.
    - `arf_mantissa`: the raw, integer, and scientific mantissas and exponents, the constructor
      from an integer mantissa and exponent, and `sci_mantissa_and_exponent_round::<f64>`, against
      arf_get_fmpz_2exp, arf_frexp, arf_set_fmpz_2exp, and arf_get_d.
    - `arf_string`: `to_string_base` and `to_string_base_upper` in bases 2, 4, 8, 16, and 32, the
      `#`-flag `Binary` and `Octal` output, and `from_string_base`, against an independent parser.

    Any other nonempty line is an error, as is an input with no lines at all.
*/

#include <ctype.h>
#include <float.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>

#include <flint/arf.h>
#include <flint/fmpq.h>
#include <flint/fmpz.h>

#include "oracle.h"

typedef enum
{
    KIND_SET_FMPZ,
    KIND_SET_D,
    KIND_FMPQ,
    KIND_GET_FMPZ,
    KIND_GET_D,
    KIND_MANTISSA,
    KIND_STRING
} conversion_kind;

static long checked;
static conversion_kind kind;
static const char * mode_name;

static int
fail(int line_number, const char * what)
{
    flint_printf("error in %s test, line %d: %s\n", mode_name, line_number, what);
    return 1;
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

/* Removes `prefix` from the start of `*s`, returning whether it was there. */
static int
eat(char ** s, const char * prefix)
{
    size_t n = strlen(prefix);
    if (strncmp(*s, prefix, n) != 0)
        return 0;
    *s += n;
    return 1;
}

/* Strips `open` from the start of `s` and `close` from its end, in place, returning the inside or
   NULL. */
static char *
strip(char * s, const char * open, char close)
{
    size_t n = strlen(s);
    size_t k = strlen(open);
    if (strncmp(s, open, k) != 0 || n <= k || s[n - 1] != close)
        return NULL;
    s[n - 1] = '\0';
    return s + k;
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
        return 1;
    return 0;
}

static const char *
ordering_name(int c)
{
    return c < 0 ? "Less" : c > 0 ? "Greater" : "Equal";
}

/* Parses a decimal integer, which must be the whole of `s`. */
static int
parse_fmpz(const char * s, fmpz_t n)
{
    const char * t = *s == '-' ? s + 1 : s;
    if (*t == '\0')
        return 1;
    for (const char * p = t; *p != '\0'; p++)
        if (!isdigit((unsigned char) *p))
            return 1;
    return fmpz_set_str(n, s, 10);
}

/* Parses a rational written as `a/b` or `a`. */
static int
parse_fmpq(const char * s, fmpq_t q)
{
    char * buf = flint_malloc(strlen(s) + 1);
    strcpy(buf, s);
    char * den = split(buf, "/");
    int result = parse_fmpz(buf, fmpq_numref(q));
    if (result == 0)
    {
        if (den == NULL)
            fmpz_one(fmpq_denref(q));
        else
            result = parse_fmpz(den, fmpq_denref(q));
    }
    if (result == 0)
        fmpq_canonicalise(q);
    flint_free(buf);
    return result;
}

/* Parses a double in the shortest round-tripping form: `NaN`, `Infinity`, `-Infinity`, or a
   decimal number. */
static int
parse_d(const char * s, double * d)
{
    char * end;
    if (strcmp(s, "NaN") == 0)
        *d = NAN;
    else if (strcmp(s, "Infinity") == 0)
        *d = INFINITY;
    else if (strcmp(s, "-Infinity") == 0)
        *d = -INFINITY;
    else
    {
        *d = strtod(s, &end);
        if (*end != '\0' || end == s)
            return 1;
    }
    return 0;
}

/* Whether the doubles `a` and `b` are the same, NaN being the same as NaN and the zeros distinct. */
static int
same_d(double a, double b)
{
    return (isnan(a) && isnan(b)) || (a == b && signbit(a) == signbit(b));
}

/* Checks `(X, O)` against the arf `z` and the direction `c` of rounding. */
static int
check_pair(int line_number, char * pair, const arf_t z, int c)
{
    char * inner = strip(pair, "(", ')');
    char * o = inner == NULL ? NULL : strstr(inner, ", ");
    if (o == NULL)
        return fail(line_number, "unreadable result");
    *o = '\0';
    o += 2;
    arf_t y;
    arf_init(y);
    int result = 0;
    if (arf_parse_base(inner, 4, y))
        result = fail(line_number, "unreadable value");
    else if (!arf_equal(y, z))
        result = fail(line_number, "wrong value");
    else if (strcmp(o, ordering_name(c)) != 0)
        result = fail(line_number, "wrong ordering");
    arf_clear(y);
    return result;
}

/* ---------------------------------------------------------------- arf_set_fmpz and arf_set_d */

static int
check_set(char * line, int line_number, int is_double)
{
    char * rest;
    if (!eat(&line, "BigExponentFloat::"))
        return fail(line_number, "unrecognized line");
    int result = 0;
    arf_t x, z;
    arf_init(x);
    arf_init(z);
    fmpz_t n;
    fmpz_init(n);
    double d = 0;
    if (eat(&line, "from("))
    {
        /* `from(v) = X` */
        rest = split(line, ") = ");
        if (rest == NULL || (is_double ? parse_d(line, &d) : parse_fmpz(line, n))
            || arf_parse_base(rest, 4, x))
            result = fail(line_number, "unreadable line");
        else
        {
            if (is_double)
                arf_set_d(z, d);
            else
                arf_set_fmpz(z, n);
            if (!arf_equal(z, x))
                result = fail(line_number, "wrong value");
        }
    }
    else
    {
        /* `from_..._prec_round(v, p, rm) = (X, O)` or `from_..._prec(v, p) = (X, O)` */
        char * args = split(line, "(");
        rest = args == NULL ? NULL : split(args, ") = ");
        int with_rm = strstr(line, "_prec_round") != NULL;
        char * p_str = args == NULL ? NULL : split(args, ", ");
        char * rm_str = with_rm && p_str != NULL ? split(p_str, ", ") : NULL;
        arf_rnd_t rnd = ARF_RND_NEAR;
        int exact = 0;
        char * end;
        unsigned long long prec = p_str == NULL ? 0 : strtoull(p_str, &end, 10);
        if (rest == NULL || p_str == NULL || prec == 0 || *end != '\0'
            || (with_rm && (rm_str == NULL || parse_rm(rm_str, &rnd, &exact)))
            || (is_double ? parse_d(args, &d) : parse_fmpz(args, n)))
            result = fail(line_number, "unreadable line");
        else
        {
            if (is_double)
                arf_set_d(x, d);
            else
                arf_set_fmpz(x, n);
            int inexact = arf_set_round(z, x, (slong) prec, rnd);
            int c = (arf_is_nan(x)) ? 0 : arf_cmp(z, x);
            if (exact && inexact)
                result = fail(line_number, "Exact rounding of an inexact value");
            else
                result = check_pair(line_number, rest, z, c);
        }
    }
    arf_clear(x);
    arf_clear(z);
    fmpz_clear(n);
    return result;
}

/* ---------------------------------------------------------------- arf_fmpq */

/* Whether the positive `d` is a power of 2. */
static int
fmpz_is_power_of_2(const fmpz_t d)
{
    return fmpz_sgn(d) > 0 && fmpz_bits(d) == fmpz_val2(d) + 1;
}

static int
check_fmpq(char * line, int line_number)
{
    if (!eat(&line, "BigExponentFloat::"))
        return fail(line_number, "unrecognized line");
    int result = 0;
    fmpq_t q, r;
    fmpq_init(q);
    fmpq_init(r);
    arf_t z, x;
    arf_init(z);
    arf_init(x);
    char * rest;
    if (eat(&line, "try_from(") || (strncmp(line, "convertible_from(", 17) == 0))
    {
        int convertible = eat(&line, "convertible_from(");
        rest = split(line, ") = ");
        if (rest == NULL || parse_fmpq(line, q))
            result = fail(line_number, "unreadable line");
        else
        {
            int dyadic = fmpz_is_power_of_2(fmpq_denref(q));
            if (convertible)
                result = strcmp(rest, dyadic ? "true" : "false") == 0
                             ? 0
                             : fail(line_number, "wrong convertibility");
            else if (!dyadic)
                result = strcmp(rest, "Err(BigExponentFloatFromRationalError)") == 0
                             ? 0
                             : fail(line_number, "expected an error");
            else
            {
                char * inner = strip(rest, "Ok(", ')');
                fmpz_t e;
                fmpz_init(e);
                fmpz_set_si(e, -(slong) fmpz_val2(fmpq_denref(q)));
                arf_set_fmpz_2exp(z, fmpq_numref(q), e);
                fmpz_clear(e);
                if (inner == NULL || arf_parse_base(inner, 4, x))
                    result = fail(line_number, "unreadable value");
                else if (!arf_equal(x, z))
                    result = fail(line_number, "wrong value");
            }
        }
    }
    else
    {
        char * args = split(line, "(");
        rest = args == NULL ? NULL : split(args, ") = ");
        int with_rm = strstr(line, "_prec_round") != NULL;
        char * p_str = args == NULL ? NULL : split(args, ", ");
        char * rm_str = with_rm && p_str != NULL ? split(p_str, ", ") : NULL;
        arf_rnd_t rnd = ARF_RND_NEAR;
        int exact = 0;
        char * end;
        unsigned long long prec = p_str == NULL ? 0 : strtoull(p_str, &end, 10);
        if (rest == NULL || p_str == NULL || prec == 0 || *end != '\0'
            || (with_rm && (rm_str == NULL || parse_rm(rm_str, &rnd, &exact)))
            || parse_fmpq(args, q))
            result = fail(line_number, "unreadable line");
        else
        {
            int inexact = arf_fmpz_div_fmpz(z, fmpq_numref(q), fmpq_denref(q), (slong) prec, rnd);
            arf_get_fmpq(r, z);
            int c = fmpq_cmp(r, q);
            if (exact && inexact)
                result = fail(line_number, "Exact rounding of an inexact value");
            else
                result = check_pair(line_number, rest, z, c);
        }
    }
    fmpq_clear(q);
    fmpq_clear(r);
    arf_clear(z);
    arf_clear(x);
    return result;
}

/* ---------------------------------------------------------------- arf_get_fmpz */

/* Checks `expected` against `Ok(<text of value>)`. */
static int
check_ok(int line_number, const char * expected, const char * value)
{
    size_t n = strlen(value);
    if (strncmp(expected, "Ok(", 3) != 0 || strncmp(expected + 3, value, n) != 0
        || strcmp(expected + 3 + n, ")") != 0)
        return fail(line_number, "wrong value");
    return 0;
}

static int
check_get_fmpz(char * line, int line_number)
{
    /* `T::method(X, ...) = r`, or `(X).method() = r` */
    char * rest;
    char * target;
    char * method;
    char * arg;
    if (line[0] == '(')
    {
        arg = line + 1;
        method = split(arg, ").");
        rest = method == NULL ? NULL : split(method, "() = ");
        target = NULL;
    }
    else
    {
        target = line;
        method = split(target, "::");
        arg = method == NULL ? NULL : split(method, "(");
        rest = arg == NULL ? NULL : split(arg, ") = ");
    }
    if (rest == NULL)
        return fail(line_number, "unrecognized line");
    char * rm_str = NULL;
    if (strcmp(method, "rounding_from") == 0)
    {
        rm_str = strrchr(arg, ',');
        if (rm_str == NULL)
            return fail(line_number, "unrecognized line");
        *rm_str = '\0';
        rm_str += 2;
    }
    arf_t x, z;
    arf_init(x);
    arf_init(z);
    fmpz_t n;
    fmpz_init(n);
    fmpq_t q;
    fmpq_init(q);
    int result = 0;
    if (arf_parse_base(arg, 4, x))
    {
        result = fail(line_number, "unreadable value");
        goto done;
    }
    int finite = arf_is_finite(x);
    int integer = finite && arf_is_int(x);
    if (target == NULL)
    {
        /* is_integer, is_real, is_gaussian_integer */
        int v = strcmp(method, "is_real") == 0 ? finite : integer;
        if (strcmp(method, "is_integer") != 0 && strcmp(method, "is_real") != 0
            && strcmp(method, "is_gaussian_integer") != 0)
            result = fail(line_number, "unrecognized method");
        else if (strcmp(rest, v ? "true" : "false") != 0)
            result = fail(line_number, "wrong result");
        goto done;
    }
    int unsigned_target = strcmp(target, "Natural") == 0 || strcmp(target, "u64") == 0;
    int bounded = strcmp(target, "u64") == 0 || strcmp(target, "i64") == 0;
    if (rm_str != NULL)
    {
        arf_rnd_t rnd;
        int exact;
        if (parse_rm(rm_str, &rnd, &exact))
        {
            result = fail(line_number, "unreadable rounding mode");
            goto done;
        }
        int c;
        if (unsigned_target && arf_sgn(x) < 0)
        {
            /* Malachite rounds a negative value to zero, from above */
            fmpz_zero(n);
            c = 1;
        }
        else
        {
            int inexact = arf_get_fmpz(n, x, rnd);
            if (exact && inexact)
            {
                result = fail(line_number, "Exact rounding of an inexact value");
                goto done;
            }
            arf_set_fmpz(z, n);
            c = arf_cmp(z, x);
        }
        char * s = fmpz_get_str(NULL, 10, n);
        char * expected = flint_malloc(strlen(s) + 32);
        sprintf(expected, "(%s, %s)", s, ordering_name(c));
        if (strcmp(expected, rest) != 0)
            result = fail(line_number, "wrong rounding");
        flint_free(expected);
        flint_free(s);
        goto done;
    }
    int is_try = strcmp(method, "try_from") == 0;
    int is_convertible = strcmp(method, "convertible_from") == 0;
    if (!is_try && !is_convertible)
    {
        result = fail(line_number, "unrecognized method");
        goto done;
    }
    if (strcmp(target, "Rational") == 0 || strcmp(target, "GaussianRational") == 0)
    {
        int gaussian = target[0] == 'G';
        if (is_convertible)
            result = strcmp(rest, finite ? "true" : "false") == 0 ? 0
                                                                   : fail(line_number, "wrong result");
        else if (!finite)
            result = strcmp(rest, gaussian ? "Err(GaussianRationalFromFloatError)"
                                           : "Err(RationalFromFloatError)") == 0
                         ? 0
                         : fail(line_number, "expected an error");
        else
        {
            arf_get_fmpq(q, x);
            char * s = fmpq_get_str(NULL, 10, q);
            result = check_ok(line_number, rest, s);
            flint_free(s);
        }
        goto done;
    }
    /* integer targets */
    const char * err = NULL;
    if (!finite)
        err = "FloatInfiniteOrNan";
    else if (unsigned_target && arf_sgn(x) < 0 && strcmp(target, "GaussianInteger") != 0)
        err = "FloatNegative";
    else if (!integer)
        err = "FloatNonIntegerOrOutOfRange";
    else if (bounded && fmpz_cmp_ui(ARF_EXPREF(x), 65) > 0)
        /* too large for 64 bits; checked first, so that a huge integer is never formed */
        err = "FloatNonIntegerOrOutOfRange";
    else
    {
        arf_get_fmpz(n, x, ARF_RND_DOWN);
        if (bounded && (unsigned_target ? !fmpz_abs_fits_ui(n) || fmpz_sgn(n) < 0
                                        : !fmpz_fits_si(n)))
            err = "FloatNonIntegerOrOutOfRange";
    }
    int gaussian = strcmp(target, "GaussianInteger") == 0;
    if (is_convertible)
        result = strcmp(rest, err == NULL ? "true" : "false") == 0 ? 0
                                                                     : fail(line_number, "wrong result");
    else if (err != NULL)
    {
        char expected[64];
        snprintf(expected, sizeof(expected), "Err(%s)",
                 gaussian ? "GaussianIntegerFromFloatError" : err);
        result = strcmp(rest, expected) == 0 ? 0 : fail(line_number, "expected an error");
    }
    else
    {
        char * s = fmpz_get_str(NULL, 10, n);
        result = check_ok(line_number, rest, s);
        flint_free(s);
    }
done:
    arf_clear(x);
    arf_clear(z);
    fmpz_clear(n);
    fmpq_clear(q);
    return result;
}

/* ---------------------------------------------------------------- arf_get_d */

static int
check_get_d(char * line, int line_number)
{
    char * method;
    if (!eat(&line, "f64::"))
        return fail(line_number, "unrecognized line");
    method = line;
    char * arg = split(method, "(");
    char * rest = arg == NULL ? NULL : split(arg, ") = ");
    if (rest == NULL)
        return fail(line_number, "unrecognized line");
    char * rm_str = NULL;
    if (strcmp(method, "rounding_from") == 0)
    {
        rm_str = strrchr(arg, ',');
        if (rm_str == NULL)
            return fail(line_number, "unrecognized line");
        *rm_str = '\0';
        rm_str += 2;
    }
    arf_t x, z;
    arf_init(x);
    arf_init(z);
    int result = 0;
    if (arf_parse_base(arg, 4, x))
    {
        result = fail(line_number, "unreadable value");
        goto done;
    }
    if (rm_str != NULL)
    {
        arf_rnd_t rnd;
        int exact;
        if (parse_rm(rm_str, &rnd, &exact))
        {
            result = fail(line_number, "unreadable rounding mode");
            goto done;
        }
        double d = arf_get_d(x, rnd);
        int c = 0;
        if (!arf_is_nan(x))
        {
            arf_set_d(z, d);
            c = arf_cmp(z, x);
        }
        char * inner = strip(rest, "(", ')');
        char * o = inner == NULL ? NULL : strstr(inner, ", ");
        double e;
        if (o == NULL)
        {
            result = fail(line_number, "unreadable result");
            goto done;
        }
        *o = '\0';
        o += 2;
        if (parse_d(inner, &e))
            result = fail(line_number, "unreadable double");
        else if (exact && c != 0)
            result = fail(line_number, "Exact rounding of an inexact value");
        else if (!same_d(d, e))
        {
            flint_printf("error in %s test, line %d: wrong double. FLINT: %.17g, Malachite: %s\n",
                         mode_name, line_number, d, inner);
            result = 1;
        }
        else if (strcmp(o, ordering_name(c)) != 0)
            result = fail(line_number, "wrong ordering");
        goto done;
    }
    /* try_from and convertible_from */
    const char * err = NULL;
    double d = 0;
    if (arf_is_nan(x))
        d = NAN;
    else if (arf_is_inf(x))
        d = arf_sgn(x) > 0 ? INFINITY : -INFINITY;
    else if (!arf_is_zero(x))
    {
        /* the scientific exponent */
        fmpz_t se;
        fmpz_init(se);
        fmpz_sub_ui(se, ARF_EXPREF(x), 1);
        if (fmpz_cmp_si(se, 1023) > 0)
            err = "Overflow";
        else if (fmpz_cmp_si(se, -1074) < 0)
            err = "Underflow";
        else
        {
            d = arf_get_d(x, ARF_RND_DOWN);
            arf_set_d(z, d);
            if (!arf_equal(z, x))
                err = "Inexact";
        }
        fmpz_clear(se);
    }
    if (strcmp(method, "convertible_from") == 0)
        result = strcmp(rest, err == NULL ? "true" : "false") == 0 ? 0
                                                                     : fail(line_number, "wrong result");
    else if (strcmp(method, "try_from") == 0)
    {
        if (err != NULL)
        {
            char expected[32];
            snprintf(expected, sizeof(expected), "Err(%s)", err);
            result = strcmp(rest, expected) == 0 ? 0 : fail(line_number, "expected an error");
        }
        else
        {
            char * inner = strip(rest, "Ok(", ')');
            double e;
            if (inner == NULL || parse_d(inner, &e))
                result = fail(line_number, "unreadable result");
            else if (!same_d(d, e))
                result = fail(line_number, "wrong double");
        }
    }
    else
        result = fail(line_number, "unrecognized method");
done:
    arf_clear(x);
    arf_clear(z);
    return result;
}

/* ---------------------------------------------------------------- arf_mantissa */

static int
check_mantissa(char * line, int line_number)
{
    arf_t x, z, m;
    arf_init(x);
    arf_init(z);
    arf_init(m);
    fmpz_t a, e, b;
    fmpz_init(a);
    fmpz_init(e);
    fmpz_init(b);
    int result = 0;
    if (eat(&line, "BigExponentFloat::from_integer_mantissa_and_exponent("))
    {
        /* `(m, e) = Some(X)` */
        char * rest = split(line, ") = ");
        char * e_str = rest == NULL ? NULL : split(line, ", ");
        char * inner = rest == NULL ? NULL : strip(rest, "Some(", ')');
        if (e_str == NULL || inner == NULL || parse_fmpz(line, a) || parse_fmpz(e_str, e)
            || arf_parse_base(inner, 4, x))
            result = fail(line_number, "unreadable line");
        else
        {
            arf_set_fmpz_2exp(z, a, e);
            if (!arf_equal(z, x))
                result = fail(line_number, "wrong value");
        }
        goto done;
    }
    if (line[0] != '(')
    {
        result = fail(line_number, "unrecognized line");
        goto done;
    }
    char * arg = line + 1;
    char * method = split(arg, ").");
    char * rest = method == NULL ? NULL : split(method, ") = ");
    if (rest == NULL || arf_parse_base(arg, 4, x) || !arf_is_normal(x))
    {
        result = fail(line_number, "unreadable line");
        goto done;
    }
    ulong bits = arf_bits(x);
    /* the scientific mantissa |x| / 2^(e - 1), in [1, 2), and exponent e - 1 */
    arf_frexp(m, e, x);
    arf_abs(m, m);
    arf_mul_2exp_si(m, m, 1);
    fmpz_sub_ui(e, e, 1);
    char * e_s = fmpz_get_str(NULL, 10, e);
    if (strcmp(method, "raw_mantissa_and_exponent(") == 0
        || strcmp(method, "integer_mantissa_and_exponent(") == 0)
    {
        arf_get_fmpz_2exp(a, b, x);
        fmpz_abs(a, a);
        char * s;
        char * t;
        if (method[0] == 'r')
        {
            /* the odd mantissa, filling whole 64-bit limbs, and the exponent */
            fmpz_mul_2exp(a, a, (64 - bits % 64) % 64);
            fmpz_set(b, ARF_EXPREF(x));
        }
        s = fmpz_get_str(NULL, 10, a);
        t = fmpz_get_str(NULL, 10, b);
        char * expected = flint_malloc(strlen(s) + strlen(t) + 8);
        sprintf(expected, "(%s, %s)", s, t);
        if (strcmp(expected, rest) != 0)
            result = fail(line_number, "wrong mantissa or exponent");
        flint_free(expected);
        flint_free(s);
        flint_free(t);
    }
    else if (strcmp(method, "sci_mantissa_and_exponent(") == 0)
    {
        char * inner = strip(rest, "(", ')');
        char * e_str = inner == NULL ? NULL : split(inner, ", ");
        if (e_str == NULL || arf_parse_base(inner, 4, z))
            result = fail(line_number, "unreadable result");
        else if (!arf_equal(z, m) || strcmp(e_str, e_s) != 0)
            result = fail(line_number, "wrong scientific mantissa or exponent");
    }
    else if (eat(&method, "sci_mantissa_and_exponent_round::<f64>("))
    {
        arf_rnd_t rnd;
        int exact;
        if (parse_rm(method, &rnd, &exact))
        {
            result = fail(line_number, "unreadable rounding mode");
            goto mantissa_done;
        }
        double d = arf_get_d(m, rnd);
        arf_set_d(z, d);
        int c = arf_cmp(z, m);
        if (exact && c != 0)
        {
            result = strcmp(rest, "None") == 0 ? 0 : fail(line_number, "expected None");
            goto mantissa_done;
        }
        if (d == 2.0)
        {
            /* rounding up carried the mantissa to 2 */
            d = 1.0;
            fmpz_add_ui(e, e, 1);
            flint_free(e_s);
            e_s = fmpz_get_str(NULL, 10, e);
        }
        char * inner = strip(rest, "Some((", ')');
        if (inner == NULL || inner[strlen(inner) - 1] != ')')
        {
            result = fail(line_number, "unreadable result");
            goto mantissa_done;
        }
        inner[strlen(inner) - 1] = '\0';
        char * e_str = split(inner, ", ");
        char * o_str = e_str == NULL ? NULL : split(e_str, ", ");
        double got;
        if (o_str == NULL || parse_d(inner, &got))
            result = fail(line_number, "unreadable result");
        else if (got != d || strcmp(e_str, e_s) != 0 || strcmp(o_str, ordering_name(c)) != 0)
            result = fail(line_number, "wrong rounded scientific mantissa");
    }
    else
        result = fail(line_number, "unrecognized method");
mantissa_done:
    flint_free(e_s);
done:
    arf_clear(x);
    arf_clear(z);
    arf_clear(m);
    fmpz_clear(a);
    fmpz_clear(e);
    fmpz_clear(b);
    return result;
}

/* ---------------------------------------------------------------- arf_string */

static int
log_of_base(const char * s)
{
    int b = atoi(s);
    switch (b)
    {
        case 2: return 1;
        case 4: return 2;
        case 8: return 3;
        case 16: return 4;
        case 32: return 5;
        default: return 0;
    }
}

/* Whether the digits of the finite `s`, before any exponent marker, are all in the given case. */
static int
digits_in_case(const char * s, int log_base, int upper)
{
    int base = 1 << log_base;
    for (const char * p = s; *p != '\0'; p++)
    {
        if ((*p == 'E' || *p == 'e') && (base < 15 || p[1] == '+' || p[1] == '-'))
            break;
        if (upper ? islower((unsigned char) *p) : isupper((unsigned char) *p))
        {
            /* the `0b`, `0o`, and `0x` prefixes stay lowercase */
            if ((p == s || (p == s + 1 && s[0] == '-')) || p[-1] != '0')
                return 0;
        }
    }
    return 1;
}

static int
check_string(char * line, int line_number)
{
    arf_t x, y;
    arf_init(x);
    arf_init(y);
    int result = 0;
    if (eat(&line, "BigExponentFloat::from_string_base("))
    {
        /* `b, "s") = Some(X)` or `= None` */
        char * s = split(line, ", \"");
        char * rest = s == NULL ? NULL : split(s, "\") = ");
        int k = log_of_base(line);
        if (rest == NULL || k == 0)
            result = fail(line_number, "unreadable line");
        else if (arf_parse_base(s, k, y))
            result = strcmp(rest, "None") == 0 ? 0 : fail(line_number, "expected None");
        else
        {
            char * inner = strip(rest, "Some(", ')');
            if (inner == NULL || arf_parse_base(inner, 4, x))
                result = fail(line_number, "unreadable result");
            else if (!arf_equal(x, y))
                result = fail(line_number, "wrong value");
        }
    }
    else if (eat(&line, "format!(\"{:#"))
    {
        /* `b}", v) = s` or `o}", v) = s` */
        int k = line[0] == 'b' ? 1 : line[0] == 'o' ? 3 : 0;
        char * v = line + 5;
        char * s = split(v, ") = ");
        if (k == 0 || s == NULL || arf_parse_exact(v, x))
            result = fail(line_number, "unreadable line");
        else if (arf_parse_base(s, k, y) || !arf_equal(x, y))
            result = fail(line_number, "wrong value");
        else if (arf_is_finite(x))
        {
            const char * body = s[0] == '-' ? s + 1 : s;
            if (strncmp(body, k == 1 ? "0b" : "0o", 2) != 0)
                result = fail(line_number, "missing prefix");
        }
    }
    else if (line[0] == '(')
    {
        /* `(v).to_string_base[_upper](b) = s` */
        char * v = line + 1;
        char * method = split(v, ").");
        char * b = method == NULL ? NULL : split(method, "(");
        char * s = b == NULL ? NULL : split(b, ") = ");
        int upper = method != NULL && strcmp(method, "to_string_base_upper") == 0;
        int k = b == NULL ? 0 : log_of_base(b);
        if (s == NULL || k == 0 || (!upper && strcmp(method, "to_string_base") != 0)
            || arf_parse_exact(v, x))
            result = fail(line_number, "unreadable line");
        else if (arf_parse_base(s, k, y) || !arf_equal(x, y))
            result = fail(line_number, "wrong value");
        else if (arf_is_finite(x) && !digits_in_case(s, k, upper))
            result = fail(line_number, "wrong case");
        else if (arf_is_finite(x))
        {
            const char * body = s[0] == '-' ? s + 1 : s;
            if (body[0] == '0' && (body[1] == 'b' || body[1] == 'o' || body[1] == 'x'))
                result = fail(line_number, "unexpected prefix");
        }
    }
    else
        result = fail(line_number, "unrecognized line");
    arf_clear(x);
    arf_clear(y);
    return result;
}

static int
check_line(char * line, int line_number)
{
    size_t n = strlen(line);
    if (n != 0 && line[n - 1] == '\n')
        line[n - 1] = '\0';
    if (line[0] == '\0')
        return 0;
    int result;
    switch (kind)
    {
        case KIND_SET_FMPZ: result = check_set(line, line_number, 0); break;
        case KIND_SET_D: result = check_set(line, line_number, 1); break;
        case KIND_FMPQ: result = check_fmpq(line, line_number); break;
        case KIND_GET_FMPZ: result = check_get_fmpz(line, line_number); break;
        case KIND_GET_D: result = check_get_d(line, line_number); break;
        case KIND_MANTISSA: result = check_mantissa(line, line_number); break;
        default: result = check_string(line, line_number); break;
    }
    if (result == 0)
        checked++;
    return result;
}

static int
run(const char * arg, const char * name, conversion_kind k)
{
    checked = 0;
    kind = k;
    mode_name = name;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_arf_set_fmpz(const char * arg)
{
    return run(arg, "arf_set_fmpz", KIND_SET_FMPZ);
}

int
run_arf_set_d(const char * arg)
{
    return run(arg, "arf_set_d", KIND_SET_D);
}

int
run_arf_fmpq(const char * arg)
{
    return run(arg, "arf_fmpq", KIND_FMPQ);
}

int
run_arf_get_fmpz(const char * arg)
{
    return run(arg, "arf_get_fmpz", KIND_GET_FMPZ);
}

int
run_arf_get_d(const char * arg)
{
    return run(arg, "arf_get_d", KIND_GET_D);
}

int
run_arf_mantissa(const char * arg)
{
    return run(arg, "arf_mantissa", KIND_MANTISSA);
}

int
run_arf_string(const char * arg)
{
    return run(arg, "arf_string", KIND_STRING);
}
