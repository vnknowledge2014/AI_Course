---
id: thiet-ke-he-thong.nhat-ky-bat-bien-va-fold.su-kien-bat-bien
title: "Sự kiện bất biến: ghi thêm, không sửa không xoá"
summary: "ghiSuKien(nk, sk) tra ve mang MOI voi su kien them vao CUOI, khong bao gio sua hay xoa phan tu cu -- doi lap truc tiep voi cach OOP vi.soDu -= soTien ghi de field da dung xuyen suot T7.2; nhat ky (NhatKy) la mot mang SuKien bat bien, moi 'thay doi' la mot su that MOI duoc ghi them, khong phai mot gia tri cu bi ghi de."
locale: vi
track: thiet-ke-he-thong
module: nhat-ky-bat-bien-va-fold
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [sd.fp.su-kien-bat-bien]
requires: [sd.boss-giao-dich-va-tien]
concepts: [sd.fp.su-kien-bat-bien]
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
Track "Thiết kế thực chiến" vừa khép lại bằng một sàn giao dịch hoàn
chỉnh — khớp lệnh, chuyển tiền, sổ cái, đúng thứ tự từng bước. Nhưng
suốt chín bài đó, một dòng lệnh lặp lại không hề bị hỏi tới:
`viNguon.soDu -= soTien`. Sửa thẳng một field. Giá trị CŨ biến mất
ngay lập tức, giá trị MỚI ghi đè lên đúng chỗ đó — và sau dòng lệnh
này, không còn cách nào biết giá trị cũ từng LÀ gì. Track MỚI hỏi một
câu khác hẳn: nếu KHÔNG được phép sửa hay xoá bất cứ thứ gì đã từng
xảy ra, trạng thái sẽ trông như thế nào?
::::

