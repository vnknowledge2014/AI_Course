---
id: ky-nghe-phan-mem.van-hanh.ci-la-gi-tu-dong-hoa-kiem-tra
title: "CI là gì — tự động hoá kiểm tra, con người QUÊN máy KHÔNG quên"
summary: "Continuous Integration (CI): MỖI lần đẩy code LÊN, MÁY tự động chạy một chuỗi kiểm tra (typecheck, lint, test, build) — KHÔNG đợi người NHỚ chạy tay. Con người LÀM cùng năm bước này hàng chục lần một ngày — bỏ sót MỘT lần là bug bò ra production. CI không phải \"chạy test\" — nó LÀ \"chạy test tự động, KHÔNG phụ thuộc người có nhớ hay không\"."
locale: vi
track: ky-nghe-phan-mem
module: van-hanh
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [vh.what-is-ci]
requires: [mau.gate-boss-cqrs-es]
concepts: [vh.what-is-ci]
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
Track mới. Trước khi đẩy code LÊN, cần chạy: kiểm KIỂU, kiểm ĐỊNH
DẠNG, chạy TEST, đóng GÓI. Bốn việc, mười lần một ngày — ai NHỚ nổi?
::::

::::explain{#ci-la-gi}
**Continuous Integration (CI)**: MỖI LẦN code được đẩy LÊN, MÁY tự
động chạy MỘT chuỗi kiểm tra (typecheck, lint, test, build) — KHÔNG
đợi CON NGƯỜI nhớ chạy TAY. Đây KHÔNG PHẢI "có chạy test" (bài học
Ở T5.2) — nó LÀ "chạy test **TỰ ĐỘNG**, KHÔNG PHỤ THUỘC người CÓ NHỚ
hay không":

```typescript title=readonly
type KetQuaKiemTra = { ten: string; qua: boolean };

function kiemTraThuCong(daNho: boolean): KetQuaKiemTra[] {
  if (!daNho) return [];
  return [{ ten: "typecheck", qua: true }];
}

function kiemTraTuDong(): KetQuaKiemTra[] {
  return [{ ten: "typecheck", qua: true }];
}

console.log(kiemTraThuCong(false).length);
console.log(kiemTraThuCong(true).length);
console.log(kiemTraTuDong().length);
```

```text title=readonly
0
1
1
```

`kiemTraThuCong` NHẬN một tham số `daNho` (MÔ PHỎNG việc con người
CÓ nhớ chạy kiểm tra hay KHÔNG) — QUÊN (`false`) thì KHÔNG kiểm tra
GÌ cả (mảng RỖNG). `kiemTraTuDong` KHÔNG nhận tham số NÀO liên quan
tới "nhớ" — nó LUÔN chạy, BẤT KỂ điều gì.
::::

::::example{#mot-tuan-quen-vai-lan}
Con người LÀM việc NÀY hàng chục lần MỘT NGÀY — chỉ CẦN quên VÀI
lần TRONG một tuần, số lần kiểm tra THẬT SỰ chạy đã ÍT HƠN HẲN số
lần LẼ RA phải chạy:

```typescript title=readonly
const lichSuMotTuan: boolean[] = [true, true, false, true, false, true, true];
// true = nho chay kiem tra, false = quen

const soLanChayThuCong = lichSuMotTuan.filter((daNho) => daNho).length;
const soLanChayTuDong = lichSuMotTuan.length; // LUON chay, khong phu thuoc "nho"

console.log(soLanChayThuCong);
console.log(soLanChayTuDong);
```

```text title=readonly
5
7
```

BẢY lần đẩy code TRONG tuần, NHƯNG chỉ NĂM lần con người THỰC SỰ
nhớ chạy kiểm tra THỦ CÔNG — HAI lần "lọt lưới" ĐÓ CHÍNH LÀ hai cơ
hội bug BÒ RA production MÀ KHÔNG AI hay biết. CI (tự động) LUÔN
chạy ĐỦ bảy lần — KHÔNG có khái niệm "lọt lưới" Ở ĐÂY.
::::

::::predict{#doan-tuan-quen-het commitOnce}
```typescript
type KetQuaKiemTra = { ten: string; qua: boolean };
function kiemTraThuCong(daNho: boolean): KetQuaKiemTra[] {
  if (!daNho) return [];
  return [{ ten: "typecheck", qua: true }];
}
function kiemTraTuDong(): KetQuaKiemTra[] {
  return [{ ten: "typecheck", qua: true }];
}

const motTuanQuenHet = [false, false, false, false, false];
const tongThuCong = motTuanQuenHet.map((daNho) => kiemTraThuCong(daNho).length).reduce((t, n) => t + n, 0);
const tongTuDong = motTuanQuenHet.map(() => kiemTraTuDong().length).reduce((t, n) => t + n, 0);
console.log(tongThuCong);
console.log(tongTuDong);
```

`motTuanQuenHet` LÀ một tuần XẤU NHẤT — con người QUÊN CẢ NĂM ngày.
Hai dòng cuối in ra gì?

:::opt{correct}
`0` rồi `5`
:::

:::opt
`0` rồi `0` — vì `kiemTraTuDong` được gọi BÊN TRONG `.map(() =>
...)`, VÀ `.map` ĐANG LẶP qua `motTuanQuenHet` (mảng TOÀN `false`),
nên MỌI lời gọi BÊN TRONG vòng lặp ĐÓ ĐỀU "thừa hưởng" giá trị
`false` từ MẢNG, KỂ CẢ khi hàm KHÔNG nhận tham số nào
::why
Gần đúng ở việc bạn nhớ ĐÚNG `kiemTraTuDong` được gọi TỪ BÊN TRONG
`.map` đang lặp qua `motTuanQuenHet` — một quan sát ĐÚNG về VỊ TRÍ
lời gọi.

Chỗ lệch: `() => kiemTraTuDong().length` LÀ một hàm mũi tên **KHÔNG
NHẬN THAM SỐ NÀO** — dấu ngoặc `()` RỖNG nghĩa LÀ nó **HOÀN TOÀN
KHÔNG ĐỌC** phần tử ĐANG được `.map` duyệt qua (KHÁC `(daNho) =>
kiemTraThuCong(daNho).length`, CÓ đọc phần tử). `kiemTraTuDong()`
KHÔNG có tham số NÀO để "thừa hưởng" `false` — nó LUÔN trả về mảng
MỘT phần tử, DÙ được gọi Ở ĐÂU, BAO NHIÊU lần. Năm lần gọi ĐỘC LẬP →
tổng `5`.
::
:::

:::opt
Máy báo lỗi biên dịch — `.map(() => kiemTraTuDong().length)` KHÔNG
sử dụng tham số của hàm mũi tên (bỏ TRỐNG dấu ngoặc), TypeScript CẤM
gọi `.map` với một hàm KHÔNG đọc phần tử ĐANG duyệt
::why
Gần đúng ở việc bạn để ý hàm mũi tên `() => ...` KHÔNG khai tham số
NÀO — một quan sát ĐÚNG về CÚ PHÁP.

Chỗ lệch: `.map` CHỈ đòi hàm callback trả về MỘT giá trị (kiểu BẤT
KỲ) — nó HOÀN TOÀN KHÔNG bắt buộc callback phải ĐỌC phần tử ĐANG
duyệt. Bỏ qua tham số (dùng `()`) LÀ cú pháp HỢP LỆ, PHỔ BIẾN khi
"chạy MỘT việc CỐ ĐỊNH đúng SỐ LẦN bằng độ dài mảng" — CHÍNH XÁC
tình huống Ở ĐÂY. Biên dịch SẠCH.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
CI: TỰ ĐỘNG, KHÔNG phụ thuộc trí nhớ con người. Bước tiếp theo: xâu
chuỗi NHIỀU bước kiểm tra thành MỘT "pipeline".
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Thực tế CI KHÔNG CHỈ chạy MỘT bước (`typecheck`) — nó chạy MỘT
**CHUỖI** bước (typecheck → lint → test → build). Nếu bước ĐẦU
TIÊN ĐÃ hỏng, CÓ NÊN chạy TIẾP các bước SAU không?
::::

::::checkpoint{mastery=0.8}
::::
