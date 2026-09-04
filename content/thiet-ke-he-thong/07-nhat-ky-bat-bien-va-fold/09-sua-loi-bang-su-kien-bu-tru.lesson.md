---
id: thiet-ke-he-thong.nhat-ky-bat-bien-va-fold.sua-loi-bang-su-kien-bu-tru
title: "Sửa lỗi bằng sự kiện bù trừ: log chỉ dài ra, không viết đè"
summary: "hoanTien(nhatKy, idSuKienSai, idSuKienMoi) THEM mot su kien MOI loai da_hoan_tien vao cuoi log de bu tru cho mot su kien sai, KHONG BAO GIO tim va sua truc tiep su kien cu -- vi du tru NHAM 30000 (e2), hoanTien them e3 (da_hoan_tien 30000), so du tro ve DUNG 100000, VA e2 (sai) van con NGUYEN trong log de tra soat sau nay."
locale: vi
track: thiet-ke-he-thong
module: nhat-ky-bat-bien-va-fold
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [sd.fp.sua-loi-bang-su-kien-bu-tru]
requires: [sd.fp.tai-tao-tu-dau]
concepts: [sd.fp.sua-loi-bang-su-kien-bu-tru]
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
Bài 1 đặt luật đầu tiên: sự kiện KHÔNG BAO GIỜ bị sửa hay xoá sau khi
ghi. Nhưng hệ thống thực tế CÓ lỗi — một giao dịch trừ nhầm số tiền,
một phép tính sai Ở lõi quyết định trước khi được vá. Nếu không được
sửa, làm sao "sửa lỗi" theo đúng luật của chính track này?
::::

