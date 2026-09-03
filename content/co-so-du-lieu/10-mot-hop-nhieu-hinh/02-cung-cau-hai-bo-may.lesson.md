---
id: co-so-du-lieu.mot-hop-nhieu-hinh.cung-cau-hai-bo-may
title: "Cùng một câu, hai bộ máy"
summary: "SELECT ten FROM nguoi WHERE tuoi > 18 chạy trên SurrealDB thật (đã verify: ['Binh','Chi']) và chạy trên logic tự viết bằng TypeScript (lọc rồi chiếu cột — đúng tinh thần Quét→Lọc→ChọnCột của q08) đều cho CÙNG kết quả — trên câu truy vấn ĐƠN giản này, hai bộ máy ĐỒNG thuận."
locale: vi
track: co-so-du-lieu
module: mot-hop-nhieu-hinh
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.query-dong-thuan]
requires: [db.surrealdb-multi-model]
concepts: [db.query-dong-thuan]
gradingMatrix:
  web-chrome: [run, tests, output]
  web-firefox: [run, tests, output]
  macos: [run, tests, output]
  windows: [run, tests, output]
  linux: [run, tests, output]
  android: [run, tests, output]
  ios: [run, tests, output]
provenance:
  authoredBy: llm-assisted
  reviewed: false
---

::::byte{trigger=enter mood=curious pose=lean-in}
`CREATE`/`SELECT *` khớp mini-engine (bài TRƯỚC). Câu truy vấn
CHÍNH q08 dạy — `SELECT ... WHERE ...` — chạy trên SurrealDB THẬT
trông ra SAO?
::::

