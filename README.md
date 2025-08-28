# redist-icons project

[![Python CI][python-ci-badge]][python-ci-link]
[![Rust CI][rust-ci-badge]][rust-ci-link]

This repository contains (deno-flavored) TS scripts that generate Rust and
Python (via FFI bindings) packages from the following npm packages:

- [`@mdi/svg`](https://www.npmjs.com/package/@mdi/svg) (see
  [generated source](crates/mdi))
- [`@primer/octicons`](https://www.npmjs.com/package/@primer/octicons) (see
  [generated source](crates/octicons))
- [`@fortawesome/fontawesome-free`](https://www.npmjs.com/package/@fortawesome/fontawesome-free)
  (see [generated source](crates/fontawesome))
- [`simple-icons`](https://www.npmjs.com/package/simple-icons)
  ([generated source](crates/simple-icons/))

The generated packages are designed to expose most metadata alongside their SVG
sources.

[python-ci-badge]: https://github.com/2bndy5/redist-icons/actions/workflows/python.yml/badge.svg
[python-ci-link]: https://github.com/2bndy5/redist-icons/actions/workflows/python.yml
[rust-ci-badge]: https://github.com/2bndy5/redist-icons/actions/workflows/rust.yml/badge.svg
[rust-ci-link]: https://github.com/2bndy5/redist-icons/actions/workflows/rust.yml
