---
id: co-so-du-lieu.moi-loi-mot-vuong-quoc.boss-moi-loi-mot-vuong-quoc
title: "BOSS — Mỗi lõi một vương quốc"
summary: "heThongTangGiaTri ráp TRỌN q12: xayVongVnodeLoi (bài 8) xây vòng định tuyến, taoKhoChoMoiLoi (bài 3) tạo kho riêng từng lõi, xuLyYeuCauTangGiaTri (bài 5) route-đọc-tăng-ghi cho MỖI yêu cầu. Xử lý [{khoa:'x',soLan:50},{khoa:'y',soLan:30}] cho kết quả CHÍNH XÁC {x:50, y:30} — không mất một lần tăng nào, khác hẳn kịch bản chia SẺ một kho (bài 1) nơi hai lõi cùng tăng 'x' bị mất một lần cập nhật. Đổi lại: khi thêm lõi (rehash, bài 7-9), dữ liệu VẪN nằm nguyên ở kho cũ nhưng không còn route TỚI đúng chỗ — cái giá của share-nothing không phải lost-update, mà là di trú dữ liệu khi topology đổi."
locale: vi
track: co-so-du-lieu
module: moi-loi-mot-vuong-quoc
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: []
requires: [db.do-ty-le-di-chuyen-vnode-loi]
concepts: [db.boss-q12]
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
Kho riêng (bài 3), routing tất định (bài 4), tin NHẮN (bài 6), vòng
vnode Ở quy mô lõi (bài 8-9). Ráp TẤT cả thành một hệ share-nothing
hoàn chỉnh — trông ra sao?
::::

