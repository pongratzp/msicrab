use crate::prelude::*;
use std::process::Command;

use super::common::Win32Product;

pub fn repair_msi(product: &Win32Product) -> Result<()> {
    let msi_path = product.package_cache.as_deref().unwrap();
    println!("Repairing: {}", msi_path);
    Command::new("msiexec.exe")
        .args(["/fa", msi_path])
        .status()?;
    Ok(())
}

pub fn start_procmon() -> Result<()> {
    Command::new(".\\Procmon64.exe")
        .spawn()?;
    Ok(())
}
