"""
   The contents of the file
       https://www.subarctic.org/manually_tagged_dictionary.txt
   are released by Andrew Johnson, (c) 2020, as an indirect part
   of this open source work, so_many_words, and under the same dual
   license structure.

   The above file was constructed from words extracted from the
   Gutenberg Project Corpus, then manually tagged with
   part-of-speech information.
"""

import requests
url = "https://www.subarctic.org/manually_tagged_dictionary.txt"
res = requests.get(url, allow_redirects=True)
open("data/manually_tagged_dictionary.txt", "wb").write(res.content)
