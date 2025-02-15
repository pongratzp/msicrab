use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::Serialize;

use crate::prelude::*;

use super::common::Win32Product;

pub fn write_csv_output<T: Serialize>(data: &Vec<T>, output: &str) -> Result<()> {
    let output_dir = PathBuf::from(output);
    fs::create_dir_all(&output_dir)?;
    let output_file = output_dir.join("list.csv");
    let mut wtr = csv::Writer::from_path(output_file)?;

    for record in data {
        wtr.serialize(record)?;
    }

    wtr.flush()?;
    Ok(())
}

pub fn copy_msi_file(product: &Win32Product, output: &str) -> Result<()> {
    let msi_path = Path::new(product.package_cache.as_deref().unwrap_or("Unknown"));
    let output_dir = PathBuf::from(output);
    fs::create_dir_all(&output_dir)?;

    let file_stem = msi_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Error getting file stem"))?;

    let output_file = output_dir.join(format!(
        "{}_{}.msi",
        product.caption.as_deref().unwrap_or("Unknown"),
        file_stem
    ));
    fs::copy(msi_path, output_file)?;
    Ok(())
}

pub fn display_products(products: &Vec<Win32Product>) {
    for product in products {
        println!("Caption: {}", product.caption.as_deref().unwrap_or("Unknown"));
        println!("Vendor: {}", product.vendor.as_deref().unwrap_or("Unknown"));
        println!("Package Name: {}", product.package_name.as_deref().unwrap_or("Unknown"));
        println!("Package Cache: {}", product.package_cache.as_deref().unwrap_or("Unknown"));
        println!();
    }
}