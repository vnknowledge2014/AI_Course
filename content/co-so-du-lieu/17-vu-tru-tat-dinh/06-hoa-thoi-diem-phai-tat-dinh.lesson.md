---
id: co-so-du-lieu.vu-tru-tat-dinh.hoa-thoi-diem-phai-tat-dinh
title: "Hoà thời điểm phải tất định"
summary: "themSuKien gán MỖI sự kiện một thuTuChen (số thứ tự chèn, tăng dần tự động) BÊN CẠNH thoiDiem. soSanhSuKien so sánh thoiDiem TRƯỚC -- nếu hoà, so sánh thuTuChen (ai chèn trước thắng). Ba sự kiện CÙNG thoiDiem=100 chèn theo thứ tự x, y, z -- layTiepTheo LUÔN trả về đúng x, y, z, lặp lại y hệt ở MỌI lần chạy lại, vì tie-break không còn phụ thuộc cách duyệt mảng mà phụ thuộc một con số tường minh."
locale: vi
track: co-so-du-lieu
module: vu-tru-tat-dinh
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [db.hoa-thoi-diem-phai-tat-dinh]
requires: [db.hang-doi-su-kien-theo-thoi-diem]
concepts: [db.hoa-thoi-diem-phai-tat-dinh]
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
Bài trước, luật hoà "AI đứng trước trong mảng thắng" chỉ LÀ tác dụng
PHỤ của cách duyệt tuyến tính — đổi CÁCH cài đặt (VÍ dụ dùng min-heap
thay VÌ quét mảng) CÓ THỂ âm thầm đổi LUÔN kết quả hoà. Cần một luật
TƯỜNG minh, không phụ thuộc cài đặt.
::::

