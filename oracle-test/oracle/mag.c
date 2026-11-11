/*
    Copyright © 2026 Mikhail Hogrefe

    This file is part of Malachite.

    Malachite is free software: you can redistribute it and/or modify it under the terms of the
    GNU Lesser General Public License (LGPL) as published by the Free Software Foundation; either
    version 3 of the License, or (at your option) any later version. See
    <https://www.gnu.org/licenses/>.

    Diffs the operations of Malachite's internal `Mag` type against FLINT's `mag_t`. A `Mag` is
    written as its exact value, `0`, `m`, or `m*2^k` with `m` odd, which is parsed exactly into a
    `mag_t` (FLINT's mantissas also have 30 bits). Each mode accepts one line format:

    - `mag_set_ui`: `Mag::from(x) = r`, for a `u64` `x`
    - `mag_set_ui_lower`: `Mag::from_u64_lower(x) = r`
    - `mag_set_fmpz`: `Mag::from(&x) = r`, for a `Natural` `x`
    - `mag_get_fmpz`: `(x).ceiling() = n`
    - `mag_cmp`: `(x).cmp(&y) = o`, `(x).max(y) = r`, and `(x).min(y) = r`, against `mag_cmp`,
      `mag_max`, and `mag_min`
    - `mag_mul`: `Mag::mul(x, y) = r`, against both `mag_mul` and `mag_fast_mul`
    - `mag_mul_ui`: `(x).mul_u64(y) = r`
    - `mag_fast_addmul`: `z := z0; z.add_mul_assign(x, y); z = r`
    - `mag_div_ui`: `(x).div_u64(y) = r`
    - `mag_sqrt`: `(x).sqrt() = r`

    Any other nonempty line is an error, as is an input with no lines at all.
*/

#include <errno.h>
#include <stdlib.h>
#include <string.h>

#include <flint/fmpz.h>
#include <flint/longlong.h>
#include <flint/mag.h>

#include "oracle.h"

typedef enum
{
    KIND_SET_UI,
    KIND_SET_UI_LOWER,
    KIND_SET_FMPZ,
    KIND_GET_FMPZ,
    KIND_CMP,
    KIND_MUL,
    KIND_MUL_UI,
    KIND_FAST_ADDMUL,
    KIND_DIV_UI,
    KIND_SQRT
} mag_kind;

static long checked;
static mag_kind kind;
static const char * mode_name;

/* Parses a decimal `u64`, which must be the whole of `s`. Returns 0 on success. */
static int
parse_u64(const char * s, ulong * out)
{
    if (*s < '0' || *s > '9')
    {
        return 1;
    }
    char * end;
    errno = 0;
    unsigned long long x = strtoull(s, &end, 10);
    if (errno != 0 || *end != '\0')
    {
        return 1;
    }
    *out = (ulong) x;
    return 0;
}

/* Parses Malachite's `Mag` format into `z`, exactly. Returns 0 on success. */
static int
parse_mag(const char * s, mag_t z)
{
    char buf[128];
    if (strlen(s) >= sizeof(buf))
    {
        return 1;
    }
    strcpy(buf, s);
    slong k = 0;
    char * star = strstr(buf, "*2^");
    if (star != NULL)
    {
        *star = '\0';
        char * end;
        errno = 0;
        long long kk = strtoll(star + 3, &end, 10);
        if (errno != 0 || *end != '\0' || end == star + 3)
        {
            return 1;
        }
        k = (slong) kk;
    }
    ulong m;
    if (parse_u64(buf, &m) != 0)
    {
        return 1;
    }
    if (m == 0)
    {
        if (k != 0)
        {
            return 1;
        }
        mag_zero(z);
        return 0;
    }
    int zeros = flint_ctz(m);
    ulong odd = m >> zeros;
    if (FLINT_BIT_COUNT(odd) > MAG_BITS)
    {
        return 1;
    }
    /* `mag_set_ui` is exact for at most `MAG_BITS` bits */
    mag_set_ui_2exp_si(z, odd, k + zeros);
    return 0;
}

/* Writes `z` in Malachite's `Mag` format. */
static void
mag_to_str(char * buf, size_t size, const mag_t z)
{
    if (mag_is_zero(z))
    {
        snprintf(buf, size, "0");
        return;
    }
    ulong man = MAG_MAN(z);
    slong exp = fmpz_get_si(MAG_EXPREF(z));
    int zeros = flint_ctz(man);
    ulong odd = man >> zeros;
    slong pow = exp - MAG_BITS + zeros;
    if (pow == 0)
    {
        snprintf(buf, size, "%lu", (unsigned long) odd);
    }
    else
    {
        snprintf(buf, size, "%lu*2^%ld", (unsigned long) odd, (long) pow);
    }
}

