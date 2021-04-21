import os
import sys

def search_replace(filename, old, new):
    with open(filename) as f:
        new_lines = []
        replaced = False
        for line in f.readlines():
            line = line.rstrip()
            if old in line:
                replaced = True
                new_lines.append(line.replace(old, new))
            else:
                new_lines.append(line)
    if replaced:
        with open(filename, 'w') as out_f:
            for line in new_lines:
                out_f.write(line + '\n')
    return replaced

old = sys.argv[1]
new = sys.argv[2]

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
    if search_replace(filename, old, new):
        print(filename)
