---
id: co-so-du-lieu.thac-du-lieu.memtable-luon-giu-thu-tu
title: Memtable — luôn giữ thứ tự khi chèn
summary: "chenMemtable dò vị trí bằng tìm nhị phân (dữ liệu đã sắp xếp), rồi hoặc GHI ĐÈ (khoá đã có) hoặc CHÈN mới đúng vị trí (splice) — không bao giờ tạo bản trùng. Kết quả luôn ở dạng đã sắp xếp, sẵn sàng flush thành SSTable mà không cần sắp xếp lại."
locale: vi
track: co-so-du-lieu
module: thac-du-lieu
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [db.memtable-sorted-insert]
requires: [db.why-write-elsewhere-first]
concepts: [db.memtable-sorted-insert]
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
Ghi trước vào bộ nhớ (bài TRƯỚC) — nhưng nếu cứ ghi NỐI vào cuối, dữ
liệu SẼ lộn xộn. Khi flush, phải sắp xếp LẠI hết — có cách nào tránh
việc đó không?
::::

::::explain{#chen-memtable}
`chenMemtable` giữ `khoa` LUÔN đã sắp xếp — mỗi lần chèn, DÒ vị trí
đúng bằng tìm NHỊ phân (nhị phân trên mảng đã sắp xếp), rồi CHÈN
đúng chỗ đó, KHÔNG ghi nối vào cuối:

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
  if (bang.khoa[lo] === khoa) {
    bang.giaTri[lo] = giaTri;
  } else {
    bang.khoa.splice(lo, 0, khoa);
    bang.giaTri.splice(lo, 0, giaTri);
  }
}

const m = taoMemtable();
chenMemtable(m, "cam", "5000");
chenMemtable(m, "buoi", "8000");
chenMemtable(m, "tao", "12000");
console.log(m.khoa.join(","), m.giaTri.join(","));
```

```text title=readonly
buoi,cam,tao 8000,5000,12000
```

Ba lần chèn, THEO thứ tự `cam`, `buoi`, `tao` — kết quả VẪN đúng
`buoi, cam, tao`, đã sắp xếp theo BẢNG chữ cái, dù thứ tự chèn hoàn
toàn KHÁC. Vòng `while` dò đúng VỊ trí trước, `splice` chèn ĐÚNG chỗ
đó — không phải chèn CUỐI rồi sắp lại.
::::

::::example{#ghi-de-khong-trung}
Chèn một khoá ĐÃ tồn tại — ghi ĐÈ giá trị cũ, KHÔNG tạo bản sao:

```typescript title=readonly
chenMemtable(m, "buoi", "9000");
console.log(m.khoa.join(","), m.giaTri.join(","));
```

```text title=readonly
buoi,cam,tao 9000,5000,12000
```

Vẫn CHỈ ba khoá — `buoi` giờ mang giá trị `9000` (mới NHẤT), `cam`
VÀ `tao` không đổi. Vòng `while` dò TỚI đúng vị trí `buoi` đang ĐỨNG
(`bang.khoa[lo] === khoa` khớp NGAY), nên nhánh GHI đè chạy, không
phải nhánh chèn MỚI.
::::

::::predict{#doan-do-dai-sau-ghi-de commitOnce}
Memtable đang có ba khoá (`buoi, cam, tao`). Byte chèn `cam` với
giá trị MỚI:

```typescript
chenMemtable(m, "cam", "6000");
console.log(m.khoa.length);
```

Dòng cuối in ra gì?

:::opt{correct}
`3`
:::

:::opt
`4` — vì MỖI lần gọi `chenMemtable` thêm MỘT phần tử mới vào mảng,
bất kể khoá đã CÓ hay chưa
::why
Gần đúng ở việc bạn nghĩ TỚI hành vi "chèn" điển hình — thêm MỘT
phần tử LÀ tăng độ dài, một trực GIÁC đúng với `push`/`splice`
thông thường.

Chỗ lệch: `chenMemtable` KIỂM tra `bang.khoa[lo] === khoa` TRƯỚC
khi quyết định — `cam` ĐÃ có sẵn Ở đúng vị trí `lo` DÒ được, nên
nhánh chạy LÀ `bang.giaTri[lo] = giaTri` (ghi ĐÈ tại chỗ), KHÔNG
phải `splice` (chèn thêm phần TỬ mới). Độ dài `khoa` không đổi.
::
:::

:::opt
Máy báo lỗi — vì chèn một khoá TRÙNG với khoá đã tồn tại LÀ thao
tác không hợp LỆ trên memtable
::why
Gần đúng ở việc bạn nghĩ TỚI một ràng buộc "khoá duy NHẤT" hợp lý
cho một cấu trúc lưu TRỮ có thứ tự.

Chỗ lệch: `chenMemtable` KHÔNG hề `throw` — ghi ĐÈ một khoá đã tồn
tại LÀ hành vi HOÀN toàn hợp lệ VÀ cố ý (đó chính LÀ cách CẬP nhật
một giá trị TRONG hệ thống LSM: ghi một bản GHI mới, không sửa bản
CŨ tại chỗ).
::
:::
::::

::::code{#viet_chen_memtable}
Hoàn thiện `chenMemtable` — sau khi đã DÒ đúng `lo`, ghi đè NẾU khoá
đã có, chèn mới NẾU chưa.

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
  ___
}

const m = taoMemtable();
chenMemtable(m, "cam", "5000");
chenMemtable(m, "buoi", "8000");
chenMemtable(m, "tao", "12000");
chenMemtable(m, "buoi", "9000");
console.log(m.khoa.join(","), m.giaTri.join(","));
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

const m = taoMemtable();
chenMemtable(m, "cam", "5000");
chenMemtable(m, "buoi", "8000");
chenMemtable(m, "tao", "12000");
chenMemtable(m, "buoi", "9000");
console.log(m.khoa.join(","), m.giaTri.join(","));
```

