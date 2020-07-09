missing_terms = set()
for line in open("data/wiktionary_pronunciation_eng.txt"):
    term = line.split()
    if len(term)==1:
        missing_terms.add(term[0])

outfile = open("data/missing_wiktionary_pronunciation_eng.txt","w")
for term in missing_terms:
    outfile.write(term+"\n")
