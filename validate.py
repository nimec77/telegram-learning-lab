from html.parser import HTMLParser
from pathlib import Path
class Check(HTMLParser):
    def __init__(self):
        super().__init__(); self.ids=[]; self.refs=[]; self.scripts=[]
    def handle_starttag(self, tag, attrs):
        a=dict(attrs)
        if 'id' in a: self.ids.append(a['id'])
        if tag=='a' and a.get('href','').startswith('#'): self.refs.append(a['href'][1:])
        if tag=='script' and 'src' in a: self.scripts.append(a['src'])
for p in [Path(__file__).with_name('index.html'), Path(__file__).with_name('lesson-02.html'), Path(__file__).with_name('lesson-03.html'), Path(__file__).with_name('lesson-04.html')]:
    text=p.read_text(); c=Check(); c.feed(text)
    assert len(c.ids)==len(set(c.ids)), 'duplicate IDs'
    assert all(r in c.ids for r in c.refs), 'broken section link'
    assert not c.scripts, 'must be self-contained'
    assert 'lang="en"' in text
    # Static lessons need no motion override; retain the check for animated pages.
    if p.name != "lesson-04.html":
        assert 'prefers-reduced-motion' in text
    else:
        assert not any(token in text for token in ("animation:", "transition:", "@keyframes"))
    assert 'fetch(' not in text and 'XMLHttpRequest' not in text, 'no network operations'
    assert all(s not in text for s in ['CalpicoFile', 'libfile_'])
    print('PASS: structure, section links, standalone scripts, accessibility hooks, no network operations or private identifiers')

