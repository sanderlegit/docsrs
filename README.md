# docsrs

[![Crates.io](https://img.shields.io/crates/v/docsrs.svg)](https://crates.io/crates/docsrs)
[![Docs.rs](https://docs.rs/docsrs/badge.svg)](https://docs.rs/docsrs)
![Status](https://img.shields.io/badge/status-beta-orange)
[![License](https://img.shields.io/crates/l/docsrs.svg)](https://crates.io/crates/docsrs)

**docsrs** is a Rust crate for retrieving and indexing `rustdoc` JSON files, enabling fuzzy searching of Rust documentation content.

Whether you're building a custom documentation browser, writing a Rust LSP plugin, or just need to programmatically explore docs — `docsrs` gives you structured access to items and metadata in a searchable form.

---

## Features

- default -> includes loading from a json file and parsing
- decompress -> includes everything from above as well as decompressing from a zst file
- fetch -> includes everything from above as well as fetching the compressed file from docs.rs

---

## Installation

Add `docsrs` to your `Cargo.toml`:

```toml
docsrs = "0.1"
```
