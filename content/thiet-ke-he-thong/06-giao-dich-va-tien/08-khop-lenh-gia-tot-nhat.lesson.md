---
id: thiet-ke-he-thong.giao-dich-va-tien.khop-lenh-gia-tot-nhat
title: "Khớp lệnh: giá tốt nhất trước, khớp một phần khi lệch số lượng"
summary: "khopLenh(sl, lenhMoi) lay lenh doi ung TOT NHAT ([0]) roi so soLuong -- m1 (mua, soLuong 10) khop VUA KHIT voi b1 (ban, gia 50000, soLuong 10), b1 bi xoa khoi so lenh; m2 (soLuong 3) khop MOT PHAN voi b2 (soLuong 5), b2 con lai DUNG 2 VAN nam trong so lenh; m3 (soLuong 10) khop het b2 (con 2), PHAN CON DU cua m3 (8) duoc them VAO chinh so lenh cua no qua themLenh -- lenh lon hon luon con lai DUNG phan chua khop."
locale: vi
track: thiet-ke-he-thong
module: giao-dich-va-tien
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [sd.khop-lenh-gia-tot-nhat]
requires: [sd.so-lenh-mua-ban]
concepts: [sd.khop-lenh-gia-tot-nhat]
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
Sổ lệnh sắp xếp đúng — giá tốt nhất LUÔN Ở đầu mỗi bên. Nhưng một sổ
lệnh đứng YÊN không làm được gì. Một lệnh MUA mới tới — nó phải khớp
VỚI lệnh BÁN nào? VÀ nếu số lượng hai bên không khớp NHAU vừa vặn,
phần dư đi đâu?
::::

