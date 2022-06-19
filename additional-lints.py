import os

MAX_LINE_LENGTH = 100

#def process_block(block):
#    words = []
#    (padding, block) = block
#    for (line_number, line) in block:
#        words.extend(line[4:].split(' '))
#    fixed_lines = []
#    current_line = ''
#    for i in range(0, padding):
#        current_line += ' '
#    current_line += '///'
#    for word in words:
#        if len(current_line) + len(word) + 1 > MAX_LINE_LENGTH:
#            fixed_lines.append(current_line)
#            current_line = ''
#            for i in range(0, padding):
#                current_line += ' '
#            current_line += '/// '
#            current_line += word
#        else:
#            current_line += ' '
#            current_line += word
#    fixed_lines.append(current_line)
#    print fixed_lines
#
#def lint(filename):
#    i = 1
#    blocks = []
#    current_block = []
#    in_doctest = False
#    previously_in_comment = False
#    with open(filename) as f:
#        for line in f.readlines():
#            line = line.rstrip()
#            line_length = len(line)
#            line = line.lstrip()
#            padding = len(line) - line_length
#            in_comment = line.startswith('///')
#            if in_comment:
#                if line == '/// ```':
#                    in_doctest = not in_doctest
#                elif not in_doctest and line == '///':
#                    if current_block:
#                        blocks.append((padding, current_block))
#                        current_block = []
#                elif not in_doctest:
#                    current_block.append((i, line))
#            if not in_comment and previously_in_comment and current_block:
#                blocks.append((padding, current_block))
#                current_block = []
#            previously_in_comment = in_comment
#            i += 1
#    for block in blocks:
#        process_block(block)
#    return i - 1

line_length_exceptions = set((
    # long Markdown table rows and/or links
    ('./malachite-base/src/lib.rs', 57),
    ('./malachite-base/src/num/arithmetic/mod.rs', 267),
    ('./malachite-base/src/num/arithmetic/mod.rs', 268),
    ('./malachite-base/src/num/arithmetic/mod.rs', 1029),
    ('./malachite-base/src/num/arithmetic/mod.rs', 1269),
    ('./malachite-base/src/num/arithmetic/mod.rs', 1270),
    ('./malachite-base/src/num/arithmetic/mod.rs', 1271),
    ('./malachite-base/src/num/arithmetic/mod.rs', 1272),
    ('./malachite-base/src/num/arithmetic/round_to_multiple_of_power_of_2.rs', 102),
    ('./malachite-base/src/num/conversion/digits/power_of_2_digit_iterable.rs', 156),
    ('./malachite-base/src/num/conversion/digits/power_of_2_digit_iterable.rs', 158),
    ('./malachite-base/src/num/exhaustive/mod.rs', 1031),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 23),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 24),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 25),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 56),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 57),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 68),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 69),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 70),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 100),
    ('./malachite-nz/src/integer/arithmetic/mod.rs', 102),
    ('./malachite-nz/src/lib.rs', 28),
    ('./malachite-nz/src/lib.rs', 94),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 30),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 31),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 32),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 117),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 118),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 141),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 142),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 143),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 521),
    ('./malachite-nz/src/natural/arithmetic/mod.rs', 523),
    ('./malachite-nz/src/natural/conversion/digits/power_of_2_digit_iterable.rs', 540),
    ('./malachite-nz/src/natural/conversion/digits/power_of_2_digit_iterable.rs', 542),
    ('./malachite-nz/src/natural/conversion/digits/power_of_2_digit_iterable.rs', 854),
    ('./malachite-nz/src/natural/conversion/digits/power_of_2_digit_iterable.rs', 856),
    ('./malachite-nz/src/natural/conversion/mantissa_and_exponent.rs', 331),
    ('./malachite-nz/src/natural/conversion/mantissa_and_exponent.rs', 486),
    ('./malachite-nz/src/natural/conversion/mantissa_and_exponent.rs', 516),
    ('./malachite-nz/src/natural/conversion/mod.rs', 189),
    ('./malachite-q/src/arithmetic/mod.rs', 70),
    ('./malachite-q/src/arithmetic/mod.rs', 72),
    ('./malachite-q/src/conversion/mantissa_and_exponent.rs', 187),
    ('./malachite-q/src/conversion/mantissa_and_exponent.rs', 213),
    ('./malachite-q/src/conversion/mantissa_and_exponent.rs', 290),
    ('./malachite-q/src/conversion/mantissa_and_exponent.rs', 316),
    ('./malachite-q/src/conversion/string/from_sci_string.rs', 14),
    ('./malachite-q/src/conversion/string/from_sci_string.rs', 83),
    ('./malachite-q/src/conversion/string/from_sci_string.rs', 122),
    ('./malachite-q/src/conversion/string/from_sci_string.rs', 207),
    ('./malachite-q/src/exhaustive/mod.rs', 36),
    ('./malachite-q/src/exhaustive/mod.rs', 38),
    ('./malachite-q/src/lib.rs', 47),
))

def lint(filename):
    i = 1
    with open(filename) as f:
        for line in f.readlines():
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
        if '/target/' not in filename and filename.endswith('.rs'):
            filename_list.append(filename)
for root, directories, filenames in os.walk('../rust-wheels'):
    for filename in filenames: 
        filename = os.path.join(root, filename) 
        if '/target/' not in filename and filename.endswith('.rs'):
            filename_list.append(filename)
filename_list.sort()

line_count = 0
for filename in filename_list:
    line_count += lint(filename)
print(f'{line_count} lines checked')
