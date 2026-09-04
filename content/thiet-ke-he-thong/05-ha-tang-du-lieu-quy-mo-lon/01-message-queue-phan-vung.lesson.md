---
id: thiet-ke-he-thong.ha-tang-du-lieu-quy-mo-lon.message-queue-phan-vung
title: "Hàng đợi phân vùng: cùng khoá, cùng chỗ"
summary: "guiTinNhan(hd, khoa, noiDung) băm khoa RỒI lấy phần dư cho soPhanVung để chọn phân vùng cố định -- cùng khoa 'don-hang-101' LUÔN rơi vào ĐÚNG một phân vùng (phân vùng 2 trong vi du 3 phan vung), offset trong phân vùng đó tăng dần 0, 1, 2; khoa khác ('don-hang-202', 'don-hang-303') co the roi vao phan vung KHAC, offset RIÊNG bắt đầu lại từ 0 -- KHÔNG có thứ tự toàn cục giữa các phân vùng."
locale: vi
track: thiet-ke-he-thong
module: ha-tang-du-lieu-quy-mo-lon
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [sd.message-queue-phan-vung]
requires: [sd.boss-thoi-gian-thuc-va-vi-tri]
concepts: [sd.message-queue-phan-vung]
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
Bốn quest T7.1 VÀ ba quest T7.2 trước đã ráp xong dịch vụ ĐỌC, nội dung
lớn, VÀ thời gian thực. Track "Thiết kế thực chiến" giờ RẼ sang một câu
hỏi khác: hệ thống KHÁC (không phải người dùng cuối) cần TRUYỀN dữ liệu
cho NHAU một cách đáng tin cậy. Mảnh đầu tiên — hàng đợi tin nhắn — bắt
đầu bằng một câu hỏi: một tin nhắn MỚI tới thì nó thuộc VỀ đâu?
::::

