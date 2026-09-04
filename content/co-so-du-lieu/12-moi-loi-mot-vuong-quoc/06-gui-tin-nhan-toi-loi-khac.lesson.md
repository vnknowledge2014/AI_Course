---
id: co-so-du-lieu.moi-loi-mot-vuong-quoc.gui-tin-nhan-toi-loi-khac
title: "Khi khoá ở lõi khác — gửi tin nhắn"
summary: "guiYeuCauDoc KHÔNG đọc trực tiếp Map của lõi khác — nó đẩy một yêu cầu vào HÀNG ĐỢI của đúng lõi sở hữu khoá đó (chonLoiChoKhoa, bài 4). xuLyHangDoiCuaLoi (đã cho sẵn) là việc CHÍNH lõi sở hữu tự làm: đọc hàng đợi của MÌNH, tra trong kho của MÌNH — không lõi nào đọc bộ nhớ của lõi khác, kể cả khi 'hỏi' dữ liệu của nhau."
locale: vi
track: co-so-du-lieu
module: moi-loi-mot-vuong-quoc
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.gui-tin-nhan-toi-loi-khac]
requires: [db.khong-can-khoa-nua]
concepts: [db.gui-tin-nhan-toi-loi-khac]
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
Lõi `0` cần ĐỌC `"nguoidung0"` — NHƯNG khoá ĐÓ thuộc VỀ lõi `2` (bài
4). Share-nothing CẤM lõi `0` đọc thẳng `Map` của lõi `2`. VẬY làm
sao?
::::

