

fn main() {
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let lock_path = std::path::Path::new(&out_dir).join("vtid.lock");
    let lock_path = lock_path.to_str().expect("Lock path is not UTF-8").to_string();

    println!("cargo::rustc-env=VTID_PROC_MACRO_LOCK_FILE_PATH={}", lock_path);
}
