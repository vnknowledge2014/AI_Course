---
id: co-so-du-lieu.thac-du-lieu.cap-nhat-va-xoa
title: Cập nhật và xoá — tombstone
summary: "Xoá một khoá KHÔNG xoá thật ngay — nó ghi một 'bia mộ': một bản ghi bình thường, chỉ khác Ở chỗ giaTri LÀ null thay vì một chuỗi thật. timMemtable phân biệt RÕ ba trạng thái: giá trị thật, null (đã xoá), undefined (chưa từng thấy)."
locale: vi
track: co-so-du-lieu
module: thac-du-lieu
order: 11
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [db.tombstone-delete]
requires: [db.bloom-skips-sstable-scan]
concepts: [db.tombstone-delete]
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
Bloom lọc trước khi quét (bài trước) hoàn thiện việc ĐỌC. Nhưng
XOÁ một khoá thì sao — dữ liệu đã nằm TRÊN SSTable bất biến, không
sửa TẠI chỗ được, làm sao "biến mất"?
::::

::::explain{#xoa-bang-bia-mo}
Xoá một khoá KHÔNG xoá thật ngay — nó ghi một "bia mộ": một bản
ghi BÌNH thường, chỉ khác Ở chỗ `giaTri` LÀ `null` thay vì một
chuỗi thật. `xoaMemtable` chỉ đơn giản GỌI LẠI `chenMemtable` với
`giaTri = null`:

```typescript title=readonly
interface Memtable {
  khoa: string[];
  giaTri: (string | null)[];
}

function taoMemtable(): Memtable {
  return { khoa: [], giaTri: [] };
}

function chenMemtable(bang: Memtable, khoa: string, giaTri: string | null): void {
  let lo = 0;
  let hi = bang.khoa.length;
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    const khoaGiua = bang.khoa[mid];
    if (khoaGiua !== undefined && khoaGiua < khoa) {
      lo = mid + 1;
    } else {
      hi = mid;
    }
  }
  if (bang.khoa[lo] === khoa) { bang.giaTri[lo] = giaTri; } else { bang.khoa.splice(lo, 0, khoa); bang.giaTri.splice(lo, 0, giaTri); }
}

function timMemtable(bang: Memtable, khoa: string): string | null | undefined {
  let lo = 0;
  let hi = bang.khoa.length;
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    const khoaGiua = bang.khoa[mid];
    if (khoaGiua === khoa) return bang.giaTri[mid];
    if (khoaGiua !== undefined && khoaGiua < khoa) {
      lo = mid + 1;
    } else {
      hi = mid;
    }
  }
  return undefined;
}

function xoaMemtable(bang: Memtable, khoa: string): void {
  chenMemtable(bang, khoa, null);
}

const m = taoMemtable();
chenMemtable(m, "cam", "5000");
chenMemtable(m, "buoi", "8000");
chenMemtable(m, "tao", "12000");
xoaMemtable(m, "cam");
console.log(timMemtable(m, "cam"));
```

```text title=readonly
null
```

`xoaMemtable(m, "cam")` gọi `chenMemtable(m, "cam", null)` — VÌ
`"cam"` đã tồn tại, nhánh GHI đè chạy, `giaTri` tại vị trí đó đổi
thành `null`. `timMemtable(m, "cam")` sau đó trả về ĐÚNG `null` —
"đã TỪNG có, nhưng giờ bị xoá", KHÔNG phải "chưa từng thấy".
::::

::::example{#phan-biet-null-va-undefined}
Một khoá CHƯA từng thêm — tìm nó trả về `undefined`, KHÁC hẳn
`null`:

```typescript title=readonly
console.log(String(timMemtable(m, "xoai")));
console.log(timMemtable(m, "buoi"));
```

```text title=readonly
undefined
8000
```

`"xoai"` KHÔNG hề xuất hiện Ở đâu — `timMemtable` quét HẾT không
khớp, trả về `undefined` (biến "chưa từng biết"). `"buoi"` chưa hề
bị xoá — VẪN trả về giá trị THẬT `"8000"`. Ba trạng thái, ba giá
trị KHÁC nhau: giá trị thật, `null` (đã xoá), `undefined` (chưa
từng thấy). Bọc `undefined` bằng `String(...)` trước khi in — nếu
KHÔNG, nó biến thành DÒNG trống, không phải chữ "undefined".
::::

::::predict{#doan-do-dai-sau-xoa commitOnce}
Memtable đang có ba khoá. Byte xoá `"cam"`, rồi hỏi memtable còn
BAO nhiêu khoá:

```typescript
xoaMemtable(m, "cam");
console.log(m.khoa.length);
```

Dòng cuối in ra gì?

:::opt{correct}
`3`
:::

:::opt
`2` — vì xoá MỘT khoá phải LÀM giảm số lượng phần tử TRONG mảng,
giống `Array.prototype.filter` loại bỏ HẲN phần tử đó
::why
Gần đúng ở việc bạn nghĩ TỚI ý nghĩa THÔNG thường của "xoá" — bỏ
HẲN một phần tử RA khỏi cấu trúc, một trực giác đúng với NHIỀU thao
tác xoá khác (như `Array.prototype.filter`, `Map.delete`).

Chỗ lệch: `xoaMemtable` KHÔNG hề gọi `splice` để loại bỏ — nó gọi
LẠI `chenMemtable` với `giaTri=null`, và `"cam"` ĐÃ tồn tại sẵn
TRONG `khoa`, nên nhánh chạy LÀ ghi đè `giaTri` tại vị trí đó, HOÀN
toàn không đụng tới `khoa`. Độ dài `khoa` không đổi — bia mộ LÀ một
bản ghi, không phải một lỗ trống.
::
:::

:::opt
Máy báo lỗi — vì xoá MỘT khoá rồi lại tìm nó (`timMemtable`) LÀ
thao tác không hợp lệ trên một khoá đã BỊ xoá
::why
Gần đúng ở việc bạn nghĩ TỚI một ràng buộc "khoá đã xoá thì không
đụng tới NỮA" hợp lý cho một số hệ thống.

Chỗ lệch: `timMemtable` KHÔNG hề `throw` cho bất kỳ khoá NÀO, kể
cả khoá đã xoá — nó LUÔN trả về MỘT trong ba trạng thái (giá trị
thật, `null`, hoặc `undefined`), không có ngoại lệ.
::
:::
::::

::::code{#viet_xoa_memtable}
Hoàn thiện `xoaMemtable(bang, khoa)` — ghi một bia mộ bằng cách
gọi lại `chenMemtable` với giá trị `null`.

```typescript title=starter
interface Memtable {
  khoa: string[];
  giaTri: (string | null)[];
}

function taoMemtable(): Memtable {
  return { khoa: [], giaTri: [] };
}

function chenMemtable(bang: Memtable, khoa: string, giaTri: string | null): void {
  let lo = 0;
  let hi = bang.khoa.length;
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    const khoaGiua = bang.khoa[mid];
    if (khoaGiua !== undefined && khoaGiua < khoa) {
      lo = mid + 1;
    } else {
      hi = mid;
    }
  }
  if (bang.khoa[lo] === khoa) { bang.giaTri[lo] = giaTri; } else { bang.khoa.splice(lo, 0, khoa); bang.giaTri.splice(lo, 0, giaTri); }
}

function timMemtable(bang: Memtable, khoa: string): string | null | undefined {
  let lo = 0;
  let hi = bang.khoa.length;
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    const khoaGiua = bang.khoa[mid];
    if (khoaGiua === khoa) return bang.giaTri[mid];
    if (khoaGiua !== undefined && khoaGiua < khoa) {
      lo = mid + 1;
    } else {
      hi = mid;
    }
  }
  return undefined;
}

function xoaMemtable(bang: Memtable, khoa: string): void {
  ___
}

const m = taoMemtable();
chenMemtable(m, "cam", "5000");
chenMemtable(m, "buoi", "8000");
chenMemtable(m, "tao", "12000");
xoaMemtable(m, "cam");
console.log(timMemtable(m, "cam"), String(timMemtable(m, "xoai")), timMemtable(m, "buoi"));
```

```typescript title=solution
interface Memtable {
  khoa: string[];
  giaTri: (string | null)[];
}

function taoMemtable(): Memtable {
  return { khoa: [], giaTri: [] };
}

function chenMemtable(bang: Memtable, khoa: string, giaTri: string | null): void {
  let lo = 0;
  let hi = bang.khoa.length;
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    const khoaGiua = bang.khoa[mid];
    if (khoaGiua !== undefined && khoaGiua < khoa) {
      lo = mid + 1;
    } else {
      hi = mid;
    }
  }
  if (bang.khoa[lo] === khoa) { bang.giaTri[lo] = giaTri; } else { bang.khoa.splice(lo, 0, khoa); bang.giaTri.splice(lo, 0, giaTri); }
}

function timMemtable(bang: Memtable, khoa: string): string | null | undefined {
  let lo = 0;
  let hi = bang.khoa.length;
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    const khoaGiua = bang.khoa[mid];
    if (khoaGiua === khoa) return bang.giaTri[mid];
    if (khoaGiua !== undefined && khoaGiua < khoa) {
      lo = mid + 1;
    } else {
      hi = mid;
    }
  }
  return undefined;
}

function xoaMemtable(bang: Memtable, khoa: string): void {
  chenMemtable(bang, khoa, null);
}

const m = taoMemtable();
chenMemtable(m, "cam", "5000");
chenMemtable(m, "buoi", "8000");
chenMemtable(m, "tao", "12000");
xoaMemtable(m, "cam");
console.log(timMemtable(m, "cam"), String(timMemtable(m, "xoai")), timMemtable(m, "buoi"));
```

```typescript title=test
const m2 = taoMemtable();
chenMemtable(m2, "cam", "5000");
chenMemtable(m2, "buoi", "8000");
chenMemtable(m2, "tao", "12000");
if (timMemtable(m2, "cam") !== "5000") throw new Error("truoc khi xoa, cam phai co gia tri that");
xoaMemtable(m2, "cam");
if (timMemtable(m2, "cam") !== null) throw new Error("sau khi xoa, tim cam phai ra null, khong phai gia tri cu hay undefined");
if (timMemtable(m2, "xoai") !== undefined) throw new Error("xoai chua tung them -- phai la undefined, khong phai null");
if (timMemtable(m2, "buoi") !== "8000") throw new Error("buoi chua bi xoa -- van phai giu gia tri that");
if (m2.khoa.length !== 3) throw new Error("xoa khong duoc lam giam so khoa -- bia mo van la mot ban ghi");
```

:::hints
- kind: attention
  body: "xoaMemtable chi can goi lai chenMemtable, truyen null lam gia tri -- mot dong duy nhat."
- kind: strategy
  body: "chenMemtable(bang, khoa, null);"
- kind: one-line
  body: "chenMemtable(bang, khoa, null);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "null undefined 8000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Xoá LÀ ghi một bia mộ — `null` khác hẳn `undefined`. Nhưng bia mộ
cứ tích LŨY mãi thì sao — có cách nào dọn SẠCH chúng không?
::::

::::reflect{#nghi-lai}
`xoaMemtable` không hề "xoá" theo nghĩa THÔNG thường — nó ghi một
bản GHI mới với `giaTri=null`, giống HỆT cách cập nhật một giá trị
(ghi bản MỚI, không sửa bản cũ). `timMemtable` phân biệt RÕ ba
trạng thái: giá trị thật, `null` (đã xoá), `undefined` (chưa từng
thấy) — phân biệt NÀY (đã dạy Ở content/lap-trinh-ham) chính LÀ thứ
làm tombstone hoạt động đúng. Nhưng bia mộ tích luỹ MÃI trên nhiều
SSTable sẽ chiếm chỗ vô ích — dọn sạch chúng diễn ra KHI nào?
::::

::::checkpoint{mastery=0.8}
::::
