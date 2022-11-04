use std::{env, fs::OpenOptions, io::Write, path::PathBuf};

/// Generates the code required for `build-ctx`
///
/// This function should only be called in a cargo build script.
pub fn generate() {
    let name = env::var("CARGO_PKG_NAME").unwrap();
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let mut path: PathBuf = env::var("OUT_DIR").unwrap().parse().unwrap();
    path.push("build-ctx.rs");

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
        .unwrap();

    file.write_all(
        format!(
            "\
pub struct BuildCtx;
impl BuildCtx {{
    pub const fn name(&self) -> &'static str {{
        \"{name}\"
    }}

    pub const fn version(&self) -> &'static str {{
        \"{version}\"
    }}
}}"
        )
        .as_bytes(),
    )
    .unwrap();
}