::::explain{#gui-yeu-cau-doc}
`guiYeuCauDoc` KHÔNG đọc trực TIẾP — nó đẩy một "yêu CẦU" vào HÀNG
đợi CỦA đúng lõi sở hữu khoá đó, giống HỆT gửi một tin NHẮN:

```typescript title=readonly
function bam(s: string): number {
  let h = 2166136261;
  for (let i = 0; i < s.length; i++) {
    h ^= s.charCodeAt(i);
    h = Math.imul(h, 16777619);
  }
  h ^= h >>> 16;
  h = Math.imul(h, 0x85ebca6b);
  h ^= h >>> 13;
  h = Math.imul(h, 0xc2b2ae35);
  h ^= h >>> 16;
  return Math.abs(h) % 1000;
}
function chonLoiChoKhoa(khoa: string, soLoi: number): number {
  return bam(khoa) % soLoi;
}
function taoKhoChoMoiLoi(soLoi: number): Map<string, number>[] {
  const khoCacLoi: Map<string, number>[] = [];
  for (let i = 0; i < soLoi; i++) khoCacLoi.push(new Map());
  return khoCacLoi;
}
function ghiVaoLoiDungCua(khoCacLoi: Map<string, number>[], khoa: string, giaTri: number): void {
  const chiSoLoi = chonLoiChoKhoa(khoa, khoCacLoi.length);
  khoCacLoi[chiSoLoi]!.set(khoa, giaTri);
}

interface YeuCau { khoa: string; }

function taoHangDoiChoMoiLoi(soLoi: number): YeuCau[][] {
  const hangDoi: YeuCau[][] = [];
  for (let i = 0; i < soLoi; i++) hangDoi.push([]);
  return hangDoi;
}

function guiYeuCauDoc(hangDoiCacLoi: YeuCau[][], khoa: string): void {
  const chiSoLoi = chonLoiChoKhoa(khoa, hangDoiCacLoi.length);
  hangDoiCacLoi[chiSoLoi]!.push({ khoa });
}

function xuLyHangDoiCuaLoi(khoCacLoi: Map<string, number>[], hangDoiCacLoi: YeuCau[][], chiSoLoi: number): (number | undefined)[] {
  const ketQua: (number | undefined)[] = [];
  for (const yeuCau of hangDoiCacLoi[chiSoLoi]!) {
    ketQua.push(khoCacLoi[chiSoLoi]!.get(yeuCau.khoa));
  }
  return ketQua;
}

const kho = taoKhoChoMoiLoi(4);
ghiVaoLoiDungCua(kho, "nguoidung0", 100);
ghiVaoLoiDungCua(kho, "nguoidung2", 300);

const hangDoi = taoHangDoiChoMoiLoi(4);
guiYeuCauDoc(hangDoi, "nguoidung0");
guiYeuCauDoc(hangDoi, "nguoidung2");

console.log(hangDoi[2]!.length);
console.log(xuLyHangDoiCuaLoi(kho, hangDoi, 2));
```

```text title=readonly
2
[ 100, 300 ]
```

CẢ `guiYeuCauDoc` LẪN `xuLyHangDoiCuaLoi` đều dùng ĐÚNG `chonLoiChoKhoa`
để xác định "khoá NÀY thuộc lõi nào" — `guiYeuCauDoc` để BIẾT đẩy yêu
cầu VÀO hàng đợi nào, `xuLyHangDoiCuaLoi` (đã cho SẴN) để lõi sở hữu
tự TRA đúng kho của CHÍNH nó. Hai hàng KHÔNG hề đọc `Map` HAY hàng
đợi của lõi khác.
::::

::::example{#khoa-khong-ton-tai-cung-co-lo-di-dung}
`guiYeuCauDoc` VẪN routing được MỘT khoá chưa TỪNG được ghi:

```typescript title=readonly
guiYeuCauDoc(hangDoi, "khong_ton_tai");
console.log(chonLoiChoKhoa("khong_ton_tai", 4));
console.log(hangDoi[1]!.length);
console.log(xuLyHangDoiCuaLoi(kho, hangDoi, 1));
```

```text title=readonly
1
1
[ undefined ]
```

`"khong_ton_tai"` route VỀ lõi `1` — MỘT lõi HOÀN toàn khác `2`. Hàng
đợi CỦA lõi `1` nhận đúng MỘT yêu cầu, VÀ khi lõi `1` tự XỬ lý (tra
TRONG kho của CHÍNH nó), nó không TÌM thấy gì — trả VỀ `undefined`,
KHÔNG lỗi. Routing hoạt ĐỘNG y hệt cho khoá tồn TẠI lẫn khoá KHÔNG.
::::

::::predict{#doan-loi-hoi-chinh-no commitOnce}
Lõi `2` (chính NÓ sở hữu `"nguoidung0"`) TỰ gọi
`guiYeuCauDoc(hangDoi, "nguoidung0")` — thay VÌ đọc thẳng `kho[2]`.
Yêu cầu ĐÓ CÓ được xử lý ĐÚNG không, hay LÀ một lỗi ("gửi tin nhắn
CHO chính mình")?

:::opt{correct}
Vẫn ĐÚNG bình thường — yêu cầu được đẩy VÀO `hangDoi[2]` (routing
KHÔNG quan tâm "AI gửi", chỉ quan TÂM khoá thuộc lõi NÀO), lõi `2`
xử lý hàng đợi CỦA chính nó NHƯ mọi yêu cầu KHÁC
:::

:::opt
LÀ một lỗi thiết KẾ — một lõi KHÔNG cần gửi tin nhắn CHO chính nó, nó
CÓ thể đọc thẳng kho của MÌNH, `guiYeuCauDoc` PHẢI kiểm tra VÀ chặn
trường hợp NÀY
::why
Gần đúng ở việc bạn nhận RA lõi `2` HOÀN toàn CÓ thể đọc thẳng
`kho[2]` (đúng — nó LÀ chủ SỞ hữu) — MỘT quan sát tối ưu HOÁ hợp lý
CHO trường hợp riêng NÀY.

Chỗ lệch: `guiYeuCauDoc`/`chonLoiChoKhoa` KHÔNG hề biết (VÀ không CẦN
biết) "ai đang GỌI hàm" — chúng chỉ nhìn VÀO khoá RỒI tính chỉ số lõi.
Gửi yêu CẦU "cho chính mình" không phải lỗi — nó CHỈ đơn giản LÀ một
yêu cầu BÌNH thường được routing ĐÚNG (VỀ đúng lõi `2`), rồi được XỬ
lý bình thường TRONG hàng đợi của lõi `2`. KHÔNG cần một nhánh XỬ lý
đặc biệt "nếu người gửi trùng người NHẬN" — routing tất định (bài 4)
đã tự nhiên xử LÝ đúng cả ca NÀY mà không CẦN thêm mã nào.
::
:::
::::

::::code{#viet_gui_yeu_cau_doc}
Hoàn thiện `guiYeuCauDoc` — tìm chỉ số lõi ĐÚNG cho khoá, RỒI đẩy một
yêu cầu (`{ khoa }`) VÀO hàng đợi của đúng lõi đó.

```typescript title=starter
function bam(s: string): number {
  let h = 2166136261;
  for (let i = 0; i < s.length; i++) {
    h ^= s.charCodeAt(i);
    h = Math.imul(h, 16777619);
  }
  h ^= h >>> 16;
  h = Math.imul(h, 0x85ebca6b);
  h ^= h >>> 13;
  h = Math.imul(h, 0xc2b2ae35);
  h ^= h >>> 16;
  return Math.abs(h) % 1000;
}
function chonLoiChoKhoa(khoa: string, soLoi: number): number {
  return bam(khoa) % soLoi;
}
function taoKhoChoMoiLoi(soLoi: number): Map<string, number>[] {
  const khoCacLoi: Map<string, number>[] = [];
  for (let i = 0; i < soLoi; i++) khoCacLoi.push(new Map());
  return khoCacLoi;
}
function ghiVaoLoiDungCua(khoCacLoi: Map<string, number>[], khoa: string, giaTri: number): void {
  const chiSoLoi = chonLoiChoKhoa(khoa, khoCacLoi.length);
  khoCacLoi[chiSoLoi]!.set(khoa, giaTri);
}

interface YeuCau { khoa: string; }

function taoHangDoiChoMoiLoi(soLoi: number): YeuCau[][] {
  const hangDoi: YeuCau[][] = [];
  for (let i = 0; i < soLoi; i++) hangDoi.push([]);
  return hangDoi;
}

function guiYeuCauDoc(hangDoiCacLoi: YeuCau[][], khoa: string): void {
  const chiSoLoi = chonLoiChoKhoa(khoa, hangDoiCacLoi.length);
  ___
}

function xuLyHangDoiCuaLoi(khoCacLoi: Map<string, number>[], hangDoiCacLoi: YeuCau[][], chiSoLoi: number): (number | undefined)[] {
  const ketQua: (number | undefined)[] = [];
  for (const yeuCau of hangDoiCacLoi[chiSoLoi]!) {
    ketQua.push(khoCacLoi[chiSoLoi]!.get(yeuCau.khoa));
  }
  return ketQua;
}

const kho = taoKhoChoMoiLoi(4);
ghiVaoLoiDungCua(kho, "nguoidung0", 100);
const hangDoi = taoHangDoiChoMoiLoi(4);
guiYeuCauDoc(hangDoi, "nguoidung0");
console.log(hangDoi[2]!.length);
```

```typescript title=solution
function bam(s: string): number {
  let h = 2166136261;
  for (let i = 0; i < s.length; i++) {
    h ^= s.charCodeAt(i);
    h = Math.imul(h, 16777619);
  }
  h ^= h >>> 16;
  h = Math.imul(h, 0x85ebca6b);
  h ^= h >>> 13;
  h = Math.imul(h, 0xc2b2ae35);
  h ^= h >>> 16;
  return Math.abs(h) % 1000;
}
function chonLoiChoKhoa(khoa: string, soLoi: number): number {
  return bam(khoa) % soLoi;
}
function taoKhoChoMoiLoi(soLoi: number): Map<string, number>[] {
  const khoCacLoi: Map<string, number>[] = [];
  for (let i = 0; i < soLoi; i++) khoCacLoi.push(new Map());
  return khoCacLoi;
}
function ghiVaoLoiDungCua(khoCacLoi: Map<string, number>[], khoa: string, giaTri: number): void {
  const chiSoLoi = chonLoiChoKhoa(khoa, khoCacLoi.length);
  khoCacLoi[chiSoLoi]!.set(khoa, giaTri);
}

interface YeuCau { khoa: string; }

function taoHangDoiChoMoiLoi(soLoi: number): YeuCau[][] {
  const hangDoi: YeuCau[][] = [];
  for (let i = 0; i < soLoi; i++) hangDoi.push([]);
  return hangDoi;
}

function guiYeuCauDoc(hangDoiCacLoi: YeuCau[][], khoa: string): void {
  const chiSoLoi = chonLoiChoKhoa(khoa, hangDoiCacLoi.length);
  hangDoiCacLoi[chiSoLoi]!.push({ khoa });
}

function xuLyHangDoiCuaLoi(khoCacLoi: Map<string, number>[], hangDoiCacLoi: YeuCau[][], chiSoLoi: number): (number | undefined)[] {
  const ketQua: (number | undefined)[] = [];
  for (const yeuCau of hangDoiCacLoi[chiSoLoi]!) {
    ketQua.push(khoCacLoi[chiSoLoi]!.get(yeuCau.khoa));
  }
  return ketQua;
}

const kho = taoKhoChoMoiLoi(4);
ghiVaoLoiDungCua(kho, "nguoidung0", 100);
const hangDoi = taoHangDoiChoMoiLoi(4);
guiYeuCauDoc(hangDoi, "nguoidung0");
console.log(hangDoi[2]!.length);
```

```typescript title=test
const kho2 = taoKhoChoMoiLoi(4);
ghiVaoLoiDungCua(kho2, "nguoidung0", 100);
ghiVaoLoiDungCua(kho2, "nguoidung2", 300);
const hangDoi2 = taoHangDoiChoMoiLoi(4);
guiYeuCauDoc(hangDoi2, "nguoidung0");
guiYeuCauDoc(hangDoi2, "nguoidung2");
console.log(hangDoi2[2]!.length);
if (hangDoi2[2]!.length !== 2) throw new Error("hai khoa cung route ve loi 2 phai tao dung 2 yeu cau trong hang doi loi 2");
if (hangDoi2[0]!.length !== 0) throw new Error("loi 0 khong lien quan, hang doi phai rong");

const ketQua2 = xuLyHangDoiCuaLoi(kho2, hangDoi2, 2);
if (ketQua2[0] !== 100) throw new Error("yeu cau dau tien (nguoidung0) phai tra ve 100");
if (ketQua2[1] !== 300) throw new Error("yeu cau thu hai (nguoidung2) phai tra ve 300");

guiYeuCauDoc(hangDoi2, "khong_ton_tai");
if (hangDoi2[2]!.length !== 2) throw new Error("khong_ton_tai KHONG duoc route ve loi 2 (no route ve loi 1)");
```

:::hints
- kind: attention
  body: "Day mot yeu cau ({ khoa }) vao dung hang doi cua chiSoLoi -- mot dong."
- kind: strategy
  body: "hangDoiCacLoi[chiSoLoi]!.push({ khoa });"
- kind: one-line
  body: "hangDoiCacLoi[chiSoLoi]!.push({ khoa });"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Gửi tin nhắn thay VÌ đọc trực tiếp — hệ thống VẪN đúng. NHƯNG chia
khoá RA lõi có LUÔN đều không?
::::

::::reflect{#nghi-lai}
`guiYeuCauDoc`/`xuLyHangDoiCuaLoi` LÀ phiên bản THU nhỏ, Ở quy mô
LÕI, của đúng cách CÁC máy giao tiếp qua MẠNG (q11): KHÔNG có bộ NHỚ
chia sẻ, chỉ CÓ tin nhắn VÀ hàng đợi. `chonLoiChoKhoa` LÀ "địa chỉ"
— nó QUYẾT định tin nhắn ĐI đâu, dù người GỬI LÀ ai (kể cả chính lõi
sở hữu tự gửi CHO mình, dự đoán Ở trên đã xác nhận). MỌI mảnh Ở đây
— kho RIÊNG (bài 3), routing (bài 4), tin NHẮN (bài NÀY) — đều ĐÃ
đúng. NHƯNG "routing tất định" (`bam(khoa) % soLoi`) đã TỪNG có vấn
đề Ở q11 (bài 7) — LIỆU vấn đề ĐÓ có quay LẠI Ở quy mô lõi không?
::::

::::checkpoint{mastery=0.85}
::::
</content>
