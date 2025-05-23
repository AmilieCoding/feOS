# Getting feOS running as a developer:
1. Clone this repo.
2. CD into your new folder.
3. Install cargo AND rustup.
4. `rustup override set nightly`
5. `cargo build --target x86_64-feos.json`

If that doesn't work you'll have to add it to Rust itself.
1. `rustup target add x86_64-feos`
2. Then try `cargo build --target x86_64-feos.json`

6. Now this works, do `cargo build` to build and `cargo run` and `cargo test` respectively.