::::explain{#phan-vung-theo-khoa}
Một hàng đợi phân tán không giữ MỘT danh sách tin nhắn duy nhất — nó
chia ra nhiều PHÂN vùng (partition) độc lập, mỗi phân vùng LÀ một mảng
tin nhắn CÓ thứ tự riêng. `guiTinNhan` băm `khoa` của tin nhắn RỒI lấy
phần dư cho SỐ phân vùng để CHỌN đúng một phân vùng cố định — offset
(vị trí trong phân vùng đó) LÀ độ dài mảng NGAY trước khi đẩy vào:

```typescript title=readonly
interface TinNhan { khoa: string; noiDung: string; }
interface HangDoiPhanVung { soPhanVung: number; phanVung: TinNhan[][]; }
function taoHangDoiPhanVung(soPhanVung: number): HangDoiPhanVung {
  const phanVung: TinNhan[][] = [];
  for (let i = 0; i < soPhanVung; i++) phanVung.push([]);
  return { soPhanVung, phanVung };
}
function bam(khoa: string): number {
  let h = 0;
  for (let i = 0; i < khoa.length; i++) {
    h = (h * 31 + khoa.charCodeAt(i)) % 1000000007;
  }
  return h;
}
function tinhPhanVung(khoa: string, soPhanVung: number): number {
  return bam(khoa) % soPhanVung;
}
interface KetQuaGui { phanVung: number; offset: number; }
function guiTinNhan(hd: HangDoiPhanVung, khoa: string, noiDung: string): KetQuaGui {
  const idx = tinhPhanVung(khoa, hd.soPhanVung);
  const ds = hd.phanVung[idx]!;
  const offset = ds.length;
  ds.push({ khoa, noiDung });
  return { phanVung: idx, offset };
}

const hd = taoHangDoiPhanVung(3);
console.log("gui don-hang-101 lan 1:", JSON.stringify(guiTinNhan(hd, "don-hang-101", "tao don")));
console.log("gui don-hang-101 lan 2:", JSON.stringify(guiTinNhan(hd, "don-hang-101", "cap nhat don")));
console.log("gui don-hang-202 lan 1:", JSON.stringify(guiTinNhan(hd, "don-hang-202", "tao don")));
```

```text title=readonly
gui don-hang-101 lan 1: {"phanVung":2,"offset":0}
gui don-hang-101 lan 2: {"phanVung":2,"offset":1}
gui don-hang-202 lan 1: {"phanVung":1,"offset":0}
```

`"don-hang-101"` băm RA cùng một chỉ số MỖI lần — cả hai tin nhắn của
nó rơi ĐÚNG vào phân vùng `2`, VÀ offset tăng dần `0` RỒI `1` NGAY
trong phân vùng đó. `"don-hang-202"` băm ra chỉ số KHÁC (`1`) — offset
của nó bắt đầu LẠI từ `0`, vì đó LÀ một mảng hoàn toàn riêng.
::::

::::example{#khoa-khac-co-the-chung-phan-vung}
Một khoá MỚI không hề được đảm bảo rơi vào phân vùng riêng — nó CHỈ
đảm bảo LUÔN rơi vào ĐÚNG một phân vùng CỐ định, dù phân vùng đó có
đang chứa tin của khoá KHÁC hay không:

```typescript title=readonly
interface TinNhan { khoa: string; noiDung: string; }
interface HangDoiPhanVung { soPhanVung: number; phanVung: TinNhan[][]; }
function taoHangDoiPhanVung(soPhanVung: number): HangDoiPhanVung {
  const phanVung: TinNhan[][] = [];
  for (let i = 0; i < soPhanVung; i++) phanVung.push([]);
  return { soPhanVung, phanVung };
}
function bam(khoa: string): number {
  let h = 0;
  for (let i = 0; i < khoa.length; i++) {
    h = (h * 31 + khoa.charCodeAt(i)) % 1000000007;
  }
  return h;
}
function tinhPhanVung(khoa: string, soPhanVung: number): number {
  return bam(khoa) % soPhanVung;
}
interface KetQuaGui { phanVung: number; offset: number; }
function guiTinNhan(hd: HangDoiPhanVung, khoa: string, noiDung: string): KetQuaGui {
  const idx = tinhPhanVung(khoa, hd.soPhanVung);
  const ds = hd.phanVung[idx]!;
  const offset = ds.length;
  ds.push({ khoa, noiDung });
  return { phanVung: idx, offset };
}

// tai lap dung trang thai tu khoi truoc, KHONG in lai: don-hang-101 da gui 2
// lan (phan vung 2), don-hang-202 da gui 1 lan (phan vung 1)
const hd = taoHangDoiPhanVung(3);
guiTinNhan(hd, "don-hang-101", "tao don");
guiTinNhan(hd, "don-hang-101", "cap nhat don");
guiTinNhan(hd, "don-hang-202", "tao don");

console.log("gui don-hang-303 lan 1:", JSON.stringify(guiTinNhan(hd, "don-hang-303", "tao don")));
console.log("gui don-hang-101 lan 3:", JSON.stringify(guiTinNhan(hd, "don-hang-101", "huy don")));
console.log("so tin nhan trong phan vung 0:", hd.phanVung[0]!.length);
console.log("so tin nhan trong phan vung 1:", hd.phanVung[1]!.length);
console.log("so tin nhan trong phan vung 2:", hd.phanVung[2]!.length);
```

```text title=readonly
gui don-hang-303 lan 1: {"phanVung":0,"offset":0}
gui don-hang-101 lan 3: {"phanVung":2,"offset":2}
so tin nhan trong phan vung 0: 1
so tin nhan trong phan vung 1: 1
so tin nhan trong phan vung 2: 3
```

`"don-hang-303"` rơi vào phân vùng `0` — khác cả `101` LẪN `202`.
`"don-hang-101"` gửi LẦN thứ ba vẫn quay VỀ đúng phân vùng `2`, offset
tiếp tục tăng thành `2` — offset LÀ một bộ đếm CỦA riêng phân vùng đó,
không phải của riêng khoá đó.
::::

::::predict{#doan-khoa-moi-chung-phan-vung commitOnce}
Phân vùng `2` hiện đang chứa `3` tin nhắn (cả ba đều khoá
`"don-hang-101"`). Một khoá HOÀN TOÀN mới, `"don-hang-404"`, tình cờ
băm RA cùng chỉ số phân vùng LÀ `2`. Gọi `guiTinNhan(hd, "don-hang-404",
...)` NGAY lúc này. Offset trả VỀ LÀ bao nhiêu?

:::opt{correct}
`3` — offset LÀ độ dài hiện tại của MẢNG phân vùng `2` (đã có `3` tin
nhắn), bất kể tin nhắn MỚI mang khoá gì; `"don-hang-404"` dùng CHUNG bộ
đếm offset của phân vùng đó VỚI `"don-hang-101"`
:::
:::opt
`0` — đây LÀ khoá hoàn toàn mới, chưa từng gửi tin nhắn NÀO, nên offset
của NÓ phải bắt đầu lại từ đầu
::why
Nhầm "offset riêng theo TỪNG khoá" VỚI "offset riêng theo TỪNG phân
vùng" — nhưng `guiTinNhan` không hề giữ một bộ đếm CHO mỗi khoá.

Chỗ lệch: dòng `const offset = ds.length;` đọc độ dài của `ds =
hd.phanVung[idx]`, tức LÀ mảng của CẢ phân vùng, không phải một mảng
riêng cho `"don-hang-404"`. Vì `"don-hang-404"` băm ra CÙNG chỉ số `2`
với `"don-hang-101"`, nó chia SẺ đúng bộ đếm offset đã có sẵn `3` tin
nhắn — offset trả về LÀ `3`, không phải `0`.
::
:::
::::

::::code{#viet_gui_tin_nhan}
Hoàn thiện `guiTinNhan` — phân vùng ĐÍCH đã được xác định (`idx`) VÀ
mảng của phân vùng đó đã lấy RA (`ds`). Còn thiếu: tính offset (độ dài
hiện tại của `ds`), đẩy tin nhắn MỚI vào `ds`, RỒI trả về kết quả.

```typescript title=starter
interface TinNhan { khoa: string; noiDung: string; }
interface HangDoiPhanVung { soPhanVung: number; phanVung: TinNhan[][]; }
function taoHangDoiPhanVung(soPhanVung: number): HangDoiPhanVung {
  const phanVung: TinNhan[][] = [];
  for (let i = 0; i < soPhanVung; i++) phanVung.push([]);
  return { soPhanVung, phanVung };
}
function bam(khoa: string): number {
  let h = 0;
  for (let i = 0; i < khoa.length; i++) {
    h = (h * 31 + khoa.charCodeAt(i)) % 1000000007;
  }
  return h;
}
function tinhPhanVung(khoa: string, soPhanVung: number): number {
  return bam(khoa) % soPhanVung;
}
interface KetQuaGui { phanVung: number; offset: number; }
function guiTinNhan(hd: HangDoiPhanVung, khoa: string, noiDung: string): KetQuaGui {
  const idx = tinhPhanVung(khoa, hd.soPhanVung);
  const ds = hd.phanVung[idx]!;
  ___
}

const hdX = taoHangDoiPhanVung(4);
console.log(JSON.stringify(guiTinNhan(hdX, "khach-99", "a")), JSON.stringify(guiTinNhan(hdX, "khach-99", "b")));
```

```typescript title=solution
interface TinNhan { khoa: string; noiDung: string; }
interface HangDoiPhanVung { soPhanVung: number; phanVung: TinNhan[][]; }
function taoHangDoiPhanVung(soPhanVung: number): HangDoiPhanVung {
  const phanVung: TinNhan[][] = [];
  for (let i = 0; i < soPhanVung; i++) phanVung.push([]);
  return { soPhanVung, phanVung };
}
function bam(khoa: string): number {
  let h = 0;
  for (let i = 0; i < khoa.length; i++) {
    h = (h * 31 + khoa.charCodeAt(i)) % 1000000007;
  }
  return h;
}
function tinhPhanVung(khoa: string, soPhanVung: number): number {
  return bam(khoa) % soPhanVung;
}
interface KetQuaGui { phanVung: number; offset: number; }
function guiTinNhan(hd: HangDoiPhanVung, khoa: string, noiDung: string): KetQuaGui {
  const idx = tinhPhanVung(khoa, hd.soPhanVung);
  const ds = hd.phanVung[idx]!;
  const offset = ds.length;
  ds.push({ khoa, noiDung });
  return { phanVung: idx, offset };
}

const hdX = taoHangDoiPhanVung(4);
console.log(JSON.stringify(guiTinNhan(hdX, "khach-99", "a")), JSON.stringify(guiTinNhan(hdX, "khach-99", "b")));
```

```typescript title=test
const hdT = taoHangDoiPhanVung(3);
const g1 = guiTinNhan(hdT, "don-hang-101", "tao don");
if (g1.phanVung !== 2) throw new Error("don-hang-101 phai roi vao phan vung 2 (dung ham bam da cho)");
if (g1.offset !== 0) throw new Error("tin nhan dau tien trong phan vung phai co offset 0");

const g2 = guiTinNhan(hdT, "don-hang-101", "cap nhat don");
if (g2.phanVung !== 2) throw new Error("cung khoa don-hang-101 phai LUON roi vao CUNG phan vung");
if (g2.offset !== 1) throw new Error("tin nhan thu hai CUNG phan vung phai co offset tang len 1");

const g3 = guiTinNhan(hdT, "don-hang-202", "tao don");
if (g3.phanVung !== 1) throw new Error("don-hang-202 phai roi vao phan vung 1 (khoa khac, phan vung khac)");
if (g3.offset !== 0) throw new Error("phan vung rieng phai co offset rieng, bat dau tu 0");

const g4 = guiTinNhan(hdT, "don-hang-404", "tao don");
if (g4.phanVung !== 2) throw new Error("don-hang-404 phai roi vao phan vung 2 (dung ham bam da cho)");
if (g4.offset !== 2) throw new Error("khoa moi nhung CHUNG phan vung 2 phai tiep tuc offset cua phan vung do (2), khong bat dau lai tu 0");

if (hdT.phanVung[0]!.length !== 0) throw new Error("phan vung 0 chua nhan tin nhan nao, phai rong");
if (hdT.phanVung[2]!.length !== 3) throw new Error("phan vung 2 phai co dung 3 tin nhan (101 x2, 404 x1)");
```

:::hints
- kind: attention
  body: "Con thieu ba viec trong `ds`: tinh offset (do dai hien tai cua ds), day tin nhan moi vao ds, roi tra ve { phanVung: idx, offset }."
- kind: strategy
  body: "const offset = ds.length; ds.push({ khoa, noiDung }); return { phanVung: idx, offset };"
- kind: one-line
  body: "const offset = ds.length; ds.push({ khoa, noiDung }); return { phanVung: idx, offset };"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "\"offset\":1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng khoá luôn cùng phân vùng, thứ tự trong MỘT phân vùng luôn giữ
nguyên — nhưng ĐÓ mới là một nửa câu chuyện. Tin nhắn còn chờ Ở đó cho
tới khi nào?
::::

::::reflect{#nghi-lai}
`guiTinNhan` không hề cân bằng tải theo kiểu xoay VÒNG (round-robin) —
nó CỐ định một khoá vào một phân vùng bằng phép băm, ĐỔI lại một điều
quan trọng: thứ tự tương đối giữa các tin nhắn CÙNG khoá luôn được giữ
nguyên. Cái giá LÀ không có thứ tự toàn cục giữa các khoá khác nhau —
`"don-hang-202"` và `"don-hang-303"` không hề biết gì về NHAU.
::::

::::checkpoint{mastery=0.66}
::::
