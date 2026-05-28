# Proyek: Student Manager
Proyek ini adalah latihan dasar penggunaan crate `serde` dan `serde_json` di Rust untuk mengolah data json.
Program ini memproses data nilai siswa dengan alur berikut:
* Input: Membaca data awal dari file data/students.json.
* Proses: Menyaring siswa dengan nilai (score) lebih dari 80.0 menggunakan fungsi filter_top_student().
* Output: Menyimpan daftar siswa yang lolos seleksi ke dalam file data/result.json

## Cara Menjalankan
Pastikan Anda sudah menginstal Rust, lalu jalankan:
```bash
cargo run
