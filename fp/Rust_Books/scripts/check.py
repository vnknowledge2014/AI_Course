import os
import glob

def check_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()
    
    # Check if the content has "Câu chuyện" or "Tưởng tượng" or is conversational
    lines = content.split('\n')
    intro_lines = lines[:100]
    
    bullets = sum(1 for l in intro_lines if l.strip().startswith('- ') or l.strip().startswith('* '))
    paragraphs = sum(1 for l in intro_lines if len(l.strip()) > 50 and not l.strip().startswith(('>', '#', '-', '*')))
    
    # Criteria: if it has very few paragraphs but many bullets, it might be a cheat sheet.
    # Otherwise, if it has paragraphs, it's an article.
    if paragraphs < 3 and bullets > 10:
        return "WARNING: Might be cheatsheet"
    return "OK"

for root, _, files in os.walk('.'):
    for f in files:
        if f.endswith('.md'):
            path = os.path.join(root, f)
            res = check_file(path)
            if res != "OK":
                print(f"{path}: {res}")
