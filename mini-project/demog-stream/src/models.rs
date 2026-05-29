use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub struct FinalReport {
    pub total_processed: usize,
    pub total_failed: usize,
    pub destination_summary: std::collections::HashMap<String, u32>,
    pub errors_log: Vec<String>
}

#[derive(Debug, Deserialize)]
pub struct RecordCitizens {
    pub nik: String,
    pub nama: String,
    pub jenis_mutasi: JenisMutasi,
    pub asal_daerah: String,
    pub tujuan_daerah: String
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum JenisMutasi {
    Pindah,
    Datang
}
