# Test project for comparing inlining and linking behaviour in Rust.

First install cargo-show-asm with disasm enabled:

```
cargo install cargo-show-asm --features disasm
```

To check asm with different linker settings try the following commands.

lto=false

```
CARGO_PROFILE_RELEASE_LTO=false cargo asm -p mathbin --disasm _add
```

lto=thin

```
CARGO_PROFILE_RELEASE_LTO=thin cargo asm -p mathbin --disasm _add
```

lto=fat

```
CARGO_PROFILE_RELEASE_LTO=fat cargo asm -p mathbin --disasm _add
```
