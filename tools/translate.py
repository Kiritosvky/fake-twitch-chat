import json, re, urllib.parse, urllib.request, concurrent.futures as cf, collections

EN = set("the you that is and lol bro why what he she it to of my this are was for not have "
         "with your just but they like all on in at so we be do if can get out up about "
         "dont im its u r pls please yes no how when who will would good bad one now than".split())

src = [l.strip() for l in open("assets/raw_en.txt", encoding="utf-8") if l.strip()]
keep = []
for s in src:
    words = re.findall(r"[a-zA-Z']+", s.lower())
    if len(words) < 4:
        continue
    hits = sum(1 for w in words if w in EN)
    if hits < 2 or hits < len(words) * 0.15:
        continue
    keep.append(s)
print("english-ish", len(keep))

def tr(batch):
    q = "".join("&q=" + urllib.parse.quote(s) for s in batch)
    req = urllib.request.Request(
        "https://clients5.google.com/translate_a/t?client=dict-chrome-ex&sl=en&tl=ru" + q,
        headers={"User-Agent": "Mozilla/5.0"})
    for _ in range(3):
        try:
            d = json.loads(urllib.request.urlopen(req, timeout=40).read().decode("utf-8", "replace"))
            return d if isinstance(d, list) else [d]
        except Exception:
            pass
    return []

batches = [keep[i:i + 20] for i in range(0, len(keep), 20)]
out = []
with cf.ThreadPoolExecutor(6) as ex:
    for res in ex.map(tr, batches):
        out += [r for r in res if isinstance(r, str)]
print("translated", len(out))

ok = []
for s in out:
    s = " ".join(s.split())
    words = re.findall(r"[\wа-яёА-ЯЁ']+", s)
    if not (3 <= len(words) <= 14) or re.search(r"\d", s):
        continue
    if any(v > 1 for v in collections.Counter(w.lower() for w in words).values()):
        continue
    if any(re.search(r"[а-яё][А-ЯЁ]", w) for w in words):
        continue
    if any(not re.search(r"[аеёиоуыэюя]", w.lower()) or re.search(r"[бвгджзйклмнпрстфхцчшщ]{4}", w.lower()) for w in words):
        continue
    if sum(1 for c in s if "\u0400" <= c <= "\u04ff") < len(s) * 0.75:
        continue
    ok.append(s)

seen = set()
uniq = [x for x in ok if not (x.lower() in seen or seen.add(x.lower()))]
open("assets/phrases.txt", "w", encoding="utf-8").write("\n".join(uniq))
print("saved", len(uniq))
