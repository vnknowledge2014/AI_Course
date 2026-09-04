---
id: thiet-ke-he-thong.ha-tang-du-lieu-quy-mo-lon.luu-tru-theo-khoi
title: "Lưu trữ theo khối: thứ tự tới không quan trọng"
summary: "nhanKhoi(tt, chiSoKhoi) chi ghi nhan mot chi so khoi vao mot Set -- KHONG can khoi toi theo dung THU TU. File 4 khoi nhan theo thu tu [2, 0, 3] (khoi 3 toi TRUOC khoi 1) van duoc theo doi dung, laHoanTat tra ve false vi con thieu khoi 1; nhan not khoi 1 thi laHoanTat=true. Gui LAI mot khoi da co (trung lap) khong lam tang so khoi da nhan."
locale: vi
track: thiet-ke-he-thong
module: ha-tang-du-lieu-quy-mo-lon
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [sd.luu-tru-theo-khoi]
requires: [sd.metrics-canh-bao-co-do-tre]
concepts: [sd.luu-tru-theo-khoi]
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
Hai mảnh về hàng đợi, hai mảnh về giám sát — mảnh HẠ tầng thứ ba
chuyển hẳn sang FILE. Một file lớn (video, ảnh, tài liệu) không được
tải lên nguyên khối — nó bị chia thành nhiều khối NHỎ trước. Câu hỏi
đầu tiên: hệ thống biết khối nào ĐÃ tới bằng cách nào?
::::

