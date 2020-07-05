from nltk.corpus import gutenberg

out = open("gutenberg_sentences.txt","w")
for fileid in gutenberg.fileids():
   for sent in gutenberg.sents(fileid):
      out.write(" ".join(sent)+"\n")
