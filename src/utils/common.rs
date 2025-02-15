use crate::prelude::*;
use wmi::{COMLibrary, WMIConnection};
use serde::Deserialize;
use windows::Win32::Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
use windows::Win32::System::Threading::GetCurrentProcess;
use windows::Win32::System::Threading::OpenProcessToken;

#[derive(Deserialize, Debug)]
#[derive(serde::Serialize)]
#[serde(rename = "Win32_Product")]
#[serde(rename_all = "PascalCase")]
pub struct Win32Product {
    pub caption: Option<String>,
    pub package_name: Option<String>,
    pub vendor: Option<String>,
    pub package_cache: Option<String>,
}


pub fn get_installed_msis(filter: Option<Vec<String>>) -> Result<Vec<Win32Product>> {
    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con)?;

    let results: Vec<Win32Product> = wmi_con.query()?;

    if let Some(ref vector) = filter {
        let filtered_results: Vec<Win32Product> = results.into_iter()
        .filter(|product| product.vendor.as_deref().map_or(false, |vendor| !vector.contains(&vendor.to_string())))
        .collect();

    Ok(filtered_results)
    } else {
        Ok(results)
    }

    
}

pub fn is_elevated() -> windows::core::Result<bool> {
    let process_handle = unsafe { GetCurrentProcess() };
    let mut token_handle = windows::Win32::Foundation::HANDLE::default();

    unsafe {
        OpenProcessToken(process_handle, TOKEN_QUERY, &mut token_handle)?;
    }

    let mut elevation = TOKEN_ELEVATION::default();
    let mut return_length = 0;

    unsafe {
        GetTokenInformation(
            token_handle,
            TokenElevation,
            Some(&mut elevation as *mut _ as *mut _),
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut return_length,
        )?;
    }

    Ok(elevation.TokenIsElevated != 0)
}