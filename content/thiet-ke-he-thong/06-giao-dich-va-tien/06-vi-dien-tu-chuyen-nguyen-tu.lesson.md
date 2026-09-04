---
id: thiet-ke-he-thong.giao-dich-va-tien.vi-dien-tu-chuyen-nguyen-tu
title: "Chuyển tiền nguyên tử: cả hai thao tác, hoặc không thao tác nào"
summary: "chuyenTien(ht, tu, den, soTien) kiem tra vi DICH ton tai VA vi NGUON du tien TRUOC khi cham vao so du -- an chuyen 30000 cho binh thanh cong (an con 70000, binh len 50000); an chuyen cho vi KHONG TON TAI 'chi' tra ve 'vi_dich_khong_ton_tai' VA so du an KHONG doi (van 70000, khong bi tru truoc roi hoan lai); an chuyen 999999 (vuot so du) cung khong dung cham vi nao -- ca hai thao tac (tru nguon, cong dich) chi xay ra CUNG luc, hoac khong xay ra gi ca."
locale: vi
track: thiet-ke-he-thong
module: giao-dich-va-tien
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [sd.vi-dien-tu-chuyen-nguyen-tu]
requires: [sd.vi-dien-tu-khong-am]
concepts: [sd.vi-dien-tu-chuyen-nguyen-tu]
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
Trừ tiền một ví, kiểm tra trước khi trừ — xong. Nhưng chuyển tiền
CHẠM vào HAI ví: trừ Ở nguồn, cộng Ở đích. Nếu thao tác thứ hai thất
bại NGAY sau khi thao tác đầu đã chạy — tiền biến MẤT khỏi hệ thống,
không thuộc VỀ ví nào cả. Chuyển tiền phải LÀ một khối duy nhất, không
tách rời được.
::::