```typescript title=test
const m2 = taoMemtable();
chenMemtable(m2, "cam", "5000");
chenMemtable(m2, "buoi", "8000");
chenMemtable(m2, "tao", "12000");
if (m2.khoa.join(",") !== "buoi,cam,tao") throw new Error("khoa phai luon o dang da sap xep sau moi lan chen");
chenMemtable(m2, "an", "1000");
if (m2.khoa.join(",") !== "an,buoi,cam,tao") throw new Error("chen vao dau -- an nho hon ca buoi");
chenMemtable(m2, "cam", "6000");
if (m2.khoa.length !== 4) throw new Error("ghi de khoa da co khong duoc tang do dai");
if (m2.giaTri.join(",") !== "1000,8000,6000,12000") throw new Error("cam phai mang gia tri MOI (6000), khong phai gia tri cu (5000)");
```

:::hints
- kind: attention
  body: "Sau vong while, lo la vi tri DUNG. Neu bang.khoa[lo] === khoa thi ghi de bang.giaTri[lo]; nguoc lai splice chen moi ca khoa lan giaTri tai lo."
- kind: strategy
  body: "if (bang.khoa[lo] === khoa) { bang.giaTri[lo] = giaTri; } else { bang.khoa.splice(lo, 0, khoa); bang.giaTri.splice(lo, 0, giaTri); }"
- kind: one-line
  body: "if (bang.khoa[lo] === khoa) { bang.giaTri[lo] = giaTri; } else { bang.khoa.splice(lo, 0, khoa); bang.giaTri.splice(lo, 0, giaTri); }"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "buoi,cam,tao 9000,5000,12000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Memtable LUÔN giữ thứ tự — flush KHÔNG cần sắp xếp lại. Nhưng dò vị
trí bằng tìm nhị phân TRÊN một mảng LÀ một cách — cấu trúc THẬT mà
LSM dùng CHO việc này tên LÀ gì?
::::

::::reflect{#nghi-lai}
`chenMemtable` giữ mảng LUÔN đã sắp xếp bằng cách dò đúng VỊ trí mỗi
lần chèn (tìm nhị phân), rồi ghi đè HOẶC chèn mới — không bao giờ có
bản TRÙNG. Splice trên một mảng JavaScript CÓ chi phí (dịch chuyển
phần tử) — hệ thống THẬT như LevelDB/RocksDB không dùng mảng trần,
mà dùng một cấu trúc TÊN là skip list. Nó trông NHƯ thế nào?
::::

::::checkpoint{mastery=0.8}
::::
