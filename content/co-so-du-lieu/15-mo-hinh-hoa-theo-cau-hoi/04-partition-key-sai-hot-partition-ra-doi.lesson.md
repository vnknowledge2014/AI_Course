---
id: co-so-du-lieu.mo-hinh-hoa-theo-cau-hoi.partition-key-sai-hot-partition-ra-doi
title: "Partition key sai — hot partition ra đời"
summary: "demTheoKhoaPhanVung đếm số sự kiện rơi vào MỖI partition key. Chọn 'ngay' làm partition key (chỉ 5 giá trị phân biệt trong 1000 sự kiện) khiến TOÀN BỘ dữ liệu dồn vào đúng 5 partition — mỗi partition 200 sự kiện, đều nhau vì cardinality thấp NHƯNG mọi partition khác trong cụm ĐỀU rỗng. Số partition VẬT LÝ trong cụm không quan trọng nếu partition key chỉ có vài giá trị khả dĩ — đó chính LÀ hot partition."
locale: vi
track: co-so-du-lieu
module: mo-hinh-hoa-theo-cau-hoi
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.partition-key-sai-hot-partition-ra-doi]
requires: [db.clustering-column-sap-xep-trong-partition]
concepts: [db.partition-key-sai-hot-partition-ra-doi]
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
Partition key ĐÚNG (phong5, bài 2) rải dữ liệu ĐỀU. Nhưng nếu chọn
SAI — dùng một trường CHỈ có vài giá trị khả dĩ — chuyện GÌ xảy ra?
::::

