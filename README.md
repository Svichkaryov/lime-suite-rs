# lime-suite

This crate provides a safe, idiomatic Rust interface to [LimeSuite](https://github.com/myriadrf/LimeSuite) API.

Linux only.

## Usage

Add dependency to your *Cargo.toml*:

```toml
[dependencies]
lime-suite = { git = "https://github.com/Svichkaryov/lime-suite.git" }
```

Or in the case of a local repository:

```toml
[dependencies]
lime-suite = { path = "<repo_path>/lime-suite" }
```

**Environment variables**:

- `LIBLIMESUITE_PATH` - path to the directory containing the `libLimeSuite.so` library. This is used when you need to build the library in your own way (with your flags or during cross-compilation, when the default logic is insufficient).
- `LIBLIMESUITE_NO_VENDOR` - set to `1` to use a system-installed library via `pkg-config` instead of building from source.
- `LIMESUITE_TOOLCHAIN` - path to a CMake toolchain file. Overrides the `CMAKE_TOOLCHAIN_FILE_<TARGET>` variable.

Indirect:

- `DEP_LIMESUITE_ROOT` - build output path. Also used to locate `libLimeSuite.so` (available when depending directly on `lime-suite-sys` due to the lack of transitive access to the `DEP_<name>_<key>` variables)

**Dependencies**:

- `libusb-1.0-0-dev`

**Build**

By default, `libLimeSuite.so` is built from the vendored sources in `lime-suite-sys/vendor`:

```bash
$ cargo build -r
```

To use a system-installed `libLimeSuite.so` set `LIBLIMESUITE_NO_VENDOR=1`:

```bash
$ export PKG_CONFIG_PATH=<path>
$ LIBLIMESUITE_NO_VENDOR=1 cargo build -r
```

Specifying the library path manually:

```bash
$ LIBLIMESUITE_PATH=<library path> cargo build -r
```

**Running**

Running via `cargo run` works without additional setup.

When running the binary directly, use a **build script** to copy `libLimeSuite.so` to the output directory:

```rust
// build.rs

use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn copy_libs_to_target_dir(target_path: &Path, libs_path: &Path) {
    let read_dir = std::fs::read_dir(libs_path).unwrap();

    for path in read_dir {
        let entry = path.unwrap();
        if entry
            .file_name()
            .into_string()
            .unwrap()
            .starts_with("libLimeSuite")
        {
            fs::copy(entry.path(), target_path.join(entry.file_name())).unwrap();
        }
    }
}

fn main() {
    let target = env::var("TARGET").unwrap();

    if !target.contains("windows") {
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
        println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/lime-util");
    }

    if let Some(root) = env::var_os("DEP_LIMESUITE_ROOT") {
        let lib_lime_suite_path = PathBuf::from(root).join("lib");

        let out_dir = env::var("OUT_DIR").unwrap();
        let target_root_path = Path::new(&out_dir).ancestors().nth(3).unwrap();

        copy_libs_to_target_dir(target_root_path, &lib_lime_suite_path);
    }
}
```

You also need to add `lime-suite-sys` to your `Cargo.toml`:

```toml
[dependencies]
lime-suite-sys = { git = "https://github.com/Svichkaryov/lime-suite.git" }
```
