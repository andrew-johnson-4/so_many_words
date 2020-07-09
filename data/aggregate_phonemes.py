terms = set()
for line in open("data/manually_tagged_dictionary_eng.txt"):
    term = line.split()[0]
    terms.add(term)

for line in open("data/wiktionary_pronunciation_eng.txt"):
    term = line.split()[0]
    terms.remove(term)


import re
from wiktionaryparser import WiktionaryParser
parser = WiktionaryParser()
parser.set_default_language('english')

outfile = open("data/wiktionary_pronunciation_eng.txt","a")
for term in terms:
    word = parser.fetch(term)
    ps = []
    for w in word:
        phone = " ".join(w["pronunciations"]["text"])
        for ipa in re.findall("[/](\\w+)[/]",phone):
           ps.append(ipa)
    outfile.write(term+" "+",".join(ps)+"\n")
    outfile.flush()
