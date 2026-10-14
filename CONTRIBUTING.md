# Contribution guidelines

This is a bun-first project. Most of the rust source code is generated from metadata and SVG assets in the npm packages that this project uses (see [README](./README.md)).

The code that generates the sources is kept in the "common/*.ts" and "crates/*/main.ts" files.

To regenerate most Rust sources, there is a bun script declared for convenience:

```shell
bun run gen
```

To only generate a certain package sources, there are bun scripts for that too:

```shell
# generate fontawesome package source
bun run gen:fa

# generate material-design-icons package source
bun run gen:mdi

# generate octicons package source
bun run gen:oct

# generate simple-icons package source
bun run gen:si
```

## Unwanted changes

Any patch that directly changes a generated file will be rejected.

> [!TIP]
> Any source file that was generated will have a comment at the top stating as much:
>
> ```rs
> // This file was generated. DO NOT EDIT.
> ```

Instead, changes should be made to the typescript source that does the actual generation of source code.

## Code format

Each rust source is formatted (using `cargo fmt`) when package generation is completed.
Afterward, `cargo clippy` can be used to lint all rust sources.

Biome (`bun run format` or `bun run format:check`) is used to format JSON and TS/JS files.

For Python sources (including type stubs), `ruff` and `mypy` are used. These tools can be invoked using `uv run`. Use `uv sync` to ensure these tools are installed.

```shell
uv run ruff format
uv run ruff check
uv run mypy
```

### Partitioned code

Long modules and function are partitioned because `rustfmt` tends to suffer a stack overflow error when processing very long files.
This can also expedite compilation time on slower machines.
