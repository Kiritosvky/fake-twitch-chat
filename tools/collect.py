import json, os, re, urllib.parse, urllib.request, concurrent.futures as cf, time, random

UA = {"User-Agent": "Mozilla/5.0"}

def get(url, data=None, headers=None):
    h = dict(UA)
    if headers:
        h.update(headers)
    req = urllib.request.Request(url, data=data, headers=h)
    for _ in range(3):
        try:
            return urllib.request.urlopen(req, timeout=40).read().decode("utf-8", "replace")
        except Exception:
            time.sleep(1.2)
    return ""

def collect():
    base = "https://huggingface.co/datasets/Daankular/twitch-chat/resolve/main/"
    api = "https://huggingface.co/api/datasets/Daankular/twitch-chat/tree/main/"
    chans = [x["path"] for x in json.loads(get(api + "data") or "[]")]
    random.seed(7)
    random.shuffle(chans)
    chans = chans[:90]

    def texts(chan):
        files = [x["path"] for x in json.loads(get(api + chan) or "[]")][:12]
        out = []
        for f in files:
            for line in get(base + f).splitlines():
                try:
                    t = json.loads(line).get("text")
                except Exception:
                    continue
                if isinstance(t, str):
                    out.append(t)
        return out

    raw = []
    with cf.ThreadPoolExecutor(12) as ex:
        for r in ex.map(texts, chans):
            raw += r
    print("raw", len(raw), flush=True)

    bad = re.compile(r"http|www\.|@|^!|\d{3,}")
    camel = re.compile(r"[a-z][A-Z]")
    seen, keep = set(), []
    for s in raw:
        s = " ".join(s.split())
        if not (18 <= len(s) <= 90) or bad.search(s) or not s.isascii():
            continue
        if len(s.split()) < 4 or not s[0].isalpha() or camel.search(s):
            continue
        k = s.lower()
        if k in seen:
            continue
        seen.add(k)
        keep.append(s)
    print("filtered", len(keep), flush=True)
    open("assets/raw_en.txt", "w", encoding="utf-8").write("\n".join(keep[:3000]))

if not os.path.exists("assets/raw_en.txt"):
    collect()

src = [l for l in open("assets/raw_en.txt", encoding="utf-8").read().splitlines() if l.strip()]
print("source", len(src), flush=True)

def tr(batch):
    q = "".join("&q=" + urllib.parse.quote(s) for s in batch)
    r = get("https://clients5.google.com/translate_a/t?client=dict-chrome-ex&sl=en&tl=ru" + q)
    try:
        d = json.loads(r)
    except Exception:
        return []
    if isinstance(d, str):
        d = [d]
    return d

camel = re.compile(r"[a-z][A-Z]")
batches = [src[i:i + 20] for i in range(0, len(src), 20)]
out = []
with cf.ThreadPoolExecutor(6) as ex:
    for res in ex.map(tr, batches):
        for r in res:
            if not isinstance(r, str):
                continue
            r = " ".join(r.split())
            if not (15 <= len(r) <= 110) or camel.search(r):
                continue
            cyr = sum(1 for c in r if "\u0400" <= c <= "\u04ff")
            if cyr >= len(r) * 0.8 and len(r.split()) >= 3:
                out.append(r)
        print(len(out), flush=True)

seen = set()
uniq = [x for x in out if not (x.lower() in seen or seen.add(x.lower()))]
open("assets/phrases.txt", "w", encoding="utf-8").write("\n".join(uniq))
print("saved", len(uniq))
