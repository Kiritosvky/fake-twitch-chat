import re, collections

lines = [l.strip() for l in open("assets/phrases.txt", encoding="utf-8") if l.strip()]
ok = []
for s in lines:
    if re.search(r"\d", s) or "http" in s.lower():
        continue
    words = re.findall(r"[\wа-яёА-ЯЁ']+", s)
    if not (3 <= len(words) <= 14):
        continue
    if any(v > 1 for v in collections.Counter(w.lower() for w in words).values()):
        continue
    if any(re.search(r"[а-яё][А-ЯЁ]", w) for w in words):
        continue
    letters = sum(1 for c in s if c.isalpha() or c.isspace() or c in ".,!?—-:;\"'()")
    if letters < len(s) * 0.92:
        continue
    if any(not re.search(r"[аеёиоуыэюя]", w.lower()) or re.search(r"[бвгджзйклмнпрстфхцчшщ]{4}", w.lower()) for w in words):
        continue
    ok.append(s)

seen = set()
uniq = [x for x in ok if not (x.lower() in seen or seen.add(x.lower()))]
open("assets/phrases.txt", "w", encoding="utf-8").write("\n".join(uniq))
print("kept", len(uniq), "of", len(lines))
