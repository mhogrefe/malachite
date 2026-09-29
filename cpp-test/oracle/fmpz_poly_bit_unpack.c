/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the lines printed by the NaturalPolynomial and IntegerPolynomial bit_unpack demos,
    `T::bit_unpack(N, b) = P` and `T::bit_unpack(&N, b) = P`, against
    fmpz_poly_bit_unpack_unsigned (with `T` NaturalPolynomial) and fmpz_poly_bit_unpack (with `T`
    IntegerPolynomial). Malachite panics when b is 0, so no line has b = 0; one that did would be
    an error here. Each mode treats any other nonempty line as an error, and an input with no lines
    at all.
*/

#include <stdlib.h>
#include <string.h>

#include <flint/fmpz_poly.h>

#include "oracle.h"

static long checked;
static const char * current_name;
static const char * current_prefix;
static void (* current_function)(fmpz_poly_t, const fmpz_t, flint_bitcnt_t);

/* Splits `PREFIX N, b) = P`, with an optional `&` before `N`. */
static int
split_line(char * line, char ** n_str, char ** bits_str, char ** p_str)
{
    line[strcspn(line, "\r\n")] = '\0';
    size_t prefix_len = strlen(current_prefix);
    if (strncmp(line, current_prefix, prefix_len) != 0)
    {
        return 0;
    }
    char * rest = line + prefix_len;
    if (*rest == '&')
    {
        rest++;
    }
    char * comma = strstr(rest, ", ");
    char * close = strstr(rest, ") = ");
    if (comma == NULL || close == NULL || comma > close)
    {
        return 0;
    }
    *comma = '\0';
    *close = '\0';
    *n_str = rest;
    *bits_str = comma + 2;
    *p_str = close + 4;
    return 1;
}

static int
check_line(char * line, int line_number)
{
    char * n_str;
    char * bits_str;
    char * p_str;
    if (!split_line(line, &n_str, &bits_str, &p_str))
    {
        if (line[0] == '\0')
        {
            return 0;
        }
        flint_printf("error in %s test, line %d: unrecognized line\n", current_name,
                     line_number);
        return 1;
    }
    checked++;
    int result = 0;
    char * end;
    unsigned long long bits = strtoull(bits_str, &end, 10);
    fmpz_t n;
    fmpz_poly_t expected, p;
    fmpz_init(n);
    fmpz_poly_init(expected);
    fmpz_poly_init(p);
    if (*end != '\0' || bits == 0 || bits > WORD_MAX || fmpz_set_str(n, n_str, 10) != 0
        || !fmpz_poly_set_str_malachite(expected, p_str))
    {
        flint_printf("error in %s test, line %d: unreadable input\n", current_name, line_number);
        result = 1;
    }
    else
    {
        current_function(p, n, (flint_bitcnt_t) bits);
        if (!fmpz_poly_equal(p, expected))
        {
            flint_printf("error in %s test, line %d. FLINT: ", current_name, line_number);
            fmpz_poly_print_pretty(p, "x");
            flint_printf("\n");
            result = 1;
        }
    }
    fmpz_clear(n);
    fmpz_poly_clear(expected);
    fmpz_poly_clear(p);
    return result;
}

static int
run(const char * arg, const char * name, const char * prefix,
    void (* function)(fmpz_poly_t, const fmpz_t, flint_bitcnt_t))
{
    checked = 0;
    current_name = name;
    current_prefix = prefix;
    current_function = function;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_fmpz_poly_bit_unpack(const char * arg)
{
    return run(arg, "fmpz_poly_bit_unpack", "IntegerPolynomial::bit_unpack(",
               fmpz_poly_bit_unpack);
}

int
run_fmpz_poly_bit_unpack_unsigned(const char * arg)
{
    return run(arg, "fmpz_poly_bit_unpack_unsigned", "NaturalPolynomial::bit_unpack(",
               fmpz_poly_bit_unpack_unsigned);
}
