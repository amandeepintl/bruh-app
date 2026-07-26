<p align="center">
  <img src="assets/logo.png" width="128" height="128" alt="BRUH-BETTER Logo">
</p>

<h1 align="center">BRUH-BETTER</h1>

<p align="center">
  <b>Modern Binary Image Container (.bruh) Viewer, Converter & Windows Integration Toolkit</b>
</p>

<p align="center">
  <a href="https://github.com/amandeepintl/bruh-BETTER/releases/tag/v1.0.0">
    <img src="https://img.shields.io/badge/Release-v1.0.0-00C8FF?style=for-the-badge&logo=github" alt="Release">
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/License-MIT-green?style=for-the-badge" alt="License">
  </a>
  <a href="https://github.com/amandeepintl/bruh-BETTER/actions">
    <img src="https://img.shields.io/badge/Build-Passing-brightgreen?style=for-the-badge&logo=github-actions" alt="Build Status">
  </a>
  <img src="https://img.shields.io/badge/Platform-Windows-blue?style=for-the-badge&logo=windows" alt="Platform">
</p>

---

## ⚡ Overview

**BRUH-BETTER** is a high-performance binary image container toolkit written in Rust. It features a custom Zlib-compressed binary file format (`.bruh`), an interactive GPU-accelerated image viewer, a 100% responsive thumbnail gallery grid, a 1-click batch image converter, persistent application settings, and native Windows Explorer shell extension integration.

---

## 📸 Screenshots & UI Showcase

### 🏠 Launcher Screen & Application Settings
> Drag & drop file input, 3 quick action buttons (`Open Image File`, `Batch Convert`, `Open Folder Gallery`), 10-item scrollable Recent Files History, and persistent Application Settings.

![BRUH-BETTER Launcher & Application Settings](assets/launcher.png)

---

### 🖼️ Interactive Image Viewer
> Fullscreen viewport displaying `.bruh`, `.png`, `.jpg`, `.webp`, `.bmp`, and `.gif` images with real-time zoom, drag/pan, and smooth navigation.

![Interactive Image Viewer](assets/viewer.png)

---

### 🛠️ Toolbar Controls Header
> Complete toolbar featuring `🔍 Fit`, `➕ Zoom In (+25%)`, `➖ Zoom Out (-25%)`, `🔄 Rotate 90° CW`, `Flip H`, `Flip V`, `💾 Save`, `📋 Copy`, `💾 Export PNG`, and `◀ Prev` / `Next ▶` folder navigation.

![Viewer Toolbar Controls](assets/toolbar.png)

---

### ⚙️ Setup & Integration Wizard (`bruh-setup.exe`)
> Automated installer setup wizard to configure Windows file associations, context menu options, desktop shortcuts, PATH environment variables, and shell thumbnail extensions.

![BRUH-BETTER Setup Wizard](assets/installer.png)

---

### 🧩 Native Windows File Explorer Thumbnail Provider
> Real-time thumbnail preview rendering for `.bruh` files directly inside Windows File Explorer.

![Windows File Explorer Thumbnail Preview](assets/thumbnail.png)

---

## 🚀 Complete Feature Breakdown

### 1. ⚡ **High-Speed Binary Container Codec (.bruh)**
* **Zlib Compression**: Custom 14-byte `BTTR` binary magic header container format with Zlib compression (`FLAG_ZLIB_COMPRESSED`).
* **Ultra-Fast Multi-Threaded Codec**: Decodes RGBA pixel payloads instantly for high-resolution images.

---

### 2. 🖼️ **Interactive Image Viewer**
* **Universal Format Support**: `.bruh`, `.png`, `.jpg`, `.jpeg`, `.webp`, `.bmp`, `.gif`.
* **Dynamic Zoom & Pan**: `🔍 Fit`, `➕ Zoom In`, `➖ Zoom Out`, `Zoom %` indicator, and mouse drag/pan.
* **Folder Navigation**: Step through folder images with keyboard arrow keys (`Left`/`Right` or `A`/`D`) & `◀ Prev` / `Next ▶` buttons.

