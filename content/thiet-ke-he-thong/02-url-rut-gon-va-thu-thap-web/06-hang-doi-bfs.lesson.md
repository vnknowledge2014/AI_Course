---
id: thiet-ke-he-thong.url-rut-gon-va-thu-thap-web.hang-doi-bfs
title: "Hàng đợi BFS: khám phá web theo từng tầng"
summary: "HangDoiBFS{danhSach} dùng push (them vao CUOI) + shift (lay ra DAU) -- FIFO thuần tuý. chayBFS bắt đầu từ 1 URL gốc, lấy ra từng URL, 'khám phá' con của nó (hàm khamPha tra một đồ thị cây cố định), đẩy TẤT CẢ con vào cuối hàng đợi trước khi lấy phần tử kế. Kết quả: thứ tự khám phá đi theo TỪNG TẦNG (trang-goc,trang-a,trang-b,trang-c,trang-d,trang-e) — khác hẳn DFS (trang-goc,trang-a,trang-c,trang-d,trang-b,trang-e) sẽ đi sâu vào nhánh trang-a trước khi quay lại trang-b."
locale: vi
track: thiet-ke-he-thong
module: url-rut-gon-va-thu-thap-web
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [sd.hang-doi-bfs]
requires: [sd.het-han-va-thu-hoi]
concepts: [sd.hang-doi-bfs]
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
Rút gọn URL (năm bài trước) đã xong. Nửa còn LẠI của quest LÀ một
crawler — một chương trình tự động ĐI theo các liên kết để khám phá
trang WEB. Mảnh đầu tiên: một hàng đợi quyết định THỨ tự khám phá.
::::