::::explain{#theo-doi-khoi-da-nhan}
`nhanKhoi` chỉ ghi nhận CHỈ số của khối vừa tới vào một `Set` — KHÔNG
đòi hỏi khối phải tới theo đúng thứ TỰ. `laHoanTat` so sánh số khối đã
nhận VỚI tổng số khối cần có; file chỉ HOÀN tất khi đủ TẤT cả:

```typescript title=readonly
interface TrangThaiUpload { idFile: string; tongSoKhoi: number; khoiDaNhan: Set<number>; }
function taoTrangThaiUpload(idFile: string, tongSoKhoi: number): TrangThaiUpload {
  return { idFile, tongSoKhoi, khoiDaNhan: new Set() };
}
function nhanKhoi(tt: TrangThaiUpload, chiSoKhoi: number): void {
  tt.khoiDaNhan.add(chiSoKhoi);
}
function laHoanTat(tt: TrangThaiUpload): boolean {
  return tt.khoiDaNhan.size === tt.tongSoKhoi;
}
function danhSachKhoiConThieu(tt: TrangThaiUpload): number[] {
  const thieu: number[] = [];
  for (let i = 0; i < tt.tongSoKhoi; i++) if (!tt.khoiDaNhan.has(i)) thieu.push(i);
  return thieu;
}

const tt = taoTrangThaiUpload("video-1", 4);
nhanKhoi(tt, 2);
console.log("nhan khoi 2 truoc tien -- hoan tat?", laHoanTat(tt));
nhanKhoi(tt, 0);
console.log("nhan them khoi 0 -- hoan tat?", laHoanTat(tt));
nhanKhoi(tt, 3);
console.log("nhan them khoi 3 -- hoan tat?", laHoanTat(tt));
console.log("khoi da nhan:", Array.from(tt.khoiDaNhan).sort());
console.log("khoi con thieu:", danhSachKhoiConThieu(tt));
```

```text title=readonly
nhan khoi 2 truoc tien -- hoan tat? false
nhan them khoi 0 -- hoan tat? false
nhan them khoi 3 -- hoan tat? false
khoi da nhan: [ 0, 2, 3 ]
khoi con thieu: [ 1 ]
```

Khối `2` tới ĐẦU tiên — TRƯỚC cả khối `0` LẪN khối `1` — nhưng
`nhanKhoi` không hề quan TÂm tới thứ tự, nó chỉ đơn giản THÊM chỉ số
đó vào `Set`. Sau ba khối (`2`, `0`, `3`), `laHoanTat` vẫn LÀ `false`
vì khối `1` chưa từng xuất hiện — `danhSachKhoiConThieu` chỉ RA đúng
khối đó.
::::

::::example{#trung-lap-khong-tang-dem}
`Set` tự loại BỎ phần tử trùng — gửi lại một khối ĐÃ có (ví dụ do mạng
gửi lặp) không hề làm tăng số khối đã nhận:

```typescript title=readonly
interface TrangThaiUpload { idFile: string; tongSoKhoi: number; khoiDaNhan: Set<number>; }
function taoTrangThaiUpload(idFile: string, tongSoKhoi: number): TrangThaiUpload {
  return { idFile, tongSoKhoi, khoiDaNhan: new Set() };
}
function nhanKhoi(tt: TrangThaiUpload, chiSoKhoi: number): void {
  tt.khoiDaNhan.add(chiSoKhoi);
}
function laHoanTat(tt: TrangThaiUpload): boolean {
  return tt.khoiDaNhan.size === tt.tongSoKhoi;
}
function danhSachKhoiConThieu(tt: TrangThaiUpload): number[] {
  const thieu: number[] = [];
  for (let i = 0; i < tt.tongSoKhoi; i++) if (!tt.khoiDaNhan.has(i)) thieu.push(i);
  return thieu;
}

// tai lap dung trang thai tu khoi truoc, KHONG in lai: da nhan khoi 2, 0, 3
// (thieu khoi 1) cua file "video-1" (tong 4 khoi)
const tt = taoTrangThaiUpload("video-1", 4);
nhanKhoi(tt, 2);
nhanKhoi(tt, 0);
nhanKhoi(tt, 3);

nhanKhoi(tt, 2); // gui lai khoi 2 (trung lap, vi du do mang lag)
console.log("nhan LAI khoi 2 (trung lap) -- so khoi da nhan:", tt.khoiDaNhan.size);

nhanKhoi(tt, 1);
console.log("nhan not khoi 1 -- hoan tat?", laHoanTat(tt));
console.log("khoi con thieu:", danhSachKhoiConThieu(tt));
```

```text title=readonly
nhan LAI khoi 2 (trung lap) -- so khoi da nhan: 3
nhan not khoi 1 -- hoan tat? true
khoi con thieu: []
```

Gửi lại khối `2` không hề đẩy `khoiDaNhan.size` lên `4` — nó vẫn dừng
Ở `3`, vì `Set.add` bỏ QUA phần tử đã tồn tại. Chỉ khi khối `1` THẬT sự
xuất hiện, `laHoanTat` mới chuyển sang `true` VÀ danh sách khối thiếu
mới trở về rỗng.
::::

::::predict{#doan-trung-lap-khong-tang-kich-thuoc commitOnce}
Một upload có `tongSoKhoi = 5`. Đã nhận các khối `[4, 1, 3]` (đúng `3`
khối, thứ tự tới không theo thứ tự). Khối `1` được gửi LẠI một lần
nữa (trùng lặp). Ngay SAU lần gửi lại đó, `khoiDaNhan.size` LÀ bao
nhiêu?

:::opt{correct}
Vẫn `3` — `nhanKhoi` gọi `Set.add`, VÀ thêm một phần tử ĐÃ có sẵn vào
`Set` không hề làm tăng kích thước của nó; khối `1` chỉ được ĐẾM một
lần dù được gửi bao nhiêu lần
:::
:::opt
`4` — mỗi lần GỌI `nhanKhoi` LÀ một lần "nhận thêm", nên gửi lại khối
`1` phải cộng dồn thành một khối MỚI trong bộ đếm
::why
Nhầm "số lần GỌI hàm" VỚI "số phần tử trong Set" — nhưng `nhanKhoi`
không hề đếm số LẦN gọi, nó chỉ thêm `chiSoKhoi` vào một tập hợp.

Chỗ lệch: `tt.khoiDaNhan.add(chiSoKhoi)` — `Set` LÀ một cấu trúc dữ
liệu chỉ giữ giá trị DUY nhất. Gọi `add(1)` lần thứ hai với `1` đã có
sẵn trong `Set` không hề tạo thêm phần tử NÀO — `khoiDaNhan.size` vẫn
dừng Ở `3` (`4`, `1`, `3`), không tăng lên `4`.
::
:::
::::

::::code{#viet_danh_sach_khoi_con_thieu}
Hoàn thiện `danhSachKhoiConThieu` — mảng kết quả `thieu` đã khai báo.
Còn thiếu: duyệt qua TỪNG chỉ số khối từ `0` tới `tongSoKhoi - 1`,
thêm vào `thieu` những chỉ số CHƯA có trong `khoiDaNhan`.

```typescript title=starter
interface TrangThaiUpload { idFile: string; tongSoKhoi: number; khoiDaNhan: Set<number>; }
function taoTrangThaiUpload(idFile: string, tongSoKhoi: number): TrangThaiUpload {
  return { idFile, tongSoKhoi, khoiDaNhan: new Set() };
}
function nhanKhoi(tt: TrangThaiUpload, chiSoKhoi: number): void {
  tt.khoiDaNhan.add(chiSoKhoi);
}
function laHoanTat(tt: TrangThaiUpload): boolean {
  return tt.khoiDaNhan.size === tt.tongSoKhoi;
}
function danhSachKhoiConThieu(tt: TrangThaiUpload): number[] {
  const thieu: number[] = [];
  ___
  return thieu;
}

const ttX = taoTrangThaiUpload("f1", 3);
nhanKhoi(ttX, 2);
console.log(JSON.stringify(danhSachKhoiConThieu(ttX)));
```

```typescript title=solution
interface TrangThaiUpload { idFile: string; tongSoKhoi: number; khoiDaNhan: Set<number>; }
function taoTrangThaiUpload(idFile: string, tongSoKhoi: number): TrangThaiUpload {
  return { idFile, tongSoKhoi, khoiDaNhan: new Set() };
}
function nhanKhoi(tt: TrangThaiUpload, chiSoKhoi: number): void {
  tt.khoiDaNhan.add(chiSoKhoi);
}
function laHoanTat(tt: TrangThaiUpload): boolean {
  return tt.khoiDaNhan.size === tt.tongSoKhoi;
}
function danhSachKhoiConThieu(tt: TrangThaiUpload): number[] {
  const thieu: number[] = [];
  for (let i = 0; i < tt.tongSoKhoi; i++) if (!tt.khoiDaNhan.has(i)) thieu.push(i);
  return thieu;
}

const ttX = taoTrangThaiUpload("f1", 3);
nhanKhoi(ttX, 2);
console.log(JSON.stringify(danhSachKhoiConThieu(ttX)));
```

```typescript title=test
const ttT = taoTrangThaiUpload("video-t", 5);
nhanKhoi(ttT, 4);
nhanKhoi(ttT, 1);
nhanKhoi(ttT, 3);
const thieu1 = danhSachKhoiConThieu(ttT);
if (JSON.stringify(thieu1) !== JSON.stringify([0, 2])) throw new Error("khoi con thieu phai la [0, 2]");
if (laHoanTat(ttT) !== false) throw new Error("chua du khoi thi laHoanTat phai la false");

nhanKhoi(ttT, 4); // trung lap
if (ttT.khoiDaNhan.size !== 3) throw new Error("nhan lai khoi da co khong duoc lam tang so khoi da nhan");

nhanKhoi(ttT, 0);
nhanKhoi(ttT, 2);
if (laHoanTat(ttT) !== true) throw new Error("du tat ca khoi thi laHoanTat phai la true");
const thieu2 = danhSachKhoiConThieu(ttT);
if (thieu2.length !== 0) throw new Error("khong con khoi nao thieu, danh sach phai rong");
```

:::hints
- kind: attention
  body: "Duyet i tu 0 den tongSoKhoi - 1 (dung vong for). Voi moi i, neu khoiDaNhan.has(i) la false thi day i vao mang thieu."
- kind: strategy
  body: "for (let i = 0; i < tt.tongSoKhoi; i++) if (!tt.khoiDaNhan.has(i)) thieu.push(i);"
- kind: one-line
  body: "for (let i = 0; i < tt.tongSoKhoi; i++) if (!tt.khoiDaNhan.has(i)) thieu.push(i);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "[0,1]"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Khối tới không theo thứ tự vẫn được theo dõi đúng. Nhưng hai file KHÁC
nhau tình cờ chứa cùng một khối NỘI dung thì SAO — có cần lưu HAI lần
không?
::::

::::reflect{#nghi-lai}
`nhanKhoi` cố tình KHÔNG quan tâm tới thứ tự — nó chỉ hỏi đúng một câu:
"chỉ số này ĐÃ có mặt chưa?". Chính vì câu hỏi đó không phụ thuộc VÀO
thứ tự tới, hệ thống có thể nhận nhiều khối SONG song (qua nhiều kết
nối mạng khác nhau) mà KHÔNG cần đồng bộ chúng LẠI theo đúng trình tự.
::::

::::checkpoint{mastery=0.74}
::::
