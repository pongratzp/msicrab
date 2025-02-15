#[allow(dead_code)]
#[derive(thiserror::Error, Debug)]
pub enum CustomError {
    #[error("Generic {0}")]
    Generic(String),

    #[error("WMI connection error: {0}")]
    WMIConnectionError(#[from] wmi::WMIError),

    #[error("CSV error: {0}")]
    CSVError(#[from] csv::Error),

    #[error("Error Getting MSI information {0}")]
    MSIError(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}