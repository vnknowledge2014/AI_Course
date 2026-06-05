#!/usr/bin/env python3
import os
import re
from datetime import datetime

WORKSPACE_DIR = os.path.dirname(os.path.abspath(__file__))
REPOS_DIR = os.path.join(WORKSPACE_DIR, "repos")
WIKI_DIR = os.path.join(WORKSPACE_DIR, "wiki")
SOURCES_DIR = os.path.join(WIKI_DIR, "sources")
CONCEPTS_DIR = os.path.join(WIKI_DIR, "concepts")
INDEX_FILE = os.path.join(WIKI_DIR, "index.md")
LOG_FILE = os.path.join(WIKI_DIR, "log.md")

def main():
    os.makedirs(SOURCES_DIR, exist_ok=True)
    os.makedirs(CONCEPTS_DIR, exist_ok=True)
    
    print("Starting auto-ingest process...")
    
    # 1. Process Repos
    repos = []
    if os.path.exists(REPOS_DIR):
        for d in os.listdir(REPOS_DIR):
            repo_path = os.path.join(REPOS_DIR, d)
            if os.path.isdir(repo_path):
                repos.append(d)
                
    repos.sort()
    
    for repo in repos:
        repo_path = os.path.join(REPOS_DIR, repo)
        readme_path = os.path.join(repo_path, "README.md")
        summary = f"No README found for {repo}."
        
        # Try finding a README (case-insensitive)
        for f in os.listdir(repo_path):
            if f.lower() == "readme.md":
                readme_path = os.path.join(repo_path, f)
                break
                
        if os.path.exists(readme_path):
            with open(readme_path, "r", encoding="utf-8", errors="ignore") as f:
                content = f.read(1500) # Read first 1500 chars
                # Clean up multiple newlines
                content = re.sub(r'\n{3,}', '\n\n', content)
                summary = f"**Bản trích xuất tự động từ README:**\n\n```text\n{content}\n...\n```"
                
        # Write summary file
        summary_file = os.path.join(SOURCES_DIR, f"Summary_{repo}.md")
        with open(summary_file, "w", encoding="utf-8") as f:
            f.write(f"# Tóm tắt: {repo}\n\n")
            f.write(f"Đây là trang tóm tắt tự động cho repository `{repo}`.\n\n")
            f.write(summary)
            f.write(f"\n\n*Nguồn gốc: [repos/{repo}](../../repos/{repo})*\n")
            
    print(f"Created {len(repos)} summary pages.")
    
    # 2. Create Concept Pages
    concepts = ["Agents", "Transformers", "RAG", "LLM", "Prompt_Engineering"]
    for concept in concepts:
        concept_file = os.path.join(CONCEPTS_DIR, f"{concept}.md")
        if not os.path.exists(concept_file):
            with open(concept_file, "w", encoding="utf-8") as f:
                f.write(f"# Concept: {concept.replace('_', ' ')}\n\n")
                f.write(f"Trang này tổng hợp các kiến thức liên quan đến **{concept.replace('_', ' ')}**.\n\n")
                f.write("## Các Repository liên quan\n")
                for repo in repos:
                    if concept.lower()[:3] in repo.lower() or ("agent" in concept.lower() and "agent" in repo.lower()):
                        f.write(f"- [[Summary_{repo}.md|{repo}]]\n")
                        
    # 3. Update index.md
    with open(INDEX_FILE, "w", encoding="utf-8") as f:
        f.write("# Index\n\nDanh mục phân loại toàn bộ nội dung trong LLM Wiki.\n\n")
        f.write("## Concepts\n")
        for concept in concepts:
            f.write(f"- [Concept: {concept.replace('_', ' ')}](concepts/{concept}.md)\n")
            
        f.write("\n## Sources (Nguồn đã phân tích)\n")
        for repo in repos:
            f.write(f"- [Repo: {repo}](sources/Summary_{repo}.md)\n")
            
    # 4. Update log.md
    today = datetime.now().strftime("%Y-%m-%d")
    with open(LOG_FILE, "a", encoding="utf-8") as f:
        f.write(f"\n## [{today}] ingest | Auto-ingest toàn bộ {len(repos)} repositories vào wiki\n")
        f.write(f"- Đã trích xuất README của {len(repos)} repo thành Summary Pages.\n")
        f.write(f"- Đã cập nhật mục lục index.md và tạo {len(concepts)} Concept Pages.\n")

    print("Auto-ingest complete!")

if __name__ == "__main__":
    main()
