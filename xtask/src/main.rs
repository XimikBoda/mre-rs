use serde::Deserialize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, exit};

#[derive(Deserialize)]
struct CargoManifest {
    package: Package,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    metadata: Option<Metadata>,
}

#[derive(Deserialize)]
struct Metadata {
    mre: Option<MreConfig>,
}

#[derive(Deserialize)]
struct MreConfig {
    #[serde(rename = "type")]
    app_type: Option<String>,
    ram: u32,
    app_name: Option<String>,
    developer: Option<String>,
    background: Option<bool>,
    api: Option<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_simulator = args.contains(&"--sim".to_string());
    let is_debug = args.contains(&"--debug".to_string());
    let is_thumb = args.contains(&"--thumb".to_string());
    
    let sdk_path = env::var("TinyMRESDK").unwrap_or_else(|_| {
        eprintln!("Error: TinyMRESDK environment variable is not set.");
        exit(1);
    });

    let target_crate = args.iter().find(|arg| !arg.starts_with('-') && *arg != "build" && !arg.contains("xtask"));
    let current_dir = env::current_dir().expect("Failed to get current working directory");
    let packages = find_mre_packages(&current_dir, target_crate.map(|s| s.as_str()));

    if packages.is_empty() {
        if let Some(name) = target_crate {
            eprintln!("Error: Could not find an MRE package named '{}'.", name);
        } else {
            eprintln!("Error: No MRE packages found in the workspace.");
        }
        exit(1);
    }

    if target_crate.is_none() {
        println!("No specific package provided. Building all discovered MRE packages...");
    }

    for pkg_dir in packages {
        build_package(&pkg_dir, is_simulator, is_debug, is_thumb, &sdk_path);
    }
}

fn find_mre_packages(dir: &Path, specific_name: Option<&str>) -> Vec<PathBuf> {
    let mut found = Vec::new();
    
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                let dir_name = path.file_name().unwrap().to_string_lossy();
                
                if dir_name == "target" || dir_name.starts_with('.') {
                    continue;
                }

                let cargo_toml = path.join("Cargo.toml");
                if cargo_toml.exists() {
                    if let Ok(manifest_str) = fs::read_to_string(&cargo_toml) {
                        if let Ok(manifest) = toml::from_str::<CargoManifest>(&manifest_str) {
                            if manifest.package.metadata.and_then(|m| m.mre).is_some() {
                                if let Some(target_name) = specific_name {
                                    if manifest.package.name == target_name {
                                        found.push(path.clone());
                                    }
                                } else {
                                    found.push(path.clone());
                                }
                            }
                        }
                    }
                } else {
                    found.append(&mut find_mre_packages(&path, specific_name));
                }
            }
        }
    }
    found
}

