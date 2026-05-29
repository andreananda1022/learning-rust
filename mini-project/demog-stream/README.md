# Proyek: Demografi Stream

`demog-stream` adalah perkakas berbasis *Command Line Interface* (CLI) yang dibangun dengan bahasa pemrograman **Rust**. Aplikasi ini dirancang khusus untuk membaca, memvalidasi, dan merekapitulasi data mutasi kependudukan (warga pindah dan datang) dari berkas mentah CSV menjadi laporan statistik terstruktur dalam format JSON. 

Perkakas ini dikembangkan dengan fokus pada performa tinggi, efisiensi memori, dan keandalan sistem tingkat tinggi (*zero panic*), menjadikannya ideal untuk otomatisasi administrasi pelayanan publik di tingkat kelurahan maupun kecamatan.

---

## 🚀 Fitur Utama

1. **Pemrosesan Aliran Data (Streaming Processing):** Memanfaatkan *streaming iterator* dari `csv` crate dan `serde` untuk memproses data baris demi baris. Aplikasi ini tetap hemat RAM (alokasi memori konstan) meskipun memproses jutaan baris data kependudukan.
2. **Arsitektur Zero-Panic:** Penanganan eror yang sangat ketat menggunakan sistem tipe `Result` dan `Option` di Rust. Aplikasi tidak akan *crash* saat menemui data kotor, kolom kosong, atau format CSV yang cacat.
3. **Validasi Logika Bisnis Otomatis:**
   * Memastikan panjang Nomor Induk Kependudukan (NIK) tepat 16 digit.
   * Melakukan pemfilteran otomatis berdasarkan jenis mutasi (hanya merekapitulasi data warga yang *Datang*).
4. **Agregasi Berperforma Tinggi:** Menggunakan **Entry API** pada `HashMap` untuk menghitung total distribusi warga per daerah tujuan secara efisien dengan kompleksitas waktu $O(1)$ per baris.

---

## 📁 Struktur Kode Program

Proyek ini menggunakan pemisahan kode modular untuk mempermudah pemeliharaan jangka panjang:

* **`src/errors.rs`**: Berisi kustomisasi penanganan eror (`enum DataError`). Mengimplementasikan *trait* `std::error::Error` dan `From` untuk mengonversi eror I/O dan CSV secara otomatis menggunakan operator `?`.
* **`src/models.rs`**: Berisi definisi struktur data utama (`RecordCitizens` dan `FinalReport`) serta struktur data enumerasi (`JenisMutasi`) yang dilengkapi dengan kemampuan deteksi otomatis dari *Serde*.
* **`src/main.rs`**: Berisi logika bisnis inti pemrosesan berkas, validasi aliran data, manajemen argumen CLI, serta mekanisme ekspor laporan ke format JSON.

---

## 📊 Spesifikasi Format Berkas

### 1. Berkas Input (`.csv`)
Berkas input wajib berupa CSV ber-header dengan urutan dan nama kolom sebagai berikut:
