/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs `n is a probable prime` and `n is not a probable prime` lines against
    fmpz_is_probabprime, and `n is a strong probable prime to base a` and `n is not a strong
    probable prime to base a` lines against fmpz_is_strong_probabprime. Both modes are strict: a
    nonempty line of any other shape is an error, and so is an input in which no line was checked.
*/

#include <string.h>

#include <flint/fmpz.h>

#include "oracle.h"

static long probabprime_checked = 0;

static void
chomp(char * line)
{
    size_t len = strlen(line);
    while (len > 0 && (line[len - 1] == '\n' || line[len - 1] == '\r'))
    {
        line[--len] = '\0';
    }
}

/* If `line` ends with `suffix`, cuts the suffix off and returns 1. */
static int
cut_suffix(char * line, const char * suffix)
{
    size_t len = strlen(line), slen = strlen(suffix);
    if (len < slen || strcmp(line + len - slen, suffix) != 0)
    {
        return 0;
    }
    line[len - slen] = '\0';
    return 1;
}

/* Reads a nonnegative decimal integer that is the whole of `s`. */
static int
read_natural(fmpz_t x, const char * s)
{
    if (s[0] == '\0' || s[0] == '-')
    {
        return 0;
    }
    for (const char * p = s; *p; p++)
    {
        if (*p < '0' || *p > '9')
        {
            return 0;
        }
    }
    return fmpz_set_str(x, s, 10) == 0;
}

static int
check_fmpz_is_probabprime_line(char * line, int line_number)
{
    chomp(line);
    if (line[0] == '\0')
    {
        return 0;
    }
    int expected;
    if (cut_suffix(line, " is not a probable prime"))
    {
        expected = 0;
    }
    else if (cut_suffix(line, " is a probable prime"))
    {
        expected = 1;
    }
    else
    {
        flint_printf("error in fmpz_is_probabprime test, line %d: unrecognized line\n  %s\n",
            line_number, line);
        return 1;
    }
    fmpz_t n;
    fmpz_init(n);
    int result = 0;
    if (!read_natural(n, line))
    {
        flint_printf("error in fmpz_is_probabprime test, line %d: unreadable n\n  %s\n",
            line_number, line);
        result = 1;
    }
    else
    {
        int flint_output = fmpz_is_probabprime(n);
        probabprime_checked += 1;
        if (flint_output != expected)
        {
            flint_printf("error in fmpz_is_probabprime test, line %d, on input %s. FLINT: %d, "
                "Malachite: %d\n", line_number, line, flint_output, expected);
            result = 1;
        }
    }
    fmpz_clear(n);
    return result;
}

int
run_fmpz_is_probabprime(const char * arg)
{
    probabprime_checked = 0;
    int result = for_each_line(arg, check_fmpz_is_probabprime_line);
    if (result != 0)
    {
        return result;
    }
    if (require_some_lines("fmpz_is_probabprime", probabprime_checked))
    {
        return 1;
    }
    flint_printf("fmpz_is_probabprime: %ld lines\n", probabprime_checked);
    return 0;
}

static int
check_fmpz_is_strong_probabprime_line(char * line, int line_number)
{
    chomp(line);
    if (line[0] == '\0')
    {
        return 0;
    }
    int expected;
    const char * negative = " is not a strong probable prime to base ";
    const char * positive = " is a strong probable prime to base ";
    char * base;
    char * mid;
    if ((mid = strstr(line, negative)) != NULL)
    {
        expected = 0;
        base = mid + strlen(negative);
    }
    else if ((mid = strstr(line, positive)) != NULL)
    {
        expected = 1;
        base = mid + strlen(positive);
    }
    else
    {
        flint_printf("error in fmpz_is_strong_probabprime test, line %d: unrecognized line\n"
            "  %s\n", line_number, line);
        return 1;
    }
    *mid = '\0';
    fmpz_t n, a;
    fmpz_init(n);
    fmpz_init(a);
    int result = 0;
    if (!read_natural(n, line) || !read_natural(a, base))
    {
        flint_printf("error in fmpz_is_strong_probabprime test, line %d: unreadable input\n",
            line_number);
        result = 1;
    }
    else
    {
        int flint_output = fmpz_is_strong_probabprime(n, a);
        probabprime_checked += 1;
        if (flint_output != expected)
        {
            flint_printf("error in fmpz_is_strong_probabprime test, line %d, on input (%s, %s). "
                "FLINT: %d, Malachite: %d\n", line_number, line, base, flint_output, expected);
            result = 1;
        }
    }
    fmpz_clear(n);
    fmpz_clear(a);
    return result;
}

int
run_fmpz_is_strong_probabprime(const char * arg)
{
    probabprime_checked = 0;
    int result = for_each_line(arg, check_fmpz_is_strong_probabprime_line);
    if (result != 0)
    {
        return result;
    }
    if (require_some_lines("fmpz_is_strong_probabprime", probabprime_checked))
    {
        return 1;
    }
    flint_printf("fmpz_is_strong_probabprime: %ld lines\n", probabprime_checked);
    return 0;
}
