# octicons-pack

[![Crates.io Version][oct-cargo-badge]][oct-cargo-link]
![MSRV][msrv-badge]
[![PyPI - Version][oct-pip-badge]][oct-pip-link]
![Min Py][min-py]

A redistribution of SVG assets and some metadata from the
[`@primer/octicons` npm package](https://www.npmjs.com/package/@primer/octicons).

## Optimized SVG data

The SVG data is embedded as strings after it is optimized with SVGO. This
package is intended to easily inject SVG data into HTML documents. Thus, we have
stripped any `width` and `height` fields from the `<svg>` element, while
retaining any `viewBox` field in the `<svg>` element.

## Usage

All icons are instantiated as constants using the `Icon` data structure.
There is a convenient `get_icon()` function to fetch an icon using it's slug name.

Note, most icons have `*_16` or `*_24` variants to indicate the
original height and width.

### In Python

```python
from octicons_pack import get_icon, GIT_BRANCH_24

fetched = get_icon("git-branch-24")
assert fetched is not None
assert GIT_BRANCH_24.svg == fetched.svg
```

### In Rust

```rust
use octicons_pack::{get_icon, GIT_BRANCH_24};

assert_eq!(GIT_BRANCH_24.svg, get_icon("git-branch-24").unwrap().svg);
```

[oct-cargo-badge]: https://img.shields.io/crates/v/octicons-pack
[oct-cargo-link]: https://crates.io/crates/octicons-pack
[oct-pip-badge]: https://img.shields.io/pypi/v/octicons-pack
[oct-pip-link]: https://pypi.org/project/octicons-pack

[msrv-badge]: https://img.shields.io/badge/MSRV-1.85.0-blue
[min-py]: https://img.shields.io/badge/Python-v3.9+-blue