---

### 3. 🔄 **Transformations & Save Options Workflow**
* **Image Transformations**: `🔄 Rotate 90° CW`, `Flip H` (Horizontal flip), `Flip V` (Vertical flip).
* **Save Options Popup Modal (`💾 Save Image Changes`)**:
  * **`💾 Overwrite Original File`**: Direct overwrite of current `.bruh` or standard image file.
  * **`📄 Save as Copy...`**: Native File Save Dialog pre-filled with `<filename>_edited.bruh` or `.png`.
  * **`💾 Export PNG`**: Instant PNG export.

---

### 4. 📂 **100% Responsive Thumbnail Gallery Grid**
* **Dynamic Reflowing Grid Layout**: Automatically calculates column count (`cols = available_width / card_width`) based on window size.
* **Lazy Thumbnail Caching**: Smooth vertical scrolling with lazy thumbnail decoding.

---

### 5. ⚡ **1-Click Batch Image Converter**
* **Folder Conversion**: Converts entire folders of standard images (`.png`, `.jpg`, `.webp`) into `.bruh` format at once.
* **Accessible Anywhere**: Triggerable from Launcher screen **`⚡ Batch Convert`** or inside any Gallery header bar!
* **Auto-Delete Source Option**: Optional `[x] Delete original image files after conversion` checkbox.

---

### 6. 🕒 **Persistent Recent Files History**
* **Stores Top 10 Recent Files**: Saved across app restarts in `%LOCALAPPDATA%\BRUH-BETTER\recent_files.txt`.
* **Scrollable History List**: Displays file sizes (`KB`/`MB`), format badges (`⚡`/`📷`), truncated filenames, and 1-click re-open.

---

### 7. ⚙️ **Persistent Application Settings**
* **Conversion Warning Preferences**: Toggle non-BRUH conversion warnings (Saved to `%LOCALAPPDATA%\BRUH-BETTER\suppress_modal.txt`).
* **Auto-Delete Toggles**: Independent controls for single conversion and batch conversion auto-deletion.
* **System Maintenance**: 1-click `⚡ Re-Register File Association` and `🗑 Clear Recent Files`.

---

### 8. 💻 **Windows OS & Explorer Shell Integration**
* **File Association**: Double-clicking `.bruh` files opens them directly in **BRUH-BETTER**.
* **Right-Click Context Menu**: Right-click any image in File Explorer -> **`Open with BRUH-BETTER`**.
* **Shell Thumbnail Provider (`bruh_thumb.dll`)**: Native `.bruh` thumbnail preview rendering in Windows File Explorer.
* **Installer Setup Wizard (`bruh-setup.exe`)**: Automated installation creating Start Menu & Desktop shortcuts.

---

## 📦 Downloads & Installation

### Option A: Download Pre-Built Binaries
Download the latest binaries directly from the [Official V1.0.0 Release](https://github.com/amandeepintl/bruh-BETTER/releases/tag/v1.0.0):
- **`bruh-setup.exe`**: GUI Setup Installer Wizard
- **`bruh.exe`**: Portable Standalone Application
- **`bruh_thumb.dll`**: Windows Shell Extension DLL

### Option B: Build From Source

```bash
# Clone repository
git clone https://github.com/amandeepintl/bruh-BETTER.git
cd bruh-BETTER

# Build release binaries
cargo build --release

# Run GUI application
cargo run --release
```

---

## 💻 CLI Usage

```bash
# Compile a standard image to .bruh format
bruh compile input.png output.bruh

# Decode a .bruh file back to PNG
bruh decode input.bruh output.png

# Register Windows file associations & right-click context menu
bruh register
```

---

## 🧪 Automated Testing

Run the Rust test suite:

```bash
cargo test
```

---

## 📄 License

Distributed under the MIT License. See [`LICENSE`](LICENSE) for more information.
