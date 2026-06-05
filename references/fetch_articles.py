import os
import re
import time
import requests
from urllib.parse import urljoin, urlparse
from bs4 import BeautifulSoup
from markdownify import markdownify as md
from playwright.sync_api import sync_playwright

BASE_DIR = os.path.dirname(os.path.abspath(__file__))
RESOURCE_FILE = os.path.join(os.path.dirname(BASE_DIR), 'resource.md')
ARTICLES_DIR = os.path.join(BASE_DIR, 'articles')
IMAGES_DIR = os.path.join(ARTICLES_DIR, 'images')

os.makedirs(ARTICLES_DIR, exist_ok=True)
os.makedirs(IMAGES_DIR, exist_ok=True)

def extract_urls(filepath):
    urls = []
    in_articles_section = False
    with open(filepath, 'r', encoding='utf-8') as f:
        for line in f:
            if "## 4. Articles & Technical Notes" in line:
                in_articles_section = True
                continue
            if in_articles_section and line.startswith("## "):
                # Reached another main section, break
                break
            if in_articles_section:
                # Find markdown links: [text](url)
                matches = re.findall(r'\[.*?\]\((https?://.*?)\)', line)
                for match in matches:
                    urls.append(match)
    return urls

def sanitize_filename(url):
    parsed = urlparse(url)
    name = parsed.netloc + parsed.path
    name = re.sub(r'[^a-zA-Z0-9_\-]', '_', name)
    name = name.strip('_')
    if not name:
        name = "index"
    if name.endswith('_html'):
         name = name[:-5]
    return name + '.qmd'

def download_image(img_url, source_url):
    try:
        # Resolve relative URLs
        full_url = urljoin(source_url, img_url)
        parsed = urlparse(full_url)
        filename = os.path.basename(parsed.path)
        if not filename:
            filename = f"img_{int(time.time())}.png"
            
        # Clean filename
        filename = re.sub(r'[^a-zA-Z0-9_.\-]', '_', filename)
        # Ensure it has an extension
        if '.' not in filename[-5:]:
             filename += '.png'
             
        local_path = os.path.join(IMAGES_DIR, filename)
        
        # Don't download if it already exists
        if not os.path.exists(local_path):
            headers = {'User-Agent': 'Mozilla/5.0'}
            response = requests.get(full_url, headers=headers, timeout=10)
            response.raise_for_status()
            with open(local_path, 'wb') as f:
                f.write(response.content)
                
        return f"images/{filename}"
    except Exception as e:
        print(f"    Failed to download image {img_url}: {e}")
        return img_url # Keep original if failed

def process_article(page, url):
    print(f"\nProcessing: {url}")
    filename = sanitize_filename(url)
    filepath = os.path.join(ARTICLES_DIR, filename)
    # if os.path.exists(filepath):
    #     print(f"  Skipping {url}, file already exists.")
    #     return

    try:
        page.goto(url, timeout=30000, wait_until='networkidle')
    except Exception as e:
        print(f"  Timeout/Error loading {url}, attempting to extract what is rendered... {e}")
        try:
            page.evaluate("window.stop()")
        except Exception:
            pass
        
    time.sleep(2) # Give a little extra time for lazy images
    
    try:
        html_content = page.content()
    except Exception as e:
        print(f"  Failed to get page content for {url}: {e}")
        return
        
    soup = BeautifulSoup(html_content, 'html.parser')
    
    # Pre-process KaTeX math formulas
    for katex_span in soup.find_all(class_='katex'):
        annotation = katex_span.find('annotation', encoding='application/x-tex')
        if annotation:
            latex = annotation.text.strip()
            if 'katex-display' in katex_span.get('class', []) or (katex_span.parent and katex_span.parent.get('class', []) == ['katex-display']):
                new_str = f"\n\n$$$\n{latex}\n$$$\n\n"
            else:
                new_str = f" $ {latex} $ "
            katex_span.replace_with(new_str)
            
    # Pre-process MathJax math formulas
    for math_script in soup.find_all('script', type=re.compile(r'^math/tex')):
        latex = math_script.text.strip()
        if 'display' in math_script.get('type', ''):
            new_str = f"\n\n$$$\n{latex}\n$$$\n\n"
        else:
            new_str = f" $ {latex} $ "
        math_script.replace_with(new_str)

    # Clean up unnecessary tags
    for tag in soup(["script", "style", "nav", "footer", "iframe", "noscript"]):
        tag.decompose()
        
    # Process images
    images = soup.find_all('img')
    print(f"  Found {len(images)} images.")
    for img in images:
        src = img.get('src') or img.get('data-src') or img.get('data-lazy-src')
        if src and not src.startswith('data:image'):
            local_src = download_image(src, url)
            img['src'] = local_src
            
    # Optional: try to find main content block to avoid sidebars
    main_content = soup.find('main') or soup.find('article') or soup.find('div', class_='content') or soup.body
    if not main_content:
        main_content = soup
        
    markdown_content = md(str(main_content), heading_style="ATX", strip=['a'])
    
    filename = sanitize_filename(url)
    filepath = os.path.join(ARTICLES_DIR, filename)
    
    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(f"# Source: {url}\n\n")
        f.write(markdown_content)
        
    print(f"  ✅ Saved to {filename}")

def main():
    urls = extract_urls(RESOURCE_FILE)
    print(f"Found {len(urls)} URLs in resource.md under Articles section.")
    
    # Remove duplicates
    urls = list(dict.fromkeys(urls))
    
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        context = browser.new_context(
            user_agent="Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
        )
        page = context.new_page()
        
        for url in urls:
            process_article(page, url)
            
        browser.close()
    
    print("\nAll articles processed successfully!")

if __name__ == "__main__":
    main()
