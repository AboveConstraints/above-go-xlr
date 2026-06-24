use clap::CommandFactory;
use clap_complete::{Shell, generate_to};
use std::env;
use std::fs::{self, File};
use std::io::Error;
use std::path::Path;
use std::process::Command;

#[cfg(target_os = "windows")]
use windres::Build;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    #[cfg(target_os = "windows")]
    {
        Build::new().compile("resources/goxlr-daemon.rc").unwrap();
    }

    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut app = Cli::command();
    for shell in Shell::value_variants() {
        let _ = generate_to(*shell, &mut app, "goxlr-daemon", &outdir)?;
    }

    let stamp_path = Path::new(&outdir).join("daemon-stamp");
    if let Err(err) = File::create(&stamp_path) {
        panic!("failed to write {}: {}", stamp_path.display(), err);
    }

    // ── Build the UI ──────────────────────────────────────────────────────────
    let manifest = env::var("CARGO_MANIFEST_DIR").unwrap();
    let root = Path::new(&manifest).parent().unwrap();
    let ui_dir = root.join("ui");
    let web_content = root.join("daemon").join("web-content");

    println!("cargo:rerun-if-changed=ui/src");
    println!("cargo:rerun-if-changed=ui/index.html");
    println!("cargo:rerun-if-changed=ui/package.json");
    println!("cargo:rerun-if-changed=ui/vite.config.js");

    if ui_dir.exists() {
        // Install deps if needed
        if !ui_dir.join("node_modules").exists() {
            println!("cargo:warning=Installing UI dependencies...");
            run_npm(&ui_dir, &["install"]);
        }

        println!("cargo:warning=Building UI...");
        run_npm(&ui_dir, &["run", "build"]);

        let dist = ui_dir.join("dist");
        if dist.exists() {
            copy_dir(&dist, &web_content);
        }
    }

    Ok(())
}

fn run_npm(dir: &Path, args: &[&str]) {
    let status = if cfg!(windows) {
        let mut cmd_args = vec!["/C", "npm"];
        cmd_args.extend_from_slice(args);
        Command::new("cmd").args(&cmd_args).current_dir(dir).status()
    } else {
        Command::new("npm").args(args).current_dir(dir).status()
    };
    match status {
        Ok(s) if s.success() => {}
        Ok(s) => panic!("npm {:?} exited with {}", args, s),
        Err(e) => panic!("failed to run npm: {}", e),
    }
}

fn copy_dir(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).unwrap();
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let dest = dst.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_dir(&entry.path(), &dest);
        } else {
            fs::copy(entry.path(), dest).unwrap();
        }
    }
}
