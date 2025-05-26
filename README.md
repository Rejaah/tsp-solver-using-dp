# Travelling Salesman Problem Solver Using Dynamic Programming

## Deskripsi
Program ini menyelesaikan permasalahan *Travelling Salesman Problem (TSP)* untuk graf **berarah** menggunakan algoritma **Dynamic Programming**. Solusi yang dihasilkan adalah **biaya minimum**, **rute perjalanan optimal** yang kembali ke titik awal, dan **waktu eksekusi**.

---

## 🛠️ Requirement

| Komponen       | Versi Minimal |
|----------------|---------------|
| 🦀 Rust        | 1.60+         |
| 📦 Cargo (untuk build) | Disertakan dengan Rust |

---

## ⚙️ Cara Kerja

1. **Representasi Graf:**
   - Graf direpresentasikan dalam bentuk matriks biaya `NxN`, di mana `m[i][j]` adalah biaya dari kota `i` ke kota `j`.
   - Graf berupa graf berarah, sehingga `m[i][j]` ≠ `m[j][i]`.

2. **Algoritma Held-Karp:**
   - Menggunakan **DP bitmask** untuk merekam semua subset kota yang dikunjungi.
   - `dp[mask][i]` menyimpan biaya minimum untuk mencapai kota `i` dengan subset kota `mask`.
   - Setelah seluruh kota dikunjungi, program menghitung biaya kembali ke kota asal (`0`) dan mencari rute optimal.

3. **Traceback:**
   - Array `parent[mask][i]` digunakan untuk melacak rute yang diambil untuk membangun jalur akhir.

4. **Output:**
   - Total biaya minimum
   - Rute optimal dari kota 0, mengunjungi seluruh kota satu kali, dan kembali ke 0
   - Waktu eksekusi algoritma

---

## 📥 Input Format

- Baris pertama: Jumlah kota `N` (misal: `4`)
- Diikuti `N` baris, masing-masing berisi `N` angka yang mempresentasikan **matriks biaya**.

### Contoh:
```
4
0 10 15 20
5 0 9 10
6 13 0 12
8 8 9 0
```

---

## 📤 Output Format

```
Biaya minimum untuk graf berarah: 35
Rute perjalanan: 0 → 1 → 3 → 2 → 0
Waktu eksekusi: 41.5000µs
```

---

## ▶️ Cara Menjalankan

### 1. **Kompilasi & Jalankan**
```bash
cargo build
cargo run < input.txt
```

### 2. **Input via Terminal**
Ketik manual saat program dijalankan:
```bash
cargo build
cargo run
```
Kemudian masukkan input sesuai format.

---

## 📦 Struktur Proyek (jika pakai Cargo)
```
tsp_solver/
├── Cargo.toml
└── src/
    └── main.rs
```

## 📌 Catatan

- Kompleksitas waktu: **O(n² * 2ⁿ)**  

---

## 🧑‍💻 Author

> Reza Ahmad Syarif (13523119)
