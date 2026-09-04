---
id: thiet-ke-he-thong.ha-tang-du-lieu-quy-mo-lon.message-queue-xac-nhan-va-gui-lai
title: "ACK và giao lại: một tin có thể tới hai lần"
summary: "nhanTinNhan(hd, dh) giao mot tin dang 'cho_giao' cho consumer, dat han ACK -- neu KHONG xacNhan truoc han (consumer 'chet'), lan goi nhanTinNhan SAU do se QUET lai, dua tin ve 'cho_giao' roi giao TIEP cho consumer khac, tang soLanDaGiao. Vi du that: don-1 giao cho A (soLanDaGiao=1), A khong ACK, qua han -> giao lai cho B (soLanDaGiao=2) -- CHUNG MINH at-least-once, mot tin co the duoc XU LY nhieu lan, khong phai dung mot lan."
locale: vi
track: thiet-ke-he-thong
module: ha-tang-du-lieu-quy-mo-lon
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [sd.message-queue-xac-nhan-va-gui-lai]
requires: [sd.message-queue-phan-vung]
concepts: [sd.message-queue-xac-nhan-va-gui-lai]
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
Bài trước xác định tin nhắn thuộc VỀ phân vùng nào. Nhưng "nằm TRONG
hàng đợi" khác hẳn "đã được XỬ lý xong" — consumer phải xác NHẬN
(acknowledge) rồi tin nhắn mới thật sự coi LÀ xong việc. Nếu consumer
"chết" giữa chừng thì SAO?
::::