/* Splits `line` at the literals `lits[0..n]` in order, in place: `line` must begin with `lits[0]`,
   and `holes[i]` receives the text between `lits[i]` and `lits[i + 1]`, or, for the last, the rest
   of the line. Returns 0 on success. */
static int
split_at(char * line, const char * const * lits, int n, char ** holes)
{
    size_t len = strlen(lits[0]);
    if (strncmp(line, lits[0], len) != 0)
    {
        return 1;
    }
    char * p = line + len;
    for (int i = 1; i < n; i++)
    {
        char * q = strstr(p, lits[i]);
        if (q == NULL)
        {
            return 1;
        }
        *q = '\0';
        holes[i - 1] = p;
        p = q + strlen(lits[i]);
    }
    holes[n - 1] = p;
    return 0;
}

static int
report(int line_number, const char * what, const char * flint, const char * expected)
{
    flint_printf("error in %s test, line %d: %s. FLINT: %s, Malachite: %s\n", mode_name,
                 line_number, what, flint, expected);
    return 1;
}

static int
unreadable(int line_number)
{
    flint_printf("error in %s test, line %d: unreadable line\n", mode_name, line_number);
    return 1;
}

/* Compares the `mag_t` `z` with Malachite's `expected`. */
static int
check_mag(int line_number, const char * what, const mag_t z, const char * expected)
{
    char buf[128];
    mag_to_str(buf, sizeof(buf), z);
    return strcmp(buf, expected) == 0 ? 0 : report(line_number, what, buf, expected);
}

static int
check_line(char * line, int line_number)
{
    size_t n = strlen(line);
    if (n != 0 && line[n - 1] == '\n')
    {
        line[n - 1] = '\0';
    }
    if (line[0] == '\0')
    {
        return 0;
    }
    char * h[4];
    int result = 0;
    mag_t x, y, z;
    mag_init(x);
    mag_init(y);
    mag_init(z);
    fmpz_t a, b;
    fmpz_init(a);
    fmpz_init(b);
    ulong u;
    switch (kind)
    {
        case KIND_SET_UI:
        {
            static const char * const lits[] = {"Mag::from(", ") = "};
            if (split_at(line, lits, 2, h) || h[0][0] == '&' || parse_u64(h[0], &u))
            {
                result = unreadable(line_number);
                break;
            }
            mag_set_ui(z, u);
            result = check_mag(line_number, "mag_set_ui", z, h[1]);
            break;
        }
        case KIND_SET_UI_LOWER:
        {
            static const char * const lits[] = {"Mag::from_u64_lower(", ") = "};
            if (split_at(line, lits, 2, h) || parse_u64(h[0], &u))
            {
                result = unreadable(line_number);
                break;
            }
            mag_set_ui_lower(z, u);
            result = check_mag(line_number, "mag_set_ui_lower", z, h[1]);
            break;
        }
        case KIND_SET_FMPZ:
        {
            static const char * const lits[] = {"Mag::from(&", ") = "};
            if (split_at(line, lits, 2, h) || h[0][0] < '0' || h[0][0] > '9'
                || fmpz_set_str(a, h[0], 10) != 0)
            {
                result = unreadable(line_number);
                break;
            }
            mag_set_fmpz(z, a);
            result = check_mag(line_number, "mag_set_fmpz", z, h[1]);
            break;
        }
        case KIND_GET_FMPZ:
        {
            static const char * const lits[] = {"(", ").ceiling() = "};
            if (split_at(line, lits, 2, h) || parse_mag(h[0], x) || fmpz_set_str(b, h[1], 10) != 0)
            {
                result = unreadable(line_number);
                break;
            }
            mag_get_fmpz(a, x);
            if (!fmpz_equal(a, b))
            {
                char * s = fmpz_get_str(NULL, 10, a);
                result = report(line_number, "mag_get_fmpz", s, h[1]);
                flint_free(s);
            }
            break;
        }
        case KIND_CMP:
        {
            static const char * const cmp_lits[] = {"(", ").cmp(&", ") = "};
            static const char * const max_lits[] = {"(", ").max(", ") = "};
            static const char * const min_lits[] = {"(", ").min(", ") = "};
            char * copy = strdup(line);
            if (split_at(line, cmp_lits, 3, h) == 0)
            {
                if (parse_mag(h[0], x) || parse_mag(h[1], y))
                {
                    result = unreadable(line_number);
                }
                else
                {
                    int c = mag_cmp(x, y);
                    const char * o = c < 0 ? "Less" : c > 0 ? "Greater" : "Equal";
                    if (strcmp(o, h[2]) != 0)
                    {
                        result = report(line_number, "mag_cmp", o, h[2]);
                    }
                }
            }
            else
            {
                strcpy(line, copy);
                int is_max = split_at(line, max_lits, 3, h) == 0;
                if (!is_max)
                {
                    strcpy(line, copy);
                }
                if ((!is_max && split_at(line, min_lits, 3, h) != 0) || parse_mag(h[0], x)
                    || parse_mag(h[1], y))
                {
                    result = unreadable(line_number);
                }
                else if (is_max)
                {
                    mag_max(z, x, y);
                    result = check_mag(line_number, "mag_max", z, h[2]);
                }
                else
                {
                    mag_min(z, x, y);
                    result = check_mag(line_number, "mag_min", z, h[2]);
                }
            }
            free(copy);
            break;
        }
        case KIND_MUL:
        {
            static const char * const lits[] = {"Mag::mul(", ", ", ") = "};
            if (split_at(line, lits, 3, h) || parse_mag(h[0], x) || parse_mag(h[1], y))
            {
                result = unreadable(line_number);
                break;
            }
            mag_mul(z, x, y);
            result = check_mag(line_number, "mag_mul", z, h[2]);
            if (result == 0)
            {
                mag_fast_mul(z, x, y);
                result = check_mag(line_number, "mag_fast_mul", z, h[2]);
            }
            break;
        }
        case KIND_MUL_UI:
        {
            static const char * const lits[] = {"(", ").mul_u64(", ") = "};
            if (split_at(line, lits, 3, h) || parse_mag(h[0], x) || parse_u64(h[1], &u))
            {
                result = unreadable(line_number);
                break;
            }
            mag_mul_ui(z, x, u);
            result = check_mag(line_number, "mag_mul_ui", z, h[2]);
            break;
        }
        case KIND_FAST_ADDMUL:
        {
            static const char * const lits[] = {"z := ", "; z.add_mul_assign(", ", ",
                                                "); z = "};
            if (split_at(line, lits, 4, h) || parse_mag(h[0], z) || parse_mag(h[1], x)
                || parse_mag(h[2], y))
            {
                result = unreadable(line_number);
                break;
            }
            mag_fast_addmul(z, x, y);
            result = check_mag(line_number, "mag_fast_addmul", z, h[3]);
            break;
        }
        case KIND_DIV_UI:
        {
            static const char * const lits[] = {"(", ").div_u64(", ") = "};
            if (split_at(line, lits, 3, h) || parse_mag(h[0], x) || parse_u64(h[1], &u)
                || u == 0)
            {
                result = unreadable(line_number);
                break;
            }
            mag_div_ui(z, x, u);
            result = check_mag(line_number, "mag_div_ui", z, h[2]);
            break;
        }
        case KIND_SQRT:
        {
            static const char * const lits[] = {"(", ").sqrt() = "};
            if (split_at(line, lits, 2, h) || parse_mag(h[0], x))
            {
                result = unreadable(line_number);
                break;
            }
            mag_sqrt(z, x);
            result = check_mag(line_number, "mag_sqrt", z, h[1]);
            break;
        }
    }
    if (result == 0)
    {
        checked++;
    }
    mag_clear(x);
    mag_clear(y);
    mag_clear(z);
    fmpz_clear(a);
    fmpz_clear(b);
    return result;
}

