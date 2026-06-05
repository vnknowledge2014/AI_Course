#!/usr/bin/env python3
import os
import re
import sys
import argparse
import subprocess
from concurrent.futures import ThreadPoolExecutor, as_completed

# Colors for terminal styling
GREEN = "\033[92m"
RED = "\033[91m"
YELLOW = "\033[93m"
CYAN = "\033[96m"
BOLD = "\033[1m"
RESET = "\033[0m"

WORKSPACE_DIR = os.path.dirname(os.path.abspath(__file__))
RESOURCE_FILE = os.path.join(WORKSPACE_DIR, "resource.md")
REPOS_DIR = os.path.join(WORKSPACE_DIR, "repos")

def get_github_urls(filepath):
    urls = []
    in_github_section = False
    
    if not os.path.exists(filepath):
        return []
        
    with open(filepath, "r", encoding="utf-8") as f:
        lines = f.readlines()
        
    for line in lines:
        if "## 2. GitHub Repositories" in line:
            in_github_section = True
            continue
        elif "## 3. Educational Platforms" in line:
            in_github_section = False
            break
            
        if in_github_section:
            match = re.search(r'\]\((https://github\.com/[^\)]+)\)', line)
            if match:
                urls.append(match.group(1).strip())
            else:
                match_raw = re.search(r'(https://github\.com/[a-zA-Z0-9_.-]+/[a-zA-Z0-9_.-]+)', line)
                if match_raw:
                    urls.append(match_raw.group(1).strip())
                    
    seen = set()
    unique_urls = []
    for url in urls:
        cleaned_url = url.rstrip('/')
        if cleaned_url not in seen:
            seen.add(cleaned_url)
            unique_urls.append(cleaned_url)
            
    return unique_urls

def get_repo_name(url):
    name = url.split('/')[-1]
    if name.endswith('.git'):
        name = name[:-4]
    return name

def check_sync_status(repo_path):
    """
    Checks the sync status of a repository compared to its remote.
    Returns a dict with status information.
    """
    status_info = {
        "has_local_changes": False,
        "sync_state": "Unknown",
        "error": None
    }
    
    try:
        # Check for local uncommitted changes
        status_res = subprocess.run(
            ["git", "status", "--porcelain"],
            cwd=repo_path,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            timeout=5
        )
        if status_res.returncode == 0:
            status_info["has_local_changes"] = bool(status_res.stdout.strip())
        
        # Get HEAD and Upstream hashes
        head_res = subprocess.run(
            ["git", "rev-parse", "HEAD"],
            cwd=repo_path,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            timeout=5
        )
        upstream_res = subprocess.run(
            ["git", "rev-parse", "@{u}"],
            cwd=repo_path,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            timeout=5
        )
        
        if head_res.returncode != 0:
            status_info["sync_state"] = "No Commits"
            return status_info
            
        if upstream_res.returncode != 0:
            status_info["sync_state"] = "No Upstream"
            return status_info
            
        local_sha = head_res.stdout.strip()
        remote_sha = upstream_res.stdout.strip()
        
        if local_sha == remote_sha:
            status_info["sync_state"] = "Up-to-date"
            return status_info
            
        # Check base
        base_res = subprocess.run(
            ["git", "merge-base", "HEAD", "@{u}"],
            cwd=repo_path,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            timeout=5
        )
        
        if base_res.returncode == 0:
            base_sha = base_res.stdout.strip()
            if local_sha == base_sha:
                status_info["sync_state"] = "Behind remote (Needs Pull)"
            elif remote_sha == base_sha:
                status_info["sync_state"] = "Ahead of remote (Needs Push)"
            else:
                status_info["sync_state"] = "Diverged"
        else:
            status_info["sync_state"] = "Diverged"
            
    except Exception as e:
        status_info["error"] = str(e)
        status_info["sync_state"] = "Error"
        
    return status_info

def fetch_repo(name, repo_path):
    """
    Fetches the latest remote tracking data for a repository.
    """
    try:
        env = os.environ.copy()
        env["GIT_TERMINAL_PROMPT"] = "0"
        res = subprocess.run(
            ["git", "fetch", "--prune"],
            cwd=repo_path,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            timeout=15,
            env=env
        )
        return name, res.returncode == 0, res.stderr.strip() if res.returncode != 0 else ""
    except Exception as e:
        return name, False, str(e)

