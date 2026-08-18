import os
import requests
from bs4 import BeautifulSoup
from urllib.parse import urljoin
import markdownify

def scrape_rig_docs():
    base_url = "https://rig.rs/docs/"
    output_dir = "/Volumes/SEAGATE/Personal_Projects/AI_Course/fp/Rust_Books/.rig_raw_data/docs"
    os.makedirs(output_dir, exist_ok=True)
    
    # Get the main page
    response = requests.get(base_url)
    soup = BeautifulSoup(response.text, 'html.parser')
    
    # Find all links in the sidebar
    links = []
    for a in soup.find_all('a', href=True):
        href = a['href']
        if href.startswith('/docs') and not href.endswith('.zip') and not href.endswith('.png'):
            href = href.split('#')[0]
            if href not in links:
                links.append(href)
                
    print(f"Found {len(links)} pages to scrape.")
    
    for link in links:
        url = urljoin("https://rig.rs", link)
        print(f"Scraping {url}...")
        try:
            res = requests.get(url)
            page_soup = BeautifulSoup(res.text, 'html.parser')
            # Extract main content
            main_content = page_soup.find('main')
            if not main_content:
                main_content = page_soup.body
            
            md_content = markdownify.markdownify(str(main_content), heading_style="ATX")
            
            filename = link.replace('/docs', '').strip('/')
            if not filename:
                filename = 'index'
            filename = filename.replace('/', '_') + '.md'
            
            filepath = os.path.join(output_dir, filename)
            with open(filepath, 'w') as f:
                f.write(md_content)
        except Exception as e:
            print(f"Failed to scrape {url}: {e}")

if __name__ == "__main__":
    scrape_rig_docs()
