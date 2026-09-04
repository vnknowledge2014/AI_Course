---
id: thiet-ke-he-thong.crdt-va-hop-nhat.so-sanh-chinh-sach-hop-nhat
title: "Ba chính sách hợp nhất: LWW mất dữ liệu, G-Counter/OR-Set giữ lại"
summary: "hai cap nhat CUNG thoi diem tu hai replica -- mo hinh bang LWWRegister: hopNhatLWW chi GIU MOT (thang), ben THUA bien mat hoan toan khong con dau vet; mo hinh CUNG hai cap nhat DO bang GCounter: hopNhatGCounter GIU CA HAI, tong dung khong mat gi -- soLuongCapNhatBiMat(a,b) tra ve gia tri BEN THUA khi hop nhat kieu LWW, dung de do truc tiep luong du lieu mat; ket luan: chon CRDT nao phu thuoc domain co CHAP NHAN mat cap nhat 'thua' hay khong."
locale: vi
track: thiet-ke-he-thong
module: crdt-va-hop-nhat
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [sd.fp.so-sanh-chinh-sach-hop-nhat]
requires: [sd.fp.lww-register]
concepts: [sd.fp.so-sanh-chinh-sach-hop-nhat]
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
Bảy bài, năm CRDT — `GCounter`, `PNCounter`, `GSet`, `ORSet`,
`LWWRegister`. Bốn cái đầu đều hợp nhất bằng cách GIỮ LẠI mọi đóng
góp, dưới dạng nào đó. `LWWRegister` thì khác hẳn: hợp nhất LÀ một
phép CHỌN — kết quả chỉ là MỘT trong hai bản ghi, bên kia biến mất.
Đặt hai kiểu hợp nhất đó cạnh nhau, trên CÙNG một tình huống, để thấy
rõ cái giá phải trả.
::::

