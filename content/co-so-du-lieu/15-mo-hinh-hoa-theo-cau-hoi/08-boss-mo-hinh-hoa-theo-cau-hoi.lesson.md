---
id: co-so-du-lieu.mo-hinh-hoa-theo-cau-hoi.boss-mo-hinh-hoa-theo-cau-hoi
title: "BOSS — Mô hình hoá theo câu hỏi"
summary: "heThongTinNhan ráp TRỌN quest: chonPartitionKeySai dồn cả 1000 tin nhắn vào đúng MỘT partition (tải 1000) — đối chứng xayHeThongDung dùng nguoiId (cardinality cao, 50 giá trị) làm partition key, mỗi partition chỉ 20 tin nhắn, VÀ sắp xếp theo thoiGian (clustering column) để layTinNhanTrongKhoang trả lời đúng câu hỏi 'tin nhắn của nguoi5 trong [100,200]' — kết quả đúng 2 tin nhắn (t=105, t=155), không cần quét hết 1000."
locale: vi
track: co-so-du-lieu
module: mo-hinh-hoa-theo-cau-hoi
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: []
requires: [db.danh-doi-bao-nhieu-bucket-la-du]
concepts: [db.boss-q15]
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
Một bảng cho mỗi câu hỏi (bài 1), partition key (bài 2), clustering
column (bài 3), hot partition (bài 4), bucket hoá (bài 5-7). Cho một
câu hỏi CỤ thể — thiết kế TOÀN bộ trông ra SAO?
::::

