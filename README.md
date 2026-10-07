# DesignCraft Studio

An open-source, sovereign desktop publishing and page layout application built in pure Rust, powered by the **Martensite** GPU-accelerated retained-mode GUI engine.

![DesignCraft Studio on Martensite](brag/demo.gif)

## Architecture

- **`crates/ui-martensite`**: Sovereign retained-mode publishing UI with facing page spreads, baseline grids, and typographic rulers.
- **`crates/engine`**: Advanced text frame threading, OpenType hyphenation and justification (H&J), and print-ready PDF/X export.

## Legal & Compliance Notice

DesignCraft is an independent open-source publishing application. It is not affiliated with Adobe Inc. Adobe, InDesign, and Creative Cloud are trademarks of Adobe Inc. Multi-column publishing layouts and master page spreads derive from traditional printing conventions.

## License

Dual-licensed under MIT OR Apache-2.0.
