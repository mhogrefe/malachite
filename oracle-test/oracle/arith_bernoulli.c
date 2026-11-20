/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the Bernoulli and Euler numbers and polynomials against FLINT's arith module. Each mode
    accepts lines of one form:

    - arith_bernoulli_number: `bernoulli_number(n) = q`, against arith_bernoulli_number;
    - arith_bernoulli_number_vec: `bernoulli_numbers_prefix(n) = [q, ...]`, against
      arith_bernoulli_number_vec, and `even_bernoulli_numbers_prefix(m) = [q, ...]`, against every
      other entry of arith_bernoulli_number_vec(2m + 1) from B_2 on;
    - arith_bernoulli_number_denom: `bernoulli_number_denominator(n) = d`, against
      arith_bernoulli_number_denom;
    - arith_bernoulli_polynomial: `bernoulli_polynomial(n) = P`, against
      arith_bernoulli_polynomial;
    - arith_euler_number: `euler_number(n) = e`, against arith_euler_number;
    - arith_euler_number_vec: `euler_numbers_prefix(n) = [e, ...]`, against arith_euler_number_vec;
    - arith_euler_polynomial: `euler_polynomial(n) = P`, against arith_euler_polynomial;

    where q is a Rational written as `a/b` or `a`, d and e are integers, and P is a polynomial in
    Malachite's format. Any other nonempty line is an error, as is an input with no lines at all.
*/

#include <stdlib.h>
#include <string.h>

#include <flint/arith.h>
#include <flint/fmpq.h>
#include <flint/fmpq_poly.h>
#include <flint/fmpq_vec.h>
#include <flint/fmpz.h>
#include <flint/fmpz_vec.h>

#include "oracle.h"

typedef enum
{
    BERNOULLI_NUMBER,
    BERNOULLI_VEC,
    BERNOULLI_DENOM,
    BERNOULLI_POLYNOMIAL,
    EULER_NUMBER,
    EULER_VEC,
    EULER_POLYNOMIAL,
} kind_t;

static kind_t mode;
static long checked;

static int
report(int line_number, const char * what)
{
    flint_printf("line %d: %s\n", line_number, what);
    return 1;
}

/* If `line` is `<name>(n) = <rest>`, sets *n and *rest and returns 1. */
static int
match(char * line, const char * name, ulong * n, char ** rest)
{
    size_t l = strlen(name);
    if (strncmp(line, name, l) != 0 || line[l] != '(')
        return 0;
    char * end = NULL;
    *n = strtoul(line + l + 1, &end, 10);
    if (end == line + l + 1 || strncmp(end, ") = ", 4) != 0)
        return 0;
    *rest = end + 4;
    return 1;
}

/* Parses `[x, y, ...]` into `len` entries, each passed to `entry` with its index; returns 0 if the
   text does not have exactly `len` entries or `entry` fails for one. */
static int
for_each_entry(char * s, slong len, int (* entry)(const char * text, slong i, void * data),
               void * data)
{
    size_t l = strlen(s);
    if (l < 2 || s[0] != '[' || s[l - 1] != ']')
        return 0;
    s[l - 1] = '\0';
    s++;
    if (*s == '\0')
        return len == 0;
    slong i = 0;
    for (char * p = s;; i++)
    {
        char * next = strstr(p, ", ");
        if (next != NULL)
            *next = '\0';
        if (i >= len || !entry(p, i, data))
            return 0;
        if (next == NULL)
            break;
        p = next + 2;
    }
    return i + 1 == len;
}

static int
fmpq_entry(const char * text, slong i, void * data)
{
    fmpq_t q;
    fmpq_init(q);
    int ok = fmpq_set_str(q, text, 10) == 0 && fmpq_equal(q, (fmpq *) data + i);
    fmpq_clear(q);
    return ok;
}

static int
fmpz_entry(const char * text, slong i, void * data)
{
    fmpz_t z;
    fmpz_init(z);
    int ok = fmpz_set_str(z, text, 10) == 0 && fmpz_equal(z, (fmpz *) data + i);
    fmpz_clear(z);
    return ok;
}

static int
check_rational(const char * text, const fmpq_t expected)
{
    fmpq_t q;
    fmpq_init(q);
    int ok = fmpq_set_str(q, text, 10) == 0 && fmpq_equal(q, expected);
    fmpq_clear(q);
    return ok;
}

static int
check_integer(const char * text, const fmpz_t expected)
{
    fmpz_t z;
    fmpz_init(z);
    int ok = fmpz_set_str(z, text, 10) == 0 && fmpz_equal(z, expected);
    fmpz_clear(z);
    return ok;
}

