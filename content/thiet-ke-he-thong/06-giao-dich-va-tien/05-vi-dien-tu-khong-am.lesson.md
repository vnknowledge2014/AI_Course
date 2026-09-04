---
id: thiet-ke-he-thong.giao-dich-va-tien.vi-dien-tu-khong-am
title: "Ví điện tử: kiểm tra trước, không phải trừ rồi sửa"
summary: "truTien(ht, chuSoHuu, soTien) kiem tra vi.soDu < soTien TRUOC khi tru -- an mo vi 100000, tru 30000 thanh cong con 70000, tru THEM 80000 bi tu choi ('khong_du_so_du') VA so du VAN giu nguyen 70000 (khong tam thoi am roi sua lai); tru DUNG BANG so du con lai (70000) van thanh cong, ve 0; tru THEM 1 dong khi so du la 0 bi tu choi -- so du KHONG BAO GIO xuong am."
locale: vi
track: thiet-ke-he-thong
module: giao-dich-va-tien
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [sd.vi-dien-tu-khong-am]
requires: [sd.thanh-toan-so-cai-kep]
concepts: [sd.vi-dien-tu-khong-am]
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
Sổ cái kép chứng minh tổng toàn hệ thống LUÔN bằng `0`. Nhưng một câu
hỏi RIÊNG vẫn còn: MỘT ví, tại một thời điểm, có được phép âm hay
không? Ví điện tử LÀ nơi câu trả lời phải LÀ "không" — VÀ cách kiểm
tra điều đó quan trọng không kém câu trả lời.
::::

