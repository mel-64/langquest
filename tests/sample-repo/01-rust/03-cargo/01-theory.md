# SHA-2 Hashing with Cargo

Cargo is Rust's build system and package manager. External libraries (crates) are declared in `Cargo.toml`:

```toml
[dependencies]
sha2 = "0.11"
```

Running `cargo build` downloads the crate and compiles it into your project.

## The `sha2` Crate

The `sha2` crate provides SHA-2 hash functions. `Sha256` produces a 256-bit (32-byte) digest:

```rust
use sha2::{Digest, Sha256};

let digest = Sha256::digest(b"hello");
```

`digest` returns an array of 32 bytes.

## Bytes to Hex

Each byte can be formatted as two hex digits with `{:x}`:

```rust
let byte: u8 = 171;
println!("{byte:x}"); // "ab"
```

Mapping over all bytes and joining the results builds the full hex string:

```rust
let hex: String = digest.map(|b| format!("{b:x}")).join("");
```
