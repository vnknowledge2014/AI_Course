---
id: co-so-du-lieu.mot-hop-nhieu-hinh.mutation-that
title: "CREATE/UPDATE/DELETE thật"
summary: "Mini-engine (q07-q09) CHỈ đọc — không hề có CREATE/UPDATE/DELETE. SurrealDB thật có cả ba: CREATE tạo bản ghi, UPDATE sửa tại chỗ (ghi đè giá trị, không tạo bản mới), DELETE xoá hẳn — SAU DELETE, SELECT trên chính bản ghi đó trả về mảng rỗng, không còn dấu vết nào ở tầng truy vấn."
locale: vi
track: co-so-du-lieu
module: mot-hop-nhieu-hinh
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [db.mutation-that]
requires: [db.document-schemaless]
concepts: [db.mutation-that]
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
`tra_cuu`, `xay_chi_muc`, `trong_khoang` — MỌI hàm mini-engine (q07-
q09) VIẾT đều CHỈ đọc. SurrealDB thật CÓ sửa/xoá được dữ liệu ĐÃ ghi
không?
::::

::::explain{#ba-lenh-mutation}
`CREATE` tạo, `UPDATE` sửa TẠI chỗ, `DELETE` xoá HẲN:

```text title=readonly
CREATE nguoi:f SET ten = 'Phuong', tuoi = 22;
UPDATE nguoi:f SET tuoi = 23;
SELECT * FROM nguoi:f;
DELETE nguoi:f;
SELECT * FROM nguoi:f;
```

```text title=readonly
[{ id: "nguoi:f", ten: "Phuong", tuoi: 22 }]
[{ id: "nguoi:f", ten: "Phuong", tuoi: 23 }]
[{ id: "nguoi:f", ten: "Phuong", tuoi: 23 }]
[]
[]
```

`UPDATE` đổi `tuoi` TỪ `22` thành `23` — GHI đè trên CÙNG một bản
ghi, không tạo bản MỚI (SELECT NGAY sau đó chỉ thấy MỘT bản ghi,
mang giá trị MỚI). `DELETE` xoá HẲN — `SELECT` NGAY sau trả VỀ mảng
rỗng, KHÔNG còn dấu vết nào Ở tầng truy vấn NÀY.
::::

::::example{#khac-voi-bitcask}
So VỚI q01-q02 (append-only log, Bitcask): xoá Ở ĐÓ ghi một BIA mộ
(tombstone) — bản ghi CŨ vẫn CÒN nằm TRONG file log, chỉ ẩn ĐI ở
tầng đọc, chờ compaction dọn THẬT. `DELETE nguoi:f` Ở q10 (TẦNG
truy vấn CỦA SurrealDB, KHÔNG phải tầng lưu trữ vật lý) trông NHƯ
xoá SẠCH ngay lập tức — nhưng cách LƯU trữ vật lý BÊN dưới (LSM, B+
Tree, HAY thứ khác) LÀ chi tiết triển khai KHÔNG lộ ra Ở tầng câu
truy vấn NÀY. Ý tưởng "xoá KHÔNG hẳn LÀ xoá ngay Ở tầng vật LÝ" (q02)
vẫn ĐÚNG — chỉ LÀ không quan sát được TỪ SQL/SurrealQL.
::::

::::predict{#doan-update-tao-moi commitOnce}
Bảng `nguoi` ĐÃ tồn tại (từ `CREATE nguoi:f` Ở trên), nhưng bản ghi
`nguoi:khong_co` CHƯA từng được `CREATE`. `UPDATE nguoi:khong_co SET
ten = 'Moi';` RỒI `SELECT * FROM nguoi:khong_co;` — dòng CUỐI in ra
gì?

:::opt{correct}
`[]` — `UPDATE` CHỈ sửa bản ghi ĐÃ tồn tại, KHÔNG tự tạo mới, giống
`bang.giaTri[lo] = giaTri` (ghi đè) của `chenMemtable` (q04) — CHỈ
chạy khi khoá ĐÃ có sẵn
:::

:::opt
`[{ id: "nguoi:khong_co", ten: "Moi" }]` — `UPDATE` trên một khoá
CHƯA tồn tại tự ĐỘNG tạo mới bản ghi ĐÓ ("upsert")
::why
Gần đúng ở việc bạn đoán "update" mang nghĩa "update-or-insert" — MỘT
quy ước THẬT tồn tại Ở NHIỀU hệ cơ sở dữ liệu khác, VÀ SurrealQL CÓ
một lệnh làm ĐÚNG điều đó.

Chỗ lệch: lệnh ĐÓ tên LÀ `UPSERT`, KHÔNG phải `UPDATE`. SurrealQL's
`UPDATE` (đã verify THẬT) đòi hỏi bản ghi PHẢI tồn tại SẴN — không
khớp bản ghi NÀO thì `UPDATE` đơn giản KHÔNG làm gì (trả VỀ mảng
rỗng), giống HỆT `chenMemtable` (q04) chỉ ghi đè khi khoá ĐÃ có sẵn.
Muốn "CÓ thì sửa, CHƯA có thì tạo" THẬT sự, PHẢI dùng `UPSERT
nguoi:khong_co SET ten = 'Moi';` thay VÌ `UPDATE`.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đọc, sửa, xoá — SurrealDB thật LÀM đủ cả ba. Nó CÒN LÀM được đồ thị
NGUYÊN sinh — không cần TỰ xây `Canh{tu,den}` VÀ hai chỉ mục NHƯ q09.
Trông RA sao?
::::

::::reflect{#nghi-lai}
Khoảng cách LỚN nhất giữa mini-engine (q07-q09) VÀ SurrealDB thật
KHÔNG nằm Ở cú pháp SELECT (bài 1-2 ĐÃ chỉ ra hai bên ĐỒNG thuận Ở
đó) — nó nằm Ở chỗ mini-engine CHƯA từng cần GHI dữ liệu MỚI: mọi
`Bang` trong q07-q09 đều LÀ dữ liệu KHỞI tạo SẴN trong `fn main`,
không hàm NÀO trả về một `Bang` đã SỬA. `CREATE`/`UPDATE`/`DELETE`
LÀ lớp chức NĂNG hoàn toàn mới — VÀ vẫn CHƯA phải LÀ tất cả.
::::

::::checkpoint{mastery=0.75}
::::