fn build_package(app_dir: &Path, is_simulator: bool, is_debug: bool, is_thumb: bool, sdk_path: &str) {
    let manifest_path = app_dir.join("Cargo.toml");
    let manifest_str = fs::read_to_string(&manifest_path).expect("Failed to read Cargo.toml");
    let manifest: CargoManifest = toml::from_str(&manifest_str).expect("Failed to parse Cargo.toml");
    
    let mre_config = manifest.package.metadata
        .and_then(|m| m.mre)
        .expect("Error: [package.metadata.mre] section not found in Cargo.toml");

    let crate_name = &manifest.package.name;
    println!("\n--- Building package: {} ---", crate_name);

    let app_type = mre_config.app_type.unwrap_or_else(|| "vxp".to_string());
    let app_name = mre_config.app_name.unwrap_or(crate_name.clone());
    let developer = mre_config.developer.unwrap_or_else(|| "Unknown".to_string());
    let api_permissions = mre_config.api.unwrap_or_default();
    let is_bg = mre_config.background.unwrap_or(false);

    let target = if is_simulator { "i686-pc-windows-msvc" } else if is_thumb { "thumbv5te-none-eabi" } else {"armv5te-none-eabi"};
    println!("Compiling Rust code for target: {}", target);

    let mut cargo_args = vec![
        "+nightly-2025-09-18".to_string(),
        "build".to_string(),
        "--package".to_string(), crate_name.to_string(),
        "--target".to_string(), target.to_string(),
    ];

    if !is_debug {
        cargo_args.push("--release".to_string());
    }

    if is_simulator {
        cargo_args.push("--lib".to_string());
    } else {
        cargo_args.push("--bin".to_string());
        cargo_args.push(crate_name.to_string());
    }

    cargo_args.push("-Z".to_string());
    cargo_args.push("build-std=core,alloc".to_string());
    cargo_args.push("-Z".to_string());
    cargo_args.push("build-std-features=compiler-builtins-mem".to_string());

    let mut rustflags = String::from("-C panic=abort -C force-frame-pointers=yes");

    if !is_simulator {
        rustflags.push_str(" -C relocation-model=pic -C link-arg=-pie");

        rustflags.push_str(" --cfg portable_atomic_unsafe_assume_single_core");
        rustflags.push_str(" --cfg getrandom_backend=\"custom\"");
    }

    let status = Command::new("cargo")
        .env("RUSTFLAGS", rustflags)
        .args(&cargo_args)
        .status()
        .expect("Failed to execute cargo build process");

    if !status.success() {
        eprintln!("Error: Rust compilation failed for {}.", crate_name);
        exit(1);
    }

    let profile_dir = if is_debug { "debug" } else { "release" };

    let binary_ext = if is_simulator { ".dll" } else { "" };
    let compiled_bin = format!("target/{}/{}/{}{}", target, profile_dir, crate_name, binary_ext);
    
    let res_dir = app_dir.join("res");
    let res_out = format!("target/{}.res", crate_name);
    
    let final_ext = match (app_type.as_str(), is_simulator) {
        ("vsm", true) => "dlm",
        ("vsm", false) => "vsm",
        (_, true) => "vc.vxp",
        (_, false) => "vxp",
    };
    let final_binary = format!("target/{}.{}", crate_name, final_ext);

    // 1. Pack Resources
    if !pack_resources(&res_dir, &res_out, sdk_path) {
        eprintln!("Error: Resource packing failed.");
        exit(1);
    }

    // 2. Pack Final Application/Library
    if !pack_application(
        sdk_path,
        &compiled_bin,
        &res_out,
        &final_binary,
        &app_type,
        0,
        mre_config.ram,
        &app_name,
        &developer,
        is_bg,
        &api_permissions,
    ) {
        eprintln!("Error: Binary packing failed.");
        exit(1);
    }

    println!("Success: Package '{}' built successfully at {}", crate_name, final_binary);
}

fn pack_resources(res_dir: &Path, res_out: &str, sdk_path: &str) -> bool {
    println!("Packing resources (invoking PackRes)...");
    
    let abs_res_out = env::current_dir()
        .expect("Failed to get current directory")
        .join(res_out)
        .to_string_lossy()
        .into_owned();

    let mut args = vec![
        "-o".to_string(), abs_res_out,
    ];

    let mut files = Vec::new();
    
    if res_dir.exists() {
        if let Ok(entries) = fs::read_dir(res_dir) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_file() {
                    files.push(path.file_name().unwrap().to_string_lossy().into_owned());
                }
            }
        }
    } else {
        println!("Warning: res/ directory not found. Generating an empty valid .res file.");
    }

    if !files.is_empty() {
        args.push("-r".to_string());
        args.extend(files);
    }

    args.push("--empty-logo".to_string());

    let mut cmd = Command::new(format!("{}/bin/PackRes", sdk_path));

    if res_dir.exists() {
        cmd.current_dir(res_dir);
    }

    let status = cmd.args(&args).status().expect("Failed to execute PackRes");

    status.success()
}

fn pack_application(
    sdk_path: &str,
    compiled_bin: &str,
    res_out: &str,
    final_binary: &str,
    app_type: &str,
    app_id: u32,
    ram: u32,
    app_name: &str,
    developer: &str,
    is_bg: bool,
    api_permissions: &str,
) -> bool {
    println!("Packing final binary: {}", final_binary);
    
    let bg_flag = if is_bg { "1" } else { "0" };
    
    let mut pack_args = vec![
        "-a".to_string(), compiled_bin.to_string(),
        "-r".to_string(), res_out.to_string(),
        "-o".to_string(), final_binary.to_string(),
        "-tr".to_string(), ram.to_string(),
        "-tn".to_string(), app_name.to_string(),
        "-tdn".to_string(), developer.to_string(),
        "-tai".to_string(), app_id.to_string(),
        "-tb".to_string(), bg_flag.to_string(),
    ];

    if app_type == "vsm" {
        pack_args.push("-ty".to_string());
        pack_args.push("vsm".to_string());
    }

    if !api_permissions.is_empty() {
        pack_args.push("-tapi".to_string());
        pack_args.push(api_permissions.to_string());
    }

    if let Ok(imsi_val) = env::var("MRE_IMSI") {
        println!("Applying IMSI configuration.");
        pack_args.push("-ti".to_string());
        pack_args.push(imsi_val);
    }

    let pack_status = Command::new(format!("{}/bin/PackApp", sdk_path))
        .args(&pack_args)
        .status()
        .expect("Failed to execute PackApp");

    pack_status.success()
}