<div align="center">

# 🛡️ Inventory Compare

**Stop asking _"have you got this?"_ — just compare.**

A tiny, fast, native Windows tool that opens two AlecaFrame `lastData.dat` files and shows exactly what each Tenno owns, what they're missing, and who has more.

[![Release](https://img.shields.io/github/actions/workflow/status/manor7777/InventoryCompare/release.yml?label=release&style=for-the-badge&color=60CDFF)](https://github.com/manor7777/InventoryCompare/actions/workflows/release.yml)
[![Latest](https://img.shields.io/github/v/release/manor7777/InventoryCompare?style=for-the-badge&color=6CCB5F)](https://github.com/manor7777/InventoryCompare/releases/latest)
![Rust](https://img.shields.io/badge/Rust-2024-f74c00?style=for-the-badge&logo=rust&logoColor=white)
![Windows](https://img.shields.io/badge/Windows-10%20%7C%2011-0078D4?style=for-the-badge&logo=windows11&logoColor=white)

</div>

---

## ✨ Features

- 🔁 **Prime trade finder:** works out which prime parts each player can hand to the other, using mastery. Parts of a mastered item are *free* to give, and if both players have it mastered they're *free to sell*. If neither has mastered it, only spare copies are suggested and it's *your call*.
- 📋 **Who needs what:** for every prime in the game, shows the parts each player still needs, and filters to *both need*, *only X needs* or *only Y needs*.
- 🖼️ **Real in-game item icons** (including part icons), downloaded on demand and cached locally.
- ⬇️ **Self-updating:** checks GitHub releases on launch and updates in one click.
- 🔓 **Decrypts `lastData.dat` locally.** Nothing is uploaded anywhere.
- ⚖️ **Side-by-side diff** of every item category: Warframes, weapons, mods, resources, blueprints and more.
- 🎯 **One-click filters:** *Only X has*, *X has more*, *Same*, *Everything*.
- 🔍 **Instant search.** Type "Forma" or "Rhino" and get your answer.
- 🗂️ **Category filter** to narrow the list.
- 🖱️ **Drag & drop** files straight onto the window.
- ✏️ **Rename the players** (click the name) so the columns show real names instead of *Player 1* / *Player 2*.
- 🪟 **Fluent / WinUI 3 look:** dark Mica-style palette, rounded cards, accent blue, Segoe UI.
- 📦 **Single `.exe`,** no installer or runtime needed.

## 🚀 Quick start

1. Grab `InventoryCompare.exe` from the [latest release](https://github.com/manor7777/InventoryCompare/releases/latest).
2. Find your own file at `%LOCALAPPDATA%\AlecaFrame\lastData.dat` (press <kbd>Win</kbd>+<kbd>R</kbd> and paste that path).
3. Ask your friend to send you theirs.
4. Open the app and drop both files in. **Prime trades** shows what you can swap; **Full compare** shows everything.

> 💡 Already-decrypted `inventory.json` files work too.

## 🧠 How it works

```
lastData.dat ──► AES-128-CBC (PKCS#7) decrypt ──► Warframe inventory JSON
                                                       │
         every array of { ItemType, ItemCount? } ◄─────┘
                          │
                 flatten → item path ↦ count
                          │
                 outer join A ⟷ B  →  table
```

Each top-level array in the inventory that holds `ItemType` entries becomes a category. Stackables use `ItemCount`; owned gear (frames, weapons) counts once per entry. Item names are derived from their `/Lotus/...` paths.

### Prime trade rules

For each prime item and each player the app checks `Mastered` (XP at the cap, 450k weapons / 900k frames and companions), `Owned` (built, not mastered) or `Missing`.

| Giver | Receiver | Result |
|---|---|---|
| Mastered | Missing parts | **Free** to give, since the giver needs none |
| Both mastered | | **Free to sell** |
| Not mastered | Missing parts | Only spare copies beyond what the recipe needs. **Your call** |

Recipe counts matter (e.g. Nami Skyla Prime needs 2 Blades and 2 Handles).

### Item data

`assets/catalog.json.gz` holds item names, icon paths and every prime recipe. It is generated from Warframe's official Public Export by `scripts/gen_catalog.py`:

```bash
python scripts/gen_catalog.py --dir path/to/exports   # ExportRecipes, ExportWeapons, ExportWarframes, ExportSentinels, ExportManifest… (+ optional warframe_lang_dict.json)
python scripts/gen_catalog.py --fetch                 # or download the latest export
```

The release workflow tries `--fetch` before each build, so new primes arrive through updates. If that fails, the committed catalog is used. Icons come from `content.warframe.com`, are cached in `%LOCALAPPDATA%\InventoryCompare\icons`, and the app works without them.

Sentinel weapons (Deconstructor Prime and friends) have no part recipes of their own. They come with their sentinel, so they are covered by the sentinel's parts.

### Key / IV overrides

The static AES key and IV are baked in, as used by other community tools. If AlecaFrame ever changes them, override without rebuilding by setting 32-character hex strings:

```powershell
$env:ALECAFRAME_KEY = "<32 hex chars>"
$env:ALECAFRAME_IV  = "<32 hex chars>"
.\InventoryCompare.exe
```

## 🛠️ Build from source

```bash
cargo run --release     # run
cargo test              # decrypt, diff and trade unit tests
python scripts/gen_catalog.py --fetch   # refresh item catalog
python scripts/make_icon.py     # regenerate the app icon
```

Requires a recent stable Rust toolchain. The UI is built with [`egui`](https://github.com/emilk/egui) and custom-styled to match Fluent Design.

## 📦 Releases

Pushing a tag like `v0.1.0` (or pushing the dev branch, which publishes a numbered pre-release) triggers the **Release** workflow, which tests, builds on `windows-latest`, and publishes `InventoryCompare.exe` plus a zip to GitHub Releases. It can also be run manually from the **Actions** tab.

```bash
git tag v0.1.1 && git push origin v0.1.1
```

## ⚠️ Disclaimer

Unofficial fan tool. Not affiliated with Digital Extremes or AlecaFrame. *Warframe* is a trademark of Digital Extremes Ltd.

<div align="center">Made so nobody has to ask "have you got this?" ever again. 🫡</div>
