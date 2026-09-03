---
id: co-so-du-lieu.mot-hop-nhieu-hinh.relate-graph-nguyen-sinh
title: "Đồ thị thật: RELATE"
summary: "RELATE nguoi:one->biet->nguoi:two tạo một BẢN GHI THẬT trong bảng biet, mang id riêng, và hai trường in/out trỏ tới hai bản ghi đã nối — không phải một struct Canh{tu,den} tự viết (q09). Cạnh đồ thị trong SurrealDB LÀ dữ liệu bậc nhất (first-class), truy vấn/gắn thêm trường được y hệt mọi bảng khác."
locale: vi
track: co-so-du-lieu
module: mot-hop-nhieu-hinh
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.relate-graph-nguyen-sinh]
requires: [db.mutation-that]
concepts: [db.relate-graph-nguyen-sinh]
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
q09 tự XÂY cạnh đồ thị: `struct Canh { tu: usize, den: usize }`,
CỘNG hai chỉ mục tự viết TAY. SurrealDB thật CÓ cú pháp đồ thị
NGUYÊN sinh — trông ra SAO?
::::

::::explain{#relate-that}
`RELATE` tạo một CẠNH có hướng — CÚ pháp `nguon->ten_quan_he->dich`:

```text title=readonly
CREATE nguoi:one SET ten = 'Byte';
CREATE nguoi:two SET ten = 'Ada';
RELATE nguoi:one->biet->nguoi:two SET tu_nam = 2020;
SELECT * FROM biet;
```

```text title=readonly
[{ id: "nguoi:one", ten: "Byte" }]
[{ id: "nguoi:two", ten: "Ada" }]
[{ id: "biet:<ngau_nhien>", in: "nguoi:one", out: "nguoi:two", tu_nam: 2020 }]
[{ id: "biet:<ngau_nhien>", in: "nguoi:one", out: "nguoi:two", tu_nam: 2020 }]
```

`RELATE` tạo một BẢN GHI THẬT — trong một bảng TÊN `biet` (đúng TÊN
quan hệ) — mang `id` RIÊNG (một chuỗi ngẫu nhiên, KHÁC mỗi lần chạy),
VÀ hai trường `in`/`out` trỏ tới hai bản ghi ĐÃ nối. `SET tu_nam =
2020` gắn THÊM một trường tuỳ Ý lên CHÍNH cạnh đó — cạnh đồ thị LÀ
dữ liệu bậc NHẤT, không PHẢI một cặp chỉ số TRẦN như `Canh{tu, den}`
(q09).
::::

::::example{#khong-can-tu-xay-chi-muc}
q09 (bài 5-8) PHẢI tự viết `MucChiMuc{khoa, vi_tri}`, `xay_chi_muc_di`,
`tim_theo_khoa` — MỘT chuỗi bước THỦ công để "tra được cạnh nào xuất
phát TỪ đỉnh X". VỚI `RELATE`, SurrealDB tự LƯU trữ VÀ đánh chỉ mục
cạnh Ở tầng ENGINE — bảng `biet` LÀ một bảng THẬT, `SELECT * FROM
biet WHERE in = nguoi:one` (cú pháp giống HỆT lọc một bảng thường)
LÀ đủ để tìm MỌI cạnh xuất phát TỪ `nguoi:one`, không CẦN xây chỉ
mục riêng.
::::

::::predict{#doan-hai-canh-cung-nguon commitOnce}
`RELATE nguoi:one->biet->nguoi:two;` RỒI `RELATE nguoi:one->biet->
nguoi:three;` (HAI lần `RELATE`, CÙNG bảng `biet`). `SELECT count()
FROM biet GROUP ALL;` trả về gì?

:::opt{correct}
`[{ count: 2 }]` — MỖI lần `RELATE` tạo một bản ghi CẠNH mới, độc
LẬP, dù cùng xuất phát TỪ `nguoi:one`
:::

:::opt
`[{ count: 1 }]` — vì CẢ hai cạnh cùng xuất phát TỪ `nguoi:one`,
SurrealDB GỘP chúng THÀNH một bản ghi DUY nhất mang danh sách đích
::why
Gần đúng ở việc bạn nghĩ TỚI một cách BIỂU diễn "gộp" hợp LÝ (một
đỉnh, một danh sách hàng XÓM) — đó chính LÀ cách `hang_xom_di` (q09)
TRẢ về kết quả (một `Vec<usize>`).

Chỗ lệch: `RELATE` KHÔNG hề gộp — MỖI lần gọi tạo ĐÚNG một bản ghi
MỚI trong bảng `biet`, mang `id` RIÊNG. Việc "một đỉnh CÓ nhiều
hàng xóm" được biểu diễn BẰNG nhiều BẢN ghi cạnh riêng BIỆT (giống
HỆT cách `Canh{tu,den}` của q09 dùng NHIỀU phần tử `Vec<Canh>`),
KHÔNG gộp thành MỘT bản ghi mang mảng. `GROUP ALL` gộp MỌI hàng khớp
LẠI thành một nhóm DUY nhất trước khi đếm — thiếu nó, `count()` đếm
TỪNG hàng RIÊNG (trả về `[{count:1},{count:1}]`, không phải một tổng
DUY nhất).
::
:::
::::

::::code{#viet_relate_gia}
Hoàn thiện `relateGia` — mô phỏng `RELATE`: tạo VÀ trả về một cạnh
mang `id` duy NHẤT (đếm tăng dần), `vao` (bản ghi XUẤT phát), `ra`
(bản ghi ĐÍCH) — đúng hình dạng `in`/`out` của SurrealDB thật.

```typescript title=starter
interface Canh {
  id: string;
  vao: string;
  ra: string;
}

let boDemCanh = 0;

function relateGia(banGhiTu: string, tenQuanHe: string, banGhiDen: string): Canh {
  boDemCanh = boDemCanh + 1;
  ___
}

const c1 = relateGia("nguoi:one", "biet", "nguoi:two");
console.log(c1.id, c1.vao, c1.ra);
```

```typescript title=solution
interface Canh {
  id: string;
  vao: string;
  ra: string;
}

let boDemCanh = 0;

function relateGia(banGhiTu: string, tenQuanHe: string, banGhiDen: string): Canh {
  boDemCanh = boDemCanh + 1;
  return { id: `${tenQuanHe}:${boDemCanh}`, vao: banGhiTu, ra: banGhiDen };
}

const c1 = relateGia("nguoi:one", "biet", "nguoi:two");
console.log(c1.id, c1.vao, c1.ra);
```

```typescript title=test
const c1t = relateGia("nguoi:one", "biet", "nguoi:two");
console.log(c1t.id, c1t.vao, c1t.ra);
if (c1t.vao !== "nguoi:one") throw new Error("vao phai la ban ghi xuat phat");
if (c1t.ra !== "nguoi:two") throw new Error("ra phai la ban ghi dich");
if (!c1t.id.startsWith("biet:")) throw new Error("id canh phai bat dau bang ten quan he 'biet:'");

const c2t = relateGia("nguoi:one", "biet", "nguoi:three");
if (c2t.id === c1t.id) throw new Error("moi lan goi relateGia phai tao id RIENG (boDemCanh phai tang moi lan)");
if (c2t.vao !== "nguoi:one") throw new Error("canh thu hai van xuat phat tu nguoi:one");
if (c2t.ra !== "nguoi:three") throw new Error("canh thu hai phai di toi nguoi:three");
```

:::hints
- kind: attention
  body: "Tra ve mot Canh moi: id ket hop tenQuanHe va boDemCanh, vao la banGhiTu, ra la banGhiDen -- mot dong."
- kind: strategy
  body: "return { id: `${tenQuanHe}:${boDemCanh}`, vao: banGhiTu, ra: banGhiDen };"
- kind: one-line
  body: "return { id: `${tenQuanHe}:${boDemCanh}`, vao: banGhiTu, ra: banGhiDen };"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "biet:1 nguoi:one nguoi:two"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cạnh đồ thị LÀ dữ liệu bậc nhất. Duyệt đồ thị (tìm hàng xóm) trên
SurrealDB thật — bằng ĐÚNG một dòng cú pháp mũi tên — trông ra sao?
::::

::::reflect{#nghi-lai}
`Canh{tu, den}` (q09) VÀ cạnh `RELATE` (SurrealDB thật) mang CÙNG
Ý tưởng cốt lõi — một cặp (nguồn, đích) — nhưng KHÁC nhau Ở CHỖ
`RELATE` LÀ một bản ghi ĐẦY đủ (có `id` riêng, nhận thêm trường tuỳ
Ý), trong khi `Canh` LÀ một struct TRẦN chỉ chứa hai chỉ số. Cả hai
đều CẦN một cách "tra cạnh THEO đỉnh nguồn" — q09 TỰ xây (`MucChiMuc`
+ `xay_chi_muc_di`), SurrealDB thật LÀM sẵn Ở tầng engine. Tra được
cạnh RỒI, bước KẾ tiếp LÀ duyệt tới ĐÍCH — cú pháp NÀO làm việc đó?
::::

::::checkpoint{mastery=0.8}
::::
