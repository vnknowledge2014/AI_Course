---
id: ky-nghe-ung-dung-ai.ky-thuat-ngu-canh.so-ghi-nho-song-sot-qua-cat-cua-so
title: "Cầu nối — sổ ghi nhớ sống sót qua cắt cửa sổ, cửa sổ thì không"
summary: "RÁP bài 3 (capNhatSoGhiNho) VỚI catCuaSoTruot (bài 2, q9.2a): moPhongPhienVoiSoGhiNho(cacLuot: ChatMessage[][], nganSach: number, cacTuKhoaCanTrich: string[]) chạy 7 lượt (CAC_LUOT_HOI_THOAI, q9.2a bài 6) — Ở MỖI lượt VỪA capNhatSoGhiNho cho message mới VỪA áp catCuaSoTruot lên cửa sổ tích luỹ khi vượt ngân sách 50. Đo hai thứ TÁCH BIỆT: tinhTyLeRecall (bài 4) trên soGhiNhoCuoiCung LUÔN LÀ 1.0 (3/3 cụm 'XY789'/'30 ngay'/'giam gia', không mất mục nào cho dù cửa sổ bị cắt bao nhiêu lần) — NHƯNG coConNhoSuKien (bài 3, q9.2a) trên cuaSoCuoiCung LÀ false cho CẢ 'XY789' LẪN '30 ngay' (đã bị cắt khỏi cửa sổ hiển thị). Sổ ghi nhớ VÀ cửa sổ LÀ HAI THỨ KHÁC NHAU — một hệ thống production tốt cần CẢ HAI."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-ngu-canh
order: 11
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [kna.so-ghi-nho-song-sot-qua-cat-cua-so]
requires: [kna.do-ty-le-recall-da-su-kien]
concepts: [kna.so-ghi-nho-song-sot-qua-cat-cua-so]
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
Ba bài liên tiếp đã xây RIÊNG hai thứ: một SỔ GHI NHỚ tích luỹ (bài
`9`/`10`) VÀ một CỬA SỔ bị cắt (`catCuaSoTruot`, bài `2` q9.2a) — nhưng
chưa bao giờ chạy CÙNG lúc trên CÙNG một phiên. Bài này ráp cả hai VÀO
MỘT vòng lặp nhiều lượt, VÀ đo NGAY TRÊN CÙNG một kịch bản: sổ ghi nhớ
có sống sót qua việc cửa sổ bị cắt hay không?
::::