::::explain{#su-kien-la-su-that-da-xay-ra}
Một "sự kiện" (event) ghi lại một sự THẬT đã xảy ra — nó không mô tả
trạng thái hiện tại, nó mô tả một điều gì đó VỪA xảy ra. Một khi đã
ghi vào nhật ký (`NhatKy`), một sự kiện KHÔNG BAO GIỜ bị sửa hay xoá.
`ghiSuKien` chỉ làm đúng MỘT việc: trả về một mảng MỚI với sự kiện
thêm vào cuối, không đụng tới mảng cũ:

```typescript title=readonly
interface Vi { soDu: number; }
function truTienKieuCu(vi: Vi, soTien: number): void {
  vi.soDu -= soTien;
}

const viCu: Vi = { soDu: 100000 };
truTienKieuCu(viCu, 30000);
console.log("kieu OOP (T7.2): sau khi tru, soDu bi GHI DE truc tiep:", viCu.soDu);

type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien {
  id: string;
  loai: LoaiSuKien;
  soTien: number;
}
type NhatKy = SuKien[];

function taoNhatKy(): NhatKy {
  return [];
}

function ghiSuKien(nk: NhatKy, sk: SuKien): NhatKy {
  return [...nk, sk];
}

let nk: NhatKy = taoNhatKy();
nk = ghiSuKien(nk, { id: "e1", loai: "da_nap", soTien: 100000 });
nk = ghiSuKien(nk, { id: "e2", loai: "da_tru", soTien: 30000 });
console.log("kieu su kien (T7.3): KHONG co field soDu nao - chi co nhat ky voi", nk.length, "su kien");
console.log(JSON.stringify(nk));
```

```text title=readonly
kieu OOP (T7.2): sau khi tru, soDu bi GHI DE truc tiep: 70000
kieu su kien (T7.3): KHONG co field soDu nao - chi co nhat ky voi 2 su kien
[{"id":"e1","loai":"da_nap","soTien":100000},{"id":"e2","loai":"da_tru","soTien":30000}]
```

`truTienKieuCu` xoá SẠCH giá trị `100000` ban đầu — sau dòng lệnh đó,
`100000` không tồn tại Ở bất cứ đâu nữa, chỉ còn `70000`. Ngược lại,
`ghiSuKien` không hề "trừ tiền" — nó chỉ THÊM một sự thật mới
(`{loai: "da_tru", soTien: 30000}`) vào nhật ký. Sự kiện `e1`
(`da_nap 100000`) vẫn còn NGUYÊN trong mảng, y hệt lúc mới ghi. Nhật
ký không có field `soDu` nào để sửa — nó chỉ có LỊCH sử.
::::

::::example{#ghi-them-khong-dung-den-mang-cu}
`ghiSuKien` không hề sửa mảng được truyền vào — mỗi lần gọi tạo ra một
mảng HOÀN TOÀN mới, mảng cũ đứng yên nguyên vẹn Ở đúng độ dài của nó:

```typescript title=readonly
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
type NhatKy = SuKien[];
function taoNhatKy(): NhatKy { return []; }
function ghiSuKien(nk: NhatKy, sk: SuKien): NhatKy { return [...nk, sk]; }

const goc: NhatKy = taoNhatKy();
const sauNap: NhatKy = ghiSuKien(goc, { id: "e1", loai: "da_nap", soTien: 5000 });
console.log("mang GOC truoc khi goi ghiSuKien co", goc.length, "phan tu");
const sauNap2: NhatKy = ghiSuKien(sauNap, { id: "e2", loai: "da_nap", soTien: 1000 });
console.log("mang GOC sau ca hai lan goi van co", goc.length, "phan tu (KHONG doi)");
console.log("mang sauNap van co", sauNap.length, "phan tu (KHONG bi anh huong boi lan ghi SAU)");
console.log("mang sauNap2 co", sauNap2.length, "phan tu");
console.log("goc va sauNap co phai CUNG mot tham chieu khong?", (goc as unknown) === (sauNap as unknown));
```

```text title=readonly
mang GOC truoc khi goi ghiSuKien co 0 phan tu
mang GOC sau ca hai lan goi van co 0 phan tu (KHONG doi)
mang sauNap van co 1 phan tu (KHONG bi anh huong boi lan ghi SAU)
mang sauNap2 co 2 phan tu
goc va sauNap co phai CUNG mot tham chieu khong? false
```

`goc` bắt đầu với `0` phần tử — VÀ vẫn còn `0` phần tử sau khi hai lần
gọi `ghiSuKien` xảy ra dựa trên nó. `sauNap` (kết quả của lần ghi đầu)
cũng không bị lần ghi THỨ HAI ảnh hưởng — nó dừng lại đúng Ở `1` phần
tử mãi mãi. Mỗi lần gọi `ghiSuKien` tạo ra một "phiên bản" nhật ký
MỚI, các phiên bản cũ không hề biến mất hay bị viết đè.
::::

::::predict{#doan-goc-sau-khi-ghi commitOnce}
Tiếp tục từ đoạn Ở trên: `goc` là một `NhatKy` rỗng (`0` phần tử).
Gọi `ghiSuKien(goc, { id: "moi", loai: "da_nap", soTien: 9000 })` một
lần NỮA (không gán kết quả vào biến nào, chỉ gọi rồi bỏ qua kết quả
trả về) — sau lệnh gọi ĐÓ, `goc.length` là bao nhiêu?

:::opt{correct}
`0` — không đổi; `ghiSuKien` luôn trả về một mảng MỚI bằng
`[...nk, sk]`, nó không hề chạm vào `nk` (ở đây là `goc`) được truyền
vào, dù kết quả trả về có được dùng hay không
:::
:::opt
`1` — vì `ghiSuKien` đã "ghi" một sự kiện, nên nhật ký được truyền
vào phải phản ánh sự kiện đó ngay
::why
Nhầm "gọi hàm ghi" với "hàm sửa TRỰC TIẾP tham số truyền vào" — nhưng
`ghiSuKien` không hề có dòng lệnh nào đụng tới `nk` cả, nó chỉ ĐỌC
`nk` để tạo mảng mới.

Chỗ lệch: thân hàm chỉ có đúng một dòng — `return [...nk, sk];`. Toán
tử spread `...nk` sao chép các phần tử của `nk` vào một mảng HOÀN
TOÀN mới, nó không sửa `nk`. Kết quả trả về mới là nhật ký "sau khi
ghi" — và Ở đây kết quả đó bị bỏ qua, không gán vào đâu cả, nên không
ai nhìn thấy nó. `goc` — biến gốc — không hề bị đụng tới.
::
:::
::::

::::code{#viet_ghi_su_kien}
Hoàn thiện `ghiSuKien` — trả về một `NhatKy` MỚI chứa toàn bộ sự kiện
của `nk` cộng thêm `sk` Ở cuối, mà KHÔNG sửa đổi `nk` truyền vào.

```typescript title=starter
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
type NhatKy = SuKien[];
function taoNhatKy(): NhatKy { return []; }

function ghiSuKien(nk: NhatKy, sk: SuKien): NhatKy {
  ___
}

const nkX = taoNhatKy();
const nkX2 = ghiSuKien(nkX, { id: "x1", loai: "da_nap", soTien: 2000 });
console.log(nkX.length, nkX2.length);
```

```typescript title=solution
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
type NhatKy = SuKien[];
function taoNhatKy(): NhatKy { return []; }

function ghiSuKien(nk: NhatKy, sk: SuKien): NhatKy {
  return [...nk, sk];
}

const nkX = taoNhatKy();
const nkX2 = ghiSuKien(nkX, { id: "x1", loai: "da_nap", soTien: 2000 });
console.log(nkX.length, nkX2.length);
```

```typescript title=test
const nkT1 = taoNhatKy();
const nkT2 = ghiSuKien(nkT1, { id: "e1", loai: "da_nap", soTien: 1000 });
if (nkT1.length !== 0) throw new Error("nhat ky GOC khong duoc bi doi (immutable)");
if (nkT2.length !== 1) throw new Error("nhat ky MOI phai co dung 1 su kien sau khi ghi");
const skDau = nkT2[0];
if (skDau === undefined || skDau.id !== "e1") throw new Error("su kien vua ghi phai co id dung");

const nkT3 = ghiSuKien(nkT2, { id: "e2", loai: "da_tru", soTien: 400 });
if (nkT2.length !== 1) throw new Error("nhat ky TRUOC (nkT2) khong duoc bi doi sau lan ghi SAU");
if (nkT3.length !== 2) throw new Error("nhat ky MOI NHAT phai co 2 su kien");
const skHai = nkT3[1];
if (skHai === undefined || skHai.loai !== "da_tru") throw new Error("su kien thu hai phai la da_tru");
```

:::hints
- kind: attention
  body: "ghiSuKien phai tra ve mot mang MOI, khong sua nk truyen vao. Dung toan tu spread de sao chep cac phan tu cu, roi them sk vao cuoi."
- kind: strategy
  body: "return [...nk, sk];"
- kind: one-line
  body: "return [...nk, sk];"
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
Nhật ký chỉ ghi, không bao giờ sửa. Nhưng một nhật ký toàn sự kiện thì
để làm GÌ — số dư ví đâu rồi? Nó không mất — nó chưa từng được LƯU.
::::

::::reflect{#nghi-lai}
`ghiSuKien` chỉ có một việc — thêm một sự thật MỚI vào cuối một danh
sách các sự thật đã có. Không field nào bị ghi đè, không giá trị nào
biến mất. Đây là khác biệt cốt lõi với toàn bộ track "Thiết kế thực
chiến" vừa xong: Ở đó, "cập nhật" nghĩa là sửa một ô nhớ; Ở đây, "cập
nhật" nghĩa là ghi thêm một dòng lịch sử. Câu hỏi còn để ngỏ: nếu số
dư không được lưu Ở đâu, làm sao biết một ví có bao nhiêu tiền?
::::

::::checkpoint{mastery=0.66}
::::
