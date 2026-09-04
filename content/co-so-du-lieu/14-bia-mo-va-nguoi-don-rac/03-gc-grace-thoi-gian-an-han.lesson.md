---
id: co-so-du-lieu.bia-mo-va-nguoi-don-rac.gc-grace-thoi-gian-an-han
title: "gc_grace — thời gian ân hạn"
summary: "coTheXoaBiaMo cho phép purge một bia mộ CHỈ khi nó đã tồn tại đủ lâu: thoiGianHienTai - banGhi.thoiGian >= gcGraceMs. Bia mộ mới (5ms sau khi ghi, gcGraceMs=10000) chưa đủ tuổi -- false. Đủ đúng 10000ms trở lên -- true. Giá trị THẬT (không phải bia mộ), dù đã 'quá hạn' về mặt thời gian, KHÔNG BAO GIỜ bị coi là xoá được -- điều kiện giaTri===null luôn đứng đầu."
locale: vi
track: co-so-du-lieu
module: bia-mo-va-nguoi-don-rac
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [db.gc-grace-thoi-gian-an-han]
requires: [db.bia-mo-hoi-sinh-du-lieu]
concepts: [db.gc-grace-thoi-gian-an-han]
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
Purge quá SỚM (bài trước) hồi sinh dữ liệu đã xoá. Vậy purge phải
CHỜ tới khi nào? Cần một QUY tắc rõ ràng, không phải đoán chừng.
::::

