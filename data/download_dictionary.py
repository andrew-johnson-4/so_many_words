"""
   The contents of the files
       https://www.subarctic.org/manually_tagged_dictionary_[language].txt
   are released by Andrew Johnson, (c) 2020, as an indirect part
   of this open source work, so_many_words, and under the same dual
   license structure.

   The above file was constructed from words extracted from the
   Gutenberg Project Corpus, then manually tagged with
   part-of-speech information.
"""

import requests

for lang in ["eng"]:
   url = "https://www.subarctic.org/manually_tagged_dictionary_" + lang + ".txt"
   res = requests.get(url, allow_redirects=True)
   open("data/manually_tagged_dictionary_" + lang + ".txt", "wb").write(res.content)
