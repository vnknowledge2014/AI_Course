#!/usr/bin/env python3
import os
import re
import subprocess
from concurrent.futures import ThreadPoolExecutor, as_completed

WORKSPACE_DIR = "/Volumes/Lexar/01_Projects/Personal_Projects/AI_Course"
RESOURCE_FILE = os.path.join(WORKSPACE_DIR, "resource.md")
REPOS_DIR = os.path.join(WORKSPACE_DIR, "repos")

def get_github_urls(filepath):
    urls = []
    in_github_section = False
    
    # Read the markdown file
    with open(filepath, "r", encoding="utf-8") as f:
        lines = f.readlines()
        
    for line in lines:
        # Check sections
        if "## 2. GitHub Repositories" in line:
            in_github_section = True
            continue
        elif "## 3. Educational Platforms" in line:
            in_github_section = False
            break
            
        if in_github_section:
            # Match markdown link target or raw URL
            # e.g., [text](url) -> we want url
            match = re.search(r'\]\((https://github\.com/[^\)]+)\)', line)
            if match:
                urls.append(match.group(1).strip())
            else:
                # Fallback to standard github URL in the line
                match_raw = re.search(r'(https://github\.com/[a-zA-Z0-9_.-]+/[a-zA-Z0-9_.-]+)', line)
                if match_raw:
                    urls.append(match_raw.group(1).strip())
                    
    # Deduplicate and return
    seen = set()
    unique_urls = []
    for url in urls:
        # Clean up any trailing / or .git
        cleaned_url = url.rstrip('/')
        if cleaned_url not in seen:
            seen.add(cleaned_url)
            unique_urls.append(cleaned_url)
            
    return unique_urls

def clone_repo(url):
    # Extract repo name from URL
    # e.g., https://github.com/user/repo-name.git -> repo-name
    repo_name = url.split('/')[-1]
    if repo_name.endswith('.git'):
        repo_name = repo_name[:-4]
        
    target_path = os.path.join(REPOS_DIR, repo_name)
    env = os.environ.copy()
    env["GIT_TERMINAL_PROMPT"] = "0"
    
    if os.path.exists(target_path):
        git_path = os.path.join(target_path, ".git")
        if os.path.isdir(git_path):
            print(f"[UPDATE] Updating {repo_name} at {target_path}...")
            try:
                result = subprocess.run(
                    ["git", "pull"],
                    cwd=target_path,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE,
                    text=True,
                    check=True,
                    env=env,
                    timeout=180 # 3 minutes timeout per update
                )
                output = result.stdout.strip()
                if "Already up to date" in output or "Already up-to-date" in output:
                    return f"[SUCCESS] {repo_name} is already up to date."
                return f"[SUCCESS] Updated {repo_name} with latest changes."
            except subprocess.TimeoutExpired:
                return f"[ERROR] Timeout while updating {repo_name}."
            except subprocess.CalledProcessError as e:
                return f"[ERROR] Failed to update {repo_name}. Error:\n{e.stderr.strip()}"
        else:
            return f"[ERROR] Directory {repo_name} exists but is not a valid git repository."
        
    print(f"[START] Cloning {repo_name} from {url}...")
    try:
        # Run git clone with --depth 1 to save time and space
        result = subprocess.run(
            ["git", "clone", "--depth", "1", url, target_path],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            check=True,
            env=env,
            timeout=300 # 5 minutes timeout per clone
        )
        return f"[SUCCESS] Cloned {repo_name}."
    except subprocess.TimeoutExpired:
        return f"[ERROR] Timeout while cloning {repo_name}."
    except subprocess.CalledProcessError as e:
        return f"[ERROR] Failed to clone {repo_name}. Error:\n{e.stderr.strip()}"

def main():
    if not os.path.exists(RESOURCE_FILE):
        print(f"Error: {RESOURCE_FILE} not found.")
        return
        
    # Ensure repos directory exists
    os.makedirs(REPOS_DIR, exist_ok=True)
    
    urls = get_github_urls(RESOURCE_FILE)
    print(f"Found {len(urls)} repositories to clone.")
    for idx, url in enumerate(urls, 1):
        print(f"  {idx}. {url}")
        
    print("\nStarting parallel cloning with 3 concurrent workers...\n")
    
    # Run in parallel using ThreadPoolExecutor
    results = []
    with ThreadPoolExecutor(max_workers=3) as executor:
        futures = {executor.submit(clone_repo, url): url for url in urls}
        for future in as_completed(futures):
            res = future.result()
            print(res)
            results.append(res)
            
    print("\nAll cloning jobs finished!")

if __name__ == "__main__":
    main()