::::explain{#khop-voi-gia-tot-nhat}
`khopLenh` lấy lệnh đối ứng Ở vị trí `[0]` — LUÔN là giá tốt nhất của
phía BÊN kia — RỒI so sánh số lượng. Nếu hai bên BẰNG nhau, cả hai
đều được khớp HẾT, lệnh đối ứng bị xoá khỏi sổ:

```typescript title=readonly
type BenLenh = "mua" | "ban";
interface Lenh { id: string; ben: BenLenh; gia: number; soLuong: number; }
interface SoLenh { lenhMua: Lenh[]; lenhBan: Lenh[]; }
function taoSoLenh(): SoLenh { return { lenhMua: [], lenhBan: [] }; }

function themLenh(sl: SoLenh, lenh: Lenh): void {
  if (lenh.ben === "mua") {
    sl.lenhMua.push(lenh);
    sl.lenhMua.sort((a, b) => b.gia - a.gia);
  } else {
    sl.lenhBan.push(lenh);
    sl.lenhBan.sort((a, b) => a.gia - b.gia);
  }
}

type TrangThaiKhop = "khong_co_doi_ung" | "khop_vua_khit" | "khop_mot_phan_lenh_moi_con_du" | "khop_mot_phan_doi_ung_con_du";
interface KetQuaKhop { trangThai: TrangThaiKhop; soLuongKhop: number; }

function khopLenh(sl: SoLenh, lenhMoi: Lenh): KetQuaKhop {
  const mangDoiUng = lenhMoi.ben === "mua" ? sl.lenhBan : sl.lenhMua;
  if (mangDoiUng.length === 0) {
    themLenh(sl, lenhMoi);
    return { trangThai: "khong_co_doi_ung", soLuongKhop: 0 };
  }
  const lenhDoiUng = mangDoiUng[0]!;
  const soLuongKhop = Math.min(lenhMoi.soLuong, lenhDoiUng.soLuong);
  if (lenhMoi.soLuong === lenhDoiUng.soLuong) {
    mangDoiUng.shift();
    return { trangThai: "khop_vua_khit", soLuongKhop };
  }
  if (lenhMoi.soLuong > lenhDoiUng.soLuong) {
    mangDoiUng.shift();
    themLenh(sl, { ...lenhMoi, soLuong: lenhMoi.soLuong - soLuongKhop });
    return { trangThai: "khop_mot_phan_lenh_moi_con_du", soLuongKhop };
  }
  lenhDoiUng.soLuong -= soLuongKhop;
  return { trangThai: "khop_mot_phan_doi_ung_con_du", soLuongKhop };
}

const sl = taoSoLenh();
themLenh(sl, { id: "b1", ben: "ban", gia: 50000, soLuong: 10 });
themLenh(sl, { id: "b2", ben: "ban", gia: 52000, soLuong: 5 });

const m1: Lenh = { id: "m1", ben: "mua", gia: 51000, soLuong: 10 };
console.log("m1 (soLuong 10) khop voi b1 (soLuong 10, gia tot nhat):", JSON.stringify(khopLenh(sl, m1)));
console.log("con lai trong lenhBan:", JSON.stringify(sl.lenhBan.map((l) => l.id)));
```

```text title=readonly
m1 (soLuong 10) khop voi b1 (soLuong 10, gia tot nhat): {"trangThai":"khop_vua_khit","soLuongKhop":10}
con lai trong lenhBan: ["b2"]
```

`khopLenh` chọn `mangDoiUng` LÀ `sl.lenhBan` vì `lenhMoi.ben === "mua"`
— nó không hề nhìn vào `b2`, chỉ nhìn Ở vị trí `[0]` (`b1`, giá tốt
nhất). Vì `10 === 10`, cả hai khớp HẾT, `b1` bị `shift()` khỏi
`lenhBan`. Chỉ còn `b2` — `b2` chưa hề bị động tới.
::::

::::example{#khop-mot-phan-hai-chieu}
Khi số lượng LỆCH nhau, bên NHỎ hơn khớp hết, bên LỚN hơn chỉ giảm
đúng phần đã khớp — VẪN nằm trong sổ lệnh (nếu LÀ lệnh đối ứng) hoặc
được thêm VÀO sổ lệnh của chính nó (nếu LÀ lệnh mới):

```typescript title=readonly
type BenLenh = "mua" | "ban";
interface Lenh { id: string; ben: BenLenh; gia: number; soLuong: number; }
interface SoLenh { lenhMua: Lenh[]; lenhBan: Lenh[]; }
function taoSoLenh(): SoLenh { return { lenhMua: [], lenhBan: [] }; }

function themLenh(sl: SoLenh, lenh: Lenh): void {
  if (lenh.ben === "mua") {
    sl.lenhMua.push(lenh);
    sl.lenhMua.sort((a, b) => b.gia - a.gia);
  } else {
    sl.lenhBan.push(lenh);
    sl.lenhBan.sort((a, b) => a.gia - b.gia);
  }
}

type TrangThaiKhop = "khong_co_doi_ung" | "khop_vua_khit" | "khop_mot_phan_lenh_moi_con_du" | "khop_mot_phan_doi_ung_con_du";
interface KetQuaKhop { trangThai: TrangThaiKhop; soLuongKhop: number; }

function khopLenh(sl: SoLenh, lenhMoi: Lenh): KetQuaKhop {
  const mangDoiUng = lenhMoi.ben === "mua" ? sl.lenhBan : sl.lenhMua;
  if (mangDoiUng.length === 0) {
    themLenh(sl, lenhMoi);
    return { trangThai: "khong_co_doi_ung", soLuongKhop: 0 };
  }
  const lenhDoiUng = mangDoiUng[0]!;
  const soLuongKhop = Math.min(lenhMoi.soLuong, lenhDoiUng.soLuong);
  if (lenhMoi.soLuong === lenhDoiUng.soLuong) {
    mangDoiUng.shift();
    return { trangThai: "khop_vua_khit", soLuongKhop };
  }
  if (lenhMoi.soLuong > lenhDoiUng.soLuong) {
    mangDoiUng.shift();
    themLenh(sl, { ...lenhMoi, soLuong: lenhMoi.soLuong - soLuongKhop });
    return { trangThai: "khop_mot_phan_lenh_moi_con_du", soLuongKhop };
  }
  lenhDoiUng.soLuong -= soLuongKhop;
  return { trangThai: "khop_mot_phan_doi_ung_con_du", soLuongKhop };
}

// tai lap dung trang thai tu khoi truoc, KHONG in lai: b1 (50000,10) da khop
// VUA KHIT voi m1, bi xoa khoi lenhBan. lenhBan gio chi con b2 (52000, 5)
const sl = taoSoLenh();
themLenh(sl, { id: "b1", ben: "ban", gia: 50000, soLuong: 10 });
themLenh(sl, { id: "b2", ben: "ban", gia: 52000, soLuong: 5 });
const m1: Lenh = { id: "m1", ben: "mua", gia: 51000, soLuong: 10 };
khopLenh(sl, m1);

const m2: Lenh = { id: "m2", ben: "mua", gia: 52500, soLuong: 3 };
console.log("m2 (soLuong 3) khop voi b2 (soLuong 5) -- lenh MOI nho hon:", JSON.stringify(khopLenh(sl, m2)));
console.log("b2 con lai trong lenhBan (giam con 2, VAN nam trong so lenh):", JSON.stringify(sl.lenhBan));

const m3: Lenh = { id: "m3", ben: "mua", gia: 51500, soLuong: 10 };
console.log("m3 (soLuong 10) khop voi b2 (soLuong 2) -- lenh MOI lon hon:", JSON.stringify(khopLenh(sl, m3)));
console.log("lenhBan sau khi b2 khop het:", JSON.stringify(sl.lenhBan));
console.log("phan con du CUA m3 (8) duoc them vao lenhMua:", JSON.stringify(sl.lenhMua.map((l) => l.id + ":" + l.soLuong)));
```

```text title=readonly
m2 (soLuong 3) khop voi b2 (soLuong 5) -- lenh MOI nho hon: {"trangThai":"khop_mot_phan_doi_ung_con_du","soLuongKhop":3}
b2 con lai trong lenhBan (giam con 2, VAN nam trong so lenh): [{"id":"b2","ben":"ban","gia":52000,"soLuong":2}]
m3 (soLuong 10) khop voi b2 (soLuong 2) -- lenh MOI lon hon: {"trangThai":"khop_mot_phan_lenh_moi_con_du","soLuongKhop":2}
lenhBan sau khi b2 khop het: []
phan con du CUA m3 (8) duoc them vao lenhMua: ["m3:8"]
```

`m2` (`3`) nhỏ hơn `b2` (`5`) — chỉ `3` được khớp, `b2` giảm CÒN `2`
NHƯNG vẫn nằm trong `lenhBan` (không bị xoá, chỉ đổi `soLuong`). Ngay
sau đó `m3` (`10`) lớn hơn `b2` (`2` lúc này) — `b2` khớp HẾT phần còn
lại VÀ bị xoá, còn `m3` dư đúng `8` — phần dư ĐÓ được `themLenh` đưa
VÀO `lenhMua`, sổ lệnh của chính `m3`, chờ khớp tiếp Ở lần sau.
::::

::::predict{#doan-khong-co-doi-ung commitOnce}
Sau đoạn TRÊN, `lenhBan` đang RỖNG. Một lệnh MỚI, `m4` (`ben: "mua"`,
bất kỳ giá VÀ số lượng nào), gọi `khopLenh(sl, m4)`. `m4` sẽ NẰM Ở
đâu ngay SAU lệnh gọi này?

:::opt{correct}
Trong `sl.lenhMua` — vì `mangDoiUng` (LÀ `sl.lenhBan`) rỗng, hàm đi
vào nhánh ĐẦU tiên, gọi `themLenh(sl, lenhMoi)` để đưa CHÍNH `m4` vào
sổ lệnh CỦA nó, giống hệt một lệnh CHƯA từng khớp
:::
:::opt
Không nằm Ở đâu cả — hàm chỉ trả về `"khong_co_doi_ung"` để BÁO rằng
không tìm được lệnh khớp, còn `m4` bị bỏ QUA, không được lưu lại
::why
Nhầm "không khớp được" VỚI "bị bỏ qua hoàn toàn" — nhưng `khopLenh`
xử lý trường hợp KHÔNG có đối ứng bằng cách coi `m4` LÀ một lệnh chờ
BÌNH thường, không phải một lệnh thất bại cần loại bỏ.

Chỗ lệch: nhánh `if (mangDoiUng.length === 0)` gọi `themLenh(sl,
lenhMoi);` TRƯỚC khi return — đây chính LÀ hàm bài trước ĐÃ xây, đưa
`lenhMoi` vào đúng mảng (`lenhMua`, vì `m4.ben === "mua"`) VÀ sắp xếp
lại. Một sàn giao dịch thật hoạt động y hệt: lệnh chưa khớp được KHÔNG
biến mất, nó chờ Ở sổ lệnh cho tới khi có lệnh đối ứng phù hợp xuất
hiện.
::
:::
::::

::::code{#viet_khop_lenh}
Hoàn thiện `khopLenh` — nếu KHÔNG có lệnh đối ứng, thêm `lenhMoi` vào
sổ lệnh CỦA nó (`themLenh`) VÀ trả về `"khong_co_doi_ung"`. Ngược lại,
lấy lệnh đối ứng TỐT nhất (`[0]`), tính `soLuongKhop`, RỒI xử lý BA
trường hợp: bằng nhau (xoá đối ứng, `"khop_vua_khit"`); lệnh MỚI lớn
hơn (xoá đối ứng, thêm phần dư của lệnh mới VÀO sổ, `"khop_mot_phan_
lenh_moi_con_du"`); đối ứng lớn hơn (giảm số lượng đối ứng,
`"khop_mot_phan_doi_ung_con_du"`).

```typescript title=starter
type BenLenh = "mua" | "ban";
interface Lenh { id: string; ben: BenLenh; gia: number; soLuong: number; }
interface SoLenh { lenhMua: Lenh[]; lenhBan: Lenh[]; }
function taoSoLenh(): SoLenh { return { lenhMua: [], lenhBan: [] }; }

function themLenh(sl: SoLenh, lenh: Lenh): void {
  if (lenh.ben === "mua") {
    sl.lenhMua.push(lenh);
    sl.lenhMua.sort((a, b) => b.gia - a.gia);
  } else {
    sl.lenhBan.push(lenh);
    sl.lenhBan.sort((a, b) => a.gia - b.gia);
  }
}

type TrangThaiKhop = "khong_co_doi_ung" | "khop_vua_khit" | "khop_mot_phan_lenh_moi_con_du" | "khop_mot_phan_doi_ung_con_du";
interface KetQuaKhop { trangThai: TrangThaiKhop; soLuongKhop: number; }

function khopLenh(sl: SoLenh, lenhMoi: Lenh): KetQuaKhop {
  ___
}

const slX = taoSoLenh();
themLenh(slX, { id: "s1", ben: "ban", gia: 100, soLuong: 4 });
const mX: Lenh = { id: "mX", ben: "mua", gia: 100, soLuong: 4 };
console.log(JSON.stringify(khopLenh(slX, mX)), slX.lenhBan.length);
```

```typescript title=solution
type BenLenh = "mua" | "ban";
interface Lenh { id: string; ben: BenLenh; gia: number; soLuong: number; }
interface SoLenh { lenhMua: Lenh[]; lenhBan: Lenh[]; }
function taoSoLenh(): SoLenh { return { lenhMua: [], lenhBan: [] }; }

function themLenh(sl: SoLenh, lenh: Lenh): void {
  if (lenh.ben === "mua") {
    sl.lenhMua.push(lenh);
    sl.lenhMua.sort((a, b) => b.gia - a.gia);
  } else {
    sl.lenhBan.push(lenh);
    sl.lenhBan.sort((a, b) => a.gia - b.gia);
  }
}

type TrangThaiKhop = "khong_co_doi_ung" | "khop_vua_khit" | "khop_mot_phan_lenh_moi_con_du" | "khop_mot_phan_doi_ung_con_du";
interface KetQuaKhop { trangThai: TrangThaiKhop; soLuongKhop: number; }

function khopLenh(sl: SoLenh, lenhMoi: Lenh): KetQuaKhop {
  const mangDoiUng = lenhMoi.ben === "mua" ? sl.lenhBan : sl.lenhMua;
  if (mangDoiUng.length === 0) {
    themLenh(sl, lenhMoi);
    return { trangThai: "khong_co_doi_ung", soLuongKhop: 0 };
  }
  const lenhDoiUng = mangDoiUng[0]!;
  const soLuongKhop = Math.min(lenhMoi.soLuong, lenhDoiUng.soLuong);
  if (lenhMoi.soLuong === lenhDoiUng.soLuong) {
    mangDoiUng.shift();
    return { trangThai: "khop_vua_khit", soLuongKhop };
  }
  if (lenhMoi.soLuong > lenhDoiUng.soLuong) {
    mangDoiUng.shift();
    themLenh(sl, { ...lenhMoi, soLuong: lenhMoi.soLuong - soLuongKhop });
    return { trangThai: "khop_mot_phan_lenh_moi_con_du", soLuongKhop };
  }
  lenhDoiUng.soLuong -= soLuongKhop;
  return { trangThai: "khop_mot_phan_doi_ung_con_du", soLuongKhop };
}

const slX = taoSoLenh();
themLenh(slX, { id: "s1", ben: "ban", gia: 100, soLuong: 4 });
const mX: Lenh = { id: "mX", ben: "mua", gia: 100, soLuong: 4 };
console.log(JSON.stringify(khopLenh(slX, mX)), slX.lenhBan.length);
```

```typescript title=test
const slT = taoSoLenh();
const nA: Lenh = { id: "nA", ben: "mua", gia: 200, soLuong: 5 };
const r0 = khopLenh(slT, nA);
if (r0.trangThai !== "khong_co_doi_ung") throw new Error("khong co doi ung khi so lenh RONG");
const soLuongMuaSauR0 = slT.lenhMua.length;
if (soLuongMuaSauR0 !== 1) throw new Error("khong co doi ung thi lenh MOI phai duoc them vao chinh so lenh cua no");

themLenh(slT, { id: "s1", ben: "ban", gia: 300, soLuong: 10 });
const nB: Lenh = { id: "nB", ben: "mua", gia: 300, soLuong: 10 };
const r1 = khopLenh(slT, nB);
if (r1.trangThai !== "khop_vua_khit") throw new Error("so luong bang nhau phai khop VUA KHIT");
if (r1.soLuongKhop !== 10) throw new Error("so luong khop phai la 10");
const soLuongBanSauR1 = slT.lenhBan.length;
if (soLuongBanSauR1 !== 0) throw new Error("lenh doi ung khop het phai bi xoa khoi so lenh");

themLenh(slT, { id: "s2", ben: "ban", gia: 400, soLuong: 20 });
const nC: Lenh = { id: "nC", ben: "mua", gia: 400, soLuong: 6 };
const r2 = khopLenh(slT, nC);
if (r2.trangThai !== "khop_mot_phan_doi_ung_con_du") throw new Error("lenh MOI nho hon doi ung phai la khop_mot_phan_doi_ung_con_du");
if (r2.soLuongKhop !== 6) throw new Error("so luong khop phai la 6 (so luong nho hon)");
const s2ConLai = slT.lenhBan[0];
if (s2ConLai === undefined || s2ConLai.soLuong !== 14) throw new Error("doi ung phai con lai DUNG 14 (20 - 6), VAN nam trong so lenh");

const nD: Lenh = { id: "nD", ben: "mua", gia: 400, soLuong: 50 };
const r3 = khopLenh(slT, nD);
if (r3.trangThai !== "khop_mot_phan_lenh_moi_con_du") throw new Error("lenh MOI lon hon doi ung phai la khop_mot_phan_lenh_moi_con_du");
if (r3.soLuongKhop !== 14) throw new Error("so luong khop phai la 14 (het phan con lai cua doi ung)");
const soLuongBanSauR3 = slT.lenhBan.length;
if (soLuongBanSauR3 !== 0) throw new Error("doi ung da khop het phai bi xoa khoi lenhBan");
const lenhMoiConDu = slT.lenhMua.find((l) => l.id === "nD");
if (lenhMoiConDu === undefined || lenhMoiConDu.soLuong !== 36) throw new Error("phan con du CUA lenh moi (50 - 14 = 36) phai duoc them vao lenhMua");
```

:::hints
- kind: attention
  body: "Bon nhanh: (1) mangDoiUng rong -> themLenh(sl, lenhMoi), tra ve khong_co_doi_ung. (2) lay lenhDoiUng = mangDoiUng[0]!, tinh soLuongKhop = Math.min(...). (3) bang nhau -> shift(), khop_vua_khit. (4) lenhMoi lon hon -> shift(), themLenh phan du CUA lenhMoi, khop_mot_phan_lenh_moi_con_du. (5) doi ung lon hon -> giam lenhDoiUng.soLuong, khop_mot_phan_doi_ung_con_du."
- kind: strategy
  body: "const mangDoiUng = lenhMoi.ben === 'mua' ? sl.lenhBan : sl.lenhMua; if (mangDoiUng.length === 0) { themLenh(sl, lenhMoi); return { trangThai: 'khong_co_doi_ung', soLuongKhop: 0 }; } const lenhDoiUng = mangDoiUng[0]!; const soLuongKhop = Math.min(lenhMoi.soLuong, lenhDoiUng.soLuong); if (lenhMoi.soLuong === lenhDoiUng.soLuong) { mangDoiUng.shift(); return { trangThai: 'khop_vua_khit', soLuongKhop }; } if (lenhMoi.soLuong > lenhDoiUng.soLuong) { mangDoiUng.shift(); themLenh(sl, { ...lenhMoi, soLuong: lenhMoi.soLuong - soLuongKhop }); return { trangThai: 'khop_mot_phan_lenh_moi_con_du', soLuongKhop }; } lenhDoiUng.soLuong -= soLuongKhop; return { trangThai: 'khop_mot_phan_doi_ung_con_du', soLuongKhop };"
- kind: one-line
  body: "Sao chep dung logic o phan Strategy, chi doi dau nhay don ' ' thanh dau nhay kep \" cho cac chuoi trangThai."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 7000
- tier: output
  match: contains
  expect: "khop_vua_khit"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Khớp giá tốt nhất, khớp một phần khi lệch số lượng — cơ chế cốt lõi
CỦA sàn giao dịch đã có. Nhưng "giá tốt nhất" chưa đủ khi HAI lệnh
CÙNG một mức giá — ai đến trước?
::::

::::reflect{#nghi-lai}
`khopLenh` tái sử dụng `themLenh` (bài trước) trong CẢ hai trường hợp
"chưa có gì để khớp" VÀ "khớp xong còn dư" — không viết lại logic
chèn-rồi-sắp-xếp một lần NỮA. Điều đó giữ cho MỘT sự thật luôn đúng dù
đi qua nhánh nào: bất cứ số lượng NÀO chưa khớp được, dù thuộc VỀ lệnh
mới hay lệnh đối ứng, LUÔN kết thúc Ở đúng một chỗ — nằm trong sổ
lệnh, chờ lượt khớp tiếp theo.
::::

::::checkpoint{mastery=0.80}
::::
