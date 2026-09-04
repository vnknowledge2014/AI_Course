---
id: thiet-ke-he-thong.url-rut-gon-va-thu-thap-web.bo-dem-toi-ma-ngan
title: "Bộ đếm toàn cục: mã ngắn không bao giờ trùng"
summary: "BoDemToanCuc{giaTriHienTai} bắt đầu tại 0, tăng đúng 1 sau MỖI lần laMaNganTiepTheo được gọi; mã ngắn trả về LÀ maHoaBase62(giaTriHienTai) TRƯỚC khi tăng. Vì maHoaBase62 là song ánh (bài 2) và bo.giaTriHienTai không bao giờ lặp lại giá trị đã cấp, 500 lần gọi liên tiếp sinh ra 500 mã HOÀN TOÀN không trùng (kiểm bằng Set, size khớp array.length) — khác hẳn cách sinh ngẫu nhiên, bộ đếm KHÔNG BAO GIỜ cần kiểm tra va chạm."
locale: vi
track: thiet-ke-he-thong
module: url-rut-gon-va-thu-thap-web
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [sd.bo-dem-toi-ma-ngan]
requires: [sd.base62-giai-ma]
concepts: [sd.bo-dem-toi-ma-ngan]
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
Mã hoá VÀ giải mã (hai bài trước) LÀ một cặp hàm biến đổi hoàn hảo.
Còn thiếu đúng một mảnh: nguồn cấp SỐ nguyên. Nguồn đơn giản nhất —
một bộ ĐẾM chỉ biết tăng.
::::

