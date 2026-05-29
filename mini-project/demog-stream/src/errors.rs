#[derive(Debug)]
pub enum DataError {
    IoError(std::io::Error),
    CsvError(csv::Error),
    InvalidNik(String)
}

impl std::fmt::Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DataError::IoError(err) => {
                write!(f, "Terjadi kesalahan I/O: {}", err)
            }
            DataError::CsvError(err) => {
                write!(f, "Terjadi kesalahan saat membaca CSV: {}", err)
            }
            DataError::InvalidNik(nik) => {
                write!(f, "NIK tidak valid: {}", nik)
            }
        }
    }
}

impl std::error::Error for DataError {}

// Implementasi From supaya error otomatis bisa dikonversi dengan operator ?
impl From<std::io::Error> for DataError {
    fn from(err: std::io::Error) -> Self {
        DataError::IoError(err)
    }
}

impl From<csv::Error> for DataError {
    fn from(err: csv::Error) -> Self {
        DataError::CsvError(err)
    }
}
