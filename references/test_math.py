import time
from bs4 import BeautifulSoup
from playwright.sync_api import sync_playwright

with sync_playwright() as p:
    browser = p.chromium.launch(headless=True)
    page = browser.new_page()
    page.goto('https://www.k-a.in/pyt-comptr.html', wait_until='networkidle')
    time.sleep(2)
    html = page.content()
    browser.close()

soup = BeautifulSoup(html, 'html.parser')

print("KaTeX elements:", len(soup.find_all(class_='katex')))
print("MathJax scripts:", len(soup.find_all('script', type=lambda t: t and 'math/tex' in t)))
print("mjx-container elements:", len(soup.find_all('mjx-container')))
print("math elements:", len(soup.find_all('math')))

if soup.find_all(class_='katex'):
    k = soup.find_all(class_='katex')[0]
    print("\nFirst KaTeX sample:")
    print(k.prettify()[:500])
    ann = k.find('annotation')
    if ann:
        print("Found annotation:", ann.text)
