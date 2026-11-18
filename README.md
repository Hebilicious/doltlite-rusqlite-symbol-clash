# doltlite + rusqlite: one SQLite silently replaces the other

The [`doltlite`](https://crates.io/crates/doltlite) crate (0.51.13) compiles its own copy of every
`sqlite3_*` C symbol. [`rusqlite`](https://crates.io/crates/rusqlite) with `bundled` does the same
through `libsqlite3-sys`. Cargo accepts both crates in one binary, and the build and link succeed
with no warning. The linker keeps the copy of whichever archive comes first, and the other crate
then calls that copy.

In this program, rusqlite's SQLite wins: `doltlite::version()` reports `3.50.2` (rusqlite's
SQLite, not DoltLite's `3.55.0`) and every `dolt_*` function is missing.

## Run

```sh
cargo run --release
```

Output (Linux x86_64, rustc 1.99.0):

```
doltlite::version() = 3.50.2
rusqlite::version() = 3.50.2
SELECT dolt_version() failed: no such function: dolt_version (code 1)
```

The program exits with code 1.

Without rusqlite, the same program works:

```sh
cargo run --release --no-default-features
```

```
doltlite::version() = 3.55.0
SELECT dolt_version() = Some(Text("v0.51.13"))
```

The winner depends on link order, which follows the source. Swap the two `println!` lines in
`src/main.rs`, and DoltLite's engine wins instead: rusqlite then silently runs on DoltLite.

## Cause

`libsqlite3-sys` declares `links = "sqlite3"` in its `Cargo.toml`. The `doltlite` crate declares no
`links` key, so Cargo cannot see that both crates provide the same native library.

## Fix

Declare `links = "sqlite3"` in the `doltlite` crate. Cargo then refuses the pair when it resolves
dependencies:

```
package `libsqlite3-sys` links to the native library `sqlite3`, but it conflicts with a previous package which links to `sqlite3` as well
```

Branch with the fix and a test:
[Hebilicious/doltlite `fix/rust-crate-links-sqlite3`](https://github.com/Hebilicious/doltlite/tree/fix/rust-crate-links-sqlite3)