::::explain{#kiem-tra-truoc-khi-tru}
`truTien` đọc `vi.soDu` VÀ so sánh VỚI `soTien` cần trừ TRƯỚC khi đụng
vào số dư — nếu KHÔNG đủ, hàm thoát NGAY, số dư giữ nguyên. Chỉ khi
ĐÃ chắc chắn đủ, dòng trừ MỚI chạy:

```typescript title=readonly
interface Vi { chuSoHuu: string; soDu: number; }
interface HeThongVi { cacVi: Map<string, Vi>; }
function taoHeThongVi(): HeThongVi { return { cacVi: new Map() }; }
function moVi(ht: HeThongVi, chuSoHuu: string, soDuBanDau: number): void {
  ht.cacVi.set(chuSoHuu, { chuSoHuu, soDu: soDuBanDau });
}

type KetQuaTru = "da_tru" | "khong_du_so_du";
function truTien(ht: HeThongVi, chuSoHuu: string, soTien: number): KetQuaTru {
  const vi = ht.cacVi.get(chuSoHuu)!;
  if (vi.soDu < soTien) return "khong_du_so_du";
  vi.soDu -= soTien;
  return "da_tru";
}

const ht = taoHeThongVi();
moVi(ht, "an", 100000);
console.log("an tru 30000:", truTien(ht, "an", 30000));
console.log("so du an sau khi tru:", ht.cacVi.get("an")!.soDu);
console.log("an tru THEM 80000 (chi con 70000):", truTien(ht, "an", 80000));
console.log("so du an KHONG doi (van 70000):", ht.cacVi.get("an")!.soDu);
```

```text title=readonly
an tru 30000: da_tru
so du an sau khi tru: 70000
an tru THEM 80000 (chi con 70000): khong_du_so_du
so du an KHONG doi (van 70000): 70000
```

Lần trừ `80000` bị từ chối vì `70000 < 80000` — đúng tại điểm KIỂM
tra, TRƯỚC khi dòng `vi.soDu -= soTien` có cơ hội chạy. Số dư của
`an` giữ NGUYÊN `70000` sau lần bị từ chối — không hề có khoảnh khắc
NÀO nó tụt xuống âm rồi được "sửa lại".
::::

::::example{#bien-tru-dung-het}
Trừ ĐÚNG bằng số dư còn lại vẫn thành công (về `0`), nhưng CHỈ cần
thiếu `1` đồng LÀ bị từ chối — VÀ mỗi ví hoàn toàn ĐỘC lập với ví
khác:

```typescript title=readonly
interface Vi { chuSoHuu: string; soDu: number; }
interface HeThongVi { cacVi: Map<string, Vi>; }
function taoHeThongVi(): HeThongVi { return { cacVi: new Map() }; }
function moVi(ht: HeThongVi, chuSoHuu: string, soDuBanDau: number): void {
  ht.cacVi.set(chuSoHuu, { chuSoHuu, soDu: soDuBanDau });
}

type KetQuaTru = "da_tru" | "khong_du_so_du";
function truTien(ht: HeThongVi, chuSoHuu: string, soTien: number): KetQuaTru {
  const vi = ht.cacVi.get(chuSoHuu)!;
  if (vi.soDu < soTien) return "khong_du_so_du";
  vi.soDu -= soTien;
  return "da_tru";
}

// tai lap dung trang thai tu khoi truoc, KHONG in lai: vi "an" mo voi 100000,
// da tru 30000 (thanh cong), con lai 70000
const ht = taoHeThongVi();
moVi(ht, "an", 100000);
truTien(ht, "an", 30000);

console.log("an tru DUNG BANG so du con lai (70000):", truTien(ht, "an", 70000));
console.log("so du an sau khi tru het:", ht.cacVi.get("an")!.soDu);
console.log("an tru THEM 1 dong (khi so du dang la 0):", truTien(ht, "an", 1));

moVi(ht, "binh", 5000);
console.log("binh (vi RIENG) tru 5000:", truTien(ht, "binh", 5000));
console.log("so du binh:", ht.cacVi.get("binh")!.soDu);
```

```text title=readonly
an tru DUNG BANG so du con lai (70000): da_tru
so du an sau khi tru het: 0
an tru THEM 1 dong (khi so du dang la 0): khong_du_so_du
binh (vi RIENG) tru 5000: da_tru
so du binh: 0
```

`an` trừ đúng `70000` — SỐ dư còn lại — vẫn thành công, về ĐÚNG `0`.
Ngay sau đó, trừ thêm CHỈ `1` đồng bị từ chối NGAY vì `0 < 1`. Ví
`binh` mở với `5000` VÀ trừ hết `5000` hoàn toàn không liên quan gì
tới trạng thái của `an` — mỗi ví LÀ một mục riêng trong `cacVi`.
::::

::::predict{#doan-tru-khong-dong commitOnce}
Một ví MỚI, `"chi"`, mở với `moVi(ht, "chi", 0)` — số dư ban đầu LÀ
`0`. Gọi `truTien(ht, "chi", 0)` (trừ ĐÚNG `0` đồng) — kết quả LÀ gì?

:::opt{correct}
`"da_tru"` — điều kiện từ chối LÀ `vi.soDu < soTien`, tức `0 < 0`,
SAI; hàm đi tiếp, trừ `0` khỏi `0` (vẫn LÀ `0`), VÀ trả về `"da_tru"`
:::
:::opt
`"khong_du_so_du"` — số dư đang LÀ `0` nghĩa LÀ ví này không còn tiền
NÀO cả, nên bất kỳ yêu cầu trừ tiền nào cũng phải bị từ chối
::why
Nhầm "số dư bằng `0`" VỚI "không thể trừ bất cứ gì" — nhưng điều kiện
từ chối chỉ kích hoạt khi số dư THỰC SỰ nhỏ HƠN số tiền cần trừ, không
phải khi số dư bằng `0` một cách chung chung.

Chỗ lệch: `vi.soDu < soTien` với `vi.soDu = 0` VÀ `soTien = 0` LÀ
`0 < 0` — biểu thức NÀY đánh giá LÀ `false`. Vì điều kiện từ chối SAI,
hàm không return sớm — nó chạy tiếp `vi.soDu -= 0`, số dư giữ nguyên
`0`, VÀ trả về `"da_tru"`. Trừ `0` đồng LÀ một thao tác hợp lệ, dù số
dư đang cạn.
::
:::
::::

::::code{#viet_tru_tien}
Hoàn thiện `truTien` — đọc `vi` từ `ht.cacVi`. Nếu `vi.soDu` NHỎ hơn
`soTien` cần trừ, trả về `"khong_du_so_du"` NGAY, không đụng vào số
dư. Ngược lại, trừ `soTien` khỏi `vi.soDu` RỒI trả về `"da_tru"`.

```typescript title=starter
interface Vi { chuSoHuu: string; soDu: number; }
interface HeThongVi { cacVi: Map<string, Vi>; }
function taoHeThongVi(): HeThongVi { return { cacVi: new Map() }; }
function moVi(ht: HeThongVi, chuSoHuu: string, soDuBanDau: number): void {
  ht.cacVi.set(chuSoHuu, { chuSoHuu, soDu: soDuBanDau });
}

type KetQuaTru = "da_tru" | "khong_du_so_du";
function truTien(ht: HeThongVi, chuSoHuu: string, soTien: number): KetQuaTru {
  const vi = ht.cacVi.get(chuSoHuu)!;
  ___
}

const htX = taoHeThongVi();
moVi(htX, "duc", 1000);
console.log(truTien(htX, "duc", 1000), truTien(htX, "duc", 1));
```

```typescript title=solution
interface Vi { chuSoHuu: string; soDu: number; }
interface HeThongVi { cacVi: Map<string, Vi>; }
function taoHeThongVi(): HeThongVi { return { cacVi: new Map() }; }
function moVi(ht: HeThongVi, chuSoHuu: string, soDuBanDau: number): void {
  ht.cacVi.set(chuSoHuu, { chuSoHuu, soDu: soDuBanDau });
}

type KetQuaTru = "da_tru" | "khong_du_so_du";
function truTien(ht: HeThongVi, chuSoHuu: string, soTien: number): KetQuaTru {
  const vi = ht.cacVi.get(chuSoHuu)!;
  if (vi.soDu < soTien) return "khong_du_so_du";
  vi.soDu -= soTien;
  return "da_tru";
}

const htX = taoHeThongVi();
moVi(htX, "duc", 1000);
console.log(truTien(htX, "duc", 1000), truTien(htX, "duc", 1));
```

```typescript title=test
const htT = taoHeThongVi();
moVi(htT, "e1", 100);
const r1 = truTien(htT, "e1", 40);
if (r1 !== "da_tru") throw new Error("tru khi CON du tien phai thanh cong");
const soDu1 = htT.cacVi.get("e1")!.soDu;
if (soDu1 !== 60) throw new Error("sau khi tru 40 tu 100, so du phai la 60");

const r2 = truTien(htT, "e1", 60);
if (r2 !== "da_tru") throw new Error("tru DUNG BANG so du con lai phai thanh cong");
const soDu2 = htT.cacVi.get("e1")!.soDu;
if (soDu2 !== 0) throw new Error("sau khi tru het, so du phai la 0");

const r3 = truTien(htT, "e1", 1);
if (r3 !== "khong_du_so_du") throw new Error("tru khi so du la 0 phai bi tu choi");
const soDu3 = htT.cacVi.get("e1")!.soDu;
if (soDu3 !== 0) throw new Error("so du KHONG duoc xuong am, phai giu nguyen 0");

moVi(htT, "e2", 0);
const r4 = truTien(htT, "e2", 0);
if (r4 !== "da_tru") throw new Error("tru DUNG 0 dong khi so du la 0 van phai thanh cong (0 < 0 la sai)");

moVi(htT, "e3", 500);
const r5 = truTien(htT, "e3", 200);
if (r5 !== "da_tru") throw new Error("vi rieng (e3) khong bi anh huong boi vi khac");
const soDuE1 = htT.cacVi.get("e1")!.soDu;
if (soDuE1 !== 0) throw new Error("vi e1 phai KHONG doi khi thao tac tren vi e3");
```

:::hints
- kind: attention
  body: "So sanh vi.soDu voi soTien TRUOC: if (vi.soDu < soTien) return 'khong_du_so_du'. Chi khi du, moi tru: vi.soDu -= soTien; roi return 'da_tru'."
- kind: strategy
  body: "if (vi.soDu < soTien) return 'khong_du_so_du'; vi.soDu -= soTien; return 'da_tru';"
- kind: one-line
  body: "if (vi.soDu < soTien) return \"khong_du_so_du\"; vi.soDu -= soTien; return \"da_tru\";"
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
Một ví, kiểm tra trước khi trừ — số dư không bao giờ âm. Nhưng chuyển
tiền lại cần HAI thao tác trên HAI ví khác nhau. Nếu thao tác thứ hai
thất bại giữa chừng, thao tác đầu có bị bỏ dở dang không?
::::

::::reflect{#nghi-lai}
`truTien` không hề "trừ rồi kiểm tra lại" — thứ tự NGƯỢC lại đó (trừ
trước, phát hiện âm thì cộng bù) sẽ để LỘ một khoảnh khắc số dư SAI,
dù chỉ tồn tại trong bộ nhớ một phần triệu giây. Kiểm tra TRƯỚC khi
trừ loại bỏ hoàn toàn khoảnh khắc đó — số dư chỉ có đúng HAI trạng
thái: chưa đổi (bị từ chối), hoặc đã trừ ĐÚNG (thành công), không có
trạng thái thứ BA nào ở giữa.
::::

::::checkpoint{mastery=0.74}
::::
