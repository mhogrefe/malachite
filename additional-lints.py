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

def lint(filename):
    i = 1
    with open(filename) as f:
        for line in f.readlines():
            line = line.rstrip()
            if len(line) > MAX_LINE_LENGTH:
                raise ValueError('line too long: ' + filename + ': ' + str(i) + ' ' + line)
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
