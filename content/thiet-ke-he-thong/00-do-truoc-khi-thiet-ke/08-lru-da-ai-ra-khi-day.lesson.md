---
id: thiet-ke-he-thong.do-truoc-khi-thiet-ke.lru-da-ai-ra-khi-day
title: "LRU: đá ai ra khi đầy"
summary: "CacheLRU dùng Map (giữ thứ tự chèn) làm hàng đợi 'lâu không dùng nhất'. truyCap xoá-rồi-set lại khoá để đẩy nó về cuối (mới nhất). themVaoCacheLRU khi ĐẦY thì xoá đúng phần tử ĐẦU tiên của Map (lâu không dùng nhất) trước khi chèn phần tử mới. Với dungLuongToiDa=3: chèn a,b,c -> thứ tự a,b,c; truyCap(a) đưa a về cuối -> b,c,a; chèn d (đầy) đá b (không phải a, dù a chèn TRƯỚC b -- vì a vừa được truy cập lại) -> kết quả c,a,d, kích thước vẫn đúng 3."
locale: vi
track: thiet-ke-he-thong
module: do-truoc-khi-thiet-ke
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [sd.lru-da-ai-ra-khi-day]
requires: [sd.cache-ben-canh-doc-va-ghi]
concepts: [sd.lru-da-ai-ra-khi-day]
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
Cache-aside (bài trước) không hề GIỚI hạn dung lượng — cứ ghi MÃI thì
cache LỚN dần vô hạn, đúng thứ nó được sinh RA để tránh (tràn bộ
nhớ). Cần một quy TẮC: khi ĐẦY, đá ai RA?
::::

