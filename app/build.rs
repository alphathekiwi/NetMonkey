fn main() {
    // Detect COSMIC environment during build
    detect_cosmic_environment();

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

/// Detect COSMIC environment and enable features accordingly
fn detect_cosmic_environment() {
    // Check for COSMIC desktop environment
    let is_cosmic = std::env::var("XDG_CURRENT_DESKTOP")
        .map(|desktop| desktop.contains("COSMIC"))
        .unwrap_or(false)
        || std::env::var("XDG_SESSION_DESKTOP")
            .map(|session| session.contains("cosmic"))
            .unwrap_or(false)
        || std::env::var("COSMIC_SESSION").is_ok();

    if is_cosmic {
        println!("cargo:rustc-cfg=cosmic_detected");
        println!(
            "cargo:warning=COSMIC desktop environment detected - consider enabling 'cosmic' feature"
        );
    }

    // Check if cosmic feature is explicitly enabled
    if cfg!(feature = "cosmic") {
        println!("cargo:rustc-cfg=cosmic_enabled");
        if is_cosmic {
            println!("cargo:warning=Building with COSMIC integration for COSMIC desktop");
        } else {
            println!(
                "cargo:warning=Building with COSMIC integration for non-COSMIC desktop (will fallback gracefully)"
            );
        }
    }

    // Help message for users
    if is_cosmic && !cfg!(feature = "cosmic") {
        println!("cargo:warning=Tip: Add --features cosmic to enable COSMIC desktop integration");
    }
}
