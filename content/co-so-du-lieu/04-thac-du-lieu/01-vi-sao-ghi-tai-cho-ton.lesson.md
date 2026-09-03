---
id: co-so-du-lieu.thac-du-lieu.vi-sao-ghi-tai-cho-ton
title: Vì sao ghi tại chỗ tốn
summary: "Mỗi lần ghi VÀO một B+Tree (q03) phải chạm ĐĨA ngay lúc đó — tìm đúng lá, có thể tách, có thể cập nhật cha. Nếu ghi TRƯỚC vào bộ nhớ (memtable), chỉ chạm đĩa MỘT lần khi memtable đầy — flush cả LÔ cùng lúc thay vì từng lần riêng lẻ."
locale: vi
track: co-so-du-lieu
module: thac-du-lieu
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.why-write-elsewhere-first]
requires: [db.range-scan-via-la-tiep]
concepts: [db.why-write-elsewhere-first]
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
q03 xây một B+Tree — TỪNG lần chèn đi TỪ gốc xuống LÁ, ghi ngay LÚC
đó. Nếu ghi TRƯỚC vào bộ nhớ, rồi mới chạm đĩa SAU, có gì khác?
::::

::::explain{#ghi-tai-cho-vs-ghi-sau}
Mỗi lần `chen_va_tach_neu_can` (q03) chạy, nó CHẠM đĩa NGAY — tìm lá,
có THỂ tách, có thể cập nhật cha, TẤT cả trong CÙNG một lần gọi:

```typescript title=readonly
function demChamDiaGhiTaiCho(soLanGhi: number): number {
  return soLanGhi;
}

function demChamDiaGhiTrenMemtable(soLanGhi: number, kichThuocMemtable: number): number {
  return Math.ceil(soLanGhi / kichThuocMemtable);
}

console.log(demChamDiaGhiTaiCho(1000));
console.log(demChamDiaGhiTrenMemtable(1000, 100));
```

```text title=readonly
1000
10
```

`demChamDiaGhiTaiCho`: mỗi lần GHI là một lần chạm đĩa — `1000` lần
ghi LÀ `1000` lần chạm. `demChamDiaGhiTrenMemtable`: gom `1000` lần
ghi VÀO một vùng nhớ (`kichThuocMemtable=100`), CHỈ chạm đĩa khi vùng
đó ĐẦY — `1000 / 100 = 10` lần chạm, GẤP trăm lần ÍT hơn.
::::

::::example{#khong-chia-het}
Số lần ghi KHÔNG chia hết cho kích thước memtable — lần CUỐI vẫn
phải flush dù CHƯA đầy hẳn:

```typescript title=readonly
console.log(demChamDiaGhiTrenMemtable(7, 10));
```

```text title=readonly
1
```

Chỉ `7` lần ghi, memtable giữ TỚI `10` — KHÔNG đầy, nhưng vẫn phải
flush MỘT lần (khi hệ thống dừng, HOẶC khi cần đọc dữ liệu chưa lên
đĩa). `Math.ceil` LÀM tròn LÊN, không phải xuống — dù chỉ ĐẦY một
phần, vẫn tính LÀ một lần chạm.
::::

::::predict{#doan-so-lan-cham-dia commitOnce}
Byte ghi `250` lần, memtable giữ TỚI `100` bản ghi TRƯỚC khi đầy:

```typescript
console.log(demChamDiaGhiTrenMemtable(250, 100));
```

Dòng cuối in ra gì?

:::opt{correct}
`3`
:::

:::opt
`2` — vì `250` chia `100` được ĐÚNG `2` lần trọn vẹn (`200`), phần
CÒN dư (`50`) chưa đủ MỘT memtable nên không tính
::why
Gần đúng ở việc bạn tính đúng phần TRỌN vẹn — `250 / 100 = 2.5`,
phần nguyên LÀ `2`, một quan sát chính xác.

Chỗ lệch: `Math.ceil` LÀM tròn LÊN, không phải cắt bỏ phần LẺ —
`Math.ceil(2.5) = 3`, không phải `2`. `50` bản ghi CÒN lại VẪN nằm
trong RAM, chưa lên đĩa — chúng vẫn cần MỘT lần flush riêng, dù
memtable CHƯA đầy hẳn khi đó xảy ra (giống VÍ dụ `7`/`10` ở trên).
::
:::

:::opt
`250` — vì mỗi bản ghi VẪN phải được lưu XUỐNG đĩa CUỐI cùng, dùng
memtable chỉ TRÌ hoãn chứ không giảm SỐ lần chạm thật sự
::why
Gần đúng ở việc bạn để Ý: DỮ liệu cuối cùng VẪN phải nằm trên đĩa
— đúng, memtable không phải nơi lưu VĨNH viễn.

Chỗ lệch: `demChamDiaGhiTrenMemtable` đếm số lần CHẠM đĩa (số lần
FLUSH), không đếm số BẢN ghi. Một lần flush GHI cả một LÔ (tới
`100` bản ghi) trong MỘT thao tác — `250` bản ghi gộp thành CHỈ `3`
thao tác flush, không phải `250` thao tác riêng LẺ.
::
:::
::::

::::code{#viet_dem_cham_dia_memtable}
Hoàn thiện `demChamDiaGhiTrenMemtable(soLanGhi, kichThuocMemtable)`
— số lần chạm đĩa LÀ số lần memtable đầy (LÀM tròn LÊN).

```typescript title=starter
function demChamDiaGhiTaiCho(soLanGhi: number): number {
  return soLanGhi;
}

function demChamDiaGhiTrenMemtable(soLanGhi: number, kichThuocMemtable: number): number {
  return ___;
}

console.log(demChamDiaGhiTaiCho(1000), demChamDiaGhiTrenMemtable(1000, 100));
```

```typescript title=solution
function demChamDiaGhiTaiCho(soLanGhi: number): number {
  return soLanGhi;
}

function demChamDiaGhiTrenMemtable(soLanGhi: number, kichThuocMemtable: number): number {
  return Math.ceil(soLanGhi / kichThuocMemtable);
}

console.log(demChamDiaGhiTaiCho(1000), demChamDiaGhiTrenMemtable(1000, 100));
```

```typescript title=test
if (demChamDiaGhiTrenMemtable(1000, 100) !== 10) throw new Error("1000 lan ghi, memtable 100 -- phai flush 10 lan");
if (demChamDiaGhiTrenMemtable(7, 10) !== 1) throw new Error("7 lan ghi, memtable 10 -- van phai flush 1 lan du chua day");
if (demChamDiaGhiTrenMemtable(250, 100) !== 3) throw new Error("250 lan ghi, memtable 100 -- phai flush 3 lan, lam tron LEN");
if (demChamDiaGhiTrenMemtable(0, 100) !== 0) throw new Error("khong ghi gi thi khong flush lan nao");
if (demChamDiaGhiTaiCho(1000) !== 1000) throw new Error("ghi tai cho -- moi lan ghi la mot lan cham dia");
```

:::hints
- kind: attention
  body: "So lan flush la soLanGhi chia kichThuocMemtable, LAM TRON LEN -- dung Math.ceil, khong phai chia nguyen."
- kind: strategy
  body: "Math.ceil(soLanGhi / kichThuocMemtable) -- chia thuong roi lam tron len."
- kind: one-line
  body: "Math.ceil(soLanGhi / kichThuocMemtable)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "1000 10"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ghi trước vào bộ nhớ, chạm đĩa ÍT hơn hẳn. Nhưng bộ nhớ đó phải
TRÔNG như gì, để vẫn tìm được NHANH TRƯỚC khi flush?
::::

::::reflect{#nghi-lai}
B+Tree (q03) chạm đĩa NGAY mỗi lần ghi — đúng VỊ trí, đúng lúc, chi
phí trả NGAY tại chỗ. Gom nhiều lần ghi vào một vùng nhớ (memtable)
trước khi chạm đĩa giảm HẲN số lần chạm — đổi LẠI, bây giờ CẦN một
cấu trúc trong bộ nhớ vừa nhận ghi NHANH, vừa giữ được thứ tự để
đọc lại nhanh. Cấu trúc đó trông NHƯ thế nào?
::::

::::checkpoint{mastery=0.8}
::::