::::explain{#tie-break-tuong-minh}
`themSuKien` gán MỖI sự kiện một `thuTuChen` (số thứ TỰ chèn, tăng
dần tự ĐỘNG) BÊN CẠNH `thoiDiem`. `soSanhSuKien` so sánh `thoiDiem`
TRƯỚC — nếu HOÀ, so sánh `thuTuChen` (ai chèn TRƯỚC thắng):

```typescript title=readonly
interface SuKien { thoiDiem: number; thuTuChen: number; nhan: string; }
interface HangDoi { danhSach: SuKien[]; demChen: number; }

function taoHangDoi(): HangDoi {
  return { danhSach: [], demChen: 0 };
}

function themSuKien(hd: HangDoi, thoiDiem: number, nhan: string): void {
  hd.danhSach.push({ thoiDiem, thuTuChen: hd.demChen, nhan });
  hd.demChen++;
}

function soSanhSuKien(a: SuKien, b: SuKien): number {
  if (a.thoiDiem !== b.thoiDiem) return a.thoiDiem - b.thoiDiem;
  return a.thuTuChen - b.thuTuChen;
}

function layTiepTheo(hd: HangDoi): SuKien | undefined {
  if (hd.danhSach.length === 0) return undefined;
  let idxNhoNhat = 0;
  for (let i = 1; i < hd.danhSach.length; i++) {
    if (soSanhSuKien(hd.danhSach[i]!, hd.danhSach[idxNhoNhat]!) < 0) idxNhoNhat = i;
  }
  return hd.danhSach.splice(idxNhoNhat, 1)[0];
}

const hd = taoHangDoi();
themSuKien(hd, 100, "x");
themSuKien(hd, 100, "y");
themSuKien(hd, 100, "z");

console.log("lay 1:", layTiepTheo(hd)?.nhan);
console.log("lay 2:", layTiepTheo(hd)?.nhan);
console.log("lay 3:", layTiepTheo(hd)?.nhan);
```

```text title=readonly
lay 1: x
lay 2: y
lay 3: z
```

BA sự kiện CÙNG `thoiDiem=100` — `soSanhSuKien` không hề THẤY chúng
"bằng nhau" theo NGHĨA mơ hồ, mà so SÁNH tiếp `thuTuChen` (`0, 1,
2`, đúng thứ tự CHÈN `x, y, z`). Kết quả `layTiepTheo` LUÔN đúng `x,
y, z` — KHÔNG phụ thuộc thuật toán TÌM phần tử nhỏ nhất là quét
tuyến TÍNH hay MỘT cấu trúc khác.
::::

::::example{#tai-sao-khong-dua-vao-thu-tu-mang}
Bài trước, "ai đứng TRƯỚC trong mảng thắng KHI hoà" chỉ đúng VÌ vòng
`for` dùng SO sánh `<` nghiêm ngặt (KHÔNG cập nhật `idxNhoNhat` khi
GẶP một phần tử BẰNG, chỉ khi NHỎ hơn thật SỰ) — một CHI tiết cài
đặt, không phải MỘT lời hứa. Nếu SAU này đổi `layTiepTheo` sang dùng
MỘT thuật toán khác (VÍ dụ `Array.prototype.sort` trước RỒI lấy
phần tử ĐẦU — `sort` KHÔNG đảm bảo ỔN định thứ tự phần tử BẰNG nhau
Ở MỌI engine), kết quả hoà CÓ thể đổi mà KHÔNG ai hay. `thuTuChen`
LÀ một trường DỮ LIỆU tường minh — bất kỳ thuật toán TÌM min nào,
MIỄN so sánh đúng `soSanhSuKien`, ĐỀU cho kết quả hoà GIỐNG hệt nhau.
::::

::::predict{#doan-hoa-mot-phan commitOnce}
Bốn sự kiện: `themSuKien(hd, 50, "p")`, `themSuKien(hd, 100, "q")`,
`themSuKien(hd, 50, "r")`, `themSuKien(hd, 30, "s")` (chèn ĐÚNG thứ
tự NÀY). Gọi `layTiepTheo` BỐN lần liên tiếp — thứ tự nhãn TRẢ về
LÀ gì?
:::opt{correct}
`s, p, r, q` — `s` (`thoiDiem=30`) nhỏ NHẤT nên ra trước; `p` VÀ `r`
hoà Ở `thoiDiem=50`, NHƯNG `p` được chèn TRƯỚC (`thuTuChen` nhỏ
hơn) nên `p` thắng; CUỐI cùng `q` (`100`, lớn nhất)
:::
:::opt
`s, r, p, q` — `r` được chèn SAU `p` nhưng "MỚI hơn" nên ưu tiên xử
lý trước
::why
Trực giác NÀY đảo NGƯỢC đúng quy tắc — có THỂ đang nghĩ tới một hệ
THỐNG kiểu "LIFO" (mới nhất xử lý TRƯỚC) — nhưng `soSanhSuKien` làm
NGƯỢC lại.

Chỗ lệch: `soSanhSuKien` trả VỀ `a.thuTuChen - b.thuTuChen` khi HOÀ
`thoiDiem` — `p` CÓ `thuTuChen` NHỎ hơn `r` (được chèn TRƯỚC, Ở lần
gọi `themSuKien` thứ HAI, trong khi `r` là lần thứ BA), nên `p`
"nhỏ hơn" `r` theo LUẬT so sánh NÀY, VÀ `layTiepTheo` chọn `p`
trước. Tie-break Ở ĐÂY LÀ "ai đến TRƯỚC được xử lý trước" (FIFO
GIỮA những sự kiện hoà `thoiDiem`), không phải ngược lại.
::
:::
::::

::::code{#viet_so_sanh_su_kien}
Hoàn thiện `soSanhSuKien` — nếu `thoiDiem` khác nhau, so sánh THEO
`thoiDiem`; nếu HOÀ, so sánh theo `thuTuChen`.

```typescript title=starter
interface SuKien { thoiDiem: number; thuTuChen: number; nhan: string; }
interface HangDoi { danhSach: SuKien[]; demChen: number; }

function taoHangDoi(): HangDoi {
  return { danhSach: [], demChen: 0 };
}

function themSuKien(hd: HangDoi, thoiDiem: number, nhan: string): void {
  hd.danhSach.push({ thoiDiem, thuTuChen: hd.demChen, nhan });
  hd.demChen++;
}

function soSanhSuKien(a: SuKien, b: SuKien): number {
  ___
  return a.thuTuChen - b.thuTuChen;
}

function layTiepTheo(hd: HangDoi): SuKien | undefined {
  if (hd.danhSach.length === 0) return undefined;
  let idxNhoNhat = 0;
  for (let i = 1; i < hd.danhSach.length; i++) {
    if (soSanhSuKien(hd.danhSach[i]!, hd.danhSach[idxNhoNhat]!) < 0) idxNhoNhat = i;
  }
  return hd.danhSach.splice(idxNhoNhat, 1)[0];
}

const hd = taoHangDoi();
themSuKien(hd, 100, "x");
themSuKien(hd, 100, "y");
console.log(layTiepTheo(hd)?.nhan);
```

```typescript title=solution
interface SuKien { thoiDiem: number; thuTuChen: number; nhan: string; }
interface HangDoi { danhSach: SuKien[]; demChen: number; }

function taoHangDoi(): HangDoi {
  return { danhSach: [], demChen: 0 };
}

function themSuKien(hd: HangDoi, thoiDiem: number, nhan: string): void {
  hd.danhSach.push({ thoiDiem, thuTuChen: hd.demChen, nhan });
  hd.demChen++;
}

function soSanhSuKien(a: SuKien, b: SuKien): number {
  if (a.thoiDiem !== b.thoiDiem) return a.thoiDiem - b.thoiDiem;
  return a.thuTuChen - b.thuTuChen;
}

function layTiepTheo(hd: HangDoi): SuKien | undefined {
  if (hd.danhSach.length === 0) return undefined;
  let idxNhoNhat = 0;
  for (let i = 1; i < hd.danhSach.length; i++) {
    if (soSanhSuKien(hd.danhSach[i]!, hd.danhSach[idxNhoNhat]!) < 0) idxNhoNhat = i;
  }
  return hd.danhSach.splice(idxNhoNhat, 1)[0];
}

const hd = taoHangDoi();
themSuKien(hd, 100, "x");
themSuKien(hd, 100, "y");
console.log(layTiepTheo(hd)?.nhan);
```

```typescript title=test
const hd2 = taoHangDoi();
themSuKien(hd2, 50, "p");
themSuKien(hd2, 100, "q");
themSuKien(hd2, 50, "r");
themSuKien(hd2, 30, "s");

const k1 = layTiepTheo(hd2);
if (k1?.nhan !== "s") throw new Error("lay dau tien phai la 's' (thoiDiem=30, nho nhat, khong hoa)");
const k2 = layTiepTheo(hd2);
if (k2?.nhan !== "p") throw new Error("lay thu hai phai la 'p' -- hoa thoiDiem=50 voi 'r', nhung 'p' chen TRUOC 'r'");
const k3 = layTiepTheo(hd2);
if (k3?.nhan !== "r") throw new Error("lay thu ba phai la 'r' (thoiDiem=50, sau 'p')");
const k4 = layTiepTheo(hd2);
if (k4?.nhan !== "q") throw new Error("lay cuoi cung phai la 'q' (thoiDiem=100, lon nhat)");

const hd3 = taoHangDoi();
themSuKien(hd3, 10, "a");
themSuKien(hd3, 10, "b");
themSuKien(hd3, 10, "c");
const thuTu3: string[] = [];
let e;
while ((e = layTiepTheo(hd3)) !== undefined) thuTu3.push(e.nhan);
if (thuTu3.join(",") !== "a,b,c") throw new Error("ba su kien hoa het thoiDiem phai ra dung thu tu chen a,b,c");

for (let lan = 0; lan < 3; lan++) {
  const hdLap = taoHangDoi();
  themSuKien(hdLap, 5, "m");
  themSuKien(hdLap, 5, "n");
  const first = layTiepTheo(hdLap);
  if (first?.nhan !== "m") throw new Error(`lan chay ${lan}: hoa thoiDiem phai LUON cho ket qua giong het nhau (tat dinh) -- phai la 'm'`);
}
```

:::hints
- kind: attention
  body: "Neu thoiDiem cua a va b khac nhau thi so sanh theo thoiDiem -- mot dong."
- kind: strategy
  body: "if (a.thoiDiem !== b.thoiDiem) return a.thoiDiem - b.thoiDiem;"
- kind: one-line
  body: "if (a.thoiDiem !== b.thoiDiem) return a.thoiDiem - b.thoiDiem;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "x"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Thời gian ảo, hàng đợi tất định (kể cả khi hoà), số ngẫu nhiên tất
định — đủ mảnh ghép để RÁP một mô phỏng chạy lại ĐÚNG tuyệt đối.
::::

::::reflect{#nghi-lai}
`soSanhSuKien` LÀ một hàm bé — hai dòng SO sánh — nhưng nó ĐÓNG vai
trò LỚN: biến "hoà thì AI thắng" từ một câu hỏi MƠ hồ (phụ thuộc
cài đặt) thành một LUẬT tường minh (phụ thuộc đúng MỘT trường dữ
liệu, `thuTuChen`). Đây LÀ nguyên tắc chung của MỌI hệ thống tất
định: KHÔNG BAO giờ để hành vi phụ thuộc VÀO một chi TIẾT cài đặt
"tình cờ" đúng — luôn LÀM nó tường minh.
::::

::::checkpoint{mastery=0.85}
::::
