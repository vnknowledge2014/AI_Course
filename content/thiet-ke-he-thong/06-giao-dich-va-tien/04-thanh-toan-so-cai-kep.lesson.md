---
id: thiet-ke-he-thong.giao-dich-va-tien.thanh-toan-so-cai-kep
title: "Sổ cái kép: mỗi đồng ghi nợ có một đồng ghi có"
summary: "ghiGiaoDich(sc, taiKhoanNo, taiKhoanCo, soTien) ghi DUNG hai but toan doi ung (mot no, mot co, cung so tien); tongSoCai(sc) cong no la SO AM, co la SO DUONG -- sau giao dich an->quan-an 50000 VA binh->quan-an 75000, tong so cai VAN la 0; kiemTraBatBienSoCai(sc) tra ve true. Mo phong loi du lieu (ghi mot but toan NO le, khong co doi ung 1000) lam tong lech thanh -1000, bat bien tra ve false -- ham phat hien DUNG loi mat can doi."
locale: vi
track: thiet-ke-he-thong
module: giao-dich-va-tien
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [sd.thanh-toan-so-cai-kep]
requires: [sd.thanh-toan-idempotency-key]
concepts: [sd.thanh-toan-so-cai-kep]
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
Khoá idempotency ngăn được việc xử lý một giao dịch HAI lần. Nhưng còn
câu hỏi khác: làm sao BIẾT chắc tiền không tự nhiên SINH ra hay MẤT đi
Ở đâu đó giữa hàng nghìn giao dịch? Kế toán giải quyết việc NÀY từ lâu
trước khi có máy tính — sổ cái KÉP: không đồng nào di chuyển một
mình, nó luôn đi kèm một bút toán ĐỐI ứng.
::::

