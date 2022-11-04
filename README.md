
Demonstrates how a library crate can access build time information about an end-user crate via a build script.

You can run the user binaries with 
```shell
cargo run --bin app_init
```
and
```shell
cargo run --bin app_main
```

## Crates
- `user`: The "user" of our library crate `build-ctx` which calls into `build-ctx-codegen` in its build script.
- `build-ctx`: The actual library crate that will provide most of the functionality to the dependent crate.
- `build-ctx-codegen`: The crate that the build script will call to do the code generation required for `build-ctx` to do its work.

## References 
- [Cargo Environment Variables](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts)
- [Cargo Build Scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html#build-scripts)
- [Cargo Workspace Dependencies](https://doc.rust-lang.org/cargo/reference/workspaces.html#the-workspacedependencies-table)