mod models;
mod errors;

use models::{FinalReport, RecordCitizens, JenisMutasi};
use errors::DataError;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("\x1b[31mError:\x1b[0m Missing input and output paths.");
        eprintln!("\x1b[32mUsage:\x1b[0m cargo run -- <input_file.csv> <output_file.json>");
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    println!("Processing data from {}...", input_path);

    let processor = DataProcessor;
    let processor_result = processor.process_file(input_path);

    match processor_result {
        Ok(final_report) => {
            match serde_json::to_string_pretty(&final_report) {
                Ok(report_json) => {
                    if let Err(e) = std::fs::write(output_path, report_json) {
                        eprintln!("\x1b[31mError:\x1b[0m Gagal menulis file laporan: {}", e);
                    } else {
                        println!("\x1b[32mSukses!\x1b[0m Laporan berhasil disimpan ke {}", output_path);
                    }
                }

                Err(e) => {
                    eprintln!("\x1b[31mError:\x1b[0m Gagal mengonversi data ke JSON: {}", e);
                }
            }
        }

        Err(e) => {
            eprintln!("\x1b[31mError:\x1b[0m {}", e);
        }
    }
}

struct DataProcessor;

impl DataProcessor {
    fn process_file<P: AsRef<std::path::Path>>(&self, file_path: P) -> Result<FinalReport, DataError> {
        let mut total_processed = 0;
        let mut total_failed = 0;
        let mut destination_summary = std::collections::HashMap::new();
        let mut errors_log: Vec<String> = Vec::new();

        // Buka file dan inisialisasi CSV reader
        let mut reader = csv::Reader::from_path(file_path)?;

        // Loop baris per baris
        // Parse deserialize setiap baris menjadi RecordCitizens
        for result in reader.deserialize::<RecordCitizens>() {
            total_processed += 1;
            match result {
                Ok(record_citizens) => {
                    // Validasi panjang NIK == 16. Jika salah, total_failed += 1, lalu continue
                    if record_citizens.nik.len() != 16 {
                        total_failed += 1;
                        eprintln!(
                            "\x1b[31m[Gagal]\x1b[0m Warga bernama '{}' dari daerah '{}' memiliki NIK tidak valid ({})", 
                            record_citizens.nama, record_citizens.asal_daerah, record_citizens.nik
                        );
                        let pesan_error = DataError::InvalidNik(record_citizens.nik).to_string();
                        errors_log.push(pesan_error);
                        continue;
                    }

                    // Masukkan ke HashMap destination_summary
                    // Mengambil kecamatan tujuan sebagai KEY, dan menambah nilai VALUE (+1)
                    if record_citizens.jenis_mutasi == JenisMutasi::Pindah {
                        *destination_summary.entry(record_citizens.tujuan_daerah).or_insert(0) += 1;
                    }
                }

                Err(err) => {
                    total_failed += 1;
                    errors_log.push(err.to_string());
                    continue;
                }
            }

        }

        Ok(FinalReport { total_processed, total_failed, destination_summary, errors_log })
    }
}
