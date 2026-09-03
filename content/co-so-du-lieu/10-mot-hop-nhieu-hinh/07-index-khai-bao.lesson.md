---
id: co-so-du-lieu.mot-hop-nhieu-hinh.index-khai-bao
title: "Chỉ mục thật: DEFINE INDEX"
summary: "DEFINE INDEX idx_tuoi ON TABLE nguoi COLUMNS tuoi; là MỘT dòng khai báo — SurrealDB tự xây và bảo trì chỉ mục ở tầng engine, không cần bất kỳ hàm nào như xay_chi_muc/trong_khoang (q09 bài 5-6) do người dùng tự viết. INFO FOR TABLE xác nhận chỉ mục đã đăng ký; câu WHERE vẫn cho đúng kết quả như khi CHƯA có chỉ mục — chỉ mục thay đổi TỐC ĐỘ, không thay đổi kết quả."
locale: vi
track: co-so-du-lieu
module: mot-hop-nhieu-hinh
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [db.index-khai-bao]
requires: [db.duyet-do-thi-mui-ten]
concepts: [db.index-khai-bao]
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
q09 tự viết `MucChiMuc{khoa, vi_tri}`, `xay_chi_muc`, `trong_khoang`
— khoảng MƯỜI lăm dòng Rust CHO một chỉ mục theo giá TRỊ. SurrealDB
thật khai báo chỉ mục bằng CÂU lệnh nào?
::::

::::explain{#define-index}
`DEFINE INDEX` khai một chỉ mục — MỘT dòng, engine tự XÂY VÀ bảo
trì phần CÒN lại:

```text title=readonly
DEFINE INDEX idx_tuoi ON TABLE nguoi COLUMNS tuoi;
CREATE nguoi:a SET ten = 'An', tuoi = 15;
CREATE nguoi:b SET ten = 'Binh', tuoi = 20;
CREATE nguoi:c SET ten = 'Chi', tuoi = 25;
SELECT ten FROM nguoi WHERE tuoi > 18;
INFO FOR TABLE nguoi;
```

```text title=readonly
[{ ten: "Binh" }, { ten: "Chi" }]
{
  events: {},
  fields: {},
  indexes: { idx_tuoi: "DEFINE INDEX idx_tuoi ON nguoi FIELDS tuoi" },
  lives: {},
  tables: {}
}
```

`SELECT ten FROM nguoi WHERE tuoi > 18` cho ĐÚNG kết quả GIỐNG hệt
lúc CHƯA có chỉ mục (bài 2) — `["Binh", "Chi"]`. `INFO FOR TABLE
nguoi` xác nhận chỉ mục ĐÃ đăng ký. Điểm CỐT lõi: chỉ mục thay đổi
TỐC độ tra cứu (q09 dạy TẠI SAO — so sánh byte trên khoá đã mã hoá
thay VÌ quét toàn bộ), KHÔNG hề thay đổi KẾT quả — MỘT câu `SELECT`
CÓ chỉ mục HAY không CHỈ mục vẫn phải trả VỀ đúng CÙNG dữ liệu.
::::

::::example{#khong-can-tu-quan-ly}
Ở q09, MỖI lần bảng THAY đổi (thêm hàng MỚI), chỉ mục PHẢI được xây
LẠI thủ công (gọi LẠI `xay_chi_muc`) — mini-engine KHÔNG hề tự động
CẬP nhật chỉ mục khi dữ liệu ĐỔI. `DEFINE INDEX` thật KHÁC hẳn: sau
khi khai MỘT lần, MỌI `CREATE`/`UPDATE`/`DELETE` sau ĐÓ tự động giữ
chỉ mục đồng bộ VỚI dữ liệu — người DÙNG không bao GIỜ tự tay "xây
lại chỉ mục".
::::

::::predict{#doan-xoa-index commitOnce}
Sau khi ĐÃ `DEFINE INDEX idx_tuoi ...` (Ở trên), chạy `REMOVE INDEX
idx_tuoi ON TABLE nguoi;` RỒI `SELECT ten FROM nguoi WHERE tuoi >
18;`. Câu SELECT NÀY cho kết quả gì?

:::opt{correct}
VẪN LÀ `["Binh", "Chi"]` — XOÁ chỉ mục không đổi KẾT quả, chỉ có
thể LÀM chậm lại việc tra cứu
:::

:::opt
Lỗi — vì câu `WHERE tuoi > 18` PHỤ thuộc VÀO `idx_tuoi` để chạy
được, xoá chỉ mục LÀM câu truy vấn KHÔNG còn hợp lệ
::why
Gần đúng ở việc bạn nghĩ TỚI chỉ mục như một "yêu cầu bắt BUỘC" cho
truy vấn — MỘT trực giác dễ hiểu NẾU coi chỉ mục LÀ phần cấu TRÚC
dữ liệu, không phải phần TỐI ưu.

Chỗ lệch: y HỆT `trong_khoang` (q09) VẪN chạy đúng dù `chi_muc`
chưa hề được SẮP xếp — chỉ mục CHỈ LÀ một con ĐƯỜNG tắt để TÌM nhanh
hơn, KHÔNG phải điều KIỆN để câu truy vấn "hợp lệ". Không CÓ chỉ
mục, SurrealDB đơn giản QUÉT toàn bộ bảng (chậm hơn, KHÔNG sai) —
giống hệt cách `tra_cuu` (q08 bài 1) quét TUYẾN tính khi chưa CÓ
`xay_chi_muc` nào cả.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chỉ mục khai báo được. SurrealDB thật CÒN đảm bảo NHIỀU câu ghi cùng
LÚC hoặc THÀNH công hết, hoặc không CÓ gì xảy ra — mini-engine chưa
từng chạm khái niệm ĐÓ. Trông ra sao?
::::

::::reflect{#nghi-lai}
`DEFINE INDEX` LÀ minh chứng RÕ nhất cho khoảng cách "tự XÂY" VÀ
"dùng công cụ THẬT": q09 dạy đúng CÁI GIÁ của một lần TRA cứu có
chỉ mục (mã hoá khoá, so SÁNH byte) bằng CÁCH bắt bạn TỰ tay làm hết
— SurrealDB thật ẩn TOÀN bộ điều đó SAU một dòng khai báo. Hiểu Ở
bên TRONG (q09) không LÀM cho một dòng `DEFINE INDEX` bớt tiện —
nhưng nó khiến dòng đó thôi LÀ ma thuật.
::::

::::checkpoint{mastery=0.75}
::::