::::explain{#kiem-tra-ca-hai-truoc-khi-dung-vao}
`chuyenTien` kiểm tra ví ĐÍCH có tồn tại KHÔNG, RỒI kiểm tra ví NGUỒN
có đủ tiền KHÔNG — CẢ hai kiểm tra đều chạy TRƯỚC khi bất kỳ dòng nào
sửa số dư. Chỉ khi CẢ hai điều kiện đều ổn, hai dòng cập nhật MỚI chạy
liền nhau:

```typescript title=readonly
interface Vi { chuSoHuu: string; soDu: number; }
interface HeThongVi { cacVi: Map<string, Vi>; }
function taoHeThongVi(): HeThongVi { return { cacVi: new Map() }; }
function moVi(ht: HeThongVi, chuSoHuu: string, soDuBanDau: number): void {
  ht.cacVi.set(chuSoHuu, { chuSoHuu, soDu: soDuBanDau });
}

type KetQuaChuyen = "da_chuyen" | "khong_du_so_du" | "vi_dich_khong_ton_tai";
function chuyenTien(ht: HeThongVi, tuChuSoHuu: string, denChuSoHuu: string, soTien: number): KetQuaChuyen {
  const viNguon = ht.cacVi.get(tuChuSoHuu)!;
  const viDich = ht.cacVi.get(denChuSoHuu);
  if (viDich === undefined) return "vi_dich_khong_ton_tai";
  if (viNguon.soDu < soTien) return "khong_du_so_du";
  viNguon.soDu -= soTien;
  viDich.soDu += soTien;
  return "da_chuyen";
}

const ht = taoHeThongVi();
moVi(ht, "an", 100000);
moVi(ht, "binh", 20000);

console.log("an chuyen 30000 cho binh:", chuyenTien(ht, "an", "binh", 30000));
console.log("so du an:", ht.cacVi.get("an")!.soDu);
console.log("so du binh:", ht.cacVi.get("binh")!.soDu);

console.log("an chuyen cho vi KHONG TON TAI 'chi':", chuyenTien(ht, "an", "chi", 10000));
console.log("so du an SAU loi (khong doi):", ht.cacVi.get("an")!.soDu);
```

```text title=readonly
an chuyen 30000 cho binh: da_chuyen
so du an: 70000
so du binh: 50000
an chuyen cho vi KHONG TON TAI 'chi': vi_dich_khong_ton_tai
so du an SAU loi (khong doi): 70000
```

Chuyển `30000` cho `binh` cập nhật CẢ hai số dư CÙNG lúc — `an` giảm
ĐÚNG bằng `binh` tăng. Chuyển cho `"chi"` (chưa từng mở ví) bị chặn
NGAY tại dòng kiểm tra `viDich === undefined` — hàm return trước khi
chạm dòng `viNguon.soDu -= soTien` — số dư của `an` giữ NGUYÊN
`70000`, không hề bị trừ RỒI hoàn lại.
::::

::::example{#bao-toan-khi-that-bai}
Thất bại VÌ không đủ tiền cũng KHÔNG chạm vào số dư nào — VÀ tổng của
hai ví liên quan luôn giữ NGUYÊN qua mọi lần chuyển thất bại, dù thất
bại VÌ lý do gì:

```typescript title=readonly
interface Vi { chuSoHuu: string; soDu: number; }
interface HeThongVi { cacVi: Map<string, Vi>; }
function taoHeThongVi(): HeThongVi { return { cacVi: new Map() }; }
function moVi(ht: HeThongVi, chuSoHuu: string, soDuBanDau: number): void {
  ht.cacVi.set(chuSoHuu, { chuSoHuu, soDu: soDuBanDau });
}

type KetQuaChuyen = "da_chuyen" | "khong_du_so_du" | "vi_dich_khong_ton_tai";
function chuyenTien(ht: HeThongVi, tuChuSoHuu: string, denChuSoHuu: string, soTien: number): KetQuaChuyen {
  const viNguon = ht.cacVi.get(tuChuSoHuu)!;
  const viDich = ht.cacVi.get(denChuSoHuu);
  if (viDich === undefined) return "vi_dich_khong_ton_tai";
  if (viNguon.soDu < soTien) return "khong_du_so_du";
  viNguon.soDu -= soTien;
  viDich.soDu += soTien;
  return "da_chuyen";
}

// tai lap dung trang thai tu khoi truoc, KHONG in lai: an mo 100000, binh mo
// 20000, an chuyen 30000 cho binh (thanh cong), chuyen cho "chi" (khong ton
// tai) that bai -- so du hien tai: an 70000, binh 50000
const ht = taoHeThongVi();
moVi(ht, "an", 100000);
moVi(ht, "binh", 20000);
chuyenTien(ht, "an", "binh", 30000);
chuyenTien(ht, "an", "chi", 10000);

const tongTruoc = ht.cacVi.get("an")!.soDu + ht.cacVi.get("binh")!.soDu;
console.log("an chuyen 999999 cho binh (KHONG DU tien):", chuyenTien(ht, "an", "binh", 999999));
console.log("so du an sau loi khong du tien:", ht.cacVi.get("an")!.soDu);
console.log("so du binh sau loi khong du tien:", ht.cacVi.get("binh")!.soDu);
const tongSau = ht.cacVi.get("an")!.soDu + ht.cacVi.get("binh")!.soDu;
console.log("tong hai vi TRUOC va SAU lan chuyen that bai co bang nhau?", tongTruoc === tongSau);
```

```text title=readonly
an chuyen 999999 cho binh (KHONG DU tien): khong_du_so_du
so du an sau loi khong du tien: 70000
so du binh sau loi khong du tien: 50000
tong hai vi TRUOC va SAU lan chuyen that bai co bang nhau? true
```

Chuyển `999999` — vượt xa số dư `70000` của `an` — bị chặn Ở dòng
kiểm tra số dư, TRƯỚC khi chạm vào bất kỳ số dư nào. Cả hai ví giữ
NGUYÊN giá trị cũ, VÀ tổng `tongTruoc` bằng HỆT `tongSau` — không một
đồng nào biến mất hay tự sinh RA trong lần thử thất bại.
::::

::::predict{#doan-thu-tu-kiem-tra commitOnce}
`an` hiện chỉ còn `70000`. Gọi `chuyenTien(ht, "an", "khong-ton-tai",
999999999)` — VỪA ví đích KHÔNG tồn tại, VỪA số tiền vượt xa số dư
CỦA `an`. Kết quả trả về LÀ gì?

:::opt{correct}
`"vi_dich_khong_ton_tai"` — dòng kiểm tra `viDich === undefined` nằm
TRƯỚC dòng kiểm tra `viNguon.soDu < soTien` trong thân hàm; hàm return
NGAY tại điều kiện ĐẦU tiên gặp phải, không bao giờ chạy tới điều kiện
thứ hai
:::
:::opt
`"khong_du_so_du"` — số tiền `999999999` lớn HƠN rất nhiều so với số
dư của `an`, đây LÀ lý do "thực sự" khiến giao dịch không thể thực
hiện, nên nó phải LÀ lỗi được báo
::why
Nhầm "lý do NÀO nghiêm trọng hơn" VỚI "lý do NÀO được kiểm tra
trước" — nhưng `chuyenTien` không hề "cân nhắc" xem lỗi nào đáng báo
hơn, nó chỉ đơn giản CHẠY các dòng `if` theo đúng thứ TỰ viết ra.

Chỗ lệch: thân hàm kiểm tra `if (viDich === undefined) return
"vi_dich_khong_ton_tai";` TRƯỚC dòng `if (viNguon.soDu < soTien)
return "khong_du_so_du";`. Khi cả hai điều kiện đều đúng, hàm gặp
điều kiện ĐẦU tiên, return NGAY LẬP tức — dòng kiểm tra số dư không
bao giờ được chạy tới, dù số tiền có lớn tới đâu.
::
:::
::::

::::code{#viet_chuyen_tien}
Hoàn thiện `chuyenTien` — `viNguon` VÀ `viDich` đã được tra cứu. Còn
thiếu: nếu `viDich` LÀ `undefined`, trả về `"vi_dich_khong_ton_tai"`;
nếu `viNguon.soDu` không đủ, trả về `"khong_du_so_du"`; ngược lại,
trừ `soTien` khỏi `viNguon.soDu` VÀ cộng vào `viDich.soDu` (cả HAI
dòng liền nhau), RỒI trả về `"da_chuyen"`.

```typescript title=starter
interface Vi { chuSoHuu: string; soDu: number; }
interface HeThongVi { cacVi: Map<string, Vi>; }
function taoHeThongVi(): HeThongVi { return { cacVi: new Map() }; }
function moVi(ht: HeThongVi, chuSoHuu: string, soDuBanDau: number): void {
  ht.cacVi.set(chuSoHuu, { chuSoHuu, soDu: soDuBanDau });
}

type KetQuaChuyen = "da_chuyen" | "khong_du_so_du" | "vi_dich_khong_ton_tai";
function chuyenTien(ht: HeThongVi, tuChuSoHuu: string, denChuSoHuu: string, soTien: number): KetQuaChuyen {
  const viNguon = ht.cacVi.get(tuChuSoHuu)!;
  const viDich = ht.cacVi.get(denChuSoHuu);
  ___
}

const htX = taoHeThongVi();
moVi(htX, "p", 500);
moVi(htX, "q", 100);
console.log(chuyenTien(htX, "p", "q", 500), chuyenTien(htX, "p", "q", 1));
```

```typescript title=solution
interface Vi { chuSoHuu: string; soDu: number; }
interface HeThongVi { cacVi: Map<string, Vi>; }
function taoHeThongVi(): HeThongVi { return { cacVi: new Map() }; }
function moVi(ht: HeThongVi, chuSoHuu: string, soDuBanDau: number): void {
  ht.cacVi.set(chuSoHuu, { chuSoHuu, soDu: soDuBanDau });
}

type KetQuaChuyen = "da_chuyen" | "khong_du_so_du" | "vi_dich_khong_ton_tai";
function chuyenTien(ht: HeThongVi, tuChuSoHuu: string, denChuSoHuu: string, soTien: number): KetQuaChuyen {
  const viNguon = ht.cacVi.get(tuChuSoHuu)!;
  const viDich = ht.cacVi.get(denChuSoHuu);
  if (viDich === undefined) return "vi_dich_khong_ton_tai";
  if (viNguon.soDu < soTien) return "khong_du_so_du";
  viNguon.soDu -= soTien;
  viDich.soDu += soTien;
  return "da_chuyen";
}

const htX = taoHeThongVi();
moVi(htX, "p", 500);
moVi(htX, "q", 100);
console.log(chuyenTien(htX, "p", "q", 500), chuyenTien(htX, "p", "q", 1));
```

```typescript title=test
const htT = taoHeThongVi();
moVi(htT, "s1", 1000);
moVi(htT, "s2", 200);

const r1 = chuyenTien(htT, "s1", "s2", 300);
if (r1 !== "da_chuyen") throw new Error("chuyen hop le phai thanh cong");
const soDuS1a = htT.cacVi.get("s1")!.soDu;
const soDuS2a = htT.cacVi.get("s2")!.soDu;
if (soDuS1a !== 700) throw new Error("vi nguon phai giam dung 300");
if (soDuS2a !== 500) throw new Error("vi dich phai tang dung 300");

const r2 = chuyenTien(htT, "s1", "khong-ton-tai", 100);
if (r2 !== "vi_dich_khong_ton_tai") throw new Error("chuyen den vi KHONG ton tai phai bao loi vi_dich_khong_ton_tai");
const soDuS1b = htT.cacVi.get("s1")!.soDu;
if (soDuS1b !== 700) throw new Error("vi nguon KHONG duoc thay doi khi vi dich khong ton tai");

const r3 = chuyenTien(htT, "s1", "s2", 999999);
if (r3 !== "khong_du_so_du") throw new Error("chuyen vuot qua so du phai bao loi khong_du_so_du");
const soDuS1c = htT.cacVi.get("s1")!.soDu;
const soDuS2c = htT.cacVi.get("s2")!.soDu;
if (soDuS1c !== 700) throw new Error("vi nguon KHONG duoc thay doi khi khong du tien");
if (soDuS2c !== 500) throw new Error("vi dich KHONG duoc thay doi khi khong du tien");

const r4 = chuyenTien(htT, "s1", "khong-ton-tai-2", 999999999);
if (r4 !== "vi_dich_khong_ton_tai") throw new Error("khi CA HAI dieu kien sai (vi dich khong ton tai VA khong du tien), phai bao loi vi dich TRUOC");

const tong = htT.cacVi.get("s1")!.soDu + htT.cacVi.get("s2")!.soDu;
if (tong !== 1200) throw new Error("tong hai vi phai giu nguyen 1200 sau moi lan chuyen that bai");
```

:::hints
- kind: attention
  body: "Ba buoc, DUNG thu tu: (1) if (viDich === undefined) return 'vi_dich_khong_ton_tai'; (2) if (viNguon.soDu < soTien) return 'khong_du_so_du'; (3) tru viNguon.soDu, cong viDich.soDu, return 'da_chuyen'."
- kind: strategy
  body: "if (viDich === undefined) return 'vi_dich_khong_ton_tai'; if (viNguon.soDu < soTien) return 'khong_du_so_du'; viNguon.soDu -= soTien; viDich.soDu += soTien; return 'da_chuyen';"
- kind: one-line
  body: "if (viDich === undefined) return \"vi_dich_khong_ton_tai\"; if (viNguon.soDu < soTien) return \"khong_du_so_du\"; viNguon.soDu -= soTien; viDich.soDu += soTien; return \"da_chuyen\";"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "khong_du_so_du"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Kiểm tra cả hai điều kiện TRƯỚC khi đụng vào số dư — không có trạng
thái nửa vời. Ba mảnh của "tiền" (idempotency, sổ cái, ví) đã xong.
Giờ chuyển hẳn sang một hệ thống khác: khớp lệnh mua bán, nơi TRẬT tự
còn quan trọng hơn cả tính đúng đắn của TỪNG lệnh riêng lẻ.
::::

::::reflect{#nghi-lai}
`chuyenTien` đạt được tính nguyên tử KHÔNG phải bằng một cơ chế khoá
hay giao dịch đặc biệt NÀO — nó đạt được bằng cách sắp xếp ĐÚNG thứ
tự: dồn HẾT các điều kiện có thể khiến thao tác thất bại lên ĐẦU thân
hàm, rồi mới cho phép hai dòng CẬP nhật trạng thái đứng CẠNH nhau,
không có gì xen giữa. Một khi đã qua được cả hai `if`, không còn lý
do NÀO để thất bại nữa — hai dòng cập nhật LUÔN chạy CÙNG nhau.
::::

::::checkpoint{mastery=0.76}
::::
