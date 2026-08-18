import os
import re
import sys

def process_file(filepath):
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()

    lines = content.split('\n')
    new_lines = []
    
    in_code_block = False
    current_block_lines = []
    code_lang = ""
    
    for line in lines:
        if line.startswith('```'):
            if not in_code_block:
                in_code_block = True
                code_lang = line.strip()
                current_block_lines.append(line)
            else:
                in_code_block = False
                current_block_lines.append(line)
                
                # Check if block is > 80 lines
                if len(current_block_lines) > 80:
                    # Try to chunk it
                    chunked_block = []
                    split_happened = False
                    
                    # 1. First pass: Try to find top level comments
                    for i, bline in enumerate(current_block_lines):
                        if i > 25 and i < len(current_block_lines) - 25:
                            if bline.startswith('# ──') or bline.startswith('# ---') or bline.startswith('# Bước') or bline.startswith('# Level') or bline.startswith('# Section') or bline.startswith('// ──') or bline.startswith('// ---') or bline.startswith('// Bước') or bline.startswith('// Level') or bline.startswith('// Section'):
                                chunked_block.append('```\n')
                                clean_comment = bline.strip('#/- ─').strip()
                                chunked_block.append(f'#### {clean_comment}\n')
                                chunked_block.append(f'{code_lang}')
                                chunked_block.append(bline)
                                split_happened = True
                                continue
                        chunked_block.append(bline)
                        
                    # 2. If no semantic comment was found, split at middle blank line
                    if not split_happened and len(current_block_lines) > 80:
                        chunked_block = []
                        mid_point = len(current_block_lines) // 2
                        
                        # find nearest blank line to mid_point
                        best_split_idx = mid_point
                        for offset in range(0, mid_point):
                            if current_block_lines[mid_point + offset].strip() == '':
                                best_split_idx = mid_point + offset
                                break
                            if current_block_lines[mid_point - offset].strip() == '':
                                best_split_idx = mid_point - offset
                                break
                                
                        for i, bline in enumerate(current_block_lines):
                            if i == best_split_idx:
                                chunked_block.append('```\n')
                                chunked_block.append(f'#### Tiếp tục phân tích...\n')
                                chunked_block.append(f'{code_lang}')
                            else:
                                chunked_block.append(bline)
                                
                    new_lines.extend(chunked_block)
                else:
                    new_lines.extend(current_block_lines)
                
                current_block_lines = []
                code_lang = ""
        else:
            if in_code_block:
                current_block_lines.append(line)
            else:
                new_lines.append(line)

    if in_code_block:
        new_lines.extend(current_block_lines)

    new_content = '\n'.join(new_lines)
    if new_content != content:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(new_content)
        print(f"Chunked: {filepath}")

def main():
    target_dir = sys.argv[1]
    for root, dirs, files in os.walk(target_dir):
        for file in files:
            if file.endswith('.md'):
                process_file(os.path.join(root, file))

if __name__ == '__main__':
    main()
