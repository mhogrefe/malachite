import os

MAX_LINE_LENGTH = 100

line_length_exceptions = set((
    # long Markdown table rows and/or links
    ('./malachite-base/src/lib.rs', 65),
    ('./malachite-base/src/num/arithmetic/mod.rs', 339),
    ('./malachite-base/src/num/arithmetic/mod.rs', 340),
    ('./malachite-base/src/num/arithmetic/mod.rs', 1340),
    ('./malachite-base/src/num/arithmetic/mod.rs', 1580),
    ('./malachite-base/src/num/arithmetic/mod.rs', 1581),
    ('./malachite-base/src/num/arithmetic/mod.rs', 1582),
    ('./malachite-base/src/num/arithmetic/mod.rs', 1583),
    ('./malachite-base/src/num/arithmetic/primorial.rs', 85),
    ('./malachite-base/src/num/arithmetic/primorial.rs', 244),
    ('./malachite-base/src/num/arithmetic/round_to_multiple_of_power_of_2.rs', 118),
    ('./malachite-base/src/num/conversion/digits/power_of_2_digit_iterable.rs', 153),
    ('./malachite-base/src/num/conversion/digits/power_of_2_digit_iterable.rs', 155),
    ('./malachite-base/src/num/exhaustive/mod.rs', 1074),
    ('./malachite-float/src/conversion/mantissa_and_exponent.rs', 478),
    ('./malachite-float/src/conversion/mantissa_and_exponent.rs', 682),
    ('./malachite-float/src/conversion/mod.rs', 227),
    ('./malachite-float/src/lib.rs', 24),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 41),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 42),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 43),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 78),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 79),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 90),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 91),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 92),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 121),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 123),
    ('./malachite-nz/src/lib.rs', 36),
    ('./malachite-nz/src/lib.rs', 103),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 48),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 49),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 50),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 164),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 165),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 188),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 189),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 190),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 573),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 575),
    ('./malachite-nz/src/natural/conversion/digits/power_of_2_digit_iterable.rs', 526),
    ('./malachite-nz/src/natural/conversion/digits/power_of_2_digit_iterable.rs', 528),
    ('./malachite-nz/src/natural/conversion/digits/power_of_2_digit_iterable.rs', 827),
    ('./malachite-nz/src/natural/conversion/digits/power_of_2_digit_iterable.rs', 829),
    ('./malachite-nz/src/natural/conversion/mantissa_and_exponent.rs', 323),
    ('./malachite-nz/src/natural/conversion/mantissa_and_exponent.rs', 508),
    ('./malachite-nz/src/natural/conversion/mod.rs', 257),
    ('./malachite-q/src/arithmetic/mod.rs', 63),
    ('./malachite-q/src/arithmetic/mod.rs', 64),
    ('./malachite-q/src/arithmetic/mod.rs', 95),
    ('./malachite-q/src/arithmetic/mod.rs', 97),
    ('./malachite-q/src/conversion/string/from_sci_string.rs', 145),
    ('./malachite-q/src/conversion/string/from_sci_string.rs', 232),
    ('./malachite-q/src/lib.rs', 54),
))

import re

FN_NAME_RE = re.compile(r'\bfn\s+([a-z_0-9]+)')
PREC_ROUND_CALL_RE = re.compile(r'\.([a-z_0-9]*_prec_round[a-z_0-9]*)\(')


def redundant_nearest_lint(filename, source):
    # Flags calls like `x.foo_prec_round*(.., Nearest)`, which should use the `foo_prec*`
    # shorthand instead. The defining delegations (the body of `foo_prec*` itself calling
    # `foo_prec_round*(.., Nearest)`) are exempt, as are the `*_rational_*` wrappers and
    # tests/demos that exercise both spellings on purpose.
    if '/tests/' in filename or '/bin_util/' in filename or '/test_util/' in filename:
        return
    lines = source.splitlines()
    current_fn = None
    for i, line in enumerate(lines, 1):
        # Doc comments (including doctests) often demonstrate the explicit-Nearest spelling on
        # purpose.
        if line.lstrip().startswith('//'):
            continue
        fn_match = FN_NAME_RE.search(line)
        if fn_match:
            current_fn = fn_match.group(1)
        for call_match in PREC_ROUND_CALL_RE.finditer(line):
            callee = call_match.group(1)
            shorthand = callee.replace('_prec_round', '_prec', 1)
            if current_fn == shorthand:
                # The shorthand's own definition delegates to the _round variant.
                continue
            (base, _, suffix) = callee.partition('_prec_round')
            if base.endswith('_rational'):
                base = base[:-len('_rational')]
            if '_assign' in suffix:
                base += '_assign'
            if current_fn == base:
                # Operator and assign trait impls delegate using the explicit form by convention.
                continue
            # Scan the argument list, handling nesting and multiple lines, to find the last
            # top-level argument.
            args = ''
            depth = 0
            done = False
            for j in range(i - 1, len(lines)):
                start = call_match.end() if j == i - 1 else 0
                for c in lines[j][start:]:
                    if c == '(':
                        depth += 1
                    elif c == ')':
                        if depth == 0:
                            done = True
                            break
                        depth -= 1
                    args += c
                if done:
                    break
                args += ' '
            last_arg = args.rsplit(',', 1)[-1].strip()
            if last_arg == 'Nearest':
                raise ValueError(
                    f'redundant Nearest: {filename}: {i}: use `{shorthand}` instead of '
                    f'`{callee}(.., Nearest)`'
                )


def lint(filename):
    i = 1
    with open(filename) as f:
        source = f.read()
    redundant_nearest_lint(filename, source)
    for line in source.splitlines():
        line = line.rstrip()
        is_exception = (filename, i) in line_length_exceptions
        if is_exception:
            if len(line) <= MAX_LINE_LENGTH:
                raise ValueError(f'line not too long: {filename}: {i} {line}')
        elif len(line) > MAX_LINE_LENGTH:
            raise ValueError(f'line too long: {filename}: {i} {line}')
        i += 1
    return i - 1

filename_list = []
for root, directories, filenames in os.walk('.'):
    for filename in filenames: 
        filename = os.path.join(root, filename) 
        if '/target/' not in filename and '.history' not in filename and filename.endswith('.rs'):
            filename_list.append(filename)
filename_list.sort()

line_count = 0
for filename in filename_list:
    line_count += lint(filename)
print(f'{line_count} lines checked')
