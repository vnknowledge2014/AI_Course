import os

def is_article(filepath):
    with open(filepath, 'r') as f:
        lines = f.readlines()
        
    paragraphs = 0
    bullets = 0
    tables = 0
    code = 0
    
    in_code = False
    for line in lines:
        stripped = line.strip()
        if stripped.startswith('```'):
            in_code = not in_code
            continue
        if in_code:
            code += 1
            continue
            
        if stripped.startswith('- ') or stripped.startswith('* '):
            bullets += 1
        elif stripped.startswith('|'):
            tables += 1
        elif len(stripped) > 40 and not stripped.startswith(('#', '>')):
            paragraphs += 1
            
    # If the file has fewer than 10 paragraphs but more than 20 bullets/tables, it's a cheatsheet
    if paragraphs < 10 and (bullets + tables) > 20:
        return False
    return True

for root, _, files in os.walk('.'):
    for f in files:
        if f.endswith('.md'):
            path = os.path.join(root, f)
            if not is_article(path):
                print(path)