static int
check_polynomial(const char * text, const fmpq_poly_t expected)
{
    fmpq_poly_t p;
    fmpq_poly_init(p);
    int ok = fmpq_poly_set_str_malachite(p, text) && fmpq_poly_equal(p, expected);
    fmpq_poly_clear(p);
    return ok;
}

static int
check_line(char * line, int line_number)
{
    size_t len = strlen(line);
    if (len != 0 && line[len - 1] == '\n')
        line[len - 1] = '\0';
    if (line[0] == '\0')
        return 0;
    ulong n;
    char * rest;
    int ok;
    if (mode == BERNOULLI_NUMBER && match(line, "bernoulli_number", &n, &rest))
    {
        fmpq_t b;
        fmpq_init(b);
        arith_bernoulli_number(b, n);
        ok = check_rational(rest, b);
        fmpq_clear(b);
    }
    else if (mode == BERNOULLI_VEC && match(line, "bernoulli_numbers_prefix", &n, &rest))
    {
        fmpq * v = _fmpq_vec_init(n);
        arith_bernoulli_number_vec(v, n);
        ok = for_each_entry(rest, n, fmpq_entry, v);
        _fmpq_vec_clear(v, n);
    }
    else if (mode == BERNOULLI_VEC && match(line, "even_bernoulli_numbers_prefix", &n, &rest))
    {
        slong full = 2 * n + 1;
        fmpq * v = _fmpq_vec_init(full);
        fmpq * evens = _fmpq_vec_init(n);
        arith_bernoulli_number_vec(v, full);
        for (ulong k = 0; k < n; k++)
            fmpq_set(evens + k, v + 2 * (k + 1));
        ok = for_each_entry(rest, n, fmpq_entry, evens);
        _fmpq_vec_clear(evens, n);
        _fmpq_vec_clear(v, full);
    }
    else if (mode == BERNOULLI_DENOM && match(line, "bernoulli_number_denominator", &n, &rest))
    {
        fmpz_t d;
        fmpz_init(d);
        arith_bernoulli_number_denom(d, n);
        ok = check_integer(rest, d);
        fmpz_clear(d);
    }
    else if (mode == BERNOULLI_POLYNOMIAL && match(line, "bernoulli_polynomial", &n, &rest))
    {
        fmpq_poly_t p;
        fmpq_poly_init(p);
        arith_bernoulli_polynomial(p, n);
        ok = check_polynomial(rest, p);
        fmpq_poly_clear(p);
    }
    else if (mode == EULER_NUMBER && match(line, "euler_number", &n, &rest))
    {
        fmpz_t e;
        fmpz_init(e);
        arith_euler_number(e, n);
        ok = check_integer(rest, e);
        fmpz_clear(e);
    }
    else if (mode == EULER_VEC && match(line, "euler_numbers_prefix", &n, &rest))
    {
        fmpz * v = _fmpz_vec_init(n);
        arith_euler_number_vec(v, n);
        ok = for_each_entry(rest, n, fmpz_entry, v);
        _fmpz_vec_clear(v, n);
    }
    else if (mode == EULER_POLYNOMIAL && match(line, "euler_polynomial", &n, &rest))
    {
        fmpq_poly_t p;
        fmpq_poly_init(p);
        arith_euler_polynomial(p, n);
        ok = check_polynomial(rest, p);
        fmpq_poly_clear(p);
    }
    else
        return report(line_number, "unrecognized line");
    if (!ok)
        return report(line_number, "mismatch");
    checked++;
    return 0;
}

static int
run(const char * arg, kind_t k, const char * name)
{
    mode = k;
    checked = 0;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_arith_bernoulli_number(const char * arg)
{
    return run(arg, BERNOULLI_NUMBER, "arith_bernoulli_number");
}

int
run_arith_bernoulli_number_vec(const char * arg)
{
    return run(arg, BERNOULLI_VEC, "arith_bernoulli_number_vec");
}

int
run_arith_bernoulli_number_denom(const char * arg)
{
    return run(arg, BERNOULLI_DENOM, "arith_bernoulli_number_denom");
}

int
run_arith_bernoulli_polynomial(const char * arg)
{
    return run(arg, BERNOULLI_POLYNOMIAL, "arith_bernoulli_polynomial");
}

int
run_arith_euler_number(const char * arg)
{
    return run(arg, EULER_NUMBER, "arith_euler_number");
}

int
run_arith_euler_number_vec(const char * arg)
{
    return run(arg, EULER_VEC, "arith_euler_number_vec");
}

int
run_arith_euler_polynomial(const char * arg)
{
    return run(arg, EULER_POLYNOMIAL, "arith_euler_polynomial");
}