::::explain{#bu-tru-khong-viet-de}
"Sửa lỗi" trong một hệ thống nhật ký bất biến KHÔNG PHẢI đi tìm sự
kiện sai rồi đổi nội dung của nó — mà LÀ ghi THÊM một sự kiện bù trừ
MỚI (compensating event). `hoanTien` tìm sự kiện sai theo `id`, rồi
append một sự kiện `"da_hoan_tien"` với đúng số tiền của nó — sự kiện
GỐC vẫn còn nguyên, log chỉ dài THÊM:

```typescript title=readonly
type LoaiSuKien = "da_nap" | "da_tru" | "da_hoan_tien";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; idSuKienGoc?: string; }
interface TrangThai { soDu: number; }
function trangThaiBanDau(): TrangThai { return { soDu: 0 }; }
function apDung(trangThai: TrangThai, suKien: SuKien): TrangThai {
  if (suKien.loai === "da_tru") return { soDu: trangThai.soDu - suKien.soTien };
  return { soDu: trangThai.soDu + suKien.soTien };
}
function tinhTrangThai(nhatKy: SuKien[]): TrangThai {
  return nhatKy.reduce(apDung, trangThaiBanDau());
}

function hoanTien(nhatKy: SuKien[], idSuKienSai: string, idSuKienMoi: string): SuKien[] {
  const suKienSai = nhatKy.find((sk) => sk.id === idSuKienSai);
  if (suKienSai === undefined) return nhatKy;
  const suKienBu: SuKien = { id: idSuKienMoi, loai: "da_hoan_tien", soTien: suKienSai.soTien, idSuKienGoc: idSuKienSai };
  return [...nhatKy, suKienBu];
}

const nhatKy: SuKien[] = [
  { id: "e1", loai: "da_nap", soTien: 100000 },
  { id: "e2", loai: "da_tru", soTien: 30000 },
];
console.log("so du SAU khi tru NHAM:", tinhTrangThai(nhatKy).soDu);

const daSua = hoanTien(nhatKy, "e2", "e3");
console.log("so su kien SAU khi sua (them, KHONG xoa):", daSua.length);
console.log("so du SAU khi hoan tien:", tinhTrangThai(daSua).soDu);

const e2ConDo = daSua.find((sk) => sk.id === "e2");
console.log("su kien e2 (loi) van con trong log?", e2ConDo !== undefined);
console.log("noi dung e2 co bi thay doi khong (van la da_tru 30000)?", JSON.stringify(e2ConDo));
```

```text title=readonly
so du SAU khi tru NHAM: 70000
so su kien SAU khi sua (them, KHONG xoa): 3
so du SAU khi hoan tien: 100000
su kien e2 (loi) van con trong log? true
noi dung e2 co bi thay doi khong (van la da_tru 30000)? {"id":"e2","loai":"da_tru","soTien":30000}
```

Sau khi `e2` (trừ nhầm `30000`) khiến số dư còn `70000`, `hoanTien`
KHÔNG hề tìm `e2` để sửa `soTien` của nó thành `0`. Nó thêm `e3`
(`"da_hoan_tien"`, `30000`) — log giờ có `3` phần tử thay vì `2`. Fold
lại: `70000 + 30000 = 100000`, đúng số dư TRƯỚC khi lỗi xảy ra. Và
`e2` — tra lại bằng `.find` — vẫn còn NGUYÊN VẸN, `{"id":"e2",
"loai":"da_tru","soTien":30000}`, y hệt lúc ghi lần đầu.
::::

::::example{#hoan-tien-an-toan-va-truy-vet-duoc}
`hoanTien` không hề sửa mảng `nhatKy` truyền vào (mảng gốc giữ nguyên
độ dài), VÀ tự động bỏ qua nếu `id` cần bù trừ không tồn tại — không
tạo ra một sự kiện bù trừ "ma" cho một thứ chưa từng xảy ra:

```typescript title=readonly
type LoaiSuKien = "da_nap" | "da_tru" | "da_hoan_tien";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; idSuKienGoc?: string; }
interface TrangThai { soDu: number; }
function trangThaiBanDau(): TrangThai { return { soDu: 0 }; }
function apDung(trangThai: TrangThai, suKien: SuKien): TrangThai {
  if (suKien.loai === "da_tru") return { soDu: trangThai.soDu - suKien.soTien };
  return { soDu: trangThai.soDu + suKien.soTien };
}
function tinhTrangThai(nhatKy: SuKien[]): TrangThai {
  return nhatKy.reduce(apDung, trangThaiBanDau());
}
function hoanTien(nhatKy: SuKien[], idSuKienSai: string, idSuKienMoi: string): SuKien[] {
  const suKienSai = nhatKy.find((sk) => sk.id === idSuKienSai);
  if (suKienSai === undefined) return nhatKy;
  const suKienBu: SuKien = { id: idSuKienMoi, loai: "da_hoan_tien", soTien: suKienSai.soTien, idSuKienGoc: idSuKienSai };
  return [...nhatKy, suKienBu];
}

const nhatKyGoc: SuKien[] = [
  { id: "e1", loai: "da_nap", soTien: 50000 },
  { id: "e2", loai: "da_tru", soTien: 15000 },
];

const sauSua = hoanTien(nhatKyGoc, "e2", "e3");
console.log("nhatKyGoc SAU khi goi hoanTien van co", nhatKyGoc.length, "su kien (KHONG doi)");
console.log("sauSua co", sauSua.length, "su kien");

const khongDoi = hoanTien(sauSua, "id-khong-ton-tai", "e4");
console.log("hoan tien id KHONG ton tai - log co doi khong?", khongDoi.length === sauSua.length);
console.log("khongDoi va sauSua cung do dai:", khongDoi.length);

const suKienBu = sauSua.find((sk) => sk.id === "e3");
console.log("su kien bu tru tro ve dung su kien goc?", suKienBu?.idSuKienGoc === "e2");
```

```text title=readonly
nhatKyGoc SAU khi goi hoanTien van co 2 su kien (KHONG doi)
sauSua co 3 su kien
hoan tien id KHONG ton tai - log co doi khong? true
khongDoi va sauSua cung do dai: 3
su kien bu tru tro ve dung su kien goc? true
```

`nhatKyGoc` không hề bị đụng tới sau khi `hoanTien` được gọi trên nó —
`sauSua` là một mảng HOÀN TOÀN mới. Gọi `hoanTien` với một `id` không
tồn tại (`"id-khong-ton-tai"`) trả về NGUYÊN `sauSua`, không thêm gì —
`khongDoi.length === sauSua.length` là `true`. Và `e3` (sự kiện bù
trừ) mang theo `idSuKienGoc: "e2"` — dấu vết để biết nó bù cho sự
kiện nào, phục vụ tra soát sau này.
::::

::::predict{#doan-cach-sua-dung commitOnce}
Một khách hàng phản ánh: sự kiện `e2` (`"da_tru"`, `15000`) ghi SAI —
đáng lẽ chỉ phải trừ `10000`. Theo đúng mô hình nhật ký bất biến, cách
SỬA đúng là gì?

:::opt{correct}
Ghi THÊM một sự kiện bù trừ (ví dụ hoàn lại phần chênh lệch `5000`,
hoặc hoàn TOÀN BỘ `15000` rồi ghi lại đúng `10000` bằng một sự kiện
`"da_tru"` mới) — KHÔNG BAO GIỜ tìm `e2` trong mảng rồi gán đè
`e2.soTien = 10000`
:::
:::opt
Tìm `e2` bằng `.find()`, rồi gán trực tiếp `e2.soTien = 10000` — sửa
xong tại chỗ, nhật ký giờ phản ánh đúng số tiền đáng lẽ phải trừ
::why
Vi phạm chính luật ĐẦU tiên của cả track này (bài 1): sự kiện là một
sự THẬT đã xảy ra, không phải một ô nhớ có thể ghi đè. `e2` với
`soTien: 15000` LÀ sự thật rằng hệ thống đã (nhầm) trừ `15000` — xoá
sự thật đó bằng cách gán đè có nghĩa là xoá luôn BẰNG CHỨNG rằng lỗi
từng xảy ra.

Chỗ lệch: gán trực tiếp `e2.soTien = 10000` mutate object `e2` NGAY
tại chỗ nó đang nằm trong mảng — đây chính là kiểu thao tác `vi.soDu
-= soTien` mà bài 1 đã bác bỏ, chỉ khác LÀ áp dụng lên một sự kiện
thay vì một số dư. Sau thao tác đó, không còn cách nào biết `e2` từng
LÀ `15000` — cả lỗi VÀ bằng chứng về lỗi đều biến mất, điều mà một hệ
thống cần audit (kiểm toán) giao dịch không bao giờ được phép.
::
:::
::::

::::code{#viet_hoan_tien}
Hoàn thiện `hoanTien` — tìm sự kiện có `id === idSuKienSai` trong
`nhatKy`. Nếu KHÔNG tìm thấy, trả về NGUYÊN `nhatKy`. Nếu tìm thấy,
tạo một sự kiện MỚI loại `"da_hoan_tien"` với `soTien` bằng đúng
`soTien` của sự kiện sai, `id` là `idSuKienMoi`, VÀ `idSuKienGoc` là
`idSuKienSai` — rồi trả về `nhatKy` với sự kiện đó THÊM vào cuối
(không sửa `nhatKy` truyền vào).

```typescript title=starter
type LoaiSuKien = "da_nap" | "da_tru" | "da_hoan_tien";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; idSuKienGoc?: string; }
interface TrangThai { soDu: number; }
function trangThaiBanDau(): TrangThai { return { soDu: 0 }; }
function apDung(trangThai: TrangThai, suKien: SuKien): TrangThai {
  if (suKien.loai === "da_tru") return { soDu: trangThai.soDu - suKien.soTien };
  return { soDu: trangThai.soDu + suKien.soTien };
}
function tinhTrangThai(nhatKy: SuKien[]): TrangThai {
  return nhatKy.reduce(apDung, trangThaiBanDau());
}

function hoanTien(nhatKy: SuKien[], idSuKienSai: string, idSuKienMoi: string): SuKien[] {
  ___
}

const nkX: SuKien[] = [{ id: "x1", loai: "da_tru", soTien: 200 }];
const daSuaX = hoanTien(nkX, "x1", "x2");
console.log(daSuaX.length, tinhTrangThai(daSuaX).soDu);
```

```typescript title=solution
type LoaiSuKien = "da_nap" | "da_tru" | "da_hoan_tien";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; idSuKienGoc?: string; }
interface TrangThai { soDu: number; }
function trangThaiBanDau(): TrangThai { return { soDu: 0 }; }
function apDung(trangThai: TrangThai, suKien: SuKien): TrangThai {
  if (suKien.loai === "da_tru") return { soDu: trangThai.soDu - suKien.soTien };
  return { soDu: trangThai.soDu + suKien.soTien };
}
function tinhTrangThai(nhatKy: SuKien[]): TrangThai {
  return nhatKy.reduce(apDung, trangThaiBanDau());
}

function hoanTien(nhatKy: SuKien[], idSuKienSai: string, idSuKienMoi: string): SuKien[] {
  const suKienSai = nhatKy.find((sk) => sk.id === idSuKienSai);
  if (suKienSai === undefined) return nhatKy;
  const suKienBu: SuKien = { id: idSuKienMoi, loai: "da_hoan_tien", soTien: suKienSai.soTien, idSuKienGoc: idSuKienSai };
  return [...nhatKy, suKienBu];
}

const nkX: SuKien[] = [{ id: "x1", loai: "da_tru", soTien: 200 }];
const daSuaX = hoanTien(nkX, "x1", "x2");
console.log(daSuaX.length, tinhTrangThai(daSuaX).soDu);
```

```typescript title=test
const nk: SuKien[] = [
  { id: "a", loai: "da_nap", soTien: 1000 },
  { id: "b", loai: "da_tru", soTien: 400 },
];
const daSua = hoanTien(nk, "b", "c");
if (daSua.length !== 3) throw new Error("hoanTien phai THEM 1 su kien moi (khong xoa, khong sua) -- tong phai la 3");
const nkLenSauKhiGoi = nk.length;
if (nkLenSauKhiGoi !== 2) throw new Error("nhat ky GOC KHONG duoc bi doi (immutable) -- van phai la 2");

const suKienCuoi = daSua[2];
if (suKienCuoi === undefined || suKienCuoi.loai !== "da_hoan_tien") throw new Error("su kien MOI them vao phai co loai la da_hoan_tien");
if (suKienCuoi.soTien !== 400) throw new Error("su kien hoan tien phai co DUNG soTien cua su kien sai (400)");
if (suKienCuoi.id !== "c") throw new Error("su kien hoan tien phai dung id MOI duoc truyen vao");

const soDuSau = tinhTrangThai(daSua).soDu;
if (soDuSau !== 1000) throw new Error("sau khi hoan tien, so du phai tro ve DUNG 1000 (1000 - 400 + 400)");

const suKienSaiConDo = daSua.find((sk) => sk.id === "b");
if (suKienSaiConDo === undefined) throw new Error("su kien SAI (b) khong duoc bi xoa khoi log");
if (suKienSaiConDo.soTien !== 400 || suKienSaiConDo.loai !== "da_tru") throw new Error("su kien SAI (b) khong duoc bi SUA noi dung -- van phai la da_tru 400");

const khongTonTai = hoanTien(nk, "khong-co", "d");
if (khongTonTai.length !== 2) throw new Error("hoan tien mot id KHONG ton tai thi KHONG duoc them gi vao log");
```

:::hints
- kind: attention
  body: "Tim su kien sai bang nhatKy.find(sk => sk.id === idSuKienSai). Neu undefined, tra ve NGUYEN nhatKy. Neu tim thay, tao mot SuKien moi voi loai 'da_hoan_tien', soTien lay tu su kien sai, id la idSuKienMoi, idSuKienGoc la idSuKienSai -- roi tra ve [...nhatKy, suKienBu]."
- kind: strategy
  body: "const suKienSai = nhatKy.find((sk) => sk.id === idSuKienSai); if (suKienSai === undefined) return nhatKy; const suKienBu: SuKien = { id: idSuKienMoi, loai: 'da_hoan_tien', soTien: suKienSai.soTien, idSuKienGoc: idSuKienSai }; return [...nhatKy, suKienBu];"
- kind: one-line
  body: "const suKienSai = nhatKy.find((sk) => sk.id === idSuKienSai); if (suKienSai === undefined) return nhatKy; const suKienBu: SuKien = { id: idSuKienMoi, loai: \"da_hoan_tien\", soTien: suKienSai.soTien, idSuKienGoc: idSuKienSai }; return [...nhatKy, suKienBu];"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "2 0"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ghi bất biến, fold thành trạng thái, snapshot tiết kiệm công, idempotent
chống trùng lặp, lõi thuần tách khỏi vỏ, tái tạo từ đầu, sửa lỗi bằng
bù trừ — bảy khái niệm giờ đã có Ở tay. Còn thiếu đúng một thứ: ráp
TẤT cả chúng vào một dịch vụ thật, hoàn chỉnh.
::::

::::reflect{#nghi-lai}
`hoanTien` không hề mâu thuẫn với luật "sự kiện bất biến" của bài 1 —
nó chính LÀ cách luật đó áp dụng vào một tình huống đời thực: lỗi VẪN
xảy ra, chỉ là cách xử lý lỗi khác hẳn thói quen "tìm và sửa". Log chỉ
DÀI ra theo thời gian, không bao giờ bị viết đè — và chính vì vậy,
lịch sử ĐẦY đủ (kể cả sai lầm VÀ lần sửa nó) luôn còn nguyên để tra
soát, một điều không thể có được nếu lỗi bị xoá sạch dấu vết ngay khi
được phát hiện.
::::

::::checkpoint{mastery=0.82}
::::
