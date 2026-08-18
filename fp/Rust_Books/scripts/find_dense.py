import os
import glob

def analyze_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()
        
    lines = content.split('\n')
    
    in_code = False
    code_lines = 0
    text_lines = 0
    max_continuous_code = 0
    current_continuous_code = 0
    
    for line in lines:
        stripped = line.strip()
        if stripped.startswith('```'):
            in_code = not in_code
            if not in_code:
                if current_continuous_code > max_continuous_code:
                    max_continuous_code = current_continuous_code
                current_continuous_code = 0
            continue
            
        if in_code:
            code_lines += 1
            current_continuous_code += 1
        elif len(stripped) > 0 and not stripped.startswith(('#', '>', '-', '|', '*')):
            text_lines += 1

    ratio = text_lines / (code_lines + 1)
    
    if max_continuous_code > 50 or (code_lines > 100 and ratio < 0.5):
        return True, max_continuous_code, ratio
    return False, max_continuous_code, ratio

import sys

target_dir = sys.argv[1] if len(sys.argv) > 1 else '.'
results = []
for root, _, files in os.walk(target_dir):
    for f in files:
        if f.endswith('.md'):
            path = os.path.join(root, f)
            flagged, max_code, ratio = analyze_file(path)
            if flagged:
                results.append((path, max_code, ratio))

results.sort(key=lambda x: x[1], reverse=True)
for r in results:
    print(f"{r[0]}: Max continuous code block: {r[1]} lines, text/code ratio: {r[2]:.2f}")
