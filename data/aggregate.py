import time
import os
import re
import random
if not os.path.exists("wiktionary_en.txt"):
   open("wiktionary_en.txt","w")

passed = set()
for line in open("wiktionary_en.txt","r"):
   passed.add(line.split(" ")[0])

outfile = open("wiktionary_en.txt","a")
frontier = set(["test"])

from wiktionaryparser import WiktionaryParser
parser = WiktionaryParser()
parser.set_default_language('english')

def rw_pos(v, force=False):
   if not force and v in passed: return
   try:
      word = parser.fetch(v)
      pos = set()
      rel = set()
      for w in word:
         for d in w["definitions"]:
            pos.add(d["partOfSpeech"])
            for r in d["relatedWords"]:
               if r['relationshipType'] in ['synonyms','antonyms']:
                  for wr in r["words"]:
                     for nv in re.findall("\\w+",wr):
                        if nv not in passed:
                           frontier.add(nv.lower())
      outfile.write(v+" "+",".join(pos)+"\n")
   except: pass
   passed.add(v)

if len(passed)>0:
   w = random.choice(list(passed))
   rw_pos(w,force=True)

while len(frontier)>0:
   w = frontier.pop()
   rw_pos(w)
   time.sleep(1)