::::explain{#lru}
LRU (Least Recently Used) — đá phần TỬ LÂU không được dùng NHẤT.
`Map` trong JavaScript giữ đúng THỨ tự CHÈN — TA tận dụng ĐIỀU đó:
phần tử Ở ĐẦU `Map` LÀ phần tử LÂU không được "chạm" tới nhất.
`truyCap` đọc một khoá VÀ đẩy nó VỀ cuối (xoá RỒI chèn lại — "MỚI
dùng gần đây nhất"):

```typescript title=readonly
interface CacheLRU { dungLuongToiDa: number; duLieu: Map<string, number>; }
function taoCacheLRU(dungLuongToiDa: number): CacheLRU {
  return { dungLuongToiDa, duLieu: new Map() };
}

function truyCap(cache: CacheLRU, khoa: string): number | undefined {
  if (!cache.duLieu.has(khoa)) return undefined;
  const giaTri = cache.duLieu.get(khoa)!;
  cache.duLieu.delete(khoa);
  cache.duLieu.set(khoa, giaTri);
  return giaTri;
}

function themVaoCacheLRU(cache: CacheLRU, khoa: string, giaTri: number): void {
  if (cache.duLieu.has(khoa)) {
    cache.duLieu.delete(khoa);
  } else if (cache.duLieu.size >= cache.dungLuongToiDa) {
    cache.duLieu.delete(cache.duLieu.keys().next().value as string);
  }
  cache.duLieu.set(khoa, giaTri);
}

const cache = taoCacheLRU(3);
themVaoCacheLRU(cache, "a", 1);
themVaoCacheLRU(cache, "b", 2);
themVaoCacheLRU(cache, "c", 3);
console.log("sau khi them a,b,c:", [...cache.duLieu.keys()].join(","));

console.log("truy cap a:", truyCap(cache, "a"));
console.log("thu tu sau truy cap a:", [...cache.duLieu.keys()].join(","));
```

```text title=readonly
sau khi them a,b,c: a,b,c
truy cap a: 1
thu tu sau truy cap a: b,c,a
```

Chèn `a`, `b`, `c` THEO đúng thứ tự — `Map` giữ NGUYÊN thứ tự đó.
`truyCap(cache, "a")` XOÁ `a` khỏi vị trí ĐẦU RỒI chèn LẠI Ở cuối —
thứ tự MỚI LÀ `b, c, a`: `b` giờ Ở ĐẦU (lâu không dùng NHẤT), `a` Ở
cuối (vừa dùng).
::::

::::example{#day-thi-da}
Cache ĐANG đầy (`3/3`). Chèn thêm `"d"` — `themVaoCacheLRU` PHẢI đá
MỘT phần tử ra TRƯỚC khi chèn:

```typescript title=readonly
interface CacheLRU { dungLuongToiDa: number; duLieu: Map<string, number>; }
function taoCacheLRU(dungLuongToiDa: number): CacheLRU {
  return { dungLuongToiDa, duLieu: new Map() };
}
function truyCap(cache: CacheLRU, khoa: string): number | undefined {
  if (!cache.duLieu.has(khoa)) return undefined;
  const giaTri = cache.duLieu.get(khoa)!;
  cache.duLieu.delete(khoa);
  cache.duLieu.set(khoa, giaTri);
  return giaTri;
}
function themVaoCacheLRU(cache: CacheLRU, khoa: string, giaTri: number): void {
  if (cache.duLieu.has(khoa)) {
    cache.duLieu.delete(khoa);
  } else if (cache.duLieu.size >= cache.dungLuongToiDa) {
    cache.duLieu.delete(cache.duLieu.keys().next().value as string);
  }
  cache.duLieu.set(khoa, giaTri);
}

// tai lap dung trang thai da co o khoi truoc: them a,b,c roi truy cap a (thu tu: b,c,a)
const cache = taoCacheLRU(3);
themVaoCacheLRU(cache, "a", 1);
themVaoCacheLRU(cache, "b", 2);
themVaoCacheLRU(cache, "c", 3);
truyCap(cache, "a");

themVaoCacheLRU(cache, "d", 4);
console.log("sau khi them d (day):", [...cache.duLieu.keys()].join(","));
console.log("b con trong cache khong:", cache.duLieu.has("b"));
console.log("kich thuoc cache:", cache.duLieu.size);
```

```text title=readonly
sau khi them d (day): c,a,d
b con trong cache khong: false
kich thuoc cache: 3
```

Trước khi chèn `d`, thứ TỰ LÀ `b, c, a` (`b` Ở đầu). `themVaoCacheLRU`
xoá đúng phần TỬ đầu tiên (`cache.duLieu.keys().next().value`) — LÀ
`b`, KHÔNG phải `a` dù `a` được chèn TRƯỚC `b` VỀ mặt thời gian gốc —
Vì `a` VỪA được `truyCap` LÀM mới lại vị trí. Kết quả cuối: `c, a,
d`, kích thước vẫn đúng `3` (không phình RA `4`).
::::

::::predict{#doan-truy-cap-truoc-khi-day commitOnce}
NGAY trước khi cache đầy, gọi `truyCap` trên một khoá ĐÃ có sẵn
trong cache. Vị trí "lâu không dùng NHẤT" của khoá ĐÓ thay đổi thế
nào?

:::opt{correct}
Khoá đó trở thành "MỚI dùng gần đây nhất" — không còn LÀ ứng viên bị
đá ĐẦU tiên nữa, VÌ `truyCap` xoá RỒI chèn lại nó VÀO cuối `Map`
:::
:::opt
KHÔNG đổi gì — `truyCap` chỉ ĐỌC dữ liệu, không ghi, NÊN không ảnh
hưởng tới thứ tự "lâu không dùng"
::why
Nhầm "đọc" (không ghi GIÁ trị mới) VỚI "không có tác dụng PHỤ nào cả"
— nhưng LRU chính LÀ một cấu trúc mà THAO tác đọc VẪN có tác dụng
phụ (cập nhật thứ tự).

Chỗ lệch: `truyCap` gọi `cache.duLieu.delete(khoa)` RỒI
`cache.duLieu.set(khoa, giaTri)` — hai THAO tác GHI thật sự lên
`Map`, dù giá trị KHÔNG đổi. Đây chính LÀ cách LRU "nhớ" ai vừa được
dùng: MỖI lần đọc đều LÀM mới vị trí, không chỉ CÓ lần ghi mới đếm.
::
:::
::::

::::code{#viet_them_vao_cache_lru}
Hoàn thiện `themVaoCacheLRU` — khi cache ĐẦY (VÀ khoá chưa CÓ sẵn),
xoá đúng phần tử ĐẦU tiên của `Map` (lâu không dùng NHẤT) trước khi
chèn phần tử mới.

```typescript title=starter
interface CacheLRU { dungLuongToiDa: number; duLieu: Map<string, number>; }
function taoCacheLRU(dungLuongToiDa: number): CacheLRU {
  return { dungLuongToiDa, duLieu: new Map() };
}

function truyCap(cache: CacheLRU, khoa: string): number | undefined {
  if (!cache.duLieu.has(khoa)) return undefined;
  const giaTri = cache.duLieu.get(khoa)!;
  cache.duLieu.delete(khoa);
  cache.duLieu.set(khoa, giaTri);
  return giaTri;
}

function themVaoCacheLRU(cache: CacheLRU, khoa: string, giaTri: number): void {
  if (cache.duLieu.has(khoa)) {
    cache.duLieu.delete(khoa);
  } else if (cache.duLieu.size >= cache.dungLuongToiDa) {
    ___
  }
  cache.duLieu.set(khoa, giaTri);
}

const cache = taoCacheLRU(2);
themVaoCacheLRU(cache, "x", 1);
themVaoCacheLRU(cache, "y", 2);
themVaoCacheLRU(cache, "z", 3);
console.log([...cache.duLieu.keys()].join(","));
```

```typescript title=solution
interface CacheLRU { dungLuongToiDa: number; duLieu: Map<string, number>; }
function taoCacheLRU(dungLuongToiDa: number): CacheLRU {
  return { dungLuongToiDa, duLieu: new Map() };
}

function truyCap(cache: CacheLRU, khoa: string): number | undefined {
  if (!cache.duLieu.has(khoa)) return undefined;
  const giaTri = cache.duLieu.get(khoa)!;
  cache.duLieu.delete(khoa);
  cache.duLieu.set(khoa, giaTri);
  return giaTri;
}

function themVaoCacheLRU(cache: CacheLRU, khoa: string, giaTri: number): void {
  if (cache.duLieu.has(khoa)) {
    cache.duLieu.delete(khoa);
  } else if (cache.duLieu.size >= cache.dungLuongToiDa) {
    cache.duLieu.delete(cache.duLieu.keys().next().value as string);
  }
  cache.duLieu.set(khoa, giaTri);
}

const cache = taoCacheLRU(2);
themVaoCacheLRU(cache, "x", 1);
themVaoCacheLRU(cache, "y", 2);
themVaoCacheLRU(cache, "z", 3);
console.log([...cache.duLieu.keys()].join(","));
```

```typescript title=test
function layKichThuoc(c: CacheLRU): number { return c.duLieu.size; }

const cacheT = taoCacheLRU(3);
themVaoCacheLRU(cacheT, "a", 1);
themVaoCacheLRU(cacheT, "b", 2);
themVaoCacheLRU(cacheT, "c", 3);
if ([...cacheT.duLieu.keys()].join(",") !== "a,b,c") throw new Error("sau khi them a,b,c thu tu phai la a,b,c");
if (layKichThuoc(cacheT) !== 3) throw new Error("kich thuoc phai la 3 sau khi them 3 phan tu");

if (truyCap(cacheT, "a") !== 1) throw new Error("truyCap('a') phai tra ve 1");
if ([...cacheT.duLieu.keys()].join(",") !== "b,c,a") throw new Error("sau truyCap('a'), thu tu phai la b,c,a (a duoc day ve cuoi)");

themVaoCacheLRU(cacheT, "d", 4);
if ([...cacheT.duLieu.keys()].join(",") !== "c,a,d") throw new Error("sau khi them d (day), thu tu phai la c,a,d (b bi da vi lau khong dung nhat)");
if (cacheT.duLieu.has("b")) throw new Error("b phai bi da khoi cache");
if (layKichThuoc(cacheT) !== 3) throw new Error("kich thuoc PHAI van la 3 (khong duoc phinh len 4)");

if (truyCap(cacheT, "khong-ton-tai") !== undefined) throw new Error("truy cap khoa khong ton tai phai tra ve undefined");
```

:::hints
- kind: attention
  body: "Xoa phan tu DAU TIEN cua Map (lau khong dung nhat) truoc khi chen phan tu moi -- mot dong."
- kind: strategy
  body: "cache.duLieu.delete(cache.duLieu.keys().next().value as string);"
- kind: one-line
  body: "cache.duLieu.delete(cache.duLieu.keys().next().value as string);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "y,z"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cache giờ có giới HẠN dung lượng. Nhưng dung lượng KHÔNG phải trục
duy nhất — một giá trị CŨ (dù cache còn CHỖ) vẫn có thể SAI lệch theo
thời gian.
::::

::::reflect{#nghi-lai}
`themVaoCacheLRU` chỉ thêm ĐÚNG một nhánh (`else if` kiểm tra ĐẦY)
vào một hàm ghi Map thông THƯỜNG — nhưng "phần tử ĐẦU tiên của Map"
CHỈ đúng nghĩa "lâu không dùng nhất" NẾU `truyCap` LUÔN đẩy khoá VỀ
cuối khi đọc. Hai hàm NÀY phụ thuộc lẫn nhau — bỏ MỘT trong hai LÀ
LRU vỡ ngay LẬP tức.
::::

::::checkpoint{mastery=0.8}
::::
