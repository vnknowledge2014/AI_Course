---
id: thiet-ke-he-thong.thoi-gian-thuc-va-vi-tri.thu-tu-tin-nhan
title: "Thứ tự tin nhắn: số thứ tự, không phải thời gian gửi"
summary: "guiTinNhan gắn soThuTu TĂNG dần riêng cho TỪNG cuộc trò chuyện (không phải theo đồng hồ) -- 3 tin nhắn gửi theo thứ tự 1,2,3 nhưng ĐẾN client theo thứ tự [3,1,2] (mạng không đảm bảo thứ tự đến), sapXepDeHienThi vẫn trả về đúng thứ tự hiển thị 1,2,3. Hai cuộc trò chuyện KHÁC nhau có bộ đếm ĐỘC LẬP -- tin đầu tiên của cuộc B vẫn mang soThuTu=1, không tiếp nối số của cuộc A."
locale: vi
track: thiet-ke-he-thong
module: thoi-gian-thuc-va-vi-tri
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [sd.thu-tu-tin-nhan]
requires: [sd.boss-noi-dung-va-kham-pha]
concepts: [sd.thu-tu-tin-nhan]
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
Mười bài BOSS VỪA ráp xong luồng đăng video — fan-out, transcode, tìm lại.
Track MỚI "Thời gian thực VÀ vị trí" đổi hẳn câu hỏi: không còn "phục vụ
NỘI DUNG cho hàng triệu người ĐỌC", mà LÀ "cập nhật LIÊN tục, độ trễ THẤP".
Mảnh đầu tiên — Chat — bắt đầu từ một sự THẬT khó chịu của mạng: gửi trước
KHÔNG có nghĩa LÀ đến trước.
::::