::::explain{#ack-va-giao-lai}
Một tin nhắn KHÔNG bị xoá khỏi hàng đợi ngay lúc GIAO cho consumer — nó
chỉ chuyển sang `"da_giao_cho_ack"` VÀ nhận một hạn ACK
(`dh.thoiGianHienTai + NGUONG_ACK_MS`). Mỗi lần `nhanTinNhan` được gọi,
nó QUÉT trước: bất kỳ tin nào đang `"da_giao_cho_ack"` mà đã QUÁ hạn sẽ
bị đưa VỀ `"cho_giao"` — sẵn sàng giao LẠI cho một consumer khác:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

type TrangThaiTinNhan = "cho_giao" | "da_giao_cho_ack" | "da_ack";
interface TinNhan { id: string; noiDung: string; trangThai: TrangThaiTinNhan; soLanDaGiao: number; hanAck: number; }
interface HangDoiAck { tinNhan: TinNhan[]; }
function taoHangDoiAck(): HangDoiAck { return { tinNhan: [] }; }
function guiVaoHangDoi(hd: HangDoiAck, id: string, noiDung: string): void {
  hd.tinNhan.push({ id, noiDung, trangThai: "cho_giao", soLanDaGiao: 0, hanAck: 0 });
}
const NGUONG_ACK_MS = 5000;
function nhanTinNhan(hd: HangDoiAck, dh: DongHoMoPhong): TinNhan | undefined {
  for (const tn of hd.tinNhan) {
    if (tn.trangThai === "da_giao_cho_ack" && dh.thoiGianHienTai > tn.hanAck) {
      tn.trangThai = "cho_giao";
    }
  }
  const tn = hd.tinNhan.find((t) => t.trangThai === "cho_giao");
  if (tn === undefined) return undefined;
  tn.trangThai = "da_giao_cho_ack";
  tn.hanAck = dh.thoiGianHienTai + NGUONG_ACK_MS;
  tn.soLanDaGiao += 1;
  return tn;
}
function xacNhan(hd: HangDoiAck, id: string): boolean {
  const tn = hd.tinNhan.find((t) => t.id === id);
  if (tn === undefined || tn.trangThai !== "da_giao_cho_ack") return false;
  tn.trangThai = "da_ack";
  return true;
}

const dh = taoDongHoMoPhong();
const hd = taoHangDoiAck();
guiVaoHangDoi(hd, "don-1", "noi dung 1");

const nhan1 = nhanTinNhan(hd, dh);
console.log("consumer A nhan:", nhan1?.id, "so lan da giao:", nhan1?.soLanDaGiao);

tienThoiGian(dh, NGUONG_ACK_MS + 1);
console.log("da qua han ACK, consumer A khong hoi am -- quet lai:");

const nhan2 = nhanTinNhan(hd, dh);
console.log("consumer B nhan LAI CUNG tin nhan:", nhan2?.id, "so lan da giao:", nhan2?.soLanDaGiao);

const ok = xacNhan(hd, "don-1");
console.log("consumer B ACK:", ok, "-- tong so lan da giao (chung minh at-least-once):", hd.tinNhan[0]!.soLanDaGiao);
```

```text title=readonly
consumer A nhan: don-1 so lan da giao: 1
da qua han ACK, consumer A khong hoi am -- quet lai:
consumer B nhan LAI CUNG tin nhan: don-1 so lan da giao: 2
consumer B ACK: true -- tong so lan da giao (chung minh at-least-once): 2
```

`"don-1"` được GIAO đúng `2` lần — một lần cho consumer A (không hề
ACK), một lần cho consumer B (ACK thành công). Hệ thống KHÔNG có cách
nào biết consumer A đã thật sự xử lý xong hay chưa TRƯỚC khi "chết" —
nó chỉ biết KHÔNG có ACK trong hạn, NÊN nó giao lại. Đây chính LÀ
"at-least-once": tin nhắn được XỬ lý ÍT NHẤT một lần, có thể NHIỀU hơn.
::::

::::example{#ack-dung-han-khong-giao-lai}
ACK đến TRƯỚC hạn thì hoàn toàn ngăn được việc giao lại — tin nhắn
chuyển sang `"da_ack"` VÀ vĩnh viễn không còn nằm trong tập `"cho_giao"`
nữa, dù đồng hồ có trôi bao XA:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

type TrangThaiTinNhan = "cho_giao" | "da_giao_cho_ack" | "da_ack";
interface TinNhan { id: string; noiDung: string; trangThai: TrangThaiTinNhan; soLanDaGiao: number; hanAck: number; }
interface HangDoiAck { tinNhan: TinNhan[]; }
function taoHangDoiAck(): HangDoiAck { return { tinNhan: [] }; }
function guiVaoHangDoi(hd: HangDoiAck, id: string, noiDung: string): void {
  hd.tinNhan.push({ id, noiDung, trangThai: "cho_giao", soLanDaGiao: 0, hanAck: 0 });
}
const NGUONG_ACK_MS = 5000;
function nhanTinNhan(hd: HangDoiAck, dh: DongHoMoPhong): TinNhan | undefined {
  for (const tn of hd.tinNhan) {
    if (tn.trangThai === "da_giao_cho_ack" && dh.thoiGianHienTai > tn.hanAck) {
      tn.trangThai = "cho_giao";
    }
  }
  const tn = hd.tinNhan.find((t) => t.trangThai === "cho_giao");
  if (tn === undefined) return undefined;
  tn.trangThai = "da_giao_cho_ack";
  tn.hanAck = dh.thoiGianHienTai + NGUONG_ACK_MS;
  tn.soLanDaGiao += 1;
  return tn;
}
function xacNhan(hd: HangDoiAck, id: string): boolean {
  const tn = hd.tinNhan.find((t) => t.id === id);
  if (tn === undefined || tn.trangThai !== "da_giao_cho_ack") return false;
  tn.trangThai = "da_ack";
  return true;
}

const dh2 = taoDongHoMoPhong();
const hd2 = taoHangDoiAck();
guiVaoHangDoi(hd2, "don-2", "noi dung 2");

const nhan = nhanTinNhan(hd2, dh2);
console.log("consumer nhan don-2:", nhan?.soLanDaGiao);

tienThoiGian(dh2, 1000);
const okAck = xacNhan(hd2, "don-2");
console.log("ACK trong han (t=1000 < 5000):", okAck);

tienThoiGian(dh2, 10000);
const nhanLai = nhanTinNhan(hd2, dh2);
console.log("qua rat lau sau, hoi lai hang doi:", nhanLai);
console.log("so lan da giao van la:", hd2.tinNhan[0]!.soLanDaGiao);

const ackLa = xacNhan(hd2, "id-khong-ton-tai");
console.log("ACK mot id khong ton tai:", ackLa);
```

```text title=readonly
consumer nhan don-2: 1
ACK trong han (t=1000 < 5000): true
qua rat lau sau, hoi lai hang doi: undefined
so lan da giao van la: 1
ACK mot id khong ton tai: false
```

Dù `10000ms` đã trôi qua kể từ lúc ACK — vượt XA `NGUONG_ACK_MS` — tin
nhắn KHÔNG hề bị giao lại, vì vòng quét CHỈ đưa những tin đang
`"da_giao_cho_ack"` VỀ `"cho_giao"`; tin đã `"da_ack"` không hề bị đụng
tới. `soLanDaGiao` mãi mãi dừng Ở `1`.
::::

::::predict{#doan-nhan-trong-han-khong-co-tin commitOnce}
Tin nhắn `"don-3"` vừa được giao cho consumer A (`soLanDaGiao = 1`,
CHƯA ACK). Đồng hồ mới trôi qua `1000ms` (còn cách hạn ACK `4000ms`,
vì `NGUONG_ACK_MS = 5000`). Không còn tin nhắn nào khác trong hàng
đợi. Gọi `nhanTinNhan` NGAY lúc này — trả về gì?

:::opt{correct}
`undefined` — `"don-3"` vẫn đang `"da_giao_cho_ack"` VÀ CHƯA quá hạn,
NÊN vòng quét không đưa nó về `"cho_giao"`; không còn tin nào khác Ở
trạng thái `"cho_giao"` để `find` tìm thấy
:::
:::opt
Chính `"don-3"` đó, giao thêm một LẦN nữa cho một consumer khác — hàng
đợi cho phép NHIỀU consumer cùng xử lý một tin đang chờ ACK để tăng tốc
::why
Nhầm "tin đang chờ ACK" VỚI "tin sẵn sàng giao thêm" — nhưng
`nhanTinNhan` chỉ giao những tin Ở trạng thái `"cho_giao"`, VÀ một tin
chỉ quay VỀ trạng thái đó khi đã QUÁ hạn ACK, không phải NGAY khi có
consumer thứ hai hỏi tới.

Chỗ lệch: điều kiện trong vòng quét LÀ `dh.thoiGianHienTai >
tn.hanAck`. Tại `t=1000`, `tn.hanAck` của `"don-3"` LÀ `5000` (gán lúc
giao, LÀ `0 + NGUONG_ACK_MS`) — `1000 > 5000` SAI, nên `"don-3"` không
hề được đưa về `"cho_giao"`. `find` sau đó không tìm thấy tin nào Ở
trạng thái đó, hàm trả về `undefined`.
::
:::
::::

::::code{#viet_nhan_tin_nhan}
Hoàn thiện `nhanTinNhan` — vòng quét đưa tin QUÁ hạn về `"cho_giao"` VÀ
việc tìm tin sẵn sàng giao (`tn`) đã có sẵn. Còn thiếu: đánh dấu `tn`
sang `"da_giao_cho_ack"`, gán hạn ACK mới, tăng `soLanDaGiao`, RỒI trả
về `tn`.

```typescript title=starter
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

type TrangThaiTinNhan = "cho_giao" | "da_giao_cho_ack" | "da_ack";
interface TinNhan { id: string; noiDung: string; trangThai: TrangThaiTinNhan; soLanDaGiao: number; hanAck: number; }
interface HangDoiAck { tinNhan: TinNhan[]; }
function taoHangDoiAck(): HangDoiAck { return { tinNhan: [] }; }
function guiVaoHangDoi(hd: HangDoiAck, id: string, noiDung: string): void {
  hd.tinNhan.push({ id, noiDung, trangThai: "cho_giao", soLanDaGiao: 0, hanAck: 0 });
}
const NGUONG_ACK_MS = 5000;
function nhanTinNhan(hd: HangDoiAck, dh: DongHoMoPhong): TinNhan | undefined {
  for (const tn of hd.tinNhan) {
    if (tn.trangThai === "da_giao_cho_ack" && dh.thoiGianHienTai > tn.hanAck) {
      tn.trangThai = "cho_giao";
    }
  }
  const tn = hd.tinNhan.find((t) => t.trangThai === "cho_giao");
  if (tn === undefined) return undefined;
  ___
}
function xacNhan(hd: HangDoiAck, id: string): boolean {
  const tn = hd.tinNhan.find((t) => t.id === id);
  if (tn === undefined || tn.trangThai !== "da_giao_cho_ack") return false;
  tn.trangThai = "da_ack";
  return true;
}

const dhX = taoDongHoMoPhong();
const hdX = taoHangDoiAck();
guiVaoHangDoi(hdX, "x1", "noi dung");
nhanTinNhan(hdX, dhX);
tienThoiGian(dhX, NGUONG_ACK_MS + 1);
const lai = nhanTinNhan(hdX, dhX);
console.log(lai?.soLanDaGiao);
```

```typescript title=solution
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

type TrangThaiTinNhan = "cho_giao" | "da_giao_cho_ack" | "da_ack";
interface TinNhan { id: string; noiDung: string; trangThai: TrangThaiTinNhan; soLanDaGiao: number; hanAck: number; }
interface HangDoiAck { tinNhan: TinNhan[]; }
function taoHangDoiAck(): HangDoiAck { return { tinNhan: [] }; }
function guiVaoHangDoi(hd: HangDoiAck, id: string, noiDung: string): void {
  hd.tinNhan.push({ id, noiDung, trangThai: "cho_giao", soLanDaGiao: 0, hanAck: 0 });
}
const NGUONG_ACK_MS = 5000;
function nhanTinNhan(hd: HangDoiAck, dh: DongHoMoPhong): TinNhan | undefined {
  for (const tn of hd.tinNhan) {
    if (tn.trangThai === "da_giao_cho_ack" && dh.thoiGianHienTai > tn.hanAck) {
      tn.trangThai = "cho_giao";
    }
  }
  const tn = hd.tinNhan.find((t) => t.trangThai === "cho_giao");
  if (tn === undefined) return undefined;
  tn.trangThai = "da_giao_cho_ack";
  tn.hanAck = dh.thoiGianHienTai + NGUONG_ACK_MS;
  tn.soLanDaGiao += 1;
  return tn;
}
function xacNhan(hd: HangDoiAck, id: string): boolean {
  const tn = hd.tinNhan.find((t) => t.id === id);
  if (tn === undefined || tn.trangThai !== "da_giao_cho_ack") return false;
  tn.trangThai = "da_ack";
  return true;
}

const dhX = taoDongHoMoPhong();
const hdX = taoHangDoiAck();
guiVaoHangDoi(hdX, "x1", "noi dung");
nhanTinNhan(hdX, dhX);
tienThoiGian(dhX, NGUONG_ACK_MS + 1);
const lai = nhanTinNhan(hdX, dhX);
console.log(lai?.soLanDaGiao);
```

```typescript title=test
const dhT = taoDongHoMoPhong();
const hdT = taoHangDoiAck();
guiVaoHangDoi(hdT, "don-t1", "noi dung");

const n1 = nhanTinNhan(hdT, dhT);
if (n1?.id !== "don-t1") throw new Error("consumer dau tien phai nhan duoc don-t1");
if (n1.soLanDaGiao !== 1) throw new Error("lan giao dau tien phai co soLanDaGiao = 1");

const n2 = nhanTinNhan(hdT, dhT);
if (n2 !== undefined) throw new Error("chua qua han ACK va khong con tin nao khac -- phai tra ve undefined");

tienThoiGian(dhT, NGUONG_ACK_MS + 1);
const n3 = nhanTinNhan(hdT, dhT);
if (n3?.id !== "don-t1") throw new Error("qua han ACK phai duoc GIAO LAI cho consumer khac");
if (n3.soLanDaGiao !== 2) throw new Error("tin nhan giao lai phai co soLanDaGiao = 2 (at-least-once)");

const okKhongTonTai = xacNhan(hdT, "khong-ton-tai");
if (okKhongTonTai !== false) throw new Error("ACK mot id khong ton tai phai tra ve false");

const okAck = xacNhan(hdT, "don-t1");
if (okAck !== true) throw new Error("ACK dung id dang cho ACK phai tra ve true");

const n4 = nhanTinNhan(hdT, dhT);
if (n4 !== undefined) throw new Error("tin nhan DA ACK khong duoc giao lai nua");
```

:::hints
- kind: attention
  body: "Con thieu bon buoc voi `tn` da tim duoc: dat trangThai = 'da_giao_cho_ack', gan hanAck = dh.thoiGianHienTai + NGUONG_ACK_MS, tang soLanDaGiao them 1, roi return tn."
- kind: strategy
  body: "tn.trangThai = 'da_giao_cho_ack'; tn.hanAck = dh.thoiGianHienTai + NGUONG_ACK_MS; tn.soLanDaGiao += 1; return tn;"
- kind: one-line
  body: "tn.trangThai = \"da_giao_cho_ack\"; tn.hanAck = dh.thoiGianHienTai + NGUONG_ACK_MS; tn.soLanDaGiao += 1; return tn;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "2"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một tin có thể tới tay MỘT consumer nhiều lần — at-least-once, không
phải exactly-once. Hàng đợi giờ ĐÁNG tin cậy. Mảnh tiếp theo: một hệ
thống KHÁC cần theo dõi mọi thứ đang chạy.
::::

::::reflect{#nghi-lai}
`nhanTinNhan` không hề biết consumer có xử LÝ xong hay không — nó chỉ
biết "có ACK trong hạn" hay "không". Sự thiếu chắc chắn ĐÓ chính LÀ lý
do at-least-once tồn tại: thà GIAO lại một tin đã xử lý xong (tốn công
thừa) còn hơn BỎ sót một tin CHƯA từng được xử lý.
::::

::::checkpoint{mastery=0.68}
::::
