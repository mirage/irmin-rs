use std::path::PathBuf;

fn find_path<E: std::error::Error>(paths: Vec<Result<PathBuf, E>>) -> (PathBuf, PathBuf) {
    for path in paths {
        if let Ok(path) = path {
            let lib = path.join("lib").join("libirmin.so");
            let header = path.join("include").join("irmin.h");
            if lib.exists() && header.exists() {
                return (lib, header);
            }
        }
    }

    panic!("Unable to locate libirmin installation, try setting LIBIRMIN_PREFIX")
}

fn main() {
    let path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let opam_prefix = std::env::var("OPAM_SWITCH_PREFIX").map(PathBuf::from);
    let libirmin_prefix = std::env::var("LIBIRMIN_PREFIX").map(PathBuf::from);
    let local_opam = PathBuf::from("_opam").join("lib").join("libirmin");
    let home_local = std::env::var("HOME").map(|x| PathBuf::from(x).join(".local"));

    let (lib, header) = find_path(vec![
        Ok(path.join("..")),
        libirmin_prefix,
        opam_prefix,
        Ok(local_opam),
        home_local,
        Ok(PathBuf::from("/usr/local")),
    ]);

    println!(
        "cargo:rustc-link-arg=-Wl,-rpath,{}",
        lib.parent().unwrap().display()
    );
    println!(
        "cargo:rustc-link-search={}",
        lib.parent().unwrap().display()
    );
    println!("cargo:rustc-link-lib=irmin");
    println!("cargo:rerun-if-changed={}", header.display());

    let bindings = bindgen::builder()
        .header(header.to_str().unwrap())
        .allowlist_type("Irmin.*")
        .allowlist_function("irmin.*")
        .allowlist_function("caml.*")
        .generate()
        .unwrap();

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("c.rs")).unwrap();
}
