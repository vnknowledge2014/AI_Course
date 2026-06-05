import os
import re

directory = "/Users/mike/Documents/AI_Course/references/articles/"

# Regex for $$ ... $$ (can span multiple lines)
block_math_re = re.compile(r'\$\$(.*?)\$\$', re.DOTALL)

# Regex for $ ... $ (single line inline math)
# Match $ then any char except $, then $
inline_math_re = re.compile(r'\$([^\$]+?)\$')

def process_file(filepath):
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()

    original_content = content

    # 1. Handle $$ ... $$
    def block_repl(match):
        inner = match.group(1).strip()
        if '\n' in inner:
            # Multiline block
            return f'$$$\n{inner}\n$$$'
        else:
            # One-line block
            # To be safe as a block, we can surround it with newlines and single $ 
            # Or just ' $ inner $ ' which works as inline too.
            return f'\n\n$ {inner} $\n\n'
            
    content = block_math_re.sub(block_repl, content)

    # 2. Handle $ ... $
    def inline_repl(match):
        inner = match.group(1).strip()
        return f' $ {inner} $ '

    content = inline_math_re.sub(inline_repl, content)
    
    # 3. Handle \( ... \) and \[ ... \]
    content = re.sub(r'\\\((.*?)\\\)', lambda m: f' $ {m.group(1).strip()} $ ', content)
    content = re.sub(r'\\\[(.*?)\\\]', lambda m: f'$$$\n{m.group(1).strip()}\n$$$', content, flags=re.DOTALL)

    if content != original_content:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(content)
        print(f"Updated {os.path.basename(filepath)}")

count = 0
for filename in os.listdir(directory):
    if filename.endswith(".qmd"):
        process_file(os.path.join(directory, filename))
        count += 1
print(f"Checked {count} files.")