::::explain{#boss-that}
`heThongTangGiaTri` xây vòng vnode (bài 8), tạo kho RIÊNG cho MỖI lõi
(bài 3), rồi XỬ lý từng yêu cầu "tăng khoá LÊN 1" bằng CÁCH route TỚI
đúng lõi rồi đọc-tăng-ghi (bài 5) — TRẢ về giá trị CUỐI cùng của mỗi
khoá:

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

interface DiemLoi { viTri: number; chiSo: number; }

function timLoiChiuTrachNhiem(vong: DiemLoi[], khoa: string): number {
  const viTriKhoa = bam(khoa);
  for (const diem of vong) {
    if (diem.viTri >= viTriKhoa) return diem.chiSo;
  }
  return vong[0]!.chiSo;
}

function xayVongVnodeLoi(soLoi: number, soVnodeMoiLoi: number): DiemLoi[] {
  const vong: DiemLoi[] = [];
  for (let loi = 0; loi < soLoi; loi++) {
    for (let i = 0; i < soVnodeMoiLoi; i++) {
      vong.push({ viTri: bam(`loi${loi}#${i}`), chiSo: loi });
    }
  }
  vong.sort((a, b) => a.viTri - b.viTri);
  return vong;
}

function taoKhoChoMoiLoi(soLoi: number): Map<string, number>[] {
  const khoCacLoi: Map<string, number>[] = [];
  for (let i = 0; i < soLoi; i++) khoCacLoi.push(new Map<string, number>());
  return khoCacLoi;
}

function xuLyYeuCauTangGiaTri(vong: DiemLoi[], khoCacLoi: Map<string, number>[], khoa: string): number {
  const chiSoLoi = timLoiChiuTrachNhiem(vong, khoa);
  const kho = khoCacLoi[chiSoLoi]!;
  const giaTriMoi = (kho.get(khoa) ?? 0) + 1;
  kho.set(khoa, giaTriMoi);
  return giaTriMoi;
}

interface YeuCauTangGiaTri { khoa: string; soLan: number; }

function heThongTangGiaTri(soLoi: number, soVnodeMoiLoi: number, cacYeuCau: YeuCauTangGiaTri[]): Map<string, number> {
  const vong = xayVongVnodeLoi(soLoi, soVnodeMoiLoi);
  const khoCacLoi = taoKhoChoMoiLoi(soLoi);
  const ketQua = new Map<string, number>();
  for (const yeuCau of cacYeuCau) {
    let giaTriCuoi = 0;
    for (let i = 0; i < yeuCau.soLan; i++) {
      giaTriCuoi = xuLyYeuCauTangGiaTri(vong, khoCacLoi, yeuCau.khoa);
    }
    ketQua.set(yeuCau.khoa, giaTriCuoi);
  }
  return ketQua;
}

const ketQua = heThongTangGiaTri(4, 20, [
  { khoa: "x", soLan: 50 },
  { khoa: "y", soLan: 30 },
]);
console.log(ketQua.get("x"), ketQua.get("y"));
```

```text title=readonly
50 30
```

`50` VÀ `30` — CHÍNH xác, KHÔNG lệch một đơn vị NÀO. So VỚI bài 1
(hai lõi CÙNG đọc-rồi-ghi một `Map` CHUNG cho khoá `"x"`, mất MỘT lần
tăng, kết quả `1` thay VÌ `2`): Ở ĐÂY mỗi khoá LUÔN route TỚI đúng
MỘT lõi DUY nhất (`timLoiChiuTrachNhiem`), VÀ lõi ĐÓ xử lý từng yêu
cầu THEO đúng thứ tự, đọc-tăng-ghi TRỌN vẹn TRƯỚC khi chuyển sang yêu
cầu kế tiếp — KHÔNG hai yêu cầu nào cho CÙNG một khoá có thể "XEN
kẽ" nhau NHƯ bài 1, VÌ chúng không hề chia SẺ một `Map`, mà LUÔN đi
qua đúng MỘT cửa DUY nhất.
::::

::::example{#doc-lap-va-cai-gia-cua-rehash}
`"x"` VÀ `"y"` route TỚI hai lõi KHÁC nhau (routing tất định, bài
4), NÊN tăng khoá `"x"` `50` lần KHÔNG hề ảnh hưởng tới `"y"` — CẢ
hai đều cho kết quả ĐÚNG, độc LẬP hoàn toàn (đúng tinh THẦN share-
nothing, bài 3).

NHƯNG cái GIÁ của kiến trúc NÀY lộ ra KHI số lõi thay ĐỔI (bài 7-9).
Ghi `"nguoidung2"` LÊN `50` VỚI `3` lõi (`20` vnode/lõi), NÓ route
tới lõi `0`. THÊM một lõi (`3→4`), vòng vnode đổi (bài 8), VÀ
`"nguoidung2"` GIỜ route tới lõi `3` — MỘT lõi hoàn toàn TRỐNG, chưa
hề có dữ liệu:

```typescript title=readonly
const vongTruoc3 = xayVongVnodeLoi(3, 20);
const khoTruoc3 = taoKhoChoMoiLoi(3);
for (let i = 0; i < 50; i++) xuLyYeuCauTangGiaTri(vongTruoc3, khoTruoc3, "nguoidung2");
const loiTruoc = timLoiChiuTrachNhiem(vongTruoc3, "nguoidung2");
console.log("truoc rehash: loi", loiTruoc, "gia tri", khoTruoc3[loiTruoc]!.get("nguoidung2"));

const vongSau4 = xayVongVnodeLoi(4, 20);
const khoSau4 = [...khoTruoc3, new Map<string, number>()]; // them 1 kho MOI, KHONG di tru gi ca
const loiSau = timLoiChiuTrachNhiem(vongSau4, "nguoidung2");
console.log("sau rehash: loi", loiSau, "doc duoc", khoSau4[loiSau]!.get("nguoidung2"));
console.log("du lieu THAT su van con o kho cu:", khoTruoc3[loiTruoc]!.get("nguoidung2"));
```

```text title=readonly
truoc rehash: loi 0 gia tri 50
sau rehash: loi 3 doc duoc undefined
du lieu THAT su van con o kho cu: 50
```

Giá trị `50` KHÔNG hề "biến mất" — NÓ vẫn nằm nguyên Ở `Map` của lõi
`0`. NHƯNG mọi yêu cầu ĐỌC `"nguoidung2"` SAU khi thêm lõi đều route
tới lõi `3`, MỘT kho RỖNG — kết quả `undefined`. Đây chính LÀ chi
phí Ở bài 7 đã gọi LÀ "thảm hoạ rehash": share-nothing xoá bỏ HẲN
lost-update (không cần khoá), NHƯNG đổi lại, thay đổi TOPOLOGY (thêm/
bớt lõi) đòi hỏi một bước DI trú dữ liệu tường minh — di chuyển ĐÚNG
những khoá bị ảnh hưởng (bài 9: CHỈ khoảng `25-30%` VỚI vnode, thay
VÌ `75%` VỚI `% soLoi` ngây thơ) SANG kho MỚI TRƯỚC khi route thay
đổi hẳn.
::::

::::predict{#doan-yeu-cau-lap-lai commitOnce}
`heThongTangGiaTri` xử lý danh SÁCH
`[{khoa:"x",soLan:10},{khoa:"y",soLan:5},{khoa:"x",soLan:10}]` — khoá
`"x"` xuất hiện HAI lần, cách nhau BỞI một yêu cầu KHÁC (`"y"`). Kết
quả CUỐI cùng CHO `"x"` LÀ bao nhiêu — `10` (giá trị của LẦN xử lý
CUỐI) hay `20` (CỘNG dồn CẢ hai lần)?

:::opt{correct}
`20` — MỖI lần xử lý yêu cầu ĐỌC giá trị hiện tại TỪ `kho.get(khoa)`
TRƯỚC khi tăng, VÀ `kho` (của lõi chịu trách nhiệm CHO `"x"`) LÀ MỘT
`Map` duy NHẤT được TÁI sử dụng xuyên suốt TOÀN bộ danh sách — lần xử
lý thứ hai ĐỌC lại đúng `10` đã ghi TỪ lần đầu, RỒI tăng tiếp `10`
lần NỮA thành `20`
:::

:::opt
`10` — MỖI phần TỬ trong `cacYeuCau` được xử LÝ độc lập, VÀ
`giaTriCuoi` LUÔN được khởi tạo LẠI về `0` Ở đầu mỗi VÒNG lặp ngoài,
NÊN lần xử lý thứ hai CỦA `"x"` bắt đầu LẠI từ đầu
::why
Gần đúng ở việc bạn ĐỌC đúng dòng
`let giaTriCuoi = 0;` nằm BÊN trong vòng lặp ngoài (`for (const
yeuCau of cacYeuCau)`) — ĐÚNG LÀ biến ĐỊA phương này reset về `0` ở
MỖI vòng.

Chỗ lệch: `giaTriCuoi` CHỈ là một biến TẠM để nhớ kết quả TRẢ về của
LẦN gọi `xuLyYeuCauTangGiaTri` cuối CÙNG trong vòng lặp TRONG — NÓ
không phải LÀ nơi lưu trữ giá trị THẬT của khoá. Giá trị THẬT nằm
TRONG `kho.get(khoa)` (bên trong `Map` của lõi chịu trách NHIỆM cho
`"x"`), VÀ `Map` đó — cùng với `khoCacLoi` chứa NÓ — được tạo ĐÚNG
MỘT lần Ở đầu `heThongTangGiaTri`, rồi DÙNG lại cho MỌI yêu cầu trong
`cacYeuCau`, KHÔNG tạo mới cho từng phần TỬ. `"x"` xuất hiện lại LẦN
hai vẫn ĐỌC đúng giá trị `10` đã ghi TỪ lần đầu — RỒI cộng thêm `10`
NỮA, cho kết quả `20`.
::
:::
::::

::::code{#viet_boss_q12}
Hoàn thiện `heThongTangGiaTri` — VỚI mỗi yêu cầu TRONG `cacYeuCau`,
gọi `xuLyYeuCauTangGiaTri` ĐÚNG khoá của yêu CẦU đó `soLan` lần, LƯU
kết quả cuối vào `ketQua`.

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

interface DiemLoi { viTri: number; chiSo: number; }

function timLoiChiuTrachNhiem(vong: DiemLoi[], khoa: string): number {
  const viTriKhoa = bam(khoa);
  for (const diem of vong) {
    if (diem.viTri >= viTriKhoa) return diem.chiSo;
  }
  return vong[0]!.chiSo;
}

function xayVongVnodeLoi(soLoi: number, soVnodeMoiLoi: number): DiemLoi[] {
  const vong: DiemLoi[] = [];
  for (let loi = 0; loi < soLoi; loi++) {
    for (let i = 0; i < soVnodeMoiLoi; i++) {
      vong.push({ viTri: bam(`loi${loi}#${i}`), chiSo: loi });
    }
  }
  vong.sort((a, b) => a.viTri - b.viTri);
  return vong;
}

