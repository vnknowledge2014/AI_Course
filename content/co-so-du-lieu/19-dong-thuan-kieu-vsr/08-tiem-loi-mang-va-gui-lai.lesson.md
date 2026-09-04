---
id: co-so-du-lieu.dong-thuan-kieu-vsr.tiem-loi-mang-va-gui-lai
title: "Tiêm lỗi mạng và gửi lại"
summary: "kiemTraCanGuiLai(dh,thoiDiemGuiLanDau,nguongTimeoutMs) so đo lệch thời gian ẢO (dh.hienTai - thoiDiemGuiLanDau) với ngưỡng, dùng >= (đúng ngưỡng LÀ đủ để gửi lại). t=0..99: false; t=100 (đúng ngưỡng): true; t=101: true. Kết hợp matGoiTheoSeed (q18): seed=3n, tỉ lệ mất 30%, gửi tới 2 backup -- backup1 mất, backup2 tới -- quorum(N=3)=2 (primary+backup2) VẪN đạt, hệ thống tiến được dù mất một gói."
locale: vi
track: co-so-du-lieu
module: dong-thuan-kieu-vsr
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.tiem-loi-mang-va-gui-lai]
requires: [db.commit-lan-xuong-backup]
concepts: [db.tiem-loi-mang-va-gui-lai]
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
Bảy bài đầu ĐÃ ráp đủ Prepare/PrepareOk/Commit trên MỘT đường mạng
hoàn HẢO. q18 dạy `matGoiTheoSeed` LÀM một gói tin biến mất. Nếu MỘT
Prepare không BAO giờ tới nơi, hệ thống có TIẾP tục được không?
::::

