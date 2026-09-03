---
id: co-so-du-lieu.mot-hop-nhieu-hinh.surrealdb-that-la-gi
title: "SurrealDB thật là gì"
summary: "SurrealDB THẬT — chạy qua @surrealdb/wasm — CHẤP nhận đúng cú pháp CREATE/SELECT bạn ĐÃ tự viết lexer/parser/executor cho (q07-q09). CREATE nguoi:one SET ten='Byte', tuoi=20 RỒI SELECT * FROM nguoi trả về ĐÚNG một hàng {id, ten, tuoi} — hành vi khớp mini-engine, nhưng bằng MỘT engine THẬT, chạy production, không phải bài học."
locale: vi
track: co-so-du-lieu
module: mot-hop-nhieu-hinh
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [db.surrealdb-multi-model]
requires: [db.hnsw-lite-idea]
concepts: [db.surrealdb-multi-model]
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
q07-q09 xây một mini-engine: lexer, parser, executor, chỉ mục, cạnh
đồ thị — TỪ đầu. SurrealDB THẬT (một cơ sở dữ liệu người ta DÙNG
thật ngoài đời) làm những việc GIỐNG hệt vậy — trông ra SAO?
::::

::::explain{#surrealdb-that}
SurrealDB LÀ một cơ sở dữ liệu multi-model THẬT — "một hộp, nhiều
hình": MỘT engine DUY nhất vừa LÀM được bảng có cấu trúc (relational,
giống mini-engine của BẠN), vừa LÀM được tài liệu KHÔNG cần khai cột
trước (document), vừa LÀM được đồ thị (graph), vừa LÀM được key-value
— quest NÀY chạy nó THẬT, qua package `@surrealdb/wasm` (một bản
biên dịch SurrealDB SANG WebAssembly, nhúng thẳng được, không cần cài
server riêng):

```text title=readonly
CREATE nguoi:one SET ten = 'Byte', tuoi = 20;
SELECT * FROM nguoi;
```

```text title=readonly
[{ id: "nguoi:one", ten: "Byte", tuoi: 20 }]
[{ id: "nguoi:one", ten: "Byte", tuoi: 20 }]
```

`CREATE nguoi:one` tạo một BẢN ghi VỚI khoá chính tường minh
(`nguoi:one` — tên bảng CỘNG một định danh, không phải chỉ số ẩn
danh NHƯ `Bang = Vec<Vec<Truong>>` của q08-q09). `SELECT * FROM
nguoi` trả VỀ đúng bản ghi VỪA tạo, kèm `id` — trường BẠN không hề
khai NHƯNG SurrealDB tự THÊM vào MỌI bản ghi.
::::

::::example{#so-voi-mini-engine}
So VỚI mini-engine (q08 bài 1): `Truong{ten, gia_tri}` LÀ một CẶP
tên/giá trị ĐƠN, một `Hang` LÀ `Vec<Truong>`. SurrealDB THẬT LÀM
cùng Ý tưởng — một bản ghi LÀ một tập CẶP tên/giá trị — NHƯNG:

- Khoá chính (`nguoi:one`) LÀ tường minh, KHÔNG phải chỉ số mảng ẩn.
- Kiểu GIÁ trị KHÔNG giới hạn Ở `i64` (mini-engine CHỈ có một kiểu
  số) — chuỗi, số, VÀ nhiều kiểu khác (bài SAU sẽ thấy).
- KHÔNG cần khai "bảng CÓ những cột GÌ" trước khi ghi (bài SAU sẽ
  thấy `nguoi:two` có TẬP trường HOÀN toàn khác `nguoi:one`).

Cả hai đIỀU cuối LÀ ĐIỀU mini-engine của BẠN KHÔNG có — quest NÀY sẽ
chỉ RA từng điểm một.
::::

::::predict{#doan-truong-khong-khai commitOnce}
`CREATE nguoi:two SET ten = 'Ada';` (KHÔNG có `tuoi`). RỒI `SELECT
tuoi FROM nguoi:two;`. Dòng CUỐI in ra gì?

:::opt{correct}
`[{}]` — một MẢNG chứa một đối tượng RỖNG (không CÓ trường `tuoi`
nào để chiếu)
:::

:::opt
Máy báo LỖI — vì `tuoi` KHÔNG tồn tại trên bản ghi `nguoi:two`, chọn
một trường KHÔNG có LÀ thao tác không hợp LỆ
::why
Gần đúng ở việc bạn để Ý đúng rằng `nguoi:two` THỰC sự không CÓ
trường `tuoi` — một quan sát chính XÁC về dữ liệu.

Chỗ lệch: SurrealDB (giống HỆT `tra_cuu` của mini-engine, q08 bài 1)
KHÔNG coi "trường không tồn tại" LÀ lỗi — nó chỉ đơn giản KHÔNG có
gì để chiếu VÀO kết quả cho trường ĐÓ, trả VỀ một đối tượng RỖNG
thay VÌ báo lỗi. Không CÓ khái niệm "cột PHẢI tồn tại" Ở đây.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
CREATE VÀ SELECT khớp mini-engine của BẠN. Nhưng "SELECT ... WHERE"
— câu truy vấn CHÍNH mini-engine dạy — chạy trên engine THẬT trông
ra SAO?
::::

::::reflect{#nghi-lai}
q10 dùng TypeScript (không phải Rust CỦA q07-q09) — VÌ đích ĐẾN LÀ
so sánh VỚI một package JavaScript/WASM THẬT, `@surrealdb/wasm`, chạy
QUA driver `surrealdb`. Quest NÀY KHÔNG viết LẠI SurrealDB — nó CHẠY
SurrealDB thật (kết quả CÁC câu truy vấn trong bài học ĐÃ được xác
minh bằng cách chạy THẬT), rồi so VỚI mini-engine tự viết Ở q07-q09
để thấy chính XÁC engine THẬT làm được GÌ hơn. `SELECT ... WHERE`
— câu truy vấn CHÍNH mini-engine của BẠN dạy — chạy TRÊN cả hai
engine trông RA sao?
::::

::::checkpoint{mastery=0.75}
::::