::::explain{#hot-partition}
`demTheoKhoaPhanVung` đếm số sự kiện rơi VÀO mỗi giá trị partition
key. Thử dùng `ngay` (ngày, CHỈ `5` giá trị khả dĩ) LÀM partition key
cho `1000` sự kiện:

```typescript title=readonly
interface SuKien { id: string; ngay: string; userId: string; }

function taoSuKien(soSuKien: number, soNgay: number, soUser: number): SuKien[] {
  const ds: SuKien[] = [];
  for (let i = 0; i < soSuKien; i++) {
    ds.push({ id: "sk" + i, ngay: "ngay" + (i % soNgay), userId: "user" + (i % soUser) });
  }
  return ds;
}

function demTheoKhoaPhanVung(cacKhoa: string[]): Map<string, number> {
  const dem = new Map<string, number>();
  for (const k of cacKhoa) dem.set(k, (dem.get(k) ?? 0) + 1);
  return dem;
}

const suKien = taoSuKien(1000, 5, 200);
const theoNgay = demTheoKhoaPhanVung(suKien.map((sk) => sk.ngay));
console.log("so partition co du lieu:", theoNgay.size);
console.log("tai partition 'ngay0':", theoNgay.get("ngay0"));
console.log("tai lon nhat:", Math.max(...theoNgay.values()));
```

```text title=readonly
so partition co du lieu: 5
tai partition 'ngay0': 200
tai lon nhat: 200
```

`1000` sự kiện, NHƯNG `ngay` chỉ CÓ `5` giá trị khả dĩ — nên CHỈ đúng
`5` partition key từng xuất hiện, mỗi partition GÁNH đúng `200` sự
kiện (`1000/5`). Cụm CÓ THỂ có `16`, `100`, hay `1000` partition vật
lý — KHÔNG hề quan trọng, vì partition key chỉ SINH ra `5` giá trị
KHÁC nhau, nên KHÔNG BAO giờ dùng tới quá `5` partition.
::::

::::example{#so-partition-vat-ly-khong-cuu-duoc}
Đây LÀ điểm khác biệt CĂN bản VỚI vấn đề Ở q12 bài 7 (thêm lõi VẪN
rải được ĐỀU, chỉ cần rehash): Ở ĐÓ, khoá CÓ hàng nghìn giá trị khả
dĩ (`"nguoidung0".."nguoidung99"`), CHỈ CẦN đủ node LÀ rải đều được.
Ở ĐÂY, `ngay` chỉ CÓ `5` giá trị — dù cụm CÓ bao nhiêu partition vật
lý, KHÔNG BAO giờ dữ liệu chạm tới quá `5` trong SỐ đó. Cardinality
(số giá trị khả DĨ) của partition key LÀ trần trên CỨNG cho số
partition thực SỰ nhận dữ liệu.
::::

::::predict{#doan-doi-so-nguoi-dung commitOnce}
Đổi `soUser` từ `200` xuống CÒN `3` (chỉ `3` người dùng khả dĩ),
NHƯNG vẫn giữ `soNgay=5`. Nếu ĐỔI partition key SANG `userId` (thay
vì `ngay`), số partition CÓ dữ liệu LÀ bao nhiêu?

:::opt{correct}
`3` — CHỈ đúng bằng cardinality của `userId` LÚC này (`3` giá trị
khả dĩ), bất kể `soNgay` LÀ bao nhiêu — partition key nào QUYẾT định
thì cardinality của TRƯỜNG đó quyết định số partition CÓ dữ liệu
:::

:::opt
`15` (`3 × 5`) — SỐ tổ hợp khả dĩ giữa `userId` VÀ `ngay` mới LÀ con
số quyết định, vì cả hai TRƯỜNG đều tồn tại trong dữ liệu
::why
Trực giác NÀY đúng NẾU partition key LÀ tổ hợp CẢ hai trường (VÍ dụ
`userId + "#" + ngay`) — nhưng câu hỏi chỉ đổi partition key SANG
`userId` MỘT mình, không GHÉP với `ngay`.

Chỗ lệch: `demTheoKhoaPhanVung` chỉ đếm THEO đúng mảng khoá được
truyền VÀO — nếu gọi VỚI `suKien.map(sk => sk.userId)`, nó KHÔNG hề
biết tới sự tồn tại của `ngay`. Chỉ trường THỰC SỰ được dùng làm
partition key mới quyết định — Ở ĐÂY LÀ `userId`, cardinality `3`.
::
:::
::::

::::code{#viet_dem_theo_khoa_phan_vung}
Hoàn thiện `demTheoKhoaPhanVung` — với mỗi khoá, tăng bộ đếm của
đúng khoá đó lên một.

```typescript title=starter
interface SuKien { id: string; ngay: string; userId: string; }

function taoSuKien(soSuKien: number, soNgay: number, soUser: number): SuKien[] {
  const ds: SuKien[] = [];
  for (let i = 0; i < soSuKien; i++) {
    ds.push({ id: "sk" + i, ngay: "ngay" + (i % soNgay), userId: "user" + (i % soUser) });
  }
  return ds;
}

function demTheoKhoaPhanVung(cacKhoa: string[]): Map<string, number> {
  const dem = new Map<string, number>();
  for (const k of cacKhoa) {
    ___
  }
  return dem;
}

const suKien = taoSuKien(1000, 5, 200);
console.log(demTheoKhoaPhanVung(suKien.map((sk) => sk.ngay)).size);
```

```typescript title=solution
interface SuKien { id: string; ngay: string; userId: string; }

function taoSuKien(soSuKien: number, soNgay: number, soUser: number): SuKien[] {
  const ds: SuKien[] = [];
  for (let i = 0; i < soSuKien; i++) {
    ds.push({ id: "sk" + i, ngay: "ngay" + (i % soNgay), userId: "user" + (i % soUser) });
  }
  return ds;
}

function demTheoKhoaPhanVung(cacKhoa: string[]): Map<string, number> {
  const dem = new Map<string, number>();
  for (const k of cacKhoa) {
    dem.set(k, (dem.get(k) ?? 0) + 1);
  }
  return dem;
}

const suKien = taoSuKien(1000, 5, 200);
console.log(demTheoKhoaPhanVung(suKien.map((sk) => sk.ngay)).size);
```

```typescript title=test
const suKien2 = taoSuKien(1000, 5, 200);
const theoNgay2 = demTheoKhoaPhanVung(suKien2.map((sk) => sk.ngay));
if (theoNgay2.size !== 5) throw new Error("chi 5 gia tri ngay kha di -- phai co dung 5 partition co du lieu");
if (theoNgay2.get("ngay0") !== 200) throw new Error("ngay0 phai co dung 200 su kien");
if (theoNgay2.get("ngay4") !== 200) throw new Error("ngay4 phai co dung 200 su kien");

let tong = 0;
for (const [, dem] of theoNgay2) tong += dem;
if (tong !== 1000) throw new Error("tong tat ca partition cong lai phai bang dung 1000");

const theoUser2 = demTheoKhoaPhanVung(suKien2.map((sk) => sk.userId));
if (theoUser2.size !== 200) throw new Error("200 gia tri userId kha di -- phai co dung 200 partition co du lieu");

if (demTheoKhoaPhanVung([]).size !== 0) throw new Error("mang rong thi phai cho ket qua rong");
if (demTheoKhoaPhanVung(["x", "x", "x"]).get("x") !== 3) throw new Error("3 lan cung mot khoa phai dem dung 3");
```

:::hints
- kind: attention
  body: "Tang bo dem cua khoa k len 1 (mac dinh 0 neu chua co) -- mot dong."
- kind: strategy
  body: "dem.set(k, (dem.get(k) ?? 0) + 1);"
- kind: one-line
  body: "dem.set(k, (dem.get(k) ?? 0) + 1);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "5"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hot partition LÀ vấn đề THẬT — nhưng partition key thấp cardinality
(`ngay`) đôi khi LÀ điều CẦN cho câu hỏi thường gặp. Sửa thế NÀO mà
không đổi hẳn câu hỏi?
::::

::::reflect{#nghi-lai}
`demTheoKhoaPhanVung` KHÔNG phải hàm mới — nó gần NHƯ y hệt `demPhanBo`
(q11 bài 7), chỉ khác Ở CHỖ đếm theo khoá STRING trực tiếp thay VÌ
qua vòng hash trước. Bài học GIỐNG hệt: cardinality thấp của khoá
dùng để phân vùng LÀ nguyên nhân GỐC của lệch tải — dù đó LÀ node
(q11) hay partition (bài NÀY). q11 giải bằng vnode (thêm điểm ảo cho
node ÍT). Ở đây, "ngay" không THỂ có thêm giá trị — cần một cách
KHÁC để tăng cardinality một cách NHÂN tạo.
::::

::::checkpoint{mastery=0.85}
::::
