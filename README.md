<div align="center">
  <h1>Tyupy</h1>

<img src='docs/logo.svg' width=80px />

Get URL(s) title in any format.

<a href="https://github.com/azzamsa/tyupy/actions/workflows/ci.yml">
    <img src="https://github.com/azzamsa/tyupy/actions/workflows/ci.yml/badge.svg" alt="Build status" />
  </a>

<a href="https://crates.io/crates/tyupy">
    <img src="https://img.shields.io/crates/v/tyupy.svg">
  </a>

<a href=" https://docs.rs/tyupy/">
    <img src="https://docs.rs/tyupy/badge.svg">
  </a>

<a href="https://azzamsa.com/support/">
    <img alt="Sponsor me" src="https://img.shields.io/badge/Sponsor%20Me-%F0%9F%92%96-ff69b4">
  </a>

<p><p/>

</div>

---

## Features

- Get URL(s) title in any format*.
  - Markdown
  - Org-mode
- Fancy error message and colorful output.
- Cross-platform and single binary.

*Please open a PR if your favorite format doesn't exist

## Usage

```bash
🦄 tyupy --help

🦄 tyupy https://github.com/azzamsa/tin/ # Get URL title in markdown format (default).
[GitHub - azzamsa/tin: Rust GraphQL Template 🏗](https://github.com/azzamsa/tin/)

🦄 tyupy https://github.com/azzamsa/tin/ --max-length 16 # Limit title length
[GitHub - azzamsa...](https://github.com/azzamsa/tin)

🦄 tyupy https://github.com/azzamsa/tin/ --max-length 16 --ellipsis "***" # Use custom ellipsis
[GitHub - azzamsa***](https://github.com/azzamsa/tin)

🦄 tyupy https://github.com/azzamsa/tin/ --format org # Use `-f o` alias for less typing
[[https://github.com/azzamsa/tin/][GitHub - azzamsa/tin: Rust GraphQL Template 🏗]]

🦄 tyupy # reads from stdin
https://github.com/azzamsa/zman
[GitHub - azzamsa/zman: A time progress bar utilities](https://github.com/azzamsa/zman)
https://github.com/azzamsa/tin/
https://github.com/azzamsa/tun
[GitHub - azzamsa/tin: Rust GraphQL Template 🏗](https://github.com/azzamsa/tin/)
[GitHub - azzamsa/tun: Rust REST API Boilerplate 🏗](https://github.com/azzamsa/tun)
```

## Installation

### From binaries

The [release page](https://github.com/azzamsa/tyupy/releases) includes
pre-compiled binaries for GNU/Linux, macOS, and Windows.

### From source

Using [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)

```bash
$ cargo binstall tyupy
```

Using Rust's package manager [cargo](https://github.com/rust-lang/cargo):

```bash
$ cargo install tyupy
```

## Development

```bash
git clone https://github.com/azzamsa/tyupy

# Build
cd tyupy
cargo build

# Run unit tests and integration tests
cargo test

# Install
cargo install --path .
```

## Contributing

To learn more read [the development guide](docs/dev/README.md)

## Origin of the name

The term "tyupy" is a playful variation inspired by [Tupai](https://id.wikipedia.org/wiki/Tupai), which translates to "Squirrel" in English. The choice is influenced by the URL-like sound associated with the word "Squirrel".

## Credits

- [rexim's org-cliplink](https://github.com/rexim/org-cliplink)
- [Noto Emoji](https://github.com/googlefonts/noto-emoji)
