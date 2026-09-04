---
id: thiet-ke-he-thong.streaming-va-hieu-ung-o-bien.effect-chi-o-bien
title: "Effect chỉ ở biên: ghiRaNgoai tách biệt hoàn toàn khỏi lõi xử lý"
summary: "ghiRaNgoai(nhatKy, lo) mo phong I/O bang cach push vao mot mang (NhatKyGhiRaNgoai.cacLoDaGhi) THAY VI console.log/network that -- chi duoc goi O LOP NGOAI CUNG, SAU khi loi (quyetDinhGomLo) da quyet dinh xong; doi effect giat lap (demSoLuongGuiRa thay ghiRaNgoai) ma KHONG sua mot dong nao cua loi, chung minh loi khong he goi truc tiep mot ham I/O nao."
locale: vi
track: thiet-ke-he-thong
module: streaming-va-hieu-ung-o-bien
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [sd.fp.effect-chi-o-bien]
requires: [sd.fp.vo-menh-lenh-cho-backpressure]
concepts: [sd.fp.effect-chi-o-bien]
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
`quyetDinhGomLo` quyết định lô đã sẵn sàng. `xuLySuKienDen` quyết định
buffer còn nhận được hay không. Cả hai đều KHÔNG hề gửi bất cứ gì ra
thế giới bên ngoài. Đến lúc một lô THẬT SỰ phải rời khỏi hệ thống —
ghi xuống đĩa, gửi qua mạng — nhưng bài học này sẽ không dùng mạng
thật hay đĩa thật. Nó dùng một MẢNG.
::::