::::explain{#lww-mat-du-lieu-gcounter-thi-khong}
Hai replica CÙNG lúc ghi nhận một cập nhật "số lượt thích" — mỗi bên
đóng góp một con số. Mô hình bằng `LWWRegister`: hai cập nhật CÙNG
`thoiDiem`, `hopNhat` phải CHỌN một, bên còn lại mất trắng. Mô hình
CÙNG tình huống đó bằng `GCounter`: cả hai đóng góp đều Ở LẠI:

```typescript title=readonly
interface GCounter { theoReplica: Record<string, number>; }
function counterRong(): GCounter { return { theoReplica: {} }; }
function tangGCounter(gc: GCounter, replicaId: string, buoc: number): GCounter {
  const hienTai = gc.theoReplica[replicaId] ?? 0;
  return { theoReplica: { ...gc.theoReplica, [replicaId]: hienTai + buoc } };
}
function giaTriGCounter(gc: GCounter): number {
  return Object.values(gc.theoReplica).reduce((tong, n) => tong + n, 0);
}
function hopNhatGCounter(a: GCounter, b: GCounter): GCounter {
  const ketQua: Record<string, number> = { ...a.theoReplica };
  for (const [replicaId, soDem] of Object.entries(b.theoReplica)) {
    const hienTai = ketQua[replicaId] ?? 0;
    ketQua[replicaId] = Math.max(hienTai, soDem);
  }
  return { theoReplica: ketQua };
}

interface LWWSo { giaTri: number; thoiDiem: number; replicaId: string; }
function hopNhatLWW(a: LWWSo, b: LWWSo): LWWSo {
  if (a.thoiDiem !== b.thoiDiem) return a.thoiDiem > b.thoiDiem ? a : b;
  return a.replicaId > b.replicaId ? a : b;
}

const capNhatA: LWWSo = { giaTri: 5, thoiDiem: 1000, replicaId: "server-a" };
const capNhatB: LWWSo = { giaTri: 3, thoiDiem: 1000, replicaId: "server-b" };
const ketQuaLWW = hopNhatLWW(capNhatA, capNhatB);
console.log("LWW hop nhat 2 cap nhat CUNG thoi diem -> chi giu MOT:", ketQuaLWW.giaTri, "(tu", ketQuaLWW.replicaId + ")");
const biMat = ketQuaLWW === capNhatA ? capNhatB.giaTri : capNhatA.giaTri;
console.log("cap nhat CON LAI bi mat hoan toan, khong con dau vet:", biMat);

let gcA = tangGCounter(counterRong(), "server-a", 5);
let gcB = tangGCounter(counterRong(), "server-b", 3);
const ketQuaGCounter = hopNhatGCounter(gcA, gcB);
console.log("GCounter hop nhat CUNG hai cap nhat -> giu CA HAI:", JSON.stringify(ketQuaGCounter.theoReplica));
console.log("tong gia tri GCounter (khong mat gi):", giaTriGCounter(ketQuaGCounter));
```

```text title=readonly
LWW hop nhat 2 cap nhat CUNG thoi diem -> chi giu MOT: 3 (tu server-b)
cap nhat CON LAI bi mat hoan toan, khong con dau vet: 5
GCounter hop nhat CUNG hai cap nhat -> giu CA HAI: {"server-a":5,"server-b":3}
tong gia tri GCounter (khong mat gi): 8
```

CÙNG một tình huống — hai server đóng góp `5` VÀ `3` — nhưng hai
cách mô hình cho hai kết quả khác hẳn nhau. `LWWRegister` xem đây là
"ghi đè một giá trị", nên chỉ giữ MỘT (`3`, thắng nhờ tie-break
`replicaId`), phần `5` của `server-a` biến mất, không cách nào phục
hồi. `GCounter` xem đây là "hai đóng góp độc lập cần CỘNG lại", giữ
CẢ hai, tổng đúng `8`. Không cách nào SAI về mặt code — cả hai đều
hợp nhất đúng luật của CHÍNH chúng. Vấn đề nằm Ở việc CHỌN đúng mô
hình cho đúng domain.
::::

::::example{#lww-khong-quan-tam-do-lon-chi-quan-tam-thoi-gian}
`LWWRegister` không so sánh GIÁ TRỊ — dù giá trị "thua" có lớn hơn
nhiều lần giá trị "thắng", nó vẫn thua nếu đến SỚM hơn theo
`thoiDiem`:

```typescript title=readonly
interface LWWSo { giaTri: number; thoiDiem: number; replicaId: string; }
function hopNhatLWW(a: LWWSo, b: LWWSo): LWWSo {
  if (a.thoiDiem !== b.thoiDiem) return a.thoiDiem > b.thoiDiem ? a : b;
  return a.replicaId > b.replicaId ? a : b;
}

const capNhatCu: LWWSo = { giaTri: 999, thoiDiem: 100, replicaId: "server-a" };
const capNhatMoi: LWWSo = { giaTri: 1, thoiDiem: 101, replicaId: "server-b" };
console.log("hopNhat(cu=999,moi=1):", JSON.stringify(hopNhatLWW(capNhatCu, capNhatMoi)));
console.log("gia tri LON HON (999) bi mat vi den TRUOC ve thoi gian");
```

```text title=readonly
hopNhat(cu=999,moi=1): {"giaTri":1,"thoiDiem":101,"replicaId":"server-b"}
gia tri LON HON (999) bi mat vi den TRUOC ve thoi gian
```

`999` là con số LỚN hơn nhiều so với `1`, nhưng `hopNhatLWW` không
hề nhìn vào `giaTri` — nó chỉ so `thoiDiem`. `capNhatCu` (`101`
mili-giây SỚM hơn) thua, dù giá trị của nó "trông hợp lý hơn". Đây là
đặc điểm — không phải lỗi — của LWW: quy tắc DUY NHẤT là "gần đây
nhất thắng", không có khái niệm "giá trị tốt hơn". Trong khi đó,
`OR-Set` (bài 6) VÀ `GCounter` (bài 1) không hề phải CHỌN — chúng
giữ lại lịch sử đầy đủ (`themVao` cộng `xoaDi`, hoặc `theoReplica`
theo từng ô), nên không có khái niệm "thắng/thua" nào cả.
::::

::::predict{#doan-thoi-diem-quyet-dinh-khong-phai-do-lon commitOnce}
`cn1 = { giaTri: 10, thoiDiem: 50, replicaId: "r1" }` VÀ `cn2 = {
giaTri: 2, thoiDiem: 60, replicaId: "r2" }`. `hopNhat(cn1,
cn2).giaTri` là bao nhiêu?

:::opt{correct}
`2` — `cn2` có `thoiDiem: 60`, LỚN hơn `cn1.thoiDiem: 50`, nên
`cn2` thắng bất kể `giaTri` của nó (`2`) nhỏ hơn `giaTri` của `cn1`
(`10`)
:::
:::opt
`10` — vì `hopNhat` LWW nên ưu tiên giữ giá trị LỚN hơn khi phải
chọn một, giống cách `GCounter` giữ giá trị lớn hơn bằng `Math.max`
::why
Nhầm luật của `GCounter` (giữ giá trị LỚN hơn) với luật của
`LWWRegister` (giữ giá trị MỚI hơn) — đây là chính xác điểm khác
biệt bài học này muốn làm rõ.

Chỗ lệch: `hopNhatLWW` không hề gọi `Math.max` trên `giaTri` — dòng
`if (a.thoiDiem !== b.thoiDiem) return a.thoiDiem > b.thoiDiem ? a :
b;` chỉ so `thoiDiem`. `cn2.thoiDiem = 60` lớn hơn `cn1.thoiDiem =
50`, nên hàm trả về NGUYÊN `cn2` — kèm theo toàn bộ `giaTri: 2` của
nó, dù con số đó nhỏ hơn `10`. LWW không có khái niệm "giá trị tốt
hơn", chỉ có "bản ghi gần đây hơn".
::
:::
::::

::::code{#viet_so_luong_cap_nhat_bi_mat}
Hoàn thiện `soLuongCapNhatBiMat` — dùng `hopNhatLWW` (đã có) để tìm
bản ghi THẮNG, rồi trả về `giaTri` của bản ghi CÒN LẠI (bên THUA,
phần dữ liệu sẽ biến mất sau khi hợp nhất).

```typescript title=starter
interface LWWSo { giaTri: number; thoiDiem: number; replicaId: string; }
function hopNhatLWW(a: LWWSo, b: LWWSo): LWWSo {
  if (a.thoiDiem !== b.thoiDiem) return a.thoiDiem > b.thoiDiem ? a : b;
  return a.replicaId > b.replicaId ? a : b;
}

function soLuongCapNhatBiMat(a: LWWSo, b: LWWSo): number {
  ___
}

const eX: LWWSo = { giaTri: 40, thoiDiem: 5, replicaId: "x" };
const eY: LWWSo = { giaTri: 70, thoiDiem: 9, replicaId: "y" };
console.log(soLuongCapNhatBiMat(eX, eY), soLuongCapNhatBiMat(eY, eX));
```

```typescript title=solution
interface LWWSo { giaTri: number; thoiDiem: number; replicaId: string; }
function hopNhatLWW(a: LWWSo, b: LWWSo): LWWSo {
  if (a.thoiDiem !== b.thoiDiem) return a.thoiDiem > b.thoiDiem ? a : b;
  return a.replicaId > b.replicaId ? a : b;
}

function soLuongCapNhatBiMat(a: LWWSo, b: LWWSo): number {
  const thang = hopNhatLWW(a, b);
  return thang === a ? b.giaTri : a.giaTri;
}

const eX: LWWSo = { giaTri: 40, thoiDiem: 5, replicaId: "x" };
const eY: LWWSo = { giaTri: 70, thoiDiem: 9, replicaId: "y" };
console.log(soLuongCapNhatBiMat(eX, eY), soLuongCapNhatBiMat(eY, eX));
```

```typescript title=test
const p1: LWWSo = { giaTri: 100, thoiDiem: 1, replicaId: "p" };
const p2: LWWSo = { giaTri: 200, thoiDiem: 2, replicaId: "q" };
if (soLuongCapNhatBiMat(p1, p2) !== 100) throw new Error("p1 co thoiDiem nho hon, phai la ben THUA -- gia tri bi mat la 100");
if (soLuongCapNhatBiMat(p2, p1) !== 100) throw new Error("doi thu tu tham so khong doi ben nao THUA -- van la p1 (100)");

const hoa1: LWWSo = { giaTri: 11, thoiDiem: 7, replicaId: "aa" };
const hoa2: LWWSo = { giaTri: 22, thoiDiem: 7, replicaId: "zz" };
if (soLuongCapNhatBiMat(hoa1, hoa2) !== 11) throw new Error("hoa thoiDiem, tie-break bang replicaId (zz > aa) -- hoa1 (11) phai la ben THUA");

const truoc1 = JSON.stringify(p1);
const truoc2 = JSON.stringify(p2);
soLuongCapNhatBiMat(p1, p2);
if (JSON.stringify(p1) !== truoc1) throw new Error("KHONG duoc mutate a truyen vao");
if (JSON.stringify(p2) !== truoc2) throw new Error("KHONG duoc mutate b truyen vao");
```

:::hints
- kind: attention
  body: "Goi hopNhatLWW(a, b) de biet BEN NAO thang. Neu ket qua CHINH LA object a (so sanh ===), ben thua la b, tra ve b.giaTri. Nguoc lai ben thua la a, tra ve a.giaTri."
- kind: strategy
  body: "const thang = hopNhatLWW(a, b); return thang === a ? b.giaTri : a.giaTri;"
- kind: one-line
  body: "const thang = hopNhatLWW(a, b); return thang === a ? b.giaTri : a.giaTri;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "40 40"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
LWW mất bên thua vĩnh viễn; G-Counter và OR-Set không mất gì, đổi lại
phải giữ nhiều dữ liệu hơn. Không có lựa chọn nào "đúng tuyệt đối" —
chỉ có lựa chọn PHÙ HỢP với domain. Giờ ráp một CRDT vào một tình
huống thực chiến để thấy sự đánh đổi đó vận hành thế nào ngoài ví dụ
tí hon.
::::

::::reflect{#nghi-lai}
Ba chính sách, ba mức giữ lại dữ liệu: `LWWRegister` giữ ĐÚNG một
bản ghi (đơn giản nhất, mất nhiều nhất); `GCounter`/`PNCounter` giữ
một con số CHO MỖI replica (không mất gì, nhưng chỉ áp dụng được cho
số cộng dồn); `OR-Set` giữ TOÀN bộ lịch sử thêm/xoá qua tag (không
mất gì, áp dụng được cho tập hợp bất kỳ, nhưng tốn bộ nhớ nhất). Câu
hỏi "CRDT nào đúng" không có câu trả lời chung — câu hỏi ĐÚNG là:
"domain này có chấp nhận một cập nhật 'thua' biến mất hoàn toàn hay
không?" Tên hiển thị — chấp nhận được. Số dư tài khoản — không bao
giờ.
::::

::::checkpoint{mastery=0.82}
::::
