use std::os::unix::fs::symlink;
use std::{env, fs, path::PathBuf};

use pkg_config::Library;

fn lib_probe(name: &str) -> Library {
    pkg_config::Config::new()
        .cargo_metadata(true)
        .print_system_libs(false)
        .probe(name)
        .unwrap()

    // if let Err(e) = pkg_config::Config::new()
    //     .cargo_metadata(true)
    //     .print_system_libs(false)
    //     .probe("LimeSuite")
    // {
    //     println!("cargo:warning = {e}");
    //     println!(
    //         "cargo:warning=Could not find LimeSuite via pkg-config: {e}"
    //     )
    // }
}

fn copy_libs_to_out_dir(out_path: PathBuf, libs_path: PathBuf) {
    let read_dir = std::fs::read_dir(libs_path).unwrap();

    for path in read_dir {
        let entry = path.unwrap();
        if entry
            .file_name()
            .into_string()
            .unwrap()
            .starts_with("libLimeSuite")
        {
            let entry_path = entry.path();
            let dst = out_path.clone().join(entry.file_name());

            if entry_path.is_symlink() {
                let original = fs::read_link(&entry_path).unwrap();
                symlink(original, dst).unwrap();
            } else {
                fs::copy(entry_path, dst).unwrap();
            }
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let _host = env::var("HOST").unwrap();
    let _target = env::var("TARGET").unwrap();

    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let lib_out = dst.join("lib");

    fs::create_dir_all(lib_out.clone()).unwrap();

    let lib_name = "LimeSuite";

    let lib_path = if let Ok(lib_path) = env::var("LIBLIMESUITE_PATH") {
        println!("cargo:rustc-link-search=native={lib_path}");
        println!("cargo:rustc-link-lib={lib_name}");
        println!("cargo:root={}", dst.to_str().unwrap());

        PathBuf::from(lib_path)
    } else if env::var("LIBLIMESUITE_NO_VENDOR").unwrap_or_default() == "1" {
        let lib = lib_probe(lib_name);
        println!("cargo:root={}", dst.to_str().unwrap());

        lib.link_paths[0].clone()
    } else {
        // cargo:root come with cmake build
        let mut cfg = cmake::Config::new("vendor");

        if let Ok(toolchain) = env::var("LIMESUITE_TOOLCHAIN") {
            cfg.define("CMAKE_TOOLCHAIN_FILE", &toolchain);
        };

        let dst = cfg.build();
        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("lib").display()
        );
        println!("cargo:rustc-link-lib={lib_name}");

        dst
    };

    copy_libs_to_out_dir(lib_out.clone(), lib_path);
}
