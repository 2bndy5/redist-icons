# material-design-icons-pack

[![Crates.io Version][mdi-cargo-badge]][mdi-cargo-link]
![MSRV][msrv-badge]
[![PyPI - Version][mdi-pip-badge]][mdi-pip-link]
![Min Py][min-py]

A redistribution of SVG assets and some metadata from the
[`@mdi/svg` npm package](https://www.npmjs.com/package/@mdi/svg).

## Optimized SVG data

The SVG data is embedded as strings after it is optimized with SVGO. This
package is intended to easily inject SVG data into HTML documents. Thus, we have
stripped any `width` and `height` fields from the `<svg>` element, while
retaining any `viewBox` field in the `<svg>` element.

## Usage

All icons are instantiated as constants using the `Icon` data structure.
There is a convenient `get_icon()` function to fetch an icon using it's slug name.

### In Python

```python
from material_design_icons_pack import get_icon, ALERT

fetched = get_icon("alert")
assert fetched is not None
assert ALERT.svg == fetched.svg
```

### In Rust

```rust
use material_design_icons_pack::{get_icon, ALERT};

assert_eq!(ALERT.svg, get_icon("alert").unwrap().svg);
```

## Rust Features

This crate has the following features:

- `deprecated-icons`: Include all icons marked as deprecated. Enabled by
  default.

The python binding does not support conditionally compiling certain icon
families. Note, in the python binding, a deprecation warning is emitted from
`get_icon()` when fetching a deprecated icon.

[mdi-cargo-badge]: https://img.shields.io/crates/v/material-design-icons-pack
[mdi-cargo-link]: https://crates.io/crates/material-design-icons-pack
[mdi-pip-badge]: https://img.shields.io/pypi/v/material-design-icons-pack
[mdi-pip-link]: https://pypi.org/project/material-design-icons-pack/

[msrv-badge]: https://img.shields.io/badge/MSRV-1.85.0-blue
[min-py]: https://img.shields.io/badge/Python-v3.9+-blue
