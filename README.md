# fuzzdoc

[![Crates.io](https://img.shields.io/crates/v/docz.svg)](https://crates.io/crates/docz)
[![Docs.rs](https://docs.rs/docz/badge.svg)](https://docs.rs/docz)
![Status](https://img.shields.io/badge/status-beta-orange)
[![License](https://img.shields.io/crates/l/docz.svg)](https://crates.io/crates/docz)

# **PLEASE DO NOT USE RIGHT NOW!! IT NEEDS A REWORK**

**fuzzdoc** is a Rust crate for retrieving and indexing `rustdoc` JSON files, enabling fuzzy searching of Rust documentation content.

Whether you're building a custom documentation browser, writing a Rust LSP plugin, or just need to programmatically explore docs — `fuzzdoc` gives you structured access to items and metadata in a searchable form.

---

## Features

- default -> includes loading from a json file and parsing
- decompress -> includes everything from above as well as decompressing from a zst file
- fetch -> includes everything from above as well as fetching the compressed file from docs.rs

---

## Installation

Add `fuzzdoc` to your `Cargo.toml`:

```toml
fuzzdoc = "0.1"
```