::::explain{#co-the-xoa-bia-mo}
`coTheXoaBiaMo` LÀ quy tắc đó: một bia mộ chỉ được PHÉP purge khi nó
đã tồn tại ĐỦ lâu — `thoiGianHienTai - banGhi.thoiGian >= gcGraceMs`
("gc_grace" — thời gian ÂN hạn dành cho garbage collection). ĐỦ lâu
Ở đây nghĩa LÀ: đủ để MỌI owner sập lúc xoá (như `gamma` Ở bài 1) có
cơ hội hồi phục VÀ được read repair/anti-entropy chữa TRƯỚC khi bằng
chứng "đã xoá" biến MẤT:

```typescript title=readonly
interface BanGhi { giaTri: number | null; thoiGian: number; }

function coTheXoaBiaMo(banGhi: BanGhi, thoiGianHienTai: number, gcGraceMs: number): boolean {
  return banGhi.giaTri === null && (thoiGianHienTai - banGhi.thoiGian) >= gcGraceMs;
}

const biaMo: BanGhi = { giaTri: null, thoiGian: 1000 };
const gcGraceMs = 10000;

console.log("t=1005 (5ms sau xoa):", coTheXoaBiaMo(biaMo, 1005, gcGraceMs));
console.log("t=10999 (9999ms sau xoa):", coTheXoaBiaMo(biaMo, 10999, gcGraceMs));
console.log("t=11000 (dung 10000ms sau xoa):", coTheXoaBiaMo(biaMo, 11000, gcGraceMs));
console.log("t=20000 (qua han lau):", coTheXoaBiaMo(biaMo, 20000, gcGraceMs));
```

```text title=readonly
t=1005 (5ms sau xoa): false
t=10999 (9999ms sau xoa): false
t=11000 (dung 10000ms sau xoa): true
t=20000 (qua han lau): true
```

Bia mộ Ở `thoiGian:1000`. VỚI `gcGraceMs:10000`, nó KHÔNG đủ tuổi
CHO tới đúng `thoiGian:11000` — trước ĐÓ dù chỉ thiếu `1ms`
(`t=10999`), `coTheXoaBiaMo` VẪN trả về `false`. Đây LÀ so sánh
`>=` (không phải `>`) — ĐÚNG `10000ms` đã ĐỦ, không cần hơn.
::::

::::example{#gia-tri-that-khong-bao-gio-purge}
`giaTri === null` LÀ điều kiện ĐẦU tiên, KHÔNG thể bỏ qua: một giá
trị THẬT (không phải bia mộ), dù `thoiGianHienTai` cách xa BAO nhiêu,
KHÔNG BAO GIỜ được `coTheXoaBiaMo` cho phép purge — hàm NÀY chỉ nói
"bia mộ NÀY đủ tuổi CHƯA", không hề LÀ một chính sách hết hạn dữ liệu
NÓI chung.
::::

::::predict{#doan-dung-luc commitOnce}
Bia mộ Ở `thoiGian:1000`, `gcGraceMs:10000`. Kiểm tra ĐÚNG lúc
`thoiGianHienTai:11000` — CHÊNH lệch chính XÁC bằng `gcGraceMs`.
`coTheXoaBiaMo` trả VỀ gì?

:::opt{correct}
`true` — điều kiện dùng `>=`, NÊN "đủ ĐÚNG bằng gcGraceMs" ĐÃ được
tính LÀ đủ tuổi, không cần chờ THÊM
:::

:::opt
`false` — PHẢI vượt QUA `gcGraceMs`, chạm ĐÚNG mốc CHƯA đủ
::why
Trực giác NÀY hợp lý cho một số hệ THỐNG dùng so sánh `>` nghiêm
ngặt Ở chỗ KHÁC (VÍ dụ `banMoiNhat` q13/q14 dùng ĐÚNG `>` nghiêm
ngặt để phá thế HOÀ) — dễ nhầm LÀ mọi ngưỡng thời gian đều dùng CÙNG
kiểu so sánh.

Chỗ lệch: `coTheXoaBiaMo` viết RÕ `>=`, KHÔNG phải `>` — `thoiGianHienTai
- banGhi.thoiGian >= gcGraceMs` đúng BẰNG thì VẪN qua được điều kiện.
::
:::
::::

::::code{#viet_co_the_xoa_bia_mo}
Hoàn thiện `coTheXoaBiaMo` — bia mộ chỉ được PHÉP xoá khi `giaTri`
LÀ `null` VÀ đã đủ `gcGraceMs` kể TỪ `thoiGian` của nó.

```typescript title=starter
interface BanGhi { giaTri: number | null; thoiGian: number; }

function coTheXoaBiaMo(banGhi: BanGhi, thoiGianHienTai: number, gcGraceMs: number): boolean {
  return ___;
}

const biaMo: BanGhi = { giaTri: null, thoiGian: 1000 };
console.log(coTheXoaBiaMo(biaMo, 11000, 10000));
```

```typescript title=solution
interface BanGhi { giaTri: number | null; thoiGian: number; }

function coTheXoaBiaMo(banGhi: BanGhi, thoiGianHienTai: number, gcGraceMs: number): boolean {
  return banGhi.giaTri === null && (thoiGianHienTai - banGhi.thoiGian) >= gcGraceMs;
}

const biaMo: BanGhi = { giaTri: null, thoiGian: 1000 };
console.log(coTheXoaBiaMo(biaMo, 11000, 10000));
```

```typescript title=test
if (coTheXoaBiaMo({ giaTri: null, thoiGian: 1000 }, 1005, 10000) !== false) throw new Error("moi xoa 5ms truoc -- chua du tuoi, phai la false");
if (coTheXoaBiaMo({ giaTri: null, thoiGian: 1000 }, 10999, 10000) !== false) throw new Error("thieu dung 1ms -- van phai la false");
if (coTheXoaBiaMo({ giaTri: null, thoiGian: 1000 }, 11000, 10000) !== true) throw new Error("dung bang gcGraceMs -- phai la true (>=)");
if (coTheXoaBiaMo({ giaTri: null, thoiGian: 1000 }, 20000, 10000) !== true) throw new Error("qua han lau -- van phai la true");
if (coTheXoaBiaMo({ giaTri: 100, thoiGian: 1000 }, 999999, 10000) !== false) throw new Error("gia tri THAT (khong phai bia mo) khong bao gio duoc phep xoa, du qua han xa");
if (coTheXoaBiaMo({ giaTri: 0, thoiGian: 1000 }, 999999, 10000) !== false) throw new Error("giaTri=0 la mot gia tri THAT (khac null) -- van khong duoc phep xoa");
```

:::hints
- kind: attention
  body: "Hai dieu kien AND: giaTri phai la null, VA da qua du gcGraceMs -- mot bieu thuc."
- kind: strategy
  body: "return banGhi.giaTri === null && (thoiGianHienTai - banGhi.thoiGian) >= gcGraceMs;"
- kind: one-line
  body: "return banGhi.giaTri === null && (thoiGianHienTai - banGhi.thoiGian) >= gcGraceMs;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`coTheXoaBiaMo` biết KHI nào một bia mộ ĐỦ tuổi. Nhưng biết "đủ tuổi"
không có nghĩa LÀ nó tự động BIẾN mất khỏi kho — ai THẬT sự dọn nó?
::::

::::reflect{#nghi-lai}
`coTheXoaBiaMo` chỉ trả lời ĐÚNG một câu hỏi: "bia mộ NÀY có ĐỦ điều
kiện bị xoá THẬT chưa?" — nó KHÔNG hề tự XOÁ gì cả. `gcGraceMs` LÀ
một con SỐ do người vận hành CHỌN: quá NGẮN thì lặp lại đúng LỖI
zombie Ở bài 2 (owner sập LÂU hơn gcGraceMs sẽ KHÔNG kịp được chữa
trước khi bia mộ biến mất); quá DÀI thì bia mộ tích luỹ (bài SAU),
tốn chỗ VÀ làm chậm đọc. Nhưng "đủ điều KIỆN xoá" và "đã bị XOÁ" LÀ
hai chuyện khác nhau — không có compaction, bia mộ VẪN nằm nguyên
Ở kho.
::::

::::checkpoint{mastery=0.8}
::::