::::explain{#bo-dem-toan-cuc}
`BoDemToanCuc` giữ đúng MỘT con số, `giaTriHienTai`, bắt đầu TỪ `0`.
Mỗi lần cấp mã NGẮN, `laMaNganTiepTheo` mã hoá giá trị HIỆN tại (dùng
`maHoaBase62` bài trước) RỒI mới tăng bộ đếm LÊN — nên mã ĐẦU tiên
luôn ứng với `0`, mã THỨ hai ứng với `1`, cứ thế:

```typescript title=readonly
const BANG_BASE62 = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

function maHoaBase62(n: number): string {
  if (n === 0) return "0";
  let ketQua = "";
  let con = n;
  while (con > 0) {
    ketQua = BANG_BASE62[con % 62]! + ketQua;
    con = Math.floor(con / 62);
  }
  return ketQua;
}

interface BoDemToanCuc { giaTriHienTai: number; }
function taoBoDemToanCuc(): BoDemToanCuc {
  return { giaTriHienTai: 0 };
}
function laMaNganTiepTheo(bo: BoDemToanCuc): string {
  const ma = maHoaBase62(bo.giaTriHienTai);
  bo.giaTriHienTai += 1;
  return ma;
}

const bo = taoBoDemToanCuc();
console.log("ma 1:", laMaNganTiepTheo(bo));
console.log("ma 2:", laMaNganTiepTheo(bo));
console.log("ma 3:", laMaNganTiepTheo(bo));
console.log("giaTriHienTai sau 3 lan cap:", bo.giaTriHienTai);
```

```text title=readonly
ma 1: 0
ma 2: 1
ma 3: 2
giaTriHienTai sau 3 lan cap: 3
```

Ba mã ĐẦU LÀ `"0"`, `"1"`, `"2"` — đúng bằng `maHoaBase62` của BA số
nguyên đầu TIÊN mà bộ đếm đã cấp. `giaTriHienTai` LUÔN tăng đúng `1`
sau mỗi LẦN gọi, không hề phụ THUỘC vào mã vừa sinh RA LÀ gì.
::::

::::example{#khong-can-kiem-trung}
Vì `giaTriHienTai` chỉ TĂNG chứ không bao giờ QUAY lại một giá trị đã
cấp, VÀ `maHoaBase62` LÀ một song ánh (mỗi số nguyên ứng đúng một
chuỗi, bài TRƯỚC), sinh hàng loạt mã NGẮN từ bộ đếm không bao giờ tạo
ra hai mã TRÙNG nhau — dù sinh BAO nhiêu mã đi nữa:

```typescript title=readonly
const BANG_BASE62 = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

function maHoaBase62(n: number): string {
  if (n === 0) return "0";
  let ketQua = "";
  let con = n;
  while (con > 0) {
    ketQua = BANG_BASE62[con % 62]! + ketQua;
    con = Math.floor(con / 62);
  }
  return ketQua;
}

interface BoDemToanCuc { giaTriHienTai: number; }
function taoBoDemToanCuc(): BoDemToanCuc {
  return { giaTriHienTai: 0 };
}
function laMaNganTiepTheo(bo: BoDemToanCuc): string {
  const ma = maHoaBase62(bo.giaTriHienTai);
  bo.giaTriHienTai += 1;
  return ma;
}

const bo = taoBoDemToanCuc();
const tatCaMa: string[] = [];
for (let i = 0; i < 200; i++) tatCaMa.push(laMaNganTiepTheo(bo));

const tapHopMa = new Set(tatCaMa);
console.log("so ma da sinh:", tatCaMa.length);
console.log("so ma DUY NHAT (qua Set):", tapHopMa.size);
console.log("khong co trung lap:", tatCaMa.length === tapHopMa.size);
console.log("5 ma dau:", tatCaMa.slice(0, 5).join(","));
console.log("5 ma cuoi:", tatCaMa.slice(-5).join(","));
```

```text title=readonly
so ma da sinh: 200
so ma DUY NHAT (qua Set): 200
khong co trung lap: true
5 ma dau: 0,1,2,3,4
5 ma cuoi: 39,3a,3b,3c,3d
```

`200` mã sinh RA, `200` mã DUY nhất trong `Set` — KHỚP tuyệt đối.
Đây LÀ điểm khác biệt LỚN nhất so VỚI cách sinh mã ngẫu nhiên (thử
random RỒI kiểm tra có trùng không): bộ đếm không CẦN bước kiểm tra
ĐÓ, vì bản chất toán học của phép mã hoá đã LOẠI trừ khả năng trùng.
::::

::::predict{#doan-tinh-duy-nhat commitOnce}
Bộ đếm toàn cục KHÔNG bao giờ được đặt LẠI (reset) giữa các lần gọi
— `giaTriHienTai` chỉ tăng, không bao giờ giảm hay quay VỀ `0`. Trong
một chương trình chạy TUẦN tự (không có nhiều luồng xử lý ĐỒNG thời),
có tình huống NÀO khiến `laMaNganTiepTheo` trả về hai mã NGẮN trùng
nhau không?

:::opt{correct}
KHÔNG — mỗi lần gọi luôn dùng một `giaTriHienTai` LỚN hơn mọi giá trị
đã dùng trước đó, VÀ `maHoaBase62` ánh xạ mỗi số nguyên khác nhau
sang một chuỗi khác nhau
:::
:::opt
CÓ thể — nếu chương trình chạy đủ LÂU, số nguyên `giaTriHienTai` sẽ
lớn tới mức "tràn số" VÀ quay vòng về những giá trị nhỏ đã cấp trước
đó
::why
Nhầm "số nguyên có giới hạn TRÊN lý thuyết" VỚI "chương trình NÀY
thực sự chạm tới giới hạn đó" — nhưng bài học không hề mô phỏng tình
huống tràn số, VÀ `giaTriHienTai += 1` không hề có logic quay VÒNG
nào cả.

Chỗ lệch: `laMaNganTiepTheo` chỉ CÓ hai dòng — mã hoá giá trị HIỆN
tại, rồi CỘNG thêm `1`. Không có bước NÀO kiểm tra tràn số hay đặt
lại VỀ `0`. Trong phạm vi bài học (sinh vài trăm mã), `giaTriHienTai`
luôn LÀ một dãy số nguyên tăng dần TUYỆT đối, không hề lặp LẠI.
::
:::
::::

::::code{#viet_la_ma_ngan_tiep_theo}
Hoàn thiện `laMaNganTiepTheo` — SAU khi đã mã hoá giá trị hiện tại
của bộ đếm (dòng TRÊN), tăng bộ đếm LÊN đúng `1` trước khi trả kết
quả về.

```typescript title=starter
const BANG_BASE62 = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

function maHoaBase62(n: number): string {
  if (n === 0) return "0";
  let ketQua = "";
  let con = n;
  while (con > 0) {
    ketQua = BANG_BASE62[con % 62]! + ketQua;
    con = Math.floor(con / 62);
  }
  return ketQua;
}

interface BoDemToanCuc { giaTriHienTai: number; }
function taoBoDemToanCuc(): BoDemToanCuc {
  return { giaTriHienTai: 0 };
}
function laMaNganTiepTheo(bo: BoDemToanCuc): string {
  const ma = maHoaBase62(bo.giaTriHienTai);
  ___
  return ma;
}

const bo = taoBoDemToanCuc();
console.log(laMaNganTiepTheo(bo), laMaNganTiepTheo(bo));
```

```typescript title=solution
const BANG_BASE62 = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

function maHoaBase62(n: number): string {
  if (n === 0) return "0";
  let ketQua = "";
  let con = n;
  while (con > 0) {
    ketQua = BANG_BASE62[con % 62]! + ketQua;
    con = Math.floor(con / 62);
  }
  return ketQua;
}

interface BoDemToanCuc { giaTriHienTai: number; }
function taoBoDemToanCuc(): BoDemToanCuc {
  return { giaTriHienTai: 0 };
}
function laMaNganTiepTheo(bo: BoDemToanCuc): string {
  const ma = maHoaBase62(bo.giaTriHienTai);
  bo.giaTriHienTai += 1;
  return ma;
}

const bo = taoBoDemToanCuc();
console.log(laMaNganTiepTheo(bo), laMaNganTiepTheo(bo));
```

```typescript title=test
const boT = taoBoDemToanCuc();
const nam = [laMaNganTiepTheo(boT), laMaNganTiepTheo(boT), laMaNganTiepTheo(boT), laMaNganTiepTheo(boT), laMaNganTiepTheo(boT)];
if (nam.join(",") !== "0,1,2,3,4") throw new Error("5 lan goi dau tien phai la 0,1,2,3,4 theo dung thu tu counter");
if (boT.giaTriHienTai !== 5) throw new Error("sau 5 lan cap ma, giaTriHienTai phai la 5");

const boT2 = taoBoDemToanCuc();
const nhieuMa: string[] = [];
for (let i = 0; i < 500; i++) nhieuMa.push(laMaNganTiepTheo(boT2));
if (new Set(nhieuMa).size !== 500) throw new Error("500 ma sinh tu counter phai HOAN TOAN khong trung lap");
if (boT2.giaTriHienTai !== 500) throw new Error("sau 500 lan cap ma, giaTriHienTai phai la 500");
```

:::hints
- kind: attention
  body: "Dong tiep theo phai LAM THAY DOI giaTriHienTai cua bo dem, khong dong cham gi den bien ma."
- kind: strategy
  body: "Tang giaTriHienTai them dung 1 don vi: bo.giaTriHienTai += 1;"
- kind: one-line
  body: "bo.giaTriHienTai += 1;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "0 1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Mã ngắn giờ không bao giờ trùng — nhưng một dịch vụ rút gọn URL thật
còn cần cho phép người dùng CHỌN alias riêng, thay VÌ nhận mã tự
sinh. VÀ đó LÀ nơi va chạm THẬT sự có thể xảy ra.
::::

::::reflect{#nghi-lai}
`laMaNganTiepTheo` không CẦN kiểm tra trùng lặp vì nó KHÔNG hề đoán
mò — nó luôn biết CHÍNH XÁC giá trị tiếp theo LÀ gì. Đây LÀ lý do
counter-based ID (VÀ Snowflake Ở quest trước) tránh được cả một lớp
lỗi mà cách sinh ngẫu nhiên buộc phải xử LÝ riêng: kiểm tra VÀ thử
lại khi trùng.
::::

::::checkpoint{mastery=0.69}
::::
