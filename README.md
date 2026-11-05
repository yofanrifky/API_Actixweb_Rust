# Restoran API

REST API restoran menggunakan Rust, Actix Web, dan PostgreSQL. Backend ini tidak menyertakan frontend. Endpoint menu dan pesanan dapat diakses langsung tanpa autentikasi. Request dapat diuji dari Postman.

## Struktur Proyek

```text
backend/
|-- postman/Restoran-API.postman_collection.json
|-- src/
|   |-- handlers/
|   |   |-- menus.rs       # Operasi menu
|   |   |-- orders.rs      # Operasi pesanan
|   |   `-- mod.rs         # Daftar modul handler
|   |-- db.rs              # Persiapan tabel database
|   |-- main.rs            # Titik mulai aplikasi
|   |-- models.rs          # Bentuk data menu dan pesanan
|   |-- routes.rs          # Pendaftaran endpoint
|   `-- state.rs           # Pool database untuk handler
|-- catatan/               # Panduan belajar per file
|-- .env                   # Konfigurasi lokal, jangan commit
`-- Cargo.toml             # Dependency Rust
```

## Persiapan dan Menjalankan

1. Pastikan Rust/Cargo dan PostgreSQL sudah terpasang. Buat database kosong yang akan dipakai aplikasi.
2. Buat file `.env` pada folder backend. Isi `DATABASE_URL`, misalnya `postgres://USER:PASSWORD@HOST:PORT/NAMA_DATABASE`.
3. Jalankan aplikasi dari folder backend:

```sh
cargo run
```

Server default berjalan di `http://127.0.0.1:8080`. Saat startup, aplikasi membuat tabel `menus` dan `orders` jika tabel belum tersedia.

`cargo check` hanya memeriksa kompilasi; perintah itu tidak menguji koneksi PostgreSQL. `cargo run` memerlukan database aktif dan nilai `.env` yang benar.

## Endpoint

| Metode | URL | Kegunaan |
|---|---|---|---|
| GET | `/api/menus` | Daftar menu atau satu kolom dengan `field` |
| GET | `/api/menus/{id}` | Detail satu menu |
| POST | `/api/menus` | Membuat menu |
| PUT | `/api/menus/{id}` | Mengganti seluruh field menu |
| PATCH | `/api/menus/{id}` | Mengganti sebagian field menu |
| DELETE | `/api/menus/{id}` | Menghapus menu |
| GET | `/api/orders` | Daftar pesanan atau satu kolom dengan `field` |
| GET | `/api/orders/{id}` | Detail satu pesanan |
| POST | `/api/orders` | Membuat pesanan |
| PUT | `/api/orders/{id}` | Mengganti seluruh field pesanan |
| PATCH | `/api/orders/{id}` | Mengganti sebagian field pesanan |
| DELETE | `/api/orders/{id}` | Menghapus pesanan |

## Contoh Data

Body untuk membuat menu:

```json
{
  "name": "Nasi Goreng",
  "price": 25000,
  "category": "Makanan"
}
```

Harga disimpan dalam tipe PostgreSQL `NUMERIC(12,2)` dan dikirim sebagai angka JSON tanpa format mata uang.

Body untuk membuat pesanan:

```json
{
  "customer_name": "Budi",
  "item_name": "Nasi Goreng",
  "quantity": 2,
  "status": "pending"
}
```

`PUT` memerlukan semua field input. `PATCH` hanya memerlukan field yang ingin diubah, contohnya:

```json
{
  "status": "diproses"
}
```

Pada `GET /api/menus`, parameter `field` yang didukung adalah `id`, `name`, `price`, dan `category`. Pada `GET /api/orders`, field-nya `id`, `customer_name`, `item_name`, `quantity`, dan `status`. Contoh: `GET /api/orders?field=status`.

## Postman

Impor `postman/Restoran-API.postman_collection.json` dan jalankan request menu/pesanan secara langsung. Sesuaikan ID pada request detail, update, dan delete dengan ID yang benar-benar ada.

Panduan menguji API dari terminal menggunakan `curl` ada di [catatan/akses_curl.md](catatan/akses_curl.md). Panduan pemula yang membahas implementasi per file ada di [catatan/README.md](catatan/README.md).