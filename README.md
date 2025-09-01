# redist-icons project

[![Python CI][python-ci-badge]][python-ci-link]
[![Rust CI][rust-ci-badge]][rust-ci-link]

<!-- markdownlint-disable MD033 -->

This repository contains (deno-flavored) TS scripts that generate Rust and
Python (via FFI bindings) packages from the following npm packages:

| npm package | Generated source | Rust package | Python package |
|:-:|:-:|:-:|:-:|
| [`@mdi/svg`][mdi-npm-link] | [crates/mdi](crates/mdi) | [![Crates.io Version][mdi-cargo-badge]][mdi-cargo-link]<br>![MSRV][msrv-badge] | [![PyPI - Version][mdi-pip-badge]][mdi-pip-link]<br>![Min Py][min-py] |
| [`@primer/octicons`][oct-npm-link] | [crates/octicons](crates/octicons) | [![Crates.io Version][oct-cargo-badge]][oct-cargo-link]<br>![MSRV][msrv-badge] | [![PyPI - Version][oct-pip-badge]][oct-pip-link]<br>![Min Py][min-py] |
| [`@fortawesome/fontawesome-free`][fa-npm-link] | [crates/fontawesome](crates/fontawesome) | [![Crates.io Version][fa-cargo-badge]][fa-cargo-link]<br>![MSRV][msrv-badge] | [![PyPI - Version][fa-pip-badge]][fa-pip-link]<br>![Min Py][min-py] |
| [`simple-icons`][si-npm-link] | [crates/simple-icons](crates/simple-icons/) | [![Crates.io Version][si-cargo-badge]][si-cargo-link]<br>![MSRV][msrv-badge] | [![PyPI - Version][si-pip-badge]][si-pip-link]<br>![Min Py][min-py] |

The generated packages are designed to expose most metadata alongside their SVG
sources.

[python-ci-badge]: https://github.com/2bndy5/redist-icons/actions/workflows/python.yml/badge.svg
[python-ci-link]: https://github.com/2bndy5/redist-icons/actions/workflows/python.yml
[rust-ci-badge]: https://github.com/2bndy5/redist-icons/actions/workflows/rust.yml/badge.svg
[rust-ci-link]: https://github.com/2bndy5/redist-icons/actions/workflows/rust.yml

[mdi-npm-link]: https://www.npmjs.com/package/@mdi/svg
[mdi-cargo-badge]: https://img.shields.io/crates/v/material-design-icons-pack
[mdi-cargo-link]: https://crates.io/crates/material-design-icons-pack
[mdi-pip-badge]: https://img.shields.io/pypi/v/material-design-icons-pack
[mdi-pip-link]: https://pypi.org/project/material-design-icons-pack/

[fa-npm-link]: https://www.npmjs.com/package/@fortawesome/fontawesome-free
[fa-cargo-badge]: https://img.shields.io/crates/v/fontawesome-free-pack
[fa-cargo-link]: https://crates.io/crates/fontawesome-free-pack
[fa-pip-badge]: https://img.shields.io/pypi/v/fontawesome-free-pack
[fa-pip-link]: https://pypi.org/project/fontawesome-free-pack/

[oct-npm-link]: https://www.npmjs.com/package/@primer/octicons
[oct-cargo-badge]: https://img.shields.io/crates/v/octicons-pack
[oct-cargo-link]: https://crates.io/crates/octicons-pack
[oct-pip-badge]: https://img.shields.io/pypi/v/octicons-pack
[oct-pip-link]: https://pypi.org/project/octicons-pack

[si-npm-link]: https://www.npmjs.com/package/simple-icons
[si-cargo-badge]: https://img.shields.io/crates/v/simple-icons-pack
[si-cargo-link]: https://crates.io/crates/simple-icons-pack
[si-pip-badge]: https://img.shields.io/pypi/v/simple-icons-pack
[si-pip-link]: https://pypi.org/project/simple-icons-pack/

[msrv-badge]: https://img.shields.io/badge/MSRV-1.85.0-blue
[min-py]: https://img.shields.io/badge/Python-v3.9+-blue