function taoKhoChoMoiLoi(soLoi: number): Map<string, number>[] {
  const khoCacLoi: Map<string, number>[] = [];
  for (let i = 0; i < soLoi; i++) khoCacLoi.push(new Map<string, number>());
  return khoCacLoi;
}

function xuLyYeuCauTangGiaTri(vong: DiemLoi[], khoCacLoi: Map<string, number>[], khoa: string): number {
  const chiSoLoi = timLoiChiuTrachNhiem(vong, khoa);
  const kho = khoCacLoi[chiSoLoi]!;
  const giaTriMoi = (kho.get(khoa) ?? 0) + 1;
  kho.set(khoa, giaTriMoi);
  return giaTriMoi;
}

interface YeuCauTangGiaTri { khoa: string; soLan: number; }

function heThongTangGiaTri(soLoi: number, soVnodeMoiLoi: number, cacYeuCau: YeuCauTangGiaTri[]): Map<string, number> {
  const vong = xayVongVnodeLoi(soLoi, soVnodeMoiLoi);
  const khoCacLoi = taoKhoChoMoiLoi(soLoi);
  const ketQua = new Map<string, number>();
  for (const yeuCau of cacYeuCau) {
    let giaTriCuoi = 0;
    for (let i = 0; i < yeuCau.soLan; i++) {
      giaTriCuoi = ___;
    }
    ketQua.set(yeuCau.khoa, giaTriCuoi);
  }
  return ketQua;
}