::::explain{#thu-tu-tin-nhan}
Mạng KHÔNG đảm bảo gói tin đến đúng thứ tự đã gửi — tin nhắn A gửi TRƯỚC
tin nhắn B hoàn toàn có thể đến sau B (đường truyền khác nhau, retry, độ
trễ khác nhau). Cách xử lý: mỗi tin nhắn được gắn một `soThuTu` TĂNG dần
ngay LÚC gửi, ĐẾM riêng cho từng cuộc trò chuyện — RỒI client sắp xếp LẠI
theo con số đó trước khi hiển thị, bất kể chúng đến theo thứ tự NÀO:

```typescript title=readonly
interface TinNhan { soThuTu: number; nguoiGui: string; noiDung: string; }

interface CuocTroChuyen { soThuTuTiepTheo: number; }
function taoCuocTroChuyen(): CuocTroChuyen { return { soThuTuTiepTheo: 1 }; }

function guiTinNhan(ctc: CuocTroChuyen, nguoiGui: string, noiDung: string): TinNhan {
  const tinNhan: TinNhan = { soThuTu: ctc.soThuTuTiepTheo, nguoiGui, noiDung };
  ctc.soThuTuTiepTheo += 1;
  return tinNhan;
}

function sapXepDeHienThi(dsNhanDuoc: TinNhan[]): TinNhan[] {
  return [...dsNhanDuoc].sort((a, b) => a.soThuTu - b.soThuTu);
}

const ctc = taoCuocTroChuyen();
const tin1 = guiTinNhan(ctc, "an", "chao ban");
const tin2 = guiTinNhan(ctc, "binh", "chao an");
const tin3 = guiTinNhan(ctc, "an", "khoe khong");

const thuTuDenThucTe = [tin3, tin1, tin2];
console.log("so thu tu DEN qua mang (chua sap xep):", thuTuDenThucTe.map((t) => t.soThuTu));

const daSapXep = sapXepDeHienThi(thuTuDenThucTe);
console.log("so thu tu HIEN THI sau khi sap xep:", daSapXep.map((t) => t.soThuTu));
console.log("noi dung dau tien hien thi:", daSapXep[0]?.noiDung);
```

```text title=readonly
so thu tu DEN qua mang (chua sap xep): [ 3, 1, 2 ]
so thu tu HIEN THI sau khi sap xep: [ 1, 2, 3 ]
noi dung dau tien hien thi: chao ban
```

`guiTinNhan` gắn `soThuTu` NGAY lúc gửi, dựa vào bộ đếm `soThuTuTiepTheo`
của CHÍNH cuộc trò chuyện — không hề liên quan tới thời điểm gói tin thật
sự tới nơi. Mảng `thuTuDenThucTe` mô phỏng đúng cái mạng THẬT làm: tin thứ
BA đến trước tiên. `sapXepDeHienThi` không quan tâm thứ tự ĐẾN — nó chỉ
sắp lại theo `soThuTu`, nên màn hình LUÔN đúng thứ tự gửi, bất kể mạng làm
gì ở giữa.
::::

::::example{#bo-dem-doc-lap-theo-cuoc-tro-chuyen}
`soThuTuTiepTheo` sống BÊN TRONG từng `CuocTroChuyen` — hai cuộc trò
chuyện khác nhau có hai bộ đếm hoàn toàn TÁCH biệt, không hề chia sẻ:

```typescript title=readonly
interface TinNhan { soThuTu: number; nguoiGui: string; noiDung: string; }
interface CuocTroChuyen { soThuTuTiepTheo: number; }
function taoCuocTroChuyen(): CuocTroChuyen { return { soThuTuTiepTheo: 1 }; }
function guiTinNhan(ctc: CuocTroChuyen, nguoiGui: string, noiDung: string): TinNhan {
  const tinNhan: TinNhan = { soThuTu: ctc.soThuTuTiepTheo, nguoiGui, noiDung };
  ctc.soThuTuTiepTheo += 1;
  return tinNhan;
}

const ctcA = taoCuocTroChuyen();
const ctcB = taoCuocTroChuyen();

const a1 = guiTinNhan(ctcA, "an", "tin dau cua cuoc A");
const b1 = guiTinNhan(ctcB, "chi", "tin dau cua cuoc B");
const a2 = guiTinNhan(ctcA, "binh", "tin thu hai cua cuoc A");

console.log("so thu tu tin nhan dau tien cua cuoc A:", a1.soThuTu);
console.log("so thu tu tin nhan dau tien cua cuoc B:", b1.soThuTu);
console.log("so thu tu tin nhan thu hai cua cuoc A:", a2.soThuTu);
```

```text title=readonly
so thu tu tin nhan dau tien cua cuoc A: 1
so thu tu tin nhan dau tien cua cuoc B: 1
so thu tu tin nhan thu hai cua cuoc A: 2
```

Tin đầu tiên của cuộc B mang `soThuTu = 1`, GIỐNG hệt tin đầu tiên của
cuộc A — không hề "tiếp nối" con số của A dù A được tạo TRƯỚC. Mỗi cuộc
trò chuyện đánh số RIÊNG, kể cả khi chạy song song trên cùng một hệ thống.
::::

::::predict{#doan-thu-tu-hien-thi-dung-du-den-sai commitOnce}
Một cuộc trò chuyện gửi ba tin theo thứ tự: `"mot"` (soThuTu=1), `"hai"`
(soThuTu=2), `"ba"` (soThuTu=3). Do mạng trễ khác nhau, client nhận được
theo thứ tự ĐẾN LÀ `["hai", "ba", "mot"]`. Gọi `sapXepDeHienThi` trên mảng
đến đó, tin nhắn hiển thị ĐẦU TIÊN có nội dung gì?

:::opt{correct}
`"mot"` — `sapXepDeHienThi` sắp theo `soThuTu` TĂNG dần, mà `"mot"` mang
`soThuTu=1` nhỏ nhất, nên nó luôn đứng đầu SAU khi sắp xếp, bất kể nó là
tin đến CUỐI cùng trong mảng thô
:::
:::opt
`"hai"` — vì đó LÀ tin nhắn đứng ĐẦU trong mảng thô client vừa nhận được,
nên hàm hiển thị nó trước tiên
::why
Nhầm "vị trí đầu tiên trong mảng ĐẾN" VỚI "vị trí đầu tiên SAU khi sắp
xếp" — nhưng `sapXepDeHienThi` không hề trả về nguyên mảng đầu vào, nó
LUÔN tạo ra một thứ tự MỚI dựa trên `soThuTu`.

Chỗ lệch: `[...dsNhanDuoc].sort((a, b) => a.soThuTu - b.soThuTu)` sắp xếp
lại toàn bộ mảng theo `soThuTu` tăng dần trước khi trả về — `"hai"` mang
`soThuTu=2`, đứng SAU `"mot"` (soThuTu=1) trong kết quả cuối cùng, dù nó
là phần tử đầu tiên trong mảng thô truyền vào.
::
:::
::::

::::code{#viet_sap_xep_de_hien_thi}
Hoàn thiện `sapXepDeHienThi` — trả về một mảng MỚI (không sửa mảng gốc),
sắp xếp theo `soThuTu` TĂNG dần.

```typescript title=starter
interface TinNhan { soThuTu: number; nguoiGui: string; noiDung: string; }

function sapXepDeHienThi(dsNhanDuoc: TinNhan[]): TinNhan[] {
  ___
}

const vd: TinNhan[] = [
  { soThuTu: 3, nguoiGui: "an", noiDung: "ba" },
  { soThuTu: 1, nguoiGui: "an", noiDung: "mot" },
  { soThuTu: 2, nguoiGui: "an", noiDung: "hai" },
];
console.log(sapXepDeHienThi(vd).map((t) => t.soThuTu));
```

```typescript title=solution
interface TinNhan { soThuTu: number; nguoiGui: string; noiDung: string; }

function sapXepDeHienThi(dsNhanDuoc: TinNhan[]): TinNhan[] {
  return [...dsNhanDuoc].sort((a, b) => a.soThuTu - b.soThuTu);
}

const vd: TinNhan[] = [
  { soThuTu: 3, nguoiGui: "an", noiDung: "ba" },
  { soThuTu: 1, nguoiGui: "an", noiDung: "mot" },
  { soThuTu: 2, nguoiGui: "an", noiDung: "hai" },
];
console.log(sapXepDeHienThi(vd).map((t) => t.soThuTu));
```

```typescript title=test
interface TinNhan { soThuTu: number; nguoiGui: string; noiDung: string; }
const daNhan: TinNhan[] = [
  { soThuTu: 3, nguoiGui: "an", noiDung: "ba" },
  { soThuTu: 1, nguoiGui: "an", noiDung: "mot" },
  { soThuTu: 2, nguoiGui: "an", noiDung: "hai" },
];
const ketQua = sapXepDeHienThi(daNhan);
if (ketQua.map((t) => t.soThuTu).join(",") !== "1,2,3") throw new Error("phai sap xep TANG dan theo soThuTu");
if (ketQua[0]?.noiDung !== "mot") throw new Error("tin nhan dau tien hien thi phai la noi dung cua soThuTu=1");
if (daNhan.map((t) => t.soThuTu).join(",") !== "3,1,2") throw new Error("mang goc dsNhanDuoc KHONG duoc bi thay doi thu tu");

const motTin: TinNhan[] = [{ soThuTu: 5, nguoiGui: "chi", noiDung: "duy nhat" }];
if (sapXepDeHienThi(motTin).length !== 1) throw new Error("mang 1 phan tu phai giu nguyen do dai sau khi sap xep");

const daSapXepSan: TinNhan[] = [
  { soThuTu: 1, nguoiGui: "an", noiDung: "a" },
  { soThuTu: 2, nguoiGui: "an", noiDung: "b" },
];
if (sapXepDeHienThi(daSapXepSan).map((t) => t.soThuTu).join(",") !== "1,2") throw new Error("mang da sap xep san phai giu nguyen thu tu");
```

:::hints
- kind: attention
  body: "Ham phai TRA VE mot mang MOI (dung spread [...dsNhanDuoc]) roi goi .sort() tren mang moi do -- khong duoc sua truc tiep dsNhanDuoc."
- kind: strategy
  body: ".sort() nhan mot ham so sanh (a, b) => a.soThuTu - b.soThuTu de sap TANG dan theo soThuTu."
- kind: one-line
  body: "return [...dsNhanDuoc].sort((a, b) => a.soThuTu - b.soThuTu);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 5000
- tier: output
  match: contains
  expect: "1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Số thứ tự đánh dấu đúng thứ tự GỐC, sắp xếp phía client phục hồi nó — dù
mạng có xáo trộn thế nào. Mảnh tiếp theo: làm sao BIẾT ai đang online.
::::

::::reflect{#nghi-lai}
`sapXepDeHienThi` chỉ LÀ một lệnh `.sort()` — nhưng điều đáng nhớ nằm ở
CHỖ đặt `soThuTu`: gắn nó LÚC gửi (không phải LÚC nhận), đếm riêng theo
từng cuộc trò chuyện. Một khi đã CÓ con số đúng, việc còn lại chỉ LÀ sắp
xếp — mạng có làm xáo trộn thứ tự đến thế nào cũng không còn quan trọng.
::::

::::checkpoint{mastery=0.66}
::::
