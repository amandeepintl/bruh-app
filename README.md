# BRUH-BETTER

BRUH-BETTER is a high-speed binary image container format (`.bruh`), interactive GUI image viewer, responsive thumbnail gallery, batch converter, and Windows Explorer shell extension toolkit.

## Features

- **⚡ Zlib Binary Container (.bruh)**: 14-byte `BTTR` header with high-ratio Zlib compression.
- **🖼 Interactive Image Viewer**: Dynamic zoom, pan, 90° rotation, horizontal & vertical flips.
- **📂 Responsive Thumbnail Gallery**: Dynamic column reflowing grid layout with lazy caching.
- **⚡ Batch Image Converter**: 1-click folder batch image conversion.
- **🕒 Recent Files History**: Persistent 10 recent files launcher history.
- **⚙️ App Settings**: Persistent conversion popup preferences and maintenance options.
- **💻 Windows Integration**: Right-click context menu and File Explorer thumbnail provider DLL.

## Usage

```bash
cargo run -- compile input.png
cargo run -- decode input.bruh output.png
```

## Development

```bash
cargo test
```

