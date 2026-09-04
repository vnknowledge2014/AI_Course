---
id: thiet-ke-he-thong.thoi-gian-thuc-va-vi-tri.xac-nhan-da-doc
title: "Xác nhận đã đọc: cần CẢ nhóm, không phải MỘT người"
summary: "daDocBoiCaNhom coi tin nhắn LÀ 'đã đọc bởi cả nhóm' CHỈ khi MỌI thành viên trong trangThaiTheoNguoi đều Ở trạng thái da_doc -- nhóm 3 người vơi an=da_nhan, binh=da_doc, chi=da_doc vẫn trả về false, chỉ khi an CŨNG chuyển sang da_doc mới thành true. 3/4 thành viên da_doc, một người còn da_nhan (chưa doc) vẫn LÀ false -- không phải chuyện đa số."
locale: vi
track: thiet-ke-he-thong
module: thoi-gian-thuc-va-vi-tri
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [sd.xac-nhan-da-doc]
requires: [sd.trang-thai-online]
concepts: [sd.xac-nhan-da-doc]
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
Online/offline chỉ cần MỘT người VÀ một ngưỡng. "Đã đọc" trong nhóm khó
hơn hẳn — một tin nhắn gửi tới MƯỜI người, mỗi người đọc Ở một thời điểm
khác nhau. Khi nào MỚI được gọi LÀ "cả nhóm đã đọc"?
::::

