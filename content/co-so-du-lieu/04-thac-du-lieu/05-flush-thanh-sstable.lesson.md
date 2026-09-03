---
id: co-so-du-lieu.thac-du-lieu.flush-thanh-sstable
title: Flush thành SSTable
summary: "flush sao chép memtable thành một SSTable — một bản ghi BẤT BIẾN (immutable). Sửa memtable SAU khi flush không ảnh hưởng SSTable đã tạo — chúng là hai đối tượng độc lập, không chia sẻ mảng bên trong."
locale: vi
track: co-so-du-lieu
module: thac-du-lieu
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.flush-to-sstable]
requires: [db.memtable-binary-search]
concepts: [db.flush-to-sstable]
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
Memtable đầy — dữ liệu phải LÊN đĩa. Kết quả trên đĩa gọi LÀ gì, và
nó có còn ĐỔI được sau khi ghi không?
::::

::::explain{#flush-sang-sstable}
`flush` sao chép memtable thành MỘT SSTable ("Sorted String Table")
— một bản ghi BẤT BIẾN, KHÔNG bao giờ sửa tại chỗ sau khi tạo:

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

interface SSTable {
  khoa: string[];
  giaTri: (string | null)[];
}

function flush(bang: Memtable): SSTable {
  return { khoa: [...bang.khoa], giaTri: [...bang.giaTri] };
}

const m = taoMemtable();
chenMemtable(m, "cam", "5000");
chenMemtable(m, "buoi", "8000");
const ss = flush(m);
console.log(ss.khoa.join(","), ss.giaTri.join(","));
```

```text title=readonly
buoi,cam 8000,5000
```

`flush` KHÔNG trả về CHÍNH `bang.khoa`/`bang.giaTri` — nó tạo mảng
MỚI bằng spread (`[...bang.khoa]`), một BẢN sao độc lập hoàn TOÀN
với memtable gốc.
::::

::::example{#doc-lap-sau-flush}
Sau khi flush, memtable gốc VẪN tiếp tục nhận ghi — SSTable đã tạo
KHÔNG hề đổi theo:

```typescript title=readonly
chenMemtable(m, "tao", "12000");
console.log("memtable:", m.khoa.join(","), m.giaTri.join(","));
console.log("SSTable:", ss.khoa.join(","), ss.giaTri.join(","));
```

```text title=readonly
memtable: buoi,cam,tao 8000,5000,12000
SSTable: buoi,cam 8000,5000
```

Memtable giờ CÓ ba khoá — nhưng `ss` (SSTable đã flush TRƯỚC đó)
vẫn CHỈ hai khoá, đúng như lúc nó ĐƯỢC tạo. Hai đối tượng hoàn TOÀN
tách biệt — không chia sẻ mảng BÊN trong.
::::

::::predict{#doan-sstable-sau-nhieu-lan-chen commitOnce}
Byte tạo memtable RỖNG, chèn MỘT khoá, flush, RỒI chèn thêm HAI
khoá nữa vào memtable gốc:

```typescript
const m2 = taoMemtable();
chenMemtable(m2, "le", "4000");
const ss2 = flush(m2);
chenMemtable(m2, "nho", "6000");
chenMemtable(m2, "oi", "2000");
console.log(ss2.khoa.length);
```

Dòng cuối in ra gì?

:::opt{correct}
`1`
:::

:::opt
`3` — vì `ss2` VÀ `m2` cùng trỏ TỚI một vùng nhớ chung, nên MỌI
thay đổi trên `m2` SAU đó cũng hiện RA trên `ss2`
::why
Gần đúng ở việc bạn nghĩ TỚI khả năng "tham chiếu chung" — MỘT lo
lắng hợp lý khi làm việc VỚI object/array trong JavaScript, nơi
gán trực TIẾP (`ss2.khoa = m2.khoa`) THẬT sự tạo tham chiếu chung.

Chỗ lệch: `flush` KHÔNG gán trực tiếp — nó dùng spread
(`[...bang.khoa]`) để tạo MẢNG MỚI, một bản sao độc LẬP hoàn toàn.
`ss2.khoa` và `m2.khoa` LÀ hai mảng khác nhau TRONG bộ nhớ ngay từ
lúc `flush` chạy xong, dù nội dung BAN đầu giống hệt nhau.
::
:::

:::opt
Máy báo lỗi — vì `flush` một memtable rồi TIẾP tục chèn thêm vào
memtable đó LÀ thao tác không hợp LỆ
::why
Gần đúng ở việc bạn nghĩ TỚI một ràng buộc "khoá lại sau flush" hợp
lý cho một hệ thống LƯU trữ thật.

Chỗ lệch: `flush` chỉ đơn thuần LÀ một hàm sao chép — nó KHÔNG hề
`throw`, VÀ KHÔNG có cơ chế "khoá" nào ngăn `chenMemtable` chạy
tiếp trên `m2`. Trong LSM thật, memtable ĐÃ flush thường được thay
bằng một memtable RỖNG mới (khoá logic Ở tầng ĐIỀU phối, không phải
bên trong `chenMemtable`/`flush`).
::
:::
::::

::::code{#viet_flush}
Hoàn thiện `flush(bang: Memtable): SSTable` — trả về một BẢN sao
độc lập, không chia sẻ mảng VỚI memtable gốc.

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

interface SSTable {
  khoa: string[];
  giaTri: (string | null)[];
}

function flush(bang: Memtable): SSTable {
  return ___;
}

const m = taoMemtable();
chenMemtable(m, "cam", "5000");
chenMemtable(m, "buoi", "8000");
const ss = flush(m);
chenMemtable(m, "tao", "12000");
console.log(ss.khoa.join(","), ss.giaTri.join(","));
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

interface SSTable {
  khoa: string[];
  giaTri: (string | null)[];
}

function flush(bang: Memtable): SSTable {
  return { khoa: [...bang.khoa], giaTri: [...bang.giaTri] };
}

const m = taoMemtable();
chenMemtable(m, "cam", "5000");
chenMemtable(m, "buoi", "8000");
const ss = flush(m);
chenMemtable(m, "tao", "12000");
console.log(ss.khoa.join(","), ss.giaTri.join(","));
```

```typescript title=test
const m2 = taoMemtable();
chenMemtable(m2, "le", "4000");
const ss2 = flush(m2);
if (ss2.khoa.length !== 1) throw new Error("SSTable phai co dung 1 khoa ngay sau flush");
chenMemtable(m2, "nho", "6000");
chenMemtable(m2, "oi", "2000");
if (ss2.khoa.length !== 1) throw new Error("SSTable KHONG duoc doi sau khi memtable goc chen them");
if (m2.khoa.length !== 3) throw new Error("memtable goc phai co 3 khoa sau hai lan chen them");
if (ss2.khoa[0] !== "le" || ss2.giaTri[0] !== "4000") throw new Error("SSTable phai giu dung du lieu tai thoi diem flush");
```

:::hints
- kind: attention
  body: "Tra ve mot object moi voi hai truong khoa/giaTri, MOI truong la mot mang MOI sao chep tu bang -- dung spread [...]."
- kind: strategy
  body: "return { khoa: [...bang.khoa], giaTri: [...bang.giaTri] };"
- kind: one-line
  body: "{ khoa: [...bang.khoa], giaTri: [...bang.giaTri] }"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "buoi,cam 8000,5000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
SSTable bất biến — flush xong LÀ khoá cứng. Nhưng mỗi lần memtable
đầy lại tạo THÊM một SSTable mới — khi CÓ nhiều cái, đọc một khoá
đi TÌM ở đâu trước?
::::

::::reflect{#nghi-lai}
`flush` biến một memtable (có THỂ đổi) thành một SSTable (KHÔNG bao
giờ đổi) bằng cách sao chép — hai đối tượng ĐỘC lập hoàn toàn ngay
từ lúc tạo. Mỗi lần memtable đầy lại sinh RA một SSTable mới — sau
một thời gian, HỆ thống có nhiều SSTable nằm CẠNH nhau trên đĩa.
Đọc một khoá lúc đó phải tìm Ở đâu TRƯỚC?
::::

::::checkpoint{mastery=0.8}
::::
