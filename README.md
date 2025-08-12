# docsrs

[![Crates.io](https://img.shields.io/crates/v/docsrs.svg)](https://crates.io/crates/docsrs)
[![Docs.rs](https://docs.rs/docsrs/badge.svg)](https://docs.rs/docsrs)
![Status](https://img.shields.io/badge/status-beta-orange)
[![License](https://img.shields.io/crates/l/docsrs.svg)](https://crates.io/crates/docsrs)

**docsrs** is a Rust crate for retrieving and indexing `rustdoc` JSON files, enabling fuzzy searching of Rust documentation content.

Whether you're building a custom documentation browser, writing a Rust LSP plugin, or just need to programmatically explore docs — `docsrs` gives you structured access to items and metadata in a searchable form.

---

## CLI

This crate also provides a command-line interface for quick documentation searches from your terminal.

### Installation

With Rust's package manager `cargo`, you can install the `docsrs` CLI via:
```sh
cargo install docsrs --features fetch
```
*Note: at the time of writing, the crate isn't published with the binary yet. You can use `cargo install --git https://github.com/kingananas20/fuzzdoc --features fetch` for now.*

### Usage

```sh
docsrs <CRATE> <QUERY> [OPTIONS]
```

**Arguments:**
- `<CRATE>`: The name of the crate to search in (e.g., `serde`, `tokio`).
- `<QUERY>`: The search query (e.g., `Serialize`, `vec push`).

**Options:**
- `-v, --crate-version <VERSION>`: The version of the crate to search [default: `latest`].
- `-n <N>`: The maximum number of search results to return [default: `10`].
- `-h, --help`: Print help information.
- `-V, --version`: Print version information.

### Examples

Search for `Serialize` in the latest version of `serde`:
```sh
docsrs serde Serialize
```

Search for `spawn` in `tokio` and get up to 5 results:
```sh
docsrs tokio spawn -n 5
```

Search in a specific version of a crate:
```sh
docsrs serde Serialize -v 1.0.193
```

---

## Features

- default -> includes loading from a json file and parsing
- decompress -> includes everything from above as well as decompressing from a zst file
- fetch -> includes everything from above as well as fetching the compressed file from docs.rs

---

## Searching

The search is case-insensitive and uses fuzzy matching on the fully qualified path of an item. This means you can use partial queries to find what you're looking for.

For example, to search for `Vec::push`, you could use queries like:
- `"std::vec::Vec::push"` (exact match)
- `"vec push"` (partial match)
- `"std::vec::push"`

### Supported Item Types

The following item types are indexed and searchable. Here are examples of how you can reference them in a query:

| Item Type       | Example Reference                  |
|-----------------|------------------------------------|
| Module          | `std::collections`                 |
| Struct          | `std::vec::Vec`                    |
| Enum            | `std::option::Option`              |
| Enum Variant    | `std::option::Option::Some`        |
| Union           | `my_crate::MyUnion`                |
| Function        | `std::mem::swap`                   |
| Method          | `std::vec::Vec::push`              |
| Trait           | `std::convert::From`               |
| Trait Item      | `std::convert::From::from`         |
| Macro           | `std::println`                     |
| Constant        | `std::f64::consts::PI`             |
| Static          | `my_crate::MY_STATIC`              |
| Type Alias      | `std::io::Result`                  |
| Primitive       | `u8`                               |

**Note:** The search is not limited to these exact formats. Thanks to fuzzy matching, you can often use shorter, more convenient queries.

---

## Installation

Add `docsrs` to your `Cargo.toml`:

```toml
docsrs = "0.1"
```
