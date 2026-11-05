# Mengakses API dengan `curl`

Panduan ini menunjukkan cara mengirim request ke backend menggunakan `curl` dari terminal Linux. Endpoint menu dan pesanan dapat dipakai langsung tanpa login atau cookie.

Navigasi: [indeks catatan belajar](README.md).

> **Inti bab:** `curl` mengirim request HTTP dari terminal dan menampilkan response server.

## Sebelum Mulai

1. Pastikan PostgreSQL berjalan dan database sudah tersedia.
2. Pastikan `.env` memiliki `DATABASE_URL` yang valid.
3. Jalankan server dari folder backend dengan `cargo run`, lalu biarkan terminal server berjalan.
4. Buka terminal kedua untuk menjalankan request berikut.

```bash
BASE_URL="http://127.0.0.1:8080"
```

Sesuaikan `BASE_URL` jika host atau port di `.env` berbeda.

## Membaca Data

Ambil semua menu, detail menu, atau satu kolom tertentu:

```bash
curl -i "$BASE_URL/api/menus"
curl -i "$BASE_URL/api/menus/1"
curl -i "$BASE_URL/api/menus?field=price"
```

Ambil semua pesanan atau hanya statusnya:

```bash
curl -i "$BASE_URL/api/orders"
curl -i "$BASE_URL/api/orders?field=status"
```

`-i` menyertakan header response, termasuk status HTTP. Query `field` memilih satu kolom yang didukung oleh endpoint tersebut.

## Membuat Data

`-H` mengirim header `Content-Type`, sementara `-d` mengirim JSON sebagai body request.

Membuat menu:

```bash
curl -i -X POST "$BASE_URL/api/menus" \
  -H "Content-Type: application/json" \
  -d '{"name":"Nasi Goreng","price":25000,"category":"Makanan"}'
```

Membuat pesanan:

```bash
curl -i -X POST "$BASE_URL/api/orders" \
  -H "Content-Type: application/json" \
  -d '{"customer_name":"Budi","item_name":"Nasi Goreng","quantity":2,"status":"pending"}'
```

Jika berhasil, server mengembalikan `201 Created` dan data baru.

## Mengubah Data

`PUT` mengganti seluruh field input. Ganti ID `1` sesuai data yang ingin diubah.

```bash
curl -i -X PUT "$BASE_URL/api/menus/1" \
  -H "Content-Type: application/json" \
  -d '{"name":"Nasi Goreng Spesial","price":30000,"category":"Makanan"}'
```

`PATCH` hanya mengubah field yang dikirim; field lain tetap seperti semula.

```bash
curl -i -X PATCH "$BASE_URL/api/menus/1" \
  -H "Content-Type: application/json" \
  -d '{"price":27500}'

curl -i -X PATCH "$BASE_URL/api/orders/1" \
  -H "Content-Type: application/json" \
  -d '{"status":"diproses"}'
```

## Menghapus Data

`DELETE` menghapus data berdasarkan ID.

```bash
curl -i -X DELETE "$BASE_URL/api/menus/1"
curl -i -X DELETE "$BASE_URL/api/orders/1"
```

## Status Umum

| Status | Makna |
|---|---|
| `200 OK` | Request berhasil, misalnya membaca atau memperbarui data. |
| `201 Created` | Menu atau pesanan berhasil dibuat. |
| `400 Bad Request` | JSON tidak valid atau input tidak memenuhi aturan. |
| `404 Not Found` | ID menu/pesanan tidak ditemukan. |
| `500 Internal Server Error` | Server atau database mengalami kegagalan. |

## Masalah Umum

- **`curl: (7) Failed to connect`**: server belum berjalan, atau `BASE_URL` memakai host/port yang salah.
- **`400 Bad Request`**: periksa nama field JSON, tanda kutip, koma, dan tipe nilai seperti `price` serta `quantity`.
- **`404 Not Found`**: pastikan ID yang digunakan memang tersedia.

API ini belum memiliki autentikasi atau kontrol akses. Gunakan hanya untuk pengembangan lokal, bukan untuk mengekspos data melalui jaringan publik.
