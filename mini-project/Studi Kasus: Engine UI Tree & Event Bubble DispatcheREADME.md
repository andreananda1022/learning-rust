# Rust Smart Pointers: Simulasi Arsitektur UI Tree & Event Bubbling

Implementasi kode ini mengeksplorasi penggabungan berbagai *Smart Pointers* dalam bahasa pemrograman Rust. Proyek ini memodelkan struktur hierarki elemen antarmuka pengguna (UI) menyerupai DOM, yang mencakup mekanisme relasi dua arah, mutabilitas internal, dan sistem perambatan event (*event bubbling*).

---

## 1. Peta Penggunaan Smart Pointers

Arsitektur sistem ini memetakan tipe *Smart Pointer* berdasarkan kebutuhan manajemen memori dan perilaku akses data:

| Komponen / Properti | Jenis Pointer | Alasan Logis |
| --- | --- | --- |
| **`painter`** | `Box<dyn Painter>` | Menggunakan *trait object* di *heap* untuk mendukung polimorfisme perilaku visual kustom tanpa overhead manajemen referensi ganda. |
| **`children`** | `Rc<RefCell<Node>>` | Membentuk kepemilikan ganda (*multiple ownership*). Induk memiliki hak kepemilikan mutlak atas anak-anaknya, dipadukan dengan `RefCell` agar struktur internalnya dapat dimutasi saat runtime. |
| **`parent`** | `Weak<RefCell<Node>>` | Referensi anak ke induk yang bersifat **lemah** (*non-owning*). Digunakan secara mutlak untuk mencegah terjadinya siklus referensi (*reference cycle / memory leak*). |

---

## 2. Bedah Logika Inti

### A. Pola Kombinasi `Rc<RefCell<T>>`

* `Rc` mengizinkan sebuah elemen anak diakses oleh beberapa pihak secara bersamaan dalam satu *thread*.
* `RefCell` menyediakan *Interior Mutability* (mutabilitas internal). Tanpa ini, kita tidak akan bisa memodifikasi vektor `children` atau mengubah status `parent` dari sebuah node karena referensi induknya dibagikan secara imutabel.

### B. Mencegah Kebocoran Memori dengan `Weak<T>`

Jika hubungan anak ke induk menggunakan `Rc` biasa, memori induk dan anak akan saling menyandera selamanya (*strong count* tidak pernah mencapai 0). Dengan menggunakan `Weak`:

1. Anak mengetahui siapa induknya melalui fungsi `.upgrade()`.
2. Hubungan ini tidak menaikkan penghitung kepemilikan utama (*strong count*).
3. Ketika induk dicabut dari hierarki, memori di *heap* dapat dibersihkan dengan bersih oleh sistem.

### C. Rekursi Event Bubbling

Fungsi `trigger_event` mendemonstrasikan perambatan sinyal dari elemen terdalam (seperti *Button*) naik ke elemen di atasnya (*Panel* $\rightarrow$ *Window*). Pemanggilan rekursi berhenti secara otomatis ketika elemen puncak (*Root*) mencapai kondisi `None` pada evaluasi pointer `Weak`-nya.

---

## 3. Hasil Eksekusi Program

```text
=== 1. Membangun Hierarki UI ===
Hierarki berhasil dibangun.

=== 2. Simulasi Klik dari User ===
Elemen 'Button Kirim' memproses event: onClick
Elemen 'Panel Utama' memproses event: onClick
Elemen 'Window (Root)' memproses event: onClick
-- Perambatan event berhenti (Mencapai root / parent dihapus).

=== 3. Pengujian Memori (Menghancurkan Hierarki) ===
Mengklik tombol yatim piatu:
Elemen 'Button Kirim' memproses event: onClick
-- Perambatan event berhenti (Mencapai root / parent dihapus).

```
