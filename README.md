# BRUH-BETTER

BRUH-BETTER is a small image container format and command-line tool for converting common image files into a `.bruh` archive and back again.

## What changed in this cleanup

- The older prototype code was replaced with a simpler modular layout under [src](src).
- Core image encode/decode logic now lives in [src/lib.rs](src/lib.rs).
- The CLI entrypoint is isolated in [src/cli.rs](src/cli.rs).
- A lightweight GUI stub is available in [src/gui.rs](src/gui.rs).

## Usage

```bash
cargo run -- compile input.png
cargo run -- decode input.bruh output.png
```

The CLI accepts:
- `compile <input> [output]`
- `decode <input> [output]`

## Development

```bash
cargo test
```

