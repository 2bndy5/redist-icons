# simple-icons-pack

A redistribution of SVG assets and some metadata from the
[`simple-icons` npm package](https://www.npmjs.com/package/simple-icons).

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
from simple_icons_pack import get_icon, SI_GITHUB

fetched = get_icon("github")
assert fetched is not None
assert SI_GITHUB.svg == fetched.svg
```

### In Rust

```rust
use simple_icons_pack::{get_icon, SI_GITHUB};

assert_eq!(SI_GITHUB.svg, get_icon("github").unwrap().svg);
```