::::explain{#trang-thai-rieng-tung-nguoi}
Trong chat MỘT-một, "đã đọc" chỉ có đúng một câu trả lời. Trong chat
NHÓM, mỗi người nhận giữ một trạng thái RIÊNG — `da_gui`, `da_nhan`, rồi
`da_doc`. Tin nhắn chỉ được coi LÀ "đã đọc bởi cả nhóm" khi TẤT CẢ thành
viên đều đạt `da_doc`, không phải khi MỘT người đạt tới đó:

```typescript title=readonly
type TrangThaiDoc = "da_gui" | "da_nhan" | "da_doc";
interface TinNhanNhom { trangThaiTheoNguoi: Map<string, TrangThaiDoc>; }

function taoTinNhanNhom(dsThanhVien: string[]): TinNhanNhom {
  const map = new Map<string, TrangThaiDoc>();
  for (const tv of dsThanhVien) map.set(tv, "da_gui");
  return { trangThaiTheoNguoi: map };
}
function danhDauDaNhan(tn: TinNhanNhom, nguoi: string): void { tn.trangThaiTheoNguoi.set(nguoi, "da_nhan"); }
function danhDauDaDoc(tn: TinNhanNhom, nguoi: string): void { tn.trangThaiTheoNguoi.set(nguoi, "da_doc"); }

function daDocBoiCaNhom(tn: TinNhanNhom): boolean {
  for (const trangThai of tn.trangThaiTheoNguoi.values()) {
    if (trangThai !== "da_doc") return false;
  }
  return true;
}

const tn = taoTinNhanNhom(["an", "binh", "chi"]);
danhDauDaNhan(tn, "an");
danhDauDaDoc(tn, "binh");
danhDauDaDoc(tn, "chi");
console.log("an=da_nhan, binh=da_doc, chi=da_doc -> ca nhom da doc:", daDocBoiCaNhom(tn));

danhDauDaDoc(tn, "an");
console.log("sau khi an cung da_doc -> ca nhom da doc:", daDocBoiCaNhom(tn));
```

```text title=readonly
an=da_nhan, binh=da_doc, chi=da_doc -> ca nhom da doc: false
sau khi an cung da_doc -> ca nhom da doc: true
```

Hai TRONG ba thành viên đã `da_doc`, chỉ THIẾU `"an"` — nhưng
`daDocBoiCaNhom` vẫn trả về `false`. Vòng lặp `for` duyệt QUA từng trạng
thái trong `trangThaiTheoNguoi`, VÀ chỉ CẦN một người CHƯA `da_doc` LÀ đủ
để dừng lại VÀ trả `false` ngay. Chỉ khi `"an"` cũng chuyển sang
`da_doc`, hàm mới duyệt hết map mà KHÔNG gặp ngoại lệ nào, VÀ trả `true`.
::::

::::example{#chua-nhan-cung-chan-nhu-chua-doc}
`da_gui` (chưa từng nhận) VÀ `da_nhan` (nhận rồi nhưng chưa đọc) đều CHẶN
cờ "cả nhóm đã đọc" GIỐNG hệt nhau — cả hai đều KHÁC `da_doc`:

```typescript title=readonly
type TrangThaiDoc = "da_gui" | "da_nhan" | "da_doc";
interface TinNhanNhom { trangThaiTheoNguoi: Map<string, TrangThaiDoc>; }

function taoTinNhanNhom(dsThanhVien: string[]): TinNhanNhom {
  const map = new Map<string, TrangThaiDoc>();
  for (const tv of dsThanhVien) map.set(tv, "da_gui");
  return { trangThaiTheoNguoi: map };
}
function danhDauDaNhan(tn: TinNhanNhom, nguoi: string): void { tn.trangThaiTheoNguoi.set(nguoi, "da_nhan"); }
function danhDauDaDoc(tn: TinNhanNhom, nguoi: string): void { tn.trangThaiTheoNguoi.set(nguoi, "da_doc"); }

function daDocBoiCaNhom(tn: TinNhanNhom): boolean {
  for (const trangThai of tn.trangThaiTheoNguoi.values()) {
    if (trangThai !== "da_doc") return false;
  }
  return true;
}

// tai lap dung trang thai: nhom 2 nguoi, ca hai bat dau o da_gui
const tn2 = taoTinNhanNhom(["dung", "em"]);
danhDauDaDoc(tn2, "dung");
console.log("dung=da_doc, em van con da_gui (chua nhan) -> ca nhom da doc:", daDocBoiCaNhom(tn2));

danhDauDaNhan(tn2, "em");
console.log("em vua chuyen sang da_nhan (van chua doc) -> ca nhom da doc:", daDocBoiCaNhom(tn2));

danhDauDaDoc(tn2, "em");
console.log("em cung da_doc -> ca nhom da doc:", daDocBoiCaNhom(tn2));
```

```text title=readonly
dung=da_doc, em van con da_gui (chua nhan) -> ca nhom da doc: false
em vua chuyen sang da_nhan (van chua doc) -> ca nhom da doc: false
em cung da_doc -> ca nhom da doc: true
```

`"em"` đi qua HAI trạng thái trung gian (`da_gui` rồi `da_nhan`) trước
khi tới `da_doc` — VÀ Ở CẢ hai bước trung gian đó, `daDocBoiCaNhom` đều
trả về `false`. Hàm không hề phân biệt "chưa nhận" với "nhận rồi nhưng
chưa đọc" — cả hai đều LÀ "chưa `da_doc`", đều chặn cờ như nhau.
::::

::::predict{#doan-da-so-khong-du commitOnce}
Một nhóm CÓ bốn thành viên: `m1`, `m2`, `m3` đã `da_doc`; RIÊNG `m4` mới
chỉ `da_nhan` (đã nhận, CHƯA đọc). `daDocBoiCaNhom` trả về gì?

:::opt{correct}
`false` — CÒN đúng một người (`m4`) chưa đạt `da_doc`, mà điều kiện đòi
hỏi TẤT CẢ thành viên, không phải phần LỚN; vòng lặp gặp `m4` LÀ dừng
ngay VÀ trả `false`
:::
:::opt
`true` — vì 3 trên 4 thành viên (đa số áp đảo) đã đọc, đủ để coi LÀ cả
nhóm đã xem tin nhắn NÀY
::why
Nhầm "đa số đã đọc" VỚI "cả nhóm đã đọc" — nhưng `daDocBoiCaNhom` không
hề đếm TỈ lệ phần trăm hay so sánh với một ngưỡng SỐ lượng nào cả.

Chỗ lệch: vòng lặp `for (const trangThai of tn.trangThaiTheoNguoi.values())`
kiểm TỪNG giá trị một, VÀ `if (trangThai !== "da_doc") return false;` dừng
NGAY khi gặp bất kỳ giá trị nào khác `da_doc` — dù đó LÀ người CUỐI cùng
trong bốn người. Không có phép đếm hay tỉ lệ NÀO ở đây cả.
::
:::
::::

::::code{#viet_da_doc_boi_ca_nhom}
Hoàn thiện `daDocBoiCaNhom` — vòng lặp `for` đã có sẵn, chỉ CẦN điền điều
kiện dừng SỚM khi gặp một trạng thái CHƯA phải `da_doc`.

```typescript title=starter
type TrangThaiDoc = "da_gui" | "da_nhan" | "da_doc";
interface TinNhanNhom { trangThaiTheoNguoi: Map<string, TrangThaiDoc>; }

function taoTinNhanNhom(dsThanhVien: string[]): TinNhanNhom {
  const map = new Map<string, TrangThaiDoc>();
  for (const tv of dsThanhVien) map.set(tv, "da_gui");
  return { trangThaiTheoNguoi: map };
}
function danhDauDaNhan(tn: TinNhanNhom, nguoi: string): void { tn.trangThaiTheoNguoi.set(nguoi, "da_nhan"); }
function danhDauDaDoc(tn: TinNhanNhom, nguoi: string): void { tn.trangThaiTheoNguoi.set(nguoi, "da_doc"); }

function daDocBoiCaNhom(tn: TinNhanNhom): boolean {
  for (const trangThai of tn.trangThaiTheoNguoi.values()) {
    ___
  }
  return true;
}

const tnX = taoTinNhanNhom(["x1", "x2"]);
danhDauDaDoc(tnX, "x1");
danhDauDaDoc(tnX, "x2");
console.log(daDocBoiCaNhom(tnX));
```

```typescript title=solution
type TrangThaiDoc = "da_gui" | "da_nhan" | "da_doc";
interface TinNhanNhom { trangThaiTheoNguoi: Map<string, TrangThaiDoc>; }

function taoTinNhanNhom(dsThanhVien: string[]): TinNhanNhom {
  const map = new Map<string, TrangThaiDoc>();
  for (const tv of dsThanhVien) map.set(tv, "da_gui");
  return { trangThaiTheoNguoi: map };
}
function danhDauDaNhan(tn: TinNhanNhom, nguoi: string): void { tn.trangThaiTheoNguoi.set(nguoi, "da_nhan"); }
function danhDauDaDoc(tn: TinNhanNhom, nguoi: string): void { tn.trangThaiTheoNguoi.set(nguoi, "da_doc"); }

function daDocBoiCaNhom(tn: TinNhanNhom): boolean {
  for (const trangThai of tn.trangThaiTheoNguoi.values()) {
    if (trangThai !== "da_doc") return false;
  }
  return true;
}

const tnX = taoTinNhanNhom(["x1", "x2"]);
danhDauDaDoc(tnX, "x1");
danhDauDaDoc(tnX, "x2");
console.log(daDocBoiCaNhom(tnX));
```

```typescript title=test
const tnRong = taoTinNhanNhom([]);
if (daDocBoiCaNhom(tnRong) !== true) throw new Error("nhom RONG (khong thanh vien) phai coi la da doc (khong co ai chua doc)");

const tn3 = taoTinNhanNhom(["an", "binh", "chi"]);
if (daDocBoiCaNhom(tn3) !== false) throw new Error("moi tao, tat ca con da_gui, chua the la da doc boi ca nhom");

danhDauDaDoc(tn3, "an");
danhDauDaDoc(tn3, "binh");
if (daDocBoiCaNhom(tn3) !== false) throw new Error("chi con thieu MOT nguoi (chi) van chua duoc tinh la da doc ca nhom");

danhDauDaNhan(tn3, "chi");
if (daDocBoiCaNhom(tn3) !== false) throw new Error("chi moi da_nhan (chua da_doc) thi ca nhom van CHUA duoc tinh la da doc");

danhDauDaDoc(tn3, "chi");
if (daDocBoiCaNhom(tn3) !== true) throw new Error("ca ba thanh vien deu da_doc thi ca nhom moi duoc tinh la da doc");

const tn4 = taoTinNhanNhom(["m1", "m2", "m3", "m4"]);
danhDauDaDoc(tn4, "m1");
danhDauDaDoc(tn4, "m2");
danhDauDaDoc(tn4, "m3");
danhDauDaNhan(tn4, "m4");
if (daDocBoiCaNhom(tn4) !== false) throw new Error("3/4 da doc van CHUA du - phai ca 4 nguoi, khong phai da so");
```

:::hints
- kind: attention
  body: "Trong vong lap, khi gap mot trangThai KHAC 'da_doc', phai dung lai NGAY va tra ve false -- khong duoc doi den het vong lap."
- kind: strategy
  body: "So sanh trangThai !== 'da_doc'; neu dung thi return false ngay lap tuc. Chi khi vong lap chay het ma khong return som, ham moi toi duoc dong return true ben duoi."
- kind: one-line
  body: "if (trangThai !== \"da_doc\") return false;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 5000
- tier: output
  match: contains
  expect: "true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một người CHƯA đọc là đủ để chặn cả cờ nhóm. Mảnh kế tiếp rời chat, sang
thông báo — nơi gửi ĐÚNG một lần, qua đúng MỘT kênh chịu nhận nó.
::::

::::reflect{#nghi-lai}
`daDocBoiCaNhom` chỉ LÀ một vòng lặp tìm PHẦN TỬ lệch chuẩn — nhưng điều
đáng nhớ LÀ nó không hề gộp trạng thái LẠI thành một con số (tỉ lệ, đếm)
rồi so SÁNH với ngưỡng. Nó giữ NGUYÊN từng trạng thái riêng lẻ VÀ chỉ hỏi
đúng một câu: "có AI còn thiếu không?"
::::

::::checkpoint{mastery=0.70}
::::