const ketQua = heThongTangGiaTri(4, 20, [
  { khoa: "x", soLan: 50 },
  { khoa: "y", soLan: 30 },
]);
console.log(ketQua.get("x"), ketQua.get("y"));
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

interface DiemLoi { viTri: number; chiSo: number; }

function timLoiChiuTrachNhiem(vong: DiemLoi[], khoa: string): number {
  const viTriKhoa = bam(khoa);
  for (const diem of vong) {
    if (diem.viTri >= viTriKhoa) return diem.chiSo;
  }
  return vong[0]!.chiSo;
}

function xayVongVnodeLoi(soLoi: number, soVnodeMoiLoi: number): DiemLoi[] {
  const vong: DiemLoi[] = [];
  for (let loi = 0; loi < soLoi; loi++) {
    for (let i = 0; i < soVnodeMoiLoi; i++) {
      vong.push({ viTri: bam(`loi${loi}#${i}`), chiSo: loi });
    }
  }
  vong.sort((a, b) => a.viTri - b.viTri);
  return vong;
}

function taoKhoChoMoiLoi(soLoi: number): Map<string, number>[] {
  const khoCacLoi: Map<string, number>[] = [];
  for (let i = 0; i < soLoi; i++) khoCacLoi.push(new Map<string, number>());
  return khoCacLoi;
}

function xuLyYeuCauTangGiaTri(vong: DiemLoi[], khoCacLoi: Map<string, number>[], khoa: string): number {
  const chiSoLoi = timLoiChiuTrachNhiem(vong, khoa);
  const kho = khoCacLoi[chiSoLoi]!;
  const giaTriMoi = (kho.get(khoa) ?? 0) + 1;
  kho.set(khoa, giaTriMoi);
  return giaTriMoi;
}

interface YeuCauTangGiaTri { khoa: string; soLan: number; }

function heThongTangGiaTri(soLoi: number, soVnodeMoiLoi: number, cacYeuCau: YeuCauTangGiaTri[]): Map<string, number> {
  const vong = xayVongVnodeLoi(soLoi, soVnodeMoiLoi);
  const khoCacLoi = taoKhoChoMoiLoi(soLoi);
  const ketQua = new Map<string, number>();
  for (const yeuCau of cacYeuCau) {
    let giaTriCuoi = 0;
    for (let i = 0; i < yeuCau.soLan; i++) {
      giaTriCuoi = xuLyYeuCauTangGiaTri(vong, khoCacLoi, yeuCau.khoa);
    }
    ketQua.set(yeuCau.khoa, giaTriCuoi);
  }
  return ketQua;
}

