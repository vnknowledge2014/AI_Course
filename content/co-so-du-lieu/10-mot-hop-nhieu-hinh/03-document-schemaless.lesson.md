---
id: co-so-du-lieu.mot-hop-nhieu-hinh.document-schemaless
title: "Document — không cần khai cột trước"
summary: "SurrealDB thật KHÔNG cần khai trước bảng có cột gì (schemaless) — nguoi:d có trường tuoi, nguoi:e có trường mau_thich, CẢ hai cùng nằm trong bảng nguoi mà không lỗi gì. Mini-engine (Bang = Vec<Vec<Truong>>, mỗi hàng là Vec<Truong> riêng) đã mô phỏng được Ý này MỘT PHẦN, nhưng chưa từng thật sự khai thác trong q07-q09 — mọi bảng dùng trong bài học đều dùng ĐỦ cùng một tập cột."
locale: vi
track: co-so-du-lieu
module: mot-hop-nhieu-hinh
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [db.document-schemaless]
requires: [db.query-dong-thuan]
concepts: [db.document-schemaless]
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
Mọi bảng q07-q09 dùng đều CÓ đủ CÙNG một tập cột Ở MỌI hàng. SurrealDB
thật CÓ bắt buộc điều đó không?
::::

::::explain{#khong-can-khai-truoc}
`nguoi:d` CÓ trường `tuoi`. `nguoi:e` KHÔNG hề có `tuoi` — nó CÓ
`mau_thich` thay VÀO đó. CẢ hai cùng nằm TRONG bảng `nguoi`, KHÔNG
CẦN khai trước "bảng `nguoi` CÓ những cột gì":

```text title=readonly
CREATE nguoi:d SET ten = 'Dung', tuoi = 30;
CREATE nguoi:e SET ten = 'Em', mau_thich = 'xanh';
SELECT * FROM nguoi ORDER BY id;
```

```text title=readonly
[
  { id: "nguoi:d", ten: "Dung", tuoi: 30 },
  { id: "nguoi:e", ten: "Em", mau_thich: "xanh" }
]
```

KHÔNG CÓ lỗi "thiếu cột `tuoi`" cho `nguoi:e`, VÀ KHÔNG có lỗi "cột
`mau_thich` lạ" cho `nguoi:d` — đây LÀ "document" (tài liệu): mỗi
bản ghi tự MANG theo tập trường CỦA riêng nó, không CÓ một schema
CỐ định áp đặt lên TOÀN bộ bảng.
::::

::::example{#mini-engine-da-lam-mot-phan}
Mini-engine (q08) ĐÃ có sẵn hình dạng NÀY, dù chưa từng khai THÁC:
`Bang = Vec<Vec<Truong>>` — MỖI hàng LÀ một `Vec<Truong>` HOÀN toàn
ĐỘC lập VỚI hàng khác. KHÔNG có gì trong `struct Truong { ten:
Vec<char>, gia_tri: i64 }` HAY `xay_chi_muc`/`tra_cuu` (q08 bài 1,
5) BẮT buộc hai hàng phải CÓ cùng tập TRƯỜNG — `tra_cuu` chỉ quét
QUA CÁC `Truong` CÓ mặt, trả VỀ `0` nếu KHÔNG tìm thấy TÊN trường
cần (đúng NHƯ `SELECT tuoi FROM nguoi:two` khi `nguoi:two` không CÓ
`tuoi`, bài 1). MỌI dữ liệu MẪU trong q07-q09 chỉ ĐƠN giản CHƯA từng
thử một bảng CÓ hàng thiếu cột — không PHẢI vì mini-engine KHÔNG hỗ
trợ được.
::::

::::predict{#doan-truong-khong-co commitOnce}
`SELECT mau_thich FROM nguoi:d;` (`nguoi:d` KHÔNG hề có `mau_thich`
— chỉ CÓ `ten`, `tuoi`). Dòng CUỐI in ra gì?

:::opt{correct}
`[{}]` — một đối tượng RỖNG, giống hệt trường hợp `SELECT tuoi FROM
nguoi:two` Ở bài 1
:::

:::opt
`[{ mau_thich: null }]` — SurrealDB TRẢ về `null` tường minh CHO
mọi trường không TỒN tại, giống cách nhiều hệ CƠ sở dữ liệu quan hệ
biểu diễn "ô trống"
::why
Gần đúng ở việc bạn liên hệ TỚI khái niệm "ô trống LÀ null" — một
quy ước THẬT của nhiều cơ sở dữ liệu quan HỆ có schema cố định
(bảng CÓ đủ cột, chỉ CÓ giá trị Ở ô LÀ null).

Chỗ lệch: SurrealDB (document, KHÔNG schema cố định) không hề "dành
CHỖ" cho một cột `mau_thich` trên `nguoi:d` để RỒI điền `null` VÀO
đó — trường ĐÓ đơn giản KHÔNG tồn tại trên bản ghi NÀY, nên bước
CHIẾU (SELECT) không CÓ gì để lấy, kết QUẢ LÀ đối tượng rỗng `{}`,
không PHẢI `{ mau_thich: null }`.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bảng KHÔNG cần khai cột trước. SurrealDB thật CÒN sửa được dữ liệu
ĐÃ ghi — mini-engine (q07-q09) CHƯA từng làm điều ĐÓ. Trông ra sao?
::::

::::reflect{#nghi-lai}
"Document" (schemaless) KHÔNG phải một tính năng XA lạ VỚI mini-
engine — CHÍNH `Bang = Vec<Vec<Truong>>` ĐÃ có đủ CẤU trúc để biểu
diễn nó, chỉ LÀ chưa BAO giờ được khai thác TRONG các bài học trước.
Điểm KHÁC biệt thật SỰ nằm Ở chỗ khác: mini-engine CHỈ đọc (`SELECT`)
— nó CHƯA từng ghi, sửa, hay XOÁ dữ liệu. SurrealDB thật CÓ cả ba.
::::

::::checkpoint{mastery=0.75}
::::
