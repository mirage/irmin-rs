use std::path::PathBuf;

fn main() {
    let path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let opam_prefix = PathBuf::from(std::env::var("OPAM_SWITCH_PREFIX").unwrap());
    let libirmin_prefix = std::env::var("LIBIRMIN_PREFIX");

    let header = if let Ok(libirmin_prefix) = libirmin_prefix {
        let prefix = PathBuf::from(libirmin_prefix);
        println!("cargo:rustc-link-search={}", prefix.join("lib").display());
        println!(
            "cargo:rustc-link-arg=-Wl,-rpath,{}",
            prefix.join("lib").display()
        );
        prefix.join("include").join("irmin.h")
    } else if path.join("../libirmin.opam").exists() {
        // In repo
        std::process::Command::new("make")
            .arg("-C")
            .arg(path.join("..").as_os_str())
            .spawn()
            .unwrap();

        let lib = path
            .join("..")
            .join("_build")
            .join("default")
            .join("libirmin.so");

        std::process::Command::new("cp")
            .arg(lib.as_os_str())
            .arg(&path)
            .spawn()
            .unwrap();

        println!("cargo:rustc-link-search={}", path.display());
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", path.display());

        path.join("..")
            .join("_build")
            .join("default")
            .join("irmin.h")
    } else if PathBuf::from("_opam")
        .join("lib")
        .join("libirmin")
        .join("lib")
        .join("libirmin.so")
        .exists()
    {
        // Using opam
        println!(
            "cargo:rustc-link-arg=-Wl,-rpath,{}",
            PathBuf::from("_opam")
                .join("lib")
                .join("libirmin")
                .join("lib")
                .display()
        );
        println!(
            "cargo:rustc-link-search={}",
            PathBuf::from("_opam")
                .join("lib")
                .join("libirmin")
                .join("lib")
                .display()
        );
        PathBuf::from("_opam")
            .join("lib")
            .join("libirmin")
            .join("include")
            .join("irmin.h")
    } else if opam_prefix
        .join("lib")
        .join("libirmin")
        .join("lib")
        .join("libirmin.so")
        .exists()
    {
        // Using opam
        println!(
            "cargo:rustc-link-arg=-Wl,-rpath,{}",
            opam_prefix
                .join("lib")
                .join("libirmin")
                .join("lib")
                .display()
        );
        println!(
            "cargo:rustc-link-search={}",
            opam_prefix
                .join("lib")
                .join("libirmin")
                .join("lib")
                .display()
        );
        opam_prefix
            .join("lib")
            .join("libirmin")
            .join("include")
            .join("irmin.h")
    } else if opam_prefix.join("lib").join("libirmin.so").exists() {
        // Using opam prefix
        println!(
            "cargo:rustc-link-arg=-Wl,-rpath,{}",
            opam_prefix.join("lib").display()
        );
        println!(
            "cargo:rustc-link-search={}",
            opam_prefix.join("lib").display()
        );
        opam_prefix.join("include").join("irmin.h")
    } else {
        // Installed in $HOME/.local or /usr/local
        let home = std::env::var("HOME").unwrap_or_default();
        let user = PathBuf::from(home).join(".local");

        if user.join("include").join("irmin.h").exists() {
            println!("cargo:rustc-link-search={}", user.join("lib").display());
            user.join("include").join("irmin.h")
        } else {
            println!("cargo:rustc-link-search=/usr/local/lib");
            PathBuf::from("/usr/local/include/irmin.h")
        }
    };

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
