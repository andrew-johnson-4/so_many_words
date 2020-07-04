
from wiktionaryparser import WiktionaryParser
parser = WiktionaryParser()
parser.set_default_language('english')
word = parser.fetch('test')
print("test:")
for w in word:
   for d in w["definitions"]:
      print(d["partOfSpeech"])
