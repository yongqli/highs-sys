#[cfg(feature = "bundled")]
use std::env;
#[cfg(feature = "bundled")]
use std::fs::File;
#[cfg(feature = "bundled")]
use std::path::{Path, PathBuf};

#[cfg(feature = "bundled")]
use tempfile::TempDir;
#[cfg(feature = "bundled")]
use ureq;
#[cfg(feature = "bundled")]
use flate2::read::GzDecoder;
#[cfg(feature = "bundled")]
use tar::Archive;

/// Get the appropriate HiGHS binary URL based on the target architecture and OS
#[cfg(feature = "bundled")]
fn get_highs_binary_url() -> String {
    let target = env::var("TARGET").unwrap();
    let version = "v1.11.0%2B1";
    let base_url = "https://github.com/JuliaBinaryWrappers/HiGHS_jll.jl/releases/download";
    
    // Determine the binary name based on target triple
    let binary_name = match target.as_str() {
        "x86_64-apple-darwin" => "HiGHS.v1.11.0.x86_64-apple-darwin.tar.gz",
        "aarch64-apple-darwin" => "HiGHS.v1.11.0.aarch64-apple-darwin.tar.gz",
        "x86_64-unknown-linux-gnu" => "HiGHS.v1.11.0.x86_64-linux-gnu-cxx11.tar.gz",
        "x86_64-unknown-linux-musl" => "HiGHS.v1.11.0.x86_64-linux-musl-cxx11.tar.gz",
        "aarch64-unknown-linux-gnu" => "HiGHS.v1.11.0.aarch64-linux-gnu-cxx11.tar.gz",
        "aarch64-unknown-linux-musl" => "HiGHS.v1.11.0.aarch64-linux-musl-cxx11.tar.gz",
        "x86_64-pc-windows-msvc" | "x86_64-pc-windows-gnu" => "HiGHS.v1.11.0.x86_64-w64-mingw32-cxx11.tar.gz",
        "i686-pc-windows-msvc" | "i686-pc-windows-gnu" => "HiGHS.v1.11.0.i686-w64-mingw32-cxx11.tar.gz",
        "armv7-unknown-linux-gnueabihf" => "HiGHS.v1.11.0.armv7l-linux-gnueabihf-cxx11.tar.gz",
        "arm-unknown-linux-gnueabihf" => "HiGHS.v1.11.0.armv6l-linux-gnueabihf-cxx11.tar.gz",
        _ => {
            // Default to x86_64-linux-gnu if no specific match
            eprintln!("cargo:warning=Unsupported target '{}', defaulting to x86_64-linux-gnu", target);
            "HiGHS.v1.11.0.x86_64-linux-gnu.tar.gz"
        }
    };
    
    format!("{}/HiGHS-{}/{}", base_url, version, binary_name)
}

/// Download and extract a tar.gz file to the target directory
#[cfg(feature = "bundled")]
fn download_and_extract_tarball(url: &str, target_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:warning=Downloading HiGHS from: {}", url);
    
    // Download the file
    let response = ureq::get(url)
        .timeout(std::time::Duration::from_secs(300))
        .call()?;
    
    // Create temporary directory for download
    let temp_dir = TempDir::new()?;
    let temp_file_path = temp_dir.path().join("highs.tar.gz");
    
    // Write the response to a temporary file
    let mut temp_file = File::create(&temp_file_path)?;
    let mut reader = response.into_reader();
    std::io::copy(&mut reader, &mut temp_file)?;
    drop(temp_file);
    
    println!("cargo:warning=Extracting HiGHS to: {}", target_dir.display());
    
    // Extract the tar.gz file
    let tar_gz = File::open(&temp_file_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(target_dir)?;
    
    Ok(())
}

/// Download HiGHS precompiled binaries and extract them to OUT_DIR/highs_install
#[cfg(feature = "bundled")]
pub fn download_highs() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let install_path = PathBuf::from(&out_dir).join("highs_install");
    
    // Check if already downloaded
    if install_path.exists() {
        println!("cargo:warning=HiGHS already downloaded at: {}", install_path.display());
        return;
    }
    
    // Create the install directory
    std::fs::create_dir_all(&install_path).expect("Failed to create install directory");
    
    // Get the appropriate binary URL
    let url = get_highs_binary_url();
    
    // Download and extract
    match download_and_extract_tarball(&url, &install_path) {
        Ok(()) => {
            println!("cargo:warning=Successfully downloaded and extracted HiGHS to: {}", install_path.display());
        }
        Err(e) => {
            panic!("Failed to download HiGHS: {}", e);
        }
    }
}

#[cfg(not(feature = "bundled"))]
pub fn download_highs() {
    // No-op when bundled feature is not enabled
}