const ketQua = heThongTangGiaTri(4, 20, [
  { khoa: "x", soLan: 50 },
  { khoa: "y", soLan: 30 },
]);
console.log(ketQua.get("x"), ketQua.get("y"));
```

```typescript title=test
const ketQua2 = heThongTangGiaTri(4, 20, [
  { khoa: "x", soLan: 50 },
  { khoa: "y", soLan: 30 },
]);
console.log(ketQua2.get("x"), ketQua2.get("y"));
if (ketQua2.get("x") !== 50) throw new Error("x phai la 50");
if (ketQua2.get("y") !== 30) throw new Error("y phai la 30, khong bi anh huong boi x");

const ketQua3 = heThongTangGiaTri(4, 20, [
  { khoa: "a", soLan: 20 },
  { khoa: "b", soLan: 7 },
]);
if (ketQua3.get("a") !== 20) throw new Error("a phai la 20");
if (ketQua3.get("b") !== 7) throw new Error("b phai la 7");

const ketQuaMotYeuCau = heThongTangGiaTri(6, 15, [{ khoa: "khoa_don", soLan: 1 }]);
if (ketQuaMotYeuCau.get("khoa_don") !== 1) throw new Error("1 lan tang phai cho ket qua 1");

const ketQuaRong = heThongTangGiaTri(4, 20, []);
if (ketQuaRong.size !== 0) throw new Error("danh sach yeu cau rong thi ket qua phai rong");

const ketQuaLapLai = heThongTangGiaTri(4, 20, [
  { khoa: "x", soLan: 10 },
  { khoa: "y", soLan: 5 },
  { khoa: "x", soLan: 10 },
]);
if (ketQuaLapLai.get("x") !== 20) throw new Error("x xuat hien hai lan trong danh sach phai cong don thanh 20");
if (ketQuaLapLai.get("y") !== 5) throw new Error("y phai la 5");
```

:::hints
- kind: attention
  body: "Blank nam trong vong lap TRONG (for i < yeuCau.soLan) -- moi vong goi mot ham DA cho san."
- kind: strategy
  body: "xuLyYeuCauTangGiaTri route toi dung loi ROI doc-tang-ghi -- gan ket qua tra ve cua no cho giaTriCuoi."
- kind: one-line
  body: "giaTriCuoi = xuLyYeuCauTangGiaTri(vong, khoCacLoi, yeuCau.khoa);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "50 30"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Kho riêng, routing tất định, vòng vnode — GHÉP lại LÀ một hệ share-
nothing đúng NGHĨA, không lost-update, chi phí rehash được ĐO và
giảm thiểu RÕ ràng. Mảnh ghép TIẾP theo: khi một node THẬT sự sập
giữa chừng, hệ THỐNG phân tán phải làm SAO để không mất VIẾT?
::::

::::reflect{#nghi-lai}
q12 giải quyết đúng vấn đề bài 1 ĐẶT ra — nhưng KHÔNG bằng khoá
(lock), MÀ bằng KIẾN trúc: mỗi khoá thuộc về đúng MỘT lõi DUY nhất,
NÊN không CÓ hai luồng NÀO có thể "đụng" và CÙNG một `Map`. Đây LÀ
đúng triết lý ScyllaDB/thread-per-core THẬT — VÀ nó tái sử DỤNG
nguyên vẹn vòng hash + vnode q11 đã xây CHO việc phân MẢNH giữa
nhiều MÁY, chỉ ÁP dụng lại Ở quy mô lõi TRONG một máy. Router tất
định GIẢI quyết "khoá nào ở ĐÂU" — nhưng "READ phải chờ ĐỦ bao nhiêu
bản SAO xác nhận, WRITE phải tới bao NHIÊU node mới được TÍNH là
thành công" LÀ câu hỏi TIẾP theo (quorum, `R`/`W`, hinted handoff) —
Ở đó khi MỘT node tạm sập, hệ thống ĐỌC/ghi ra sao mà VẪN đúng.
::::

::::checkpoint{mastery=0.85}
::::
</content>