::::explain{#hang-doi-fifo}
Web LÀ một đồ thị: mỗi trang LÀ một đỉnh, mỗi liên kết LÀ một cạnh.
Cách khám phá "theo TỪNG tầng" (tầng gần gốc trước, tầng XA sau) gọi
LÀ BFS (breadth-first search) — VÀ nó dùng đúng MỘT cấu trúc dữ liệu
đơn giản: hàng đợi FIFO (vào TRƯỚC ra trước). `themVaoHangDoi` thêm
VÀO cuối, `layTuHangDoi` lấy RA từ đầu:

```typescript title=readonly
interface HangDoiBFS { danhSach: string[]; }
function taoHangDoiBFS(): HangDoiBFS { return { danhSach: [] }; }
function themVaoHangDoi(hd: HangDoiBFS, url: string): void { hd.danhSach.push(url); }
function layTuHangDoi(hd: HangDoiBFS): string | undefined { return hd.danhSach.shift(); }

const hd = taoHangDoiBFS();
themVaoHangDoi(hd, "trang-goc");
themVaoHangDoi(hd, "trang-a");
themVaoHangDoi(hd, "trang-b");

console.log("lay ra 1:", layTuHangDoi(hd));
console.log("lay ra 2:", layTuHangDoi(hd));
themVaoHangDoi(hd, "trang-c");
console.log("lay ra 3:", layTuHangDoi(hd));
console.log("lay ra 4:", layTuHangDoi(hd));
console.log("hang doi rong (undefined):", layTuHangDoi(hd));
```

```text title=readonly
lay ra 1: trang-goc
lay ra 2: trang-a
lay ra 3: trang-b
lay ra 4: trang-c
hang doi rong (undefined): undefined
```

Ba URL thêm VÀO theo thứ tự `goc, a, b` — VÀ lấy RA đúng theo thứ tự
ĐÓ, dù `"trang-c"` được thêm VÀO GIỮA chừng (sau khi đã lấy RA hai
phần tử đầu). `layTuHangDoi` trên hàng đợi RỖNG trả về `undefined`,
không hề NÉM lỗi.
::::

::::example{#chay-bfs-theo-tang}
Ghép hàng đợi VỚI một hàm "khám phá" (trả VỀ danh sách trang con của
một trang) tạo THÀNH một vòng lặp BFS hoàn chỉnh: lấy MỘT trang ra,
ghi NHẬN nó, đẩy TẤT cả con của nó vào CUỐI hàng đợi, lặp LẠI:

```typescript title=readonly
interface HangDoiBFS { danhSach: string[]; }
function taoHangDoiBFS(): HangDoiBFS { return { danhSach: [] }; }
function themVaoHangDoi(hd: HangDoiBFS, url: string): void { hd.danhSach.push(url); }
function layTuHangDoi(hd: HangDoiBFS): string | undefined { return hd.danhSach.shift(); }

function khamPha(url: string): string[] {
  const DO_THI: Record<string, string[]> = {
    "trang-goc": ["trang-a", "trang-b"],
    "trang-a": ["trang-c", "trang-d"],
    "trang-b": ["trang-e"],
    "trang-c": [],
    "trang-d": [],
    "trang-e": [],
  };
  return DO_THI[url] ?? [];
}

function chayBFS(urlGoc: string): string[] {
  const hd = taoHangDoiBFS();
  themVaoHangDoi(hd, urlGoc);
  const thuTuKhamPha: string[] = [];
  let hienTai = layTuHangDoi(hd);
  while (hienTai !== undefined) {
    thuTuKhamPha.push(hienTai);
    for (const con of khamPha(hienTai)) themVaoHangDoi(hd, con);
    hienTai = layTuHangDoi(hd);
  }
  return thuTuKhamPha;
}

console.log("thu tu BFS:", chayBFS("trang-goc").join(" -> "));
console.log("(so sanh: DFS se la trang-goc -> trang-a -> trang-c -> trang-d -> trang-b -> trang-e)");
```

```text title=readonly
thu tu BFS: trang-goc -> trang-a -> trang-b -> trang-c -> trang-d -> trang-e
(so sanh: DFS se la trang-goc -> trang-a -> trang-c -> trang-d -> trang-b -> trang-e)
```

`trang-goc` có HAI con: `trang-a` VÀ `trang-b` — CẢ hai được khám
PHÁ NGAY sau `trang-goc`, TRƯỚC khi bất kỳ "cháu" nào (con của
`trang-a`, con của `trang-b`) được chạm TỚI. Đây chính LÀ đặc trưng
của BFS: đi HẾT một tầng rồi mới sang TẦNG kế — khác hẳn DFS (đi sâu
theo MỘT nhánh trước, như hàng đợi ĐỔI thành ngăn xếp).
::::

::::predict{#doan-thu-tu-fifo commitOnce}
Một hàng đợi RỖNG. Thêm `"x"` rồi `"y"` (theo đúng thứ tự đó), lấy RA
một phần tử, RỒI thêm `"z"`. Hai lần lấy TIẾP theo cho ra thứ tự nào?

:::opt{correct}
`"y"` rồi `"z"` — hàng đợi vẫn LÀ FIFO: `"y"` vào TRƯỚC `"z"` nên ra
TRƯỚC, bất kể `"x"` đã bị lấy ra từ LÚC nào
:::
:::opt
`"z"` rồi `"y"` — phần TỬ mới thêm vào (`"z"`) luôn được ưu TIÊN lấy
ra trước những phần tử đang CHỜ sẵn
::why
Nhầm hàng đợi FIFO (`push` + `shift`) VỚI một ngăn xếp LIFO (`push` +
`pop`) — nhưng `layTuHangDoi` dùng `shift()`, LUÔN lấy từ ĐẦU mảng,
không phải từ CUỐI.

Chỗ lệch: sau khi thêm `"x"`, `"y"` VÀ lấy ra một phần tử (`"x"`,
vì `shift()` lấy đầu TIÊN), `danhSach` còn lại `["y"]`. Thêm
`"z"` vào CUỐI bằng `push()` cho ra `["y", "z"]`. Hai lần `shift()`
tiếp theo LUÔN lấy phần tử ĐẦU mảng trước: `"y"` rồi `"z"` — thứ tự
THÊM vào quyết định thứ tự lấy RA, không phải mới hay CŨ.
::
:::
::::

::::code{#viet_chay_bfs}
Hoàn thiện `chayBFS` — sau khi ghi nhận trang HIỆN tại vào kết quả
(dòng TRÊN), khám phá các trang CON của nó VÀ đẩy TỪNG trang con vào
cuối hàng đợi.

```typescript title=starter
interface HangDoiBFS { danhSach: string[]; }
function taoHangDoiBFS(): HangDoiBFS { return { danhSach: [] }; }
function themVaoHangDoi(hd: HangDoiBFS, url: string): void { hd.danhSach.push(url); }
function layTuHangDoi(hd: HangDoiBFS): string | undefined { return hd.danhSach.shift(); }

function khamPha(url: string): string[] {
  const DO_THI: Record<string, string[]> = {
    "trang-goc": ["trang-a", "trang-b"],
    "trang-a": ["trang-c", "trang-d"],
    "trang-b": ["trang-e"],
    "trang-c": [],
    "trang-d": [],
    "trang-e": [],
  };
  return DO_THI[url] ?? [];
}

function chayBFS(urlGoc: string): string[] {
  const hd = taoHangDoiBFS();
  themVaoHangDoi(hd, urlGoc);
  const thuTuKhamPha: string[] = [];
  let hienTai = layTuHangDoi(hd);
  while (hienTai !== undefined) {
    thuTuKhamPha.push(hienTai);
    ___
    hienTai = layTuHangDoi(hd);
  }
  return thuTuKhamPha;
}

console.log(chayBFS("trang-goc").join(","));
```

```typescript title=solution
interface HangDoiBFS { danhSach: string[]; }
function taoHangDoiBFS(): HangDoiBFS { return { danhSach: [] }; }
function themVaoHangDoi(hd: HangDoiBFS, url: string): void { hd.danhSach.push(url); }
function layTuHangDoi(hd: HangDoiBFS): string | undefined { return hd.danhSach.shift(); }

function khamPha(url: string): string[] {
  const DO_THI: Record<string, string[]> = {
    "trang-goc": ["trang-a", "trang-b"],
    "trang-a": ["trang-c", "trang-d"],
    "trang-b": ["trang-e"],
    "trang-c": [],
    "trang-d": [],
    "trang-e": [],
  };
  return DO_THI[url] ?? [];
}

function chayBFS(urlGoc: string): string[] {
  const hd = taoHangDoiBFS();
  themVaoHangDoi(hd, urlGoc);
  const thuTuKhamPha: string[] = [];
  let hienTai = layTuHangDoi(hd);
  while (hienTai !== undefined) {
    thuTuKhamPha.push(hienTai);
    for (const con of khamPha(hienTai)) themVaoHangDoi(hd, con);
    hienTai = layTuHangDoi(hd);
  }
  return thuTuKhamPha;
}

console.log(chayBFS("trang-goc").join(","));
```

```typescript title=test
const ketQua = chayBFS("trang-goc");
if (ketQua.join(",") !== "trang-goc,trang-a,trang-b,trang-c,trang-d,trang-e") throw new Error("thu tu BFS phai di theo TUNG TANG (level-order), khong phai theo chieu sau (DFS)");
if (ketQua.length !== 6) throw new Error("phai kham pha dung 6 trang trong cay nay");
if (ketQua[0] !== "trang-goc") throw new Error("trang dau tien duoc kham pha phai la trang-goc");
if (ketQua[1] !== "trang-a" || ketQua[2] !== "trang-b") throw new Error("hai trang o tang 1 (trang-a, trang-b) phai duoc kham pha NGAY sau trang-goc, TRUOC ca tang 2");

const ketQuaTuA = chayBFS("trang-a");
if (ketQuaTuA.join(",") !== "trang-a,trang-c,trang-d") throw new Error("bat dau tu trang-a chi kham pha nhanh con cua no");
```

:::hints
- kind: attention
  body: "Ngay sau khi ghi nhan hienTai, phai duyet TAT CA cac trang con cua no (tu khamPha) VA them tung trang con vao cuoi hang doi."
- kind: strategy
  body: "for (const con of khamPha(hienTai)) themVaoHangDoi(hd, con);"
- kind: one-line
  body: "for (const con of khamPha(hienTai)) themVaoHangDoi(hd, con);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 5000
- tier: output
  match: contains
  expect: "trang-goc,trang-a,trang-b"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Thứ tự khám phá đã đúng. Nhưng web THẬT có nhiều URL "khác chuỗi
nhưng cùng đích" — crawler cần biết TRANG nào đã thăm rồi để không đi
VÒNG mãi.
::::

::::reflect{#nghi-lai}
`chayBFS` không CÓ gì phức tạp — nó chỉ LÀ một vòng lặp `while` VỚI
đúng hai thao TÁC hàng đợi. Cái LÀM nên BFS (thay vì DFS) hoàn TOÀN
nằm ở việc CHỌN đúng cấu trúc dữ liệu: `push` + `shift` (FIFO) CHO
tầng, `push` + `pop` (LIFO) sẽ cho CHIỀU sâu.
::::

::::checkpoint{mastery=0.75}
::::
