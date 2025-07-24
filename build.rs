fn main() {
    #[cfg(target_os = "windows")]
    {
        #[cfg(target_env = "msvc")]
        {
            println!("cargo:rustc-link-arg=/stack:{}", 8 * 1024 * 1024);
        }

        let icon = std::path::Path::new("assets/net_monkey.ico");
        println!("cargo:rerun-if-changed={}", icon.display());

        let mut res = winresource::WindowsResource::new();

        // Depending on the security applied to the computer, winresource might fail
        // fetching the RC path. Therefore, we add a way to explicitly specify the
        // toolkit path, allowing winresource to use a valid RC path.
        if let Ok(explicit_rc_toolkit_path) = std::env::var("RC_TOOLKIT_PATH") {
            res.set_toolkit_path(explicit_rc_toolkit_path.as_str());
        }
        res.set_icon(icon.to_str().unwrap());
        res.set("FileDescription", "Net Monkey");
        res.set("ProductName", "Net Monkey");

        if let Err(e) = res.compile() {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}
