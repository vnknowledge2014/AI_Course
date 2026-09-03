---
id: co-so-du-lieu.mot-hop-nhieu-hinh.transaction-atomic
title: "Transaction thật"
summary: "BEGIN TRANSACTION ... CANCEL TRANSACTION huỷ TOÀN BỘ các câu ghi ở giữa — sau CANCEL, bảng chưa từng được tạo (SELECT trên nó báo lỗi 'table does not exist', không phải mảng rỗng). BEGIN ... COMMIT TRANSACTION thì các câu ghi giữ nguyên. Mini-engine (q07-q09) chưa từng có khái niệm 'nhiều câu ghi cùng thành công hoặc cùng thất bại' — đây là khoảng cách lớn nhất với q05 (2PL/deadlock, chỉ dạy lý thuyết khoá, chưa có atomicity thật)."
locale: vi
track: co-so-du-lieu
module: mot-hop-nhieu-hinh
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [db.transaction-atomic]
requires: [db.index-khai-bao]
concepts: [db.transaction-atomic]
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
q05 dạy 2PL, deadlock — LÝ thuyết khoá GIỮA nhiều giao dịch. Nhưng
mini-engine CHƯA từng thực sự CHẠY một transaction. SurrealDB thật
có atomicity — MỘT nhóm câu ghi CÙNG thành công HOẶC cùng KHÔNG xảy
ra gì cả — trông ra sao?
::::

::::explain{#begin-cancel}
`BEGIN TRANSACTION` mở một GIAO dịch, `CANCEL TRANSACTION` HUỶ toàn
bộ các câu Ở giữa — như thể CHÚNG chưa từng chạy. Gửi CẢ ba câu
(`BEGIN`/`CREATE`/`CANCEL`) trong MỘT lượt:

```text title=readonly
BEGIN TRANSACTION;
CREATE tk:x SET so_du = 100;
CANCEL TRANSACTION;
```

```text title=readonly
Lỗi: The query was not executed due to a cancelled transaction
```

CẢ lượt gửi báo LỖI — KHÔNG chỉ câu `CANCEL`, mà TOÀN bộ giao dịch bị
coi LÀ "chưa từng chạy" NGAY Ở tầng driver, không trả VỀ kết quả
riêng LẺ cho `CREATE` bên trong. Gửi một câu MỚI, RIÊNG biệt, Ở lượt
SAU:

```text title=readonly
SELECT * FROM tk;
```

```text title=readonly
Lỗi: The table 'tk' does not exist
```

Bảng `tk` thậm CHÍ chưa hề được TẠO ra — `CANCEL` không CHỈ "hoàn
tác giá TRỊ", nó khiến TOÀN bộ giao dịch NHƯ chưa từng tồn TẠI. Đây
LÀ atomicity — "tất CẢ hoặc không GÌ cả" — Ở dạng mạnh NHẤT: một giao
dịch bị HUỶ không để lại DẤU vết nào, kể cả Ở tầng THÔNG báo lỗi.
::::

::::example{#commit-giu-nguyen}
`COMMIT TRANSACTION` thay VÌ `CANCEL` — CÁC câu ghi giữ NGUYÊN:

```text title=readonly
BEGIN TRANSACTION;
CREATE tk:y SET so_du = 50;
COMMIT TRANSACTION;
SELECT * FROM tk;
```

```text title=readonly
[{ id: "tk:y", so_du: 50 }]
```

CÙNG cấu trúc CÂU lệnh (`BEGIN` ... `CREATE` ... kết thúc), CHỈ khác
từ KHOÁ cuối (`COMMIT` thay VÌ `CANCEL`) — kết QUẢ đối lập hoàn
toàn: bản ghi TỒN tại, bảng `tk` CÓ thật.
::::

::::predict{#doan-hai-cau-trong-mot-giao-dich commitOnce}
Một giao dịch chuyển TIỀN: `BEGIN TRANSACTION; UPDATE tk:y SET
so_du = so_du - 20; CREATE tk:z SET so_du = 20; CANCEL TRANSACTION;`
(người viết ĐỔI Ý, huỷ NGANG). `SELECT so_du FROM tk:y;` SAU đó cho
kết quả gì?

:::opt{correct}
`[{ so_du: 50 }]` — VẪN LÀ giá trị TRƯỚC giao dịch, `UPDATE` bên
trong bị huỷ hoàn TOÀN cùng cả giao DỊCH
:::

:::opt
`[{ so_du: 30 }]` — CÂU `UPDATE` đã CHẠY xong (`50 - 20 = 30`) trước
khi `CANCEL` được gọi, nên giá trị TRỪ đi vẫn giữ NGUYÊN
::why
Gần đúng ở việc bạn tính ĐÚNG phép trừ `50 - 20 = 30` — MỘT phép
toán chính xác NẾU `UPDATE` thực sự "chốt" ngay LÚC chạy.

Chỗ lệch: BÊN trong một giao dịch (`BEGIN` ... `CANCEL`/`COMMIT`),
KHÔNG câu lệnh NÀO "chốt" riêng lẻ — mọi thay đổi CHỈ trở thành THẬT
Ở đúng thời điểm `COMMIT`. `CANCEL` huỷ ĐỒNG thời TẤT cả (`UPDATE`
lẫn `CREATE`), y hệt CÁCH `CANCEL` một mình (đầu bài) khiến bảng
`tk` chưa từng TỒN tại — `tk:y` giữ nguyên giá TRỊ TRƯỚC giao dịch,
`50`.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Atomicity thật — không phải chỉ LÝ thuyết. SurrealDB thật CÒN có
những kiểu dữ liệu phong PHÚ hơn HẲN `Truong{ten, gia_tri: i64}`
CỦA mini-engine. Trông ra sao?
::::

::::reflect{#nghi-lai}
q05 dạy 2PL VÀ deadlock — ĐIỀU KIỆN để nhiều giao dịch dùng CHUNG dữ
liệu một cách AN toàn — nhưng mini-engine (q07-q09) không hề CÓ khái
niệm "một nhóm câu ghi PHẢI cùng thành CÔNG hay cùng thất BẠI".
`BEGIN`/`COMMIT`/`CANCEL` LÀ CHÍNH khái niệm ĐÓ, thật SỰ chạy được:
huỷ MỘT giao dịch không để LẠI dấu vết NÀO, dù giao dịch ĐÓ có BAO
nhiêu câu lệnh Ở giữa.
::::

::::checkpoint{mastery=0.8}
::::