static int
run(const char * arg, const char * name, mag_kind k)
{
    checked = 0;
    kind = k;
    mode_name = name;
    int result = for_each_line(arg, check_line);
    return result != 0 ? result : require_some_lines(name, checked);
}

int
run_mag_set_ui(const char * arg)
{
    return run(arg, "mag_set_ui", KIND_SET_UI);
}

int
run_mag_set_ui_lower(const char * arg)
{
    return run(arg, "mag_set_ui_lower", KIND_SET_UI_LOWER);
}

int
run_mag_set_fmpz(const char * arg)
{
    return run(arg, "mag_set_fmpz", KIND_SET_FMPZ);
}

int
run_mag_get_fmpz(const char * arg)
{
    return run(arg, "mag_get_fmpz", KIND_GET_FMPZ);
}

int
run_mag_cmp(const char * arg)
{
    return run(arg, "mag_cmp", KIND_CMP);
}

int
run_mag_mul(const char * arg)
{
    return run(arg, "mag_mul", KIND_MUL);
}

int
run_mag_mul_ui(const char * arg)
{
    return run(arg, "mag_mul_ui", KIND_MUL_UI);
}

int
run_mag_fast_addmul(const char * arg)
{
    return run(arg, "mag_fast_addmul", KIND_FAST_ADDMUL);
}

int
run_mag_div_ui(const char * arg)
{
    return run(arg, "mag_div_ui", KIND_DIV_UI);
}

int
run_mag_sqrt(const char * arg)
{
    return run(arg, "mag_sqrt", KIND_SQRT);
}
