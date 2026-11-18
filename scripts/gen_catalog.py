#!/usr/bin/env python3
"""Build assets/catalog.json from the WFCD/warframe-items data.

Usage: python scripts/gen_catalog.py [local_dir_with_json]
If a directory is given it is used instead of downloading.
"""
import json, re, sys, urllib.request
from pathlib import Path

BASE = "https://raw.githubusercontent.com/WFCD/warframe-items/master/data/json/"
# file -> mastery XP cap used to decide "mastered" (weapons 450k, frames/companions 900k)
PRIME_FILES = {
    "Warframes": 900000, "Archwing": 900000, "Sentinels": 900000, "Pets": 900000,
    "Primary": 450000, "Secondary": 450000, "Melee": 450000,
    "Arch-Gun": 450000, "Arch-Melee": 450000,
}
ICON_FILES = list(PRIME_FILES) + ["Mods", "Resources", "Misc", "Gear", "Arcanes"]


def load(name, local):
    if local:
        return json.loads((Path(local) / f"{name}.json").read_text(encoding="utf-8"))
    with urllib.request.urlopen(BASE + name + ".json", timeout=120) as r:
        return json.loads(r.read())


def split_camel(s):
    s = s.replace("_", " ")
    return re.sub(r"(?<=[a-z0-9])(?=[A-Z])|(?<=[A-Z])(?=[A-Z][a-z])", " ", s)


def part_name(item_name, unique):
    seg = unique.rsplit("/", 1)[-1]
    seg = re.sub(r"Component$", "", seg)
    base = item_name.replace(" Prime", "").replace(" ", "").replace("&", "")
    rest = seg.replace("Prime", "", 1).replace(base, "", 1)
    rest = rest.strip()
    return f"{item_name} {split_camel(rest)}" if rest else f"{item_name} {split_camel(seg)}"


def main():
    local = sys.argv[1] if len(sys.argv) > 1 else None
    items, primes = {}, []
    for f in ICON_FILES:
        for x in load(f, local):
            u, n, img = x.get("uniqueName"), x.get("name"), x.get("imageName")
            if u and n:
                items[u] = [n, img or ""]
            if f in PRIME_FILES and n and n.endswith(" Prime") and x.get("components"):
                parts = []
                for c in x["components"]:
                    cu = c["uniqueName"]
                    if "/Recipes/" in cu and "Prime" in cu:
                        pn = part_name(n, cu)
                        parts.append([cu, pn, c.get("itemCount", 1)])
                        items[cu] = [pn, img or ""]
                if parts:
                    primes.append({"u": u, "n": n, "i": img or "", "x": PRIME_FILES[f], "p": parts})
    out = {"items": items, "primes": primes}
    p = Path(__file__).resolve().parent.parent / "assets" / "catalog.json"
    p.write_text(json.dumps(out, separators=(",", ":"), ensure_ascii=False), encoding="utf-8")
    print(f"{len(items)} items, {len(primes)} primes -> {p} ({p.stat().st_size/1e6:.2f} MB)")


if __name__ == "__main__":
    main()
