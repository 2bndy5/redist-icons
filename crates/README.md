# crates

This directory contains the distributed packages in this monorepo (workspace).

Each crate is designed to only require maintaining the main.ts script. All other
sources should be considered static (eg. src/lib.rs and various metadata).

To regenerate the crates' source, use bun scripts as defined in
crates/*/package.json.

```shell
bun run gen
```

> [!TIP]
> The root package.json file has scripts to facilitate running crate-specific bun
> scripts from the repo root.