::::explain{#hieu-ung-chi-o-lop-ngoai-cung}
`ghiRaNgoai` mô phỏng một hiệu ứng I/O bằng cách đẩy `lo` vào
`nhatKy.cacLoDaGhi` — một mảng trong bộ nhớ, thay vì gọi
`console.log`, `fetch`, hay ghi file thật. Điểm mấu chốt không nằm Ở
CÁCH mô phỏng, mà Ở CHỖ nó được gọi: `timCacLoCanGui` (LÕI, ráp từ
`quyetDinhGomLo` bài 2) không hề chứa một lời gọi `ghiRaNgoai` nào bên
trong nó — hiệu ứng chỉ xảy ra Ở một vòng lặp NGOÀI CÙNG, sau khi lõi
đã quyết định xong TOÀN bộ danh sách lô:

```typescript title=readonly
interface SuKien { id: string; nguon: string; thoiDiem: number; soLuong: number; }
interface KetQuaGomLo { loMoi: SuKien[] | undefined; bufferMoi: SuKien[]; }
function quyetDinhGomLo(buffer: SuKien[], suKienMoi: SuKien, kichThuocLo: number, thoiGianChoMs: number): KetQuaGomLo {
  const bufferSau = [...buffer, suKienMoi];
  const dauBuffer = bufferSau[0];
  const daDuKichThuoc = bufferSau.length >= kichThuocLo;
  const daHetThoiGianCho = dauBuffer !== undefined && suKienMoi.thoiDiem - dauBuffer.thoiDiem >= thoiGianChoMs;
  if (daDuKichThuoc || daHetThoiGianCho) {
    return { loMoi: bufferSau, bufferMoi: [] };
  }
  return { loMoi: undefined, bufferMoi: bufferSau };
}

interface NhatKyGhiRaNgoai { cacLoDaGhi: SuKien[][]; }
function taoNhatKyGhiRaNgoai(): NhatKyGhiRaNgoai { return { cacLoDaGhi: [] }; }
function ghiRaNgoai(nhatKy: NhatKyGhiRaNgoai, lo: SuKien[]): void {
  nhatKy.cacLoDaGhi.push(lo);
}

const suKien = (id: string, thoiDiem: number): SuKien => ({ id, nguon: "edge-1", thoiDiem, soLuong: 1 });
const luong: SuKien[] = [
  suKien("e1", 0), suKien("e2", 100), suKien("e3", 200),
  suKien("e4", 300), suKien("e5", 400), suKien("e6", 500),
];

// LOI: chi QUYET DINH, khong goi mot ham I/O nao
const { cacLoCanGui } = luong.reduce(
  (tich, sk) => {
    const kq = quyetDinhGomLo(tich.buffer, sk, 3, 100000);
    if (kq.loMoi !== undefined) return { buffer: kq.bufferMoi, cacLoCanGui: [...tich.cacLoCanGui, kq.loMoi] };
    return { buffer: kq.bufferMoi, cacLoCanGui: tich.cacLoCanGui };
  },
  { buffer: [] as SuKien[], cacLoCanGui: [] as SuKien[][] }
);
console.log("LOI da quyet dinh", cacLoCanGui.length, "lo -- CHUA co gi duoc ghi ra ngoai");

// VO: DUY NHAT noi ghiRaNgoai duoc goi, SAU khi loi da quyet dinh xong het
const nhatKy = taoNhatKyGhiRaNgoai();
for (const lo of cacLoCanGui) {
  ghiRaNgoai(nhatKy, lo);
}
console.log("VO da ghi", nhatKy.cacLoDaGhi.length, "lo ra ngoai");
console.log("noi dung lo dau tien da ghi:", JSON.stringify(nhatKy.cacLoDaGhi[0]?.map((sk) => sk.id)));
```

```text title=readonly
LOI da quyet dinh 2 lo -- CHUA co gi duoc ghi ra ngoai
VO da ghi 2 lo ra ngoai
noi dung lo dau tien da ghi: ["e1","e2","e3"]
```

Ngay sau khi tính xong `cacLoCanGui`, đã có đúng `2` lô — nhưng
`nhatKy.cacLoDaGhi` vẫn hoàn toàn rỗng cho tới khi vòng lặp `for`
(nằm NGOÀI logic gom lô) gọi `ghiRaNgoai` cho từng lô một. Đây chính
là ranh giới "effect chỉ Ở biên": mọi quyết định XONG trước, mọi hiệu
ứng xảy ra SAU, VÀ không có dòng nào trộn lẫn cả hai.
::::

::::example{#doi-hieu-ung-khong-sua-loi}
Vì lõi không hề biết `ghiRaNgoai` tồn tại, có thể thay hiệu ứng bằng
một hàm HOÀN TOÀN khác — ví dụ chỉ ĐẾM thay vì lưu nội dung — mà không
sửa MỘT dòng nào của `timCacLoCanGui` hay `quyetDinhGomLo`:

```typescript title=readonly
interface SuKien { id: string; nguon: string; thoiDiem: number; soLuong: number; }
interface KetQuaGomLo { loMoi: SuKien[] | undefined; bufferMoi: SuKien[]; }
function quyetDinhGomLo(buffer: SuKien[], suKienMoi: SuKien, kichThuocLo: number, thoiGianChoMs: number): KetQuaGomLo {
  const bufferSau = [...buffer, suKienMoi];
  const dauBuffer = bufferSau[0];
  const daDuKichThuoc = bufferSau.length >= kichThuocLo;
  const daHetThoiGianCho = dauBuffer !== undefined && suKienMoi.thoiDiem - dauBuffer.thoiDiem >= thoiGianChoMs;
  if (daDuKichThuoc || daHetThoiGianCho) {
    return { loMoi: bufferSau, bufferMoi: [] };
  }
  return { loMoi: undefined, bufferMoi: bufferSau };
}
interface NhatKyGhiRaNgoai { cacLoDaGhi: SuKien[][]; }
function taoNhatKyGhiRaNgoai(): NhatKyGhiRaNgoai { return { cacLoDaGhi: [] }; }
function ghiRaNgoai(nhatKy: NhatKyGhiRaNgoai, lo: SuKien[]): void {
  nhatKy.cacLoDaGhi.push(lo);
}

function timCacLoCanGui(luong: SuKien[], kichThuocLo: number, thoiGianChoMs: number): SuKien[][] {
  return luong.reduce(
    (tich, sk) => {
      const kq = quyetDinhGomLo(tich.buffer, sk, kichThuocLo, thoiGianChoMs);
      if (kq.loMoi !== undefined) return { buffer: kq.bufferMoi, cacLoCanGui: [...tich.cacLoCanGui, kq.loMoi] };
      return { buffer: kq.bufferMoi, cacLoCanGui: tich.cacLoCanGui };
    },
    { buffer: [] as SuKien[], cacLoCanGui: [] as SuKien[][] }
  ).cacLoCanGui;
}

const suKien = (id: string, thoiDiem: number): SuKien => ({ id, nguon: "edge-1", thoiDiem, soLuong: 1 });
const luong: SuKien[] = [
  suKien("e1", 0), suKien("e2", 100), suKien("e3", 200),
  suKien("e4", 300), suKien("e5", 400), suKien("e6", 500),
];
const cacLoCanGui = timCacLoCanGui(luong, 3, 100000);

// hieu ung THU NHAT: ghi vao nhat ky gia lap
const nhatKy = taoNhatKyGhiRaNgoai();
for (const lo of cacLoCanGui) ghiRaNgoai(nhatKy, lo);
console.log("hieu ung ghiRaNgoai: da ghi", nhatKy.cacLoDaGhi.length, "lo");

// hieu ung THU HAI: chi DEM, khong luu noi dung -- KHONG mot dong nao cua
// timCacLoCanGui hay quyetDinhGomLo bi sua doi de dung hieu ung nay
interface BoDemGuiRa { soLo: number; tongSuKien: number; }
function taoBoDemGuiRa(): BoDemGuiRa { return { soLo: 0, tongSuKien: 0 }; }
function demSoLuongGuiRa(dem: BoDemGuiRa, lo: SuKien[]): void {
  dem.soLo += 1;
  dem.tongSuKien += lo.length;
}
const boDem = taoBoDemGuiRa();
for (const lo of cacLoCanGui) demSoLuongGuiRa(boDem, lo);
console.log("hieu ung demSoLuongGuiRa: soLo =", boDem.soLo, ", tongSuKien =", boDem.tongSuKien);

console.log("ca hai hieu ung cung xu ly DUNG", cacLoCanGui.length, "lo -- LOI (timCacLoCanGui) khong doi mot dong nao");
```

```text title=readonly
hieu ung ghiRaNgoai: da ghi 2 lo
hieu ung demSoLuongGuiRa: soLo = 2 , tongSuKien = 6
ca hai hieu ung cung xu ly DUNG 2 lo -- LOI (timCacLoCanGui) khong doi mot dong nao
```

`timCacLoCanGui` được gọi ĐÚNG một lần, cho ra `cacLoCanGui` CỐ định.
Hai vòng lặp `for` KHÁC nhau — một dùng `ghiRaNgoai`, một dùng
`demSoLuongGuiRa` — chạy TRÊN đúng dữ liệu đó, không hề gọi lại
`timCacLoCanGui` hay sửa nó theo bất kỳ cách nào. Đây chính là bằng
chứng: lõi không biết, và không cần biết, hiệu ứng cuối cùng sẽ LÀ gì.
::::

::::predict{#doan-ghi-lai-cung-lo commitOnce}
`nhatKy` là một `NhatKyGhiRaNgoai` mới tạo (rỗng). Gọi
`ghiRaNgoai(nhatKy, lo)` HAI lần liên tiếp, với ĐÚNG CÙNG một mảng
`lo`. `nhatKy.cacLoDaGhi.length` sau hai lệnh gọi đó là bao nhiêu?

:::opt{correct}
`2` — `ghiRaNgoai` chỉ đơn giản `push` vào mảng mỗi lần được gọi,
không hề kiểm tra `lo` đã từng được ghi trước đó hay chưa; nó LÀ một
hiệu ứng, không phải một phép áp dụng idempotent
:::
:::opt
`1` — vì `lo` truyền vào hai lần LÀ CÙNG một dữ liệu, `ghiRaNgoai`
tự động nhận ra VÀ bỏ qua lần ghi trùng, giống cách `apDungIdempotent`
Ở quest "Nhật ký bất biến và fold" bỏ qua sự kiện có `id` đã gặp
::why
Nhầm hành vi của MỘT effect Ở biên với hành vi của LÕI thuần idempotent
Ở quest trước — nhưng `ghiRaNgoai` không hề nhận hay kiểm tra bất kỳ
`id` nào; nó chỉ có đúng một dòng `nhatKy.cacLoDaGhi.push(lo)`.

Chỗ lệch: tính idempotent (áp dụng nhiều lần cho hiệu ứng NHƯ áp dụng
một lần) là một tính chất PHẢI được xây dựng có chủ đích — như
`apDungIdempotent` đã làm bằng cách kiểm `idDaApDung.has(suKien.id)`
TRƯỚC khi cộng dồn. `ghiRaNgoai` không có bước kiểm tra đó; nó là một
effect "ngây thơ" — gọi bao nhiêu lần, ghi thêm bấy nhiêu lần. Đây
chính là lý do mọi effect Ở biên cần được thiết kế cẩn thận: không có
gì đảm bảo effect tự động an toàn khi gọi lại, trừ khi người viết nó
CHỦ ĐỘNG thêm cơ chế đó vào.
::
:::
::::

::::code{#viet_ghi_ra_ngoai}
Hoàn thiện `ghiRaNgoai` — thêm `lo` vào cuối `nhatKy.cacLoDaGhi` bằng
`.push`.

```typescript title=starter
interface SuKien { id: string; nguon: string; thoiDiem: number; soLuong: number; }
interface NhatKyGhiRaNgoai { cacLoDaGhi: SuKien[][]; }
function taoNhatKyGhiRaNgoai(): NhatKyGhiRaNgoai { return { cacLoDaGhi: [] }; }

function ghiRaNgoai(nhatKy: NhatKyGhiRaNgoai, lo: SuKien[]): void {
  ___
}

const nhatKyX = taoNhatKyGhiRaNgoai();
ghiRaNgoai(nhatKyX, [{ id: "x1", nguon: "e", thoiDiem: 0, soLuong: 1 }]);
ghiRaNgoai(nhatKyX, [{ id: "x2", nguon: "e", thoiDiem: 0, soLuong: 1 }, { id: "x3", nguon: "e", thoiDiem: 0, soLuong: 1 }]);
console.log(nhatKyX.cacLoDaGhi.length, nhatKyX.cacLoDaGhi[1]?.length);
```

```typescript title=solution
interface SuKien { id: string; nguon: string; thoiDiem: number; soLuong: number; }
interface NhatKyGhiRaNgoai { cacLoDaGhi: SuKien[][]; }
function taoNhatKyGhiRaNgoai(): NhatKyGhiRaNgoai { return { cacLoDaGhi: [] }; }

function ghiRaNgoai(nhatKy: NhatKyGhiRaNgoai, lo: SuKien[]): void {
  nhatKy.cacLoDaGhi.push(lo);
}

const nhatKyX = taoNhatKyGhiRaNgoai();
ghiRaNgoai(nhatKyX, [{ id: "x1", nguon: "e", thoiDiem: 0, soLuong: 1 }]);
ghiRaNgoai(nhatKyX, [{ id: "x2", nguon: "e", thoiDiem: 0, soLuong: 1 }, { id: "x3", nguon: "e", thoiDiem: 0, soLuong: 1 }]);
console.log(nhatKyX.cacLoDaGhi.length, nhatKyX.cacLoDaGhi[1]?.length);
```

```typescript title=test
const tNhatKy = taoNhatKyGhiRaNgoai();
const tSoLo0 = tNhatKy.cacLoDaGhi.length;
if (tSoLo0 !== 0) throw new Error("nhat ky moi tao phai RONG");

const tLo1: SuKien[] = [{ id: "a1", nguon: "e", thoiDiem: 0, soLuong: 5 }];
ghiRaNgoai(tNhatKy, tLo1);
const tSoLo1 = tNhatKy.cacLoDaGhi.length;
if (tSoLo1 !== 1) throw new Error("sau 1 lan ghi phai co 1 lo trong nhat ky");
const tLoDauId = tNhatKy.cacLoDaGhi[0]?.[0]?.id;
if (tLoDauId !== "a1") throw new Error("lo da ghi phai dung noi dung");

const tLo2: SuKien[] = [{ id: "b1", nguon: "e", thoiDiem: 0, soLuong: 1 }, { id: "b2", nguon: "e", thoiDiem: 0, soLuong: 1 }];
ghiRaNgoai(tNhatKy, tLo2);
const tSoLo2 = tNhatKy.cacLoDaGhi.length;
if (tSoLo2 !== 2) throw new Error("sau 2 lan ghi phai co 2 lo");
const tLoHaiDoDai = tNhatKy.cacLoDaGhi[1]?.length;
if (tLoHaiDoDai !== 2) throw new Error("lo thu hai phai co 2 su kien");

ghiRaNgoai(tNhatKy, tLo1);
const tSoLo3 = tNhatKy.cacLoDaGhi.length;
if (tSoLo3 !== 3) throw new Error("ghi LAI cung lo van phai THEM entry moi -- ghiRaNgoai khong tu loai trung");
```

:::hints
- kind: attention
  body: "Chi can mot dong: goi nhatKy.cacLoDaGhi.push(lo). Day la HAM DUY NHAT trong quest nay duoc phep 'gia lap I/O' -- no khong tra ve gia tri (void), chi mutate nhatKy."
- kind: strategy
  body: "nhatKy.cacLoDaGhi.push(lo);"
- kind: one-line
  body: "nhatKy.cacLoDaGhi.push(lo);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "2 2"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Lõi quyết định, biên thực thi — VÀ có thể đổi biên mà không đụng lõi.
Nhưng windowing (bài 1) mới chỉ xử lý sự kiện đến ĐÚNG thứ tự thời
gian. Sự kiện đến TRỄ thì sao?
::::

::::reflect{#nghi-lai}
`ghiRaNgoai` không hề phức tạp — đúng MỘT dòng `push`. Sự đơn giản đó
chính là điểm mấu chốt: một effect Ở biên NÊN đơn giản tới mức không
còn gì để kiểm thử sai, bởi vì mọi QUYẾT ĐỊNH phức tạp (khi nào gom,
khi nào chặn, khung nào thuộc về đâu) đã xảy ra Ở lõi, trước khi effect
này được gọi. Nếu một effect bắt đầu chứa logic rẽ nhánh phức tạp, đó
là dấu hiệu logic đang bị viết nhầm chỗ — nó thuộc về lõi, không thuộc
về biên.
::::

::::checkpoint{mastery=0.79}
::::
