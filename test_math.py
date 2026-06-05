import requests
from bs4 import BeautifulSoup

url = "https://www.k-a.in/pyt-comptr.html"
html = requests.get(url).text
soup = BeautifulSoup(html, 'html.parser')

print("KaTeX:", len(soup.find_all(class_='katex')))
print("MathJax scripts:", len(soup.find_all('script', type=lambda t: t and 'math/tex' in t)))
print("MathML:", len(soup.find_all('math')))

if soup.find_all(class_='katex'):
    k = soup.find_all(class_='katex')[0]
    print("Sample KaTeX HTML:")
    print(k.prettify()[:500])