::::explain{#gui-lai-theo-dong-ho-ao}
`kiemTraCanGuiLai(dh, thoiDiemGuiLanDau, nguongTimeoutMs)` so ĐO độ
lệch thời gian ẢO (`dh.hienTai - thoiDiemGuiLanDau`) VỚI ngưỡng, dùng
`>=` (ĐÚNG ngưỡng LÀ đã đủ để gửi LẠI, không cần chờ VƯỢT qua) — dựa
HOÀN toàn trên `DongHoAo` (q17), KHÔNG hề gọi `Date.now()`:

```typescript title=readonly
interface DongHoAo { hienTai: number; }
function taoDongHoAo(): DongHoAo { return { hienTai: 0 }; }
function tienToi(dh: DongHoAo, soMs: number): void { dh.hienTai += soMs; }

function kiemTraCanGuiLai(dh: DongHoAo, thoiDiemGuiLanDau: number, nguongTimeoutMs: number): boolean {
  return dh.hienTai - thoiDiemGuiLanDau >= nguongTimeoutMs;
}

const dh = taoDongHoAo();
tienToi(dh, 99);
console.log("t=99:", kiemTraCanGuiLai(dh, 0, 100));
tienToi(dh, 1);
console.log("t=100 (dung nguong):", kiemTraCanGuiLai(dh, 0, 100));
```

```text title=readonly
t=99: false
t=100 (dung nguong): true
```

Kết hợp VỚI `matGoiTheoSeed` (q18): `seed=3n`, tỉ lệ mất `30%`, gửi
Prepare TỚI hai backup —

```typescript title=readonly
const MASK64 = (1n << 64n) - 1n;
interface TrangThai { s0: bigint; s1: bigint; }
function gieoHat(seed: bigint): TrangThai {
  let s0 = seed & MASK64;
  if (s0 === 0n) s0 = 0x9e3779b97f4a7c15n;
  let s1 = (seed * 6364136223846793005n + 1442695040888963407n) & MASK64;
  if (s1 === 0n) s1 = 0xbf58476d1ce4e5b9n;
  return { s0, s1 };
}
function soTiepTheo(tt: TrangThai): bigint {
  let s1 = tt.s0;
  const s0 = tt.s1;
  const ketQua = (s1 + s0) & MASK64;
  tt.s0 = s0;
  s1 ^= (s1 << 23n) & MASK64;
  s1 ^= s1 >> 17n;
  s1 ^= s0 ^ (s0 >> 26n);
  tt.s1 = s1 & MASK64;
  return ketQua;
}
function soNguyenTrongKhoang(tt: TrangThai, min: number, max: number): number {
  return min + Number(soTiepTheo(tt) % BigInt(max - min));
}
function matGoiTheoSeed(tt: TrangThai, tyLeMatPhanTram: number): boolean {
  return soNguyenTrongKhoang(tt, 0, 100) < tyLeMatPhanTram;
}
const tt = gieoHat(3n);
const toiBackup1 = !matGoiTheoSeed(tt, 30);
const toiBackup2 = !matGoiTheoSeed(tt, 30);
console.log("Prepare toi backup1 den noi?", toiBackup1, "Prepare toi backup2 den noi?", toiBackup2);
```

```text title=readonly
Prepare toi backup1 den noi? false Prepare toi backup2 den noi? true
```

`backup1` MẤT gói — nhưng `backup2` NHẬN được: quorum(`N=3`) `= 2`
(primary + `backup2`) VẪN đạt, hệ thống VẪN tiến được. `kiemTraCanGuiLai`
LÀ lưới an TOÀN cho trường HỢP xấu hơn — khi CẢ hai backup cùng LỠ
gói trong MỘT vòng, primary chờ ĐỦ `nguongTimeoutMs` (đo BẰNG
`DongHoAo`, không phải đồng hồ THẬT) rồi gửi lại.
::::

::::example{#tai-sao-khong-date-now}
NẾU `kiemTraCanGuiLai` dùng `Date.now()` thay VÌ `DongHoAo`, kết quả
sẽ phụ thuộc VÀO tốc độ máy CHẠY mô phỏng — chạy trên máy CHẬM có thể
"gửi lại" SỚM hơn máy nhanh, PHÁ vỡ tính tái hiện tuyệt đối (q17 bài
7). `DongHoAo` tách "thời gian mô PHỎNG" khỏi "thời gian THẬT" — y
hệt LÝ do q17 bài 4 cấm `Date.now()` NGAY từ đầu.
::::

::::predict{#doan-do-lech-am commitOnce}
`kiemTraCanGuiLai(dh, 500, 100)` với `dh.hienTai = 480` (`thoiDiemGuiLanDau`
LỚN hơn `dh.hienTai`, một tình huống KHÔNG nên xảy ra trong thực TẾ
nhưng vẫn LÀ đầu VÀO hợp lệ về kiểu). Kết quả LÀ gì?
:::opt{correct}
`false` — `dh.hienTai - thoiDiemGuiLanDau = 480 - 500 = -20`, VÀ
`-20 >= 100` LÀ sai; hàm KHÔNG hề kiểm tra dấu, chỉ so SÁNH số học
thuần, độ lệch ÂM luôn nhỏ hơn một ngưỡng dương
:::
:::opt
Lỗi runtime — `thoiDiemGuiLanDau` KHÔNG được lớn hơn `dh.hienTai`
::why
Trực giác NÀY đúng về mặt Ý nghĩa (thời điểm gửi LẦN đầu không thể
"trong tương lai") — nhưng SAI về code: `kiemTraCanGuiLai` LÀ một
phép TRỪ VÀ so sánh trần trụi, giống HỆT `tienToi` (q17 bài 4) không
kiểm tra dấu.

Chỗ lệch: `dh.hienTai - thoiDiemGuiLanDau` LÀ một biểu thức số học
BÌNH thường — JavaScript không TỰ chặn kết quả âm, VÀ `-20 >= 100`
đơn giản LÀ `false`. Bài học LẶP lại từ q17: một hàm "đo thời gian
TRÔI qua" không tự BẢO vệ khỏi đầu vào phi lý — code GỌI nó phải tự
đảm bảo tính hợp LỆ.
::
:::
::::

::::code{#viet_kiem_tra_can_gui_lai}
Hoàn thiện `kiemTraCanGuiLai` — so sánh độ lệch thời gian ẢO với
`nguongTimeoutMs`, dùng `>=`.

```typescript title=starter
interface DongHoAo { hienTai: number; }
function taoDongHoAo(): DongHoAo { return { hienTai: 0 }; }
function tienToi(dh: DongHoAo, soMs: number): void { dh.hienTai += soMs; }

function kiemTraCanGuiLai(dh: DongHoAo, thoiDiemGuiLanDau: number, nguongTimeoutMs: number): boolean {
  ___
}

const dh = taoDongHoAo();
tienToi(dh, 100);
console.log(kiemTraCanGuiLai(dh, 0, 100));
```

```typescript title=solution
interface DongHoAo { hienTai: number; }
function taoDongHoAo(): DongHoAo { return { hienTai: 0 }; }
function tienToi(dh: DongHoAo, soMs: number): void { dh.hienTai += soMs; }

function kiemTraCanGuiLai(dh: DongHoAo, thoiDiemGuiLanDau: number, nguongTimeoutMs: number): boolean {
  return dh.hienTai - thoiDiemGuiLanDau >= nguongTimeoutMs;
}

const dh = taoDongHoAo();
tienToi(dh, 100);
console.log(kiemTraCanGuiLai(dh, 0, 100));
```

```typescript title=test
const dh2 = taoDongHoAo();
if (kiemTraCanGuiLai(dh2, 0, 100) !== false) throw new Error("t=0, chua qua nguong -- phai la false");
tienToi(dh2, 99);
if (kiemTraCanGuiLai(dh2, 0, 100) !== false) throw new Error("t=99 < nguong=100 -- van phai la false");
tienToi(dh2, 1); // t=100
if (kiemTraCanGuiLai(dh2, 0, 100) !== true) throw new Error("t=100 == nguong -- phai la true (>= khong phai >)");
tienToi(dh2, 1); // t=101
if (kiemTraCanGuiLai(dh2, 0, 100) !== true) throw new Error("t=101 > nguong -- phai la true");
const dh3 = taoDongHoAo();
tienToi(dh3, 500);
if (kiemTraCanGuiLai(dh3, 480, 100) !== false) throw new Error("hienTai=500, guiLanDau=480 -- do lech chi la 20 < 100 -- phai la false");
if (kiemTraCanGuiLai(dh3, 400, 100) !== true) throw new Error("hienTai=500, guiLanDau=400 -- do lech la 100 >= 100 -- phai la true");
```

:::hints
- kind: attention
  body: "Tra ve dh.hienTai - thoiDiemGuiLanDau >= nguongTimeoutMs -- mot dong."
- kind: strategy
  body: "return dh.hienTai - thoiDiemGuiLanDau >= nguongTimeoutMs;"
- kind: one-line
  body: "return dh.hienTai - thoiDiemGuiLanDau >= nguongTimeoutMs;"
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
Mất một gói không làm hệ thống ngã. Nhưng NẾU mất TẤT CẢ mọi gói từ
primary — nghĩa LÀ primary CHẾT — hệ thống có PHÁT hiện ra không?
::::

::::reflect{#nghi-lai}
`kiemTraCanGuiLai` không giới thiệu phép TOÁN mới — nó tái sử dụng
đúng "so ĐO thời gian ẢO" (q17 bài 4) cho một VAI trò mới: quyết định
KHI nào retry. Kết hợp VỚI `matGoiTheoSeed` (q18), bài NÀY chứng minh
đúng lời hứa CỦA VSR: một hệ thống ĐỒNG thuận đúng nghĩa KHÔNG cần
mạng hoàn hảo — chỉ cần QUORUM còn sống VÀ đủ kiên nhẫn để gửi lại.
::::

::::checkpoint{mastery=0.85}
::::