::::explain{#surrealdb-that-where}
Ba người, tuổi `15, 20, 25`. Lọc TUỔI lớn hơn `18`, chỉ CHỌN cột
`ten`:

```text title=readonly
CREATE nguoi:a SET ten = 'An', tuoi = 15;
CREATE nguoi:b SET ten = 'Binh', tuoi = 20;
CREATE nguoi:c SET ten = 'Chi', tuoi = 25;
SELECT ten FROM nguoi WHERE tuoi > 18;
```

```text title=readonly
[{ ten: "Binh" }, { ten: "Chi" }]
```

`An` (tuổi `15`) bị LOẠI. `Binh` VÀ `Chi` (tuổi `20`, `25`) qua
được — VÀ kết quả chỉ CÒN trường `ten`, KHÔNG có `id`/`tuoi` (SELECT
chỉ chiếu ĐÚNG cột được LIỆT kê, giống HỆT `chon_cot` của q08 bài
11).
::::

::::example{#doi-chieu-logic-tu-viet}
Cùng dữ liệu, cùng điều kiện, viết LẠI bằng TypeScript thuần — LỌC
trước (`chonTheoTuoi`), CHIẾU cột sau (`chonCotTen`) — đúng thứ tự
`Quét → Lọc → ChọnCột` mà q08 dạy:

```typescript title=readonly
interface Nguoi {
  ten: string;
  tuoi: number;
}

function chonTheoTuoi(bang: Nguoi[], nguongDuoi: number): Nguoi[] {
  const ketQua: Nguoi[] = [];
  for (const hang of bang) {
    if (hang.tuoi > nguongDuoi) {
      ketQua.push(hang);
    }
  }
  return ketQua;
}

function chonCotTen(hangDaLoc: Nguoi[]): string[] {
  const ketQua: string[] = [];
  for (const hang of hangDaLoc) {
    ketQua.push(hang.ten);
  }
  return ketQua;
}

const bang: Nguoi[] = [
  { ten: "An", tuoi: 15 },
  { ten: "Binh", tuoi: 20 },
  { ten: "Chi", tuoi: 25 },
];
console.log(chonCotTen(chonTheoTuoi(bang, 18)).join(","));
```

```text title=readonly
Binh,Chi
```

Kết quả `["Binh","Chi"]` — KHỚP CHÍNH XÁC danh sách `ten` mà
SurrealDB thật trả về. Trên câu truy vấn ĐƠN giản NÀY (một điều
kiện, một phép so sánh), hai bộ MÁY hoàn toàn đồng THUẬN.
::::

::::predict{#doan-nguong-cao-hon commitOnce}
CÙNG bảng ba người, đổi ngưỡng THÀNH `tuoi > 25`. `chonCotTen
(chonTheoTuoi(bang, 25))` trả về gì?

:::opt{correct}
`[]` — mảng RỖNG, không AI có tuổi lớn hơn `25`
:::

:::opt
Lỗi — vì `25` LÀ tuổi LỚN nhất trong bảng, so sánh VỚI chính giá trị
LỚN nhất là một trường hợp KHÔNG hợp lệ
::why
Gần đúng ở việc bạn để Ý ĐÚNG rằng `25` chính LÀ tuổi lớn nhất
trong bảng — một quan SÁT thật về dữ liệu.

Chỗ lệch: `hang.tuoi > nguongDuoi` LÀ so sánh NGHIÊM ngặt (`>`,
không `>=`) — `Chi` (tuổi `25`) KHÔNG thoả `25 > 25` (SAI), nên bị
LOẠI giống mọi người khác. KHÔNG hàng NÀO qua được điều kiện, vòng
`for` chạy hết mà không đẩy GÌ vào `ketQua` — kết quả LÀ mảng rỗng,
hoàn toàn hợp LỆ, không phải lỗi.
::
:::
::::

::::code{#viet_chon_theo_tuoi}
Hoàn thiện `chonTheoTuoi` — giữ lại hàng CÓ `tuoi` lớn hơn
`nguongDuoi`.

```typescript title=starter
interface Nguoi {
  ten: string;
  tuoi: number;
}

function chonTheoTuoi(bang: Nguoi[], nguongDuoi: number): Nguoi[] {
  const ketQua: Nguoi[] = [];
  for (const hang of bang) {
    if (___) {
      ketQua.push(hang);
    }
  }
  return ketQua;
}

function chonCotTen(hangDaLoc: Nguoi[]): string[] {
  const ketQua: string[] = [];
  for (const hang of hangDaLoc) {
    ketQua.push(hang.ten);
  }
  return ketQua;
}

const bang: Nguoi[] = [
  { ten: "An", tuoi: 15 },
  { ten: "Binh", tuoi: 20 },
  { ten: "Chi", tuoi: 25 },
];
console.log(chonCotTen(chonTheoTuoi(bang, 18)).join(","));
```

```typescript title=solution
interface Nguoi {
  ten: string;
  tuoi: number;
}

function chonTheoTuoi(bang: Nguoi[], nguongDuoi: number): Nguoi[] {
  const ketQua: Nguoi[] = [];
  for (const hang of bang) {
    if (hang.tuoi > nguongDuoi) {
      ketQua.push(hang);
    }
  }
  return ketQua;
}

function chonCotTen(hangDaLoc: Nguoi[]): string[] {
  const ketQua: string[] = [];
  for (const hang of hangDaLoc) {
    ketQua.push(hang.ten);
  }
  return ketQua;
}

const bang: Nguoi[] = [
  { ten: "An", tuoi: 15 },
  { ten: "Binh", tuoi: 20 },
  { ten: "Chi", tuoi: 25 },
];
console.log(chonCotTen(chonTheoTuoi(bang, 18)).join(","));
```

```typescript title=test
const bang2: Nguoi[] = [
  { ten: "An", tuoi: 15 },
  { ten: "Binh", tuoi: 20 },
  { ten: "Chi", tuoi: 25 },
];
const ketQua2 = chonCotTen(chonTheoTuoi(bang2, 18));
console.log(ketQua2.join(","));
if (ketQua2.join(",") !== "Binh,Chi") throw new Error("phai khop dung ket qua SurrealDB that: Binh,Chi");

const bangRong: Nguoi[] = [];
if (chonTheoTuoi(bangRong, 18).length !== 0) throw new Error("bang rong phai tra ve mang rong");

const bangDuoiNguong: Nguoi[] = [{ ten: "An", tuoi: 5 }];
if (chonTheoTuoi(bangDuoiNguong, 18).length !== 0) throw new Error("tuoi duoi nguong phai bi loc");

const bangDungNguong: Nguoi[] = [{ ten: "Bang", tuoi: 18 }];
if (chonTheoTuoi(bangDungNguong, 18).length !== 0) throw new Error("tuoi DUNG BANG nguong phai bi loc -- dieu kien la lon hon NGHIEM NGAT, khong phai lon hon hoac bang");
```

:::hints
- kind: attention
  body: "Dieu kien giu lai hang: tuoi cua no lon hon nguongDuoi -- mot dong."
- kind: strategy
  body: "hang.tuoi > nguongDuoi"
- kind: one-line
  body: "if (hang.tuoi > nguongDuoi) {"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "Binh,Chi"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Câu truy vấn ĐƠN giản khớp Ở CẢ hai engine. SurrealDB thật CÒN LÀM
được điều mini-engine KHÔNG hề chạm tới — không cần khai CỘT trước
khi ghi. Trông ra SAO?
::::

::::reflect{#nghi-lai}
Trên MỘT câu truy vấn ĐƠN giản (một điều kiện so sánh, một cột được
chọn), logic TỰ viết bằng TypeScript VÀ SurrealDB thật cho ĐÚNG cùng
kết quả — ĐIỀU đó xác nhận mini-engine (q07-q09) không hề "bịa" ra
một ngữ nghĩa RIÊNG, nó nắm ĐÚNG lõi của điều SELECT...WHERE THẬT
làm. Nhưng "lõi" KHÔNG phải "toàn bộ" — SurrealDB thật LÀM được rất
nhiều ĐIỀU mini-engine không hề chạm tới, bắt ĐẦU từ việc KHÔNG cần
khai TRƯỚC bảng có những CỘT gì.
::::

::::checkpoint{mastery=0.8}
::::