::::explain{#moi-giao-dich-hai-but-toan}
`ghiGiaoDich` không ghi MỘT dòng "chuyển X đồng" — nó ghi ĐÚNG hai bút
toán: một bút toán `"no"` (ghi nợ) cho tài khoản NGUỒN, một bút toán
`"co"` (ghi có) cho tài khoản ĐÍCH, cùng SỐ tiền. `tongSoCai` cộng nợ
LÀ số ÂM, có LÀ số DƯƠNG — một giao dịch cân bằng luôn đóng góp đúng
`0` vào tổng:

```typescript title=readonly
type LoaiButToan = "no" | "co";
interface ButToan { taiKhoan: string; loai: LoaiButToan; soTien: number; }
interface SoCai { cacButToan: ButToan[]; }
function taoSoCai(): SoCai { return { cacButToan: [] }; }

function ghiGiaoDich(sc: SoCai, taiKhoanNo: string, taiKhoanCo: string, soTien: number): void {
  sc.cacButToan.push({ taiKhoan: taiKhoanNo, loai: "no", soTien });
  sc.cacButToan.push({ taiKhoan: taiKhoanCo, loai: "co", soTien });
}

function tongSoCai(sc: SoCai): number {
  return sc.cacButToan.reduce((tong, bt) => tong + (bt.loai === "no" ? -bt.soTien : bt.soTien), 0);
}

function kiemTraBatBienSoCai(sc: SoCai): boolean {
  return tongSoCai(sc) === 0;
}

const sc = taoSoCai();
ghiGiaoDich(sc, "vi-an", "vi-quan-an", 50000);
console.log("sau giao dich 1, tong so cai:", tongSoCai(sc));
console.log("bat bien giu vung?", kiemTraBatBienSoCai(sc));

ghiGiaoDich(sc, "vi-binh", "vi-quan-an", 75000);
console.log("sau giao dich 2, tong so cai:", tongSoCai(sc));
console.log("so but toan (2 giao dich x 2 but toan):", sc.cacButToan.length);
```

```text title=readonly
sau giao dich 1, tong so cai: 0
bat bien giu vung? true
sau giao dich 2, tong so cai: 0
so but toan (2 giao dich x 2 but toan): 4
```

Mỗi lần `ghiGiaoDich` chạy, `cacButToan` dài thêm ĐÚNG hai phần tử —
sau `2` giao dịch, có `4` bút toán. Dù `"vi-an"` VÀ `"vi-binh"` chuyển
số tiền KHÁC nhau (`50000` VÀ `75000`) cho CÙNG một tài khoản
(`"vi-quan-an"`), tổng sổ cái vẫn LÀ `0` sau cả hai — mỗi bút toán nợ
luôn có đúng MỘT bút toán có đối ứng CÙNG giá trị.
::::

::::example{#phat-hien-mat-can-doi}
Bất biến chỉ thật sự có ích khi nó PHÁT hiện được lúc sổ cái mất cân
đối — mô phỏng một lỗi dữ liệu (ghi bút toán nợ nhưng QUÊN bút toán có
đối ứng) để xem `kiemTraBatBienSoCai` phản ứng thế nào:

```typescript title=readonly
type LoaiButToan = "no" | "co";
interface ButToan { taiKhoan: string; loai: LoaiButToan; soTien: number; }
interface SoCai { cacButToan: ButToan[]; }
function taoSoCai(): SoCai { return { cacButToan: [] }; }

function ghiGiaoDich(sc: SoCai, taiKhoanNo: string, taiKhoanCo: string, soTien: number): void {
  sc.cacButToan.push({ taiKhoan: taiKhoanNo, loai: "no", soTien });
  sc.cacButToan.push({ taiKhoan: taiKhoanCo, loai: "co", soTien });
}

function tongSoCai(sc: SoCai): number {
  return sc.cacButToan.reduce((tong, bt) => tong + (bt.loai === "no" ? -bt.soTien : bt.soTien), 0);
}

function kiemTraBatBienSoCai(sc: SoCai): boolean {
  return tongSoCai(sc) === 0;
}

// tai lap dung trang thai tu khoi truoc, KHONG in lai: 2 giao dich da ghi
// (an->quan-an 50000, binh->quan-an 75000), tong so cai dang la 0
const sc = taoSoCai();
ghiGiaoDich(sc, "vi-an", "vi-quan-an", 50000);
ghiGiaoDich(sc, "vi-binh", "vi-quan-an", 75000);

ghiGiaoDich(sc, "vi-chi", "vi-quan-an", 30000);
console.log("sau giao dich 3 (chi), tong so cai:", tongSoCai(sc));
console.log("bat bien giu vung?", kiemTraBatBienSoCai(sc));

// BUG mo phong: ghi NO ma QUEN ghi CO tuong ung (loi du lieu that su)
sc.cacButToan.push({ taiKhoan: "vi-loi", loai: "no", soTien: 1000 });
console.log("tong so cai SAU loi du lieu (thieu but toan doi ung):", tongSoCai(sc));
console.log("bat bien con giu vung?", kiemTraBatBienSoCai(sc));
```

```text title=readonly
sau giao dich 3 (chi), tong so cai: 0
bat bien giu vung? true
tong so cai SAU loi du lieu (thieu but toan doi ung): -1000
bat bien con giu vung? false
```

Bút toán nợ `1000` bị thêm ĐƠN độc — không có bút toán có ĐỐI ứng —
ngay lập tức kéo `tongSoCai` xuống `-1000`, VÀ `kiemTraBatBienSoCai`
đổi từ `true` sang `false`. Đây chính LÀ giá trị của bất biến: nó
không cần biết bút toán NÀO sai, chỉ cần một con số DUY nhất lệch khỏi
`0` LÀ đủ để báo động.
::::

::::predict{#doan-sua-mat-can-doi commitOnce}
Ngay SAU đoạn trên, sổ cái đang mất cân đối (`tongSoCai` LÀ `-1000`).
Ai đó thêm MỘT bút toán `"co"` `1000` vào một tài khoản BẤT kỳ, ví dụ
`"vi-sua"` — nhưng KHÔNG thông qua `ghiGiaoDich`, chỉ `push` trực
tiếp. Gọi `kiemTraBatBienSoCai(sc)` NGAY sau đó — kết quả LÀ gì?

:::opt{correct}
`true` — `kiemTraBatBienSoCai` CHỈ nhìn vào tổng của TOÀN bộ sổ cái;
bút toán có `1000` mới cộng NGƯỢC lại đúng phần bị lệch, tổng trở về
`0`, dù bút toán ĐÓ hoàn toàn không thuộc VỀ cùng một giao dịch với bút
toán nợ gây lỗi ban đầu
:::
:::opt
`false` — bút toán có MỚI không hề "sửa" bút toán nợ SAI kia, chúng
vẫn LÀ hai bút toán rời rạc không thuộc cùng giao dịch, nên bất biến
phải tiếp tục báo lỗi
::why
Nhầm "bất biến kiểm tra TỪNG giao dịch có đúng cặp hay không" VỚI
"bất biến kiểm tra TỔNG toàn hệ thống có bằng 0 hay không" — đây LÀ
hai thứ khác nhau, VÀ `kiemTraBatBienSoCai` chỉ làm việc THỨ hai.

Chỗ lệch: `tongSoCai` cộng dồn TẤT cả bút toán, không quan tâm bút
toán nào "thuộc về" giao dịch nào. `-1000` (từ bút toán nợ lẻ) cộng
`+1000` (từ bút toán có mới) RA đúng `0` — VỀ mặt con số, bất biến
được thoả. Đây chính LÀ giới hạn quan trọng của kiểu kiểm tra tổng
toàn cục: nó phát hiện được sổ cái CÓ lệch, nhưng không tự nó xác định
được lệch đó nằm Ở đâu, hay đã được "sửa" đúng chỗ hay chưa.
::
:::
::::

::::code{#viet_tong_so_cai}
Hoàn thiện `tongSoCai` — duyệt qua từng bút toán trong
`sc.cacButToan`, cộng dồn: bút toán `"no"` trừ ĐI `soTien`, bút toán
`"co"` cộng THÊM `soTien`. `kiemTraBatBienSoCai` (đã có sẵn) sẽ dùng
kết quả NÀY để xác nhận bất biến kế toán.

```typescript title=starter
type LoaiButToan = "no" | "co";
interface ButToan { taiKhoan: string; loai: LoaiButToan; soTien: number; }
interface SoCai { cacButToan: ButToan[]; }
function taoSoCai(): SoCai { return { cacButToan: [] }; }

function ghiGiaoDich(sc: SoCai, taiKhoanNo: string, taiKhoanCo: string, soTien: number): void {
  sc.cacButToan.push({ taiKhoan: taiKhoanNo, loai: "no", soTien });
  sc.cacButToan.push({ taiKhoan: taiKhoanCo, loai: "co", soTien });
}

function tongSoCai(sc: SoCai): number {
  ___
}

function kiemTraBatBienSoCai(sc: SoCai): boolean {
  return tongSoCai(sc) === 0;
}

const scX = taoSoCai();
ghiGiaoDich(scX, "a", "b", 999);
console.log(tongSoCai(scX), kiemTraBatBienSoCai(scX));
```

```typescript title=solution
type LoaiButToan = "no" | "co";
interface ButToan { taiKhoan: string; loai: LoaiButToan; soTien: number; }
interface SoCai { cacButToan: ButToan[]; }
function taoSoCai(): SoCai { return { cacButToan: [] }; }

function ghiGiaoDich(sc: SoCai, taiKhoanNo: string, taiKhoanCo: string, soTien: number): void {
  sc.cacButToan.push({ taiKhoan: taiKhoanNo, loai: "no", soTien });
  sc.cacButToan.push({ taiKhoan: taiKhoanCo, loai: "co", soTien });
}

function tongSoCai(sc: SoCai): number {
  let tong = 0;
  for (const bt of sc.cacButToan) {
    tong += bt.loai === "no" ? -bt.soTien : bt.soTien;
  }
  return tong;
}

function kiemTraBatBienSoCai(sc: SoCai): boolean {
  return tongSoCai(sc) === 0;
}

const scX = taoSoCai();
ghiGiaoDich(scX, "a", "b", 999);
console.log(tongSoCai(scX), kiemTraBatBienSoCai(scX));
```

```typescript title=test
const scT = taoSoCai();
if (tongSoCai(scT) !== 0) throw new Error("so cai RONG phai co tong bang 0");
if (!kiemTraBatBienSoCai(scT)) throw new Error("so cai RONG phai thoa bat bien");

ghiGiaoDich(scT, "x", "y", 100);
if (tongSoCai(scT) !== 0) throw new Error("MOT giao dich hop le (no+co bang nhau) phai giu tong bang 0");

ghiGiaoDich(scT, "y", "z", 250);
ghiGiaoDich(scT, "z", "x", 40);
if (tongSoCai(scT) !== 0) throw new Error("NHIEU giao dich hop le lien tiep van phai giu tong bang 0");
if (!kiemTraBatBienSoCai(scT)) throw new Error("bat bien phai DUNG sau nhieu giao dich hop le");

const soButToanTruoc = scT.cacButToan.length;
if (soButToanTruoc !== 6) throw new Error("3 giao dich x 2 but toan = 6 but toan");

scT.cacButToan.push({ taiKhoan: "loi", loai: "no", soTien: 500 });
if (tongSoCai(scT) !== -500) throw new Error("mot but toan LE (khong co doi ung) phai lam tong lech di dung bang gia tri cua no");
if (kiemTraBatBienSoCai(scT)) throw new Error("bat bien phai SAI khi so cai mat can doi");

scT.cacButToan.push({ taiKhoan: "sua", loai: "co", soTien: 500 });
if (tongSoCai(scT) !== 0) throw new Error("them dung but toan doi ung phai dua tong VE lai 0");
if (!kiemTraBatBienSoCai(scT)) throw new Error("bat bien phai DUNG lai sau khi can bang");
```

:::hints
- kind: attention
  body: "Duyet sc.cacButToan bang vong lap hoac reduce, cong don: neu bt.loai === 'no' thi TRU soTien, neu 'co' thi CONG soTien. Tra ve tong cuoi cung."
- kind: strategy
  body: "let tong = 0; for (const bt of sc.cacButToan) { tong += bt.loai === 'no' ? -bt.soTien : bt.soTien; } return tong;"
- kind: one-line
  body: "return sc.cacButToan.reduce((tong, bt) => tong + (bt.loai === \"no\" ? -bt.soTien : bt.soTien), 0);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "0 true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Idempotency key ngăn xử lý trùng, sổ cái kép chứng minh không đồng
nào biến mất giữa đường — hai mảnh của "Payment System" đã xong. Giờ
chuyển sang một góc nhìn khác: số dư của MỘT tài khoản, tại một thời
điểm.
::::

::::reflect{#nghi-lai}
`tongSoCai` không hề biết gì VỀ ý nghĩa của từng tài khoản — nó chỉ
cộng dồn một quy ước đơn giản: nợ trừ, có cộng. Chính sự ĐƠN giản đó
LÀ sức mạnh của bất biến kế toán: một con số DUY nhất, tính được sau
MỌI giao dịch, phải LUÔN bằng `0`. Nhưng bài dự đoán vừa rồi cũng chỉ
RA giới hạn của nó — bất biến TOÀN cục phát hiện được "có gì đó sai",
không tự nó chỉ RA "sai Ở đâu, sai vì giao dịch nào".
::::

::::checkpoint{mastery=0.72}
::::