::::explain{#boss-that}
Câu hỏi thường gặp: "tin nhắn CỦA một người, trong một khoảng thời
gian". `heThongTinNhan` đối chiếu HAI thiết kế: `chonPartitionKeySai`
(một hằng số CỐ định — MỌI tin nhắn "cùng loại", cardinality `1`) VÀ
`xayHeThongDung` (partition key = `nguoiId`, clustering column =
`thoiGian`):

```typescript title=readonly
interface TinNhan { id: string; nguoiId: string; thoiGian: number; }

function taoTinNhanMau(soTinNhan: number, soNguoiDung: number): TinNhan[] {
  const ds: TinNhan[] = [];
  for (let i = 0; i < soTinNhan; i++) ds.push({ id: "tn" + i, nguoiId: "nguoi" + (i % soNguoiDung), thoiGian: i });
  return ds;
}

function demTheoKhoaPhanVung(cacKhoa: string[]): Map<string, number> {
  const dem = new Map<string, number>();
  for (const k of cacKhoa) dem.set(k, (dem.get(k) ?? 0) + 1);
  return dem;
}

function chonPartitionKeySai(tn: TinNhan): string {
  return "tat-ca-tin-nhan";
}

function xayHeThongDung(tatCa: TinNhan[]): Map<string, TinNhan[]> {
  const bang = new Map<string, TinNhan[]>();
  for (const t of tatCa) {
    const ds = bang.get(t.nguoiId) ?? [];
    ds.push(t);
    bang.set(t.nguoiId, ds);
  }
  for (const [k, ds] of bang) bang.set(k, [...ds].sort((a, b) => a.thoiGian - b.thoiGian));
  return bang;
}

function layTinNhanTrongKhoang(bang: Map<string, TinNhan[]>, nguoiId: string, tu: number, den: number): TinNhan[] {
  const ds = bang.get(nguoiId) ?? [];
  const ketQua: TinNhan[] = [];
  for (const t of ds) {
    if (t.thoiGian > den) break;
    if (t.thoiGian >= tu) ketQua.push(t);
  }
  return ketQua;
}

const duLieu = taoTinNhanMau(1000, 50);

const taiSai = demTheoKhoaPhanVung(duLieu.map(chonPartitionKeySai));
console.log("thiet ke SAI -- tai lon nhat:", Math.max(...taiSai.values()), "so partition:", taiSai.size);

const heThongDung = xayHeThongDung(duLieu);
const taiDung = [...heThongDung.values()].map((ds) => ds.length);
console.log("thiet ke DUNG -- tai lon nhat:", Math.max(...taiDung), "so partition:", heThongDung.size);

console.log("nguoi5 trong [100,200]:", layTinNhanTrongKhoang(heThongDung, "nguoi5", 100, 200).length);
```

```text title=readonly
thiet ke SAI -- tai lon nhat: 1000 so partition: 1
thiet ke DUNG -- tai lon nhat: 20 so partition: 50
nguoi5 trong [100,200]: 2
```

`chonPartitionKeySai` LUÔN trả VỀ cùng một hằng SỐ — cardinality
`1`, nghĩa LÀ TOÀN bộ `1000` tin nhắn dồn VÀO đúng MỘT partition,
hot partition CỰC đoan (bài 4). `xayHeThongDung` dùng `nguoiId`
(`50` giá trị) LÀM partition key — tải lớn nhất giảm CÒN `20` (bài
2). VÀ vì MỖI partition đã ĐƯỢC sắp xếp theo `thoiGian` (clustering
column, bài 3), `layTinNhanTrongKhoang` trả LỜI đúng "tin nhắn của
`nguoi5` trong `[100,200]`" — CHÍNH XÁC `2` tin nhắn, không cần quét
hết `1000`.
::::

::::example{#ca-ba-manh-ghep-cung-lam-viec}
Ba quyết định thiết kế Ở ĐÂY không độc lập: chọn `nguoiId` LÀM
partition key (bài 2) GIẢI quyết hot partition (bài 4) MÀ không cần
bucket hoá (bài 5-7) — VÌ `nguoiId` đã CÓ cardinality đủ cao (`50`
giá trị cho `1000` tin nhắn). Bucket hoá CHỈ cần khi partition key
"đúng về mặt CÂU hỏi" (VÍ dụ `ngay`, q14 bài 4) nhưng "sai VỀ mặt
cardinality" — hai vấn đề KHÁC nhau, hai cách giải KHÁC nhau.
::::

::::predict{#doan-neu-dung-ngay-thay-nguoiId commitOnce}
Nếu ĐỔI `xayHeThongDung` sang dùng `"ngay" + Math.floor(i / 200)`
(chỉ `5` giá trị NGÀY khả dĩ cho `1000` tin nhắn) THAY vì `nguoiId`
LÀM partition key — tải lớn NHẤT sẽ LÀ bao nhiêu?

:::opt{correct}
`200` — y hệt bài 4: chỉ `5` giá trị khả dĩ CHO `1000` tin nhắn nghĩa
LÀ mỗi partition gánh trung BÌNH `1000/5=200`, GẤP `10` lần tải của
thiết kế dùng `nguoiId` (`20`)
:::

:::opt
VẪN LÀ `20` — miễn LÀ có sắp xếp theo `thoiGian` (clustering column),
tải mỗi partition KHÔNG phụ thuộc partition key chọn TRƯỜNG nào
::why
Trực giác NÀY nhầm vai TRÒ của clustering column (sắp xếp BÊN TRONG
một partition) VỚI vai trò của partition key (quyết định BAO nhiêu
hàng rơi và MỘT partition).

Chỗ lệch: clustering column KHÔNG ảnh hưởng gì tới SỐ hàng mỗi
partition CHỨA — nó chỉ quyết định THỨ tự các hàng ĐÓ. Số hàng mỗi
partition hoàn TOÀN do cardinality của partition key quyết ĐỊNH
(bài 2, bài 4): `nguoiId` CÓ `50` giá trị → tải `20`; `ngay` kiểu
NÀY chỉ CÓ `5` giá trị → tải `200`, dù CẢ hai thiết kế đều sắp xếp
đúng theo `thoiGian`.
::
:::
::::

::::code{#viet_he_thong_tin_nhan}
Hoàn thiện `xayHeThongDung` — mỗi partition (theo `nguoiId`) PHẢI
được sắp xếp lại theo `thoiGian` trước khi lưu, để hỗ trợ range
query.

```typescript title=starter
interface TinNhan { id: string; nguoiId: string; thoiGian: number; }

function taoTinNhanMau(soTinNhan: number, soNguoiDung: number): TinNhan[] {
  const ds: TinNhan[] = [];
  for (let i = 0; i < soTinNhan; i++) ds.push({ id: "tn" + i, nguoiId: "nguoi" + (i % soNguoiDung), thoiGian: i });
  return ds;
}

function xayHeThongDung(tatCa: TinNhan[]): Map<string, TinNhan[]> {
  const bang = new Map<string, TinNhan[]>();
  for (const t of tatCa) {
    const ds = bang.get(t.nguoiId) ?? [];
    ds.push(t);
    bang.set(t.nguoiId, ds);
  }
  for (const [k, ds] of bang) {
    ___
  }
  return bang;
}

function layTinNhanTrongKhoang(bang: Map<string, TinNhan[]>, nguoiId: string, tu: number, den: number): TinNhan[] {
  const ds = bang.get(nguoiId) ?? [];
  const ketQua: TinNhan[] = [];
  for (const t of ds) {
    if (t.thoiGian > den) break;
    if (t.thoiGian >= tu) ketQua.push(t);
  }
  return ketQua;
}

const duLieu = taoTinNhanMau(1000, 50);
console.log(layTinNhanTrongKhoang(xayHeThongDung(duLieu), "nguoi5", 100, 200).length);
```

```typescript title=solution
interface TinNhan { id: string; nguoiId: string; thoiGian: number; }

function taoTinNhanMau(soTinNhan: number, soNguoiDung: number): TinNhan[] {
  const ds: TinNhan[] = [];
  for (let i = 0; i < soTinNhan; i++) ds.push({ id: "tn" + i, nguoiId: "nguoi" + (i % soNguoiDung), thoiGian: i });
  return ds;
}

function xayHeThongDung(tatCa: TinNhan[]): Map<string, TinNhan[]> {
  const bang = new Map<string, TinNhan[]>();
  for (const t of tatCa) {
    const ds = bang.get(t.nguoiId) ?? [];
    ds.push(t);
    bang.set(t.nguoiId, ds);
  }
  for (const [k, ds] of bang) {
    bang.set(k, [...ds].sort((a, b) => a.thoiGian - b.thoiGian));
  }
  return bang;
}

function layTinNhanTrongKhoang(bang: Map<string, TinNhan[]>, nguoiId: string, tu: number, den: number): TinNhan[] {
  const ds = bang.get(nguoiId) ?? [];
  const ketQua: TinNhan[] = [];
  for (const t of ds) {
    if (t.thoiGian > den) break;
    if (t.thoiGian >= tu) ketQua.push(t);
  }
  return ketQua;
}

const duLieu = taoTinNhanMau(1000, 50);
console.log(layTinNhanTrongKhoang(xayHeThongDung(duLieu), "nguoi5", 100, 200).length);
```

```typescript title=test
const duLieuKt = taoTinNhanMau(1000, 50);
const heThongKt = xayHeThongDung(duLieuKt);

if (heThongKt.size !== 50) throw new Error("phai co dung 50 partition (moi nguoi dung mot partition)");
if ((heThongKt.get("nguoi5") ?? []).length !== 20) throw new Error("nguoi5 phai co dung 20 tin nhan");

const dsNguoi5 = heThongKt.get("nguoi5")!;
for (let i = 1; i < dsNguoi5.length; i++) {
  if (dsNguoi5[i]!.thoiGian < dsNguoi5[i - 1]!.thoiGian) throw new Error("moi partition PHAI duoc sap xep tang dan theo thoiGian, khong duoc giu nguyen thu tu chen ban dau");
}

const ketQuaKhoang = layTinNhanTrongKhoang(heThongKt, "nguoi5", 100, 200);
if (ketQuaKhoang.length !== 2) throw new Error("nguoi5 trong [100,200] phai co dung 2 tin nhan");
if (ketQuaKhoang[0]!.thoiGian !== 105 || ketQuaKhoang[1]!.thoiGian !== 155) throw new Error("phai la dung 2 tin nhan thoiGian=105 va 155, dung thu tu tang dan");

if (layTinNhanTrongKhoang(heThongKt, "nguoi-khong-ton-tai", 0, 1000).length !== 0) throw new Error("nguoi khong ton tai thi phai tra ve mang rong");

let tongTatCa = 0;
for (const [, ds] of heThongKt) tongTatCa += ds.length;
if (tongTatCa !== 1000) throw new Error("tong so tin nhan trong moi partition cong lai phai bang dung 1000");

const duLieuXaoTron: TinNhan[] = [
  { id: "a", nguoiId: "x", thoiGian: 30 },
  { id: "b", nguoiId: "x", thoiGian: 10 },
  { id: "c", nguoiId: "x", thoiGian: 20 },
];
const heThongXaoTron = xayHeThongDung(duLieuXaoTron);
const dsX = heThongXaoTron.get("x")!;
if (dsX.map((t) => t.thoiGian).join(",") !== "10,20,30") throw new Error("du lieu dua vao KHONG theo thu tu (30,10,20) -- xayHeThongDung PHAI tu sap xep lai thanh 10,20,30, khong duoc giu nguyen thu tu chen");
const ketQuaXaoTron = layTinNhanTrongKhoang(heThongXaoTron, "x", 15, 25);
if (ketQuaXaoTron.length !== 1 || ketQuaXaoTron[0]!.thoiGian !== 20) throw new Error("sau khi sap xep dung, khoang [15,25] phai tim dung 1 tin nhan (thoiGian=20)");
```

:::hints
- kind: attention
  body: "Sap xep lai danh sach ds theo thoiGian tang dan, roi luu lai vao bang -- mot dong."
- kind: strategy
  body: "bang.set(k, [...ds].sort((a, b) => a.thoiGian - b.thoiGian));"
- kind: one-line
  body: "bang.set(k, [...ds].sort((a, b) => a.thoiGian - b.thoiGian));"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "2"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Partition key chọn ĐÚNG, clustering column sắp xếp ĐÚNG, bucket hoá
khi CẦN — ba quyết định NÀY chính LÀ "mô hình hoá theo câu hỏi".
q15 khép LẠI — R6-2 (96 bài, q07-q15) hoàn TẤT, và R6 (toàn bộ,
227/227) hoàn TẤT.
::::

::::reflect{#nghi-lai}
`heThongTinNhan` không giới THIỆU một khái niệm MỚI nào — nó xếp
đúng THỨ tự mọi thứ ĐÃ học xuyên suốt q11-q15: hashing để định vị
(q11, tái hiện Ở partition key bài 2), thứ tự BÊN trong một nhóm dữ
liệu (bài 3, cùng tinh thần memtable q04), cardinality quyết định
tải (bài 4), VÀ đánh đổi ghi‑so‑với‑đọc khi cần bucket hoá (bài 5-7,
cùng tinh thần STCS/LCS/TWCS q14). Bài học LỚN nhất của q15, VÀ có
lẽ CỦA toàn bộ R6-2: KHÔNG có một cách "đúng" duy nhất để tổ CHỨC dữ
liệu — chỉ có cách phù HỢP nhất với câu hỏi THẬT sự sẽ được hỏi.
::::

::::checkpoint{mastery=0.9}
::::
