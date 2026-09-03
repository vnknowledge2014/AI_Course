---
id: co-so-du-lieu.mot-hop-nhieu-hinh.kieu-du-lieu-phong-phu
title: "Kiểu dữ liệu phong phú"
summary: "Truong{ten, gia_tri: i64} (q08) chỉ có MỘT kiểu giá trị — số nguyên. SurrealDB thật có record link (bo = nguoi:cha, tham chiếu THẲNG tới một bản ghi khác), mảng (so_thich = [...]), và object lồng nhau (dia_chi = {...}) — TẤT CẢ dùng được ngay trong một SET, không cần khai kiểu. Chấm bo.ten tự động ĐI THEO record link, đọc trường của bản ghi được trỏ tới — một dạng graph traversal KHÔNG cần RELATE."
locale: vi
track: co-so-du-lieu
module: mot-hop-nhieu-hinh
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [db.kieu-du-lieu-phong-phu]
requires: [db.transaction-atomic]
concepts: [db.kieu-du-lieu-phong-phu]
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
`struct Truong { ten: Vec<char>, gia_tri: i64 }` (q08) — MỖI trường
chỉ mang ĐÚNG một kiểu, số nguyên. SurrealDB thật cho phép trường
mang những KIỂU gì?
::::

::::explain{#kieu-phong-phu}
Một trường CÓ thể LÀ record link (tham chiếu THẲNG tới bản ghi
khác), mảng, HOẶC object lồng nhau — TRONG cùng một `SET`:

```text title=readonly
CREATE nguoi:cha SET ten = 'Cha';
CREATE nguoi:con SET
  ten = 'Con',
  bo = nguoi:cha,
  so_thich = ['doc sach', 've'],
  dia_chi = { thanh_pho: 'HN', so_nha: 12 };
SELECT * FROM nguoi:con;
```

```text title=readonly
[{
  id: "nguoi:con",
  ten: "Con",
  bo: "nguoi:cha",
  so_thich: ["doc sach", "ve"],
  dia_chi: { thanh_pho: "HN", so_nha: 12 }
}]
```

`bo` LÀ một record link (LƯU thẳng `"nguoi:cha"`, id CỦA bản ghi
khác). `so_thich` LÀ mảng chuỗi. `dia_chi` LÀ một object LỒNG bên
trong — KHÔNG có trường NÀO trong số NÀY cần khai kiểu TRƯỚC, VÀ
CẢ ba đều nằm gọn trong MỘT lệnh `CREATE ... SET`.
::::

::::example{#cham-di-theo-record-link}
Chấm VÀO một record link ĐI THEO nó, đọc trường CỦA bản ghi được
TRỎ tới — MỘT dạng duyệt đồ thị KHÔNG cần `RELATE`:

```text title=readonly
SELECT bo.ten AS ten_cha FROM nguoi:con;
```

```text title=readonly
[{ ten_cha: "Cha" }]
```

`bo.ten` tự động "NHẢY" từ `nguoi:con` SANG `nguoi:cha` (bản ghi mà
`bo` trỏ TỚI), rồi đọc `ten` CỦA nó. Đây LÀ một cách LIÊN kết dữ
liệu KHÁC hẳn `RELATE` (bài 5-6): `RELATE` tạo một BẢN ghi cạnh
riêng (trong bảng `biet`), còn record LINK CHỈ là một trường THƯỜNG
mang giá trị LÀ id của bản ghi khác — nhẹ hơn, phù hợp VỚI quan hệ
"một-tới-một" (một CON có ĐÚNG một cha), trong khi `RELATE` phù hợp
VỚI quan HỆ "nhiều-tới-nhiều" (một người CÓ nhiều bạn) mang thêm dữ
liệu RIÊNG trên cạnh (`tu_nam`, bài 5).
::::

::::predict{#doan-mang-long-object commitOnce}
`SELECT so_thich[0] FROM nguoi:con;` (LẤY phần TỬ đầu của mảng
`so_thich`). Dòng CUỐI in ra gì?

:::opt{correct}
`[{ so_thich: "doc sach" }]` — chỉ số MẢNG dùng được NGAY trong câu
truy vấn, KHÔNG cần đọc TOÀN bộ mảng rồi tự chỉ số Ở tầng ỨNG dụng
:::

:::opt
Máy báo LỖI — vì `SELECT` chỉ chọn được TÊN trường, KHÔNG hỗ trợ
biểu thức CÓ chỉ số mảng NGAY trong câu truy vấn
::why
Gần đúng ở việc bạn nghĩ TỚI `SELECT` như một phép CHIẾU tên-trường
ĐƠN thuần (đúng VỚI mọi ví dụ Ở bài 2 — `SELECT ten`) — MỘT quan
sát hợp lý dựa TRÊN những gì ĐÃ thấy.

Chỗ lệch: SurrealQL cho phép biểu thức PHỨC tạp hơn tên-trường TRẦN
Ở vị trí SELECT — `so_thich[0]` LÀ một biểu thức CHỈ số hợp lệ, hoạt
động y HỆT chỉ số mảng trong lập trình THÔNG thường. Kết quả VẪN
mang TÊN trường gốc (`so_thich`), giá TRỊ LÀ phần tử ĐẦU tiên,
KHÔNG phải LỖI.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Kiểu dữ liệu phong phú — record link, mảng, object lồng nhau. Còn
một mảng THAO tác mini-engine chưa hề chạm: đếm VÀ nhóm dữ liệu.
Trông ra sao?
::::

::::reflect{#nghi-lai}
`Truong{ten, gia_tri: i64}` (q08) LÀ một lựa chọn thiết KẾ cố Ý đơn
giản — quest tập TRUNG dạy CƠ chế thực thi (quét, lọc, chọn cột),
KHÔNG phải hệ thống KIỂU. SurrealDB thật CẦN hỗ trợ đủ kiểu dữ liệu
mà ỨNG dụng thật cần — record link, mảng, object LỒNG — VÀ record
link tự CHẤM (`bo.ten`) cho THẤY ranh giới GIỮA "trường thường" VÀ
"quan hệ đồ thị" (`RELATE`) MỜ hơn nhiều SO với những gì q09 dạy.
::::

::::checkpoint{mastery=0.75}
::::