::::explain{#rap-so-ghi-nho-va-cua-so}
`moPhongPhienVoiSoGhiNho` chạy qua từng lượt trong `cacLuot`. Ở MỖI
lượt, HAI việc diễn ra ĐỘC LẬP: (1) MỖI message mới trong lượt được đưa
qua `capNhatSoGhiNho` (bài `9`) — sổ ghi nhớ CHỈ LỚN LÊN, không bao giờ
bị xoá bớt; (2) TOÀN BỘ lịch sử tích luỹ (`lichSu`) được kiểm
`vuotNganSach` (bài `1`, q9.2a) — nếu vượt, ÁP `catCuaSoTruot` (bài `2`,
q9.2a) LÊN CỬA SỔ, có thể XOÁ HẲN message chứa một sự kiện quan trọng
khỏi PHẦN HIỂN THỊ CHO MODEL:

```typescript title=readonly
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}
interface MucGhiNho {
  cumTu: string;
  noiDung: string;
}

function demTokenGiaLap(text: string): number {
  return Math.ceil(text.length / 4);
}
function tinhTongToken(messages: ChatMessage[]): number {
  return messages.reduce((tong, tin) => tong + demTokenGiaLap(tin.content), 0);
}
function vuotNganSach(messages: ChatMessage[], nganSach: number): boolean {
  return tinhTongToken(messages) > nganSach;
}
function catCuaSoTruot(messages: ChatMessage[], nganSach: number): ChatMessage[] {
  if (messages.length === 0) return [];
  const laHeThong = messages[0]!.role === "system";
  const phanGiuCoDinh = laHeThong ? [messages[0]!] : [];
  const phanCoTheCat = laHeThong ? messages.slice(1) : messages.slice();
  let batDau = 0;
  while (batDau < phanCoTheCat.length && tinhTongToken([...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)]) > nganSach) { batDau += 1; }
  return [...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)];
}
function coConNhoSuKien(messages: ChatMessage[], cumTuCanTim: string): boolean {
  return messages.some((tin) => tin.content.includes(cumTuCanTim));
}
function trichXuatGhiNho(tin: ChatMessage, cacTuKhoaCanTrich: string[]): MucGhiNho[] {
  const ketQua: MucGhiNho[] = [];
  for (const tuKhoa of cacTuKhoaCanTrich) {
    if (tin.content.includes(tuKhoa)) {
      ketQua.push({ cumTu: tuKhoa, noiDung: tin.content });
    }
  }
  return ketQua;
}
function capNhatSoGhiNho(
  soGhiNhoHienTai: MucGhiNho[],
  tinMoi: ChatMessage,
  cacTuKhoaCanTrich: string[],
): MucGhiNho[] {
  return [...soGhiNhoHienTai, ...trichXuatGhiNho(tinMoi, cacTuKhoaCanTrich)];
}

function moPhongPhienVoiSoGhiNho(
  cacLuot: ChatMessage[][],
  nganSach: number,
  cacTuKhoaCanTrich: string[],
): { soGhiNhoCuoiCung: MucGhiNho[]; cuaSoCuoiCung: ChatMessage[] } {
  let lichSu: ChatMessage[] = [];
  let soGhiNho: MucGhiNho[] = [];
  for (const luot of cacLuot) {
    for (const tin of luot) {
      soGhiNho = capNhatSoGhiNho(soGhiNho, tin, cacTuKhoaCanTrich);
    }
    lichSu = [...lichSu, ...luot];
    if (vuotNganSach(lichSu, nganSach)) {
      lichSu = catCuaSoTruot(lichSu, nganSach);
    }
  }
  return { soGhiNhoCuoiCung: soGhiNho, cuaSoCuoiCung: lichSu };
}
```

`soGhiNho` VÀ `lichSu` LÀ HAI biến HOÀN TOÀN riêng biệt — không có dòng
nào trong vòng `for` khiến việc cắt `lichSu` ảnh hưởng ngược lại
`soGhiNho`. Đây CHÍNH LÀ điểm mấu chốt: sổ ghi nhớ không "nằm bên
trong" cửa sổ, nó LÀ một cấu trúc dữ liệu SONG SONG, được cập nhật từ
CÙNG message NHƯNG không bao giờ bị đụng tới bởi bất kỳ phép cắt nào.
::::

::::example{#do-tren-bay-luot}
Bảy lượt — CÙNG `CAC_LUOT_HOI_THOAI` (bài `6`, q9.2a), ngân sách `50`,
ba cụm từ cần theo dõi (`"XY789"`, `"30 ngay"`, `"giam gia"`):

```typescript title=readonly
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}
interface MucGhiNho {
  cumTu: string;
  noiDung: string;
}

function demTokenGiaLap(text: string): number {
  return Math.ceil(text.length / 4);
}
function tinhTongToken(messages: ChatMessage[]): number {
  return messages.reduce((tong, tin) => tong + demTokenGiaLap(tin.content), 0);
}
function vuotNganSach(messages: ChatMessage[], nganSach: number): boolean {
  return tinhTongToken(messages) > nganSach;
}
function catCuaSoTruot(messages: ChatMessage[], nganSach: number): ChatMessage[] {
  if (messages.length === 0) return [];
  const laHeThong = messages[0]!.role === "system";
  const phanGiuCoDinh = laHeThong ? [messages[0]!] : [];
  const phanCoTheCat = laHeThong ? messages.slice(1) : messages.slice();
  let batDau = 0;
  while (batDau < phanCoTheCat.length && tinhTongToken([...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)]) > nganSach) { batDau += 1; }
  return [...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)];
}
function coConNhoSuKien(messages: ChatMessage[], cumTuCanTim: string): boolean {
  return messages.some((tin) => tin.content.includes(cumTuCanTim));
}
function trichXuatGhiNho(tin: ChatMessage, cacTuKhoaCanTrich: string[]): MucGhiNho[] {
  const ketQua: MucGhiNho[] = [];
  for (const tuKhoa of cacTuKhoaCanTrich) {
    if (tin.content.includes(tuKhoa)) {
      ketQua.push({ cumTu: tuKhoa, noiDung: tin.content });
    }
  }
  return ketQua;
}
function capNhatSoGhiNho(
  soGhiNhoHienTai: MucGhiNho[],
  tinMoi: ChatMessage,
  cacTuKhoaCanTrich: string[],
): MucGhiNho[] {
  return [...soGhiNhoHienTai, ...trichXuatGhiNho(tinMoi, cacTuKhoaCanTrich)];
}
function tinhTyLeRecall(soGhiNho: MucGhiNho[], cacCumTuCanKiemTra: string[]): number {
  const soCoMat = cacCumTuCanKiemTra.filter((cum) => soGhiNho.some((m) => m.cumTu === cum)).length;
  return soCoMat / cacCumTuCanKiemTra.length;
}
function moPhongPhienVoiSoGhiNho(
  cacLuot: ChatMessage[][],
  nganSach: number,
  cacTuKhoaCanTrich: string[],
): { soGhiNhoCuoiCung: MucGhiNho[]; cuaSoCuoiCung: ChatMessage[] } {
  let lichSu: ChatMessage[] = [];
  let soGhiNho: MucGhiNho[] = [];
  for (const luot of cacLuot) {
    for (const tin of luot) {
      soGhiNho = capNhatSoGhiNho(soGhiNho, tin, cacTuKhoaCanTrich);
    }
    lichSu = [...lichSu, ...luot];
    if (vuotNganSach(lichSu, nganSach)) {
      lichSu = catCuaSoTruot(lichSu, nganSach);
    }
  }
  return { soGhiNhoCuoiCung: soGhiNho, cuaSoCuoiCung: lichSu };
}

const CAC_LUOT_HOI_THOAI: ChatMessage[][] = [
  [
    { role: "system", content: "Ban la tro ly cham soc khach hang, tra loi ngan gon." },
    { role: "user", content: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  ],
  [{ role: "assistant", content: "Toi da ghi nhan ma don hang cua ban, de toi kiem tra ngay bay gio." }],
  [{ role: "user", content: "Trong luc cho, toi muon hoi them ve chinh sach doi tra san pham noi chung." }],
  [{ role: "assistant", content: "Chinh sach doi tra cho phep doi tra trong vong 30 ngay ke tu ngay nhan hang." }],
  [{ role: "user", content: "Ngoai ra cua hang co dang chay chuong trinh khuyen mai nao khong." }],
  [{ role: "assistant", content: "Hien tai cua hang dang giam gia 10 phan tram cho don hang tren 500 nghin dong." }],
  [{ role: "user", content: "Toi cung muon hoi them ve thoi gian giao hang trung binh la bao lau." }],
];
const TU_KHOA = ["XY789", "30 ngay", "giam gia"];

const ketQua = moPhongPhienVoiSoGhiNho(CAC_LUOT_HOI_THOAI, 50, TU_KHOA);
console.log("so ghi nho cuoi cung:", ketQua.soGhiNhoCuoiCung.map((m) => m.cumTu));
console.log("ty le recall (so ghi nho):", tinhTyLeRecall(ketQua.soGhiNhoCuoiCung, TU_KHOA));
console.log("cua so cuoi cung (role):", ketQua.cuaSoCuoiCung.map((m) => m.role));
console.log("con nho XY789 trong CUA SO:", coConNhoSuKien(ketQua.cuaSoCuoiCung, "XY789"));
console.log("con nho 30 ngay trong CUA SO:", coConNhoSuKien(ketQua.cuaSoCuoiCung, "30 ngay"));
console.log("con nho giam gia trong CUA SO:", coConNhoSuKien(ketQua.cuaSoCuoiCung, "giam gia"));
```

```text title=readonly
so ghi nho cuoi cung: ["XY789","30 ngay","giam gia"]
ty le recall (so ghi nho): 1
cua so cuoi cung (role): ["system","assistant","user"]
con nho XY789 trong CUA SO: false
con nho 30 ngay trong CUA SO: false
con nho giam gia trong CUA SO: true
```

`tinhTyLeRecall` trên `soGhiNhoCuoiCung` LÀ `1` — CẢ BA sự kiện đều
được ghi nhớ ĐẦY ĐỦ, KHÔNG PHỤ THUỘC việc cửa sổ đã bị `catCuaSoTruot`
cắt bao nhiêu lần dọc đường. Nhưng `coConNhoSuKien` trên
`cuaSoCuoiCung` (chỉ còn `3` message: `system` + hai message MỚI NHẤT)
LÀ `false` cho CẢ `"XY789"` LẪN `"30 ngay"` — hai sự kiện đó đã bị cắt
khỏi PHẦN HIỂN THỊ CHO MODEL từ lâu. Sổ ghi nhớ VÀ cửa sổ hiển thị LÀ
HAI cấu trúc dữ liệu KHÁC NHAU, VÀ đo CHÚNG cho ra HAI kết quả khác
nhau trên CÙNG một kịch bản.
::::

::::predict{#doan-ngan-sach-lon commitOnce}
Tăng `nganSach` lên rất lớn (`1000` — TOÀN BỘ lịch sử không bao giờ vượt
ngân sách, `catCuaSoTruot` không bao giờ được gọi). `coConNhoSuKien`
trên `cuaSoCuoiCung` cho `"XY789"` sẽ đổi thành gì? VÀ
`tinhTyLeRecall` trên `soGhiNhoCuoiCung` có đổi theo không?

:::opt{correct}
`coConNhoSuKien` đổi thành `true` (cửa sổ giữ NGUYÊN toàn bộ `8`
message, message chứa `"XY789"` không hề bị cắt); nhưng
`tinhTyLeRecall` VẪN LÀ `1` như cũ — nó KHÔNG PHỤ THUỘC `nganSach` chút
nào, sổ ghi nhớ được cập nhật TỪ message, không phải từ cửa sổ
:::
:::opt
CẢ HAI đều đổi — tăng ngân sách LÀM toàn bộ hệ thống "nhớ tốt hơn",
bao gồm CẢ sổ ghi nhớ lẫn cửa sổ
::why
Nhầm "hệ thống nhớ tốt hơn nói chung" VỚI cách HAI cấu trúc dữ liệu
NÀY thực sự phụ thuộc `nganSach` — chỉ `cuaSoCuoiCung` (qua
`catCuaSoTruot`, VÀ điều kiện `vuotNganSach`) đọc tham số `nganSach`;
`capNhatSoGhiNho`/`trichXuatGhiNho` (xây `soGhiNho`) KHÔNG hề nhận
`nganSach` LÀM tham số, không có cách nào để nó bị ảnh hưởng.

Chỗ lệch: trong thân `moPhongPhienVoiSoGhiNho`, dòng cập nhật `soGhiNho`
nằm HOÀN TOÀN tách biệt khỏi khối `if (vuotNganSach(...))` — đổi
`nganSach` chỉ đụng tới khối `if` đó, không đụng tới dòng cập nhật
`soGhiNho`.
::
:::
:::opt
Không đổi gì cả — `nganSach = 1000` vẫn LÀ một con số hữu hạn, VÀ tổng
token của cả `8` message CÓ THỂ vẫn vượt qua nó
::why
Gần đúng Ở việc bạn thận trọng với "hữu hạn không có nghĩa LÀ đủ" —
quan sát đó ĐÚNG về nguyên tắc.

Chỗ lệch: tổng token của TOÀN BỘ tám message trong kịch bản CỤ THỂ này
nhỏ hơn nhiều so với `1000` — `vuotNganSach(lichSu, 1000)` LÀ `false`
Ở MỌI lượt, nên `catCuaSoTruot` không bao giờ được gọi, VÀ `lichSu`
cuối cùng giữ NGUYÊN toàn bộ tám message — bao gồm message chứa
`"XY789"`.
::
:::
::::

::::code{#viet_mo_phong_phien_voi_so_ghi_nho}
Hoàn thiện `moPhongPhienVoiSoGhiNho` — phần ĐẦU vòng `for`: cập nhật
`soGhiNho` cho TỪNG message trong `luot` (dùng `capNhatSoGhiNho`).
Phần CUỐI (sau vòng `for`): trả về đủ hai trường
`soGhiNhoCuoiCung`/`cuaSoCuoiCung`.

```typescript title=starter
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}
interface MucGhiNho {
  cumTu: string;
  noiDung: string;
}

function demTokenGiaLap(text: string): number {
  return Math.ceil(text.length / 4);
}
function tinhTongToken(messages: ChatMessage[]): number {
  return messages.reduce((tong, tin) => tong + demTokenGiaLap(tin.content), 0);
}
function vuotNganSach(messages: ChatMessage[], nganSach: number): boolean {
  return tinhTongToken(messages) > nganSach;
}
function catCuaSoTruot(messages: ChatMessage[], nganSach: number): ChatMessage[] {
  if (messages.length === 0) return [];
  const laHeThong = messages[0]!.role === "system";
  const phanGiuCoDinh = laHeThong ? [messages[0]!] : [];
  const phanCoTheCat = laHeThong ? messages.slice(1) : messages.slice();
  let batDau = 0;
  while (batDau < phanCoTheCat.length && tinhTongToken([...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)]) > nganSach) { batDau += 1; }
  return [...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)];
}
function coConNhoSuKien(messages: ChatMessage[], cumTuCanTim: string): boolean {
  return messages.some((tin) => tin.content.includes(cumTuCanTim));
}
function trichXuatGhiNho(tin: ChatMessage, cacTuKhoaCanTrich: string[]): MucGhiNho[] {
  const ketQua: MucGhiNho[] = [];
  for (const tuKhoa of cacTuKhoaCanTrich) {
    if (tin.content.includes(tuKhoa)) {
      ketQua.push({ cumTu: tuKhoa, noiDung: tin.content });
    }
  }
  return ketQua;
}
function capNhatSoGhiNho(
  soGhiNhoHienTai: MucGhiNho[],
  tinMoi: ChatMessage,
  cacTuKhoaCanTrich: string[],
): MucGhiNho[] {
  return [...soGhiNhoHienTai, ...trichXuatGhiNho(tinMoi, cacTuKhoaCanTrich)];
}
function tinhTyLeRecall(soGhiNho: MucGhiNho[], cacCumTuCanKiemTra: string[]): number {
  const soCoMat = cacCumTuCanKiemTra.filter((cum) => soGhiNho.some((m) => m.cumTu === cum)).length;
  return soCoMat / cacCumTuCanKiemTra.length;
}

function moPhongPhienVoiSoGhiNho(
  cacLuot: ChatMessage[][],
  nganSach: number,
  cacTuKhoaCanTrich: string[],
): { soGhiNhoCuoiCung: MucGhiNho[]; cuaSoCuoiCung: ChatMessage[] } {
  let lichSu: ChatMessage[] = [];
  let soGhiNho: MucGhiNho[] = [];
  for (const luot of cacLuot) {
    ___
    lichSu = [...lichSu, ...luot];
    if (vuotNganSach(lichSu, nganSach)) {
      lichSu = catCuaSoTruot(lichSu, nganSach);
    }
  }
  ___
}

const CAC_LUOT_HOI_THOAI: ChatMessage[][] = [
  [
    { role: "system", content: "Ban la tro ly cham soc khach hang, tra loi ngan gon." },
    { role: "user", content: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  ],
  [{ role: "assistant", content: "Toi da ghi nhan ma don hang cua ban, de toi kiem tra ngay bay gio." }],
  [{ role: "user", content: "Trong luc cho, toi muon hoi them ve chinh sach doi tra san pham noi chung." }],
  [{ role: "assistant", content: "Chinh sach doi tra cho phep doi tra trong vong 30 ngay ke tu ngay nhan hang." }],
  [{ role: "user", content: "Ngoai ra cua hang co dang chay chuong trinh khuyen mai nao khong." }],
  [{ role: "assistant", content: "Hien tai cua hang dang giam gia 10 phan tram cho don hang tren 500 nghin dong." }],
  [{ role: "user", content: "Toi cung muon hoi them ve thoi gian giao hang trung binh la bao lau." }],
];
const TU_KHOA = ["XY789", "30 ngay", "giam gia"];

const ketQua = moPhongPhienVoiSoGhiNho(CAC_LUOT_HOI_THOAI, 50, TU_KHOA);
console.log(
  JSON.stringify(ketQua.soGhiNhoCuoiCung.map((m) => m.cumTu)),
  tinhTyLeRecall(ketQua.soGhiNhoCuoiCung, TU_KHOA),
  coConNhoSuKien(ketQua.cuaSoCuoiCung, "XY789"),
);
```

```typescript title=solution
interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}
interface MucGhiNho {
  cumTu: string;
  noiDung: string;
}

function demTokenGiaLap(text: string): number {
  return Math.ceil(text.length / 4);
}
function tinhTongToken(messages: ChatMessage[]): number {
  return messages.reduce((tong, tin) => tong + demTokenGiaLap(tin.content), 0);
}
function vuotNganSach(messages: ChatMessage[], nganSach: number): boolean {
  return tinhTongToken(messages) > nganSach;
}
function catCuaSoTruot(messages: ChatMessage[], nganSach: number): ChatMessage[] {
  if (messages.length === 0) return [];
  const laHeThong = messages[0]!.role === "system";
  const phanGiuCoDinh = laHeThong ? [messages[0]!] : [];
  const phanCoTheCat = laHeThong ? messages.slice(1) : messages.slice();
  let batDau = 0;
  while (batDau < phanCoTheCat.length && tinhTongToken([...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)]) > nganSach) { batDau += 1; }
  return [...phanGiuCoDinh, ...phanCoTheCat.slice(batDau)];
}
function coConNhoSuKien(messages: ChatMessage[], cumTuCanTim: string): boolean {
  return messages.some((tin) => tin.content.includes(cumTuCanTim));
}
function trichXuatGhiNho(tin: ChatMessage, cacTuKhoaCanTrich: string[]): MucGhiNho[] {
  const ketQua: MucGhiNho[] = [];
  for (const tuKhoa of cacTuKhoaCanTrich) {
    if (tin.content.includes(tuKhoa)) {
      ketQua.push({ cumTu: tuKhoa, noiDung: tin.content });
    }
  }
  return ketQua;
}
function capNhatSoGhiNho(
  soGhiNhoHienTai: MucGhiNho[],
  tinMoi: ChatMessage,
  cacTuKhoaCanTrich: string[],
): MucGhiNho[] {
  return [...soGhiNhoHienTai, ...trichXuatGhiNho(tinMoi, cacTuKhoaCanTrich)];
}
function tinhTyLeRecall(soGhiNho: MucGhiNho[], cacCumTuCanKiemTra: string[]): number {
  const soCoMat = cacCumTuCanKiemTra.filter((cum) => soGhiNho.some((m) => m.cumTu === cum)).length;
  return soCoMat / cacCumTuCanKiemTra.length;
}

function moPhongPhienVoiSoGhiNho(
  cacLuot: ChatMessage[][],
  nganSach: number,
  cacTuKhoaCanTrich: string[],
): { soGhiNhoCuoiCung: MucGhiNho[]; cuaSoCuoiCung: ChatMessage[] } {
  let lichSu: ChatMessage[] = [];
  let soGhiNho: MucGhiNho[] = [];
  for (const luot of cacLuot) {
    for (const tin of luot) {
      soGhiNho = capNhatSoGhiNho(soGhiNho, tin, cacTuKhoaCanTrich);
    }
    lichSu = [...lichSu, ...luot];
    if (vuotNganSach(lichSu, nganSach)) {
      lichSu = catCuaSoTruot(lichSu, nganSach);
    }
  }
  return { soGhiNhoCuoiCung: soGhiNho, cuaSoCuoiCung: lichSu };
}

const CAC_LUOT_HOI_THOAI: ChatMessage[][] = [
  [
    { role: "system", content: "Ban la tro ly cham soc khach hang, tra loi ngan gon." },
    { role: "user", content: "Ma don hang cua toi la XY789, xin giup kiem tra tinh trang giao hang." },
  ],
  [{ role: "assistant", content: "Toi da ghi nhan ma don hang cua ban, de toi kiem tra ngay bay gio." }],
  [{ role: "user", content: "Trong luc cho, toi muon hoi them ve chinh sach doi tra san pham noi chung." }],
  [{ role: "assistant", content: "Chinh sach doi tra cho phep doi tra trong vong 30 ngay ke tu ngay nhan hang." }],
  [{ role: "user", content: "Ngoai ra cua hang co dang chay chuong trinh khuyen mai nao khong." }],
  [{ role: "assistant", content: "Hien tai cua hang dang giam gia 10 phan tram cho don hang tren 500 nghin dong." }],
  [{ role: "user", content: "Toi cung muon hoi them ve thoi gian giao hang trung binh la bao lau." }],
];
const TU_KHOA = ["XY789", "30 ngay", "giam gia"];

const ketQua = moPhongPhienVoiSoGhiNho(CAC_LUOT_HOI_THOAI, 50, TU_KHOA);
console.log(
  JSON.stringify(ketQua.soGhiNhoCuoiCung.map((m) => m.cumTu)),
  tinhTyLeRecall(ketQua.soGhiNhoCuoiCung, TU_KHOA),
  coConNhoSuKien(ketQua.cuaSoCuoiCung, "XY789"),
);
```

```typescript title=test
if (JSON.stringify(ketQua.soGhiNhoCuoiCung.map((m) => m.cumTu)) !== JSON.stringify(["XY789", "30 ngay", "giam gia"])) {
  throw new Error("so ghi nho cuoi cung phai co dung 3 cum, dung thu tu xuat hien");
}
if (Math.abs(tinhTyLeRecall(ketQua.soGhiNhoCuoiCung, TU_KHOA) - 1) > 1e-9) {
  throw new Error("ty le recall tren so ghi nho phai la 1.0 -- khong mat cum nao du cua so bi cat");
}
if (JSON.stringify(ketQua.cuaSoCuoiCung.map((m) => m.role)) !== JSON.stringify(["system", "assistant", "user"])) {
  throw new Error("cua so cuoi cung (sau khi cat) phai con dung 3 message: system + 2 message moi nhat");
}
if (coConNhoSuKien(ketQua.cuaSoCuoiCung, "XY789") !== false) {
  throw new Error("XY789 phai bi mat khoi CUA SO hien thi (context rot), du soGhiNho van con");
}
if (coConNhoSuKien(ketQua.cuaSoCuoiCung, "30 ngay") !== false) {
  throw new Error("30 ngay cung phai bi mat khoi CUA SO hien thi");
}
if (coConNhoSuKien(ketQua.cuaSoCuoiCung, "giam gia") !== true) {
  throw new Error("giam gia (o luot gan cuoi) van con trong CUA SO hien thi");
}

const ketQuaNganSachLon = moPhongPhienVoiSoGhiNho(CAC_LUOT_HOI_THOAI, 1000, TU_KHOA);
if (coConNhoSuKien(ketQuaNganSachLon.cuaSoCuoiCung, "XY789") !== true) {
  throw new Error("ngan sach 1000 (khong bao gio can cat) thi cua so PHAI con nho XY789");
}
if (Math.abs(tinhTyLeRecall(ketQuaNganSachLon.soGhiNhoCuoiCung, TU_KHOA) - 1) > 1e-9) {
  throw new Error("doi nganSach KHONG duoc lam thay doi ty le recall tren so ghi nho (van phai la 1.0)");
}
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (dau vong for): mot vong for-of noi bo -- for (const tin of luot) { soGhiNho = capNhatSoGhiNho(soGhiNho, tin, cacTuKhoaCanTrich); }. Cho hai (sau vong for ngoai): return { soGhiNhoCuoiCung: soGhiNho, cuaSoCuoiCung: lichSu };"
- kind: strategy
  body: "Cho dau: for (const tin of luot) { soGhiNho = capNhatSoGhiNho(soGhiNho, tin, cacTuKhoaCanTrich); } Cho hai: return { soGhiNhoCuoiCung: soGhiNho, cuaSoCuoiCung: lichSu };"
- kind: one-line
  body: "Sao chep dung hai khoi o phan Strategy vao dung vi tri tuong ung, DUNG THU TU."
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 7000
- tier: output
  match: contains
  expect: "[\"XY789\",\"30 ngay\",\"giam gia\"] 1 false"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`1.0` trên sổ ghi nhớ, `false` trên cửa sổ — CÙNG một phiên, CÙNG hai
sự kiện, HAI cấu trúc dữ liệu cho ra HAI câu trả lời khác nhau. BOSS
của q9.2b ráp TẤT CẢ sáu bài — tóm tắt, ngân sách theo khu vực, sổ ghi
nhớ, tỉ lệ recall, VÀ đo trên MỘT phiên nhiều lượt CUỐI CÙNG, đóng
q9.2b tại `6/6`.
::::

::::reflect{#nghi-lai}
Bài này không phát minh cơ chế mới nào — nó chỉ CHẠY hai cơ chế ĐÃ CÓ
(`capNhatSoGhiNho` bài `9`, `catCuaSoTruot` bài `2` q9.2a) SONG SONG
trên CÙNG một phiên, VÀ đo CẢ HAI kết quả cùng lúc. Bài học quan trọng
nhất: "cửa sổ" (những gì model THẬT SỰ nhìn thấy Ở lượt kế tiếp) VÀ
"sổ ghi nhớ" (những gì hệ thống đã TRÍCH XUẤT VÀ LƯU LẠI) LÀ HAI khái
niệm khác nhau, phục vụ hai mục đích khác nhau — cửa sổ nhỏ, đủ NGẮN để
gửi trực tiếp cho model MỖI lượt; sổ ghi nhớ lớn hơn, BỀN hơn, dùng để
TRA CỨU khi cần (ví dụ: tóm tắt lại, hoặc chèn NGƯỢC vào cửa sổ nếu một
lượt sau hỏi đúng về sự kiện đó). Một hệ thống production nghiêm túc
cần CẢ HAI — không cái nào thay thế được cái còn lại.
::::

::::checkpoint{mastery=0.86}
::::
