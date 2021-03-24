#[allow(unused)]
fn link() {
    use std::env;
    use std::process::Command;

    let out_dir = env::var("OUT_DIR").unwrap();
    let dune_dir = "_build/default/src";
    assert!(Command::new("dune")
        .args(&["clean"])
        .status()
        .expect("Dune failed")
        .success());

    assert!(Command::new("dune")
        .args(&["build", "src/irmin_rs.so", "--force"])
        .status()
        .expect("Dune failed")
        .success());
    Command::new("cp")
        .args(&[
            &format!("{}/irmin_rs.so", dune_dir),
            &format!("{}/libirmin_rs.so", out_dir),
        ])
        .status()
        .expect("File copy failed.");
    /*Command::new("rm")
        .args(&["-f", &format!("{}/libirmin_rs.a", out_dir)])
        .status()
        .expect("rm failed");
    Command::new("ar")
        .args(&[
            "qs",
            &format!("{}/libirmin_rs.a", out_dir),
            &format!("{}/libirmin_rs.o", out_dir),
        ])
        .status()
        .expect("ar failed");*/

    println!("cargo:rustc-link-search={}", out_dir);

    println!("cargo:rustc-link-search=.");
    println!("cargo:rustc-link-lib=irmin_rs");
}

fn main() {
    #[cfg(feature = "bindings")]
    link()
}
