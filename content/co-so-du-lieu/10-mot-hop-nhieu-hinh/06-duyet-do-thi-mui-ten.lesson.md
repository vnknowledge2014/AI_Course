---
id: co-so-du-lieu.mot-hop-nhieu-hinh.duyet-do-thi-mui-ten
title: "Duyệt đồ thị thật: ->"
summary: "SELECT ->biet->nguoi.ten AS ban_be FROM nguoi:one đi qua MỌI cạnh 'biet' xuất phát từ nguoi:one rồi lấy trường ten của đích — MỘT dòng, thay cho tim_theo_khoa + tra ngược canh[cs[i]].den (q09 bài 8). KHÔNG có AS alias, kết quả LỒNG theo đúng đường ->biet->nguoi thay vì phẳng ra — và khi có từ hai cạnh trở lên, THỨ TỰ các đích trong mảng KHÔNG được đảm bảo giữa các lần chạy."
locale: vi
track: co-so-du-lieu
module: mot-hop-nhieu-hinh
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [db.duyet-do-thi-mui-ten]
requires: [db.relate-graph-nguyen-sinh]
concepts: [db.duyet-do-thi-mui-ten]
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
Cạnh `RELATE` (bài TRƯỚC) LÀ một bản ghi thật. Tìm "MỌI người BẠN
đi từ X" — TƯƠNG đương `hang_xom_di` (q09 bài 8) — cú pháp SurrealDB
thật trông ra SAO?
::::

::::explain{#cu-phap-mui-ten}
`->biet->nguoi` đi QUA mọi cạnh `biet` xuất phát TỪ bản ghi hiện
tại, RỒI tới bảng `nguoi` Ở đầu KIA. `.ten AS ban_be` lấy trường
`ten` CỦA mỗi đích, VÀ đặt TÊN kết quả LÀ `ban_be` (không CÓ `AS`,
kết quả sẽ LỒNG theo đúng đường ĐI, xem ví dụ SAU):

```text title=readonly
CREATE nguoi:one SET ten = 'Byte';
CREATE nguoi:two SET ten = 'Ada';
RELATE nguoi:one->biet->nguoi:two;
SELECT ->biet->nguoi.ten AS ban_be FROM nguoi:one;
```

```text title=readonly
[{ ban_be: ["Ada"] }]
```

MỘT dòng SurrealQL LÀM trọn việc mà `hang_xom_di` (q09 bài 8) cần
BA bước: `tim_theo_khoa` (tra chỉ mục "đi") RỒI dịch NGƯỢC chỉ số
cạnh VỀ đỉnh đích qua `canh[cs[i]].den`.
::::

::::example{#khong-alias-long-va-thu-tu}
BỎ `AS ban_be` — kết quả LỒNG theo đúng đường `->biet->nguoi`, KHÔNG
phẳng ra thành một trường ĐƠN:

```text title=readonly
SELECT ->biet->nguoi FROM nguoi:one;
```

```text title=readonly
[{ "->biet": { "->nguoi": ["nguoi:two"] } }]
```

VÀ khi MỘT đỉnh có TỪ hai cạnh đi trở LÊN, thứ tự các đích trong
mảng KHÔNG được đảm bảo — chạy CÙNG một câu truy vấn NHIỀU lần trên
CÙNG dữ liệu (hai cạnh `nguoi:one -> nguoi:two`, `nguoi:one ->
nguoi:three`) đã quan sát ĐƯỢC cả `["Ada", "Kern"]` LẪN `["Kern",
"Ada"]` Ở những lần chạy KHÁC nhau — SurrealQL, giống hầu HẾT ngôn
ngữ truy vấn, KHÔNG cam kết thứ TỰ hàng trả về TRỪ khi có `ORDER BY`
tường minh. Đây LÀ một khác biệt THẬT so VỚI mini-engine (q09):
`hang_xom_di` LUÔN trả về đúng thứ tự cạnh xuất hiện trong `Vec<
Canh>` — không hề "xáo trộn" giữa các lần gọi.
::::

::::predict{#doan-canh-de-trong-danh commitOnce}
Đỉnh `nguoi:two` KHÔNG có cạnh `biet` nào ĐI (nó chỉ LÀ đích, không
phải NGUỒN). `SELECT ->biet->nguoi FROM nguoi:two;` trả về gì?

:::opt{correct}
`[{ "->biet": { "->nguoi": [] } }]` — mảng RỖNG lồng Ở đúng vị trí,
KHÔNG lỗi
:::

:::opt
`[]` — mảng NGOÀI cùng rỗng, vì bản thân TRUY vấn không tìm thấy gì
để trả VỀ
::why
Gần đúng ở việc bạn nắm ĐÚNG "không CÓ cạnh nào" nghĩa LÀ kết quả
RỖNG Ở đâu đó — một trực GIÁC hợp lý.

Chỗ lệch: `SELECT ... FROM nguoi:two` VẪN LÀ một `SELECT` trên một
bản ghi CÓ THẬT (`nguoi:two` tồn tại) — nó LUÔN trả về đúng MỘT
dòng CHO bản ghi ĐÓ (giống mọi `SELECT ... FROM <id>` khác), CHỈ LÀ
trường `->biet->nguoi` bên TRONG dòng đó LÀ mảng rỗng (không CÓ
cạnh nào để duyệt). Mảng NGOÀI cùng vẫn CÓ đúng một PHẦN TỬ, không
phải rỗng.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Duyệt đồ thị — một DÒNG cú pháp mũi tên. SurrealDB thật CÒN có chỉ
mục khai BÁO được, không cần tự viết `MucChiMuc` (q09). Trông ra
sao?
::::

::::reflect{#nghi-lai}
`->ten_quan_he->bang.truong` LÀ cú pháp duyệt đồ thị NGUYÊN sinh —
so VỚI `hang_xom_di` (q09 bài 8), nó gộp CẢ ba bước (tra chỉ mục,
lấy chỉ số cạnh, dịch VỀ đỉnh đích) thành MỘT biểu thức. Đây LÀ cái
GIÁ trả cho việc DÙNG một engine THẬT thay VÌ tự viết: cú pháp NGẮN
hơn rất NHIỀU, nhưng "kết quả CÓ giữ đúng thứ tự chèn không" TRỞ
thành một câu hỏi KHÔNG còn câu trả lời CHẮC chắn — mini-engine
(`Vec<Canh>`) LUÔN giữ thứ tự VÌ nó chỉ LÀ một mảng, còn engine thật
CÓ thể tổ chức lưu trữ THEO cách khác hẳn Ở dưới, VÀ không hứa GÌ về
thứ tự trừ khi được YÊU cầu tường minh (`ORDER BY`).
::::

::::checkpoint{mastery=0.8}
::::
