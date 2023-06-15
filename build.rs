fn main() {
    #[cfg(target_os = "linux")]
    {
        let python_pkg = pkg_config::Config::new()
            .probe("python3")
            .expect("Failed to find python3 via pkg-config");

        for lib in python_pkg.libs {
            println!("cargo:rustc-link-lib={}", lib);
        }

        for include_path in python_pkg.include_paths {
            println!("cargo:rustc-link-search=native={}", include_path.to_string_lossy());
        }
    }
}