def main():
    parser = argparse.ArgumentParser(description="Audit local GitHub repositories.")
    parser.add_argument(
        "-d", "--deep",
        action="store_true",
        help="Perform deep sync check by fetching remote updates from GitHub and checking for local changes."
    )
    args = parser.parse_args()

    print(f"\n{BOLD}{CYAN}=== GITHUB REPOSITORY AUDIT ==={RESET}\n")
    
    if not os.path.exists(RESOURCE_FILE):
        print(f"{RED}[ERROR] {RESOURCE_FILE} not found.{RESET}")
        sys.exit(1)
        
    if not os.path.exists(REPOS_DIR):
        print(f"{YELLOW}[WARNING] Repos directory '{REPOS_DIR}' does not exist.{RESET}")
        os.makedirs(REPOS_DIR, exist_ok=True)
        print(f"{GREEN}[INFO] Created repos directory.{RESET}\n")

    urls = get_github_urls(RESOURCE_FILE)
    if not urls:
        print(f"{YELLOW}[WARNING] No repositories found in {RESOURCE_FILE}.{RESET}")
        sys.exit(0)

    print(f"Parsed {BOLD}{len(urls)}{RESET} expected repositories from {BOLD}resource.md{RESET}.\n")

    expected_repos = {}
    for url in urls:
        name = get_repo_name(url)
        expected_repos[name] = url

    # Read actual directories inside REPOS_DIR
    actual_dirs = []
    try:
        actual_dirs = [d for d in os.listdir(REPOS_DIR) if os.path.isdir(os.path.join(REPOS_DIR, d))]
    except Exception as e:
        print(f"{RED}[ERROR] Could not read {REPOS_DIR}: {e}{RESET}")
        sys.exit(1)

    actual_set = set(actual_dirs)
    expected_set = set(expected_repos.keys())

    missing = sorted(list(expected_set - actual_set))
    extra = sorted(list(actual_set - expected_set))
    matching = sorted(list(expected_set & actual_set))

    # Audit each matching directory for basic structure
    invalid = []
    valid = []
    for name in matching:
        repo_path = os.path.join(REPOS_DIR, name)
        git_path = os.path.join(repo_path, ".git")
        
        if not os.path.isdir(git_path):
            invalid.append(name)
        else:
            valid.append(name)

    # If user requested deep check, perform parallel fetch first
    sync_results = {}
    if args.deep and valid:
        print(f"{BOLD}{CYAN}Performing deep audit... Fetching remote status from GitHub in parallel (max 5 workers)...{RESET}")
        
        fetch_errors = {}
        with ThreadPoolExecutor(max_workers=5) as executor:
            futures = {
                executor.submit(fetch_repo, name, os.path.join(REPOS_DIR, name)): name 
                for name in valid
            }
            for future in as_completed(futures):
                name, success, err = future.result()
                if not success:
                    fetch_errors[name] = err
        
        if fetch_errors:
            print(f"\n{YELLOW}[WARNING] Failed to fetch remote metadata for {len(fetch_errors)} repos (offline or private?):{RESET}")
            for name, err in fetch_errors.items():
                print(f"  - {name}: {err[:80]}")
            print()

        print(f"{BOLD}{CYAN}Analyzing repository synchronization states...{RESET}\n")
        for name in valid:
            sync_results[name] = check_sync_status(os.path.join(REPOS_DIR, name))

    # 1. Matching and Valid Repos
    print(f"{BOLD}{GREEN}✓ Cloned and Valid ({len(valid)}/{len(expected_repos)}):{RESET}")
    for name in valid:
        if args.deep:
            info = sync_results.get(name, {})
            sync_state = info.get("sync_state", "Unknown")
            has_local = info.get("has_local_changes", False)
            
            # Format sync state color
            if "Up-to-date" in sync_state:
                state_str = f"{GREEN}{sync_state}{RESET}"
            elif "Behind" in sync_state:
                state_str = f"{RED}{BOLD}{sync_state}{RESET}"
            elif "Ahead" in sync_state or "Diverged" in sync_state:
                state_str = f"{YELLOW}{sync_state}{RESET}"
            else:
                state_str = f"{CYAN}{sync_state}{RESET}"
                
            local_str = f" | {RED}Modified Files{RESET}" if has_local else f" | {GREEN}Clean{RESET}"
            print(f"  {GREEN}✓{RESET} {name:<45} -> {state_str}{local_str}")
        else:
            print(f"  {GREEN}✓{RESET} {name:<45} {CYAN}({expected_repos[name]}){RESET}")

    # 2. Invalid Clones
    if invalid:
        print(f"\n{BOLD}{RED}✗ Invalid Clones (Directories exist but lack a .git folder) ({len(invalid)}):{RESET}")
        for name in invalid:
            print(f"  {RED}✗{RESET} {name:<45} {CYAN}({expected_repos[name]}){RESET}")

    # 3. Missing Repos
    if missing:
        print(f"\n{BOLD}{RED}✗ Missing Repos (Not cloned yet) ({len(missing)}):{RESET}")
        for name in missing:
            print(f"  {RED}✗{RESET} {name:<45} {CYAN}({expected_repos[name]}){RESET}")

    # 4. Extra Repos
    if extra:
        print(f"\n{BOLD}{YELLOW}! Extra Repos (Cloned but not listed in resource.md) ({len(extra)}):{RESET}")
        for name in extra:
            repo_path = os.path.join(REPOS_DIR, name)
            git_url = "Unknown Source"
            try:
                res = subprocess.run(
                    ["git", "-C", repo_path, "remote", "get-url", "origin"],
                    stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE,
                    text=True,
                    timeout=5
                )
                if res.returncode == 0:
                    git_url = res.stdout.strip()
            except Exception:
                pass
            print(f"  {YELLOW}!{RESET} {name:<45} {CYAN}({git_url}){RESET}")

    print(f"\n{BOLD}{CYAN}=== AUDIT SUMMARY ==={RESET}")
    print(f"  Total expected: {BOLD}{len(expected_repos)}{RESET}")
    print(f"  Successfully cloned: {GREEN}{BOLD}{len(valid)}{RESET}/{len(expected_repos)}")
    
    if args.deep:
        behind_count = sum(1 for info in sync_results.values() if "Behind" in info.get("sync_state", ""))
        dirty_count = sum(1 for info in sync_results.values() if info.get("has_local_changes", False))
        print(f"  Out of sync (Behind remote): {RED}{BOLD}{behind_count}{RESET}")
        print(f"  With uncommitted local changes: {YELLOW}{BOLD}{dirty_count}{RESET}")
        
    if invalid or missing:
        print(f"  Status: {RED}{BOLD}INCOMPLETE{RESET}")
        sys.exit(1)
    elif args.deep and (behind_count > 0 or dirty_count > 0):
        print(f"  Status: {YELLOW}{BOLD}OUT OF SYNC OR DIRTY{RESET}")
        sys.exit(2)
    else:
        print(f"  Status: {GREEN}{BOLD}ALL PLANNED REPOSITORIES SECURELY CLONED & IN SYNC{RESET}")
        sys.exit(0)

if __name__ == "__main__":
    main()
