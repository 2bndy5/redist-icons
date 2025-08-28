# crates

This directory contains the distributed packages in this monorepo (workspace).

Each crate is designed to only require maintaining the main.ts script. All other
sources should be considered static (eg. src/lib.rs and various metadata).

To regenerate the crates' source, use deno `tasks` as defined in
crates/\*/deno.json.

```shell
deno task gen
```

> [!TIP]
> The root deno.json file has `tasks` to facilitate running crate-specific deno
> `tasks` from the repo root.
