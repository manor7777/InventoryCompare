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

- 🔓 **Decrypts `lastData.dat` locally.** Nothing is uploaded anywhere.
- ⚖️ **Side-by-side diff** of every item category: Warframes, weapons, mods, resources, blueprints and more.
- 🎯 **One-click filters:** *Only X has*, *X has more*, *Same*, *Everything*.
- 🔍 **Instant search.** Type "Forma" or "Rhino" and get your answer.
- 🗂️ **Category filter** to narrow the list.
- 🖱️ **Drag & drop** files straight onto the window.
- ✏️ **Rename the players** (click the name) so the columns say *You* and *Charlie*.
- 🪟 **Fluent / WinUI 3 look:** dark Mica-style palette, rounded cards, accent blue, Segoe UI.
- 📦 **Single `.exe`,** no installer or runtime needed.

## 🚀 Quick start

1. Grab `InventoryCompare.exe` from the [latest release](https://github.com/manor7777/InventoryCompare/releases/latest).
2. Find your own file at `%LOCALAPPDATA%\AlecaFrame\lastData.dat` (press <kbd>Win</kbd>+<kbd>R</kbd> and paste that path).
3. Ask your friend to send you theirs.
4. Open the app, drop both files in, and pick **Only Charlie has**.

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
cargo test              # decrypt + diff unit tests
```

Requires a recent stable Rust toolchain. The UI is built with [`egui`](https://github.com/emilk/egui) and custom-styled to match Fluent Design.

## 📦 Releases

Pushing a tag like `v0.1.0` triggers the **Release** workflow, which tests, builds on `windows-latest`, and publishes `InventoryCompare.exe` plus a zip to GitHub Releases. It can also be run manually from the **Actions** tab.

```bash
git tag v0.1.1 && git push origin v0.1.1
```

## ⚠️ Disclaimer

Unofficial fan tool. Not affiliated with Digital Extremes or AlecaFrame. *Warframe* is a trademark of Digital Extremes Ltd.

<div align="center">Made so nobody has to ask "have you got this?" ever again. 🫡</div>
